use testing::sorting_algorithms::{selection_sort, bubble_sort, insertion_sort};

fn main() {
    let mut vec1 = vec![5, 3, 8, 4, 2, 11, 1, 32, 23, 7, 10];
    let mut vec2 = vec1.clone();
    let mut vec3 = vec1.clone();

    selection_sort(&mut vec1);
    println!("Selection Sort: {:?}", vec1);

    bubble_sort(&mut vec2);
    println!("Bubble Sort: {:?}", vec2);

    insertion_sort(&mut vec3);
    println!("Insertion Sort: {:?}", vec3);
}
