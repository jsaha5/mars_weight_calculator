use std::io;

fn main() {
    let mut user_weight = String::new();
    println!("What is your weight in KG ?");
    io::stdin().read_line(&mut user_weight).unwrap();
    println!("Your weight: {}", user_weight);

    let weight_on_mars = calculate_weight_on_mars(user_weight.trim().parse().unwrap());
    println!("Weight on Mars {} kg", weight_on_mars);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}