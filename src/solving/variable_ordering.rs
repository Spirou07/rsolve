//! This file contains the implementation of an adaptable heap suitable to implement a VSIDS-like
//! variable ordering
use core::*;
use collections::VarIdxVec;
//use arrays::Array;

// -----------------------------------------------------------------------------------------------
/// # Variable Ordering
/// The variable ordering structure (aka the variable heap)
// -----------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct VariableOrdering {
    /// A binary heap implemented as an array of variables
    heap: Vec<Variable>,
    /// The score associated with each element
    score   : VarIdxVec<f64>,
    /// The position of each id in the `heap` array
    position: VarIdxVec<uint>,
    /// The current size (#elements) in the heap
    size: uint,
    /// The capacity of the buffers
    capa: uint,

    /// current increment for vsids
    vsids_increment: f64,
    /// the vsids decay factor
    vsids_decay: f64
}

impl VariableOrdering {
    /// Creates a new VariableOrdering heap capable with the given capacity. That is to say, one
    /// able to accept up to `capa` items.
    pub fn new(capa: uint) -> VariableOrdering {
        let mut ret = VariableOrdering {
            capa    : capa,
            size    : capa,
            heap    : Vec::with_capacity((1+capa) as usize),
            score   : VarIdxVec::with_capacity(capa as usize),
            position: VarIdxVec::with_capacity(capa as usize),

            vsids_increment: 1.0,
            vsids_decay    : 0.75
        };

        // fill padding with a non-existing variable
        ret.heap.push(Variable::from(capa+1));

        // initialize the heap with actual values !
        for i in 1..(capa+1) {
            ret.heap.push(Variable::from(i));
            ret.position.push(i);
            ret.score.push(1.0);
        }

        // reclaim wastefully overallocated memory
        ret.heap    .shrink_to_fit();
        ret.position.shrink_to_fit();
        ret.score   .shrink_to_fit();

        return ret;
    }

    /// return true iff there is no element left in the heap
    #[inline]
    pub fn is_empty(&self) -> bool {
        return self.size == 0;
    }

    /// updates the variable's score using microsat scoring scheme
    ///
    /// # Panics
    /// - if the given variable does not fit in the range [1 .. capa]
    #[inline]
    pub fn bump(&mut self, var: Variable) {
        self.score[var] += self.vsids_increment;

        if self.position[var] <= self.size { self.swim(var); }
    }

    /// Decays the score of all the variables. Given that this is implemented as a MiniSat-like
    /// E-VSIDS, it simply consists in multiplying the score by 1/vsids_decay
    pub fn decay(&mut self) {
        self.vsids_increment *= 1.0/self.vsids_decay;

        if self.vsids_increment > 1e100 {
            // rescale all the scores
            for i in self.score.iter_mut() {
                *i *= 1e-100;
            }
            self.vsids_increment *= 1e-100;
        }
    }

    /// Places the given `var` back in the heap (if not already present)
    ///
    /// # Panics
    /// - if the given variable does not fit in the range [1 .. capa]
    #[inline]
	pub fn push_back(&mut self, var: Variable) {
        let var_pos = self.position[var];

        // Do it iff it is not already present
        if var_pos > self.size {
            let other_pos = self.size +1;
            let other_var = self.heap[other_pos as usize];

            self.size                      += 1;
            self.heap[ var_pos   as usize ] = other_var;
            self.heap[ other_pos as usize ] = var;

            self.position[ other_var ] = var_pos;
            self.position[ var       ] = other_pos;

            self.swim(var);
        }
    }

	/// Removes the element with highest score from the heap and returns it.
	///
	/// # Return Value
	/// Returns the element with highest score on the heap.
	///
	/// # Panics
	/// - when one tries to pop an empty heap.
	#[inline]
    pub fn pop_top(&mut self) -> Variable {
        debug_assert!( !self.is_empty(), "Cannot pop from an empty heap");

        let var = self.heap[1];

        self.heap[1] = self.heap[self.size as usize];
        self.heap[self.size as usize] = var;

        self.position[ self.heap[1] ] = 1;
        self.position[ var ] = self.size;
        self.size -= 1;

        let new_head = self.heap[1];
        self.sink(new_head);

        return var;
    }

    /// Returns the score associated with some given variable
    #[inline]
    pub fn get_score(&self, var: Variable) -> f64 {
        self.score[var]
    }

    /// Sinks the given variable down the heap until the moment when the heap
    /// invariant is restored.
    ///
    /// # Note
    /// This function assumes that `var` has already been sanity checked.
    #[inline]
    fn sink(&mut self, var: Variable) {
        let mut var_pos = self.position[var] as usize;
        let var_scr = self.score[var];

        let mut kid_pos = self.max_child_of(var_pos);
        let mut kid = self.heap[kid_pos]; // this might denote a non existing variable
        let mut kid_scr = if kid_pos != 0 { self.score[kid] } else { 0.0 };

        while kid_pos != 0 && kid_scr > var_scr {
            self.heap[var_pos] = kid;
            self.position[kid] = var_pos as uint;

            var_pos = kid_pos;
            kid_pos = self.max_child_of(var_pos);
            kid     = self.heap [kid_pos];
            kid_scr = if kid_pos != 0 { self.score[kid] } else { 0.0 };
        }

        self.heap[var_pos] = var;
        self.position[var] = var_pos as uint;
    }

    /// Swims the given variable up the heap until the moment when the heap
    /// invariant is restored.
    ///
    /// # Note
    /// This function assumes that `var` has already been sanity checked.
    #[inline]
    fn swim(&mut self, var: Variable) {
        let mut var_pos = self.position[var] as usize;
        let var_scr = self.score   [var];

        let mut par_pos = var_pos >> 1;
        let mut par= self.heap [par_pos];
        let mut par_scr = if par_pos != 0 { self.score[par] } else { 0.0 };

        while par_pos > 0 && par_scr < var_scr {
            self.heap[var_pos] = par;
            self.position[par] = var_pos as uint;

            var_pos = par_pos;
            par_pos = par_pos >> 1;
            par     = self.heap [par_pos];
            par_scr = if par_pos != 0 { self.score[par] } else { 0.0 };
        }

        self.heap[var_pos] = var;
        self.position[var] = var_pos as uint;
    }

    /// Returns the *position* of the next child to use while sinking
    /// down the item at position `pos`.
    ///
    /// # Params
    /// - pos the position of a node in the heap
    /// - the position of the child with the highest score or zero
    ///   when no such child exists.
    #[inline]
    fn max_child_of(&self, pos: usize) -> usize {
        let l_pos = pos << 1;
        let r_pos = l_pos +1;

        if l_pos > (self.size as usize) { return 0;    }
        if r_pos > (self.size as usize) { return l_pos;}

        let l_scr = self.score[ self.heap[l_pos] ];
        let r_scr = self.score[ self.heap[r_pos] ];

        return if l_scr > r_scr { l_pos } else { r_pos };
    }
}

// -----------------------------------------------------------------------------------------------
/// # Unit Tests
// -----------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    const MAX: uint = 100;

    #[test]
    fn test_new() {
        let result = VariableOrdering::new(1);
        eprintln!("{:#?}", result);
    }

    #[test]
    /// isEmpty is false as long as everything is not popped
    fn is_empty_remains_false_while_everything_wasnt_popped(){
        let mut tested = VariableOrdering::new(MAX);

        for _ in 1..MAX+1 {
            assert!( !tested.is_empty() );
            tested.pop_top();
        };

        assert!( tested.is_empty() );
    }

    /// isEmpty is false after a push back
    #[test]
    fn is_empty_is_false_after_push_back(){
        let mut tested = VariableOrdering::new(MAX);

        // make it empty
        for _ in 1..MAX+1 {
            tested.pop_top();
        }

        tested.push_back(Variable::from(4));
        assert!( !tested.is_empty() );
    }

    #[test]
    #[should_panic]
    /// bump fails for zero
    fn bump_must_fail_for_zero(){
        let mut tested = VariableOrdering::new(MAX);

        tested.bump(Variable::from(0));
    }

    #[test]
    #[should_panic]
    /// bump fails above the max
    fn bump_must_fail_above_the_max() {
        let mut tested = VariableOrdering::new(MAX);
        // because the ordering can hold up to MAX variables, it means that the accepted vars
        // range from [1;MAX+1]. Hence, to get out of bounds, we need to use MAX+2.
        tested.bump(Variable::from(MAX+2));
    }

    #[test]
    /// bump changes the score, and adapts the position
    fn bump_must_update_the_score_and_position(){
        let mut tested = VariableOrdering::new(MAX);
        tested.bump(Variable::from(50));

        assert_eq!( tested.pop_top(), Variable::from(50));
    }

    #[test]
    /// bump wont push back an item that has already been popped
    fn bump_wont_push_back_an_item_that_has_been_popped(){
        let mut tested = VariableOrdering::new(MAX);
        // empty it
        for _ in 1..MAX+1 { tested.pop_top(); }

        assert!(tested.is_empty());
        tested.bump(Variable::from(42));
        assert!(tested.is_empty());
    }

    #[test]
    /// bump wont reactivate a popped item
    fn bump_wont_let_an_item_that_has_been_popped_sneak_into_the_active_ones(){
        let mut tested = VariableOrdering::new(MAX);
        // empty it
        for _ in 1..MAX+1 { tested.pop_top(); }

        assert!(tested.is_empty());
        tested.push_back(Variable::from(5));
        tested.bump(Variable::from(42));
        assert_eq!(tested.pop_top(), Variable::from(5));
        assert!(tested.is_empty());
    }

    #[test]
    /// Bump updates the score even when item is popped
    fn bump_updates_score_even_when_item_is_popped(){
        let mut tested = VariableOrdering::new(MAX);
        // empty it
        for _ in 1..MAX+1 { tested.pop_top(); }

        //assert!(tested.is_empty());
        tested.bump(Variable::from(42));
        assert!(tested.is_empty());

        // refill it
        for i in 1..MAX+1 { tested.push_back(Variable::from(i)); }

        assert_eq!(tested.pop_top(), Variable::from(42));
    }

    #[test]
    #[should_panic]
    /// pushBack fails for zero
    fn push_back_must_fail_for_zero(){
        let mut tested = VariableOrdering::new(MAX);
        tested.push_back(Variable::from(0));
    }

    #[test]
    /// pushBack has no effect if the item is already in the heap
    fn push_back_has_no_effect_when_already_on_heap(){
        let mut tested = VariableOrdering::new(MAX);
        // empty it
        for _ in 1..MAX+1 { tested.pop_top(); }
        // only 10 on heap
        tested.push_back(Variable::from(10));
        tested.push_back(Variable::from(10));

        assert_eq!(Variable::from(10), tested.pop_top());
        assert!(tested.is_empty());
    }

    #[test]
    /// pushBack effectively insert the item at the right place in the heap
    fn push_back_must_effectively_put_item_back_on_the_heap(){
        let mut tested = VariableOrdering::new(MAX);
        // empty it
        for _ in 1..MAX+1 { tested.pop_top(); }
        // only 10 on heap
        tested.push_back(Variable::from(10));

        assert!( !tested.is_empty());
        assert_eq!(Variable::from(10), tested.pop_top());
        assert!(tested.is_empty());
    }

    #[test]
    /// pushBack effectively insert the item at the right place in the heap
    fn push_back_must_effectively_put_item_back_on_the_heap_2(){
        let mut tested = VariableOrdering::new(MAX);
        // empty it
        for _ in 1..MAX+1 { tested.pop_top(); }

        tested.bump(Variable::from(2));
        tested.decay();
        tested.bump(Variable::from(3));
        tested.decay();
        tested.bump(Variable::from(7));
        tested.decay();
        tested.bump(Variable::from(9));

        tested.push_back(Variable::from(7));
        tested.push_back(Variable::from(3));
        tested.push_back(Variable::from(9));
        tested.push_back(Variable::from(2));

        assert_eq!(tested.pop_top(),  Variable::from(9));
        assert_eq!(tested.pop_top(),  Variable::from(7));
        assert_eq!(tested.pop_top(),  Variable::from(3));
        assert_eq!(tested.pop_top(),  Variable::from(2));
        assert_eq!(tested.is_empty(), true);
    }

    #[test]
    #[should_panic]
    fn pop_top_must_fail_on_empty_heap(){
        let mut tested = VariableOrdering::new(MAX);
        // empty it
        for _ in 1..MAX+1 { tested.pop_top(); }
        // should fail
        tested.pop_top();
    }

    #[test]
    fn pop_top_must_remove_items_in_decreasing_score_order(){
        let mut tested = VariableOrdering::new(MAX);
        for i in 1..MAX+1 {
            tested.bump(Variable::from(i));
            tested.decay();
        }

        let mut last = usize::max_value();
        for i in 0..MAX {
            let popped = tested.pop_top();
            assert_eq!(popped, Variable::from(MAX-i));
            assert!   (popped.to_usize() < last);
            last = popped.to_usize();
        }
    }

    #[test]
    fn get_score_should_return_the_score_of_some_given_variable() {
        let mut tested = VariableOrdering::new(MAX);

        for i in 1..1+MAX {
            assert_eq!(1.0, tested.get_score(var(i)));
        }

        tested.bump(var(3));
        assert_eq!(2.0, tested.get_score(var(3)));
    }

    #[test]
    fn decay_should_only_update_the_vsids_increment() {
        let mut tested = VariableOrdering::new(MAX);

        let increment_before = tested.vsids_increment;
        let scores_before : Vec<f64> = (1..MAX)
        .map(|v| tested.get_score(var(v)))
        .collect();


        tested.decay();

        let increment_after = tested.vsids_increment;
        let scores_after : Vec<f64> = (1..MAX)
        .map(|v| tested.get_score(var(v)))
        .collect();

        assert_eq!(increment_after, increment_before * 1.0/tested.vsids_decay );
        assert_eq!(scores_before, scores_after);
    }

    #[test]
    fn decay_should_trigger_a_rescale_when_vsids_increment_grows_too_high() {
        let mut tested = VariableOrdering::new(MAX);

        tested.vsids_increment = 1e100; // this is the limit which will provoke a rescale

        let increment_before = tested.vsids_increment;
        let scores_rescaled : Vec<f64> = (1..MAX)
            .map(|v| tested.get_score(var(v)) * 1e-100)
            .collect();

        tested.decay();

        let increment_after = tested.vsids_increment;
        let scores_after : Vec<f64> = (1..MAX)
            .map(|v| tested.get_score(var(v)))
            .collect();

        assert_eq!(increment_after, increment_before * 1.0/tested.vsids_decay * 1e-100 );
        assert_eq!(scores_rescaled, scores_after);
    }
}