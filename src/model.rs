// (row, col)
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct Pos(pub usize, pub usize);

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Ride {
    pub id: usize,
    pub start: Pos,
    pub finish: Pos,
    pub earliest: usize,
    pub latest: usize,
    pub done: bool,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Vehicle {
    pub id: usize,
    pub pos: Pos,
    pub time: usize,
}

#[derive(Debug, Clone)]
pub struct Problem {
    pub rows: usize,
    pub cols: usize,
    pub n_vehicles: usize,
    pub n_rides: usize,
    pub bonus: usize,
    pub sim_steps: usize,
}

pub fn travel_time(vehicle: &Vehicle, ride: &Ride) -> (usize, usize, usize, usize) {
    let to_start = distance(&vehicle.pos, &ride.start);
    let w = ride.earliest as isize - (vehicle.time + to_start) as isize;
    let waiting: usize = if w <= 0 { 0 } else { w as usize };
    let ride_distance = distance(&ride.start, &ride.finish);
    let total = to_start + waiting + ride_distance;

    (to_start, waiting, ride_distance, total)
}

pub fn gets_bonus(vehicle: &Vehicle, ride: &Ride) -> bool {
    let to_start = distance(&vehicle.pos, &ride.start);
    let waiting = ride.earliest as isize - (vehicle.time + to_start) as isize;
    waiting >= 0
}

fn distance(x: &Pos, y: &Pos) -> usize {
    ((x.0 as isize - y.0 as isize).abs() + (x.1 as isize - y.1 as isize).abs()) as usize
}
