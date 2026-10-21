// Exact linear-algebra control for signed large-prime cycles.
const P: i64 = 101;
fn inv(a: i64) -> i64 {
    (1..P).find(|&b| (a * b).rem_euclid(P) == 1).unwrap()
}
fn rank(mut a: Vec<Vec<i64>>) -> usize {
    let mut r = 0;
    for j in 0..a[0].len() {
        if let Some(i) = (r..a.len()).find(|&i| a[i][j].rem_euclid(P) != 0) {
            a.swap(r, i);
            let v = inv(a[r][j]);
            for x in &mut a[r] {
                *x = (*x * v).rem_euclid(P);
            }
            for i in 0..a.len() {
                if i != r {
                    let v = a[i][j];
                    for k in j..a[0].len() {
                        a[i][k] = (a[i][k] - v * a[r][k]).rem_euclid(P);
                    }
                }
            }
            r += 1;
        }
        if r == a.len() {
            break;
        }
    }
    r
}
fn main() {
    let triangle = vec![vec![1, 1, 0], vec![0, 1, 1], vec![1, 0, 1]];
    assert_eq!(rank(triangle), 3);
    // All 101 target values are compatible with this triangle's LP equations.
    for k in 0..P {
        let rhs = [(7 + k) % P, (11 + 2 * k) % P, (10 + 3 * k) % P];
        let x = ((rhs[0] - rhs[1] + rhs[2]) * inv(2)).rem_euclid(P);
        let y = (rhs[0] - x).rem_euclid(P);
        let z = (rhs[1] - y).rem_euclid(P);
        assert_eq!((x + z) % P, rhs[2]);
    }
    let square = vec![
        vec![1, 1, 0, 0],
        vec![0, 1, 1, 0],
        vec![0, 0, 1, 1],
        vec![1, 0, 0, 1],
    ];
    assert_eq!(rank(square), 3);
    let x: [i64; 4] = [3, 4, 7, 8];
    let b: [i64; 4] = [1, 2, 3, 5];
    let k: i64 = 37;
    let a: Vec<_> = (0..4)
        .map(|i| (x[i] + x[(i + 1) % 4] - b[i] * k).rem_euclid(P))
        .collect();
    let ca = (a[0] - a[1] + a[2] - a[3]).rem_euclid(P);
    let cb = (b[0] - b[1] + b[2] - b[3]).rem_euclid(P);
    assert_ne!(cb, 0);
    assert_eq!((-ca * inv(cb)).rem_euclid(P), k);
    println!("modulus,cycle,vertices,edges,rank,free_target_values,extracts_target");
    println!("101,triangle_all_plus,3,3,3,101,false");
    println!("101,square_all_plus_nonzero_target_coefficient,4,4,3,1,true");
}
