use std::collections::HashSet;

fn sieve_of_eratosthenes(n: u64) -> Vec<u64> {
    let mut primes = vec![true; n as usize + 1];
    primes[0] = false;
    primes[1] = false;
    let mut p = 2;
    while p * p <= n {
        if primes[p as usize] {
            let mut i = p * p;
            while i <= n {
                primes[i as usize] = false;
                i += p;
            }
        }
        p += 1;
    }
    primes
        .iter()
        .enumerate()
        .filter_map(|(i, &is_prime)| if is_prime { Some(i as u64) } else { None })
        .collect()
}

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn get_prime_count(masked_number: &[i8]) -> Vec<u64> {
    let mut list = vec![];

    for j in 0..10 {
        let mut number = 0_u64;

        for i in masked_number {
            if *i == -1 {
                number = number * 10 + j;
                continue;
            }

            let m = *i as u64;
            number = number * 10 + m;
        }

        if is_prime(number) {
            list.push(number);
        }
    }

    list
}

fn split_number(number: u64) -> Vec<i8> {
    let mut digits = Vec::new();
    let mut n = number;

    while n > 0 {
        digits.push((n % 10) as i8);
        n /= 10;
    }

    digits.reverse();
    digits
}

fn permute_array(arr: &mut [i8], l: usize, r: usize, set: &mut HashSet<Vec<i8>>) {
    if set.contains(arr) {
        return;
    }

    if l == r {
        set.insert(arr.to_vec());
    } else {
        for i in l..=r {
            arr.swap(l, i);
            permute_array(arr, l + 1, r, set);
            arr.swap(l, i);
        }
    }
}

fn insert_wildcard(number: &[i8], number_wildcards: u8) -> Vec<Vec<i8>> {
    let mut list = vec![];
    let mut number_mask = vec![0; number.len()];

    (1..=number_wildcards).for_each(|i| {
        number_mask[i as usize] = -1;
    });

    let mut perms = HashSet::new();
    permute_array(&mut number_mask, 0, number.len() - 1, &mut perms);

    for perm in perms {
        let mut new_number = number.to_vec();

        for (i, j) in perm.iter().enumerate() {
            if *j == -1 {
                new_number[i] = -1;
            }
        }

        list.push(new_number);
    }

    list
}

/*




MARK: - Main


*/
fn main() {
    let zeros = 6;

    let primes_list = sieve_of_eratosthenes(10_u64.pow(zeros) - 1);
    let cutup_primes = primes_list
        .iter()
        .filter(|x| **x > 10_u64.pow(zeros - 1))
        .map(|x| split_number(*x))
        .collect::<Vec<_>>();

    let t0 = std::time::Instant::now();

    let mut set = HashSet::new();

    for i in cutup_primes {
        let numbers = insert_wildcard(&i, 3);

        for n in numbers {
            let primes = get_prime_count(&n);

            if primes.len() < 8 || set.contains(&n) {
                continue;
            }

            println!("Prime: {:?} ", i);
            for j in primes {
                println!("{}", j);
            }

            set.insert(n);
        }
    }

    println!("Time: {}s", t0.elapsed().as_secs_f64());
}

/*





MARK: Tests

*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_wildcard() {
        let w0 = insert_wildcard(&[1, 2, 3], 1);
        let eql = vec![vec![-1, 2, 3], vec![1, -1, 3], vec![1, 2, -1]];
        let contains = w0.iter().all(|x| eql.contains(x));
        assert_eq!(contains, true);

        let w1 = insert_wildcard(&[1, 2, 3], 2);
        let eql = vec![
            vec![-1, -1, 3],
            vec![-1, 2, -1],
            vec![1, -1, -1],
            vec![-1, 2, 3],
            vec![1, -1, 3],
            vec![1, 2, -1],
        ];

        let contains = w1.iter().all(|x| eql.contains(x));
        assert_eq!(contains, true);
    }

    #[test]
    fn test_permute_array() {
        let mut arr = [1, -1, -1];

        let mut set = HashSet::new();
        permute_array(&mut arr, 0, 2, &mut set);

        println!("{:?}", set);
    }

    #[test]
    fn test_get_prime_count() {
        assert_eq!(get_prime_count(&[-1, 3]), vec![3, 13, 23, 43, 53, 73, 83]);
        assert_eq!(
            get_prime_count(&[5, 6, -1, -1, 3]),
            vec![56003, 56113, 56333, 56443, 56663, 56773, 56993]
        );
    }

    #[test]
    fn test_split_number() {
        assert_eq!(split_number(123), vec![1, 2, 3]);
        assert_eq!(split_number(123456789), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
