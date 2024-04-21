pub mod algorithms {
    pub mod quick_sort;
    pub mod selection_sort;
    pub mod insertion_sort;
    pub mod merge_sort;
}

use algorithms::{quick_sort, selection_sort, insertion_sort, merge_sort};

fn main() {
    // Demonstrating the usage of the sorting algorithms
    let mut numbers = vec![34, 50, 25, 18, 65, 90, 6];
    println!("Original numbers: {:?}", numbers);

    let mut qs_numbers = numbers.clone();
    quick_sort::quick_sort(&mut qs_numbers);
    println!("Quick sorted: {:?}", qs_numbers);

    let mut ss_numbers = numbers.clone();
    selection_sort::selection_sort(&mut ss_numbers);
    println!("Selection sorted: {:?}", ss_numbers);

    let mut is_numbers = numbers.clone();
    insertion_sort::insertion_sort(&mut is_numbers);
    println!("Insertion sorted: {:?}", is_numbers);

    let mut ms_numbers = numbers.clone();
    merge_sort::merge_sort(&mut ms_numbers);
    println!("Merge sorted: {:?}", ms_numbers);

}
