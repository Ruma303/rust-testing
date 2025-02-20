use testing::sorting_algorithms::{selection_sort, bubble_sort, insertion_sort};

use criterion::{Criterion, criterion_main, criterion_group, black_box};

// La funzione di benchmarking ottiene l'accesso a un'istanza "c" di Criterion
fn sort_benchmark(c: &mut Criterion) {
   let mut numbers = vec![1, 2, 3, 4, 5];

   c.bench_function("Selection Sort", |b|  {
       b.iter(|| selection_sort(&mut numbers))
   });

   c.bench_function("Insertion Sort", |b|  {
       b.iter(|| insertion_sort(&mut numbers))
   });

   c.bench_function("Bubble Sort", |b|  {
       b.iter(|| bubble_sort(&mut numbers))
   });
}

criterion_group!(sorting_benchmarks, sort_benchmark);

criterion_main!(sorting_benchmarks);


// c.bench_function("Bubble Sort", |b| b.iter(|| bubble_sort(black_box(&mut numbers))));

// c.bench_function("Insertion Sort", |b| b.iter(|| insertion_sort(black_box(&mut numbers))));
