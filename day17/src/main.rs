use std::env;

fn main() {
    let (top_left, bottom_right) = if env::args()
        .skip(1)
        .next()
        .map_or(false, |flag| flag == "--sample")
    {
        ((20, -5), (30, -10))
    } else {
        ((153, -75), (199, -114))
    };

    let x_candidates = find_x_candidates(top_left.0, bottom_right.0);
    let y_candidates = find_y_candidates(bottom_right.1);
    let part_1 = find_max_y(top_left, bottom_right, x_candidates, y_candidates).unwrap();
    println!("Part 1: {:?}", part_1);
}

fn find_x_candidates(from: isize, to: isize) -> Vec<isize> {
    let mut result = vec![];
    let mut candidate = 0;

    while candidate <= to {
        let mut current_position = 0;
        let mut current_speed = candidate;

        while current_speed != 0 && current_position <= to {
            current_position += current_speed;

            if current_position >= from && current_position <= to {
                result.push(candidate);
                break;
            }

            if current_speed > 0 {
                current_speed -= 1;
            } else if current_speed < 0 {
                current_speed += 1;
            }
        }

        candidate += 1;
    }

    result
}

fn find_y_candidates(to: isize) -> Vec<isize> {
    (to..0).rev().map(|target_y| target_y.abs() - 1).collect()
}

fn find_max_y(
    top_left: (isize, isize),
    bottom_right: (isize, isize),
    x_candidates: Vec<isize>,
    y_candidates: Vec<isize>,
) -> Option<isize> {
    x_candidates
        .iter()
        .flat_map(|x| {
            y_candidates
                .iter()
                .filter_map(move |y| simulate((*x, *y), top_left, bottom_right))
        })
        .max()
}

fn simulate(
    speed: (isize, isize),
    top_left: (isize, isize),
    bottom_right: (isize, isize),
) -> Option<isize> {
    let mut current_position = (0, 0);
    let mut current_speed = speed;
    let mut max_y = 0;

    while current_position.0 <= bottom_right.0 {
        current_position = (
            current_position.0 + current_speed.0,
            current_position.1 + current_speed.1,
        );

        if current_position.1 > max_y {
            max_y = current_position.1;
        }

        if current_position.0 >= top_left.0
            && current_position.0 <= bottom_right.0
            && current_position.1 <= top_left.1
            && current_position.1 >= bottom_right.1
        {
            return Some(max_y);
        }

        // Stopped moving in X and it already went past the target area vertically
        if current_speed.0 == 0 && current_position.1 < bottom_right.1 {
            return None;
        } else if current_speed.0 > 0 {
            current_speed.0 -= 1;
        } else if current_speed.0 < 0 {
            current_speed.0 += 1;
        }

        current_speed.1 -= 1;
    }

    None
}
