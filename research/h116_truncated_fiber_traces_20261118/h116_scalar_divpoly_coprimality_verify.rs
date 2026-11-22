//! Independent Rust replay for H116-SCALAR-DIVPOLY-COPRIMALITY-001.
//!
//! This implementation uses signed residues, an extended-Euclid inverse and
//! an owned polynomial type. It imports no producer code.

use std::collections::HashMap;
use std::fs;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Polynomial {
    coefficients: Vec<i64>,
    prime: i64,
}

impl Polynomial {
    fn new(coefficients: Vec<i64>, prime: i64) -> Self {
        let mut result = Self {
            coefficients: coefficients.into_iter().map(|value| value.rem_euclid(prime)).collect(),
            prime,
        };
        result.trim();
        result
    }

    fn zero(prime: i64) -> Self {
        Self::new(vec![0], prime)
    }

    fn one(prime: i64) -> Self {
        Self::new(vec![1], prime)
    }

    fn trim(&mut self) {
        while self.coefficients.len() > 1 && self.coefficients.last() == Some(&0) {
            self.coefficients.pop();
        }
    }

    fn is_zero(&self) -> bool {
        self.coefficients == [0]
    }

    fn degree(&self) -> usize {
        self.coefficients.len() - 1
    }

    fn leading(&self) -> i64 {
        *self.coefficients.last().expect("polynomial nonempty")
    }

    fn sub(&self, other: &Self) -> Self {
        assert_eq!(self.prime, other.prime);
        Self::new(
            (0..self.coefficients.len().max(other.coefficients.len()))
                .map(|index| {
                    self.coefficients.get(index).copied().unwrap_or(0)
                        - other.coefficients.get(index).copied().unwrap_or(0)
                })
                .collect(),
            self.prime,
        )
    }

    fn mul(&self, other: &Self) -> Self {
        assert_eq!(self.prime, other.prime);
        if self.is_zero() || other.is_zero() {
            return Self::zero(self.prime);
        }
        let mut product = vec![0_i64; self.coefficients.len() + other.coefficients.len() - 1];
        for (i, &left) in self.coefficients.iter().enumerate() {
            for (j, &right) in other.coefficients.iter().enumerate() {
                product[i + j] = (product[i + j] + left * right).rem_euclid(self.prime);
            }
        }
        Self::new(product, self.prime)
    }

    fn scale(&self, scalar: i64) -> Self {
        Self::new(self.coefficients.iter().map(|value| value * scalar).collect(), self.prime)
    }

    fn shift(&self) -> Self {
        let mut coefficients = vec![0];
        coefficients.extend_from_slice(&self.coefficients);
        Self::new(coefficients, self.prime)
    }

    fn remainder(&self, denominator: &Self) -> Self {
        assert_eq!(self.prime, denominator.prime);
        assert!(!denominator.is_zero());
        let mut remainder = self.clone();
        let inverse_lead = inverse_mod(denominator.leading(), self.prime);
        while !remainder.is_zero() && remainder.degree() >= denominator.degree() {
            let shift = remainder.degree() - denominator.degree();
            let coefficient = (remainder.leading() * inverse_lead).rem_euclid(self.prime);
            let mut subtractor = vec![0; shift];
            subtractor.extend(denominator.coefficients.iter().map(|term| term * coefficient));
            remainder = remainder.sub(&Self::new(subtractor, self.prime));
        }
        remainder
    }

    fn exact_div(&self, denominator: &Self) -> Self {
        assert!(!denominator.is_zero());
        let mut remainder = self.clone();
        let mut quotient = vec![0_i64; self.degree() - denominator.degree() + 1];
        let inverse_lead = inverse_mod(denominator.leading(), self.prime);
        while !remainder.is_zero() && remainder.degree() >= denominator.degree() {
            let shift = remainder.degree() - denominator.degree();
            let coefficient = (remainder.leading() * inverse_lead).rem_euclid(self.prime);
            quotient[shift] = coefficient;
            let mut subtractor = vec![0; shift];
            subtractor.extend(denominator.coefficients.iter().map(|term| term * coefficient));
            remainder = remainder.sub(&Self::new(subtractor, self.prime));
        }
        assert!(remainder.is_zero(), "division polynomial recurrence division was not exact");
        Self::new(quotient, self.prime)
    }

    fn monic_gcd(&self, other: &Self) -> Self {
        let mut left = self.clone();
        let mut right = other.clone();
        while !right.is_zero() {
            let remainder = left.remainder(&right);
            left = right;
            right = remainder;
        }
        if left.is_zero() {
            left
        } else {
            left.scale(inverse_mod(left.leading(), left.prime))
        }
    }

    fn csv(&self) -> String {
        self.coefficients.iter().map(i64::to_string).collect::<Vec<_>>().join(",")
    }
}

fn inverse_mod(value: i64, prime: i64) -> i64 {
    let (mut old_r, mut r) = (value.rem_euclid(prime), prime);
    let (mut old_s, mut s) = (1_i64, 0_i64);
    while r != 0 {
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_s, s) = (s, old_s - quotient * s);
    }
    assert_eq!(old_r, 1);
    old_s.rem_euclid(prime)
}

#[derive(Clone)]
struct DivisionTerm {
    polynomial: Polynomial,
    carries_y: bool,
}

impl DivisionTerm {
    fn multiply(&self, other: &Self, cubic: &Polynomial) -> Self {
        let mut polynomial = self.polynomial.mul(&other.polynomial);
        if self.carries_y && other.carries_y {
            polynomial = polynomial.mul(cubic);
        }
        Self { polynomial, carries_y: self.carries_y ^ other.carries_y }
    }

    fn power(&self, mut exponent: usize, cubic: &Polynomial) -> Self {
        let mut base = self.clone();
        let mut result = Self { polynomial: Polynomial::one(cubic.prime), carries_y: false };
        while exponent > 0 {
            if exponent & 1 == 1 {
                result = result.multiply(&base, cubic);
            }
            exponent >>= 1;
            if exponent > 0 {
                base = base.multiply(&base, cubic);
            }
        }
        result
    }

    fn subtract(&self, other: &Self) -> Self {
        assert_eq!(self.carries_y, other.carries_y);
        Self { polynomial: self.polynomial.sub(&other.polynomial), carries_y: self.carries_y }
    }
}

fn divide_by_two_y(term: &DivisionTerm, cubic: &Polynomial) -> DivisionTerm {
    let half = inverse_mod(2, cubic.prime);
    if term.carries_y {
        DivisionTerm { polynomial: term.polynomial.scale(half), carries_y: false }
    } else {
        DivisionTerm { polynomial: term.polynomial.exact_div(cubic).scale(half), carries_y: true }
    }
}

fn scalar_map(scalar: usize, a: i64, b: i64, prime: i64) -> (Polynomial, Polynomial) {
    let cubic = Polynomial::new(vec![b, a, 0, 1], prime);
    let psi3 = Polynomial::new(vec![-a * a, 12 * b, 6 * a, 0, 3], prime);
    let psi4_core = Polynomial::new(
        vec![-8 * b * b - a * a * a, -4 * a * b, -5 * a * a, 20 * b, 5 * a, 0, 1],
        prime,
    );
    let mut terms = vec![
        DivisionTerm { polynomial: Polynomial::zero(prime), carries_y: false },
        DivisionTerm { polynomial: Polynomial::one(prime), carries_y: false },
        DivisionTerm { polynomial: Polynomial::new(vec![2], prime), carries_y: true },
        DivisionTerm { polynomial: psi3, carries_y: false },
        DivisionTerm { polynomial: psi4_core.scale(4), carries_y: true },
    ];
    for index in 5..=scalar + 1 {
        let next = if index % 2 == 1 {
            let m = (index - 1) / 2;
            terms[m + 2]
                .multiply(&terms[m].power(3, &cubic), &cubic)
                .subtract(&terms[m - 1].multiply(&terms[m + 1].power(3, &cubic), &cubic))
        } else {
            let m = index / 2;
            let inside = terms[m + 2]
                .multiply(&terms[m - 1].power(2, &cubic), &cubic)
                .subtract(&terms[m - 2].multiply(&terms[m + 1].power(2, &cubic), &cubic));
            divide_by_two_y(&terms[m].multiply(&inside, &cubic), &cubic)
        };
        terms.push(next);
    }
    let denominator = terms[scalar].multiply(&terms[scalar], &cubic);
    let adjacent = terms[scalar - 1].multiply(&terms[scalar + 1], &cubic);
    assert!(!denominator.carries_y && !adjacent.carries_y);
    (denominator.polynomial.shift().sub(&adjacent.polynomial), denominator.polynomial)
}

#[derive(Clone, Copy)]
struct Cell {
    slug: &'static str,
    p: i64,
    a: i64,
    b: i64,
    scalar: usize,
    singular: bool,
}

const CELLS: [Cell; 13] = [
    Cell { slug: "fp13-a1-b1-n2", p: 13, a: 1, b: 1, scalar: 2, singular: false },
    Cell { slug: "fp13-a1-b1-n3", p: 13, a: 1, b: 1, scalar: 3, singular: false },
    Cell { slug: "fp13-a1-b1-n5", p: 13, a: 1, b: 1, scalar: 5, singular: false },
    Cell { slug: "fp13-a1-b1-n7", p: 13, a: 1, b: 1, scalar: 7, singular: false },
    Cell { slug: "fp17-a2-b3-n2", p: 17, a: 2, b: 3, scalar: 2, singular: false },
    Cell { slug: "fp17-a2-b3-n3", p: 17, a: 2, b: 3, scalar: 3, singular: false },
    Cell { slug: "fp17-a2-b3-n5", p: 17, a: 2, b: 3, scalar: 5, singular: false },
    Cell { slug: "fp17-a2-b3-n7", p: 17, a: 2, b: 3, scalar: 7, singular: false },
    Cell { slug: "fp19-a1-b4-n2", p: 19, a: 1, b: 4, scalar: 2, singular: false },
    Cell { slug: "fp19-a1-b4-n3", p: 19, a: 1, b: 4, scalar: 3, singular: false },
    Cell { slug: "fp19-a1-b4-n5", p: 19, a: 1, b: 4, scalar: 5, singular: false },
    Cell { slug: "fp19-a1-b4-n7", p: 19, a: 1, b: 4, scalar: 7, singular: false },
    Cell { slug: "control-fp13-a0-b0-n2", p: 13, a: 0, b: 0, scalar: 2, singular: true },
];

fn expected(cell: Cell) -> HashMap<&'static str, String> {
    let discriminant = (4 * cell.a * cell.a * cell.a + 27 * cell.b * cell.b).rem_euclid(cell.p);
    let (numerator, denominator) = scalar_map(cell.scalar, cell.a, cell.b, cell.p);
    let gcd = numerator.monic_gcd(&denominator);
    let common = Polynomial::new(vec![1, 1], cell.p);
    let mutation_gcd = numerator.mul(&common).monic_gcd(&denominator.mul(&common));
    let degrees_pass = numerator.degree() == cell.scalar * cell.scalar
        && denominator.degree() == cell.scalar * cell.scalar - 1;
    let primary_pass = !cell.singular && discriminant != 0 && degrees_pass && gcd == Polynomial::one(cell.p);
    let mutation_pass = !cell.singular && mutation_gcd == common;
    let singular_control_pass = cell.singular && discriminant == 0 && gcd.degree() > 0;
    let accepted = if cell.singular { singular_control_pass } else { primary_pass && mutation_pass };
    let mut values = HashMap::new();
    macro_rules! put { ($key:literal, $value:expr) => { values.insert($key, ($value).to_string()); }; }
    put!("kind", if cell.singular { "singular" } else { "primary" });
    put!("p", cell.p); put!("a", cell.a); put!("b", cell.b); put!("scalar", cell.scalar);
    put!("discriminant", discriminant); put!("numerator_degree", numerator.degree());
    put!("denominator_degree", denominator.degree()); put!("numerator_lead", numerator.leading());
    put!("denominator_lead", denominator.leading()); put!("gcd_degree", gcd.degree());
    put!("gcd_coefficients", gcd.csv()); put!("mutation_gcd_degree", mutation_gcd.degree());
    put!("mutation_gcd_coefficients", mutation_gcd.csv()); put!("degrees_pass", degrees_pass);
    put!("primary_pass", primary_pass); put!("mutation_pass", mutation_pass);
    put!("singular_control_pass", singular_control_pass); put!("accepted", accepted);
    values
}

fn verify(path: &str) -> Result<(), String> {
    let contents = fs::read_to_string(path).map_err(|error| format!("read {path}: {error}"))?;
    let mut lines = contents.lines();
    if lines.next() != Some("schema\th116.scalar-divpoly-coprimality-result/v1") {
        return Err("schema mismatch".to_string());
    }
    let header: Vec<&str> = lines.next().ok_or_else(|| "missing header".to_string())?.split('\t').collect();
    let rows: Vec<&str> = lines.filter(|line| !line.is_empty()).collect();
    if rows.len() != CELLS.len() {
        return Err(format!("expected {} rows, got {}", CELLS.len(), rows.len()));
    }
    let compared = &header[1..20];
    let operation_columns = &header[20..];
    let mut accepted = 0_usize;
    for (cell, line) in CELLS.iter().copied().zip(rows.iter().copied()) {
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() != header.len() {
            return Err(format!("{}: field-count mismatch", cell.slug));
        }
        let row: HashMap<&str, &str> = header.iter().copied().zip(fields.iter().copied()).collect();
        if row["slug"] != cell.slug {
            return Err(format!("expected {}, got {}", cell.slug, row["slug"]));
        }
        let replay = expected(cell);
        for name in compared {
            if row[name] != replay[name] {
                return Err(format!("{}: {name} recorded {}, replay {}", cell.slug, row[name], replay[name]));
            }
        }
        for name in operation_columns {
            let _: u64 = row[name].parse().map_err(|_| format!("{}: invalid {name}", cell.slug))?;
        }
        accepted += usize::from(row["accepted"] == "true");
    }
    println!("verified_rows\t{}", rows.len());
    println!("accepted_rows\t{accepted}");
    println!("independent_recurrence\ttrue");
    Ok(())
}

fn self_test() {
    let prime = 29;
    let common = Polynomial::new(vec![7, 1], prime);
    let left = common.mul(&Polynomial::new(vec![3, 2, 1], prime));
    let right = common.mul(&Polynomial::new(vec![4, 1], prime));
    assert_eq!(left.monic_gcd(&right), common);
    let (numerator, denominator) = scalar_map(4, 2, 1, prime);
    assert_eq!((numerator.degree(), denominator.degree()), (16, 15));
    println!("h116 scalar divpoly coprimality verifier self-test ok");
}

fn run() -> Result<(), String> {
    let arguments: Vec<String> = std::env::args().collect();
    if arguments.get(1).is_some_and(|argument| argument == "--self-test") {
        self_test();
        Ok(())
    } else if let Some(path) = arguments.get(1) {
        verify(path)
    } else {
        Err("usage: verifier RESULT.tsv | --self-test".to_string())
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("h116 scalar divpoly coprimality verifier error: {error}");
        std::process::exit(1);
    }
}
