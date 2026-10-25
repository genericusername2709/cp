#![allow(unused_imports)]

mod reader;
mod utils;

use reader::Reader;
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap};
use std::convert::TryInto;
use std::io::{stdout, BufReader, BufWriter, Stdin, Stdout, Write};

use utils::dsu::DisjointSetUnion;
use utils::modulus::Mod;
use utils::{
    gcd, lcm, LazySegTreeFn, PointUpdateTree, RangeAddSumTree, RangeGcdTree, RangeMaxTree,
    RangeMinTree, RangeSetMaxTree, RangeSetMinTree, Utils,
};

// const MOD: i64 = 1_000_000_007;
const MOD: i64 = 998_244_353;

#[allow(unused)]
// const ARR_LIMIT: usize = 1 << 18;
// const ARR_LIMIT: usize = 1_000_005;
const ARR_LIMIT: usize = 300_005;

#[allow(unused)]
fn solve(
    t: usize,
    r: &mut Reader<BufReader<Stdin>>,
    w: &mut BufWriter<Stdout>,
    utils: &mut Utils<MOD>,
) {
    let n: usize = r.next_int();
    // pen!(w, n);
}

#[allow(unused)]
fn preproc(utils: &mut Utils<MOD>) {
    // utils.comb.populate_factorials(ARR_LIMIT);
    // utils.prime.init_prime_seive(ARR_LIMIT);
    // utils.preprocdata.push(vec![0]);
}

#[allow(dead_code)]
fn main() {
    let mut reader: Reader<BufReader<Stdin>> = Reader::new();
    let writer = &mut BufWriter::new(stdout());
    let mut utils = Utils::default();
    preproc(&mut utils);
    let t: usize = reader.next_int();
    for i in 1..=t {
        solve(i, &mut reader, writer, &mut utils);
    }
}
