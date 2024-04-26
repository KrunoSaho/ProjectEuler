fn count_frequencies(data: &[u32]) -> [u32; 10] {
    let mut frequencies = [0; 10];
    for &d in data {
        frequencies[d as usize] += 1;
    }
    frequencies
}

fn number_to_digits(n: u32) -> Vec<u32> {
    n.to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect::<Vec<_>>()
}

fn main() {
    for n in 1..1_000_000 {
        let nds = [2, 3, 4, 5, 6]
            .iter()
            .map(|x| number_to_digits(n * x))
            .map(|x| count_frequencies(x.as_slice()))
            .collect::<Vec<_>>();

        // ensure all are equal
        let contains = nds.iter().all(|x| x == &nds[0]);

        if contains {
            println!("{}", n);
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_frequencies() {
        let data = [1, 2, 5, 8, 7, 4];
        let frequencies = count_frequencies(&data);
        assert_eq!(frequencies, [0, 1, 1, 0, 1, 1, 0, 1, 1, 0]);

        let data = [1, 2, 2, 3, 5, 5];
        let frequencies = count_frequencies(&data);
        assert_eq!(frequencies, [0, 1, 2, 1, 0, 2, 0, 0, 0, 0]);
    }
}
