use std::io;


fn main() {
    println!("Enter your weight (kg): ");
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).unwrap();

    let weight = input.trim().parse().unwrap();

    let cov_weight = cal_weight_on_mars(weight);
    println!("Weight on Mars: {}kg", cov_weight);
}


fn cal_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}