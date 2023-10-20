#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_length() {
        let mut count = String::new();
        println!("Enter length of password you want to generate:");
        io::stdin()
            .read_line(&mut count)
            .expect("Unable to read input.");
        let count = count.trim().parse().expect("Invalid input.");
        let password = generate_password(count);
        assert_eq!(password.len(), 8);
    }

    #[test]
    fn test_password_contains_special_chars() {
        let mut count = String::new();
        println!("Enter length of password you want to generate:");
        io::stdin()
            .read_line(&mut count)
            .expect("Unable to read input.");
        let count = count.trim().parse().expect("Invalid input.");
        let password = generate_password(count);
        assert!(password.contains(|c: char| "!@#$%^&*()_-+=<>?".contains(c)));
    }
}
