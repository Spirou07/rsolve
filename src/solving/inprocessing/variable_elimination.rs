use core::*;
/*
pub fn ve(c1: &mut Clause, c2: &Clause) {
    for lit in c1.iter(){
        if c2.contains_lit(-*lit){
            (*c1).remove_lit(*lit);
            let it: Vec<Literal> = c2.iter()
                .map(|x| *x)
                .filter(|x| (*x != -*lit) && (!c1.contains(x)))
                .collect();
            for l in it {
                c1.push(l);
            }
        }
    }
}*/

pub fn ve1(c1: &mut Clause, c2: &Clause) {
    let mut found_l = Literal::from(42);
    let mut found = false;
    for lit in c1.iter(){
        if c2.contains_lit(-*lit){
            found = true;
            found_l = *lit
        }
    }
    if !found {return;}
    (*c1).remove_lit(found_l);
    let it: Vec<Literal> = c2.iter()
        .map(|x| *x)
        .filter(|x| (*x != -found_l) && (!c1.contains(x)))
        .collect();
    for l in it {
        c1.push(l);
    }
}
