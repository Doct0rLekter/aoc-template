fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    input.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_works() {
        let test_input = include_str!("./example1.txt");
        let result = part1(test_input);
        assert_eq!(result, result);
    }
}
