// strings3.rs
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    let mut x: usize = 0;
    let mut y: usize = input.len();

    let asBytes = input.as_bytes();

    for i in 0..input.len() {
        if asBytes[i] as char != ' ' {
            break;
        }
        x += 1;
    }

    for i in 1..input.len() + 1 {
        if asBytes[input.len() - i] as char != ' ' {
            break;
        }
        y -= 1;
    }

    input[x..y].to_string()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    let mut string = input.to_string();
    string.push_str(" world!");
    String::from(string)
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    let mut string = "".to_string();
    let asBytes = input.as_bytes();

    let mut i: usize = 0;

    // const cars: String = "cars".chars().take(4).collect();

    while i <= input.len() - 4 {
        if &input[i..i + 4] == "cars" {
            println!("Matched");
            string.push_str("balloons");
            i += 4;
            continue;
        }
        string.push(asBytes[i] as char);
        i += 1;
    }

    string.push_str(&input[i..input.len()]);

    string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool"
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons"
        );
    }
}
