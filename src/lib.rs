// Variables


mod ownership;

pub fn variables_explaining() {
    let bunnies: i32 = 4;
    let carrots: i32 = 101;

    println!(
        "I have a total of {} bunnies and {} carrots\n",
        bunnies, carrots
    );
}

// Functions

pub fn do_stuff(quantity: i32, price: f32) {
    let final_price = quantity as f32 * price;

    println!("The full price is {}", final_price)
}


