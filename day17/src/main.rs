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

    let x_candidates = find_x_candidates(bottom_right.0);
    let y_candidates = find_y_candidates(bottom_right.1);
    let part_1 = find_max_y(&y_candidates);
    println!("Part 1: {:?}", part_1);
    let part_2 = find_all_velocities(top_left, bottom_right, &x_candidates, &y_candidates).len();
    println!("Part 2: {:?}", part_2);
}

fn find_x_candidates(to: isize) -> Vec<isize> {
    (0..=to).collect()
}

fn find_y_candidates(to: isize) -> Vec<isize> {
    (to..-to).collect()
}

fn find_max_y(y_candidates: &Vec<isize>) -> isize {
    let max_y = y_candidates.iter().max().unwrap();
    max_y * (max_y + 1) / 2
}

fn find_all_velocities(
    top_left: (isize, isize),
    bottom_right: (isize, isize),
    x_candidates: &Vec<isize>,
    y_candidates: &Vec<isize>,
) -> Vec<(isize, isize)> {
    x_candidates
        .iter()
        .copied()
        .flat_map(|x| {
            y_candidates.iter().copied().filter_map(move |y| {
                if simulate((x, y), top_left, bottom_right) {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn simulate(speed: (isize, isize), top_left: (isize, isize), bottom_right: (isize, isize)) -> bool {
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
            return true;
        }

        // Stopped moving in X and it already went past the target area vertically
        if current_speed.0 == 0 && current_position.1 < bottom_right.1 {
            return false;
        } else if current_speed.0 > 0 {
            current_speed.0 -= 1;
        } else if current_speed.0 < 0 {
            current_speed.0 += 1;
        }

        current_speed.1 -= 1;
    }

    false
}
