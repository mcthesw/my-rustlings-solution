// option1.rs
// Make me compile! Execute `rustlings hint option1` for hints

use std::convert::TryInto;

// you can modify anything EXCEPT for this function's sig
fn print_number(maybe_number: Option<u16>) {
    println!("printing: {}", maybe_number.unwrap());
}

fn main() {
    print_number(Some(13));
    print_number(Some(99));

    let mut numbers= vec![Option::<u16>::None; 5];
    for iter in 0..5 {
        let number_to_add: u16 = {
            (((iter * 1235) + 2) / (4 * 16)).try_into().unwrap()
        };

        numbers[iter as usize] = Some(number_to_add);
    }
}
