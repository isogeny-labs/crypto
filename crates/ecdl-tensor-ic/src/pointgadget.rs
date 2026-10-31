//! Point-level gadgets and the gadget relation network (Go: pointgadget.go).

use std::rc::Rc;

use num_bigint::{BigInt, BigUint};

use crate::curve::{Curve, Point};
use crate::gadgets::{bits_uint, bool_int, ArithRef, Builder, Operand, Term, Var};
use crate::relation::FactorBases;
use crate::tensor::{Network, NetworkStats, PredVertex, Vertex};

/// FieldPoint is a point whose coordinates are Vars. Inf, if not None, is a
/// 1-bit Var flagging the identity, encoded canonically as Inf = 1, X = Y = 0.
/// A None Inf means the point is known to be affine.
#[derive(Clone)]
pub struct FieldPoint {
    pub x: Var,
    pub y: Var,
    pub inf: Option<Var>,
}

/// Go: bi64(x uint64) *big.Int.
pub fn bi64(x: u64) -> BigUint {
    BigUint::from(x)
}

fn ov(v: &Var) -> Operand {
    Operand::Var(v.clone())
}

fn term(coef: i64, v: &Var) -> Term {
    Term { coef, v: v.clone() }
}

impl Builder {
    /// AddGeneric defines P3 = P1 + P2 for the generic case x1 != x2, using
    /// only low-degree gadgets:
    ///
    /// ```text
    /// dx = x2 - x1 (nonzero), dy = y2 - y1, lambda*dx = dy,
    /// s = lambda^2, x3 = s - x1 - x2, e = x1 - x3, f = lambda*e, y3 = f - y1.
    /// ```
    ///
    /// Every intermediate value is range-checked to [0, p), so it is pinned
    /// down uniquely. Pairs with x1 = x2 (doubling, P + (-P)) and the identity
    /// have no satisfying assignment.
    pub fn add_generic(&mut self, ar: &ArithRef, p1: &FieldPoint, p2: &FieldPoint) -> FieldPoint {
        let zero = BigInt::from(0);
        let dx = self.field_var(ar, "dx");
        self.lin_check(
            ar,
            Some(&dx),
            true,
            &[term(-1, &p2.x), term(1, &p1.x)],
            &zero,
        );
        self.range_check(ar, &dx, &ar.p, true);
        let dy = self.field_var(ar, "dy");
        self.lin_check(
            ar,
            Some(&dy),
            true,
            &[term(-1, &p2.y), term(1, &p1.y)],
            &zero,
        );

        let lambda = self.field_var(ar, "lambda");
        let p = ar.p.clone();
        self.hint(
            &[dx.clone(), dy.clone()],
            std::slice::from_ref(&lambda),
            move |inn| match crate::gadgets::mod_inverse(&inn[0], &p) {
                None => Vec::new(),
                Some(inv) => vec![vec![(inv * &inn[1]) % &p]],
            },
        );
        self.range_check(ar, &lambda, &ar.p, false);
        self.mul_check(ar, &ov(&lambda), &ov(&dx), &dy, false);

        let s = self.field_var(ar, "s");
        self.mul_check(ar, &ov(&lambda), &ov(&lambda), &s, true);
        let x3 = self.field_var(ar, "x3");
        self.lin_check(
            ar,
            Some(&x3),
            true,
            &[term(-1, &s), term(1, &p1.x), term(1, &p2.x)],
            &zero,
        );
        let e = self.field_var(ar, "e");
        self.lin_check(ar, Some(&e), true, &[term(-1, &p1.x), term(1, &x3)], &zero);
        let f = self.field_var(ar, "f");
        self.mul_check(ar, &ov(&lambda), &ov(&e), &f, true);
        let y3 = self.field_var(ar, "y3");
        self.lin_check(ar, Some(&y3), true, &[term(-1, &f), term(1, &p1.y)], &zero);
        FieldPoint {
            x: x3,
            y: y3,
            inf: None,
        }
    }

    /// ValidityCheck constrains P to be an affine point of c with canonical
    /// coordinates: y^2 = x^3 + a x + b. The coordinates must be defined
    /// already.
    pub fn validity_check(&mut self, ar: &ArithRef, c: &Curve, pt: &FieldPoint) {
        self.range_check(ar, &pt.x, &ar.p, false);
        self.range_check(ar, &pt.y, &ar.p, false);
        let y2 = self.field_var(ar, "y2");
        self.mul_check(ar, &ov(&pt.y), &ov(&pt.y), &y2, true);
        let x2 = self.field_var(ar, "x2");
        self.mul_check(ar, &ov(&pt.x), &ov(&pt.x), &x2, true);
        let x3 = self.field_var(ar, "x3");
        self.mul_check(ar, &ov(&x2), &ov(&pt.x), &x3, true);
        let ax = self.field_var(ar, "ax");
        self.mul_check(ar, &Operand::Const(bi64(c.a)), &ov(&pt.x), &ax, true);
        self.lin_check(
            ar,
            None,
            false,
            &[term(1, &y2), term(-1, &x3), term(-1, &ax)],
            &(-BigInt::from(c.b)),
        );
    }

    /// EqualsConst constrains v = k block by block.
    pub fn equals_const(&mut self, ar: &ArithRef, v: &Var, k: &BigUint) {
        let blk = self.use_var(v);
        let kb = ar.blocks_of(k);
        for j in 0..blk.len() {
            let want = kb[j].clone();
            self.add_vertex(Vertex::Pred(PredVertex {
                e: blk[j].clone(),
                p: Rc::new(move |vals: &[bool]| bits_uint(vals) == want),
            }));
        }
    }

    /// MembershipCheck constrains the affine point pt to lie in F_i of an
    /// affine factor base: h = alpha x + beta y + gamma (mod p) and
    /// h < Bound_i. Panics "ecdl: MembershipCheck needs affine factor bases".
    pub fn membership_check(&mut self, ar: &ArithRef, fb: &FactorBases, i: usize, pt: &FieldPoint) {
        if !fb.affine {
            panic!("ecdl: MembershipCheck needs affine factor bases");
        }
        let u1 = self.field_var(ar, "alpha*x");
        self.mul_check(
            ar,
            &Operand::Const(bi64(fb.alpha[i])),
            &ov(&pt.x),
            &u1,
            true,
        );
        let u2 = self.field_var(ar, "beta*y");
        self.mul_check(ar, &Operand::Const(bi64(fb.beta[i])), &ov(&pt.y), &u2, true);
        let h = self.field_var(ar, "hash");
        self.lin_check(
            ar,
            Some(&h),
            true,
            &[term(-1, &u1), term(-1, &u2)],
            &(-BigInt::from(fb.gamma[i])),
        );
        self.range_check(ar, &h, &bi64(fb.bound[i]), false);
        if !fb.x_bound.is_empty() {
            // Compact coordinate constraint: x < x_bound[i], a block-level
            // compare chain of degree O(t) that reads only x.
            self.range_check(ar, &pt.x, &bi64(fb.x_bound[i]), false);
        }
    }

    /// CompleteAdd defines P3 = P1 + P2 for every input, including the
    /// identity, doubling and P + (-P), without branching. Every case's
    /// intermediate values are pinned down uniquely:
    ///
    /// ```text
    /// dx = x2 - x1, dy = y2 - y1, zdx = [dx = 0], zdy = [dy = 0]
    /// den = zdx ? (zdy ? 2 y1 : 0) : dx,  num = zdx ? 3 x1^2 + a : dy
    /// zden = [den = 0]; lambda * den = (zden ? 0 : num), and zden => lambda = 0
    /// (x', y') = chord/tangent formulas from lambda, or (0, 0) if zden
    /// P3 = P1 is O ? P2 : P2 is O ? P1 : (x', y'), with
    /// inf3 = (inf1 and inf2) or (not inf1 and not inf2 and zden).
    /// ```
    ///
    /// zden covers both x1 = x2 cases that give O: y2 = -y1 != y1, and
    /// y1 = y2 = 0.
    pub fn complete_add(
        &mut self,
        ar: &ArithRef,
        c: &Curve,
        p1: &FieldPoint,
        p2: &FieldPoint,
    ) -> FieldPoint {
        let zero = BigInt::from(0);
        let zero_u = BigUint::from(0u32);
        let dx = self.field_var(ar, "dx");
        self.lin_check(
            ar,
            Some(&dx),
            true,
            &[term(-1, &p2.x), term(1, &p1.x)],
            &zero,
        );
        let dy = self.field_var(ar, "dy");
        self.lin_check(
            ar,
            Some(&dy),
            true,
            &[term(-1, &p2.y), term(1, &p1.y)],
            &zero,
        );
        let (zdx, zdy) = (self.is_zero(&dx), self.is_zero(&dy));

        let x1sq = self.field_var(ar, "x1^2");
        self.mul_check(ar, &ov(&p1.x), &ov(&p1.x), &x1sq, true);
        let num_d = self.field_var(ar, "3x1^2+a");
        self.lin_check(
            ar,
            Some(&num_d),
            true,
            &[term(-3, &x1sq)],
            &(-BigInt::from(c.a)),
        );
        let den_d = self.field_var(ar, "2y1");
        self.lin_check(ar, Some(&den_d), true, &[term(-2, &p1.y)], &zero);

        let t1 = self.select(ar, &zdy, &ov(&den_d), &Operand::Const(zero_u.clone()));
        let den = self.select(ar, &zdx, &ov(&t1), &ov(&dx));
        let num = self.select(ar, &zdx, &ov(&num_d), &ov(&dy));
        let zden = self.is_zero(&den);
        let num_sel = self.select(ar, &zden, &Operand::Const(zero_u.clone()), &ov(&num));

        let lambda = self.field_var(ar, "lambda");
        let p = ar.p.clone();
        self.hint(
            &[den.clone(), num_sel.clone()],
            std::slice::from_ref(&lambda),
            move |inn| {
                if inn[0].bits() == 0 {
                    return vec![vec![BigUint::from(0u32)]];
                }
                let inv = crate::gadgets::mod_inverse(&inn[0], &p)
                    .expect("ecdl: complete add: denominator not invertible");
                vec![vec![(inv * &inn[1]) % &p]]
            },
        );
        self.range_check(ar, &lambda, &ar.p, false);
        self.mul_check(ar, &ov(&lambda), &ov(&den), &num_sel, false);
        self.zero_if(ar, &zden, &lambda);

        let s = self.field_var(ar, "s");
        self.mul_check(ar, &ov(&lambda), &ov(&lambda), &s, true);
        let xr = self.field_var(ar, "x'");
        self.lin_check(
            ar,
            Some(&xr),
            true,
            &[term(-1, &s), term(1, &p1.x), term(1, &p2.x)],
            &zero,
        );
        let e = self.field_var(ar, "e");
        self.lin_check(ar, Some(&e), true, &[term(-1, &p1.x), term(1, &xr)], &zero);
        let f = self.field_var(ar, "f");
        self.mul_check(ar, &ov(&lambda), &ov(&e), &f, true);
        let yr = self.field_var(ar, "y'");
        self.lin_check(ar, Some(&yr), true, &[term(-1, &f), term(1, &p1.y)], &zero);

        let mut x3 = self.select(ar, &zden, &Operand::Const(zero_u.clone()), &ov(&xr));
        let mut y3 = self.select(ar, &zden, &Operand::Const(zero_u.clone()), &ov(&yr));
        if let Some(i2) = &p2.inf {
            x3 = self.select(ar, i2, &ov(&p1.x), &ov(&x3));
            y3 = self.select(ar, i2, &ov(&p1.y), &ov(&y3));
        }
        if let Some(i1) = &p1.inf {
            x3 = self.select(ar, i1, &ov(&p2.x), &ov(&x3));
            y3 = self.select(ar, i1, &ov(&p2.y), &ov(&y3));
        }

        // inf3 from the flags; absent flags are constant false.
        let inf3 = self.new_bit("inf");
        let mut flags: Vec<Var> = Vec::new();
        let has_inf = [p1.inf.is_some(), p2.inf.is_some()];
        for fl in [&p1.inf, &p2.inf].into_iter().flatten() {
            flags.push(fl.clone());
        }
        flags.push(zden.clone());
        let rule = move |vals: &[bool]| -> bool {
            let mut i = 0usize;
            let mut get = |present: bool| -> bool {
                if !present {
                    return false;
                }
                let v = vals[i];
                i += 1;
                v
            };
            let i1 = get(has_inf[0]);
            let i2 = get(has_inf[1]);
            let z = get(true);
            (i1 && i2) || (!i1 && !i2 && z)
        };
        self.hint(&flags, std::slice::from_ref(&inf3), move |inn| {
            let vals: Vec<bool> = inn.iter().map(|x| x.bits() != 0).collect();
            vec![vec![bool_int(rule(&vals))]]
        });
        let mut edges: Vec<usize> = Vec::new();
        for fl in &flags {
            edges.extend(self.use_var(fl)[0].iter().copied());
        }
        edges.extend(self.use_var(&inf3)[0].iter().copied());
        self.add_vertex(Vertex::Pred(PredVertex {
            e: edges,
            p: Rc::new(move |vals: &[bool]| {
                let n = vals.len();
                rule(&vals[..n - 1]) == vals[n - 1]
            }),
        }));
        FieldPoint {
            x: x3,
            y: y3,
            inf: Some(inf3),
        }
    }
}

/// GadgetRelationNetwork is RelationNetwork built entirely from low-degree
/// gadgets, for affine factor bases. Each A_i is proposed by a hint (which
/// lists F_i, a toy shortcut) and constrained only by ValidityCheck and
/// MembershipCheck; partial sums use CompleteAdd. It has no support vertex,
/// and it counts exactly what RelationNetwork counts for the same factor
/// bases.
pub fn gadget_relation_network(
    c: &Curve,
    ar: &ArithRef,
    fb: &FactorBases,
    r: Point,
) -> (Network, Vec<FieldPoint>) {
    let mut b = Builder::default();
    let mut a: Vec<FieldPoint> = Vec::with_capacity(fb.sets.len());
    for (i, set) in fb.sets.iter().enumerate() {
        let pt = FieldPoint {
            x: b.field_var(ar, "x"),
            y: b.field_var(ar, "y"),
            inf: None,
        };
        let set = if fb.x_bound.is_empty() {
            set.clone()
        } else {
            fb.window_candidates(c, i)
        };
        b.hint(&[], &[pt.x.clone(), pt.y.clone()], move |_| {
            set.iter().map(|q| vec![bi64(q.x), bi64(q.y)]).collect()
        });
        b.validity_check(ar, c, &pt);
        b.membership_check(ar, fb, i, &pt);
        a.push(pt);
    }
    let mut s = a[0].clone();
    for k in 1..a.len() {
        s = b.complete_add(ar, c, &s, &a[k]);
    }
    b.equals_const(
        ar,
        s.inf.as_ref().expect("ecdl: sum has no infinity flag"),
        &bool_int(r.inf),
    );
    b.equals_const(ar, &s.x, &bi64(r.x));
    b.equals_const(ar, &s.y, &bi64(r.y));
    (b.finalize(), a)
}

/// GadgetShapes builds, without evaluating, a field multiplication, a
/// complete point addition, and the per-point checks of a relation network
/// (validity plus membership) for ar's modulus, and returns their statistics.
/// Returns (mul, add, point).
pub fn gadget_shapes(ar: &ArithRef) -> (NetworkStats, NetworkStats, NetworkStats) {
    let mut b = Builder::default();
    let (x, y, c) = (
        b.field_var(ar, "x"),
        b.field_var(ar, "y"),
        b.field_var(ar, "c"),
    );
    b.hint_nil(&[], &[x.clone(), y.clone()]);
    b.range_check(ar, &x, &ar.p, false);
    b.range_check(ar, &y, &ar.p, false);
    b.mul_check(ar, &ov(&x), &ov(&y), &c, true);
    let mul = b.finalize().stats();

    let curve = Curve {
        a: 3,
        b: 7,
        ..Default::default()
    };
    let mut b = Builder::default();
    let p1 = FieldPoint {
        x: b.field_var(ar, "x1"),
        y: b.field_var(ar, "y1"),
        inf: Some(b.new_bit("i1")),
    };
    let p2 = FieldPoint {
        x: b.field_var(ar, "x2"),
        y: b.field_var(ar, "y2"),
        inf: Some(b.new_bit("i2")),
    };
    b.hint_nil(
        &[],
        &[
            p1.x.clone(),
            p1.y.clone(),
            p1.inf.clone().unwrap(),
            p2.x.clone(),
            p2.y.clone(),
            p2.inf.clone().unwrap(),
        ],
    );
    let p3 = b.complete_add(ar, &curve, &p1, &p2);
    b.range_check(ar, &p3.x, &ar.p, false);
    b.range_check(ar, &p3.y, &ar.p, false);
    b.equals_const(ar, p3.inf.as_ref().unwrap(), &BigUint::from(0u32));
    let add = b.finalize().stats();

    let fb = FactorBases {
        affine: true,
        alpha: vec![5],
        beta: vec![9],
        gamma: vec![1],
        bound: vec![1 << 20],
        ..Default::default()
    };
    let mut b = Builder::default();
    let pt = FieldPoint {
        x: b.field_var(ar, "x"),
        y: b.field_var(ar, "y"),
        inf: None,
    };
    b.hint_nil(&[], &[pt.x.clone(), pt.y.clone()]);
    b.validity_check(ar, &curve, &pt);
    b.membership_check(ar, &fb, 0, &pt);
    let point = b.finalize().stats();
    (mul, add, point)
}
