use core::*;
use collections::*;
use solving::heuristics::BranchingHeuristic;

// -----------------------------------------------------------------------------------------------
/// The Average Conflict-Index Decision Score (ACIDS) heuristic for selecting decision variables.
// -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct ACIDS {
    /// A binary heap implemented as an array of variables
    heap: VarHeap,

    /// the acids decay factor
    conflict_index: u64,
}


impl BranchingHeuristic for ACIDS {
    /// Creates a new ACIDS capable of dealing with `capa` variables.
    #[inline]
    fn new(capa: usize) -> ACIDS {
        ACIDS {
            heap: VarHeap::new_zero(capa),
            conflict_index: 1
        }
    }

    /// updates the variable's score
    ///
    /// # Panics
    /// - if the given variable does not fit in the range [1 .. capa]
    #[inline]
    fn bump(&mut self, var: Variable) {
        self.heap.score[var] /= 2.0;
        self.heap.score[var] += self.conflict_index as f64 /2.0;

        if self.heap.position[var] <= self.heap.size { self.heap.swim(var); }
    }


    /// Just increase conflict index
    #[inline]
    fn decay(&mut self) {
        self.conflict_index += 1;
    }
    /// return true iff there is no element left in the heap
    #[inline]
    fn is_empty(&self) -> bool { self.heap.is_empty() }

    /// Places the given `var` back in the heap (if not already present)
    ///
    /// # Panics
    /// - if the given variable does not fit in the range [1 .. capa]
    #[inline]
    fn push_back(&mut self, var: Variable) { self.heap.push_back(var) }

    /// Removes the element with highest score from the heap and returns it.
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
        let result = ACIDS::new(1);
        eprintln!("{:#?}", result);
    }

    #[test]
    /// isEmpty is false as long as everything is not popped
    fn is_empty_remains_false_while_everything_wasnt_popped(){
        let mut tested = ACIDS::new(MAX);

        for _ in 1..MAX+1 {
            assert!( !tested.is_empty() );
            tested.pop_top();
        };

        assert!( tested.is_empty() );
    }

    /// isEmpty is false after a push back
    #[test]
    fn is_empty_is_false_after_push_back(){
        let mut tested = ACIDS::new(MAX);

        // make it empty
        for _ in 1..MAX+1 {
            tested.pop_top();
        }

        tested.push_back(Variable::from(4_u32));
        assert!( !tested.is_empty() );
    }

    #[test]
    #[should_panic]
    /// bump fails for zero
    fn bump_must_fail_for_zero(){
        let mut tested = ACIDS::new(MAX);

        tested.bump(Variable::from(0_u32));
    }

    #[test]
    #[should_panic]
    /// bump fails above the max
    fn bump_must_fail_above_the_max() {
        let mut tested = ACIDS::new(MAX);
        // because the ordering can hold up to MAX variables, it means that the accepted vars
        // range from [1;MAX+1]. Hence, to get out of bounds, we need to use MAX+2.
        tested.bump(Variable::from(MAX+2));
    }

    #[test]
    /// bump changes the score, and adapts the position
    fn bump_must_update_the_score_and_position(){
        let mut tested = ACIDS::new(MAX);
        tested.bump(Variable::from(50_u32));
        assert_eq!( tested.pop_top(), Variable::from(50_u32));
    }

    #[test]
    /// bump wont push back an item that has already been popped
    fn bump_wont_push_back_an_item_that_has_been_popped(){
        let mut tested = ACIDS::new(MAX);
        // empty it
        for _ in 1..MAX+1 { tested.pop_top(); }

        assert!(tested.is_empty());
        tested.bump(Variable::from(42_u32));
        assert!(tested.is_empty());
    }

    #[test]
    /// bump wont reactivate a popped item
    fn bump_wont_let_an_item_that_has_been_popped_sneak_into_the_active_ones(){
        let mut tested = ACIDS::new(MAX);
        // empty it
        for _ in 1..MAX+1 { tested.pop_top(); }

        assert!(tested.is_empty());
        tested.push_back(Variable::from(5_u32));
        tested.bump(Variable::from(42_u32));
        assert_eq!(tested.pop_top(), Variable::from(5_u32));
        assert!(tested.is_empty());
    }

    #[test]
    /// Bump updates the score even when item is popped
    fn bump_updates_score_even_when_item_is_popped(){
        let mut tested = ACIDS::new(MAX);
        // empty it
        for _ in 1..MAX+1 { tested.pop_top(); }

        //assert!(tested.is_empty());
        tested.bump(Variable::from(42_u32));
        assert!(tested.is_empty());

        // refill it
        for i in 1..MAX+1 { tested.push_back(Variable::from(i)); }

        assert_eq!(tested.pop_top(), Variable::from(42_u32));
    }

    #[test]
    #[should_panic]
    /// pushBack fails for zero
    fn push_back_must_fail_for_zero(){
        let mut tested = ACIDS::new(MAX);
        tested.push_back(Variable::from(0_u32));
    }

    #[test]
    /// pushBack has no effect if the item is already in the heap
    fn push_back_has_no_effect_when_already_on_heap(){
        let mut tested = ACIDS::new(MAX);
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
        let mut tested = ACIDS::new(MAX);
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
        let mut tested = ACIDS::new(MAX);
        // empty it
        for _ in 1..MAX+1 { tested.pop_top(); }

        tested.bump(Variable::from(2_u32));
        tested.decay();
        tested.bump(Variable::from(3_u32));
        tested.decay();
        tested.bump(Variable::from(7_u32));
        tested.decay();
        tested.bump(Variable::from(9_u32));

        tested.push_back(Variable::from(7_u32));
        tested.push_back(Variable::from(3_u32));
        tested.push_back(Variable::from(9_u32));
        tested.push_back(Variable::from(2_u32));

        assert_eq!(tested.pop_top(),  Variable::from(9_u32));
        assert_eq!(tested.pop_top(),  Variable::from(7_u32));
        assert_eq!(tested.pop_top(),  Variable::from(3_u32));
        assert_eq!(tested.pop_top(),  Variable::from(2_u32));
        assert_eq!(tested.is_empty(), true);
    }

    #[test]
    #[should_panic]
    fn pop_top_must_fail_on_empty_heap(){
        let mut tested = ACIDS::new(MAX);
        // empty it
        for _ in 1..MAX+1 { tested.pop_top(); }
        // should fail
        tested.pop_top();
    }

    #[test]
    fn pop_top_must_remove_items_in_decreasing_score_order(){
        let mut tested = ACIDS::new(MAX);
        for i in 1..MAX+1 {
            tested.bump(Variable::from(i));
            tested.decay();
        }

        let mut last = usize::max_value();
        for i in 0..MAX {
            let popped = tested.pop_top();
            assert_eq!(popped, Variable::from(MAX-i));
            assert!   (usize::from(popped) < last);
            last = popped.into();
        }
    }


}