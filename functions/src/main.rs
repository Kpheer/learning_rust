

fn main() {
    println!("Hello, world!");

    another_function();
    another_function_num(34);

    println!("5 + 1 = {}", plus_one(5));

    println!("Fibo of 15 is: {}", fibo(15));
}

fn another_function(){
    println!("Another function");
}

fn another_function_num(num: i32){
    //This function recieves a signed i32 called num

    println!("Number recieved: {}", num);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn fibo(n: isize) -> isize {
    if n < 2 {
        return n;
    }
    return fibo(n - 1) + fibo(n - 2);
}