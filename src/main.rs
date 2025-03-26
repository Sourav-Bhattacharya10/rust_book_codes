mod chapter6;

// use chapter6::part_6_2::{Coin, UsState, value_in_cents};
// use chapter6::part_6_2::plus_one;

// fn add_fancy_hat() {
//     println!("Add fancy hat");
// }
// fn remove_fancy_hat() {
//     println!("Remove fancy hat");
// }
// fn re_roll(other: i32) {
//     println!("Re-roll {other}");
// }

use chapter6::part_6_2::{Coin, UsState, describe_state_quarter};

fn main() {
    // chapter6 -> part_6_2
    // let new_coin = Coin::Quarter(UsState::Alaska);
    // println!("The new coin in cents: {}", value_in_cents(new_coin));

    // let x: Option<i32> = Some(5);
    // println!("Plus one results in {:?}", plus_one(x));

    // let dice_roll = 10;
    // match dice_roll {
    //     3 => add_fancy_hat(),
    //     7 => remove_fancy_hat(),
    //     other => re_roll(other),
    // }

    // chapter6 -> part_6_3
    // let config_max = Some(3u8);
    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {max}"),
    //     _ => (),
    // };

    // let config_max: Option<u8> = Some(3);
    // if let Some(3) = config_max {
    //     println!("The maximum is configured to be 3");
    // } else if let Some(max) = config_max {
    //     println!("The maximum is configured to be {max}");
    // } else {
    //     println!("The maximum is not configured");
    // }

    let coin = Coin::Quarter(UsState::Alabama);
    println!("let else example: {:?}", describe_state_quarter(coin));

    let coin2 = Coin::Nickel;
    println!("let else example2: {:?}", describe_state_quarter(coin2));
}
