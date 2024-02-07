// https://curiouscoding.nl/posts/1brc/
// Optimizations:
// 1. Bytes instead of strings; strings are checked to be valid UTF8
// 2. Manual parsing; instead of parsing as f32, parse manually to a fixed-precision i32 signed integer
// 3. Inline hash keys
// 4. Faster HashMap implementation`
// 5. Allocating the right
// 6. memchr for scanning
#![feature(slice_split_once, slice_internals)]

use core::slice::memchr::memchr;
use std::{env::args, io::Read};

use fxhash::FxHashMap;

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
        _ => panic!("Unknown pattern {:?}", std::str::from_utf8(s).unwrap()),
    };
    let v = a as V * 1000 + b as V * 100 + c as V * 10 + d as V;

    if neg { -v } else { v }
}

fn format(v: V) -> String {
    format!("{:.1}", v as f64 / 10.0)
}

fn to_key(name: &[u8]) -> u64 {
    // Initializes an array key of length 8 with zeros
    let mut key = [0u8; 8];
    let l = name.len().min(8);

    // It copies the first l bytes from name to key.
    key[..l].copy_from_slice(&name[..l]);
    // The bitwise XOR operation compares corresponding bits of two operands.
    // If the bits are the same, the result is 0, otherwise, it's 1
    // Only Alexandra and Alexandria coincide, so we’ll xor in the length of the string to make them unique
    key[0] ^= name.len() as u8;
    u64::from_ne_bytes(key)
}

fn main() {
    let filename = &args().nth(1).unwrap_or("measurements-small.txt".to_string());
    let mut data = vec![];
    {
        let stat = std::fs::metadata(filename).unwrap();
        data.reserve(stat.len() as usize + 1);
        let mut file = std::fs::File::open(filename).unwrap();
        file.read_to_end(&mut data).unwrap();
        assert_eq!(data.pop(), Some(b'\n'));
    }

    let mut h = FxHashMap::default();
    let mut data = &data[..];
    loop {
        let Some(separator) = memchr(b';', data) else { break; };
        let end = memchr(b'\n', &data[separator..]).unwrap();
        let name = &data[..separator];
        let value = &data[separator + 1..separator + end];

        h.entry(to_key(name))
            .or_insert((Record::default(), name))
            .0
            .add(parse(value));
        data = &data[separator + end + 1..]
    }
    let mut v = h.into_iter().collect::<Vec<_>>();
    v.sort_unstable_by_key(|p| p.0);

    for (_key, (r, name)) in v {
        println!(
            "{}: {}/{}/{}",
            std::str::from_utf8(name).unwrap(),
            format(r.min),
            format(r.avg()),
            format(r.max)
        );
    }
}