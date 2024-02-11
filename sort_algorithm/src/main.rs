use rand::{prelude::ThreadRng, Rng};
use std::time::{Duration, Instant};

trait Sort {
    fn name(&self) -> &String;
    fn sort<'a, T: PartialOrd + Copy>(&self, arr: &'a mut [T]) -> &'a [T];
}

pub struct BubbleSort {
    name: String,
}

impl Default for BubbleSort {
    fn default() -> Self {
        Self {
            name: "BubbleSort".to_string(),
        }
    }
}

impl BubbleSort {
    pub fn new() -> BubbleSort {
        Self::default()
    }
}

impl Sort for BubbleSort {
    fn name(&self) -> &String {
        &self.name
    }
    fn sort<'a, T: PartialOrd + Copy>(&self, arr: &'a mut [T]) -> &'a [T] {
        let len: usize = arr.len();
        for i in 0..len {
            for j in 0..len - i - 1 {
                if arr[j] > arr[j + 1] {
                    arr.swap(j, j + 1);
                }
            }
        }
        arr
    }
}

pub struct ChooseSortMin {
    name: String,
}

impl Default for ChooseSortMin {
    fn default() -> Self {
        Self {
            name: "ChooseSortMin".to_string(),
        }
    }
}

impl ChooseSortMin {
    pub fn new() -> ChooseSortMin {
        Self::default()
    }
}

impl Sort for ChooseSortMin {
    fn name(&self) -> &String {
        &self.name
    }
    fn sort<'a, T: PartialOrd + Copy>(&self, arr: &'a mut [T]) -> &'a [T] {
        let len: usize = arr.len();
        for i in 0..len {
            let mut min_index: usize = i;
            for j in i + 1..len {
                if arr[j] < arr[min_index] {
                    min_index = j;
                }
            }
            arr.swap(i, min_index);
        }
        arr
    }
}

pub struct ChooseSortMax {
    name: String,
}

impl Default for ChooseSortMax {
    fn default() -> Self {
        Self {
            name: "ChooseSortMax".to_string(),
        }
    }
}

impl ChooseSortMax {
    pub fn new() -> ChooseSortMax {
        Self::default()
    }
}

impl Sort for ChooseSortMax {
    fn name(&self) -> &String {
        &self.name
    }

    fn sort<'a, T: PartialOrd + Copy>(&self, arr: &'a mut [T]) -> &'a [T] {
        let len: usize = arr.len();
        for i in 0..len {
            let mut max_index: usize = i;
            for j in i + 1..len {
                if arr[j] > arr[max_index] {
                    max_index = j;
                }
            }
            arr.swap(i, max_index);
        }
        arr
    }
}

struct InsertSort {
    name: String,
}

impl Default for InsertSort {
    fn default() -> Self {
        Self {
            name: "InsertSort".to_string(),
        }
    }
}

impl InsertSort {
    pub fn new() -> InsertSort {
        Self::default()
    }
}

impl Sort for InsertSort {
    fn name(&self) -> &String {
        &self.name
    }

    fn sort<'a, T: PartialOrd + Copy>(&self, arr: &'a mut [T]) -> &'a [T] {
        let len: usize = arr.len();
        for i in 1..len {
            let mut j: usize = i;
            while j > 0 && arr[j] < arr[j - 1] {
                arr.swap(j, j - 1);
                j -= 1;
            }
        }
        arr
    }
}

struct InsertSort2 {
    name: String,
}

impl Default for InsertSort2 {
    fn default() -> Self {
        Self {
            name: "InsertSort2".to_string(),
        }
    }
}

impl InsertSort2 {
    pub fn new() -> InsertSort2 {
        Self::default()
    }
}

impl Sort for InsertSort2 {
    fn name(&self) -> &String {
        &self.name
    }

    fn sort<'a, T: PartialOrd + Copy>(&self, arr: &'a mut [T]) -> &'a [T] {
        Self::sort_recursively(arr);
        arr
    }
}

impl InsertSort2 {
    fn sort_recursively<T: PartialOrd + Copy>(arr: &mut [T]) {
        let len: usize = arr.len();
        if len <= 1 {
            return;
        }
        Self::sort_recursively(&mut arr[0..len - 1]);
        let temp: T = arr[len - 1];
        let mut j: usize = len - 1;
        while j > 0 && temp < arr[j - 1] {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = temp;
    }
}

struct QuickSort {
    name: String,
}

impl Default for QuickSort {
    fn default() -> Self {
        Self {
            name: "QuickSort".to_string(),
        }
    }
}

impl QuickSort {
    pub fn new() -> QuickSort {
        Self::default()
    }
}

impl Sort for QuickSort {
    fn name(&self) -> &String {
        &self.name
    }

    fn sort<'a, T: PartialOrd + Copy>(&self, arr: &'a mut [T]) -> &'a [T] {
        Self::sort_recursively(arr, 0, arr.len() - 1);
        arr
    }
}

impl QuickSort {
    fn sort_recursively<T: PartialOrd + Copy>(arr: &mut [T], left: usize, right: usize) {
        
        if left >= right {
            return;
        }
        let random: usize = rand::thread_rng().gen_range(left..right);
        arr.swap(left, random);
        let pivot: T = arr[left];
        let mut i: usize = left;
        let mut j: usize = right;
        while i < j {
            while i < j && arr[j] >= pivot {
                j -= 1;
            }
            arr[i] = arr[j];
            while i < j && arr[i] <= pivot {
                i += 1;
            }
            arr[j] = arr[i];
        }
        arr[i] = pivot;
        if i > 1 {
            Self::sort_recursively(arr, left, i - 1);
        }
        if i < right {
            Self::sort_recursively(arr, i + 1, right);
        }
    }
}

struct MergeSort {
    name: String,
}

impl Default for MergeSort {
    fn default() -> Self {
        Self {
            name: "MergeSort".to_string(),
        }
    }
}

impl MergeSort {
    pub fn new() -> MergeSort {
        Self::default()
    }
}

impl Sort for MergeSort {
    fn name(&self) -> &String {
        &self.name
    }

    fn sort<'a, T: PartialOrd + Copy>(&self, arr: &'a mut [T]) -> &'a [T] {
        let len: usize = arr.len();
        let mut temp: Vec<T> = vec![arr[0]; len];
        Self::sort_recursively(arr, &mut temp, 0, len - 1);
        arr
    }
}

impl MergeSort {
    fn sort_recursively<T: PartialOrd + Copy>(
        arr: &mut [T],
        temp: &mut [T],
        left: usize,
        right: usize,
    ) {
        if left >= right {
            return;
        }
        let mid: usize = (left + right) / 2;
        Self::sort_recursively(arr, temp, left, mid);
        Self::sort_recursively(arr, temp, mid + 1, right);
        Self::merge(arr, temp, left, mid, right);
    }

    fn merge<T: PartialOrd + Copy>(
        arr: &mut [T],
        temp: &mut [T],
        left: usize,
        mid: usize,
        right: usize,
    ) {
        let mut i: usize = left;
        let mut j: usize = mid + 1;
        let mut k: usize = left;
        while i <= mid && j <= right {
            if arr[i] <= arr[j] {
                temp[k] = arr[i];
                i += 1;
            } else {
                temp[k] = arr[j];
                j += 1;
            }
            k += 1;
        }
        while i <= mid {
            temp[k] = arr[i];
            i += 1;
            k += 1;
        }
        while j <= right {
            temp[k] = arr[j];
            j += 1;
            k += 1;
        }
        arr[left..=right].clone_from_slice(&temp[left..=right]);
    }
}

fn test_sort_trait<SORT: Sort, T: PartialOrd + Copy + std::fmt::Debug>(
    sort: &SORT,
    numbers: &mut [T],
    print_numbers: Option<bool>,
) {
    let print_numbers: bool = print_numbers.unwrap_or(false);
    let start: Instant = Instant::now();
    sort.sort(numbers);
    if print_numbers {
        println!("{0}\n: {1:?}", sort.name(), numbers);
    }
    let duration: Duration = start.elapsed();
    println!("{0}: {1:?}", sort.name(), duration);
}

fn get_random_numbers(n: i32) -> Vec<i32> {
    let mut rng: ThreadRng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(-100..100)).collect()
}

fn main() {
    let ns = [10, 100, 1000,2500,3750,5000,6250,7500,10000];

    for n in ns.iter() {
        println!("\nn: {0}", n);
        let numbers: Vec<i32> = get_random_numbers(*n);
        let print_numbers: bool = false;
        let print_numbers: Option<bool> = Some(print_numbers);
        let bubble_sort: BubbleSort = BubbleSort::new();
        test_sort_trait(&bubble_sort, &mut numbers.clone(), print_numbers);

        let choose_sort_min: ChooseSortMin = ChooseSortMin::new();
        test_sort_trait(&choose_sort_min, &mut numbers.clone(), print_numbers);

        let choose_sort_max: ChooseSortMax = ChooseSortMax::new();
        test_sort_trait(&choose_sort_max, &mut numbers.clone(), print_numbers);

        let insert_sort: InsertSort = InsertSort::new();
        test_sort_trait(&insert_sort, &mut numbers.clone(), print_numbers);

        let insert_sort2: InsertSort2 = InsertSort2::new();
        test_sort_trait(&insert_sort2, &mut numbers.clone(), print_numbers);

        let quick_sort: QuickSort = QuickSort::new();
        test_sort_trait(&quick_sort, &mut numbers.clone(), print_numbers);

        let merge_sort: MergeSort = MergeSort::new();
        test_sort_trait(&merge_sort, &mut numbers.clone(), print_numbers);
    }
}
