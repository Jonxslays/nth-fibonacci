use std::io::{stdin, stdout, Write};


fn main() {
    let mut input = String::new();

    print!("Calculate Nth fibonacci sequence, enter N: ");

    flush_out();
    read_in(&mut input);

    println!("Calculating....");
    let input = convert(&mut input);

    println!("#{} in the sequence is {}.", input, calculate(input));
}

fn calculate(mut n: u64) -> u64 {
    let mut first = 0;
    let mut second = 1;
    let mut output = 0;

    if n == 1 {
        output = 0;
    } else {
        while n > 1 {
            output = first + second;
            first = second;
            second = output;
            n -= 1;
        }
    }

    output
}

fn convert(input: &mut String) -> u64 {
    input.trim().parse().expect("Failed to convert to u64.")
}

// Flushes stdout
fn flush_out() {
    stdout().flush()
        .expect("Failed to flush stdout.");
}

// Reads text from stdin
fn read_in(input: &mut String) {
    stdin().read_line(input)
        .expect("Failed to read stdin.");
}