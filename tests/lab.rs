//! Tests for the mergesort module.
#[cfg(test)]
mod mergesort_my_tests {
    use parallel_mergesort::mergesort_mt;

    #[cfg(test)]
    mod tests {
        use super::*;

        fn generate_random_array(num_items: usize) -> Vec<i32> {
            let mut rval = Vec::with_capacity(num_items);
            for _ in 0..num_items {
                rval.push(rand::random());
            }
            rval
        }

        #[test]
        fn mergesort_mt_random() {
            for _ in 0..5 {
            // Randomly generate an arary of upto 1M items.
            let num_items = rand::random::<usize>() % 1_000_000;
            let mut arr = generate_random_array(num_items);
            let mut sorted_arr = arr.clone();
            sorted_arr.sort();
            mergesort_mt(&mut arr[..], 16);

            assert!(arr.iter().eq(sorted_arr.iter()));
            }
        }

        #[test]
        fn mergesort_mt_works() {
            let mut input_vector = vec![
                10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, -1, -2, -3, -4, -5, -6, -7, -8, -9, -10,
            ];
            let mut sorted_vector = input_vector.clone();
            sorted_vector.sort();
            mergesort_mt(&mut input_vector[..], 2);

            assert!(input_vector.iter().eq(sorted_vector.iter()));
        }

        #[test]
        fn mergesort_mt_10_threads() {
            let mut input_vector = vec![
                10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, -1, -2, -3, -4, -5, -6, -7, -8, -9, -10,
            ];
            let mut sorted_vector = input_vector.clone();
            sorted_vector.sort();
            mergesort_mt(&mut input_vector[..], 10);

            assert!(input_vector.iter().eq(sorted_vector.iter()));
        }

        #[test]
        fn mergesort_mt_shortest_possible() {
            let mut input_vector = vec![2, 1];
            let mut sorted = input_vector.clone();
            sorted.sort();

            mergesort_mt(&mut input_vector[..], 2);
            assert!(input_vector.iter().eq(sorted.iter()));
        }

        #[test]
        fn mergesort_mt_odd_length_array() {
            let mut input_vector = vec![2, 1, 3];
            let mut sorted = input_vector.clone();
            sorted.sort();

            mergesort_mt(&mut input_vector[..], 2);
            assert!(input_vector.iter().eq(sorted.iter()));
        }

        #[test]
        fn mergesort_mt_larger_array() {
            let mut input_vector = vec![
                10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                24, 25, 26, 27, 28, 29,
            ];
            let mut sorted = input_vector.clone();
            sorted.sort();

            mergesort_mt(&mut input_vector[..], 5);
            assert!(input_vector.iter().eq(sorted.iter()));
        }

        #[test]
        fn mergesort_mt_unsorted_array_small() {
            let mut unsorted_even = (0..10).collect::<Vec<i32>>();
            let mut unsorted_odd = (0..13).collect::<Vec<i32>>();

            let mut sorted_even = unsorted_even.clone();
            let mut sorted_odd = unsorted_odd.clone();
            sorted_even.sort();
            sorted_odd.sort();

            mergesort_mt(&mut unsorted_even, 5);
            mergesort_mt(&mut unsorted_odd, 5);

            assert!(unsorted_even.iter().eq(sorted_even.iter()));
            assert!(unsorted_odd.iter().eq(sorted_odd.iter()));
        }
    }
}
