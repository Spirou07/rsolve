#[test]
fn find_new_literal_does_nothing_if_the_clause_is_already_sat(){
    let mut valuation= Valuation::new(8);

    valuation[Variable::from(1)] = VariableState{ value: Bool::True , reason: None};
    valuation[Variable::from(2)] = VariableState{ value: Bool::False, reason: None};
    valuation[Variable::from(4)] = VariableState{ value: Bool::False, reason: None};
    valuation[Variable::from(8)] = VariableState{ value: Bool::Undef, reason: None};

    // create the tested clause
    let mut clause = Clause(vec![
        Literal::from(1),
        Literal::from(2),
        Literal::from(4),
        Literal::from(8)]);

    let watched = Literal::from(2);
    assert_eq!(clause.find_new_literal(watched, &valuation), Ok(Literal::from(2)))
}

#[test]
fn find_new_literal_does_nothing_if_the_clause_is_already_sat_2(){
    let mut valuation= Valuation::new(8);

    valuation[Variable::from(1)] = VariableState{ value: Bool::False, reason: None};
    valuation[Variable::from(2)] = VariableState{ value: Bool::True , reason: None};
    valuation[Variable::from(4)] = VariableState{ value: Bool::False, reason: None};
    valuation[Variable::from(8)] = VariableState{ value: Bool::Undef, reason: None};

    // create the tested clause
    let mut clause = Clause(vec![
        Literal::from(1),
        Literal::from(2),
        Literal::from(4),
        Literal::from(8)]);

    let watched = Literal::from(1);
    assert_eq!(clause.find_new_literal(watched, &valuation), Ok(Literal::from(1)))
}

#[test]
fn find_new_literal_returns_ok_with_the_first_unassigned(){
    let mut valuation= Valuation::new(8);

    valuation[Variable::from(1)] = VariableState{ value: Bool::False, reason: None};
    valuation[Variable::from(2)] = VariableState{ value: Bool::False, reason: None};
    valuation[Variable::from(4)] = VariableState{ value: Bool::Undef, reason: None};
    valuation[Variable::from(8)] = VariableState{ value: Bool::Undef, reason: None};

    // create the tested clause
    let mut clause = Clause(vec![
        Literal::from(1),
        Literal::from(2),
        Literal::from(4),
        Literal::from(8)]);

    let watched = Literal::from(1);
    assert_eq!(clause.find_new_literal(watched, &valuation), Ok(Literal::from(4)))
}

#[test]
fn find_new_literal_does_not_pick_one_of_the_wl_as_new_wl(){
    let mut valuation= Valuation::new(8);

    valuation[Variable::from(1)] = VariableState{ value: Bool::False, reason: None};
    valuation[Variable::from(2)] = VariableState{ value: Bool::Undef, reason: None};
    valuation[Variable::from(4)] = VariableState{ value: Bool::False, reason: None};
    valuation[Variable::from(8)] = VariableState{ value: Bool::Undef, reason: None};

    // create the tested clause
    let mut clause = Clause(vec![
        Literal::from(1),
        Literal::from(2),
        Literal::from(4),
        Literal::from(8)]);

    let watched = Literal::from(1);
    assert_eq!(clause.find_new_literal(watched, &valuation), Ok(Literal::from(8)))
}

#[test]
fn find_new_literal_returns_ok_with_first_satisfied_literal_when_one_is_found_1(){
    let mut valuation= Valuation::new(8);

    valuation[Variable::from(1)] = VariableState{ value: Bool::False, reason: None};
    valuation[Variable::from(2)] = VariableState{ value: Bool::Undef, reason: None};
    valuation[Variable::from(4)] = VariableState{ value: Bool::True , reason: None};
    valuation[Variable::from(8)] = VariableState{ value: Bool::Undef, reason: None};

    // create the tested clause
    let mut clause = Clause(vec![
        Literal::from(1),
        Literal::from(2),
        Literal::from(4),
        Literal::from(8)]);

    let watched = Literal::from(1);
    assert_eq!(clause.find_new_literal(watched, &valuation), Ok(Literal::from(4)))
}

#[test]
fn find_new_literal_returns_ok_with_first_satisfied_literal_when_one_is_found_2(){
    let mut valuation= Valuation::new(8);

    valuation[Variable::from(1)] = VariableState{ value: Bool::False, reason: None};
    valuation[Variable::from(2)] = VariableState{ value: Bool::Undef, reason: None};
    valuation[Variable::from(4)] = VariableState{ value: Bool::False, reason: None};
    valuation[Variable::from(8)] = VariableState{ value: Bool::True , reason: None};

    // create the tested clause
    let mut clause = Clause(vec![
        Literal::from(1),
        Literal::from(2),
        Literal::from(4),
        Literal::from(8)]);

    let watched = Literal::from(1);
    assert_eq!(clause.find_new_literal(watched, &valuation), Ok(Literal::from(8)))
}

#[test]
fn find_new_literal_tells_what_literal_to_assert_when_it_fails_to_find_a_new_lit(){
    let mut valuation= Valuation::new(8);

    valuation[Variable::from(1)] = VariableState{ value: Bool::False, reason: None};
    valuation[Variable::from(2)] = VariableState{ value: Bool::Undef, reason: None};
    valuation[Variable::from(4)] = VariableState{ value: Bool::False, reason: None};
    valuation[Variable::from(8)] = VariableState{ value: Bool::False, reason: None};

    // create the tested clause
    let mut clause = Clause(vec![
        Literal::from(1),
        Literal::from(2),
        Literal::from(4),
        Literal::from(8)]);

    let watched = Literal::from(1);
    assert_eq!(clause.find_new_literal(watched, &valuation), Err(Literal::from(2)))
}

#[test]
fn find_new_literal_does_not_return_one_that_has_already_been_falsified(){
    /*-
     * a ------------------------------------/--- c
     *                                      /
     *     /------- e ---- f --- -b --- -h +
     *    /                    /           \
     * d /-- g ---------------/             \--- -c
     *
     */
    let mut solver = Solver::new(8);
    solver.add_problem_clause(vec![ 1,-8, 3]); // c0
    solver.add_problem_clause(vec![ 1, 4,-5]); // c1
    solver.add_problem_clause(vec![ 5,-6, 7]); // c2
    solver.add_problem_clause(vec![ 6, 2, 7]); // c3
    solver.add_problem_clause(vec![ 4,-7]);    // c4
    solver.add_problem_clause(vec![-2, 8]);    // c5
    solver.add_problem_clause(vec![-8,-3]);    // c6

    assert_eq!(Ok(()), solver.trail.assign(lit(-4), None));

    let c1_alias = solver.constraints[1].alias();
    let c1 = c1_alias.get_mut().unwrap();
    assert_eq!(Ok(lit(-5)), c1.find_new_literal(lit(4), &solver.trail.valuation));

    assert_eq!(Ok(()), solver.trail.assign(lit(-1), None));

    assert!(solver.trail.valuation.is_false(lit(1)));
    assert!(solver.trail.valuation.is_false(lit(4)));
    assert!(solver.trail.valuation.is_undef(lit(-5)));

    assert_eq!(Err(lit(-5)), c1.find_new_literal(lit(1), &solver.trail.valuation));
}