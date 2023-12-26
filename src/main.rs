mod grid;
mod parser;
mod solutions;
mod utils;

#[macro_use]
extern crate lazy_static;

use std::{
    cmp::Ordering,
    time::{Duration, Instant},
};

use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use rayon::prelude::*;

#[derive(Parser)]
#[command(name = "aoc2023")]
#[command(author = "Stephen Weinberg")]
#[command(about = "Solves Advent of Code 2023", long_about = None)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        day: usize,
        problem: usize,
        #[arg(long)]
        input: Option<String>,
    },
    RunAll {
        #[arg(long)]
        parallel: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Run {
            day,
            problem,
            input,
        } => run(day, problem, input),
        Commands::RunAll { parallel } => run_all(parallel),
    }
}

fn run(day: usize, problem: usize, input: Option<String>) -> Result<()> {
    let flag_input = input
        .as_ref()
        .map(|x| std::fs::read_to_string(x).context("failed to read input file"))
        .transpose()?;

    let (ans, duration) = run_problem(day, problem, flag_input.as_deref())?;

    println!("{}", ans);
    println!("\nComputed in {:?}", duration);

    Ok(())
}

fn run_all(parallel: bool) -> Result<()> {
    let days = {
        let mut d: Vec<usize> = solutions::SOLUTIONS.keys().copied().collect();
        d.sort_unstable();
        d
    };

    let mut times: Vec<_> = if parallel {
        days.par_iter()
            .copied()
            .flat_map(|day| [(day, 1), (day, 2)])
            .map(|(day, problem)| (day, problem, run_problem(day, problem, None)))
            .collect()
    } else {
        days.iter()
            .copied()
            .flat_map(|day| [(day, 1), (day, 2)])
            .map(|(day, problem)| (day, problem, run_problem(day, problem, None)))
            .collect()
    };

    // Sort by duration in descending order. Errors are sorted at the bottom
    // by day/part.
    times.sort_by(|a, b| match (&a.2, &b.2) {
        (Ok(a_res), Ok(b_res)) => a_res.1.cmp(&b_res.1).reverse(),
        (Err(_), Err(_)) => a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)),
        (Ok(_), Err(_)) => Ordering::Less,
        (Err(_), Ok(_)) => Ordering::Greater,
    });

    for (day, problem, res) in &times {
        if let Ok((_, duration)) = res {
            println!("{:2}-{}: {:?}", day, problem, duration);
        } else {
            println!("{:2}-{}: ERROR", day, problem);
        }
    }

    Ok(())
}

fn run_problem(day: usize, problem: usize, input: Option<&str>) -> Result<(String, Duration)> {
    let solution = solutions::SOLUTIONS
        .get(&day)
        .ok_or(anyhow!("unknown day: {}", day))?;
    let problem_fn = match problem {
        1 => solution.problem1,
        2 => solution.problem2,
        _ => return Err(anyhow!("unknown problem number: {}", problem)),
    };

    let input = input.unwrap_or(solution.input);

    let start = Instant::now();
    let ans = problem_fn(input).context("problemfn failed")?;
    let end = Instant::now();

    return Ok((ans, end.duration_since(start)));
}
