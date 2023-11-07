use rayon::prelude::*;

// Minimum size threshold at which mergesort will convert to insertion sort.
const SIZE_THRESHOLD: usize = 5;

// All sortable slices must satisfy the following constraints:
// 1. Ord - All items of this type must be comparable and orderable.
// 2. Debug - So that we can have nice dbg!() invocations in tests.
// 3. Send - Should be safe to send across threads.
// 4. Clone - Should be cloneable to store in temporary area before merge.
pub trait Item: Ord + std::fmt::Debug + Send + Clone {}

// Blanket implementation for all items that implement the Item trait's subtraits.
impl<T> Item for T where T: Ord + std::fmt::Debug + Send + Clone {}

// Insertion sort the given mutable slice.
fn insertion_sort<T: Ord>(input: &mut [T]) {
    for i in 1..input.len() {
        let mut j = i;
        while j > 0 && input[j - 1] > input[j] {
            input.swap(j - 1, j);
            j -= 1;
        }
    }
}

// Normal mergesort. This function takes in a slice and calls single-threded
// mergesort on it repeatedly.
fn parallel_mergesort<T: Item>(input: &mut [T]) {
    // If the input is too small, sort it with insertion sort and return.
    if input.len() <= SIZE_THRESHOLD {
        insertion_sort(input);
        return;
    }

    // For a sufficiently-large input, split it into two chunks
    // and call parallel_mergesort recursively on both.
    let mid: usize = input.len() / 2;
    let (mut chunk1, mut chunk2) = input.split_at_mut(mid);
    parallel_mergesort(&mut chunk1);
    parallel_mergesort(&mut chunk2);

    // Allocate an extra vector to hold the chunks' sorted elements in sorted order.
    let chunk_vec = merge_sorted_chunks(chunk1, chunk2);
    input[..].clone_from_slice(&chunk_vec);
}

// Generate a larger vec from two chunks.
fn merge_sorted_chunks<T: Item>(chunk1: &[T], chunk2: &[T]) -> Vec<T> {
    let mut rval = Vec::with_capacity(chunk1.len() + chunk2.len());

    let mut i = 0;
    let mut j = 0;

    while i < chunk1.len() && j < chunk2.len() {
        if chunk1[i] < chunk2[j] {
            rval.push(chunk1[i].clone()); // calling ".clone()" for primitives is
                                          // effectively the same as derefencing them (.copy()).
                                          // See https://doc.rust-lang.org/src/core/clone.rs.html.
            i += 1;
        } else {
            rval.push(chunk2[j].clone());
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
/// input - the input slice to sort.
/// num_threads - the number of threads to use to sort this slice.
///
/// `mergesort_mt` can sort slices of types that implement the [`Item`] trait.
pub fn mergesort_mt<T: Item>(input: &mut [T], num_threads: usize) {
    if input.len() <= 1 {
        return;
    }

    let input_len = input.len();
    let num_chunks = num_threads.min(input.len());
    let chunk_size = input_len / num_chunks;
    let mut left_chunk_len = chunk_size;

    input
        .par_chunks_mut(chunk_size)
        .for_each(|mut t| {
            parallel_mergesort(&mut t)
        });


    while left_chunk_len < input_len {
        let (left, right) = input.split_at_mut(left_chunk_len);
        let right = &mut right[..chunk_size.min(input_len - left_chunk_len)];

        let mut i = left.len() - 1;
        let mut j = 0;

        while j < right.len() && left[i] > right[j] {
            let temp = left[i].clone();
            left[i] = right[j].clone();
            right[j] = temp.clone();

            i -= 1;
            j += 1;
        }

        // At this point, the left chunk contains all elements smaller than the right chunk.
        // Sort the chunks individually.
        parallel_mergesort(left);
        parallel_mergesort(right);

        // Increase left length
        left_chunk_len = left.len() + right.len();
    }

}
