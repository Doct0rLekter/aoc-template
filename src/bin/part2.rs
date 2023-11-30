fn main() {
    let input = include_str!("../../input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    input.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_works() {
        let test_input = include_str!("../../example2.txt");
        let test_output = "".to_string();
        let result = part2(test_input);

        assert_eq!(test_output, result);
    }
}
