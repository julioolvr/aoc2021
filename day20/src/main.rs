use std::{collections::HashMap, env, fmt};

use bitvec::prelude::*;

fn main() {
    let file = if env::args()
        .skip(1)
        .next()
        .map_or(false, |flag| flag == "--sample")
    {
        include_str!("../sample.txt")
    } else {
        include_str!("../input.txt")
    };

    let mut lines = file.lines();
    let algorithm: BitVec = lines.next().unwrap().chars().map(|c| c == '#').collect();
    lines.next();
    let image = parse_image(&mut lines);
    let image = image.decompress_step(&algorithm);
    let image = image.decompress_step(&algorithm);
    let part_1 = image.light_pixels_count();
    println!("Part 1: {}", part_1);
}

#[derive(Hash, PartialEq, Eq, Debug)]
struct Coordinates {
    x: isize,
    y: isize,
}

impl From<(isize, isize)> for Coordinates {
    fn from((x, y): (isize, isize)) -> Self {
        Coordinates { x, y }
    }
}

struct Image {
    pixels: PixelMap,
    empty_pixel_value: bool,
}

impl Image {
    fn new(pixel_coordinates: PixelMap, empty_pixel_value: bool) -> Self {
        Image {
            pixels: pixel_coordinates,
            empty_pixel_value,
        }
    }

    fn light_pixels_count(&self) -> usize {
        self.pixels.pixels.values().filter(|pixel| **pixel).count()
    }
}

struct PixelMap {
    pixels: HashMap<Coordinates, bool>,
}

impl PixelMap {
    fn top_left(&self) -> Coordinates {
        (
            self.pixels
                .keys()
                .map(|coordinates| coordinates.x)
                .min()
                .unwrap(),
            self.pixels
                .keys()
                .map(|coordinates| coordinates.y)
                .min()
                .unwrap(),
        )
            .into()
    }

    fn bottom_right(&self) -> Coordinates {
        (
            self.pixels
                .keys()
                .map(|coordinates| coordinates.x)
                .max()
                .unwrap(),
            self.pixels
                .keys()
                .map(|coordinates| coordinates.y)
                .max()
                .unwrap(),
        )
            .into()
    }
}

impl From<HashMap<Coordinates, bool>> for PixelMap {
    fn from(pixels: HashMap<Coordinates, bool>) -> PixelMap {
        PixelMap { pixels }
    }
}

fn parse_image(lines: &mut dyn Iterator<Item = &str>) -> Image {
    let pixel_coordinates: HashMap<Coordinates, bool> = lines
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as isize, y as isize).into(), c == '#'))
        })
        .collect();

    Image::new(pixel_coordinates.into(), false)
}

impl Image {
    fn decompress_step(&self, algorithm: &BitVec) -> Image {
        let mut new_pixels = HashMap::new();
        let top_left = self.top_left();
        let bottom_right = self.bottom_right();

        for pixel_y in top_left.y - 2..=bottom_right.y + 2 {
            for pixel_x in top_left.x - 2..=bottom_right.x + 2 {
                let mut algorithm_key = bitvec![];

                for y in pixel_y - 1..=pixel_y + 1 {
                    for x in pixel_x - 1..=pixel_x + 1 {
                        let value = self
                            .pixels
                            .pixels
                            .get(&(x, y).into())
                            .copied()
                            .unwrap_or(self.empty_pixel_value);
                        algorithm_key.push(value);
                    }
                }

                let algorithm_key: usize = algorithm_key
                    .into_iter()
                    .fold(0, |acc, bit| (acc << 1) | bit as usize);

                new_pixels.insert((pixel_x, pixel_y).into(), algorithm[algorithm_key]);
            }
        }

        let empty_pixel_value = if self.empty_pixel_value {
            *algorithm.last().unwrap()
        } else {
            *algorithm.first().unwrap()
        };

        Image::new(new_pixels.into(), empty_pixel_value)
    }

    fn top_left(&self) -> Coordinates {
        self.pixels.top_left()
    }

    fn bottom_right(&self) -> Coordinates {
        self.pixels.bottom_right()
    }
}

impl fmt::Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let top_left = self.top_left();
        let bottom_right = self.bottom_right();

        for y in top_left.y..=bottom_right.y {
            for x in top_left.x..=bottom_right.x {
                if *self
                    .pixels
                    .pixels
                    .get(&(x, y).into())
                    .unwrap_or(&self.empty_pixel_value)
                {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }

            writeln!(f, "")?;
        }

        Ok(())
    }
}
