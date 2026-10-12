pub async fn make_check(eyes: String, water: String, walk: String) -> (bool, bool, bool)  {
    //=============================================
    //====First check: Is input numbers?============
    let mut eyes_bool = match eyes.trim().parse::<u64>() { 
            Ok(0) => false,
            Ok(_) => true,
            Err(_) => false,
    };

    let mut water_bool = match water.trim().parse::<u64>() {
            Ok(0) => false,
            Ok(_) => true,
            Err(_) => false,
    };

    let walk_bool = match walk.trim().parse::<u64>() {
        Ok(0) => false,
        Ok(_) => true,
        Err(_) => false,
    };
    //============================================= 
    //====Second check: is NUMBERS correct?========
    //=============================================

    if eyes_bool && walk_bool && water_bool {
        let walk = walk.trim().parse::<u64>().unwrap();       // It's not dangerous
        let water = water.trim().parse::<u64>().unwrap();
        let eyes = eyes.trim().parse::<u64>().unwrap();

        if walk < water {
            water_bool = false
        }
        if walk < eyes {
            eyes_bool = false
        }
    }
    
    (walk_bool, water_bool, eyes_bool)
    
}