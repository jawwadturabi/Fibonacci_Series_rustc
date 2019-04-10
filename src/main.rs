use std::io;

fn main() {
    println!("\t\t\t\tFibonacci generator\n");
    println!("Type \"quit\" to end the program");

    let mut fib3 : u32;
    let mut fib1=0;
    let mut fib2=1;
    
    loop {
        let mut n = String::new();

        println!("\nEnter a positive integer:");

        io::stdin().read_line(&mut n).expect("Failed to read line");

        if n.trim() == "quit" {
            break;
        }

        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        let mut c=2;
        println!("The Fibonacci Series for 1st {} numbers is below : \n",n);
        println!("{} ",fib1);
        println!("{} ",fib2);
        while c<n{
            fib3=fib1 + fib2;
            c=c+1;
        println!("{}",fib3);
        fib1=fib2;
        fib2=fib3;
        }
        
    }
}