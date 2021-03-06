mod model;
mod simulation;
mod solver;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use model::*;
use simulation::*;
use solver::*;

fn main() {
    let root = env!("CARGO_MANIFEST_DIR");
    let input_path = Path::new(&root).join("input/").to_path_buf();

    let mut sum = 0;

    sum += solve_and_score(&input_path.join("a_example.in"));
    sum += solve_and_score(&input_path.join("b_should_be_easy.in"));
    sum += solve_and_score(&input_path.join("c_no_hurry.in"));
    sum += solve_and_score(&input_path.join("d_metropolis.in"));
    sum += solve_and_score(&input_path.join("e_high_bonus.in"));

    println!("Total score: {}", sum);
}

fn solve_and_score(path: &Path) -> usize {
    let (problem, vehicles, rides) = parse_input_file(path);

    let mut solver = Solver::new(problem.clone(), vehicles.clone(), rides.clone());
    let assignment = solver.solve();

    let simulation = Simulation::new(problem, vehicles, rides);
    let (score, errors) = simulation.score(&assignment);
    if !errors.is_empty() {
        for error in errors {
            println!("{}", error);
        }
    }

    println!("Score {:?}: {}", path.file_name().unwrap(), score);

    score
}

fn parse_input_file(path: &Path) -> (Problem, Vec<Vehicle>, Vec<Ride>) {
    let mut f = File::open(&path).unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let header = lines.next().unwrap();

    let p: Vec<usize> = read_line(header);

    let mut rides = Vec::new();

    for (ride_id, line) in lines.enumerate() {
        let l = read_line(line);
        let v = Ride {
            id: ride_id,
            start: Pos(l[0], l[1]),
            finish: Pos(l[2], l[3]),
            earliest: l[4],
            latest: l[5],
            done: false,
        };
        rides.push(v);
    }

    let mut vehicles = Vec::new();
    for i in 0..p[2] {
        let v = Vehicle {
            id: i,
            pos: Pos(0, 0),
            time: 0,
        };
        vehicles.push(v);
    }

    let problem = Problem {
        rows: p[0],
        cols: p[1],
        n_vehicles: p[2],
        n_rides: p[3],
        bonus: p[4],
        sim_steps: p[5],
    };

    (problem, vehicles, rides)
}

#[allow(dead_code)]
fn parse_output_file(path: &Path) -> Vec<Vec<usize>> {
    let mut f = File::open(&path).unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    let mut assignment = Vec::new();

    for line in input.lines() {
        let p = read_line(line);
        if p.is_empty() {
            println!("p.is_empty()");
            continue;
        }
        if p[0] == 0 {
            continue;
        }
        let mut rides = Vec::with_capacity(p[0]);

        for i in 0..p[0] {
            rides.push(p[i + 1]);
        }

        assignment.push(rides);
    }

    assignment
}

fn read_line(line: &str) -> Vec<usize> {
    line.trim().split(' ').map(|v| v.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simulation_b() {
        let input_file = Path::new("b_should_be_easy.in");
        let output_file = Path::new("b_should_be_easy 173950.in");
        let score = run_and_cmp(input_file, output_file);

        assert_eq!(score, 173950);
    }

    #[test]
    fn simulation_d() {
        let input_file = Path::new("d_metropolis.in");
        let output_file = Path::new("d_metropolis 10420438.in");
        let score = run_and_cmp(input_file, output_file);

        assert_eq!(score, 10420438);
    }

    fn run_and_cmp(input_file: &Path, output_file: &Path) -> usize {
        let root = env!("CARGO_MANIFEST_DIR");
        let input = Path::new(&root).join("input/").join(input_file);
        let output = Path::new(&root).join("output/").join(output_file);

        let (problem, vehicles, rides) = parse_input_file(&input);

        let assignment = parse_output_file(&output);

        let simulation = Simulation::new(problem, vehicles, rides);
        let (score, errors) = simulation.score(assignment);

        assert_eq!(errors.len(), 0);

        score
    }
}
