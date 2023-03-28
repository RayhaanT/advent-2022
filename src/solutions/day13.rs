use crate::Solution;
use std::cmp::Ordering;

// Maybe come back and do this again with Ord instead of compare_packets
#[derive(Clone)]
struct Packet {
    arr: Option<Vec<Packet>>,
    num: Option<u32>,
}

impl Packet {
    fn print(&self) {
        if self.num.is_some() {
            print!("{},", self.num.unwrap());
        } else {
            print!("[");
            for p in self.arr.as_ref().unwrap() {
                p.print();
            }
            print!("]");
        }
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        compare_packets(self, other) == Ordering::Equal
    }
}

impl Eq for Packet {}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        compare_packets(self, other)
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct StringStream {
    string: String,
    start: usize,
}

impl StringStream {
    fn len(&self) -> usize {
        self.string.len() - self.start
    }

    fn peek(&self) -> char {
        self.string.chars().nth(self.start).unwrap()
    }

    fn next(&mut self) -> char {
        self.start += 1;
        self.string.chars().nth(self.start - 1).unwrap()
    }

    fn back(&mut self) {
        self.start -= 1;
    }
}

fn parse_packet(stream: &mut StringStream) -> Packet {
    if stream.len() == 0 {
        Packet {
            arr: None,
            num: None,
        }
    } else if stream.peek() != '[' {
        let mut val: u32 = 0;
        let mut c = stream.next();
        while c.is_numeric() {
            val *= 10;
            val += c as u32 - '0' as u32;
            c = stream.next();
        }

        // Restore "]"
        if c == ']' {
            stream.back();
        }

        Packet {
            arr: None,
            num: Some(val),
        }
    } else {
        stream.next();
        let mut arr: Vec<Packet> = Vec::new();
        while stream.peek() != ']' {
            arr.push(parse_packet(stream));
        }
        stream.next();
        if stream.len() != 0 {
            if stream.peek() == ',' {
                stream.next();
            }
        }

        Packet {
            arr: Some(arr),
            num: None,
        }
    }
}

fn compare_packets(left: &Packet, right: &Packet) -> Ordering {
    // left.print();
    // print!(" vs ");
    // right.print();
    // println!("");

    if left.num.is_some() && right.num.is_some() {
        if left.num.unwrap() == right.num.unwrap() {
            Ordering::Equal
        } else {
            if left.num.unwrap() < right.num.unwrap() {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
    } else if left.arr.is_some() && right.arr.is_some() {
        let larr = left.arr.as_ref().unwrap();
        let rarr = right.arr.as_ref().unwrap();
        let min_length = usize::min(larr.len(), rarr.len());

        let tie_break = if rarr.len() == larr.len() {
            Ordering::Equal
        } else {
            if min_length == larr.len() {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        };

        for i in 0..min_length {
            let res = compare_packets(&larr[i], &rarr[i]);
            if res != Ordering::Equal {
                return res;
            }
        }

        tie_break
    } else if left.num.is_some() {
        compare_packets(
            &Packet {
                arr: Some(vec![Packet {
                    arr: None,
                    num: left.num,
                }]),
                num: None,
            },
            right,
        )
    } else {
        compare_packets(
            left,
            &Packet {
                arr: Some(vec![Packet {
                    arr: None,
                    num: right.num,
                }]),
                num: None,
            },
        )
    }
}

pub fn solve(input: String) -> Solution {
    let mut packets: Vec<Packet> = Vec::new();
    let lines: Vec<String> = input.lines().map(|l| l.to_string()).collect();

    for i in (0..lines.len()).step_by(3) {
        packets.push(parse_packet(&mut StringStream {
            string: lines[i].clone(),
            start: 0,
        }));
        packets.push(parse_packet(&mut StringStream {
            string: lines[i + 1].clone(),
            start: 0,
        }));
    }

    let mut first = 0;
    for i in (0..packets.len()).step_by(2) {
        if compare_packets(&packets[i], &packets[i + 1]) != Ordering::Greater {
            first += (i / 2) + 1;
        }
    }

    let two: Packet = Packet {
        arr: Some(vec![Packet {
            arr: None,
            num: Some(2),
        }]),
        num: None,
    };
    let six: Packet = Packet {
        arr: Some(vec![Packet {
            arr: None,
            num: Some(6),
        }]),
        num: None,
    };

    packets.push(two.clone());
    packets.push(six.clone());

    packets.sort();

    let mut two_ind = None;
    let mut six_ind = None;
    for (ind, packet) in packets.iter().enumerate() {
        if two_ind.is_none() && *packet == two {
            two_ind = Some(ind + 1);
        }
        if six_ind.is_none() && *packet == six {
            six_ind = Some(ind + 1);
        }
    }

    for p in &packets {
        p.print();
        println!("");
    }

    Solution {
        first: first.to_string(),
        second: (two_ind.unwrap() * six_ind.unwrap()).to_string(),
    }
}
