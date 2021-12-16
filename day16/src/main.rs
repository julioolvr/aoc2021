use bitvec::prelude::*;

fn main() {
    let file = include_str!("../input.txt").trim();
    let packet = parse(file);
    let part_1 = version_numbers_sum(&packet);
    println!("Part 1: {}", part_1);
}

type Bit = bool;

#[derive(PartialEq, Debug)]
enum Packet {
    Literal(u8, usize),
    Operator(u8, Vec<Packet>),
}

fn parse(input: &'static str) -> Packet {
    parse_packet(&mut parse_bits(input).iter().by_ref()).0
}

fn parse_bits(input: &'static str) -> BitVec {
    input
        .chars()
        .flat_map(|hexa| match hexa {
            '0' => bitvec![0, 0, 0, 0],
            '1' => bitvec![0, 0, 0, 1],
            '2' => bitvec![0, 0, 1, 0],
            '3' => bitvec![0, 0, 1, 1],
            '4' => bitvec![0, 1, 0, 0],
            '5' => bitvec![0, 1, 0, 1],
            '6' => bitvec![0, 1, 1, 0],
            '7' => bitvec![0, 1, 1, 1],
            '8' => bitvec![1, 0, 0, 0],
            '9' => bitvec![1, 0, 0, 1],
            'A' => bitvec![1, 0, 1, 0],
            'B' => bitvec![1, 0, 1, 1],
            'C' => bitvec![1, 1, 0, 0],
            'D' => bitvec![1, 1, 0, 1],
            'E' => bitvec![1, 1, 1, 0],
            'F' => bitvec![1, 1, 1, 1],
            c => panic!("Unexpected hexadecimal character {}", c),
        })
        .collect()
}

fn parse_packet<'a>(bits: &mut impl Iterator<Item = &'a Bit>) -> (Packet, usize) {
    use Packet::*;

    let mut bits_read = 0;

    let version = bits.take(3).fold(0, |acc, bit| acc << 1 | (*bit as u8));
    bits_read += 3;

    let type_id = bits.take(3).fold(0, |acc, bit| acc << 1 | (*bit as u8));
    bits_read += 3;

    let (packet, more_bits_read) = match type_id {
        4 => {
            let (literal, more_bits_read) = consume_literal(bits);
            (Literal(version, literal), more_bits_read)
        }
        _ => {
            let (subpackets, more_bits_read) = parse_subpackets(bits);
            (Operator(version, subpackets), more_bits_read)
        }
    };

    (packet, bits_read + more_bits_read)
}

fn consume_literal<'a>(bits: &mut impl Iterator<Item = &'a Bit>) -> (usize, usize) {
    let mut result = 0;
    let mut bits_read = 0;

    while *bits.next().unwrap() {
        result = result << 1 | (*bits.next().unwrap() as usize);
        result = result << 1 | (*bits.next().unwrap() as usize);
        result = result << 1 | (*bits.next().unwrap() as usize);
        result = result << 1 | (*bits.next().unwrap() as usize);
        bits_read += 5
    }
    bits_read += 1;

    result = result << 1 | (*bits.next().unwrap() as usize);
    result = result << 1 | (*bits.next().unwrap() as usize);
    result = result << 1 | (*bits.next().unwrap() as usize);
    result = result << 1 | (*bits.next().unwrap() as usize);
    bits_read += 4;

    (result, bits_read)
}

fn parse_subpackets<'a>(bits: &mut impl Iterator<Item = &'a Bit>) -> (Vec<Packet>, usize) {
    let (subpackets, bits_read) = if *bits.next().unwrap() {
        parse_subpackets_by_count(bits)
    } else {
        parse_subpackets_by_length(bits)
    };

    (subpackets, bits_read + 1)
}

fn parse_subpackets_by_count<'a>(bits: &mut impl Iterator<Item = &'a Bit>) -> (Vec<Packet>, usize) {
    let count: usize = bits.take(11).fold(0, |acc, bit| acc << 1 | (*bit as usize));

    let mut bits_read = 0;
    let mut packets = vec![];

    for _ in 0..count {
        let (packet, more_bits_read) = parse_packet(bits);
        packets.push(packet);
        bits_read += more_bits_read;
    }

    (packets, bits_read + 11)
}

fn parse_subpackets_by_length<'a>(
    bits: &mut impl Iterator<Item = &'a Bit>,
) -> (Vec<Packet>, usize) {
    let length: usize = bits.take(15).fold(0, |acc, bit| acc << 1 | (*bit as usize));

    let mut bits_read = 0;
    let mut packets = vec![];

    while bits_read < length {
        let (packet, more_bits_read) = parse_packet(bits);
        packets.push(packet);
        bits_read += more_bits_read;
    }

    (packets, bits_read + 15)
}

fn version_numbers_sum(packet: &Packet) -> usize {
    use Packet::*;

    match packet {
        Literal(version, _) => *version as usize,
        Operator(version, subpackets) => {
            let sum: usize = subpackets
                .iter()
                .map(|packet| version_numbers_sum(packet))
                .sum();

            (*version as usize) + sum
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_literal_packet() {
        assert_eq!(parse("D2FE28"), Packet::Literal(6, 2021));
    }

    #[test]
    fn test_parse_operator_with_bit_length() {
        assert_eq!(
            parse("38006F45291200"),
            Packet::Operator(1, vec![Packet::Literal(6, 10), Packet::Literal(2, 20)])
        );
    }

    #[test]
    fn test_parse_operator_with_subpacket_count() {
        assert_eq!(
            parse("EE00D40C823060"),
            Packet::Operator(
                7,
                vec![
                    Packet::Literal(2, 1),
                    Packet::Literal(4, 2),
                    Packet::Literal(1, 3),
                ]
            )
        );
    }

    #[test]
    fn test_added_up_version_numbers() {
        assert_eq!(version_numbers_sum(&parse("8A004A801A8002F478")), 16);
        assert_eq!(
            version_numbers_sum(&parse("620080001611562C8802118E34")),
            12
        );
        assert_eq!(
            version_numbers_sum(&parse("C0015000016115A2E0802F182340")),
            23
        );
        assert_eq!(
            version_numbers_sum(&parse("A0016C880162017C3686B18A3D4780")),
            31
        );
    }
}
