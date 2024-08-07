use futures::executor::block_on;

fn main() {
    println!("Hello, world!");
    block_on(async_main());
}

#[derive(Debug)]
struct Song {}

async fn learn_song() -> Song {
    Song {}
}

async fn sing_song(_song: Song) {
    println!("Singing song")
}

async fn dance() {
    println!("Dancing")
}

async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    futures::join!(f1, f2);
}
