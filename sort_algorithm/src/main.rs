use rand::Rng;
use std::time::Instant;

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
        let len = arr.len();
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
        let len = arr.len();
        for i in 0..len {
            let mut min_index = i;
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
        let len = arr.len();
        for i in 0..len {
            let mut max_index = i;
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

fn test_sort_trait<SORT: Sort, T: PartialOrd + Copy + std::fmt::Debug>(
    sort: &SORT,
    numbers: &mut [T],
) {
    let start = Instant::now();
    sort.sort(numbers);
    let duration = start.elapsed();
    println!("{0}: {1:?}", sort.name(), duration);
}

fn get_random_numbers(n: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(-100..100)).collect()
}

fn main() {
    let numbers = get_random_numbers(100);

    let bubble_sort = BubbleSort::new();
    test_sort_trait(&bubble_sort, &mut numbers.clone());

    let choose_sort_min = ChooseSortMin::new();
    test_sort_trait(&choose_sort_min, &mut numbers.clone());

    let choose_sort_max = ChooseSortMax::new();
    test_sort_trait(&choose_sort_max, &mut numbers.clone());
}
