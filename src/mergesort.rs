use std::time::Instant;

use anyhow::Result;
use clap::Parser;
use parallel_mergesort::mergesort_mt;

fn generate_random_array(num_items: usize) -> Vec<i32> {
    let mut rval = Vec::with_capacity(num_items);
    for _ in 0..num_items {
        rval.push(rand::random());
    }
    rval
}

#[derive(Parser, Debug)]
struct Arguments {
    #[clap(
        short = 'n',
        long = "num_elements",
        help = "Number of Elements to generate"
    )]
    num_elements: usize,
    #[clap(
        short = 't',
        long = "num_threads",
        help = "Number of threads to use to sort the array"
    )]
    num_threads: usize,
}

fn main() -> Result<()> {
    let args = Arguments::parse();

    let mut arr = generate_random_array(args.num_elements);
    let mut arr_clone = arr.clone();
    arr_clone.sort();
    let start = Instant::now();
    mergesort_mt(&mut arr[..], args.num_threads);
    println!("{},{},{}", args.num_elements, args.num_threads, start.elapsed().as_millis());
    assert!(arr_clone.iter().eq(arr.iter())); 
    Ok(())
}
