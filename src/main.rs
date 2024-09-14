use std::io;

fn main() {
    let inp = get_init();
    let res = fib(inp);
    println!("Result: {res}");
}

fn fib(x : u32) -> u32 {
    if x < 2 {
        return 1;
    }
    fib(x-1) + fib(x-2)
}

fn get_init() -> u32{
    let mut inp = String::new();
    loop {
        println!("Enter an integer:");
        io::stdin().read_line(&mut inp).expect("Failed to read line");
        let inp: u32 = match inp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return inp; 
    }
}
