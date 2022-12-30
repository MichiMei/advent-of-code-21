#![allow(dead_code)]

use std::io;

pub fn a() {
    let mut stream = get_binary();
    let (v, _, _) = parse_packet(&mut stream);
    println!("{}",v);
}

pub fn b() {
    let mut stream = get_binary();
    let (_, _, r) = parse_packet(&mut stream);
    println!("{}",r);
}

// returns (version_sum, bit_count, res_value)
fn parse_packet(stream: &mut PacketStream) -> (u64, u64, u64) {
    let mut v_count = stream.next_bits_u64(3);
    let t = stream.next_bits_u64(3);
    let mut bit_count = 6;
    let res_value;
    match t {
        4 => {
            let (tmp_b, tmp_r) = parse_const(stream);
            bit_count += tmp_b;
            res_value = tmp_r;
        }
        _ => {
            let (tmp_v, tmp_b, tmp_r) = parse_operator(stream, t);
            bit_count += tmp_b;
            v_count += tmp_v;
            res_value = tmp_r;
        }
    }
    (v_count, bit_count, res_value)
}

// returns (bit_count, res_value)
fn parse_const(stream: &mut PacketStream) -> (u64, u64) {
    let mut res = 0;
    let mut bit_count = 0;
    loop {
        let cont = stream.next_bits_u64(1);
        let val = stream.next_bits_u64(4);
        res = res << 4;
        res += val;
        bit_count += 5;
        if cont == 0 { break; }
    }
    (bit_count, res)
}

// returns (version_sum, bit_count, res_value)
fn parse_operator(stream: &mut PacketStream, t: u64) -> (u64, u64, u64) {
    let mut version_sum = 0;
    let mut bit_count = 0;
    let mut res_value = 0;


    let i = stream.next_bits_u64(1);
    bit_count += 1;

    let mut sub_packets = vec![];

    match i {
        0 => {
            let length = stream.next_bits_u64(15);
            bit_count += 15;
            let mut tmp_bit_count = 0;
            while tmp_bit_count < length {
                let packet = parse_packet(stream);
                tmp_bit_count += packet.1;
                version_sum += packet.0;
                sub_packets.push(packet);
            }
            bit_count += tmp_bit_count;
        }
        1 => {
            let length = stream.next_bits_u64(11);
            bit_count += 11;
            for _ in 0..length {
                let packet = parse_packet(stream);
                version_sum += packet.0;
                bit_count += packet.1;
                sub_packets.push(packet);
            }
        }
        _ => panic!(),
    }

    match t {
        0 => {
            for (_, _, tmp_r) in sub_packets {
                res_value += tmp_r;
            }
        }
        1 => {
            res_value = 1;
            for (_, _, tmp_r) in sub_packets {
                res_value *= tmp_r;
            }
        }
        2 => {
            res_value = u64::MAX;
            for (_, _, tmp_r) in sub_packets {
                if tmp_r < res_value {
                    res_value = tmp_r;
                }
            }
        }
        3 => {
            for (_, _, tmp_r) in sub_packets {
                if tmp_r > res_value {
                    res_value = tmp_r;
                }
            }
        }
        5 => {
            if sub_packets.len() != 2 { panic!() }
            if sub_packets[0].2 > sub_packets[1].2 {
                res_value = 1;
            }
        }
        6 => {
            if sub_packets.len() != 2 { panic!() }
            if sub_packets[0].2 < sub_packets[1].2 {
                res_value = 1;
            }
        }
        7 => {
            if sub_packets.len() != 2 { panic!() }
            if sub_packets[0].2 == sub_packets[1].2 {
                res_value = 1;
            }
        }
        _ => panic!(),
    }

    (version_sum, bit_count, res_value)
}

fn get_binary() -> PacketStream {
    let mut buf = String::new();
    match io::stdin().read_line(&mut buf) {
        Ok(_) => {}
        Err(_) => panic!(),
    }
    PacketStream::new(&buf)
}

pub struct PacketStream {
    bit_string: String,
    index: usize,
}

impl PacketStream {
    pub fn new(byte_repr: &str) -> Self {
        let bit_vec: Vec<String> = byte_repr.chars().map(|x| {
            let t = match u8::from_str_radix(&x.to_string(), 16) {
                Ok(y) => y,
                Err(_) => return String::new(),
            };
            let bit_string = format!("{:04b}", t);
            bit_string
        }).collect();
        let bit_string = bit_vec.join("");
        println!("{:?}", bit_string);
        PacketStream{bit_string, index:0}
    }

    pub fn is_empty(&self) -> bool {
        if self.index == self.bit_string.len() {
            return true
        }
        false
    }

    pub fn next_bits_u64(&mut self, count: usize) -> u64 {
        if count > 64 { panic!() }

        let substr = self.next_bits_str(count);

        let int = u64::from_str_radix(substr, 2).unwrap();

        int
    }

    pub fn next_bits_str(&mut self, count: usize) -> &str {
        if self.index+count > self.bit_string.len() {
            panic!("len({}), ind({}), cou({})", self.bit_string.len(), self.index, count)
        }
        let tmp = &self.bit_string[self.index..self.index+count];
        self.index += count;
        tmp
    }
}