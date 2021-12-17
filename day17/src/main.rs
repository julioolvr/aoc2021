use std::env;

/**
 * --- Day 17: Trick Shot ---
 *
 * The program has to calculate different initial velocities for a projectile that has to show up
 * within a given area in any number of integer steps. The input of the program is the boundaries
 * of this area. The projectile moves in X and Y given some initial speed (X increases to the right,
 * Y increases upwards). It slows down to 0 in X, and it gets gravity applied in Y (decreasing Y
 * velocity by 1 in each step, going faster and faster in the negative direction).
 *
 * The first part of the problem asks for the highest possible height Y can achieve. Since X and Y
 * change position and velocity independently, it is possible to calculate this ignoring X. There's
 * one caveat: we have to assume there's some velocity in X that will make it slow down to 0 within
 * the boundaries given. This is the case for the inputs. Given that, we can start with that
 * velocity in X and shoot as high as we want to - it will slow down to 0 within the given area so
 * the only challenge is ensuring it passes through the area in Y. One property of the movement in Y
 * is that regardless of the initial upwards speed, it will always go through 0 again with that
 * initial speed in the negative direction - 1. So we can calculate how fast it can go when it goes
 * downwards. In the sample the lower bound for Y is -10. If it starts at 0 with Y speed -10, it
 * will barely hit the area. -11 and lower won't work. So we turn that in the opposite direction and
 * subtract 1 and it gives us 9 as the starting velocity upward. To calculate the maximum height it
 * will be 9+8+7+6+... or a triangular number. This is calculated in `find_max_y`, and gives the
 * result for part 1.
 *
 * For part 2 we simply simulate. We need some lower and upper bounds for the speeds of X and Y.
 * For X, it has to start at least at 0 to go *somewhere*. The upper bound is the right-most side of
 * the target area - any more and it would go past in a single step. For Y it's similar, except that
 * gravity means we can shoot in the opposite direction of the target area too. So the minimum value
 * is the lower end of the target area as explained before, and the upper value is the one we found
 * in part 1 (which in turn we used to calculate the maximum height). Then we simulate all
 * combinations. It surely is possible to optimize these candidates, but the program runs fast
 * enough with that simple approach (< 5ms).
 */

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
    (1..=to).collect()
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
