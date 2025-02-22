use testing::sorting_algorithms::{selection_sort, bubble_sort, insertion_sort};

use criterion::{Criterion, criterion_main, criterion_group, black_box, BenchmarkId};

// La funzione di benchmarking ottiene l'accesso a un'istanza "c" di Criterion
// fn sort_benchmark(c: &mut Criterion) {
//    let mut numbers = vec![1, 2, 3, 4, 5];

//    c.bench_function("Selection Sort", |b|  {
//        b.iter(|| selection_sort(&mut numbers))
//    });

//    c.bench_function("Insertion Sort", |b|  {
//        b.iter(|| insertion_sort(&mut numbers))
//    });

//    c.bench_function("Bubble Sort", |b|  {
//        b.iter(|| bubble_sort(&mut numbers))
//    });
// }


fn sort_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sorting Algorithms");

    let numbers = vec![5, 3, 8, 4, 2, 11, 1, 32, 23, 7, 10];

    group.bench_with_input(BenchmarkId::new("Selection Sort", numbers.len()), &numbers, |b, data| {
        b.iter(|| selection_sort(&mut data.clone()))
    });

    group.bench_with_input(BenchmarkId::new("Insertion Sort", numbers.len()), &numbers, |b, data| {
        b.iter(|| insertion_sort(&mut data.clone()))
    });

    group.bench_with_input(BenchmarkId::new("Bubble Sort", numbers.len()), &numbers, |b, data| {
        b.iter(|| bubble_sort(&mut data.clone()))
    });

    group.finish();
}

criterion_group!(sorting_benchmarks, sort_benchmark);

criterion_main!(sorting_benchmarks);


// c.bench_function("Bubble Sort", |b| b.iter(|| bubble_sort(black_box(&mut numbers))));

// c.bench_function("Insertion Sort", |b| b.iter(|| insertion_sort(black_box(&mut numbers))));
