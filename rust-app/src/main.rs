#[macro_use] extern crate rocket;

// Naive recursive Fibonacci function
fn fibonacci(n: i64) -> i64 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

#[get("/fibonacci/<n>")]
fn compute_fibonacci(n: i64) -> String {
    let result = fibonacci(n);
    format!("Fibonacci({}) = {}", n, result)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![compute_fibonacci])
}
