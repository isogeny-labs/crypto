//! Independent Sylvester-determinant replay. No production field/polynomial calls.
use serde::Deserialize;
use serde_json::Value;
fn ensure(ok: bool, why: &str) -> Result<(), String> {
    if ok {
        Ok(())
    } else {
        Err(why.into())
    }
}
fn mul(a: u64, b: u64, p: u64) -> u64 {
    (a as u128 * b as u128 % p as u128) as u64
}
fn pow(mut a: u64, mut n: u64, p: u64) -> u64 {
    let mut out = 1;
    while n > 0 {
        if n % 2 == 1 {
            out = mul(out, a, p);
        }
        a = mul(a, a, p);
        n >>= 1;
    }
    out
}
fn prime(p: u64) -> bool {
    p >= 2
        && (2..)
            .take_while(|d| d * d <= p)
            .all(|d| !p.is_multiple_of(d))
}
fn determinant(mut matrix: [[u64; 7]; 7], p: u64) -> u64 {
    let mut det = 1;
    for column in 0..7 {
        let Some(pivot) = (column..7).find(|&r| matrix[r][column] != 0) else {
            return 0;
        };
        if pivot != column {
            matrix.swap(pivot, column);
            det = if det == 0 { 0 } else { p - det };
        }
        let value = matrix[column][column];
        det = mul(det, value, p);
        let inv = pow(value, p - 2, p);
        for row in column + 1..7 {
            let scale = mul(matrix[row][column], inv, p);
            for j in column..7 {
                matrix[row][j] = (matrix[row][j] + p - mul(scale, matrix[column][j], p)) % p;
            }
        }
    }
    det
}
pub fn compute(p: u64, tail: &[u64]) -> Result<(u64, u64, i8), String> {
    ensure(
        (5..=1511).contains(&p)
            && prime(p)
            && tail.len() == 5
            && tail[4] != 0
            && tail.iter().all(|&x| x < p),
        "independent canonical quartic/prime",
    )?;
    let f = [tail[4], tail[3], tail[2], tail[1], tail[0]];
    let derivative = [
        mul(4, tail[4], p),
        mul(3, tail[3], p),
        mul(2, tail[2], p),
        tail[1],
    ];
    let mut matrix = [[0; 7]; 7];
    for row in 0..3 {
        matrix[row][row..row + 5].copy_from_slice(&f);
    }
    for row in 0..4 {
        matrix[3 + row][row..row + 4].copy_from_slice(&derivative);
    }
    let resultant = determinant(matrix, p);
    let class = mul(tail[4], resultant, p);
    let character = if class == 0 {
        0
    } else if pow(class, (p - 1) / 2, p) == 1 {
        1
    } else {
        -1
    };
    Ok((resultant, class, character))
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Screen {
    resultant: u64,
    discriminant_class: u64,
    character: i8,
}
pub fn check(p: u64, tail: &[u64], raw: &Value) -> Result<i8, String> {
    let screen: Screen = serde_json::from_value(raw.clone()).map_err(|e| e.to_string())?;
    ensure(
        compute(p, tail)?
            == (
                screen.resultant,
                screen.discriminant_class,
                screen.character,
            ),
        "Sylvester resultant/discriminant character mismatch",
    )?;
    Ok(screen.character)
}
