use anyhow::{anyhow, bail};
use std::{
    collections::{HashMap, HashSet},
    convert::TryFrom,
    env,
    iter::FromIterator,
    str::FromStr,
};

/**
 * --- Day 8: Seven Segment Search ---
 *
 * Today's challenge involves a seven-segment digits display that is incorrectly wired. A
 * seven-segment display is made out of seven segments named A to G in this way:
 *
 *   aaaa
 *  b    c
 *  b    c
 *   dddd
 *  e    f
 *  e    f
 *   gggg
 *
 * The input is a list of "signals" where each signal is made of several digits as its input and
 * several digits as its output. Each digit in a signal is represented as a set of segments (e.g.
 * `cgeb`) that are enabled for that digit. The challenge is that the segments in the signal don't
 * correspond to the drawing above - for each signal, each segment has been improperly wired so
 * that, for example, in a given signal the segment c might turn on the segment a in the reference
 * drawing above.
 *
 * The input is parsed into three main structures - a `Signal` which has an input and an output,
 * each of which is a vec of `Digit`s. Each `Digit` is made of a set of enabled `Segment`s, and
 * `Segment` is simply an enum of letters A through G.
 *
 * Part 1 asks for what we call here "trivial" digits, more specifically how many are in the
 * signals' outputs. Trivial digits are those that, given the amount of segments present in a digit,
 * it's obvious which number they represent. For instance, if there are two segments enabled for a
 * digit, that digit could only possibly be a one. The other trivial numbers are 4 (4 segments), 7
 * (3 segments) and 8 (7 segments). Counting the segments for each digit in the output is enough to
 * arrive to the solution.
 *
 * Part 2 asks for the sum of all the fully-decoded outputs. In order to figure out the exact
 * mapping for each signal, we'll consider that the schema above is the "correct" position for each
 * segment - it gives each segment a name. Then let's consider a signal that looks like this:
 *
 *  dddd
 * e    a
 * e    a
 *  ffff
 * g    b
 * g    b
 *  cccc
 *
 * In this schema, the A segment is mapped to the D "input". In this signal, turning on the A and B
 * input will highlight the C and F "output" segments, forming a 1.
 *
 * The `Guesses` struct is used to find this input/output mapping. We start by saying that each
 * output could be *any* of the inputs - we don't have any information so far. This is represented
 * by a `HashMap` from a `Segment` to a `HashSet<Segment>`. Then we start iterating over each digit
 * in the signal. Given the count of segments on each digit it can only be one of a few possible
 * numbers. Those numbers define the only possible target segment for each of the segments in that
 * digit. For instance, with the mapping above, a digit `ab` can only be a 1. In the target, a 1 is
 * formed by C and F. So now we know that C and F must each be mapped to either A or B. This is
 * narrowed down by the intersection of the current possibilities for each segment (right now C and
 * F could be anything) and the ones that correspond to the current digit (A and B). With that
 * narrowed down we continue with the next digit. Once we identified one input/output pair (let's
 * say that input A corresponds to output C), we can remove A as a possibility from all other
 * targets. We continue looping narrowing down the possibilities this way until all targets have
 * only one input, and use that to decode the output of the signal.
 */

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

    let mut signals: Vec<Signal> = file
        .lines()
        .filter(|line| line.trim() != "")
        .map(|line| line.parse().unwrap())
        .collect();

    let part_1: usize = signals
        .iter()
        .map(|signal| signal.trivial_digits_in_output())
        .sum();

    println!("Part 1: {}", part_1);

    let part_2: usize = signals
        .iter_mut()
        .map(|signal| signal.decode_output())
        .sum();

    println!("Part 2: {}", part_2);
}

#[derive(Debug)]
struct Signal {
    input: Vec<Digit>,
    output: Vec<Digit>,
    guesses: Guesses,
}

impl Signal {
    fn new(input: Vec<Digit>, output: Vec<Digit>) -> Self {
        Signal {
            input,
            output,
            guesses: Guesses::new(),
        }
    }

    fn digits(&self) -> impl Iterator<Item = &Digit> {
        self.input.iter().chain(self.output.iter())
    }

    fn decode_output(&mut self) -> usize {
        let digits: Vec<Digit> = self.digits().cloned().collect();
        let mut digits_iteration = digits.into_iter().cycle();

        while !self.guesses.all_guessed() {
            use Segment::{A, B, C, D, E, F, G};

            let next_input = digits_iteration.next().unwrap();

            if let Some(value) = next_input.trivial_value() {
                match value {
                    1 => {
                        self.guesses.guess(&C, &next_input.enabled_segments);
                        self.guesses.guess(&F, &next_input.enabled_segments);
                    }
                    4 => {
                        self.guesses.guess(&B, &next_input.enabled_segments);
                        self.guesses.guess(&C, &next_input.enabled_segments);
                        self.guesses.guess(&D, &next_input.enabled_segments);
                        self.guesses.guess(&F, &next_input.enabled_segments);
                    }
                    7 => {
                        self.guesses.guess(&A, &next_input.enabled_segments);
                        self.guesses.guess(&C, &next_input.enabled_segments);
                        self.guesses.guess(&F, &next_input.enabled_segments);
                    }
                    8 => {
                        self.guesses.guess(&A, &next_input.enabled_segments);
                        self.guesses.guess(&B, &next_input.enabled_segments);
                        self.guesses.guess(&C, &next_input.enabled_segments);
                        self.guesses.guess(&D, &next_input.enabled_segments);
                        self.guesses.guess(&E, &next_input.enabled_segments);
                        self.guesses.guess(&F, &next_input.enabled_segments);
                        self.guesses.guess(&G, &next_input.enabled_segments);
                    }
                    _ => panic!("Unexpected 'trivial' value {}", value),
                }
            } else {
                match next_input.enabled_segments.len() {
                    5 => {
                        self.guesses.guess(&A, &next_input.enabled_segments);
                        self.guesses.guess(&D, &next_input.enabled_segments);
                        self.guesses.guess(&G, &next_input.enabled_segments);
                    }
                    6 => {
                        self.guesses.guess(&A, &next_input.enabled_segments);
                        self.guesses.guess(&B, &next_input.enabled_segments);
                        self.guesses.guess(&F, &next_input.enabled_segments);
                        self.guesses.guess(&G, &next_input.enabled_segments);
                    }
                    n => panic!("Unexpected non-trivial count of segments {}", n),
                }
            }
        }

        self.output
            .iter()
            .map(|digit| digit.decode(&self.guesses).to_string())
            .collect::<String>()
            .parse()
            .unwrap()
    }

    fn trivial_digits_in_output(&self) -> usize {
        self.output
            .iter()
            .filter(|digit| digit.is_trivial())
            .count()
    }
}

impl FromStr for Signal {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('|');
        let input = split
            .next()
            .ok_or(anyhow!("Couldn't extract input digits from signal string"))?;
        let output = split
            .next()
            .ok_or(anyhow!("Couldn't extract output digits from signal string"))?;

        let input = input
            .split(' ')
            .map(|digit| digit.trim())
            .filter(|digit| digit != &"")
            .map(|digit| digit.parse())
            .collect::<anyhow::Result<Vec<Digit>>>()?;
        let output = output
            .split(' ')
            .map(|digit| digit.trim())
            .filter(|digit| digit != &"")
            .map(|digit| digit.parse())
            .collect::<anyhow::Result<Vec<Digit>>>()?;

        Ok(Signal::new(input, output))
    }
}

#[derive(Debug)]
struct Guesses {
    guesses: HashMap<Segment, HashSet<Segment>>,
}

impl Guesses {
    fn new() -> Self {
        use Segment::{A, B, C, D, E, F, G};

        Guesses {
            guesses: HashMap::from_iter([
                (A, HashSet::from_iter([A, B, C, D, E, F, G])),
                (B, HashSet::from_iter([A, B, C, D, E, F, G])),
                (C, HashSet::from_iter([A, B, C, D, E, F, G])),
                (D, HashSet::from_iter([A, B, C, D, E, F, G])),
                (E, HashSet::from_iter([A, B, C, D, E, F, G])),
                (F, HashSet::from_iter([A, B, C, D, E, F, G])),
                (G, HashSet::from_iter([A, B, C, D, E, F, G])),
            ]),
        }
    }

    fn guess(&mut self, segment: &Segment, possible_segments: &HashSet<Segment>) {
        let guesses = self.guesses.get_mut(segment).unwrap();
        *guesses = guesses.intersection(possible_segments).copied().collect();
        if guesses.len() == 1 {
            let guessed_segment = guesses.iter().next().unwrap().clone();
            for (other_segment, guesses) in self.guesses.iter_mut() {
                if segment != other_segment {
                    guesses.remove(&guessed_segment);
                }
            }
        }
    }

    fn decode(&self, segment: &Segment) -> Segment {
        if !self.all_guessed() {
            panic!("Tried to decode a segment without having guessed all mappings yet");
        }

        self.guesses
            .iter()
            .find_map(|(k, v)| {
                if v.iter().next().unwrap() == segment {
                    Some(k)
                } else {
                    None
                }
            })
            .unwrap()
            .clone()
    }

    fn all_guessed(&self) -> bool {
        self.guesses
            .values()
            .all(|remaining_guesses| remaining_guesses.len() == 1)
    }
}

#[derive(Debug, Clone)]
struct Digit {
    enabled_segments: HashSet<Segment>,
}

impl Digit {
    fn trivial_value(&self) -> Option<usize> {
        match self.enabled_segments.len() {
            2 => Some(1),
            3 => Some(7),
            4 => Some(4),
            7 => Some(8),
            _ => None,
        }
    }

    fn is_trivial(&self) -> bool {
        self.trivial_value().is_some()
    }

    fn decode(&self, guesses: &Guesses) -> usize {
        if let Some(value) = self.trivial_value() {
            return value;
        }

        let decoded_segments: HashSet<Segment> = self
            .enabled_segments
            .iter()
            .map(|segment| guesses.decode(segment))
            .collect();

        match decoded_segments.len() {
            5 => {
                if decoded_segments.contains(&Segment::E) {
                    2
                } else if decoded_segments.contains(&Segment::C) {
                    3
                } else {
                    5
                }
            }
            6 => {
                if !decoded_segments.contains(&Segment::D) {
                    0
                } else if decoded_segments.contains(&Segment::E) {
                    6
                } else {
                    9
                }
            }
            n => panic!("Unexpected non-trivial segment count {}", n),
        }
    }
}

impl FromStr for Digit {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Digit {
            enabled_segments: s
                .chars()
                .map(|c| Segment::try_from(c))
                .collect::<anyhow::Result<HashSet<Segment>>>()?,
        })
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl TryFrom<char> for Segment {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let segment = match value {
            'a' => Segment::A,
            'b' => Segment::B,
            'c' => Segment::C,
            'd' => Segment::D,
            'e' => Segment::E,
            'f' => Segment::F,
            'g' => Segment::G,
            _ => bail!("Invalid char for segment: {}", value),
        };

        Ok(segment)
    }
}
