//! Independent packed-field checker. Curve addition uses line intersection
//! against an exhaustively enumerated curve, not the producer slope formula.
use serde_json::{json, Value};
use std::{fs, path::Path};
type Pt = Option<(i64, i64)>;
struct Ref {
    p: i64,
    d: i64,
    a: i64,
    b: i64,
    points: Vec<(i64, i64)>,
}
impl Ref {
    fn plus(&self, x: i64, y: i64) -> i64 {
        (x % self.p + y % self.p).rem_euclid(self.p)
            + self.p * ((x / self.p + y / self.p).rem_euclid(self.p))
    }
    fn minus(&self, x: i64) -> i64 {
        (-x % self.p).rem_euclid(self.p) + self.p * ((-x / self.p).rem_euclid(self.p))
    }
    fn sub(&self, x: i64, y: i64) -> i64 {
        self.plus(x, self.minus(y))
    }
    fn times(&self, x: i64, y: i64) -> i64 {
        let (a, b) = (x % self.p, x / self.p);
        let (c, d) = (y % self.p, y / self.p);
        (a * c + self.d * b * d).rem_euclid(self.p) + self.p * ((a * d + b * c).rem_euclid(self.p))
    }
    fn scale(&self, n: i64, x: i64) -> i64 {
        self.times(n.rem_euclid(self.p), x)
    }
    fn inverse(&self, x: i64) -> i64 {
        assert_ne!(x, 0);
        (1..self.p * self.p)
            .find(|&y| self.times(x, y) == 1)
            .unwrap()
    }
    fn readf(&self, v: &Value) -> i64 {
        let a = v[0].as_i64().unwrap();
        let b = v[1].as_i64().unwrap();
        assert!((0..self.p).contains(&a) && (0..self.p).contains(&b));
        a + self.p * b
    }
    fn readp(&self, v: &Value) -> Pt {
        if v.is_null() {
            None
        } else {
            Some((self.readf(&v[0]), self.readf(&v[1])))
        }
    }
    fn neg(&self, p: Pt) -> Pt {
        p.map(|(x, y)| (x, self.minus(y)))
    }
    fn add(&self, p: Pt, q: Pt) -> Pt {
        let ((x, y), (u, v)) = match (p, q) {
            (None, _) => return q,
            (_, None) => return p,
            (Some(p), Some(q)) => (p, q),
        };
        if x == u && self.plus(y, v) == 0 {
            return None;
        }
        let tangent = |x: i64, y: i64| {
            (
                self.plus(self.scale(3, self.times(x, x)), self.a),
                self.scale(-2, y),
            )
        };
        let (aa, bb, cc) = if p == q {
            let (aa, bb) = tangent(x, y);
            (
                aa,
                bb,
                self.minus(self.plus(self.times(aa, x), self.times(bb, y))),
            )
        } else {
            (
                self.sub(y, v),
                self.sub(u, x),
                self.sub(self.times(x, v), self.times(u, y)),
            )
        };
        if p != q {
            for z in [p, q] {
                let (xx, yy) = z.unwrap();
                let (ax, by) = tangent(xx, yy);
                if self.sub(self.times(aa, by), self.times(bb, ax)) == 0 {
                    return self.neg(z);
                }
            }
        }
        let others: Vec<_> = self
            .points
            .iter()
            .copied()
            .filter(|&(xx, yy)| {
                Some((xx, yy)) != p
                    && Some((xx, yy)) != q
                    && self.plus(self.plus(self.times(aa, xx), self.times(bb, yy)), cc) == 0
            })
            .collect();
        if others.is_empty() {
            assert_eq!(p, q);
            self.neg(p)
        } else {
            assert_eq!(others.len(), 1);
            self.neg(Some(others[0]))
        }
    }
    fn eval(&self, a: &[i64], x: i64) -> i64 {
        a.iter()
            .rev()
            .fold(0, |z, &v| self.plus(self.times(z, x), v))
    }
    fn product(&self, a: &[i64], b: &[i64]) -> Vec<i64> {
        let mut c = vec![0; a.len() + b.len() - 1];
        for (i, &x) in a.iter().enumerate() {
            for (j, &y) in b.iter().enumerate() {
                c[i + j] = self.plus(c[i + j], self.times(x, y));
            }
        }
        c
    }
    fn accum(&self, a: &mut Vec<i64>, b: &[i64], scale: i64) {
        a.resize(a.len().max(b.len()), 0);
        for (i, &v) in b.iter().enumerate() {
            a[i] = self.plus(a[i], self.times(v, scale));
        }
    }
    fn divide_root(&self, a: &[i64], x: i64) -> Vec<i64> {
        let mut q = vec![0; a.len() - 1];
        q[a.len() - 2] = *a.last().unwrap();
        for i in (1..a.len() - 1).rev() {
            q[i - 1] = self.plus(a[i], self.times(x, q[i]))
        }
        assert_eq!(self.plus(a[0], self.times(x, q[0])), 0);
        q
    }
    fn array(&self, v: &Value) -> Vec<i64> {
        v.as_array()
            .unwrap()
            .iter()
            .map(|v| self.readf(v))
            .collect()
    }
}
type Block = (Vec<i64>, Vec<i64>, Vec<i64>);
fn merge_blocks(f: &Ref, leaves: &[Block], merges: &mut usize) -> Block {
    if leaves.len() == 1 {
        return leaves[0].clone();
    }
    let m = leaves.len() / 2;
    let (d1, a1, b1) = merge_blocks(f, &leaves[..m], merges);
    let (d2, a2, b2) = merge_blocks(f, &leaves[m..], merges);
    let d = f.product(&d1, &d2);
    let mut a = f.product(&a1, &d2);
    f.accum(&mut a, &f.product(&a2, &d1), 1);
    let mut b = f.product(&b1, &d2);
    f.accum(&mut b, &f.product(&b2, &d1), 1);
    *merges += 1;
    (d, a, b)
}
fn balanced(f: &Ref, kernel: &[Pt], image: &[Pt], shift: Pt) -> (Vec<i64>, Vec<i64>, usize) {
    let mut leaves = vec![];
    let mut seen = vec![];
    for i in 1..kernel.len() {
        let (x, y) = kernel[i].unwrap();
        if seen.contains(&x) {
            continue;
        }
        seen.push(x);
        let j = kernel.iter().position(|&v| v == f.neg(kernel[i])).unwrap();
        let up = f.add(shift, image[i]).unwrap().0;
        let um = f.add(shift, image[j]).unwrap().0;
        let q = [
            f.plus(f.times(f.a, x), f.scale(2, f.b)),
            f.plus(f.a, f.times(x, x)),
            x,
        ];
        leaves.push((
            vec![f.times(x, x), f.scale(-2, x), 1],
            q.iter().map(|&z| f.times(z, f.plus(up, um))).collect(),
            vec![f.scale(2, f.times(y, f.sub(um, up)))],
        ));
    }
    let mut merges = 0;
    let (d, part, b) = merge_blocks(f, &leaves, &mut merges);
    assert_eq!(merges, leaves.len() - 1);
    let mut a = vec![0];
    a.extend(d.iter().map(|&v| f.times(v, shift.unwrap().0)));
    f.accum(&mut a, &part, 1);
    (a, b, merges)
}
fn verify(c: &Value) -> Value {
    let mut f = Ref {
        p: c["p"].as_i64().unwrap(),
        d: c["nonsquare"].as_i64().unwrap(),
        a: c["a"].as_i64().unwrap(),
        b: c["b"].as_i64().unwrap(),
        points: vec![],
    };
    for x in 0..f.p * f.p {
        let rhs = f.plus(f.plus(f.times(f.times(x, x), x), f.times(f.a, x)), f.b);
        for y in 0..f.p * f.p {
            if f.times(y, y) == rhs {
                f.points.push((x, y))
            }
        }
    }
    let g = f.readp(&c["G"]);
    let shift = f.readp(&c["shift"]);
    let s = f.readp(&c["S"]);
    let r = c["r"].as_u64().unwrap() as usize;
    let mut kernel = vec![None];
    for i in 1..r {
        kernel.push(f.add(kernel[i - 1], g))
    }
    assert_eq!(f.add(*kernel.last().unwrap(), g), None);
    assert_eq!(
        f.points
            .iter()
            .filter(|(x, y)| *x < f.p && *y < f.p)
            .count()
            + 1,
        r
    );
    let mut roots = vec![];
    let mut fp = vec![1];
    for &(x, _) in kernel.iter().flatten() {
        if !roots.contains(&x) {
            roots.push(x);
            fp = f.product(&fp, &[f.minus(x), 1]);
        }
    }
    assert_eq!(fp, f.array(&c["kernel_polynomial"]));
    let den = f.product(&fp, &fp);
    let dp: Vec<_> = (1..fp.len()).map(|i| f.scale(i as i64, fp[i])).collect();
    let mut actions = 0;
    let mut traces = 0;
    let mut coefficients = 0;
    let mut tree_merges = 0;
    for t in c["targets"].as_array().unwrap() {
        let q = f.readp(&t["Q"]);
        let mut image = vec![None];
        for i in 1..r {
            image.push(f.add(image[i - 1], q))
        }
        assert_eq!(f.add(*image.last().unwrap(), q), None);
        let mut aa = vec![0];
        aa.extend(den.iter().map(|&z| f.times(z, shift.unwrap().0)));
        let mut bb = vec![0];
        // Sum individual signed graph terms; producer groups each +/- pair.
        for i in 1..r {
            let (x, y) = kernel[i].unwrap();
            let weight = f.add(shift, image[i]).unwrap().0;
            let quot = f.divide_root(&f.divide_root(&den, x), x);
            let term = [
                f.plus(f.times(f.a, x), f.scale(2, f.b)),
                f.plus(f.a, f.times(x, x)),
                x,
            ];
            f.accum(&mut aa, &f.product(&quot, &term), weight);
            f.accum(&mut bb, &quot, f.scale(-2, f.times(y, weight)));
        }
        assert_eq!(aa, f.array(&t["A"]));
        assert_eq!(bb, f.array(&t["B"]));
        let (tree_a, tree_b, merges) = balanced(&f, &kernel, &image, shift);
        assert_eq!(tree_a, aa);
        assert_eq!(tree_b, bb);
        tree_merges += merges;
        coefficients += aa.len() + bb.len();
        let mut probe = s;
        for rec in t["probes"].as_array().unwrap() {
            assert_eq!(probe, f.readp(&rec["P"]));
            let (px, py) = probe.unwrap();
            let mut trace = 0;
            for i in 0..r {
                trace = f.plus(
                    trace,
                    f.times(
                        f.add(probe, kernel[i]).unwrap().0,
                        f.add(shift, image[i]).unwrap().0,
                    ),
                );
            }
            assert_eq!(trace, f.readf(&rec["trace"]));
            assert_eq!(
                f.plus(f.eval(&aa, px), f.times(py, f.eval(&bb, px))),
                f.times(f.eval(&den, px), trace)
            );
            let pi = probe.map(|(x, y)| {
                (
                    f.sub(x % f.p, f.p * (x / f.p)),
                    f.sub(y % f.p, f.p * (y / f.p)),
                )
            });
            let map = f.add(probe, f.neg(pi)).unwrap().0;
            assert_eq!(map, f.readf(&rec["x_frobenius_difference"]));
            assert_eq!(
                f.times(
                    f.eval(&f.array(&c["velu_numerator"]), px),
                    f.inverse(f.eval(&den, px))
                ),
                map
            );
            probe = f.add(probe, s);
            traces += 1;
        }
        for rec in t["actions"].as_array().unwrap() {
            let u = f.readp(&rec["U"]);
            let i = kernel.iter().position(|&z| z == u).unwrap();
            let (x, y) = u.unwrap();
            let norm = f.scale(4, f.times(y, y));
            assert_eq!(norm, f.readf(&rec["raw_normalization"]));
            let norm = f.times(norm, f.times(f.eval(&dp, x), f.eval(&dp, x)));
            let value = f.times(
                f.sub(f.eval(&aa, x), f.times(y, f.eval(&bb, x))),
                f.inverse(norm),
            );
            assert_eq!(value, f.readf(&rec["pole_value"]));
            let expected = image[i];
            assert_eq!(expected, f.readp(&rec["image"]));
            assert_eq!(value, f.add(shift, expected).unwrap().0);
            let candidates: Vec<_> = f
                .points
                .iter()
                .filter(|(xx, _)| *xx == value)
                .map(|&p| f.add(Some(p), f.neg(shift)))
                .filter(|p| p.map(|(x, y)| x < f.p && y < f.p).unwrap_or(false))
                .collect();
            assert_eq!(candidates, vec![expected]);
            actions += 1;
        }
    }
    json!({"p":f.p,"r":r,"curve_points_enumerated_with_origin":f.points.len()+1,"coefficient_checks":coefficients,"balanced_numerator_reconstructions":c["targets"].as_array().unwrap().len(),"balanced_internal_merges":tree_merges,"direct_trace_checks":traces,"marked_actions":actions,"point_addition":"line intersection against complete Fp2 point list"})
}
fn main() {
    let p = std::env::args()
        .nth(1)
        .expect("usage: verify ROUND56_DIRECTORY");
    let r = Path::new(&p);
    let c: Value =
        serde_json::from_str(&fs::read_to_string(r.join("certificate.json")).unwrap()).unwrap();
    let rows: Vec<_> = c["controls"]
        .as_array()
        .unwrap()
        .iter()
        .map(verify)
        .collect();
    let full: Value = serde_json::from_str(
        &fs::read_to_string(r.join("../round55-p192/certificate.json")).unwrap(),
    )
    .unwrap();
    let mut bridge = 0;
    for target in full["targets"].as_array().unwrap() {
        let cyc = c["controls"][0]["targets"]
            .as_array()
            .unwrap()
            .iter()
            .find(|v| v["Q"][0][0] == target["Q"][0][0] && v["Q"][1][0] == target["Q"][1][0])
            .unwrap();
        for action in target["actions"].as_array().unwrap() {
            let a = cyc["actions"]
                .as_array()
                .unwrap()
                .iter()
                .find(|v| {
                    v["U"][0][0] == action["input"][0][0] && v["U"][1][0] == action["input"][1][0]
                })
                .unwrap();
            let old = &action["receipt"]["trace_principal_value"];
            assert_eq!(old[0], a["pole_value"][0]);
            assert_eq!(old[3], a["pole_value"][1]);
            for i in [1, 2, 4, 5] {
                assert_eq!(old[i], 0);
            }
            bridge += 1;
        }
    }
    assert_eq!(bridge, 36);
    println!("{}",serde_json::to_string_pretty(&json!({"all_checks_pass":true,"controls":rows,"full_to_cyclic_principal_bridge_checks":bridge,"scope":"independent packed quadratic arithmetic, line-intersection group law and unpaired polynomial reconstruction; no trusted signed scientific verdict"})).unwrap());
}
