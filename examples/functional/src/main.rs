fn main() {
    test_some_functional();
    test_move();
    test_fn_fnmut_fnonce();
    test_iterating();
}

fn test_some_functional() {
    let add = |a, b| a + b;

    assert_eq!(add(1, 2), 3);
    // Range (1..10) implements IntoIterator
    assert_eq!((1..10).filter(|x| x % 2 == 0).collect::<Vec<u64>>(), vec![2, 4, 6, 8]);
    assert_eq!((1..10).map(|x| x * 2).collect::<Vec<u64>>(), vec![2, 4, 6, 8, 10, 12, 14, 16, 18]);
}

fn test_move() {
    // we must declare the times variable as mutable
    let mut times = 2;
    {
    // we must declare the borrow closure as mutable
    // and use another scope, so the compiler doesn't complain about double borrowing when we try
    // to assert the value.
        let mut borrow = |x| times += x;
        borrow(5);
    }
    assert_eq!(times, 7);

    // here move gets a copy of the variable times, that's because we get a warning.
    // Of course, the variable times must implement the Copy trait.
    let mut own = move |x| times += x;
    own(5);
    assert_eq!(times, 7);
}

fn test_fn_fnmut_fnonce() {
    assert_eq!(mapping(|x| x * 2), vec![2, 4, 6]);
    assert_eq!(mapping_mut(|x| x * 2), vec![2, 4, 6]);

    foreaching(|x| println!("{}", x));
}

fn mapping<F>(mapper: F) -> Vec<u64> where F: std::ops::Fn(&u64) -> u64 {
    let v: Vec<u64> = vec![1, 2, 3];

    // It is not needed to specify the type, but I want to.
    return v.iter().map::<u64, F>(mapper).collect::<Vec<u64>>();
}

fn mapping_mut<F>(mapper: F) -> Vec<u64> where F: std::ops::FnMut(&u64) -> u64 {
    let v: Vec<u64> = vec![1, 2, 3];

    // It is not needed to specify the type, but I want to.
    return v.iter().map::<u64, F>(mapper).collect::<Vec<u64>>();
}

fn foreaching<F>(foreach: F) where F: std::ops::Fn(&u64) {
    let v: Vec<u64> = vec![1, 2, 3];

    v.iter().for_each::<F>(foreach);
}

fn test_iterating() {
    let numbers = 1..5; // Range implements std::iterator::Iterator
    for i in numbers {
        println!("{}", i);
    }

    for i in 1..=5 {
        println!("{}", i);
    }

    for i in 1..=100 {
        println!("collatz for {} is {}", i, collatz(i));
    }

    for i in new_collatz(10).take(2) {
        println!("step: {}", i);
    }

    for i in new_collatz(10).skip(2) {
        println!("step: {}", i);
    }
}

fn collatz(number: u64) -> u64 {
    let mut count: u64 = 0;
    let mut tmp: u64 = number;

    while tmp != 1 {
        if tmp % 2 == 0 {
            tmp /= 2;
        } else {
            tmp = tmp * 3 + 1;
        }
        count += 1;
    }

    return count;
}

trait Compute {
    fn amount(&self) -> u64;
}

#[derive(Debug)]
struct Collatz {
    current: u64,
    end: u64,
}

impl Compute for Collatz {

    fn amount(&self) -> u64 {
        let mut count: u64 = 0;
        let mut tmp: u64 = self.current;

        while tmp != 1 {
            if tmp % 2 == 0 {
                tmp /= 2;
            } else {
                tmp = tmp * 3 + 1;
            }
            count += 1;
        }

        return count;
    }
}

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            return None;
        }

        if self.current % 2 == 0 {
            self.current /= 2;
        } else {
            self.current = self.current * 3 + 1;
        }

        return Some(self.current);
    }
}

fn new_collatz(input: u64) -> Collatz {
    Collatz {
        current: input,
        end: 1u64,
    }
}
