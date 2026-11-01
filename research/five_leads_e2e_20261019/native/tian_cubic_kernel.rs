//! Exact replay of the torsion data in Tian, arXiv:2012.07173, section 3.4.
//! This certifies the kernel data only; it does not reconstruct the quotient.
use serde_json::json;

const P: i64 = 523;
const Q3: u64 = 523 * 523 * 523;
fn m(x: i64) -> i64 {
    x.rem_euclid(P)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct K3([i64; 3]);
impl K3 {
    const Z: Self = Self([0, 0, 0]);
    const ONE: Self = Self([1, 0, 0]);
    const W: Self = Self([0, 1, 0]);
    fn c(c: i64) -> Self {
        Self([m(c), 0, 0])
    }
    fn add(self, o: Self) -> Self {
        Self(std::array::from_fn(|i| m(self.0[i] + o.0[i])))
    }
    fn neg(self) -> Self {
        Self(self.0.map(|x| m(-x)))
    }
    fn sub(self, o: Self) -> Self {
        self.add(o.neg())
    }
    fn mul(self, o: Self) -> Self {
        let mut t = [0; 5];
        for i in 0..3 {
            for j in 0..3 {
                t[i + j] += self.0[i] * o.0[j];
            }
        }
        for i in (3..5).rev() {
            t[i - 3] -= 2 * t[i];
        }
        Self([m(t[0]), m(t[1]), m(t[2])])
    }
    fn pow(self, mut e: u64) -> Self {
        let mut a = self;
        let mut z = Self::ONE;
        while e > 0 {
            if e & 1 == 1 {
                z = z.mul(a);
            }
            a = a.mul(a);
            e >>= 1;
        }
        z
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct K6 {
    a: K3,
    b: K3,
}
impl K6 {
    const Z: Self = Self { a: K3::Z, b: K3::Z };
    const ONE: Self = Self {
        a: K3::ONE,
        b: K3::Z,
    };
    fn c(x: i64) -> Self {
        Self {
            a: K3::c(x),
            b: K3::Z,
        }
    }
    fn a(a: K3) -> Self {
        Self { a, b: K3::Z }
    }
    fn theta(b: K3) -> Self {
        Self { a: K3::Z, b }
    }
    fn add(self, o: Self) -> Self {
        Self {
            a: self.a.add(o.a),
            b: self.b.add(o.b),
        }
    }
    fn neg(self) -> Self {
        Self {
            a: self.a.neg(),
            b: self.b.neg(),
        }
    }
    fn sub(self, o: Self) -> Self {
        self.add(o.neg())
    }
    fn mul(self, o: Self) -> Self {
        let theta2 = K3::W.add(K3::c(6));
        Self {
            a: self.a.mul(o.a).add(self.b.mul(o.b).mul(theta2)),
            b: self.a.mul(o.b).add(self.b.mul(o.a)),
        }
    }
    fn pow(self, mut e: u64) -> Self {
        let mut a = self;
        let mut z = Self::ONE;
        while e > 0 {
            if e & 1 == 1 {
                z = z.mul(a);
            }
            a = a.mul(a);
            e >>= 1;
        }
        z
    }
    fn inv(self) -> Self {
        assert_ne!(self, Self::Z);
        let theta2 = K3::W.add(K3::c(6));
        let norm = self.a.mul(self.a).sub(self.b.mul(self.b).mul(theta2));
        let ni = norm.pow(Q3 - 2);
        Self {
            a: self.a.mul(ni),
            b: self.b.neg().mul(ni),
        }
    }
    fn div(self, o: Self) -> Self {
        self.mul(o.inv())
    }
    fn frob(self) -> Self {
        self.pow(P as u64)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Pt {
    x: K6,
    y: K6,
}
type MaybePt = Option<Pt>;
fn add(p: MaybePt, q: MaybePt, a: K6) -> MaybePt {
    let (Some(p), Some(q)) = (p, q) else {
        return p.or(q);
    };
    if p.x == q.x && p.y == q.y.neg() {
        return None;
    }
    let slope = if p == q {
        K6::c(3).mul(p.x.mul(p.x)).add(a).div(K6::c(2).mul(p.y))
    } else {
        q.y.sub(p.y).div(q.x.sub(p.x))
    };
    let x = slope.mul(slope).sub(p.x).sub(q.x);
    let y = slope.mul(p.x.sub(x)).sub(p.y);
    Some(Pt { x, y })
}
fn mul(mut p: MaybePt, mut n: u64, a: K6) -> MaybePt {
    let mut z = None;
    while n > 0 {
        if n & 1 == 1 {
            z = add(z, p, a);
        }
        p = add(p, p, a);
        n >>= 1;
    }
    z
}
fn frob_pt(p: MaybePt) -> MaybePt {
    p.map(|p| Pt {
        x: p.x.frob(),
        y: p.y.frob(),
    })
}
fn tangent(p: Pt, t: Pt, a: K6) -> K6 {
    let slope = K6::c(3).mul(p.x.mul(p.x)).add(a).div(K6::c(2).mul(p.y));
    t.y.sub(p.y).sub(slope.mul(t.x.sub(p.x)))
}
fn pairing(p: Pt, q: Pt, a: K6, b: K6, offset: i64) -> K6 {
    // For 3-torsion the tangent line at P has divisor 3(P)-3(O).
    // The divisor-evaluation definition of the Weil pairing uses an
    // D_P=(P)-(O), D_Q=(Q+S)-(S). The function with divisor 3D_Q is
    // f_Q(T-S), so evaluate the second tangent at -S and P-S.
    for step in 0..P {
        let x0 = (step + offset) % P;
        let x = K3::c(x0);
        let rhs = x.mul(x).mul(x).add(a.a.mul(x)).add(b.a);
        let y = rhs.pow((Q3 + 1) / 4);
        if y.mul(y) != rhs {
            continue;
        }
        let s = Pt {
            x: K6::a(x),
            y: K6::a(y),
        };
        let Some(qs) = add(Some(q), Some(s), a) else {
            continue;
        };
        let minus_s = Pt {
            x: s.x,
            y: s.y.neg(),
        };
        let Some(p_minus_s) = add(Some(p), Some(minus_s), a) else {
            continue;
        };
        if s == p || qs == p || s == q || qs == q {
            continue;
        }
        let den1 = tangent(p, s, a);
        let den2 = tangent(q, p_minus_s, a);
        if den1 == K6::Z || den2 == K6::Z {
            continue;
        }
        let n1 = tangent(p, qs, a);
        let n2 = tangent(q, minus_s, a);
        if n1 == K6::Z || n2 == K6::Z {
            continue;
        }
        let e = n1.div(den1).mul(n2.div(den2));
        assert_eq!(e.pow(3), K6::ONE);
        return e;
    }
    panic!("no auxiliary point for pairing");
}
fn on_curve(p: Pt, a: K6, b: K6) -> bool {
    p.y.mul(p.y) == p.x.mul(p.x).mul(p.x).add(a.mul(p.x)).add(b)
}
fn k3(c: [i64; 3]) -> K3 {
    K3([m(c[0]), m(c[1]), m(c[2])])
}
fn prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}
fn pt(x: [i64; 3], y: [i64; 3]) -> MaybePt {
    Some(Pt {
        x: K6::a(k3(x)),
        y: K6::theta(k3(y)),
    })
}
type Triple = [MaybePt; 3];
fn plus(p: Triple, q: Triple, aa: &[K6; 3]) -> Triple {
    std::array::from_fn(|j| add(p[j], q[j], aa[j]))
}
fn times(p: Triple, n: u64, aa: &[K6; 3]) -> Triple {
    std::array::from_fn(|j| mul(p[j], n, aa[j]))
}
fn phi(p: Triple) -> Triple {
    [frob_pt(p[2]), frob_pt(p[0]), frob_pt(p[1])]
}
fn encode(p: Triple, q: Triple, r: Triple, aa: &[K6; 3]) -> Vec<Triple> {
    let mut out = vec![];
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                out.push(plus(
                    plus(times(p, i, aa), times(q, j, aa), aa),
                    times(r, k, aa),
                    aa,
                ));
            }
        }
    }
    out
}
fn main() {
    assert!((0..P).all(|x| m(x * x * x + 2) != 0));
    let theta2 = K3::W.add(K3::c(6));
    assert_eq!(theta2.pow((Q3 - 1) / 2), K3::c(-1));
    let a0 = K6::a(k3([140, 92, 82]));
    let b0 = K6::a(k3([243, 339, 359]));
    let source_order = 143069629u64;
    assert!(prime(source_order));
    assert!((source_order as i128 - Q3 as i128 - 1).pow(2) <= 4 * Q3 as i128);
    assert!(source_order as u128 * source_order as u128 > 16 * Q3 as u128);
    let source_point = (0..P)
        .find_map(|x0| {
            let x = K3::c(x0);
            let rhs = x.mul(x).mul(x).add(a0.a.mul(x)).add(b0.a);
            let y = rhs.pow((Q3 + 1) / 4);
            if y.mul(y) == rhs {
                Some(Pt {
                    x: K6::a(x),
                    y: K6::a(y),
                })
            } else {
                None
            }
        })
        .expect("source rational point");
    assert!(on_curve(source_point, a0, b0));
    assert_eq!(mul(Some(source_point), source_order, a0), None);
    let aa = [a0, a0.frob(), a0.frob().frob()];
    let bb = [b0, b0.frob(), b0.frob().frob()];
    let generators: [Triple; 3] = [
        [
            pt([337, 370, 222], [400, 387, 122]),
            pt([156, 287, 312], [493, 362, 21]),
            pt([156, 275, 415], [451, 251, 419]),
        ],
        [
            pt([156, 484, 319], [108, 484, 298]),
            pt([337, 442, 245], [6, 456, 305]),
            pt([156, 275, 415], [72, 272, 104]),
        ],
        [
            pt([156, 484, 319], [415, 39, 225]),
            pt([156, 287, 312], [493, 362, 21]),
            pt([337, 234, 56], [222, 109, 150]),
        ],
    ];
    for g in generators {
        for j in 0..3 {
            let p = g[j].expect("nonidentity generator component");
            assert!(
                on_curve(p, aa[j], bb[j]),
                "published component {j} off curve"
            );
            assert_eq!(mul(g[j], 3, aa[j]), None);
        }
    }
    let kernel = encode(generators[0], generators[1], generators[2], &aa);
    let unique: std::collections::HashSet<_> = kernel.iter().copied().collect();
    assert_eq!(unique.len(), 27);
    let mut matrix = [[0u8; 3]; 3];
    for (i, g) in generators.into_iter().enumerate() {
        let image = phi(g);
        let idx = kernel
            .iter()
            .position(|&h| h == image)
            .expect("Frobenius-stable kernel");
        matrix[i] = [(idx / 9) as u8, ((idx / 3) % 3) as u8, (idx % 3) as u8];
    }
    let mut pairing_products = vec![];
    for i in 0..3 {
        for k in (i + 1)..3 {
            let mut product = K6::ONE;
            let mut individual = vec![];
            for j in 0..3 {
                let x = generators[i][j].unwrap();
                let y = generators[k][j].unwrap();
                let e = pairing(x, y, aa[j], bb[j], 0);
                assert_eq!(e, pairing(x, y, aa[j], bb[j], 7));
                assert_eq!(e.mul(pairing(y, x, aa[j], bb[j], 0)), K6::ONE);
                product = product.mul(e);
                individual.push(json!([e.a.0, e.b.0]));
            }
            assert_eq!(product, K6::ONE, "kernel is not isotropic");
            pairing_products.push(json!({"i":i+1,"j":k+1,"components":individual,"product":[product.a.0,product.b.0]}));
        }
    }
    let output = json!({
        "source":"Tian arXiv:2012.07173 section 3.4",
        "field":"F_523[w,theta]/(w^3+2,theta^2-w-6)",
        "source_group_order":source_order,
        "source_group_order_certified_by_prime_point_order_and_hasse":true,
        "source_order_witness":[source_point.x.a.0,source_point.y.a.0],
        "field_tower_irreducible":true,
        "all_nine_components_on_conjugate_curves":true,
        "all_nine_components_exact_order_3":true,
        "kernel_cardinality":unique.len(),
        "source_subgroup_kernel_intersection_trivial_by_coprime_orders":source_order%3!=0,
        "frobenius_generator_rows":matrix,
        "pairing_products":pairing_products,
        "maximal_isotropic":true,
        "quotient_constructed":false,
        "transfer_executed":false
    });
    println!("{}", serde_json::to_string_pretty(&output).unwrap());
}
