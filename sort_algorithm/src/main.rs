

fn bubble_sort<T:PartialOrd+Copy>(arr: &[T])->Vec<T> {
    let mut result = arr.to_vec();
    let len = result.len();
    for i in 0..len{
        for j in 0..len-i-1{
            if result[j] > result[j+1]{
                result.swap(j, j+1);
            }
        }
    }
    result
}

fn main() {
  let numbers=vec![4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    let sorted_numbers = bubble_sort(&numbers);
    println!("{:?}", sorted_numbers);
    
}
