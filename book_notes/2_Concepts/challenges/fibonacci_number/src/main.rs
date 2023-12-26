use std::io;
fn fibonnacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut prev = 0;
    let mut next = 1;

    for _ in 2..=n {
        let temp = next;
        next = prev + next;
        prev = temp;
    }
    next
}

fn main() {
    loop {
        println!("Enter the value of n for Fibonacci sequence:");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let n: u32 = input.trim().parse().expect("Entter a valid number");
        println!("The {}th Fibonacci number is : {}", n, fibonnacci(n));
    }
}
