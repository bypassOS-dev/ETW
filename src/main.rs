use std::{io::{self, Write}};

#[tokio::main]
async fn main() {
    //=================Eyes=================
    println!("Hi! How often you wanna make a rest for your eyes (minutes)?   ");
    let mut eyes = String::new();

    let eyes = loop {
        eyes.clear();

        io::stdin()
            .read_line(&mut eyes)
            .unwrap();
        let normal_eyes = match eyes.trim().parse::<u64>() { 
            Ok(0) => {
                println!("Number must be more than 0!");
                continue;
            }
            Ok(n) => n,
            Err(_) => {
                println!("Enter a number more than 0!");
                continue;
            }
        };

        break normal_eyes;
    };
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
        eprintln!("\n\n\n\n Problem ------>   Your work-time is too short \n\n\n\n\n");
        std::io::stdout().flush().ok();

        std::process::exit(1);
    }

    let need_eyes: u64 = walk / eyes;
    println!("{need_eyes} ---> 1");

    let need_wather:u64 = walk / water;
    println!("{need_wather} ---> 2");

    tokio::spawn(async move{
        for _  in 1..=need_eyes {
            tokio::time::sleep(tokio::time::Duration::from_mins(eyes)).await;
            println!("U need go and take rest for eyes!");
        }
    });
    
    tokio::spawn(async move{
        for _ in 1..=need_wather {
            tokio::time::sleep(tokio::time::Duration::from_mins(water)).await;
            println!("U need to go and drink water!");
        }
    });

    tokio::time::sleep(tokio::time::Duration::from_mins(walk)).await;
    
}