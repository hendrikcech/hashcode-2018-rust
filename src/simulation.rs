use model::*;

pub struct Simulation {
    problem: Problem,
    rides: Vec<Ride>,
    vehicles: Vec<Vehicle>,
}

impl Simulation {
    pub fn new(problem: Problem, vehicles: Vec<Vehicle>, rides: Vec<Ride>) -> Simulation {
        Simulation {
            problem: problem,
            rides: rides,
            vehicles: vehicles,
        }
    }

    pub fn score(mut self, assignment: Vec<Vec<usize>>) -> (usize, Vec<String>) {
        let mut score = 0;
        let mut errors = Vec::new();

        'outer: for (vehicle_id, vehicle_assignment) in assignment.iter().enumerate() {
            for ride_id in vehicle_assignment {
                let mut vehicle = &mut self.vehicles[vehicle_id];
                let mut ride = &mut self.rides[*ride_id];

                let (_, _, ride_distance, total) = travel_time(&vehicle, &ride);

                if ride.done {
                    let error = String::from(format!(
                        "Ride {} assigned to {} already done.",
                        ride_id, vehicle_id
                    ));
                    errors.push(error);
                    continue 'outer;
                }

                if vehicle.time + total > self.problem.sim_steps {
                    let error = String::from(format!(
                        "Ride {} not finished by {} before end of simulation.",
                        ride_id, vehicle_id
                    ));
                    errors.push(error);
                    continue 'outer;
                }

                if vehicle.time + total >= ride.latest {
                    let error = String::from(format!(
                        "Ride {} not finished by {} before ride deadline: {:?}, {:?}, {}",
                        ride_id, vehicle_id, ride, vehicle, total
                    ));
                    errors.push(error);
                    continue 'outer;
                }

                let bonus = if gets_bonus(&vehicle, &ride) {
                    self.problem.bonus
                } else {
                    0
                };

                ride.done = true;
                vehicle.pos = ride.finish;
                vehicle.time += total;

                score += bonus + ride_distance;
            }
        }

        (score, errors)
    }
}
