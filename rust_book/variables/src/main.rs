mod references;
mod structs;

use structs::dummy;

fn main() {
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut v[2];

    let num2: &i32 = num;
    println!("{} {}", *num, *num2);

    dummy();
}
