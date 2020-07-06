use itertools::Itertools;
use lazy_static::*;
use std::cmp::max;
use std::collections::{HashMap, HashSet};

lazy_static! {
    static ref PRIMES: PrimeCache = PrimeCache::new(1000000);
}

pub fn p1() -> i32 {
    (0..1000).filter(|n| n % 3 == 0 || n % 5 == 0).sum()
}

pub fn p2() -> i32 {
    let limit = 4000000;
    (1..limit)
        .scan((1, 0), |acc, _| {
            *acc = (acc.0 + acc.1, acc.0);
            Some(*acc)
        })
        .map(|x| x.0)
        .filter(|x| x % 2 == 0)
        .take_while(|&x| x < limit)
        .sum()
}

pub fn p3() -> i32 {
    let mut current_prime: i64 = 2;
    let mut current_number: i64 = 600851475143;

    while current_prime < current_number {
        if current_number % current_prime == 0 {
            current_number = current_number / current_prime;
        } else {
            current_prime = match PRIMES.next_prime(current_prime) {
                None => panic!("no more primes"),
                Some(p) => p as i64,
            }
        }
    }

    current_prime as i32
}

pub fn p4() -> i32 {
    let mut largest = 0;

    (100..1000)
        .cartesian_product(100..1000)
        .map(|(a, b)| a * b)
        .filter(|&x| is_palindrome(x))
        .for_each(|x| largest = max(x, largest));

    largest
}

pub fn p5() -> i64 {
    let max_factor = 20;
    let mut highest_prime_power = HashMap::new();

    let mut current_prime: i64 = 2;
    while current_prime < max_factor {
        let mut current_prime_power = 0;
        while current_prime.pow(current_prime_power + 1) < max_factor {
            current_prime_power += 1;
        }

        let highest = match highest_prime_power.get(&current_prime) {
            None => 0,
            Some(cur) => *cur,
        };
        highest_prime_power.insert(current_prime, max(current_prime_power, highest));
        current_prime = PRIMES.next_prime(current_prime).unwrap();
    }
    let mut product: i64 = 1;
    for (prime, power) in highest_prime_power {
        product *= prime.pow(power);
    }
    product
}

pub fn p6() -> i32 {
    let a: i32 = (1..101).map(|x: i32| x.pow(2)).sum();
    let b: i32 = ((1..101).sum::<i32>()).pow(2);
    b - a
}

pub fn p7() -> i64 {
    PRIMES.at_index(10000).unwrap()
}

fn is_palindrome(val: i32) -> bool {
    let v = val.to_string();

    let v1 = &v[0..(v.len() / 2)];
    let v2 = &v[(v.len() / 2)..v.len()];

    v1 == v2.chars().rev().collect::<String>()
}

struct PrimeCache {
    cache: Vec<i64>,
    cache_set: HashSet<i64>,
}

impl PrimeCache {
    fn new(end: i64) -> PrimeCache {
        println!("Populating primes up to: {}", end);

        let mut c = PrimeCache {
            cache: Vec::new(),
            cache_set: HashSet::new(),
        };

        let mut confirmed_non_prime: Vec<bool> = vec![false; end as usize];
        for i in 2..end {
            if !confirmed_non_prime[i as usize] {
                c.cache.push(i);
                c.cache_set.insert(i);
                let mut j = i;
                while j < end {
                    confirmed_non_prime[j as usize] = true;
                    j += i;
                }
            }
        }
        c
    }

    fn is_prime(&self, val: i64) -> bool {
        self.cache_set.contains(&val)
    }

    fn next_prime(&self, val: i64) -> Option<i64> {
        if !self.is_prime(val) {
            return None;
        }
        let position = self.cache.iter().position(|&x| x == val)?;
        self.at_index(position + 1)
    }

    fn at_index(&self, i: usize) -> Option<i64> {
        Some(*self.cache.get(i)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_p1() {
        assert_eq!(p1(), 233168);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(), 4613732);
    }

    #[test]
    fn test_p3() {
        assert_eq!(p3(), 6857);
    }

    #[test]
    fn test_p4() {
        assert_eq!(p4(), 906609);
    }

    #[test]
    fn test_p5() {
        assert_eq!(p5(), 232792560);
    }

    #[test]
    fn test_p6() {
        assert_eq!(p6(), 25164150);
    }

    #[test]
    fn test_p7() {
        assert_eq!(p7(), 104743);
    }
}
