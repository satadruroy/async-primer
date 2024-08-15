use std::thread;
use std::time::Duration;
use std::time::Instant;
use tokio::time::sleep;

async fn prep_coffee_mug() {
    println!("Pouring milk");
    thread::sleep(Duration::from_secs(3));
    println!("Milk poured");
    println!("Puring instant coffee");
    thread::sleep(Duration::from_secs(3));
}

async fn make_coffee() {
    println!("Boiling water");
    sleep(Duration::from_secs(3)).await;
    println!("water boiled");
    println!("Pouring boiled water");
    thread::sleep(Duration::from_secs(3));
    println!("Boiled water poured");
}

async fn make_toast() {
    println!("putting bread in toaster");
    sleep(Duration::from_secs(3)).await;
    println!("Bread toasted");
    println!("Buttering bread");
    sleep(Duration::from_secs(3)).await;
    println!("Bread buttered");
}
#[tokio::main]
async fn main() {
    println!("starting breakfast");
    let start_time = Instant::now();
    let coffee_mug_step = prep_coffee_mug();
    let make_coffee_step = make_coffee();
    let toast_step = make_toast();

    tokio::join!(make_coffee_step, coffee_mug_step, toast_step);
    let elapsed_time = start_time.elapsed();
    println!("It took {} secs", elapsed_time.as_secs());
}
