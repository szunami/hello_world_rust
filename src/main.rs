fn main() {
    println!("Hello, world!");
}

fn add() -> i32 {
    return 5;
}

fn test() -> String {
    return String::from("asdf");
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(), 3);
    }

    #[test]
    fn test_test() {
        assert_eq!(add(), String::from("asdf"));
    }
}
