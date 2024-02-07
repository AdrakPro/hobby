// https://curiouscoding.nl/posts/1brc/
// Optimizations:
// No optimization -> 877.4 ms
// 1. Bytes instead of strings; strings are checked to be valid UTF8 -> 132.5 ms
// 2. Manual parsing; instead of parsing as f32, parse manually to a fixed-precision i32 signed integer
#![feature(slice_split_once)]

use std::{collections::HashMap, env::args, io::Read};

type V = i32;

struct Record {
    count: u32,
    min: V,
    max: V,
    sum: V,
}

impl Record {
    fn default() -> Self {
        Self {
            count: 0,
            min: i32::MAX,
            max: i32::MIN,
            sum: 0,
        }
    }
    fn add(&mut self, value: V) {
        self.count += 1;
        self.sum += value;
        self.min = self.min.min(value);
        self.max = self.max.max(value);
    }
    fn avg(&self) -> V {
        self.sum / self.count as V
    }
}


fn parse(mut s: &[u8]) -> V {
    // Check the first byte of slice is equal to -
    let neg = if s[0] == b'-' {
        // Slice it removing first bit
        s = &s[1..];
        true
    } else {
        false
    };

    let (a, b, c, d) = match s {
        [c, b'.', d] => (0, 0, c - b'0', d - b'0'),
        [b, c, b'.', d] => (0, b - b'0', c - b'0', d - b'0'),
        [a, b, c, b'.', d] => (a - b'0', b - b'0', c - b'0', d - b'0'),
        [c] => (0, 0, 0, c - b'0'),
        [b, c] => (0, b - b'0', c - b'0', 0),
        [a, b, c] => (a - b'0', b - b'0', c - b'0', 0),
        _ => panic!("Unknown patters {:?}", std::str::from_utf8(s).unwrap()),
    };
    let v = a as V * 1000 + b as V * 100 + c as V * 10 + d as V;

    if neg { -v } else { v }
}

fn format(v: V) -> String {
    format!("{:.1}", v as f64 / 10.0)
}

fn main() {
    let filename = args().nth(1).unwrap_or("measurements-small.txt".to_string());
    let mut data = vec![];
    {
        let mut file = std::fs::File::open(filename).unwrap();
        file.read_to_end(&mut data).unwrap();
        assert_eq!(data.pop(), Some(b'\n'));
    }

    let mut h = HashMap::new();

    for line in data.split(|&c| c == b'\n') {
        let (name, value) = line.split_once(|&c| c == b';').unwrap();

        h.entry(name).or_insert(Record::default()).add(parse(value));
    }

    let mut v = h.into_iter().collect::<Vec<_>>();
    v.sort_unstable_by_key(|p| p.0);

    for (name, r) in v {
        println!("{}: {:.1}/{:.1}/{:.1}",
                 std::str::from_utf8(name).unwrap(),
                 format(r.min),
                 format(r.avg()),
                 format(r.max)
        )
    }
}