use std::{io::{self, Write}};
use notify_rust::{Notification};

async fn _some() -> Result<(), Box<dyn std::error::Error>>  {
    println!("Hi! How often you wanna make a rest for your eyes (minutes)?   ");
    let mut eyes = String::new();

    let eyes = loop {
        eyes.clear();

        io::stdin()
            .read_line(&mut eyes)
            .unwrap();

        match eyes.trim().parse::<u64>() { 
            Ok(0) => {
                println!("Number must be more than 0! Try again:   ");
                continue;
            }
            Ok(n) => break n,
            Err(_) => {
                println!("Enter a number more than 0! Try again:   ");
                continue;
            }
        }
    };
    //=======================================
    //===============Water=====================
    println!("Ok! Now tell me: how often u wanna drink water (minutes)?   ");
    let mut water = String::new();

    let water = loop {
        water.clear();

        io::stdin()
            .read_line(&mut water)
            .unwrap();

        match water.trim().parse::<u64>() {
            Ok(0) => {
                println!("Number must be more than 0! Try again:  ");
                continue;
            },
            Ok(n) => break n,
            Err(_) => {
                println!("Enter a number more than 0! Try again:   ");
                continue;
            }
        }
    };
    //==========================================
    //===============Stop the work=================
    println!("Great! But when u wanna stop the work and get rest? (also at minutes)?   ");
    let mut walk = String::new();

    let walk = loop {
        io::stdin()
            .read_line(&mut walk)
            .unwrap();

        match walk.trim().parse::<u64>() {
            Ok(0) => {
                println!("Number must be more than 0! Try again:  ");
                continue;
            }
            Ok(n) => break n,
            Err(_) => {
                println!("Enter a number more than 0! Try again:   ");
                continue;
            }
        }
    };
    //============================================= 
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
            Notification::new()
                .summary("ETW")
                .body("Hey! U must go and take rest for eyes! ")
                .timeout(10000)
                .show()
                .ok();
        }
    });
    
    tokio::spawn(async move{
        for _ in 1..=need_wather {
            tokio::time::sleep(tokio::time::Duration::from_mins(water)).await;
            Notification::new()
                .summary("ETW")
                .body("Hey! U must go and drink some water!  ")
                .timeout(10000)
                .show()
                .ok();
        }
    });

    tokio::time::sleep(tokio::time::Duration::from_mins(walk)).await;

    Ok(())
    
}