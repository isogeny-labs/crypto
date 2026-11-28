#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Z(pub i128, pub i128);
pub const ZERO: Z = Z(0, 0);
pub const ONE: Z = Z(1, 0);
pub const W: Z = Z(0, 1);
pub type Matrix = [[Z; 3]; 3];
pub const H: Matrix = [[Z(2, 0), ONE, Z(-1, 0)], [ONE, Z(3, 0), Z(-2, 1)], [Z(-1, 0), Z(-1, -1), Z(3, 0)]];
pub const G: Matrix = [[Z(2, 0), Z(-1, 0), ZERO], [Z(-1, 0), Z(2, 0), ZERO], [ZERO, ZERO, Z(3, 0)]];
pub const D: Matrix = [[ONE, ZERO, ZERO], [ZERO, ONE, ZERO], [ZERO, ZERO, Z(-1, 0)]];

impl Z {
    pub fn add(self, rhs: Self) -> Self { Self(self.0 + rhs.0, self.1 + rhs.1) }
    pub fn sub(self, rhs: Self) -> Self { Self(self.0 - rhs.0, self.1 - rhs.1) }
    pub fn mul(self, rhs: Self) -> Self { Self(self.0 * rhs.0 - 5 * self.1 * rhs.1, self.0 * rhs.1 + self.1 * rhs.0 + self.1 * rhs.1) }
    pub fn conj(self) -> Self { Self(self.0 + self.1, -self.1) }
    pub fn norm(self) -> i128 { self.0 * self.0 + self.0 * self.1 + 5 * self.1 * self.1 }
    pub fn eval11(self, root: i128) -> i128 { (self.0 + root * self.1).rem_euclid(11) }
}

pub fn mmul(a: Matrix, b: Matrix) -> Matrix {
    std::array::from_fn(|i| std::array::from_fn(|j| (0..3).fold(ZERO, |s, k| s.add(a[i][k].mul(b[k][j])))))
}

pub fn adjoint(a: Matrix) -> Matrix {
    std::array::from_fn(|i| std::array::from_fn(|j| a[j][i].conj()))
}

pub fn scale(a: Matrix, n: i128) -> Matrix { a.map(|r| r.map(|z| z.mul(Z(n, 0)))) }
pub fn identity() -> Matrix { std::array::from_fn(|i| std::array::from_fn(|j| if i == j { ONE } else { ZERO })) }
pub fn flat(a: Matrix) -> Vec<i128> { a.into_iter().flatten().flat_map(|z| [z.0, z.1]).collect() }

pub fn determinant(a: Matrix) -> Z {
    let p = a[0][0].mul(a[1][1].mul(a[2][2]).sub(a[1][2].mul(a[2][1])));
    let q = a[0][1].mul(a[1][0].mul(a[2][2]).sub(a[1][2].mul(a[2][0])));
    let r = a[0][2].mul(a[1][0].mul(a[2][1]).sub(a[1][1].mul(a[2][0])));
    p.sub(q).add(r)
}

pub fn adjugate(a: Matrix) -> Matrix {
    std::array::from_fn(|i| std::array::from_fn(|j| {
        let rows: Vec<_> = (0..3).filter(|&r| r != j).collect();
        let cols: Vec<_> = (0..3).filter(|&c| c != i).collect();
        a[rows[0]][cols[0]].mul(a[rows[1]][cols[1]]).sub(a[rows[0]][cols[1]].mul(a[rows[1]][cols[0]])).mul(Z(if (i + j) % 2 == 0 { 1 } else { -1 }, 0))
    }))
}

pub fn divide(a: Matrix, n: i128) -> Option<Matrix> {
    if a.iter().flatten().any(|z| z.0 % n != 0 || z.1 % n != 0) { return None; }
    Some(a.map(|r| r.map(|z| Z(z.0 / n, z.1 / n))))
}

pub fn gram(basis: Matrix, form: Matrix) -> Matrix { mmul(adjoint(basis), mmul(form, basis)) }

pub fn compatible(s: Matrix, t: Matrix) -> Option<Matrix> {
    let a0 = mmul(adjugate(t), mmul(s, t));
    let numerator = mmul(t, mmul(D, mmul(a0, mmul(D, adjugate(t)))));
    divide(numerator, 9)
}

pub struct InputMap { pub id: usize, pub s: Matrix, pub mixed: bool }

fn parse_matrix(text: &str) -> Matrix {
    let numbers: Vec<i128> = text.split(|c: char| !c.is_ascii_digit() && c != '-').filter(|v| !v.is_empty()).map(|v| v.parse().expect("integer input")).collect();
    assert_eq!(numbers.len(), 18);
    std::array::from_fn(|i| std::array::from_fn(|j| Z(numbers[6 * i + 2 * j], numbers[6 * i + 2 * j + 1])))
}

pub fn import_maps(raw: &str) -> Vec<InputMap> {
    let mut out = Vec::new();
    for line in raw.lines().filter(|line| line.starts_with("FOUR_MAP\t")) {
        let fields: Vec<_> = line.split('\t').collect();
        assert_eq!(fields.len(), 10);
        let id: usize = fields[1].strip_prefix("id=").expect("id prefix").parse().expect("id integer");
        assert_eq!(id, out.len());
        let s = parse_matrix(fields[2].strip_prefix("S=").expect("matrix prefix"));
        let mixed = match fields[9] { "2divisible=false" => true, "2divisible=true" => false, _ => panic!("divisibility field") };
        assert_eq!(fields[4], "Norm=64");
        assert_eq!(fields[5], if mixed { "Smith=[1, 1, 2, 2, 4, 4]" } else { "Smith=[2, 2, 2, 2, 2, 2]" });
        assert_eq!(s.iter().flatten().all(|z| z.0 % 2 == 0 && z.1 % 2 == 0), !mixed);
        assert_eq!(gram(s, H), scale(H, 4));
        assert_eq!(determinant(s).norm(), 64);
        out.push(InputMap { id, s, mixed });
    }
    assert_eq!(out.len(), 156);
    assert_eq!(out.iter().filter(|m| m.mixed).count(), 144);
    let unique: std::collections::BTreeSet<_> = out.iter().map(|m| m.s).collect();
    assert_eq!(unique.len(), out.len());
    out
}
