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
            manhattan_distance: manhattan_distance(position, beacon),
        }
    }
}

fn manhattan_distance(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
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

    let beacon_positions = sensors
        .clone()
        .map(|sensor| sensor.beacon)
        .collect::<HashSet<(i32, i32)>>();
    
    part_one(sensors.clone(), beacon_positions);
    part_two(sensors);
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

fn part_two<I>(sensors: I)
where
    I: Iterator<Item = Sensor> + Clone,
{
    let width = 4_000_000;
    let height = 4_000_000;

    let mut y = 0;
    while y < height {
        let mut x = 0;
        while x < width {
            let mut within_sensors = false;
            for sensor in sensors.clone() {
                let distance = manhattan_distance(sensor.position, (x, y));
                if distance <= sensor.manhattan_distance {
                    // if we are inside the area where a sensor detected a 
                    // beacon, jump to the next x coord outside
                    let distance_y = (y - sensor.position.1).abs();
                    let max_distance_x = sensor.manhattan_distance - distance_y;
                    x = sensor.position.0 + max_distance_x;
                    within_sensors = true;

                    if x >= width {
                        break;
                    }
                }
            }

            if within_sensors == false {
                // found distress beacon!
                println!("{}", x as u64 * 4_000_000 + y as u64);
            }

            x += 1;
        }
        y += 1;
    }
}
