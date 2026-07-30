use std::time::Duration;

async fn power_on() {
    println!("Power on a computher...");
    tokio::time::sleep(Duration::from_secs(2)).await;
    println!("A computher is working!");
}
async fn load_os() {
    let power_on = power_on().await;
    println!("A computher is loading OS...");
    tokio::time::sleep(Duration::from_secs(2)).await;
    println!("A OS is runing!");
}
async fn open_browser() {
    let load_os = load_os().await;
    println!("You opening your browser...");
    tokio::time::sleep(Duration::from_secs(2)).await;
    println!("You opened a browser!");
}
async fn watch_youtube() {
    let watch_youtube = open_browser().await;
    println!("You writeing a 'youtube.com'...");
    tokio::time::sleep(Duration::from_secs(2)).await;
    println!("Your computher was hacked a russian hacker lol!");
}

#[tokio::main]
async fn main() {
    let a = watch_youtube().await;
}