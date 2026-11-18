use std::cell::Cell;
use std::collections::HashSet;

const MAX_D: usize = 6;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Felt {
    pub c: [u64; MAX_D],
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Point {
    Infinity,
    Affine(Felt, Felt),
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CountSnapshot {
    pub base_additions: u64,
    pub base_multiplications: u64,
    pub base_inversions: u64,
    pub extension_additions: u64,
    pub extension_multiplications: u64,
    pub extension_inversions: u64,
    pub frobenius_maps: u64,
    pub point_additions: u64,
    pub scalar_multiplications: u64,
}

#[derive(Debug, Default)]
struct Counters {
    base_additions: Cell<u64>,
    base_multiplications: Cell<u64>,
    base_inversions: Cell<u64>,
    extension_additions: Cell<u64>,
    extension_multiplications: Cell<u64>,
    extension_inversions: Cell<u64>,
    frobenius_maps: Cell<u64>,
    point_additions: Cell<u64>,
    scalar_multiplications: Cell<u64>,
}

impl Counters {
    fn snapshot(&self) -> CountSnapshot {
        CountSnapshot {
            base_additions: self.base_additions.get(),
            base_multiplications: self.base_multiplications.get(),
            base_inversions: self.base_inversions.get(),
            extension_additions: self.extension_additions.get(),
            extension_multiplications: self.extension_multiplications.get(),
            extension_inversions: self.extension_inversions.get(),
            frobenius_maps: self.frobenius_maps.get(),
            point_additions: self.point_additions.get(),
            scalar_multiplications: self.scalar_multiplications.get(),
        }
    }

    fn reset(&self) {
        self.base_additions.set(0);
        self.base_multiplications.set(0);
        self.base_inversions.set(0);
        self.extension_additions.set(0);
        self.extension_multiplications.set(0);
        self.extension_inversions.set(0);
        self.frobenius_maps.set(0);
        self.point_additions.set(0);
        self.scalar_multiplications.set(0);
    }
}

#[derive(Debug)]
pub struct Field {
    pub p: u64,
    pub d: usize,
    pub q: u64,
    pub modulus: [u64; MAX_D + 1],
    counters: Counters,
}

fn bump(cell: &Cell<u64>, by: u64) {
    cell.set(
        cell.get()
            .checked_add(by)
            .expect("operation counter overflow"),
    );
}

fn add_mod(a: u64, b: u64, p: u64) -> u64 {
    (a + b) % p
}

fn sub_mod(a: u64, b: u64, p: u64) -> u64 {
    (a + p - b) % p
}

fn mul_mod(a: u64, b: u64, p: u64) -> u64 {
    (a * b) % p
}

fn pow_u64(mut base: u64, mut exponent: usize) -> u64 {
    let mut result = 1_u64;
    while exponent != 0 {
        if exponent & 1 == 1 {
            result = result.checked_mul(base).expect("field size overflow");
        }
        exponent >>= 1;
        if exponent != 0 {
            base = base.checked_mul(base).expect("field size overflow");
        }
    }
    result
}

impl Field {
    pub fn new(p: u64, d: usize, modulus_coefficients: &[u64]) -> Result<Self, String> {
        if !(1..=MAX_D).contains(&d) {
            return Err(format!("unsupported extension degree {d}"));
        }
        if modulus_coefficients.len() != d + 1 || modulus_coefficients[d] != 1 {
            return Err("extension modulus must be monic of exact degree".to_string());
        }
        let mut modulus = [0_u64; MAX_D + 1];
        for (slot, &coefficient) in modulus.iter_mut().zip(modulus_coefficients) {
            *slot = coefficient % p;
        }
        Ok(Self {
            p,
            d,
            q: pow_u64(p, d),
            modulus,
            counters: Counters::default(),
        })
    }

    pub fn snapshot(&self) -> CountSnapshot {
        self.counters.snapshot()
    }

    pub fn reset_counters(&self) {
        self.counters.reset();
    }

    pub fn zero(&self) -> Felt {
        Felt::default()
    }

    pub fn one(&self) -> Felt {
        self.from_base(1)
    }

    pub fn from_base(&self, value: u64) -> Felt {
        let mut result = Felt::default();
        result.c[0] = value % self.p;
        result
    }

    pub fn from_index(&self, mut index: u64) -> Felt {
        assert!(index < self.q, "field index out of range");
        let mut result = Felt::default();
        for coefficient in result.c.iter_mut().take(self.d) {
            *coefficient = index % self.p;
            index /= self.p;
        }
        result
    }

    pub fn to_index(&self, value: Felt) -> u64 {
        let mut result = 0_u64;
        let mut place = 1_u64;
        for &coefficient in value.c.iter().take(self.d) {
            result += coefficient * place;
            place *= self.p;
        }
        result
    }

    pub fn is_base(&self, value: Felt) -> bool {
        value.c[1..self.d]
            .iter()
            .all(|&coefficient| coefficient == 0)
    }

    pub fn to_base(&self, value: Felt) -> Result<u64, String> {
        if !self.is_base(value) {
            return Err(format!("non-base-field element {}", self.to_index(value)));
        }
        Ok(value.c[0])
    }

    pub fn add(&self, left: Felt, right: Felt) -> Felt {
        bump(&self.counters.extension_additions, 1);
        bump(
            &self.counters.base_additions,
            u64::try_from(self.d).expect("degree fits u64"),
        );
        let mut result = Felt::default();
        for i in 0..self.d {
            result.c[i] = add_mod(left.c[i], right.c[i], self.p);
        }
        result
    }

    pub fn neg(&self, value: Felt) -> Felt {
        let mut result = Felt::default();
        for i in 0..self.d {
            result.c[i] = if value.c[i] == 0 {
                0
            } else {
                self.p - value.c[i]
            };
        }
        result
    }

    pub fn sub(&self, left: Felt, right: Felt) -> Felt {
        self.add(left, self.neg(right))
    }

    pub fn mul(&self, left: Felt, right: Felt) -> Felt {
        bump(&self.counters.extension_multiplications, 1);
        let mut product = [0_u64; 2 * MAX_D - 1];
        for i in 0..self.d {
            for j in 0..self.d {
                bump(&self.counters.base_multiplications, 1);
                bump(&self.counters.base_additions, 1);
                product[i + j] = add_mod(
                    product[i + j],
                    mul_mod(left.c[i], right.c[j], self.p),
                    self.p,
                );
            }
        }
        for degree in (self.d..=(2 * self.d - 2)).rev() {
            let leading = product[degree];
            if leading == 0 {
                continue;
            }
            for i in 0..self.d {
                bump(&self.counters.base_multiplications, 1);
                bump(&self.counters.base_additions, 1);
                let reduction = mul_mod(leading, self.modulus[i], self.p);
                product[degree - self.d + i] =
                    sub_mod(product[degree - self.d + i], reduction, self.p);
            }
            product[degree] = 0;
        }
        let mut result = Felt::default();
        result.c[..self.d].copy_from_slice(&product[..self.d]);
        result
    }

    pub fn square(&self, value: Felt) -> Felt {
        self.mul(value, value)
    }

    pub fn pow(&self, mut base: Felt, mut exponent: u64) -> Felt {
        let mut result = self.one();
        while exponent != 0 {
            if exponent & 1 == 1 {
                result = self.mul(result, base);
            }
            exponent >>= 1;
            if exponent != 0 {
                base = self.square(base);
            }
        }
        result
    }

    pub fn inv(&self, value: Felt) -> Result<Felt, String> {
        if value == self.zero() {
            return Err("attempted inversion of zero".to_string());
        }
        bump(&self.counters.extension_inversions, 1);
        Ok(self.pow(value, self.q - 2))
    }

    pub fn div(&self, numerator: Felt, denominator: Felt) -> Result<Felt, String> {
        Ok(self.mul(numerator, self.inv(denominator)?))
    }

    pub fn frobenius(&self, value: Felt) -> Felt {
        bump(&self.counters.frobenius_maps, 1);
        self.pow(value, self.p)
    }

    pub fn frobenius_power(&self, mut value: Felt, exponent: usize) -> Felt {
        for _ in 0..exponent {
            value = self.frobenius(value);
        }
        value
    }

    pub fn sqrt(&self, value: Felt) -> Result<Option<Felt>, String> {
        if value == self.zero() {
            return Ok(Some(value));
        }
        if self.pow(value, (self.q - 1) / 2) != self.one() {
            return Ok(None);
        }
        let mut odd = self.q - 1;
        let mut twos = 0_u32;
        while odd % 2 == 0 {
            odd /= 2;
            twos += 1;
        }
        if twos == 1 {
            return Ok(Some(self.pow(value, (self.q + 1) / 4)));
        }
        let mut nonresidue = None;
        for index in 2..self.q {
            let candidate = self.from_index(index);
            if self.pow(candidate, (self.q - 1) / 2) == self.neg(self.one()) {
                nonresidue = Some(candidate);
                break;
            }
        }
        let z = nonresidue.ok_or_else(|| "quadratic nonresidue not found".to_string())?;
        let mut c = self.pow(z, odd);
        let mut x = self.pow(value, (odd + 1) / 2);
        let mut t = self.pow(value, odd);
        let mut m = twos;
        while t != self.one() {
            let mut i = 1_u32;
            let mut probe = self.square(t);
            while i < m && probe != self.one() {
                probe = self.square(probe);
                i += 1;
            }
            if i == m {
                return Err("Tonelli-Shanks invariant failure".to_string());
            }
            let exponent = 1_u64 << (m - i - 1);
            let b = self.pow(c, exponent);
            x = self.mul(x, b);
            let b2 = self.square(b);
            t = self.mul(t, b2);
            c = b2;
            m = i;
        }
        if self.square(x) != value {
            return Err("square-root verification failed".to_string());
        }
        Ok(Some(x))
    }
}

#[derive(Debug)]
pub struct Curve<'a> {
    pub field: &'a Field,
    pub a: Felt,
    pub b: Felt,
}

impl Curve<'_> {
    pub fn is_on_curve(&self, point: Point) -> bool {
        match point {
            Point::Infinity => true,
            Point::Affine(x, y) => {
                let left = self.field.square(y);
                let x2 = self.field.square(x);
                let x3 = self.field.mul(x2, x);
                let right = self
                    .field
                    .add(self.field.add(x3, self.field.mul(self.a, x)), self.b);
                left == right
            }
        }
    }

    pub fn neg(&self, point: Point) -> Point {
        match point {
            Point::Infinity => Point::Infinity,
            Point::Affine(x, y) => Point::Affine(x, self.field.neg(y)),
        }
    }

    pub fn add(&self, left: Point, right: Point) -> Result<Point, String> {
        bump(&self.field.counters.point_additions, 1);
        match (left, right) {
            (Point::Infinity, point) | (point, Point::Infinity) => Ok(point),
            (Point::Affine(x1, y1), Point::Affine(x2, y2)) => {
                if x1 == x2 && self.field.add(y1, y2) == self.field.zero() {
                    return Ok(Point::Infinity);
                }
                let slope = if x1 == x2 && y1 == y2 {
                    let three = self.field.from_base(3);
                    let two = self.field.from_base(2);
                    let numerator = self
                        .field
                        .add(self.field.mul(three, self.field.square(x1)), self.a);
                    self.field.div(numerator, self.field.mul(two, y1))?
                } else {
                    self.field
                        .div(self.field.sub(y2, y1), self.field.sub(x2, x1))?
                };
                let x3 = self
                    .field
                    .sub(self.field.sub(self.field.square(slope), x1), x2);
                let y3 = self
                    .field
                    .sub(self.field.mul(slope, self.field.sub(x1, x3)), y1);
                let result = Point::Affine(x3, y3);
                if !self.is_on_curve(result) {
                    return Err("point addition left the curve".to_string());
                }
                Ok(result)
            }
        }
    }

    pub fn sub(&self, left: Point, right: Point) -> Result<Point, String> {
        self.add(left, self.neg(right))
    }

    pub fn scalar_mul(&self, mut scalar: u64, mut point: Point) -> Result<Point, String> {
        bump(&self.field.counters.scalar_multiplications, 1);
        let mut result = Point::Infinity;
        while scalar != 0 {
            if scalar & 1 == 1 {
                result = self.add(result, point)?;
            }
            scalar >>= 1;
            if scalar != 0 {
                point = self.add(point, point)?;
            }
        }
        Ok(result)
    }

    pub fn frobenius(&self, point: Point) -> Point {
        match point {
            Point::Infinity => Point::Infinity,
            Point::Affine(x, y) => Point::Affine(self.field.frobenius(x), self.field.frobenius(y)),
        }
    }

    pub fn frobenius_power(&self, mut point: Point, exponent: usize) -> Point {
        for _ in 0..exponent {
            point = self.frobenius(point);
        }
        point
    }
}

fn poly_trim(poly: &mut Vec<u64>) {
    while poly.len() > 1 && poly.last() == Some(&0) {
        poly.pop();
    }
}

fn poly_sub(left: &[u64], right: &[u64], p: u64) -> Vec<u64> {
    let mut result = vec![0_u64; left.len().max(right.len())];
    for (i, slot) in result.iter_mut().enumerate() {
        *slot = sub_mod(
            left.get(i).copied().unwrap_or(0),
            right.get(i).copied().unwrap_or(0),
            p,
        );
    }
    poly_trim(&mut result);
    result
}

fn base_inv(value: u64, p: u64) -> u64 {
    let mut result = 1_u64;
    let mut base = value;
    let mut exponent = p - 2;
    while exponent != 0 {
        if exponent & 1 == 1 {
            result = mul_mod(result, base, p);
        }
        exponent >>= 1;
        if exponent != 0 {
            base = mul_mod(base, base, p);
        }
    }
    result
}

fn poly_mod(mut numerator: Vec<u64>, denominator: &[u64], p: u64) -> Vec<u64> {
    poly_trim(&mut numerator);
    let denominator_degree = denominator.len() - 1;
    let inverse_lead = base_inv(denominator[denominator_degree], p);
    while numerator.len() >= denominator.len() && numerator != [0] {
        let shift = numerator.len() - denominator.len();
        let coefficient = mul_mod(
            *numerator.last().expect("nonempty polynomial"),
            inverse_lead,
            p,
        );
        for (i, &term) in denominator.iter().enumerate() {
            let index = shift + i;
            numerator[index] = sub_mod(numerator[index], mul_mod(coefficient, term, p), p);
        }
        poly_trim(&mut numerator);
    }
    numerator
}

fn poly_mul_mod(left: &[u64], right: &[u64], modulus: &[u64], p: u64) -> Vec<u64> {
    let mut product = vec![0_u64; left.len() + right.len() - 1];
    for (i, &a) in left.iter().enumerate() {
        for (j, &b) in right.iter().enumerate() {
            product[i + j] = add_mod(product[i + j], mul_mod(a, b, p), p);
        }
    }
    poly_mod(product, modulus, p)
}

fn poly_pow_mod(mut base: Vec<u64>, mut exponent: u64, modulus: &[u64], p: u64) -> Vec<u64> {
    let mut result = vec![1_u64];
    while exponent != 0 {
        if exponent & 1 == 1 {
            result = poly_mul_mod(&result, &base, modulus, p);
        }
        exponent >>= 1;
        if exponent != 0 {
            base = poly_mul_mod(&base, &base, modulus, p);
        }
    }
    result
}

fn poly_gcd(mut left: Vec<u64>, mut right: Vec<u64>, p: u64) -> Vec<u64> {
    poly_trim(&mut left);
    poly_trim(&mut right);
    while right != [0] {
        let remainder = poly_mod(left, &right, p);
        left = right;
        right = remainder;
    }
    let inverse = base_inv(*left.last().expect("nonempty gcd"), p);
    for coefficient in &mut left {
        *coefficient = mul_mod(*coefficient, inverse, p);
    }
    left
}

fn prime_divisors(mut value: usize) -> Vec<usize> {
    let mut divisors = Vec::new();
    let mut candidate = 2_usize;
    while candidate * candidate <= value {
        if value % candidate == 0 {
            divisors.push(candidate);
            while value % candidate == 0 {
                value /= candidate;
            }
        }
        candidate += 1;
    }
    if value > 1 {
        divisors.push(value);
    }
    divisors
}

pub fn is_irreducible(modulus: &[u64], p: u64) -> bool {
    let degree = modulus.len() - 1;
    if degree == 0 || modulus[degree] != 1 {
        return false;
    }
    let x = vec![0_u64, 1];
    let mut powers = Vec::with_capacity(degree + 1);
    powers.push(x.clone());
    for _ in 1..=degree {
        let next = poly_pow_mod(powers.last().expect("power exists").clone(), p, modulus, p);
        powers.push(next);
    }
    if poly_sub(&powers[degree], &x, p) != [0] {
        return false;
    }
    for divisor in prime_divisors(degree) {
        let difference = poly_sub(&powers[degree / divisor], &x, p);
        if poly_gcd(modulus.to_vec(), difference, p) != [1] {
            return false;
        }
    }
    true
}

pub fn first_irreducible(p: u64, degree: usize) -> Result<Vec<u64>, String> {
    let candidates = pow_u64(p, degree);
    for code in 0..candidates {
        let mut remainder = code;
        let mut coefficients = vec![0_u64; degree + 1];
        for i in 0..degree {
            let place = pow_u64(p, degree - 1 - i);
            coefficients[i] = remainder / place;
            remainder %= place;
        }
        coefficients[degree] = 1;
        if is_irreducible(&coefficients, p) {
            return Ok(coefficients);
        }
    }
    Err(format!(
        "no irreducible polynomial for p={p}, degree={degree}"
    ))
}

pub fn trace_group_order(p: u64, trace: i64, degree: usize) -> Result<u64, String> {
    let mut previous_previous = 2_i128;
    let mut previous = i128::from(trace);
    let power_sum = match degree {
        0 => previous_previous,
        1 => previous,
        _ => {
            for _ in 2..=degree {
                let current = i128::from(trace) * previous - i128::from(p) * previous_previous;
                previous_previous = previous;
                previous = current;
            }
            previous
        }
    };
    let q = i128::from(pow_u64(p, degree));
    u64::try_from(q + 1 - power_sum).map_err(|_| "invalid extension group order".to_string())
}

pub fn mod_inverse(value: u64, modulus: u64) -> Result<u64, String> {
    let (mut old_r, mut r) = (i128::from(value), i128::from(modulus));
    let (mut old_s, mut s) = (1_i128, 0_i128);
    while r != 0 {
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_s, s) = (s, old_s - quotient * s);
    }
    if old_r != 1 {
        return Err(format!("{value} is not invertible modulo {modulus}"));
    }
    let reduced = old_s.rem_euclid(i128::from(modulus));
    u64::try_from(reduced).map_err(|_| "modular inverse conversion failed".to_string())
}

pub fn torsion_table(
    curve: &Curve<'_>,
    rational: Point,
    transverse: Point,
    order: u64,
) -> Result<HashSet<Point>, String> {
    let mut table = HashSet::new();
    for i in 0..order {
        let left = curve.scalar_mul(i, rational)?;
        for j in 0..order {
            let right = curve.scalar_mul(j, transverse)?;
            let point = curve.add(left, right)?;
            if curve.scalar_mul(order, point)? != Point::Infinity {
                return Err("torsion-table element not killed by r".to_string());
            }
            if !table.insert(point) {
                return Err("dependent torsion basis".to_string());
            }
        }
    }
    Ok(table)
}

pub fn selftest() -> Result<(), String> {
    let expected = [(7_u64, 4_usize), (5, 6), (13, 4), (31, 4), (47, 3)];
    for (p, degree) in expected {
        let modulus = first_irreducible(p, degree)?;
        if !is_irreducible(&modulus, p) {
            return Err("irreducibility selftest failed".to_string());
        }
        let field = Field::new(p, degree, &modulus)?;
        for index in 0..field.q.min(128) {
            let value = field.from_index(index);
            if field.to_index(value) != index {
                return Err("field serialization round-trip failed".to_string());
            }
            if value != field.zero() {
                let inverse = field.inv(value)?;
                if field.mul(value, inverse) != field.one() {
                    return Err("field inversion selftest failed".to_string());
                }
            }
            if field.frobenius_power(value, degree) != value {
                return Err("Frobenius order selftest failed".to_string());
            }
        }
    }
    Ok(())
}
