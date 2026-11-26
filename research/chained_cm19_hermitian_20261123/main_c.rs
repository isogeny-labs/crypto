use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Z(i128, i128);

const ZERO: Z = Z(0, 0);
const ONE: Z = Z(1, 0);
const W: Z = Z(0, 1);
type Vector = [Z; 3];
type Matrix = [[Z; 3]; 3];
const H: Matrix = [
    [Z(2, 0), ONE, Z(-1, 0)],
    [ONE, Z(3, 0), Z(-2, 1)],
    [Z(-1, 0), Z(-1, -1), Z(3, 0)],
];

impl Z {
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }

    fn mul(self, rhs: Self) -> Self {
        Self(
            self.0 * rhs.0 - 5 * self.1 * rhs.1,
            self.0 * rhs.1 + self.1 * rhs.0 + self.1 * rhs.1,
        )
    }

    fn conj(self) -> Self {
        Self(self.0 + self.1, -self.1)
    }

    fn norm(self) -> i128 {
        self.0 * self.0 + self.0 * self.1 + 5 * self.1 * self.1
    }

    fn trace(self) -> i128 {
        2 * self.0 + self.1
    }

    fn mod2(self) -> u8 {
        (self.0.rem_euclid(2) + 2 * self.1.rem_euclid(2)) as u8
    }

    fn eval11(self, root: i128) -> i128 {
        (self.0 + root * self.1).rem_euclid(11)
    }
}

fn inner(a: Vector, b: Vector) -> Z {
    let mut out = ZERO;
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            out = out.add(x.conj().mul(H[i][j]).mul(y));
        }
    }
    out
}

fn mmul(a: Matrix, b: Matrix) -> Matrix {
    std::array::from_fn(|i| {
        std::array::from_fn(|j| (0..3).fold(ZERO, |acc, k| acc.add(a[i][k].mul(b[k][j]))))
    })
}

fn adjoint(a: Matrix) -> Matrix {
    std::array::from_fn(|i| std::array::from_fn(|j| a[j][i].conj()))
}

fn mscale(a: Matrix, factor: i128) -> Matrix {
    a.map(|row| row.map(|z| z.mul(Z(factor, 0))))
}

fn identity() -> Matrix {
    std::array::from_fn(|i| std::array::from_fn(|j| if i == j { ONE } else { ZERO }))
}

fn determinant(a: Matrix) -> Z {
    let first = a[0][0].mul(a[1][1].mul(a[2][2]).sub(a[1][2].mul(a[2][1])));
    let second = a[0][1].mul(a[1][0].mul(a[2][2]).sub(a[1][2].mul(a[2][0])));
    let third = a[0][2].mul(a[1][0].mul(a[2][1]).sub(a[1][1].mul(a[2][0])));
    first.sub(second).add(third)
}

fn inverse_unimodular(a: Matrix) -> Matrix {
    assert_eq!(determinant(a), ONE);
    std::array::from_fn(|i| {
        std::array::from_fn(|j| {
            let rows: Vec<_> = (0..3).filter(|&r| r != j).collect();
            let cols: Vec<_> = (0..3).filter(|&c| c != i).collect();
            let minor = a[rows[0]][cols[0]]
                .mul(a[rows[1]][cols[1]])
                .sub(a[rows[0]][cols[1]].mul(a[rows[1]][cols[0]]));
            minor.mul(Z(if (i + j) % 2 == 0 { 1 } else { -1 }, 0))
        })
    })
}

fn det_int(mut a: Vec<Vec<i128>>) -> i128 {
    let n = a.len();
    if n == 0 {
        return 1;
    }
    let mut sign = 1;
    let mut previous = 1;
    for k in 0..n - 1 {
        let Some(pivot) = (k..n).find(|&row| a[row][k] != 0) else {
            return 0;
        };
        if pivot != k {
            a.swap(pivot, k);
            sign = -sign;
        }
        let leading = a[k][k];
        for i in k + 1..n {
            for j in k + 1..n {
                let numerator = a[i][j] * leading - a[i][k] * a[k][j];
                assert_eq!(numerator % previous, 0);
                a[i][j] = numerator / previous;
            }
            a[i][k] = 0;
        }
        previous = leading;
    }
    sign * a[n - 1][n - 1]
}

fn trace_gram() -> Vec<Vec<i128>> {
    (0..6)
        .map(|i| {
            (0..6)
                .map(|j| {
                    let x = if i % 2 == 0 { ONE } else { W };
                    let y = if j % 2 == 0 { ONE } else { W };
                    x.conj().mul(H[i / 2][j / 2]).mul(y).trace()
                })
                .collect()
        })
        .collect()
}

fn hermitian(a: Matrix) -> bool {
    a == adjoint(a)
}

fn similitude(a: Matrix, multiplier: i128) -> bool {
    mmul(mmul(adjoint(a), H), a) == mscale(H, multiplier)
}

fn f4mul(a: u8, b: u8) -> u8 {
    let a0 = a & 1;
    let a1 = (a >> 1) & 1;
    let b0 = b & 1;
    let b1 = (b >> 1) & 1;
    ((a0 * b0) ^ (a1 * b1)) | (((a0 * b1) ^ (a1 * b0) ^ (a1 * b1)) << 1)
}

fn rank_f4(a: Matrix) -> usize {
    let mut matrix = a.map(|row| row.map(Z::mod2));
    let mut rank = 0;
    for column in 0..3 {
        if let Some(pivot) = (rank..3).find(|&row| matrix[row][column] != 0) {
            matrix.swap(rank, pivot);
            let inv = f4mul(matrix[rank][column], matrix[rank][column]);
            for value in &mut matrix[rank] {
                *value = f4mul(*value, inv);
            }
            let pivot_row = matrix[rank];
            for row in matrix.iter_mut().skip(rank + 1) {
                let factor = row[column];
                for (entry, &value) in row.iter_mut().zip(&pivot_row) {
                    *entry ^= f4mul(factor, value);
                }
            }
            rank += 1;
        }
    }
    rank
}

fn integer_matrix(a: Matrix) -> Vec<Vec<i128>> {
    let mut out = vec![vec![0; 6]; 6];
    for (i, row) in a.iter().enumerate() {
        for (j, &Z(x, y)) in row.iter().enumerate() {
            out[2 * i][2 * j] = x;
            out[2 * i][2 * j + 1] = -5 * y;
            out[2 * i + 1][2 * j] = y;
            out[2 * i + 1][2 * j + 1] = x + y;
        }
    }
    out
}

fn gcd(mut a: i128, mut b: i128) -> i128 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn subsets(k: u32) -> Vec<Vec<usize>> {
    (0u32..64)
        .filter(|mask| mask.count_ones() == k)
        .map(|mask| (0..6).filter(|&i| mask & (1 << i) != 0).collect())
        .collect()
}

fn smith(a: Matrix) -> Vec<i128> {
    let integer = integer_matrix(a);
    assert_eq!(det_int(integer.clone()), determinant(a).norm());
    let mut previous = 1;
    let mut factors = Vec::new();
    let mut minors = 0;
    for k in 1..=6 {
        let sets = subsets(k);
        let mut divisor = 0;
        for rows in &sets {
            for cols in &sets {
                let minor = rows
                    .iter()
                    .map(|&i| cols.iter().map(|&j| integer[i][j]).collect())
                    .collect();
                divisor = gcd(divisor, det_int(minor));
                minors += 1;
            }
        }
        assert_ne!(divisor, 0);
        assert_eq!(divisor % previous, 0);
        factors.push(divisor / previous);
        previous = divisor;
    }
    assert_eq!(minors, 923);
    assert!(factors.windows(2).all(|pair| pair[1] % pair[0] == 0));
    assert_eq!(factors.iter().product::<i128>(), determinant(a).norm());
    factors
}

fn elements(a_bound: i128, b_bound: i128, norm_bound: i128) -> Vec<Z> {
    let mut out = Vec::new();
    for a in -a_bound..=a_bound {
        for b in -b_bound..=b_bound {
            let z = Z(a, b);
            assert_eq!(z.mul(z.conj()), Z(z.norm(), 0));
            if z.norm() <= norm_bound {
                out.push(z);
            }
        }
    }
    out
}

fn vectors(gram: &[Vec<i128>]) -> Vec<Vec<Vector>> {
    let first = elements(5, 2, 24);
    let other = elements(9, 3, 60);
    let mut out = vec![Vec::new(); 13];
    let mut grid = 0;
    for &a in &first {
        for &b in &other {
            for &c in &other {
                let v = [a, b, c];
                let norm = inner(v, v);
                assert_eq!(norm.1, 0);
                assert!(norm.0 >= 0);
                let coords = [a.0, a.1, b.0, b.1, c.0, c.1];
                let mut independent = 0;
                for (i, &x) in coords.iter().enumerate() {
                    for (j, &y) in coords.iter().enumerate() {
                        independent += x * gram[i][j] * y;
                    }
                }
                assert_eq!(independent, 2 * norm.0);
                grid += 1;
                if (1..=12).contains(&norm.0) {
                    out[norm.0 as usize].push(v);
                    println!("SHORT_VECTOR\tnorm={}\t{v:?}", norm.0);
                }
            }
        }
    }
    assert!(out[1].is_empty());
    println!(
        "VECTOR_GRID\tfirst={}\tother={}\tgrid={grid}\tcomplete bounds [24,60,60]\tnorm1=0",
        first.len(),
        other.len()
    );
    for (norm, rows) in out.iter().enumerate().skip(1) {
        println!("VECTOR_COUNT\tnorm={norm}\tcount={}", rows.len());
    }
    out
}

fn enumerate(vectors: &[Vec<Vector>], multiplier: usize) -> Vec<Matrix> {
    let mut out = Vec::new();
    let m = multiplier as i128;
    for &a in &vectors[2 * multiplier] {
        let seconds: Vec<_> = vectors[3 * multiplier]
            .iter()
            .copied()
            .filter(|&b| inner(a, b) == Z(m, 0))
            .collect();
        let thirds: Vec<_> = vectors[3 * multiplier]
            .iter()
            .copied()
            .filter(|&c| inner(a, c) == Z(-m, 0))
            .collect();
        for &b in &seconds {
            for &c in &thirds {
                if inner(b, c) == Z(-2 * m, m) {
                    let s = std::array::from_fn(|i| [a[i], b[i], c[i]]);
                    assert!(similitude(s, m));
                    assert_eq!(determinant(s).norm(), m.pow(3));
                    out.push(s);
                }
            }
        }
    }
    out.sort();
    let count = out.len();
    out.dedup();
    assert_eq!(count, out.len());
    out
}

fn main() {
    println!("PROFILE\tcomplete-integral-CM19-four-similitudes-A\tunsigned\tno actual divisor/cost claim");
    assert_eq!(W.mul(W), W.sub(Z(5, 0)));
    assert_eq!(W.mul(W.conj()), Z(5, 0));
    for a in 0..4 {
        for b in 0..4 {
            let aa = Z((a & 1) as i128, (a >> 1) as i128);
            let bb = Z((b & 1) as i128, (b >> 1) as i128);
            assert_eq!(f4mul(a, b), aa.mul(bb).mod2());
            if a != 0 {
                assert_eq!(f4mul(a, f4mul(a, a)), 1);
            }
        }
    }
    assert!(hermitian(H));
    assert_eq!(determinant(H), ONE);
    assert_eq!(H[0][0], Z(2, 0));
    assert_eq!(H[0][0].mul(H[1][1]).sub(H[0][1].mul(H[1][0])), Z(5, 0));
    let inverse = inverse_unimodular(H);
    assert_eq!(mmul(H, inverse), identity());
    assert_eq!(mmul(inverse, H), identity());
    assert_eq!(
        [inverse[0][0], inverse[1][1], inverse[2][2]],
        [Z(2, 0), Z(5, 0), Z(5, 0)]
    );
    let gram = trace_gram();
    for k in 1..=6 {
        let leading = gram[..k].iter().map(|row| row[..k].to_vec()).collect();
        assert!(det_int(leading) > 0);
    }
    assert_eq!(det_int(gram.clone()), 6859);
    println!("LATTICE\tH={H:?}\tinverse={inverse:?}\ttrace_Gram={gram:?}\tdet_trace=6859");
    let mut changed_h = H;
    changed_h[1][2] = changed_h[1][2].add(ONE);
    assert!(!hermitian(changed_h));
    assert!(hermitian(H));
    let twice = mscale(identity(), 2);
    assert!(similitude(identity(), 1));
    assert!(similitude(twice, 4));
    assert_eq!(smith(twice), vec![2; 6]);
    let mut changed = twice;
    changed[0][0] = changed[0][0].add(ONE);
    assert!(!similitude(changed, 4));
    assert!(similitude(twice, 4));
    println!("CONTROLS\tI unitary\t2I positive\tactual 3,2,2 mutant rejects\tnonHermitian mutant rejects\tPASS");
    let short = vectors(&gram);
    let automorphisms = enumerate(&short, 1);
    assert!(automorphisms.contains(&identity()));
    for (index, a) in automorphisms.iter().enumerate() {
        println!("AUTOMORPHISM\tid={index}\t{a:?}");
    }
    let maps = enumerate(&short, 4);
    assert!(maps.contains(&twice));
    for &a in &automorphisms {
        assert!(maps.contains(&mscale(a, 2)));
    }
    let mut classes = BTreeMap::new();
    let mut first_mixed = None;
    let mut divisible = 0;
    let mut mixed = 0;
    for (index, &s) in maps.iter().enumerate() {
        let factors = smith(s);
        assert_eq!(factors.iter().product::<i128>(), 64);
        let rank = rank_f4(s);
        assert_eq!(factors.iter().filter(|&&x| x % 2 != 0).count(), 2 * rank);
        let determinant = determinant(s);
        let eval3 = determinant.eval11(3);
        let eval9 = determinant.eval11(9);
        assert_ne!(eval3, 0);
        assert_ne!(eval9, 0);
        *classes.entry(factors.clone()).or_insert(0) += 1;
        let is_divisible = s.iter().flatten().all(|z| z.0 % 2 == 0 && z.1 % 2 == 0);
        if is_divisible {
            let half = s.map(|row| row.map(|z| Z(z.0 / 2, z.1 / 2)));
            assert!(automorphisms.contains(&half));
            assert_eq!(factors, [2, 2, 2, 2, 2, 2]);
            divisible += 1;
        }
        if factors == [1, 1, 2, 2, 4, 4] {
            mixed += 1;
            first_mixed.get_or_insert(s);
            assert_eq!(rank, 1);
        }
        println!("FOUR_MAP\tid={index}\tS={s:?}\tdet={determinant:?}\tNorm=64\tSmith={factors:?}\tF4rank={rank}\tdet_at3={eval3}\tdet_at9={eval9}\t2divisible={is_divisible}");
    }
    assert_eq!(divisible, automorphisms.len());
    println!("KERNEL_CLASSES\t{classes:?}");
    println!("FIRST_LEX_MIXED\t{first_mixed:?}");
    println!("SUMMARY\tautomorphisms={}\tfour_maps={}\t2divisible={divisible}\tmixed4422={mixed}\tcomplete_fixed_H_multiplier4=PASS\tactual_alpha_divisor_maps=OPEN\tDLP_cost=UNMEASURED", automorphisms.len(), maps.len());
}
