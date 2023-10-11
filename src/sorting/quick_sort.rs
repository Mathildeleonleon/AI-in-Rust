use std::cmp::PartialOrd;

pub fn partition<T: PartialOrd>(arr: &mut [T], lo: usize, hi: usize) -> usize {
    let pivot = hi;
    let mut i = lo;
    let mut j = hi;

    loop {
        while arr[i] < arr[pivot] {
            i += 1;
        }
        while j > 0 && arr[j - 1] > arr[pivot] {
            j -= 1;
        }
        if j == 0 || i >= j - 1 {
            break;
        } else if arr[i] == arr[j - 1] {
            i += 1;
            j -= 1;
        } else {
            arr.swap(i, j - 1);
        }
    }
    arr.swap(i, pivot);
    i
}

fn _quick_sort<T: Ord>(arr: &mut [T], lo: usize, hi: usize) {
    if lo < hi {
        let p = partition(arr, lo, hi);
        if p > 0 {
            _quick_sort(arr, lo, p - 1);
        }
        _quick_sort(arr, p + 1, hi);
    }
}

pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    if len > 1 {
        _quick_sort(arr, 0, len - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sorting::have_same_elements;
    use crate::sorting::is_sorted;
    use crate::sorting::sort_utils;

    #[test]
    fn basic() {
        let mut res = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
        let cloned = res.clone();
        quick_sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn basic_string() {
        let mut res = vec!["a", "bb", "d", "cc"];
        let cloned = res.clone();
        quick_sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn empty() {
        let mut res = Vec::<u8>::new();
        let cloned = res.clone();
        quick_sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn one_element() {
        let mut res = vec![1];
        let cloned = res.clone();
        quick_sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn pre_sorted() {
        let mut res = vec![1, 2, 3, 4];
        let cloned = res.clone();
        quick_sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn reverse_sorted() {
        let mut res = vec![4, 3, 2, 1];
        let cloned = res.clone();
        quick_sort(&mut res);
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn large_elements() {
        let mut res = sort_utils::generate_random_vec(300000, 0, 1000000);
        let cloned = res.clone();
        sort_utils::log_timed("large elements test", || {
            quick_sort(&mut res);
        });
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn nearly_ordered_elements() {
        let mut res = sort_utils::generate_nearly_ordered_vec(3000, 10);
        let cloned = res.clone();

        sort_utils::log_timed("nearly ordered elements test", || {
            quick_sort(&mut res);
        });

        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn repeated_elements() {
        let mut res = sort_utils::generate_repeated_elements_vec(1000000, 3);
        let cloned = res.clone();

        sort_utils::log_timed("repeated elements test", || {
            quick_sort(&mut res);
        });

        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }
}
