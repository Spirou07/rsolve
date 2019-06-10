extern crate rand;

use solving::heuristics::BranchingHeuristic;
use core::*;
use self::rand::Rng;


// -----------------------------------------------------------------------------------------------
/// A Random heuristic for selecting decision variables.
// -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Random {
    /// A binary heap implemented as an array of variables
    available: Vec<Variable>
}

impl BranchingHeuristic for Random {
    /// Creates a new Random capable of dealing with `capa` variables.
    #[inline]
    fn new(capa: usize) -> Random {
        let mut ret = Random {available: Vec::with_capacity(capa) };

        for i in 1..(1+capa) {
            ret.available.push(Variable::from(i))
        }

        return ret;
    }

    #[allow(unused_variables)]
    /// (Optional) Updates the variable's score according to the implemented heuristic
    fn bump(&mut self, var: Variable) {}

    /// (Optional) Ages the score of all the variables to make them appear less relevant
    fn decay(&mut self) {}

    /// return true iff there is no element left in the heap
    #[inline]
    fn is_empty(&self) -> bool { self.available.is_empty() }

    /// Places the given `var` back in the heap (if not already present)
    ///
    /// # Panics
    /// - if the given variable does not fit in the range [1 .. capa]
    #[inline]
    fn push_back(&mut self, var: Variable) {
        if self.available.iter().find(|v| **v == var).is_none() {
            self.available.push(var);
        }
    }

    /// Removes the element with highest score from the heap and returns it.
    ///
	/// # Return Value
	/// Returns a random element of the heap.
	///
    /// # Panics
    /// - when one tries to pop an empty heap.
    #[inline]
    fn pop_top(&mut self) -> Variable {
        let random_number = rand::thread_rng().gen_range(0, self.available.len());
        self.available.swap_remove(random_number)
    }
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
        let result = Random::new(1);
        eprintln!("{:#?}", result);
    }

    #[test]
    /// isEmpty is false as long as everything is not popped
    fn is_empty_remains_false_while_everything_wasnt_popped(){
        let mut tested = Random::new(MAX);

        for _ in 1..MAX+1 {
            assert!( !tested.is_empty() );
            tested.pop_top();
        };

        assert!( tested.is_empty() );
    }

    /// isEmpty is false after a push back
    #[test]
    fn is_empty_is_false_after_push_back(){
        let mut tested = Random::new(MAX);

        // make it empty
        for _ in 1..MAX+1 {
            tested.pop_top();
        }

        tested.push_back(Variable::from(4_u32));
        assert!( !tested.is_empty() );
    }
    #[test]
    #[should_panic]
    /// pushBack fails for zero
    fn push_back_must_fail_for_zero(){
        let mut tested = Random::new(MAX);
        tested.push_back(Variable::from(0_u32));
    }

    #[test]
    /// pushBack has no effect if the item is already in the heap
    fn push_back_has_no_effect_when_already_on_heap(){
        let mut tested = Random::new(MAX);
        // empty it
        for _ in 1..MAX+1 { tested.pop_top(); }
        // only 10 on heap
        tested.push_back(Variable::from(10_u32));
        tested.push_back(Variable::from(10_u32));

        assert_eq!(Variable::from(10_u32), tested.pop_top());
        assert!(tested.is_empty());
    }

    #[test]
    #[should_panic]
    fn pop_top_must_fail_on_empty_heap(){
        let mut tested = Random::new(MAX);
        // empty it
        for _ in 1..MAX+1 { tested.pop_top(); }
        // should fail
        tested.pop_top();
    }

}