const INPUT: &str = include_str!("first-B-native/resolvents-incidence.stdout");
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
const ZERO: O = O(0, 0);
const ONE: O = O(1, 0);
type Mat = [[O; 3]; 3];
fn product(a: Mat, b: Mat) -> Mat {
    std::array::from_fn(|i| {
        std::array::from_fn(|j| (0..3).fold(ZERO, |s, k| s.plus(a[i][k].times(b[k][j]))))
    })
}
fn dagger(a: Mat) -> Mat {
    std::array::from_fn(|i| std::array::from_fn(|j| a[j][i].bar()))
}
fn dot(a: [O; 3], b: [O; 3]) -> O {
    a.into_iter()
        .zip(b)
        .fold(ZERO, |s, (a, b)| s.plus(a.times(b)))
}
fn action(a: Mat, b: [O; 3]) -> [O; 3] {
    a.map(|r| dot(r, b))
}
fn norm(a: Mat, b: [O; 3]) -> O {
    dot(b.map(O::bar), action(a, b))
}
fn parse(raw: &str) -> Vec<O> {
    let n: Vec<i128> = raw
        .split(|c: char| !c.is_ascii_digit() && c != '-')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();
    assert!(n.len().is_multiple_of(2));
    n.as_chunks::<2>().0.iter().map(|c| O(c[0], c[1])).collect()
}
fn main() {
    println!("PROFILE\tactual-CM19-kernel-basis-lower-norm-C\tfixed-known-object\tall-scientific-standing-unsigned\tno-runtime-benchmark-or-minimum-claim");
    let line = INPUT
        .lines()
        .find(|s| s.starts_with("ACTUAL_INTEGRAL_PRYM_INPUT\t"))
        .unwrap();
    let field = |prefix: &str| {
        line.split('\t')
            .find_map(|s| s.strip_prefix(prefix))
            .unwrap()
    };
    let hflat = parse(field("H="));
    assert_eq!(hflat.len(), 9);
    let h: Mat = std::array::from_fn(|i| std::array::from_fn(|j| hflat[3 * i + j]));
    let tau: [O; 3] = parse(field("tau=")).try_into().unwrap();
    let ell: [O; 3] = parse(field("ell=")).try_into().unwrap();
    let old: [O; 3] = parse(field("kappa=4tau-Ldagger=")).try_into().unwrap();
    assert_eq!((ell[1], ell[2]), (ONE, ZERO));
    assert_eq!(dot(ell, tau), ONE);
    let first = [ONE, ZERO.minus(ell[0]), ZERO];
    let second = [ZERO, ZERO, ONE];
    let f: Mat = std::array::from_fn(|i| [tau[i], first[i], second[i]]);
    let r0 = std::array::from_fn(|i| O(i128::from(i == 0), 0).minus(tau[0].times(ell[i])));
    let r2 = std::array::from_fn(|i| O(i128::from(i == 2), 0).minus(tau[2].times(ell[i])));
    let inverse = [ell, r0, r2];
    let identity: Mat =
        std::array::from_fn(|i| std::array::from_fn(|j| if i == j { ONE } else { ZERO }));
    assert_eq!(product(f, inverse), identity);
    assert_eq!(product(inverse, f), identity);
    assert_eq!(dot(ell, first), ZERO);
    assert_eq!(dot(ell, second), ZERO);
    assert_eq!(dot(r0, tau), ZERO);
    assert_eq!(dot(r2, tau), ZERO);
    let gram = product(product(dagger(f), h), f);
    let restricted: [[O; 2]; 2] =
        std::array::from_fn(|i| std::array::from_fn(|j| gram[i + 1][j + 1]));
    let det = restricted[0][0]
        .times(restricted[1][1])
        .minus(restricted[0][1].times(restricted[1][0]));
    assert_eq!(det, O(4, 0));
    let incidence_gram: Mat = std::array::from_fn(|i| {
        std::array::from_fn(|j| h[i][j].times(O(2, 0)).plus(ell[i].bar().times(ell[j])))
    });
    let incidence_basis = product(product(dagger(f), incidence_gram), f);
    let incidence_restricted: [[O; 2]; 2] =
        std::array::from_fn(|i| std::array::from_fn(|j| incidence_basis[i + 1][j + 1]));
    assert_eq!(
        incidence_restricted,
        restricted.map(|r| r.map(|v| v.times(O(2, 0))))
    );
    let rho3 = [ZERO, ZERO, ONE];
    assert_eq!(dot(rho3, second), ONE);
    assert_eq!(dot(ell, second), ZERO);
    let input_norm = norm(h, second);
    let chain_norm = norm(incidence_gram, second);
    let old_norm = norm(h, old);
    assert_eq!(
        (input_norm, chain_norm, old_norm),
        (O(3, 0), O(6, 0), O(124, 0))
    );
    println!("FULL_ACTUAL_KERNEL_DECOMPOSITION\tH={h:?}\tell={ell:?}\ttau={tau:?}\tbasis-F={f:?}\tinverse={inverse:?}\tF-inverse={:?}\tinverse-F={:?}",product(f,inverse),product(inverse,f));
    println!("FULL_ACTUAL_KERNEL_HERMITIAN\tFdagger-H-F={gram:?}\tG={restricted:?}\tdet-G={det:?}\tincidence-pullback=2H+ell-dagger-ell={incidence_gram:?}\tfull-incidence-basis={incidence_basis:?}\tincidence-restricted={incidence_restricted:?}");
    println!("ACTUAL_BETA3_PRYM_CHAIN_INPUT\tkappa3={second:?}\trho3={rho3:?}\tell-kappa3={:?}\trho3-kappa3={:?}\tinput-norm={input_norm:?}\tchain-ambient-norm={chain_norm:?}\told-kappa-norm={old_norm:?}\told-chain-ambient-norm={:?}\tactual-Psi-Jacobian-identity=separate-qualified-written-bridge",dot(ell,second),dot(rho3,second),old_norm.times(O(2,0)));
    let mut bad_input = second;
    bad_input[0] = bad_input[0].plus(ONE);
    assert_ne!(dot(ell, bad_input), ZERO);
    let mut bad_rho = rho3;
    bad_rho[2] = bad_rho[2].plus(ONE);
    assert_ne!(dot(bad_rho, second), ONE);
    println!("REAL_INPUT_AND_RETRACTION_MUTANTS\tinput0+1={bad_input:?}\tell-bad={:?}\tactual-bad-input-norm={:?}\trho2+1={bad_rho:?}\tbad-rho-kappa={:?}\twrong-kernel-and-left-inverse-rejected",dot(ell,bad_input),norm(h,bad_input),dot(bad_rho,second));
    assert_eq!(dot(ell, second), ZERO);
    assert_eq!(dot(rho3, second), ONE);
    assert_eq!(norm(incidence_gram, second), O(6, 0));
    println!("RESTORED_PRISTINE\tactual-split-kernel-basis\tlower-norm-beta3-chain\tall-integral-identities\tno-new-point-population-or-global-shortest-vector-assertion");
}
