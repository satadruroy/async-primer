use reqwest::Error;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "https://jsonplaceholder.typicode.com/posts/1";
    let start_time = Instant::now();

    // let _ = reqwest::get(url).await?;
    // let _ = reqwest::get(url).await?;
    // let _ = reqwest::get(url).await?;
    // let _ = reqwest::get(url).await?;

    let (_, _, _, _) = tokio::join!(
        reqwest::get(url),
        reqwest::get(url),
        reqwest::get(url),
        reqwest::get(url)
    );

    let elapsed_time = start_time.elapsed();

    println!("Requests took {} ms", elapsed_time.as_millis());
    Ok(())
}
