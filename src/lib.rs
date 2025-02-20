pub mod sorting_algorithms {
    pub fn selection_sort(arr: &mut Vec<i32>) {
        let len = arr.len();
        for i in 0..len {
            let mut min_index = i;
            for j in (i + 1)..len {
                if arr[j] < arr[min_index] {
                    min_index = j;
                }
            }
            arr.swap(i, min_index);
        }
    }

    pub fn bubble_sort(arr: &mut Vec<i32>) {
        let len = arr.len();
        for i in 0..len {
            for j in 0..(len - i - 1) {
                if arr[j] > arr[j + 1] {
                    arr.swap(j, j + 1);
                }
            }
        }
    }

    pub fn insertion_sort(arr: &mut Vec<i32>) {
        let len = arr.len();
        for i in 1..len {
            let mut j = i;
            while j > 0 && arr[j - 1] > arr[j] {
                arr.swap(j, j - 1);
                j -= 1;
            }
        }
    }
}
