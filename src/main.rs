use rand::Rng;
use std::io;

fn generate_password(length: usize) -> String {
    let charset: Vec<char> =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_-+=<>?"
            .chars()
            .collect();
    let mut range = rand::thread_rng();
    let password: String = (0..length)
        .map(|_| range.gen_range(0..charset.len()))
        .map(|i| charset[i])
        .collect();
    password
}

fn main() {
    let mut count = String::new();
    println!("Enter length of password you want to generate:");
    io::stdin()
        .read_line(&mut count)
        .expect("Unable to read input.");
    let count = count.trim().parse().expect("Invalid input.");
    let password = generate_password(count);
    println!("Generated Password: {}", password);
}
