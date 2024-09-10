use warp::Filter;

#[tokio::main]
async fn main() {
    // get method by default
    // let hello = warp::path("hello").map(|| format!("Hello, World!"));
    let hello = warp::get().map(|| format!("Hello, World!"));


    warp::serve(hello)
        .run(([127,0,0,1], 3030))
        .await;
}
