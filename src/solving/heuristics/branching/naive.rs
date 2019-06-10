use core::*;
use collections::*;
use solving::heuristics::BranchingHeuristic;

/// This is the implementation of a very naive (and basic) variable selection heuristic.
/// It isn't smart nor effective, and you probably don't want to use it except for the comparison.
#[derive(Debug)]
pub struct NaiveVariableSelection {
    heap: VarHeap
}

impl BranchingHeuristic for NaiveVariableSelection {
    /// Creates a new heuristic capable of dealing with `capa` variables.
    fn new(capa: usize) -> Self {
        let mut ret = NaiveVariableSelection {
            heap: VarHeap::new(capa),
        };

        for i in 1..capa+1 {
            ret.heap.score[Variable::from(i)] = (capa - i) as f64//(Variable::from(i))
        }

        return ret;
    }

    /// return true iff there is no element left in the heap
    #[inline]
    fn is_empty(&self) -> bool { self.heap.is_empty() }

    #[allow(unused_variables)]
    /// (Optional) Updates the variable's score according to the implemented heuristic
    fn bump(&mut self, var: Variable) {}

    /// (Optional) Ages the score of all the variables to make them appear less relevant
    fn decay(&mut self) {}

    /// Places the given `var` back in the heap (if not already present)
    ///
    /// # Panics
    /// - if the given variable does not fit in the range [1 .. capa]
    #[inline]
    fn push_back(&mut self, var: Variable) { self.heap.push_back(var) }

    /// Removes the element with highest score from the heap and returns it.
	///
	/// # Return Value
	/// Returns the element with highest score on the heap.
	///
	/// # Panics
	/// - when one tries to pop an empty heap.
    #[inline]
    fn pop_top(&mut self) -> Variable { self.heap.pop_top() }
}

// -----------------------------------------------------------------------------------------------
/// # Unit Tests
// -----------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    const MAX: usize = 100;

    #[test]
    fn test_new() {
        let result = NaiveVariableSelection::new(1);
        eprintln!("{:#?}", result);
    }

    #[test]
    /// isEmpty is false as long as everything is not popped
    fn is_empty_remains_false_while_everything_wasnt_popped() {
        let mut tested = NaiveVariableSelection::new(MAX);

        for _ in 1..MAX + 1 {
            assert!(!tested.is_empty());
            tested.pop_top();
        };

        assert!(tested.is_empty());
    }

    /// isEmpty is false after a push back
    #[test]
    fn is_empty_is_false_after_push_back() {
        let mut tested = NaiveVariableSelection::new(MAX);

        // make it empty
        for _ in 1..MAX + 1 {
            tested.pop_top();
        }

        tested.push_back(Variable::from(4_u32));
        assert!(!tested.is_empty());
    }

    #[test]
    #[should_panic]
    /// bump fails for zero
    fn bump_must_fail_for_zero() {
        let mut tested = NaiveVariableSelection::new(MAX);

        tested.bump(Variable::from(0_u32));
    }

    #[test]
    /// bump changes the score, and adapts the position
    fn bump_should_do_nothing() {
        let mut tested = NaiveVariableSelection::new(MAX);
        tested.bump(Variable::from(50_u32));

        assert_eq!(tested.pop_top(), Variable::from(1_u32));
    }

    #[test]
    #[should_panic]
    /// pushBack fails for zero
    fn push_back_must_fail_for_zero(){
        let mut tested = NaiveVariableSelection::new(MAX);
        tested.push_back(Variable::from(0_u32));
    }

    #[test]
    /// pushBack has no effect if the item is already in the heap
    fn push_back_has_no_effect_when_already_on_heap(){
        let mut tested = NaiveVariableSelection::new(MAX);
        // empty it
        for _ in 1..MAX+1 { tested.pop_top(); }
        // only 10 on heap
        tested.push_back(Variable::from(10_u32));
        tested.push_back(Variable::from(10_u32));

        assert_eq!(Variable::from(10_u32), tested.pop_top());
        assert!(tested.is_empty());
    }

    #[test]
    /// pushBack effectively insert the item at the right place in the heap
    fn push_back_must_effectively_put_item_back_on_the_heap(){
        let mut tested = NaiveVariableSelection::new(MAX);
        // empty it
        for _ in 1..MAX+1 { tested.pop_top(); }
        // only 10 on heap
        tested.push_back(Variable::from(10_u32));

        assert!( !tested.is_empty());
        assert_eq!(Variable::from(10_u32), tested.pop_top());
        assert!(tested.is_empty());
    }

    #[test]
    /// pushBack effectively insert the item at the right place in the heap
    fn push_back_must_effectively_put_item_back_on_the_heap_2(){
        let mut tested = NaiveVariableSelection::new(MAX);
        // empty it
        for _ in 1..MAX+1 { tested.pop_top(); }


        tested.push_back(Variable::from(7_u32));
        tested.push_back(Variable::from(3_u32));
        tested.push_back(Variable::from(9_u32));
        tested.push_back(Variable::from(2_u32));

        assert_eq!(tested.pop_top(),  Variable::from(2_u32));
        assert_eq!(tested.pop_top(),  Variable::from(3_u32));
        assert_eq!(tested.pop_top(),  Variable::from(7_u32));
        assert_eq!(tested.pop_top(),  Variable::from(9_u32));
        assert_eq!(tested.is_empty(), true);
    }

    #[test]
    #[should_panic]
    fn pop_top_must_fail_on_empty_heap(){
        let mut tested = NaiveVariableSelection::new(MAX);
        // empty it
        for _ in 1..MAX+1 { tested.pop_top(); }
        // should fail
        tested.pop_top();
    }

    #[test]
    fn pop_top_must_remove_items_in_decreasing_score_order(){
        let mut tested = NaiveVariableSelection::new(MAX);

        let mut last = 0;
        for i in 1..MAX+1 {
            let popped = tested.pop_top();
            assert_eq!(popped, Variable::from(i));
            assert!   (usize::from(popped) > last);
            last = popped.into();
        }
    }
}
