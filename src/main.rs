extern crate rsolve;

extern crate argparse;
extern crate xz2;
extern crate bzip2;
extern crate flate2;
extern crate time;
use rsolve::*;

use argparse::*;

use std::io::{stdin, BufRead, BufReader};
use std::fs::File;
use xz2::bufread::XzDecoder;
use bzip2::bufread::BzDecoder;
use flate2::bufread::GzDecoder;
//use time::PreciseTime;

use std::time::*;

// TODO: Test - Solver.rs -> partial restarts
// TODO: Test - dimacs.rs -> *
// TODO: Dev  - LRB
// TODO: Dev  - LCM
// TODO: Dev  - subsumption
// TODO: Dev  - self-subsuming resolution
// TODO: Dev  - variable-elimination
// TODO: Dev  - unhiding
// TODO: Ref. - Maybe create traits Backtracking and Restart
// TODO: Ref. - Add getters for public fields of the solver

/// This simple structure encapsulates the options and arguments that are passed to the solver using
/// the command line interface (cli).
struct CliArgs {
    filename   : Option<String>,
    print_model: bool,
    drat       : bool  // See: ''Trimming while Checking Clausal Proofs'' -- Heule, Hunt, Wetzler (FMCAD), 2013
}

fn main() {
    let now = SystemTime::now();

    print_header();
    let args = arguments();
    let mut lines = input(&args).lines();
    let mut solver = parse_header(&mut lines);

    solver.drat = args.drat;
    /*
    println!("start load clauses");
    let start = PreciseTime::now();
    */
    load_clauses(&mut solver, &mut lines);
    // solver.preprocess(); TODO
    println!("clauses : {}", solver.clauses.len());
    /*
    let end = PreciseTime::now();
    println!("end load clauses {}", start.to(end));*/

    let satisfiable = solver.solve();

    let duration = match now.elapsed() {
        Ok(t) => t,
        Err(t)=> t.duration() // totally unlikely !!!
    };

    print_result(&solver,&args, satisfiable, &duration);
}

fn print_header() {
    println!("c ******************************************************************************");
    println!("c This is the `rsolve` SAT solver version 0.1.0");
    println!("c ------------------------------------------------------------------------------");
    println!("c Copyright 2017 Xavier Gillard, LVL -- Université Catholique de Louvain (BE)");
    println!("c This software is licensed to you under the terms of the MIT license");
    println!("c ==============================================================================");
}

fn print_result(solver: &Solver, config: &CliArgs, satisfiable: bool, elapsed: &Duration){
    if satisfiable {
        println!("s SATISFIABLE");

        if config.print_model { print_model(solver); }

    } else {
        println!("s UNSATISFIABLE");
    }

    let elapsed_time = elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9;
    println!("c ------------------------------------------------------------------------------");
    println!("c nb_conflicts {}"  , solver.nb_conflicts);
    println!("c nb_restarts  {}"  , solver.nb_restarts);
    println!("c elapsed time {:.3} s", elapsed_time);
    println!("c removed {}", solver.removed);
    println!("c ******************************************************************************");
}

fn print_model(solver: &Solver) {
    let mut model = String::from("v ");
    for v in 1..solver.nb_vars()+1 {
        let var_value = match solver.get_value(lit(v as iint)) {
            Bool::True =>  v as isize,
            Bool::False=>-(v as isize),
            Bool::Undef=> panic!("The problem is supposed to be SOLVED ! How can it be ?")
        };

        model.push_str(&format!("{} ", &var_value.to_string()));
    }
    model.push_str("0");

    println!("{}", model);
}

/// This function parses the command line arguments of the program and returns an object
/// representing these arguments.
fn arguments() -> CliArgs {
    let mut options= CliArgs { filename: None, print_model: false, drat: false };

    // This is where we actually handle the command line arguments with Argparse (like we'd do in
    // python3). Note, this scope is necessary since it allows us to close the borrow scope for
    // options and hence, to return it.
    {
        let mut parser = ArgumentParser::new();
        parser.set_description("rsolve, a simple yet performant propositional SAT solver ");

        // --- Declare the actual options here ----------------------------------------------------
        parser.refer(&mut options.filename)
            .add_argument("input_file",
                            StoreOption,
                            "The input file. This should be a dimacs cnf file which may be \
                                   compressed with bz2 (bzip2) , gz (gzip) or xz (lzma)");

        parser.refer(&mut options.print_model)
            .add_option(&["-p", "--print-model"],
                        StoreTrue,
                        "Prints a model when the instance is proven satisfiable.");

        parser.refer(&mut options.drat)
            .add_option(&["-d", "--drat"],
                        StoreTrue,
                        "Prints a proof of unsatisfiability in DRAT format (aka UNSAT certificate).");

        parser.parse_args_or_exit();
    }

    return options;
}

/// This function returns the BufRead reader which can be used to iterate over all the lines of the
/// DIMACS CNF input. If the cli-args did not provide any input file, then stdin is used to read the
/// problem. Otherwise, the input file is read (and potentially unpacked).
/// Whenever the filename ends with one of .bz2, .gz, .gzip, .lzma or .xz, the input file will be
/// de-compressed using the ad-hoc decoder. In all other cases, the input file is assumed to be in
/// plaintext format.
fn input(args : &CliArgs) -> Box<BufRead> {
    match args.filename {
        None => {
            let input = stdin();
            return Box::new(BufReader::new(input))
        },
        Some(ref fname) => {
            let file = File::open(fname).expect(&format!("{} does not exists", fname));
            let input = BufReader::new(file);

            let canonical = fname.to_lowercase();
            if canonical.ends_with(".bz2") {
                let decoder = BzDecoder::new(input);
                return Box::new(BufReader::new(decoder));
            }
            if canonical.ends_with(".gz") || canonical.ends_with(".gzip") {
                let decoder = GzDecoder::new(input).expect("problem with gzip file");
                return Box::new(BufReader::new(decoder));
            }
            if canonical.ends_with(".xz") || canonical.ends_with(".lzma") {
                let decoder = XzDecoder::new(input);
                return Box::new(BufReader::new(decoder));
            }

            // it is assumed that the input file is in plain text (.cnf, .dimacs, .txt, ...)
            return Box::new(input);
        }
    }
}