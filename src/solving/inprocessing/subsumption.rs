extern crate fixedbitset;
use core::*;
use std::ops::{BitAnd, BitOr};
use self::fixedbitset::FixedBitSet;


fn is_tautology(c: &Clause) -> bool{
    for lit in c.iter(){
        if c.contains(&-*lit) {return true; }
    }
    return false;
}
pub fn subsume(c1: &Clause, c2: &Clause) -> bool {
    if c1.len() > c2.len() {return false; }
    for lit in c1.iter(){
        if !c2.contains(lit){
            return false
        }
    }
    return true;
}

pub fn subsume_without_lit(c1: &Clause, c2: &Clause, l: Literal) -> bool {
    if c1.len() > c2.len()+1 {return false; }
    for lit in c1.iter(){
        if *lit != l &&!c2.contains(lit){
            return false
        }
    }
    return true;
}
pub fn self_subsuming_resolution_forward(c1: &mut Clause, c2: &mut Clause) {
    if c1.len() > c2.len() {return;}
    let mut idx = 0;
    let mut found = false;
    let mut l_remove: Literal = Literal::from(42);
    for lit in c1.iter(){
        if c2.contains(&-*lit) && subsume_without_lit(c1,c2,*lit){
            found = true;
            l_remove = -*lit;
            break;
        }
        idx += 1;
    }
    if found {
        if c1.len() == c2.len(){c1.swap_remove(idx);}
        c2.remove_lit(l_remove);
    }
}
/*
fn is_tautology(c: &Clause) -> bool{
    for lit in c.iter(){
        if lit.to_isize() > 0 && c.negative.contains(lit.to_isize() as usize){ return true; }
            else if lit.to_isize() < 0 && c.positive.contains((-lit.to_isize()) as usize){ return true; }
    }
    return false;
}

fn subsume(c1: &Clause, c2: &Clause) -> bool {
    if c1.len() > c2.len() {return false; }
    return ((*c1).positive.bitand(&(*c2).positive) == (*c1).positive) &&
        ((*c1).negative.bitand(&(*c2).negative) == (*c1).negative);
}

// Lit must be in c1 and -Lit in c2 !!!
pub fn subsume_without_lit(c1: &Clause, c2: &Clause, l: Literal) -> bool {
    if c1.len() > c2.len()+1 {return false; }
    let zero = FixedBitSet::with_capacity((*c1).positive.len());
    let mut pos_bitset = (*c1).positive.bitand(&(*c2).positive);
    let mut neg_bitset = (*c1).negative.bitand(&(*c2).negative);
    let mut pos_without = zero.bitor(&(*c1).positive);
    let mut neg_without = zero.bitor(&(*c1).negative);
    if l.to_isize() > 0 {
        pos_bitset.set(usize::from(l.var()) , false);
        pos_without.set(usize::from(l.var()), false);
    }
        else {
            neg_bitset.set(usize::from(l.var()), false);
            neg_without.set(usize::from(l.var()), false);
        }
    return pos_bitset == pos_without && neg_bitset == neg_without;
}

fn self_subsuming_resolution_forward(c1: &mut Clause, c2: &mut Clause) {
    if c1.len() > c2.len() {return;} // TODO : directly call reverse ?
    let mut idx = 0;
    let mut found = false;
    let mut l_remove: Literal = Literal::from(42);
    for lit in c1.iter(){
        if c2.contains_lit(Literal::from_var(lit.var(), -lit.sign())) && subsume_without_lit(c1,c2,*lit){
            found = true;
            l_remove = Literal::from_var(lit.var(), -lit.sign());
            break;
        }
        idx += 1;
    }
    if found {
        if c1.len() == c2.len(){c1.remove(idx);}
        c2.remove_lit(l_remove);
    }
}
*/

// -----------------------------------------------------------------------------------------------
/// # Unit Tests
// -----------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn clause_subsumes_itself() {
        let clause1 = Clause::new(vec![
            Literal::from(1),
            Literal::from(2),
            Literal::from(4),
            Literal::from(8)], false);

        let clause2 = Clause::new(vec![
            Literal::from(2),
            Literal::from(4),
            Literal::from(1),
            Literal::from(8)], false);
        assert_eq!(subsume(&clause1,&clause2), true);
        assert_eq!(subsume(&clause1,&clause1), true)
    }
    #[test]
    fn clause_does_not_subsumes() {
        let clause1 = Clause::new(vec![
            Literal::from(1),
            Literal::from(2),
            Literal::from(4),
            Literal::from(8)], false);

        let clause2 = Clause::new(vec![
            Literal::from(2),
            Literal::from(4),
            Literal::from(1),
            Literal::from(7)], false);
        let clause3 = Clause::new(vec![
            Literal::from(2),
            Literal::from(5),
            Literal::from(1),
            Literal::from(8)], false);
        assert_eq!(subsume(&clause1,&clause2), false);
        assert_eq!(subsume(&clause1,&clause3), false)
    }
    #[test]
    fn bigger_clause_does_not_subsume() {
        let clause1 = Clause::new(vec![
            Literal::from(1),
            Literal::from(2),
            Literal::from(4),
            Literal::from(8),
            Literal::from(9)], false);

        let clause2 = Clause::new(vec![
            Literal::from(1),
            Literal::from(2),
            Literal::from(4),
            Literal::from(8)], false);
        assert_eq!(subsume(&clause1, &clause2), false)
    }
    #[test]
    fn tautology(){
        let clause1 = Clause::new(vec![
            Literal::from(1),
            Literal::from(2),
            Literal::from(4),
            Literal::from(-4),
            Literal::from(9)], false);
        let clause2 = Clause::new(vec![
            Literal::from(1),
            Literal::from(2),
            Literal::from(4),
            Literal::from(8),
            Literal::from(9)], false);
        let clause3 = Clause::new(vec![
            Literal::from(1),
            Literal::from(-1)], false);
        assert_eq!(is_tautology(&clause1), true);
        assert_eq!(is_tautology(&clause3), true);
        assert_eq!(is_tautology(&clause2), false)
    }
    #[test]
    fn clause_subsumes_without() {
        let clause1 = Clause::new(vec![
            Literal::from(1),
            Literal::from(2),
            Literal::from(4)], false);

        let clause2 = Clause::new(vec![
            Literal::from(1),
            Literal::from(2),
            Literal::from(-4)], false);
        assert_eq!(subsume_without_lit(&clause1,&clause2,Literal::from(4)), true);
        assert_eq!(subsume_without_lit(&clause2,&clause1,Literal::from(-4)), true);
        let clause1 = Clause::new(vec![
            Literal::from(1),
            Literal::from(2),
            Literal::from(4),
            Literal::from(7),
            Literal::from(8)], false);

        let clause2 = Clause::new(vec![
            Literal::from(2),
            Literal::from(4),
            Literal::from(1),
            Literal::from(8)], false);
        assert_eq!(subsume(&clause1,&clause2), false);
        assert_eq!(subsume_without_lit(&clause1,&clause2, Literal::from(7)), true);

    }
    #[test]
    fn bigger_clause_does_not_subsume_without() {
        let clause1 = Clause::new(vec![
            Literal::from(1),
            Literal::from(2),
            Literal::from(4),
            Literal::from(8),
            Literal::from(9),
            Literal::from(10)], false);

        let clause2 = Clause::new(vec![
            Literal::from(1),
            Literal::from(2),
            Literal::from(4),
            Literal::from(8)], false);
        assert_eq!(subsume_without_lit(&clause1, &clause2, Literal::from(10)), false)
    }

    #[test]
    fn self_subsuming_resolution_test() {
        let mut clause1 = Clause::new(vec![
            Literal::from(1),
            Literal::from(-2),
            Literal::from(4)], false);

        let mut clause2 = Clause::new(vec![
            Literal::from(1),
            Literal::from(-2),
            Literal::from(-4)], false);

        let mut clause3 = Clause::new(vec![
            Literal::from(1),
            Literal::from(2),
            Literal::from(5)], false);
        self_subsuming_resolution_forward(&mut clause2, &mut clause3);
        assert_eq!(*clause2, vec![Literal::from(1),Literal::from(-2), Literal::from(-4)]);
        assert_eq!(*clause3, vec![Literal::from(1),Literal::from(2), Literal::from(5)]);

        self_subsuming_resolution_forward(&mut clause1, &mut clause2);
        assert_eq!(*clause1, vec![Literal::from(1),Literal::from(-2)]);
        assert_eq!(*clause2, vec![Literal::from(1),Literal::from(-2)]);

        self_subsuming_resolution_forward(&mut clause2, &mut clause3);
        assert_eq!(*clause2, vec![Literal::from(1),Literal::from(-2)]);
        assert_eq!(*clause3, vec![Literal::from(1), Literal::from(5)]);
    }

}


/*
// -----------------------------------------------------------------------------------------------
/// # Unit Tests
// -----------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn clause_subsumes_itself() {
        let clause1 = Clause::new(vec![
            Literal::from(1),
            Literal::from(2),
            Literal::from(4),
            Literal::from(8)], false,8);

        let clause2 = Clause::new(vec![
            Literal::from(2),
            Literal::from(4),
            Literal::from(1),
            Literal::from(8)], false,8);
        assert_eq!(subsume(&clause1,&clause2), true);
        assert_eq!(subsume(&clause1,&clause1), true)
    }
    #[test]
    fn clause_does_not_subsumes() {
        let clause1 = Clause::new(vec![
            Literal::from(1),
            Literal::from(2),
            Literal::from(4),
            Literal::from(8)], false,8);

        let clause2 = Clause::new(vec![
            Literal::from(2),
            Literal::from(4),
            Literal::from(1),
            Literal::from(7)], false,8);
        let clause3 = Clause::new(vec![
            Literal::from(2),
            Literal::from(5),
            Literal::from(1),
            Literal::from(8)], false,8);
        assert_eq!(subsume(&clause1,&clause2), false);
        assert_eq!(subsume(&clause1,&clause3), false)
    }
    #[test]
    fn bigger_clause_does_not_subsume() {
        let clause1 = Clause::new(vec![
            Literal::from(1),
            Literal::from(2),
            Literal::from(4),
            Literal::from(8),
            Literal::from(9)], false,9);

        let clause2 = Clause::new(vec![
            Literal::from(1),
            Literal::from(2),
            Literal::from(4),
            Literal::from(8)], false,8);
        assert_eq!(subsume(&clause1, &clause2), false)
    }
    #[test]
    fn tautology(){
        let clause1 = Clause::new(vec![
            Literal::from(1),
            Literal::from(2),
            Literal::from(4),
            Literal::from(-4),
            Literal::from(9)], false,9);
        let clause2 = Clause::new(vec![
            Literal::from(1),
            Literal::from(2),
            Literal::from(4),
            Literal::from(8),
            Literal::from(9)], false,9);
        let clause3 = Clause::new(vec![
            Literal::from(1),
            Literal::from(-1)], false,1);
        assert_eq!(is_tautology(&clause1), true);
        assert_eq!(is_tautology(&clause3), true);
        assert_eq!(is_tautology(&clause2), false)
    }
    #[test]
    fn clause_subsumes_without() {
        let mut clause1 = Clause::new(vec![
            Literal::from(1),
            Literal::from(2),
            Literal::from(4)], false,5);

        let mut clause2 = Clause::new(vec![
            Literal::from(1),
            Literal::from(2),
            Literal::from(-4)], false,5);
        assert_eq!(subsume_without_lit(&clause1,&clause2,Literal::from(4)), true);
        assert_eq!(subsume_without_lit(&clause2,&clause1,Literal::from(-4)), true);
        let clause1 = Clause::new(vec![
            Literal::from(1),
            Literal::from(2),
            Literal::from(4),
            Literal::from(7),
            Literal::from(8)], false,8);

        let clause2 = Clause::new(vec![
            Literal::from(2),
            Literal::from(4),
            Literal::from(1),
            Literal::from(8)], false,8);
        assert_eq!(subsume(&clause1,&clause2), false);
        assert_eq!(subsume_without_lit(&clause1,&clause2, Literal::from(7)), true);

    }
    #[test]
    fn bigger_clause_does_not_subsume_without() {
        let clause1 = Clause::new(vec![
            Literal::from(1),
            Literal::from(2),
            Literal::from(4),
            Literal::from(8),
            Literal::from(9),
            Literal::from(10)], false,10);

        let clause2 = Clause::new(vec![
            Literal::from(1),
            Literal::from(2),
            Literal::from(4),
            Literal::from(8)], false,10);
        assert_eq!(subsume_without_lit(&clause1, &clause2, Literal::from(10)), false)
    }

    #[test]
    fn self_subsuming_resolution_test() {
        let mut clause1 = Clause::new(vec![
            Literal::from(1),
            Literal::from(-2),
            Literal::from(4)], false,5);

        let mut clause2 = Clause::new(vec![
            Literal::from(1),
            Literal::from(-2),
            Literal::from(-4)], false,5);

        let mut clause3 = Clause::new(vec![
            Literal::from(1),
            Literal::from(2),
            Literal::from(5)], false,5);
        self_subsuming_resolution_forward(&mut clause2, &mut clause3);
        assert_eq!(*clause2, vec![Literal::from(1),Literal::from(-2), Literal::from(-4)]);
        assert_eq!(*clause3, vec![Literal::from(1),Literal::from(2), Literal::from(5)]);

        self_subsuming_resolution_forward(&mut clause1, &mut clause2);
        assert_eq!(*clause1, vec![Literal::from(1),Literal::from(-2)]);
        assert_eq!(*clause2, vec![Literal::from(1),Literal::from(-2)]);

        self_subsuming_resolution_forward(&mut clause2, &mut clause3);
        assert_eq!(*clause2, vec![Literal::from(1),Literal::from(-2)]);
        assert_eq!(*clause3, vec![Literal::from(1), Literal::from(5)]);
    }

}*/
