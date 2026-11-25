#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct O(pub i128, pub i128);
pub type Matrix = [[O; 3]; 3];
pub const ZERO: O = O(0, 0);
pub const ONE: O = O(1, 0);
impl O {
    pub fn plus(self, b: Self) -> Self {
        Self(self.0 + b.0, self.1 + b.1)
    }
    pub fn negative(self) -> Self {
        Self(-self.0, -self.1)
    }
    pub fn times(self, b: Self) -> Self {
        Self(
            self.0 * b.0 - 5 * self.1 * b.1,
            self.0 * b.1 + self.1 * b.0 + self.1 * b.1,
        )
    }
    pub fn bar(self) -> Self {
        Self(self.0 + self.1, -self.1)
    }
}
pub fn product(a: Matrix, b: Matrix) -> Matrix {
    a.map(|row| {
        std::array::from_fn(|j| {
            row.into_iter()
                .enumerate()
                .fold(ZERO, |s, (k, x)| s.plus(x.times(b[k][j])))
        })
    })
}
pub fn dagger(a: Matrix) -> Matrix {
    std::array::from_fn(|i| std::array::from_fn(|j| a[j][i].bar()))
}
pub fn scaled(a: Matrix, n: i128) -> Matrix {
    a.map(|row| row.map(|x| x.times(O(n, 0))))
}
pub fn identity() -> Matrix {
    std::array::from_fn(|i| std::array::from_fn(|j| if i == j { ONE } else { ZERO }))
}
fn cofactor(a: Matrix, i: usize, j: usize) -> O {
    let r: Vec<_> = (0..3).filter(|&k| k != i).collect();
    let c: Vec<_> = (0..3).filter(|&k| k != j).collect();
    let value = a[r[0]][c[0]]
        .times(a[r[1]][c[1]])
        .plus(a[r[0]][c[1]].times(a[r[1]][c[0]]).negative());
    if (i + j).is_multiple_of(2) {
        value
    } else {
        value.negative()
    }
}
fn determinant(a: Matrix) -> O {
    (0..3).fold(ZERO, |s, j| s.plus(a[0][j].times(cofactor(a, 0, j))))
}
pub fn qualify() -> (Matrix, Matrix, Matrix) {
    let pi = O(-2, 1);
    let h = [
        [O(2, 0), ONE, O(-1, 0)],
        [ONE, O(3, 0), pi],
        [O(-1, 0), pi.bar(), O(3, 0)],
    ];
    let s = [
        [O(-4, 0), O(-2, 0), O(2, 0)],
        [O(2, 2), O(-1, 2), O(-1, -2)],
        [O(-4, 2), O(-3, 1), O(3, -1)],
    ];
    let hi = std::array::from_fn(|i| std::array::from_fn(|j| cofactor(h, j, i)));
    assert_eq!(determinant(h), ONE);
    assert_eq!(product(h, hi), identity());
    assert_eq!(product(product(dagger(s), h), s), scaled(h, 4));
    let ds = determinant(s);
    assert_eq!(ds, O(8, 0));
    assert_eq!(ds.times(ds.bar()), O(64, 0));
    let r = product(product(hi, dagger(s)), h);
    assert_eq!(product(r, s), scaled(identity(), 4));
    assert_eq!(product(s, r), scaled(identity(), 4));
    assert_eq!(product(product(dagger(r), h), r), scaled(h, 4));
    assert!(s.iter().flatten().any(|c| c.0 % 2 != 0 || c.1 % 2 != 0));
    let mut wrong_s = s;
    wrong_s[0][0] = wrong_s[0][0].plus(ONE);
    assert_ne!(product(product(dagger(wrong_s), h), wrong_s), scaled(h, 4));
    let mut wrong_h = h;
    wrong_h[0][0] = wrong_h[0][0].plus(ONE);
    assert_ne!(product(product(dagger(s), wrong_h), s), scaled(wrong_h, 4));
    assert_eq!(product(product(dagger(s), h), s), scaled(h, 4));
    println!("ACTUAL_CM_CHAIN\tH={:?}\tS={:?}\tR={:?}\tH_inverse={:?}\tnormdet=64\tSdagHS=RdagHR=4H\tRS=SR=4I\tactual_S_H_mutations_reject_restore\tPASS",h,s,r,hi);
    (h, s, r)
}
pub fn cover_inclusion() -> Matrix {
    [
        [ONE, ONE, ZERO],
        [O(-1, -1), ZERO, O(-1, 0)],
        [O(2, -1), ZERO, O(-1, 0)],
    ]
}
