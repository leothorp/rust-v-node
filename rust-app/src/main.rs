#[macro_use] extern crate rocket;

use rocket::tokio;
use std::env;

struct Item {
    id: i32,
    name: String,
}

// Naive recursive Fibonacci function
fn fibonacci(n: i64) -> i64 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

#[get("/compute/fibonacci/<n>")]
fn compute_fibonacci(n: i64) -> String {
    let result = fibonacci(n);
    format!("Fibonacci({}) = {}", n, result)
}

// #[get("/item/<id>")]
// async fn get_item(id: i32, db: &rocket::State<PgPool>) -> Json<Item> {
//     let item = sqlx::query_as::<_, Item>("SELECT * FROM items WHERE id = $1")
//         .bind(id)
//         .fetch_one(&**db)
//         .await
//         .expect("Failed to fetch item");
//     Json(item)
// }

#[launch]
fn rocket() -> _ {
    // let db_url = env::var("DATABASE_URL").expect("Missing DATABASE_URL");
    // let db_pool = PgPool::connect_lazy(&db_url).expect("Failed to create pool");

    rocket::build()
        // .manage(db_pool)
        .mount("/", routes![compute_fibonacci])
}
