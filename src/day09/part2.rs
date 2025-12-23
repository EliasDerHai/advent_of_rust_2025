use crate::util::point::Point;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    iter::successors,
};

#[derive(Debug)]
enum Field {
    Red,
    Green,
}

pub fn solve_day_09_part_02(input: &str) -> u32 {
    let mut map: HashMap<Point, Field> = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;
    let parsed: Vec<(i32, i32)> = input
        .trim()
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').expect("line must contain ','");
            let x = x.parse().expect("nan");
            let y = y.parse().expect("nan");
            max_x = max_x.max(x);
            max_y = max_y.max(y);
            (x, y)
        })
        .collect();

    let &(mut last_x, mut last_y) = parsed.first().expect("non empty");
    map.insert(Point::new(last_x, last_y), Field::Red);

    for &(x, y) in &parsed {
        for (px, py) in line_points((last_x, last_y), (x, y)).skip(1) {
            if px == last_x && py == last_y {
                // already set; also should be skipped
                continue;
            } else if px == x && py == y {
                // end
                map.insert(Point::new(x, y), Field::Red);
                break;
            } else {
                map.insert(Point::new(px, py), Field::Green);
            }
        }
        last_x = x;
        last_y = y;
    }

    let &(first_x, first_y) = parsed.first().expect("non empty");
    for (px, py) in line_points((first_x, first_y), (last_x, last_y)).skip(1) {
        if px == last_x && py == last_y {
            // already set; also should be skipped
            continue;
        } else if px == first_x && py == first_y {
            // end
            map.insert(Point::new(px, py), Field::Red);
            break;
        } else {
            map.insert(Point::new(px, py), Field::Green);
        }
    }

    let mut stack = VecDeque::new();
    let first = Point::from(get_flood_fill_start(
        *parsed.first().expect("non empty"),
        *parsed.get(2).expect("non empty"),
    ));
    println!("{first}");

    stack.push_back(first);
    while let Some(p) = stack.pop_front() {
        println!("{p}");
        map.entry(p).or_insert(Field::Green);

        p.neighbors().iter().for_each(|&n| {
            if !map.contains_key(&n) {
                stack.push_back(n);
            }
        });
    }

    // print_map(max_x, max_y, &map);

    0
}

fn get_flood_fill_start(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    ((a.0 + b.0) / 2, (a.1 + b.1) / 2)
}

fn line_points(a: (i32, i32), b: (i32, i32)) -> impl Iterator<Item = (i32, i32)> {
    let (x0, y0) = a;
    let (x1, y1) = b;

    let dx = (x1 - x0).signum();
    let dy = (y1 - y0).signum();

    assert!(dx == 0 || dy == 0);

    successors(Some((x0, y0)), move |&(x, y)| {
        if x == x1 && y == y1 {
            None
        } else {
            Some((x + dx, y + dy))
        }
    })
}

fn print_map(max_x: i32, max_y: i32, map: &HashMap<Point, Field>) {
    for y in 0..max_y + 1 {
        for x in 0..max_x + 1 {
            let c = match map.get(&Point::new(x, y)) {
                Option::None => '.',
                Option::Some(Field::Red) => '#',
                Option::Some(Field::Green) => 'X',
            };
            print!("{c}");
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;
    use indoc::indoc;

    #[test]
    fn should_solve_day_09_part_02() {
        let input = read_string("./src/day09/input.txt").unwrap();

        let solution = solve_day_09_part_02(&input);

        // assert_eq!(0, solution);
        println!("{solution}");
    }

    #[test]
    fn should_solve_day_09_part_02_sample() {
        let input = indoc! {"
            7,1
            11,1
            11,7
            9,7
            9,5
            2,5
            2,3
            7,3
        "}
        .trim();

        assert_eq!(24, solve_day_09_part_02(input));
    }
}
