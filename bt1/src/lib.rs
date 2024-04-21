pub fn quick_sort<T: PartialOrd + Clone>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    let pivot_index = partition(arr);
    quick_sort(&mut arr[0..pivot_index]);
    quick_sort(&mut arr[pivot_index + 1..]);
}

fn partition<T: PartialOrd + Clone>(arr: &mut [T]) -> usize {
    let pivot_index = arr.len() / 2;
    arr.swap(pivot_index, arr.len() - 1);
    let mut i = 0;
    for j in 0..arr.len() - 1 {
        if arr[j] <= arr[arr.len() - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, arr.len() - 1);
    i
}

pub fn selection_sort<T: PartialOrd + Clone>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let mut min_index = i;
        for j in i + 1..arr.len() {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        arr.swap(i, min_index);
    }
}

pub fn insertion_sort<T: PartialOrd + Clone>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

pub fn merge_sort<T: PartialOrd + Clone>(arr: &mut [T]) {
    if arr.len() > 1 {
        let mid = arr.len() / 2;
        let mut left = arr[0..mid].to_vec();
        let mut right = arr[mid..].to_vec();
        merge_sort(&mut left);
        merge_sort(&mut right);
        merge(arr, &left, &right);
    }
}

fn merge<T: PartialOrd + Clone>(arr: &mut [T], left: &[T], right: &[T]) {
    let (mut i, mut j, mut k) = (0, 0, 0);
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i].clone();
            i += 1;
        } else {
            arr[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }
    while i < left.len() {
        arr[k] = left[i].clone();
        i += 1;
        k += 1;
    }
    while j < right.len() {
        arr[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut vec = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        quick_sort(&mut vec);
        assert_eq!(vec, [1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
    }

    #[test]
    fn test_selection_sort() {
        let mut vec = vec![3, 1, 4, 1, 5];
        selection_sort(&mut vec);
        assert_eq!(vec, [1, 1, 3, 4, 5]);
    }

    #[test]
    fn test_insertion_sort() {
        let mut vec = vec![5, 2, 9, 1, 5, 6];
        insertion_sort(&mut vec);
        assert_eq!(vec, [1, 2, 5, 5, 6, 9]);
    }

    #[test]
    fn test_merge_sort() {
        let mut vec = vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 5];
        merge_sort(&mut vec);
        assert_eq!(vec, [1, 2, 2, 2, 3, 3, 4, 5, 6, 7, 9]);
    }
}

