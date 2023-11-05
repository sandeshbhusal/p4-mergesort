use rayon::prelude::*;

const SIZE_THRESHOLD: usize = 5;
pub trait Item: PartialEq + Eq + Ord + PartialOrd + Send + std::fmt::Debug + Copy {}
impl Item for i32 {}
impl Item for u32 {}
impl Item for u8 {}

fn insertion_sort<T: Ord>(input: &mut [T]) {
    for i in 1..input.len() {
        let mut j = i;
        while j > 0 && input[j - 1] > input[j] {
            input.swap(j - 1, j);
            j -= 1;
        }
    }
}

fn parallel_mergesort<T: Item>(input: &mut [T]) {
    // If the input is too small, sort it with insertion sort and return.
    if input.len() <= SIZE_THRESHOLD {
        insertion_sort(input);
        return;
    }

    // For a sufficiently-large input, split it into two chunks and call parallel_mergesort recursively on both.
    let mid: usize = input.len() / 2;
    let (mut chunk1, mut chunk2) = input.split_at_mut(mid);
    parallel_mergesort(&mut chunk1);
    parallel_mergesort(&mut chunk2);

    // Allocate an extra vector to hold the chunks' sorted elements in sorted order.
    let chunk_vec = merge_sorted_chunks(chunk1, chunk2);
    chunk_vec
        .iter()
        .enumerate()
        .for_each(|(i, e)| input[i] = *e);
}

// Generate a larger vec from two chunks.
fn merge_sorted_chunks<T: Item>(chunk1: &[T], chunk2: &[T]) -> Vec<T> {
    let mut rval = Vec::with_capacity(chunk1.len() + chunk2.len());

    let mut i = 0;
    let mut j = 0;

    while i < chunk1.len() && j < chunk2.len() {
        if chunk1[i] < chunk2[j] {
            rval.push(chunk1[i]);
            i += 1;
        } else {
            rval.push(chunk2[j]);
            j += 1;
        }
    }

    if i < chunk1.len() {
        rval.extend_from_slice(&chunk1[i..]);
    } else {
        rval.extend_from_slice(&chunk2[j..]);
    }

    return rval;
}

/// Mergesort Multithreaded - sorts a given array with multiple threads.
pub fn mergesort_mt<T: Item>(input: &mut [T], num_threads: usize) {
    if input.len() <= 1 {
        return;
    }

    input
        .par_chunks_mut(num_threads.min(input.len()))
        .for_each(|mut t| parallel_mergesort(&mut t));

    let mut first_chunk_size = num_threads.min(input.len());
    let second_chunk_size = first_chunk_size;

    while first_chunk_size < input.len() {
        let first_chunk = &input[0..first_chunk_size]; // The first chunk to merge
        let second_chunk =
            &input[first_chunk_size..(first_chunk_size + second_chunk_size).min(input.len())]; // The second chunk to merge.
        let merged = merge_sorted_chunks(first_chunk, second_chunk); // Merged chunk

        first_chunk_size = (first_chunk_size + second_chunk_size).min(input.len()); // First chunk size increases, second chunk size is same.
                                                                                    // But the length of first chunk cannot exceed that of the string itself.
        input[0..first_chunk_size].copy_from_slice(&merged); // Copy 0..first_chunk_size to the input array
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29,
        ];
        let mut sorted = input_vector.clone();
        sorted.sort();

        mergesort_mt(&mut input_vector[..], 5);
        assert!(input_vector.iter().eq(sorted.iter()));
    }
}
