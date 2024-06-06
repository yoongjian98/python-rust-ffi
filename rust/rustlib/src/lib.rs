pub fn fibonacci(nth: u32) -> u32 {
    if nth <= 2 {
        return 1;
    }

    fibonacci(nth - 1) + fibonacci(nth - 2)
}