#[macro_use]
extern crate error_chain;
extern crate rayon;

use rayon::prelude::*;

error_chain!{}

struct Person {
    age: i32,
}

fn run() -> Result<()> {

    let v: Vec<Person> = vec![
        Person { age: 23 },
        Person { age: 19 },
        Person { age: 42 },
        Person { age: 17 },
        Person { age: 17 },
        Person { age: 31 },
        Person { age: 30 },
    ];

    let gt_30 = |x: i32| x > 30;
    let avg_age = v.par_iter()
        .filter(|&ref x| gt_30(x.age))
        .map(|ref x| x.age)
        .reduce(|| 0, |x, y| x + y) as f64 /
        v.par_iter().filter(|&ref x| gt_30(x.age)).count() as f64;
    println!("The average age of people older than 30 is {}", avg_age);
    Ok(())
}

quick_main!(run);
