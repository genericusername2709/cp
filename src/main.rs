#![allow(unused_imports)]

mod reader;
mod utils;

use reader::Reader;
use std::cmp::{max, min, Reverse};
use std::collections::BinaryHeap;
use std::io::{stdout, BufWriter, Stdout, Write};
use utils::modulus::Mod;
use utils::Utils;

// const MOD: i64 = 1_000_000_007;
const MOD: i64 = 998_244_353;
// const ARR_LIMIT: usize = 1 << 18;
const ARR_LIMIT: usize = 1_000_005;

#[allow(unused)]
fn solve(reader: &mut Reader, writer: &mut BufWriter<Stdout>, utils: &mut Utils<MOD>) {}

#[allow(dead_code)]
fn main() {
    let mut reader: Reader = Reader::default();
    let writer = &mut BufWriter::new(stdout());
    let mut utils = Utils::default();
    let t: usize = reader.next(); // Read the number of test case
    for _ in 0..t {
        solve(&mut reader, writer, &mut utils);
    }
}
