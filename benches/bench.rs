#![allow(unused)]
#![feature(iter_collect_into, iter_intersperse, file_create_new)]

use num_format::{Locale, ToFormattedString};
use rand::{
    distributions::{Alphanumeric, Standard},
    thread_rng, Rng, random,
};
use std::{
    collections::{HashMap, BTreeMap},
    fs::File,
    hint::black_box,
    io::{BufWriter, Seek, SeekFrom, Write},
    ops::Deref,
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
            &self.duration,
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
        self.result[name].push(average);
    }

    #[inline]
    pub fn write(&self, w: &mut BufWriter<File>) -> std::io::Result<()> {
        for (key, value) in self.result.iter() {
            let value = value
                .iter()
                .map(|x| format!("{}", x.as_nanos()))
                .intersperse(String::from(","))
                .collect::<String>();

            w.write_fmt(format_args!("{key},{value}\n"))?;
        }

        return Ok(());
    }
}

fn insert_with_size(size: usize, b: &mut Bencher) {
    let entries = thread_rng()
        .sample_iter(Standard)
        .take(size)
        .collect::<Vec<(u32, u32)>>();

    b.iter("hashmap", size, || {
        let mut hashmap = HashMap::with_capacity(size);
        for (k, v) in entries.iter().copied() {
            let _ = hashmap.insert(k, v);
        }
        return hashmap;
    });

    // b.iter("btreemap", size, || {
    //     let mut hashmap = BTreeMap::new();
    //     for (k, v) in entries.iter().copied() {
    //         let _ = hashmap.insert(k, v);
    //     }
    //     return hashmap;
    // });

    b.iter("vecmap", size, || {
        let mut hashmap = VecMap::with_capacity(size);
        for (k, v) in entries.iter().copied() {
            let _ = hashmap.insert(k, v);
        }
        return hashmap;
    });

    b.iter("binarymap", size, || {
        let mut hashmap = BinaryMap::with_capacity(size);
        for (k, v) in entries.iter().copied() {
            let _ = hashmap.insert(k, v);
        }
        return hashmap;
    });
}

fn search_with_size(size: usize, b: &mut Bencher) {
    let entries = thread_rng()
        .sample_iter(Standard)
        .take(size)
        .collect::<Vec<(u32, u32)>>();

    let searches = (0..size).map(|_| match random::<bool>() {
        true => random::<u32>(),
        false => unsafe { entries.get_unchecked(thread_rng().gen_range(0..size)).0 }
    }).collect::<Vec<_>>();

    let hashmap = entries.iter().copied().collect::<HashMap<_, _>>();
    b.iter("hashmap", size, || {
        for key in searches.iter() {
            black_box(hashmap.get(key));
        }
    });

    let btreemap = entries.iter().copied().collect::<BTreeMap<_, _>>();
    b.iter("btreemap", size, || {
        for key in searches.iter() {
            black_box(btreemap.get(key));
        }
    });

    let vecmap = entries.iter().copied().collect::<VecMap<_, _>>();
    b.iter("vecmap", size, || {
        for key in searches.iter() {
            black_box(vecmap.get(key));
        }
    });

    let binarymap = entries.iter().copied().collect::<BinaryMap<_, _>>();
    b.iter("binarymap", size, || {
        for key in searches.iter() {
            black_box(binarymap.get(key));
        }
    });
}

pub fn main() -> std::io::Result<()> {
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
            //("btreemap", Vec::new()),
            ("vecmap", Vec::new()),
            ("binarymap", Vec::new()),
        ]
        .into_iter()
        .collect(),
    };

    let mut header = vec![String::new()];
    let mut file = BufWriter::new(File::create_new(
        String::from_utf8_lossy(&file_name).deref(),
    )?);

    for i in (2..=200).step_by(10) {
        // Add entry count to headers
        header.push(format!("{i}"));

        // Run benchmark
        search_with_size(i, &mut b);

        // Go to the start of the file
        file.seek(SeekFrom::Start(0))?;

        // Write header
        let mut header = header.join(",").into_bytes();
        header.push(b'\n');
        file.write_all(&header)?;

        // Write contents
        b.write(&mut file)?;
    }

    file.flush()?;
    return Ok(());
}
