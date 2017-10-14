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

    Ok(())
}

quick_main!(run);
