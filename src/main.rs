mod async_await;
mod concurrency;
mod errors;
mod fundamentals;
mod iterators;
mod memory;
mod traits;
mod types;
mod welcome;

use warp::Filter;

/// GRADUATION PROJECT
///
/// In this free-form, open-ended exercise, you will use the Warp web framework to build a simple
/// REST API. The project's hello world is already implemented below.
///
/// By now, you should have enough experience with Rust that understanding the syntax and type
/// signatures of the Warp API should be straightforward.
///
/// Good luck, and congratulations on finishing the course!
#[tokio::main]
async fn main() {
    // GET /hello/{name} => 200 OK with body "Hello, warp!"
    let hello = 
        warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
