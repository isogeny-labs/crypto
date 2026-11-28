use crate::field_d::*;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct K(pub [u8; 3]);
pub fn k(c: u8) -> K {
    K([c, 0, 0])
}
impl K {
    pub fn plus(self, b: Self) -> Self {
        Self(std::array::from_fn(|i| add(self.0[i], b.0[i])))
    }
    pub fn minus(self, b: Self) -> Self {
        self.plus(b.scaled(6))
    }
    pub fn scaled(self, c: u8) -> Self {
        Self(self.0.map(|a| mul(a, c)))
    }
    pub fn times(self, b: Self) -> Self {
        let mut raw = [0; 5];
        for (i, &a) in self.0.iter().enumerate() {
            for (j, &b) in b.0.iter().enumerate() {
                raw[i + j] = add(raw[i + j], mul(a, b));
            }
        }
        for degree in (3..5).rev() {
            let c = raw[degree];
            raw[degree - 3] = add(raw[degree - 3], c);
            raw[degree - 2] = add(raw[degree - 2], mul(5, c));
            raw[degree] = 0;
        }
        Self([raw[0], raw[1], raw[2]])
    }
    pub fn pow(self, n: usize) -> Self {
        (0..n).fold(k(1), |a, _| a.times(self))
    }
    pub fn inverse(self) -> Self {
        assert_ne!(self, k(0));
        let r = self.pow(341);
        assert_eq!(self.times(r), k(1));
        r
    }
}
pub fn actual_roots() -> [K; 3] {
    let values: Vec<u8> = (0..7)
        .map(|x| add(add(mul(mul(x, x), x), mul(2, x)), 6))
        .collect();
    assert!(values.iter().all(|&c| c != 0));
    let alpha = K([0, 1, 0]);
    let roots = [alpha, alpha.pow(7), alpha.pow(49)];
    for (i, &a) in roots.iter().enumerate() {
        assert_eq!(a.pow(3).plus(a.scaled(2)).plus(k(6)), k(0));
        assert_eq!(a.pow(343), a);
        assert_eq!(a.pow(7), roots[(i + 1) % 3]);
    }
    assert!(roots
        .iter()
        .enumerate()
        .all(|(i, a)| roots.iter().enumerate().all(|(j, b)| i == j || a != b)));
    assert_eq!(roots.into_iter().fold(k(0), |a, b| a.plus(b)), k(0));
    println!("ACTUAL_CUBIC_FIELD\tmodulus=[6,2,0,1]\tall-seven-base-evaluations={values:?}\troots-alpha-Frob7-Frob49={roots:?}\tall-three-Frob343-and-cubic-identities=true");
    roots
}
pub fn mobius(roots: [K; 3]) -> [u8; 4] {
    let a = roots[0];
    let b = a.pow(49);
    let columns = [a.scaled(6), k(6), a.times(b), b];
    let original: Vec<Vec<u8>> = (0..3)
        .map(|i| columns.iter().map(|v| v.0[i]).collect())
        .collect();
    let mut m = original.clone();
    let mut row = 0;
    let mut pivots = Vec::new();
    for column in 0..4 {
        let Some(pivot) = (row..m.len()).find(|&i| m[i][column] != 0) else {
            continue;
        };
        m.swap(row, pivot);
        let scale = inv(m[row][column]);
        for c in &mut m[row] {
            *c = mul(*c, scale);
        }
        let normalized = m[row].clone();
        for (i, r) in m.iter_mut().enumerate() {
            if i != row {
                let c = r[column];
                for (v, &p) in r.iter_mut().zip(&normalized) {
                    *v = sub(*v, mul(c, p));
                }
            }
        }
        pivots.push(column);
        row += 1;
    }
    println!(
        "ACTUAL_FROB49_MOBIUS_SYSTEM\tinput={original:?}\trank={}\tpivots={pivots:?}\trref={m:?}",
        pivots.len()
    );
    assert_eq!(pivots.len(), 3);
    let free: Vec<usize> = (0..4).filter(|i| !pivots.contains(i)).collect();
    assert_eq!(free.len(), 1);
    let mut v = [0; 4];
    v[free[0]] = 1;
    for (i, &col) in pivots.iter().enumerate() {
        v[col] = sub(0, m[i][free[0]]);
    }
    let scale = inv(*v.iter().find(|&&c| c != 0).unwrap());
    v = v.map(|c| mul(c, scale));
    let residual: Vec<u8> = original
        .iter()
        .map(|r| r.iter().zip(v).fold(0, |s, (&a, b)| add(s, mul(a, b))))
        .collect();
    assert!(residual.iter().all(|&v| v == 0));
    assert_ne!(sub(mul(v[0], v[3]), mul(v[1], v[2])), 0);
    assert_ne!(v[2], 0);
    println!("ACTUAL_FROB49_MOBIUS_KERNEL\tA-B-C-D={v:?}\tall-row-residuals={residual:?}");
    for (i, root) in roots.into_iter().enumerate() {
        let numerator = root.scaled(v[0]).plus(k(v[1]));
        let denominator = root.scaled(v[2]).plus(k(v[3]));
        let image = numerator.times(denominator.inverse());
        assert_eq!(image, root.pow(49));
        println!("ALL_ACTUAL_TORSION_ROOT_IMAGES\troot-index={i}\troot={root:?}\tnumerator={numerator:?}\tdenominator={denominator:?}\timage={image:?}\tactual-Frob49={:?}",root.pow(49));
    }
    v
}
