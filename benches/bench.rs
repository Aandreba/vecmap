#![feature(iter_collect_into, file_create_new)]

use num_format::{Locale, ToFormattedString};
use rand::{
    distributions::{Alphanumeric, Standard},
    thread_rng, Rng,
};
use std::{
    collections::{BTreeMap, HashMap},
    hint::black_box,
    time::{Duration, Instant},
};
use vector_mapp::{binary::BinaryMap, vec::VecMap};

pub struct Bencher {
    warmup: Duration,
    duration: Duration,
    result: VecMap<&'static str, Vec<Duration>>,
}

impl Bencher {
    #[inline(never)]
    pub fn iter<T, F: FnMut() -> T>(&mut self, name: &str, param: usize, mut f: F) {
        // Warmup
        println!("Warming up '{name}' [{param}] for {:?}", &self.warmup);
        let now = Instant::now();
        let mut runs = 0u128;
        loop {
            black_box(f());
            if now.elapsed() >= self.warmup {
                break;
            }
            runs += 1
        }

        // Benchmark
        runs = ((runs as f64) * self.duration.as_secs_f64() / self.warmup.as_secs_f64()) as u128;
        println!(
            "Benchmarking '{name}' [{param}] for {:?}  (expect arround {} runs)",
            &self.warmup,
            runs.to_formatted_string(&Locale::es)
        );
        let now = Instant::now();
        for _ in 0..runs {
            black_box(f());
        }
        let delta = now.elapsed();

        // Show results
        let average = Duration::from_secs_f64(delta.as_secs_f64() / (runs as f64));
        println!("Benchmarked '{name}' [{param}]: {average:?}");
        println!();
        self.result[name]
    }
}

fn insert_with_size(size: usize, b: &mut Bencher) {
    let entries = thread_rng()
        .sample_iter(Standard)
        .take(size)
        .collect::<Vec<(u32, u32)>>();

    let hash = b.iter("hashmap", size, || {
        let mut hashmap = HashMap::new();
        entries.iter().copied().collect_into(&mut hashmap);
    });

    let btree = b.iter("btreemap", size, || {
        let mut hashmap = BTreeMap::new();
        entries.iter().copied().collect_into(&mut hashmap);
    });

    let vec = b.iter("vecmap", size, || {
        let mut hashmap = VecMap::new();
        entries.iter().copied().collect_into(&mut hashmap);
    });

    let bin = b.iter("binarymap", size, || {
        let mut hashmap = BinaryMap::new();
        entries.iter().copied().collect_into(&mut hashmap);
    });

    b.output.write_fmt(format_args!("{}\n")).unwrap();
}

pub fn main() {
    let file_name = thread_rng()
        .sample_iter::<u8, _>(Alphanumeric)
        .take(10)
        .chain(b".csv".iter().copied())
        .collect::<Vec<u8>>();

    let mut b = Bencher {
        warmup: Duration::from_secs(3),
        duration: Duration::from_secs(5),
        result: [
            ("hashmap", Vec::new()),
            ("btreemap", Vec::new()),
            ("vecmap", Vec::new()),
            ("binarymap", Vec::new()),
        ]
        .into_iter()
        .collect(),
    };

    for i in (2..=200).step_by(10) {
        insert_with_size(i, &mut b);
        //insert_prealloc_with_size(i, c);
    }
}
