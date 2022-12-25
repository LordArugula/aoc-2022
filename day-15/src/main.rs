use std::{collections::HashSet, fs};

struct Sensor {
    position: (i32, i32),
    beacon: (i32, i32),
    manhattan_distance: i32,
}
impl Sensor {
    fn new(position: (i32, i32), beacon: (i32, i32)) -> Self {
        Self {
            position,
            beacon,
            manhattan_distance: (beacon.1 - position.1).abs() + (beacon.0 - position.0).abs(),
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Expected input file.");

    let sensors = input.lines().map(|line| {
        let (sensor_str, beacon_str) = line.split_once(':').expect("Expected ':' delimiter");

        let (_, rest) = sensor_str
            .split_once('=')
            .expect("Expected string in format: x=[], y=[]");
        let (x, rest) = rest.split_once(',').unwrap();
        let (_, y) = rest.split_once('=').unwrap();
        let sensor = (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap());

        let (_, rest) = beacon_str
            .split_once('=')
            .expect("Expected string in format: x=[], y=[]");
        let (x, rest) = rest.split_once(',').unwrap();
        let (_, y) = rest.split_once('=').unwrap();
        let beacon = (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap());

        Sensor::new(sensor, beacon)
    });


    let (sensor_positions, beacon_positions): (HashSet<(i32, i32)>, HashSet<(i32, i32)>) = sensors
        .clone()
        .map(|sensor| (sensor.position, sensor.beacon))
        .unzip();

    part_one(sensors, beacon_positions);
}

fn part_one<I>(sensors: I, beacon_positions: HashSet<(i32, i32)>)
where
    I: Iterator<Item = Sensor>,
{
    let row = 2_000_000;

    let mut empty_positions = HashSet::with_capacity(row as usize);
    for sensor in sensors {
        let distance_y = (sensor.position.1 - row).abs();

        if distance_y > sensor.manhattan_distance {
            continue;
        }

        let max_distance_x = sensor.manhattan_distance - distance_y;

        for x in 0..=max_distance_x {
            let position = (sensor.position.0 - x, row);
            if !beacon_positions.contains(&position) {
                empty_positions.insert(position);
            }
        }

        for x in 0..=max_distance_x {
            let position = (sensor.position.0 + x, row);
            if !beacon_positions.contains(&position) {
                empty_positions.insert(position);
            }
        }
    }
    println!("{}", empty_positions.len());
}
