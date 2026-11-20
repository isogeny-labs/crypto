mod algebra;
#[path = "../triangular-witness/core.rs"]
mod core;
#[path = "../triangular-witness/cost.rs"]
mod cost;
#[path = "../triangular-witness/geometry.rs"]
mod geometry;
#[path = "../canonical-root/root.rs"]
mod root;
use hec_native_ic::{
    counted,
    fp::Fp,
    instance::factor_base,
    jac::{Curve, Div},
    poly::Poly,
    rng::Rng,
};
#[derive(Clone)]
struct Context {
    c: Curve,
    d: Div,
    target: Div,
    ell: u64,
    sm: u64,
}
#[derive(Clone)]
struct Packet {
    z: u64,
    reps: Vec<(u64, u64)>,
    primitives: Vec<core::Primitive>,
    branches: Vec<(u64, Vec<u64>)>,
    k: u64,
}
fn rows(c: &Context, p: &Packet, r: u64) -> Option<Vec<Vec<u64>>> {
    let f = Fp::new(c.ell);
    let n = p.reps.len() + 2;
    let mut index = std::collections::BTreeMap::new();
    for (j, &(x, y)) in p.reps.iter().enumerate() {
        let (mut xx, mut w) = (x, 1);
        for _ in 0..5 {
            if index.insert((xx, y), (j, w)).is_some() {
                return None;
            }
            let ny = c.c.fp.neg(y);
            if ny != y && index.insert((xx, ny), (j, f.neg(w))).is_some() {
                return None;
            }
            xx = c.c.fp.mul(xx, p.z);
            w = f.mul(w, r);
        }
    }
    let mut public = vec![0; n];
    public[0] = 1;
    public[n - 1] = f.neg(f.inv(c.sm));
    let mut out = vec![public];
    for prim in &p.primitives {
        let mut v = vec![0; n];
        v[n - 2] = 1;
        v[n - 1] = prim.a;
        for xy in &prim.points {
            let &(j, w) = index.get(xy)?;
            v[j] = f.add(v[j], w);
        }
        out.push(v);
    }
    Some(out)
}
fn verify(c: &Context, p: &Packet) -> bool {
    let cv = &c.c;
    let ell = match (cv.fp.p, cv.f.c[0]) {
        (101, 2) => 7481,
        (251, 18) => 67931,
        (601, 7) => 411491,
        _ => return false,
    };
    if c.ell != ell
        || cv.g != 2
        || cv.f != Poly::from_coeffs(&[cv.f.c[0], 0, 0, 0, 0, 1])
        || c.sm == 0
        || c.sm >= ell
        || p.k >= ell
        || p.z <= 1
        || p.z >= cv.fp.p
        || cv.fp.pow(p.z, 5) != 1
        || p.reps.len() < 2
        || p.primitives.is_empty()
        || p.branches.len() != 4
    {
        return false;
    }
    if !cv.is_valid(&c.d) || !cv.is_valid(&c.target) || cv.is_zero(&c.d) {
        return false;
    }
    for &(x, y) in &p.reps {
        if x == 0 || x >= cv.fp.p || y >= cv.fp.p || cv.fp.mul(y, y) != cv.f.eval(x, &cv.fp) {
            return false;
        }
    }
    if cv.mul(&cv.place(p.reps[0].0, p.reps[0].1), c.sm) != c.d {
        return false;
    }
    let f = Fp::new(ell);
    let mut roots = std::collections::BTreeSet::new();
    let mut prod = Poly::one();
    for (r, _) in &p.branches {
        if *r >= ell || !roots.insert(*r) {
            return false;
        }
        prod = prod.mul(&Poly::x_minus(*r, &f), &f);
    }
    if prod != Poly::from_coeffs(&[1; 5]) {
        return false;
    }
    let maxa = p.primitives.iter().map(|a| a.a).max().unwrap();
    if maxa == 0 || maxa > 16 {
        return false;
    }
    let mut anchors = vec![c.target];
    for _ in 0..maxa {
        anchors.push(cv.add(anchors.last().unwrap(), &c.d));
    }
    for prim in &p.primitives {
        if prim.a == 0
            || prim.t >= cv.fp.p
            || !geometry::exact(cv, &anchors[prim.a as usize], prim.t, &prim.points)
        {
            return false;
        }
    }
    let n = p.reps.len() + 2;
    let mut want = vec![0; n];
    want[n - 2] = 1;
    want[n - 1] = f.neg(p.k);
    for (r, w) in &p.branches {
        if w.len() != p.primitives.len() + 1 || w.iter().any(|&x| x >= ell) {
            return false;
        }
        let Some(a) = rows(c, p, *r) else {
            return false;
        };
        if !algebra::verify(&a, &want, w, &f) {
            return false;
        }
    }
    true
}
fn main() {
    counted::disable();
    let (mut cells, mut bads) = (0, 0);
    for (p, cc, ell) in [(101, 2, 7481), (251, 18, 67931), (601, 7, 411491)] {
        let cv = Curve::new(Fp::new(p), Poly::from_coeffs(&[cc, 0, 0, 0, 0, 1]));
        let pt = factor_base(&cv)[0];
        let base = cv.place(pt.0, pt.1);
        let mut rng = Rng::new(2026113000 ^ p);
        for sm in [1, 17] {
            let d = cv.mul(&base, sm);
            for case in 0..2 {
                let k = 1 + rng.below(ell - 1);
                let target = cv.mul(&d, k);
                let c = Context {
                    c: cv.clone(),
                    d,
                    target,
                    ell,
                    sm,
                };
                let tr = (1..=3)
                    .map(|seed| core::solve(&cv, pt, &d, &target, ell, sm, seed, true, true))
                    .find(|t| t.answer.is_some())
                    .unwrap();
                let answer = tr.answer.unwrap().1;
                assert_eq!(answer, k);
                let mut packet = Packet {
                    z: tr.z,
                    reps: tr.points,
                    primitives: tr.primitives,
                    branches: vec![],
                    k: answer,
                };
                let f = Fp::new(ell);
                let z = root::power_map(&f, 5).0;
                let n = packet.reps.len() + 2;
                let mut want = vec![0; n];
                want[n - 2] = 1;
                want[n - 1] = f.neg(answer);
                counted::reset();
                for exponent in 1..5 {
                    let r = f.pow(z, exponent);
                    let a = rows(&c, &packet, r).unwrap();
                    let w = algebra::witness(&a, &want, &f).unwrap();
                    packet.branches.push((r, w));
                }
                let construction = counted::snapshot();
                counted::reset();
                assert!(verify(&c, &packet));
                let verification = counted::snapshot();
                counted::disable();
                assert_eq!(cv.mul(&d, packet.k), target);
                let mut mutations = vec![];
                let mut q = packet.clone();
                q.k = f.add(q.k, 1);
                mutations.push(q);
                let mut q = packet.clone();
                q.z = 1;
                mutations.push(q);
                let mut q = packet.clone();
                q.branches.pop();
                mutations.push(q);
                let mut q = packet.clone();
                q.branches[1].0 = q.branches[0].0;
                mutations.push(q);
                let mut q = packet.clone();
                q.reps.push(q.reps[0]);
                mutations.push(q);
                let mut q = packet.clone();
                q.reps[0].1 = cv.fp.add(q.reps[0].1, 1);
                mutations.push(q);
                let mut q = packet.clone();
                q.primitives[0].t = cv.fp.add(q.primitives[0].t, 1);
                mutations.push(q);
                let mut q = packet.clone();
                q.primitives[0].a = 0;
                mutations.push(q);
                let mut q = packet.clone();
                q.primitives[0].points.pop();
                mutations.push(q);
                for i in 0..4 {
                    let mut q = packet.clone();
                    q.branches[i].1[0] = f.add(q.branches[i].1[0], 1);
                    mutations.push(q);
                }
                let mut local = 0;
                for q in mutations {
                    assert!(!verify(&c, &q));
                    local += 1;
                }
                let mut wrong = c.clone();
                wrong.target = cv.add(&target, &d);
                assert!(!verify(&wrong, &packet));
                local += 1;
                let mut wrong = c.clone();
                wrong.d = cv.neg(&d);
                assert!(!verify(&wrong, &packet));
                local += 1;
                let mut wrong = c.clone();
                wrong.sm = if sm == 1 { 17 } else { 1 };
                assert!(!verify(&wrong, &packet));
                local += 1;
                let mut wrong = c.clone();
                wrong.ell += 2;
                assert!(!verify(&wrong, &packet));
                local += 1;
                bads += local;
                cells += 1;
                println!("CELL\tp={p}\tsm={sm}\tcase={case}\trows={}\torbits={}\tmutations={local}\tconstruction={construction:?}\tverification={verification:?}",packet.primitives.len(),packet.reps.len());
            }
        }
    }
    assert_eq!(cells, 12);
    assert_eq!(bads, 204);
    eprintln!("PASS packets=12 homogeneous_components=48 mutations_rejected={bads} native_scalar_checks=12 independent_verification=false total_pipeline_cost_claim=false");
}
