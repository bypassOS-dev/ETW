use std::io;

#[tokio::main]
async fn main() {
    //=================Eyes=================
    println!("Hi! How often you wanna make a rest for your eyes (minutes)?   ");
    let mut eyes = String::new();

    io::stdin()
        .read_line(&mut eyes)
        .unwrap();

    let eyes = eyes.trim().parse::<u64>().unwrap();
    //=======================================
    //===============Water=====================
    println!("Ok! Now tell me: how often u wanna drink water (minutes)?   ");
    let mut water = String::new();

    io::stdin()
        .read_line(&mut water)
        .unwrap();

    let water = water.trim().parse::<u64>().unwrap();
    //==========================================
    //===============Stop the work=================
    println!("Great! But when u wanna stop the work and get rest? (also at minutes)?   ");
    let mut walk = String::new();

    io::stdin()
        .read_line(&mut walk)
        .unwrap();

    let walk = walk.trim().parse::<u64>().unwrap();
    //============================================= 

    if eyes > walk || water > walk {
        println!("Your work-time is too short");
        panic!(); // It's bad. I know
    }

    let need_eyes: u64 = walk / eyes;
    println!("{need_eyes} ---> 1");

    let need_wather:u64 = walk / water;
    println!("{need_wather} ---> 2");

    for i  in 1..=need_eyes {
        tokio::spawn(async move{
            tokio::time::sleep(tokio::time::Duration::from_mins(eyes)).await;
            println!("U need go and take rest for eyes!");
        });
    }

    for i in 1..need_wather {
        tokio::spawn(async move{
            tokio::time::sleep(tokio::time::Duration::from_mins(eyes)).await;
            println!("U need to go and drink water!");
        });
    }
    
}