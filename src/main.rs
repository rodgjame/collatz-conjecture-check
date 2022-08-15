use clap::Parser;

/// Provide a number you want to check to see how it follows the Collatz conjecture
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // Value you want to test against the conjecture
    #[clap(short, long, value_parser, default_value_t = 5)]
    n: u128
}

fn collatz_function(n: u128) -> u128 {
    if n % 2 == 0 {
        return n / 2
    } else {
        return 3 * n + 1
    }
}

fn main() {
    let args = Args::parse();
    let mut results_vec = Vec::new();
    let mut n: u128 = args.n;
    results_vec.push(n);

    while n > 1 {
       n = collatz_function(n);
       results_vec.push(n); 
    }
    
    for x in results_vec {
        print!("{x}, ");
    }
}