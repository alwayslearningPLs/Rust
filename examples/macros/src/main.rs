macro_rules! factorial {
    ($x:expr) => {
        {
            let mut result = 1;
            for i in 1..($x+1) {
                result = result * i;
            }
            result
        }
    };
}

fn main() {
    let arg = std::env::args().nth(1).expect("enter a number to calculate the factorial")
        .parse::<u64>().expect("enter a number to calculate the factorial");

    println!("Result {}", factorial!(arg));
}

