pub fn setupRecursionPractice() {
    let factorialNum = 3;
    let factorial = factorial(factorialNum);
    println!("factorial of {factorialNum} is {factorial}");

    let numFib = 2;
    let fibNum = fib(numFib);
    println!("The number at position {numFib} in the fib sequence is {fibNum}");
}  

fn factorial(num: i32) -> i32 {
    if num == 0 {
        return 0;
    }
    if num == 1 {
        return 1;
    }
    return num * factorial(num - 1);
}

fn fib(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}