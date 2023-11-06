use anyhow::Result;
use parallel_mergesort::mergesort_mt;

fn generate_random_array(num_items: usize) -> Vec<i32> {
    let mut rval = Vec::with_capacity(num_items);
    for _ in 0..num_items {
        rval.push(rand::random());
    }
    rval
}

fn main() -> Result<()> {
    for _ in 0..5 {
        // Randomly generate an arary of upto 1M items.
        let num_items = rand::random::<usize>() % 1000000;
        println!("Testing for {} items", num_items);
        let mut arr = generate_random_array(num_items);
        let mut sorted_arr = arr.clone();
        sorted_arr.sort();
        mergesort_mt(&mut arr[..], 16);

        assert!(arr.iter().eq(sorted_arr.iter()));
    }

    Ok(())
}
