use hec_native_ic::jac::Curve;
#[path = "../setup/fast.rs"]
mod fast;
pub fn jacobian_order(c: &Curve) -> u64 {
    let (_, _, a) = fast::prepare(c);
    let d = 2 * c.g;
    assert_eq!(a.len(), d + 1);
    let mut matrix = vec![vec![0i128; d]; d];
    for col in 0..d {
        let mut v = vec![0i128; 2 * d];
        for (j, &x) in a.iter().enumerate() {
            v[j + col] = x as i128
        }
        v[col] -= 1;
        for k in (d..2 * d).rev() {
            let lead = v[k];
            v[k] = 0;
            for j in 0..d {
                v[k - d + j] -= lead
            }
        }
        for row in 0..d {
            matrix[row][col] = v[row]
        }
    }
    let (mut sign, mut denom) = (1i128, 1i128);
    for k in 0..d - 1 {
        let pivot = (k..d).find(|&i| matrix[i][k] != 0).unwrap();
        if pivot != k {
            matrix.swap(pivot, k);
            sign = -sign
        }
        let p = matrix[k][k];
        for i in k + 1..d {
            for j in k + 1..d {
                let num = matrix[i][j]
                    .checked_mul(p)
                    .unwrap()
                    .checked_sub(matrix[i][k].checked_mul(matrix[k][j]).unwrap())
                    .unwrap();
                assert_eq!(num % denom, 0);
                matrix[i][j] = num / denom
            }
            matrix[i][k] = 0
        }
        denom = p
    }
    let answer = sign * matrix[d - 1][d - 1];
    assert!(answer > 0);
    answer.try_into().unwrap()
}
