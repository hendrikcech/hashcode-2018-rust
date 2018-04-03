use std::cmp::Ordering;
use std::collections::BinaryHeap;

use model::*;

impl Ord for Vehicle {
    fn cmp(&self, other: &Vehicle) -> Ordering {
        other.time.cmp(&self.time)
    }
}

impl PartialOrd for Vehicle {
    fn partial_cmp(&self, other: &Vehicle) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
pub struct Solver {
    problem: Problem,
    vehicles: BinaryHeap<Vehicle>,
    rides: Vec<Ride>,
}

impl Solver {
    pub fn new(problem: Problem, vehicles: Vec<Vehicle>, rides: Vec<Ride>) -> Solver {
        let mut vehicle_heap = BinaryHeap::new();
        vehicle_heap.reserve_exact(problem.n_vehicles);
        for vehicle in vehicles {
            vehicle_heap.push(vehicle);
        }

        Solver {
            problem,
            vehicles: vehicle_heap,
            rides,
        }
    }

    pub fn solve(&mut self) -> Vec<Vec<usize>> {
        let mut assignment = Vec::new();
        for _ in 0..self.problem.n_vehicles {
            assignment.push(Vec::new());
        }

        while !self.rides.is_empty() && !self.vehicles.is_empty() {
            let vehicle = &mut self.vehicles.pop().unwrap();

            if let Some(ride_id) = self.best_ride_for_vehicle(vehicle) {
                assignment[vehicle.id].push(ride_id);

                let mut ride = &mut self.rides[ride_id];
                ride.done = true;

                let (_, _, _, total) = travel_time(vehicle, ride);

                vehicle.pos = ride.finish;
                vehicle.time += total;

                self.vehicles.push(vehicle.clone());
            }
        }

        assignment
    }

    fn best_ride_for_vehicle(&mut self, vehicle: &Vehicle) -> Option<usize> {
        let mut best_score: f64 = 0.0;
        let mut best_ride_id = None;
        for ride in &self.rides {
            if ride.done {
                continue;
            }

            let (to_start, waiting, ride_distance, total) = travel_time(vehicle, ride);

            let doable_in_simulation = (vehicle.time + total) < self.problem.sim_steps;
            let doable_before_ride_deadline = (vehicle.time + total) < ride.latest;

            if doable_in_simulation && doable_before_ride_deadline {
                let bonus = if gets_bonus(vehicle, ride) {
                    self.problem.bonus
                } else {
                    0
                };
                let opportunity = bonus + ride_distance;
                let cost = to_start + waiting + ride_distance;
                let score: f64 = opportunity as f64 / cost as f64;

                if best_score < score {
                    best_score = score;
                    best_ride_id = Some(ride.id);
                }
            }
        }

        best_ride_id
    }
}
