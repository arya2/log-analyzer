use crossbeam::channel::unbounded;
use log_analyzer::{parse_type_name};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[tokio::main]
async fn bench(test_id: &'static str) {
    let (s_io, r_io) = unbounded::<(usize, String)>();
    let (s_data, r_data) = unbounded();

    tokio::spawn(async move {
        let file = File::open(format!("./sample_data/{}.in", test_id))
            .expect("Failed to find and read input file");
        let mut line_num = 0;
        for line in BufReader::new(file).lines() {
            line_num += 1;
            if let Ok(line) = line {
                s_io.send((line_num, line)).expect("s_io.send failed");
            } else {
                break;
            }
        }
    });

    tokio::spawn(async move {
        let mut errors: Vec<usize> = vec![];
        while let Ok((line_num, line)) = r_io.recv() {
            let byte_size = line.len();
            match parse_type_name(&line) {
                Ok((_, type_name)) => {
                    s_data
                        .send((type_name.to_string(), byte_size))
                        .expect("s_data.send failed");
                }
                Err(_) => {
                    errors.push(line_num);
                }
            };
        }

        errors
    });

    let type_data_join = tokio::spawn(async move {
        let mut type_data: HashMap<String, (usize, usize)> = HashMap::new();
        while let Ok((type_name, byte_size)) = r_data.recv() {
            let updated_value = type_data
                .get(&type_name)
                .map_or((1, byte_size), |p| (p.0 + 1, p.1 + byte_size));
            type_data.insert(type_name, updated_value);
        }

        type_data
    });

    let mut results: Vec<_> = type_data_join
        .await
        .expect("stateful thread failed")
        .into_iter()
        .map(|(name, (num_instances, total_byte_size))| (name, num_instances, total_byte_size))
        .collect();

    results.sort_unstable_by(|&(_, _, ref s1), &(_, _, s2)| s2.cmp(s1));
}

use criterion::{black_box, criterion_group, criterion_main, Criterion};
fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("group1");
    group.sample_size(10);
    group.bench_function("t0", |b| b.iter(|| bench("t0")));
    group.bench_function("t1", |b| b.iter(|| bench("t1")));
    group.bench_function("t2", |b| b.iter(|| bench("t2")));
    group.bench_function("t3", |b| b.iter(|| bench("t3")));
    group.bench_function("t4", |b| b.iter(|| bench("t4")));
    group.bench_function("t5", |b| b.iter(|| bench("t5")));
    group.bench_function("t6", |b| b.iter(|| bench("t6")));
    group.bench_function("t7", |b| b.iter(|| bench("t7")));
    group.bench_function("t8", |b| b.iter(|| bench("t8")));
    group.bench_function("t9", |b| b.iter(|| bench("t9")));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
