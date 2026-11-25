const INPUT: &str =
    include_str!("../chained_cm19_actual_j4_20261123/first-B-native/actual-maps.stdout");
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct O(i128, i128);
impl O {
    fn plus(self, b: Self) -> Self {
        Self(self.0 + b.0, self.1 + b.1)
    }
    fn minus(self, b: Self) -> Self {
        self.plus(Self(-b.0, -b.1))
    }
    fn times(self, b: Self) -> Self {
        Self(
            self.0 * b.0 - 5 * self.1 * b.1,
            self.0 * b.1 + self.1 * b.0 + self.1 * b.1,
        )
    }
    fn bar(self) -> Self {
        Self(self.0 + self.1, -self.1)
    }
}
type Mat = [[O; 3]; 3];
const ZERO: O = O(0, 0);
const ONE: O = O(1, 0);
fn product(a: Mat, b: Mat) -> Mat {
    std::array::from_fn(|i| {
        std::array::from_fn(|j| (0..3).fold(ZERO, |s, k| s.plus(a[i][k].times(b[k][j]))))
    })
}
fn action(a: Mat, v: [O; 3]) -> [O; 3] {
    a.map(|row| dot(row, v))
}
fn dot(a: [O; 3], b: [O; 3]) -> O {
    a.into_iter()
        .zip(b)
        .fold(ZERO, |s, (a, b)| s.plus(a.times(b)))
}
fn norm(h: Mat, v: [O; 3]) -> O {
    dot(v.map(O::bar), action(h, v))
}
fn parse(raw: &str) -> Mat {
    let n: Vec<i128> = raw
        .split(|c: char| !c.is_ascii_digit() && c != '-')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();
    assert_eq!(n.len(), 18);
    std::array::from_fn(|i| std::array::from_fn(|j| O(n[6 * i + 2 * j], n[6 * i + 2 * j + 1])))
}
pub fn qualify() {
    let line = INPUT
        .lines()
        .find(|s| s.starts_with("ACTUAL_CM_CHAIN\t"))
        .unwrap();
    let field = |prefix: &str| {
        line.split('\t')
            .find_map(|s| s.strip_prefix(prefix))
            .unwrap()
    };
    let (h, hi, s) = (
        parse(field("H=")),
        parse(field("H_inverse=")),
        parse(field("S=")),
    );
    let identity =
        std::array::from_fn(|i| std::array::from_fn(|j| if i == j { ONE } else { ZERO }));
    assert_eq!(product(h, hi), identity);
    assert_eq!(product(hi, h), identity);
    let v = [ONE, O(-1, -1), O(2, -1)];
    let u = action(s, v);
    let ell = [O(0, 1), ONE, ZERO];
    let dual = action(hi, ell.map(O::bar));
    let kappa = std::array::from_fn(|i| u[i].times(O(4, 0)).minus(dual[i]));
    let rho = [O(-23, 0), O(-7, 0), O(10, 0)];
    assert_eq!(dot(ell, u), ONE);
    assert_eq!(dot(ell, dual), O(4, 0));
    assert_eq!(dot(ell, kappa), ZERO);
    assert_eq!(dot(rho, kappa), ONE);
    assert_eq!(norm(h, u), O(8, 0));
    assert_eq!(norm(h, kappa), O(124, 0));
    println!("ACTUAL_INTEGRAL_PRYM_INPUT\tH={h:?}\tHinv={hi:?}\tS={s:?}\tphi0={v:?}\ttau={u:?}\tell={ell:?}\tLdagger={dual:?}\tkappa=4tau-Ldagger={kappa:?}\trho={rho:?}\tell-tau={:?}\tell-Ldagger={:?}\tell-kappa={:?}\trho-kappa={:?}\ttau-norm={:?}\tkappa-norm={:?}",dot(ell,u),dot(ell,dual),dot(ell,kappa),dot(rho,kappa),norm(h,u),norm(h,kappa));
    let mut bad = kappa;
    bad[0] = bad[0].plus(ONE);
    assert_ne!(dot(ell, bad), ZERO);
    assert_ne!(dot(rho, bad), ONE);
    assert_eq!(dot(ell, kappa), ZERO);
    assert_eq!(dot(rho, kappa), ONE);
    println!("REAL_PRYM_INPUT_MUTANT\tkappa0+1\tell-bad={:?}\trho-bad={:?}\tboth-reject\tpristine-restored\tactual-Jacobian-correspondence-bridge-separate",dot(ell,bad),dot(rho,bad));
}
