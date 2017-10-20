#[macro_use]
extern crate error_chain;
extern crate rayon;

use rayon::prelude::*;

error_chain!{}

fn run() -> Result<()> {
    let paragraph = String::from(
        "It was the best of times, it was the worst of times, \
                      it was the age of wisdom, it was the age of foolishness...",
    );
    let sentences: Vec<&str> = paragraph.split(" ").collect();

    let word_count: i32 = sentences.par_iter().map(|_x| 1).reduce(|| 0, |x, y| x + y);
    println!("Word count: {}", word_count);

    // filter - map - reduce example
    let v: Vec<i32> = vec![1, 2, 7, 3, 9, 3, 8, 4, 9, 11, 1, 2, 12];
    let expected = 1 + 4 + 9 + 9 + 16 + 1 + 4;
    let square_sum_lt_5 = v.par_iter().filter(|&&x| x < 5).map(|&x| x * x).reduce(
        || 0,
        |x, y| x + y,
    );
    println!(
        "Sum of squared values for elements less than 5 is {} (expected value: {})",
        square_sum_lt_5,
        expected
    );

    Ok(())
}

quick_main!(run);
