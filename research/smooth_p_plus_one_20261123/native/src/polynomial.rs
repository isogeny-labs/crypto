pub fn pow(mut value: i64, mut exponent: i64, p: i64) -> i64 {
    let mut result = 1;
    while exponent != 0 {
        if exponent & 1 != 0 {
            result = result * value % p;
        }
        value = value * value % p;
        exponent >>= 1;
    }
    result
}

pub fn inverse(value: i64, p: i64) -> i64 {
    let normalized = value.rem_euclid(p);
    assert_ne!(normalized, 0, "inverse of zero");
    pow(normalized, p - 2, p)
}

fn trim(poly: &mut Vec<i64>) {
    while poly.last() == Some(&0) {
        poly.pop();
    }
}

pub fn product(left: &[i64], right: &[i64], p: i64) -> Vec<i64> {
    if left.is_empty() || right.is_empty() {
        return vec![];
    }
    let mut result = vec![0; left.len() + right.len() - 1];
    for (i, left_coefficient) in left.iter().enumerate() {
        for (j, right_coefficient) in right.iter().enumerate() {
            result[i + j] = (result[i + j] + left_coefficient * right_coefficient).rem_euclid(p);
        }
    }
    trim(&mut result);
    result
}

fn divide(mut left: Vec<i64>, right: &[i64], p: i64) -> (Vec<i64>, Vec<i64>) {
    if left.len() < right.len() {
        return (vec![], left);
    }
    let scale = inverse(*right.last().unwrap(), p);
    let mut quotient = vec![0; left.len() + 1 - right.len()];
    for i in (0..quotient.len()).rev() {
        let q = left[i + right.len() - 1] * scale % p;
        quotient[i] = q;
        for (j, coefficient) in right.iter().enumerate() {
            left[i + j] = (left[i + j] - q * coefficient).rem_euclid(p);
        }
    }
    trim(&mut left);
    trim(&mut quotient);
    (quotient, left)
}

fn gcd(mut left: Vec<i64>, mut right: Vec<i64>, p: i64) -> Vec<i64> {
    while !right.is_empty() {
        let remainder = divide(left, &right, p).1;
        left = right;
        right = remainder;
    }
    if let Some(leading) = left.last() {
        let factor = inverse(*leading, p);
        for coefficient in &mut left {
            *coefficient = *coefficient * factor % p;
        }
    }
    left
}

fn mod_pow(mut value: Vec<i64>, mut exponent: i64, modulus: &[i64], p: i64) -> Vec<i64> {
    let mut result = vec![1];
    while exponent != 0 {
        if exponent & 1 != 0 {
            result = divide(product(&result, &value, p), modulus, p).1;
        }
        value = divide(product(&value, &value, p), modulus, p).1;
        exponent >>= 1;
    }
    result
}

pub fn evaluate(poly: &[i64], x: i64, p: i64) -> i64 {
    poly.iter()
        .rev()
        .fold(0, |sum, coefficient| (sum * x + coefficient).rem_euclid(p))
}

fn split(poly: Vec<i64>, p: i64, roots: &mut Vec<i64>) -> Result<(), String> {
    match poly.len() {
        0 | 1 => return Ok(()),
        2 => {
            roots.push((-poly[0] * inverse(poly[1], p)).rem_euclid(p));
            return Ok(());
        }
        _ => {}
    }
    for offset in 0..p.min(128) {
        let mut power = mod_pow(vec![offset, 1], (p - 1) / 2, &poly, p);
        if power.is_empty() {
            power.push(0);
        }
        power[0] = (power[0] - 1).rem_euclid(p);
        trim(&mut power);
        let factor = gcd(poly.clone(), power, p);
        if factor.len() > 1 && factor.len() < poly.len() {
            let (other, remainder) = divide(poly, &factor, p);
            assert!(remainder.is_empty());
            split(factor, p, roots)?;
            return split(other, p, roots);
        }
    }
    Err("bounded root splitting exhausted".into())
}

pub fn field_roots(poly: Vec<i64>, p: i64) -> Result<Vec<i64>, String> {
    if poly.len() < 2 {
        return if poly.is_empty() {
            Err("zero polynomial in root extraction".into())
        } else {
            Ok(vec![])
        };
    }
    let mut frobenius = mod_pow(vec![0, 1], p, &poly, p);
    frobenius.resize(frobenius.len().max(2), 0);
    frobenius[1] = (frobenius[1] - 1).rem_euclid(p);
    trim(&mut frobenius);
    let rational = gcd(poly, frobenius, p);
    let mut roots = vec![];
    split(rational, p, &mut roots)?;
    roots.sort_unstable();
    Ok(roots)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rational_roots_without_duplicates() {
        let a = product(&[126, 1], &[126, 1], 127);
        let b = product(&[125, 1], &[1, 0, 1], 127);
        assert_eq!(field_roots(product(&a, &b, 127), 127).unwrap(), vec![1, 2]);
    }
}
