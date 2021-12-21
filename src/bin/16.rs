#[derive(Debug, PartialEq, Eq)]
enum PacketData {
    Literal(usize),
    Operator(usize, Vec<Packet>)
}

#[derive(Debug, PartialEq, Eq)]
struct Packet {
    version: usize,
    data: PacketData,
}

fn read_number(bitstream: &mut &[u8], n_bits: usize) -> Option<usize> {
    let mut number = 0;

    for _ in 0..n_bits {
        let bit = *bitstream.first()? as usize;
        *bitstream = &bitstream[1..];
        number = (number << 1) + bit;
    }

    Some(number)
}

fn read_literal(bitstream: &mut &[u8]) -> Option<usize> {
    let mut value = 0;

    loop {
        let continue_bit = read_number(bitstream, 1)?;
        value = (value << 4) + read_number(bitstream, 4)?;

        if continue_bit == 0 {
            return Some(value);
        }
    }
}

fn read_packet(bitstream: &mut &[u8]) -> Option<Packet> {
    let version = read_number(bitstream, 3)?;
    let type_id = read_number(bitstream, 3)?;

    if type_id == 4 {
        let value = read_literal(bitstream)?;
        Some(Packet{ version, data: PacketData::Literal(value) })
    } else {
        let length_type_id = read_number(bitstream, 1)?;
        let mut subpackets = vec![];

        if length_type_id == 0 {
            // The next 15 bits encode the lenght of the sub-packets in bits.
            let length_subpackets = read_number(bitstream, 15)?;
            if bitstream.len() < length_subpackets {
                return None;
            }

            let sub_bitstream = &mut &bitstream[0..length_subpackets];
            while sub_bitstream.len() > 0 {
                subpackets.push(read_packet(sub_bitstream)?);
            }
            *bitstream = &bitstream[length_subpackets..];
        } else {
            // The next 11 bits encode the number of sub-packets.
            let n_subpackets = read_number(bitstream, 11)?;

            for _ in 0..n_subpackets {
                subpackets.push(read_packet(bitstream)?);
            }
        }

        Some(Packet{ version, data: PacketData::Operator(type_id, subpackets) })
    }
}

impl Packet {
    fn create_from(s: &str) -> Self {
        let bits: Vec<_> = s
            .trim()
            .chars()
            .flat_map(|c| {
                let v = c.to_digit(16).unwrap() as u8;
                [v >> 3 & 0b1, v >> 2 & 0b1, v >> 1 & 0b1, v & 0b1]
            })
            .collect();

        read_packet(&mut &bits[..]).unwrap()
    }

    fn sum_of_versions(&self) -> usize {
        match &self.data {
            PacketData::Literal(_) => {
                self.version
            },
            PacketData::Operator(_, subpackets) => {
                self.version + subpackets.iter().map(|s| s.sum_of_versions()).sum::<usize>()
            },
        }
    }

    fn value(&self) -> usize {
        match &self.data {
            PacketData::Literal(v) => {
                *v
            },
            PacketData::Operator(0, subpackets) => {
                subpackets.iter().map(|s| s.value()).sum()
            },
            PacketData::Operator(1, subpackets) => {
                subpackets.iter().map(|s| s.value()).product()
            },
            PacketData::Operator(2, subpackets) => {
                subpackets.iter().map(|s| s.value()).min().unwrap()
            },
            PacketData::Operator(3, subpackets) => {
                subpackets.iter().map(|s| s.value()).max().unwrap()
            },
            PacketData::Operator(5, subpackets) => {
                if subpackets[0].value() > subpackets[1].value() {
                    1
                } else {
                    0
                }
            },
            PacketData::Operator(6, subpackets) => {
                if subpackets[0].value() < subpackets[1].value() {
                    1
                } else {
                    0
                }
            },
            PacketData::Operator(7, subpackets) => {
                if subpackets[0].value() == subpackets[1].value() {
                    1
                } else {
                    0
                }
            }
            _ => panic!(),
        }
    }
}

fn solve(input: &str) -> (usize, usize) {
    let packet = Packet::create_from(input);
    //println!("{:?}", packet);

    (packet.sum_of_versions(), packet.value())
}

fn main() {
    let input = std::fs::read_to_string("input/16.txt").unwrap();
    let now = std::time::Instant::now();
    let s = solve(&input);
    println!("Solution: {:?}", s);
    println!("Time: {}ms", now.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example01() {
        assert_eq!(
            Packet::create_from("D2FE28"),
            Packet {
                version: 6,
                data: PacketData::Literal(2021)
            }
        );
        assert_eq!(
            Packet::create_from("38006F45291200"),
            Packet {
                version: 1,
                data: PacketData::Operator(6, vec![
                    Packet { version: 6, data: PacketData::Literal(10) },
                    Packet { version: 2, data: PacketData::Literal(20) },
                ]),
            }
        );
        assert_eq!(
            Packet::create_from("EE00D40C823060"),
            Packet {
                version: 7,
                data: PacketData::Operator(3, vec![
                    Packet { version: 2, data: PacketData::Literal(1) },
                    Packet { version: 4, data: PacketData::Literal(2) },
                    Packet { version: 1, data: PacketData::Literal(3) },
                ]),
            }
        );
    }

    #[test]
    fn example02() {
        assert_eq!(Packet::create_from("8A004A801A8002F478").sum_of_versions(), 16);
        assert_eq!(Packet::create_from("620080001611562C8802118E34").sum_of_versions(), 12);
        assert_eq!(Packet::create_from("C0015000016115A2E0802F182340").sum_of_versions(), 23);
        assert_eq!(Packet::create_from("A0016C880162017C3686B18A3D4780").sum_of_versions(), 31);
    }

    #[test]
    fn example03() {
        assert_eq!(Packet::create_from("C200B40A82").value(), 3);
        assert_eq!(Packet::create_from("04005AC33890").value(), 54);
        assert_eq!(Packet::create_from("880086C3E88112").value(), 7);
        assert_eq!(Packet::create_from("CE00C43D881120").value(), 9);
        assert_eq!(Packet::create_from("D8005AC2A8F0").value(), 1);
        assert_eq!(Packet::create_from("F600BC2D8F").value(), 0);
        assert_eq!(Packet::create_from("9C005AC2F8F0").value(), 0);
        assert_eq!(Packet::create_from("9C0141080250320F1802104A08").value(), 1);
    }
}
