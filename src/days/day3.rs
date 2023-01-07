fn day3_part1(lines: &str) -> usize {
    let result = lines.split('\n').map(|line| {
        let str_half_index: usize = line.len() / 2;
        let (first, last) = line.split_at(str_half_index);
        let common_chars: Vec<char> = first.chars().filter(|x| last.contains(*x)).collect();
        let result = match common_chars.first() {
            Some(common_char) => get_points(*common_char),
            None => 0,
        };
        result
        // let result = common_chars.iter().map(|x| get_points(*x)).sum::<usize>();
    }).sum::<usize>();

    result
}

fn get_points(input: char) -> usize {
    let input_usize = input as usize;
    if input.is_lowercase() {
        return input_usize - 'a' as usize + 1;
    } else {
        return input_usize - 'A' as usize + 27;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_points() {
        assert_eq!(get_points('p'), 16)
    }

    #[test]
    fn test_day3_part1_sample() {
        let input = r##"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"##;
        assert_eq!(day3_part1(input), 26)
    }

    #[test]
    fn test_day3_part1_full() {
        let input = include_str!("../../input/day3_a.txt");

        assert_eq!(day3_part1(input), 26)
    }
}
