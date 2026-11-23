#![allow(dead_code, unused_imports)]
include!("kernel.rs");
use std::collections::{BTreeMap, HashMap, HashSet};
type Mat = [u64; 4];
type Poly = [u64; 3];
fn fm(a: u64, b: u64, r: u64) -> u64 {
    ((a as u128 * b as u128) % r as u128) as u64
}
fn fa(a: u64, b: u64, r: u64) -> u64 {
    (a + b) % r
}
fn fsu(a: u64, b: u64, r: u64) -> u64 {
    (a + r - b) % r
}
fn fpow(mut x: u64, mut e: u64, r: u64) -> u64 {
    let mut z = 1;
    while e > 0 {
        if e & 1 == 1 {
            z = fm(z, x, r)
        }
        x = fm(x, x, r);
        e >>= 1
    }
    z
}
fn fi(x: u64, r: u64) -> u64 {
    assert_ne!(x, 0);
    fpow(x, r - 2, r)
}
fn mn(mut m: Mat, r: u64) -> Mat {
    let z = *m.iter().find(|&&x| x != 0).unwrap();
    let iz = fi(z, r);
    for v in &mut m {
        *v = fm(*v, iz, r)
    }
    assert_ne!(fsu(fm(m[0], m[3], r), fm(m[1], m[2], r), r), 0);
    m
}
fn mm(a: Mat, b: Mat, r: u64) -> Mat {
    mn(
        [
            fa(fm(a[0], b[0], r), fm(a[1], b[2], r), r),
            fa(fm(a[0], b[1], r), fm(a[1], b[3], r), r),
            fa(fm(a[2], b[0], r), fm(a[3], b[2], r), r),
            fa(fm(a[2], b[1], r), fm(a[3], b[3], r), r),
        ],
        r,
    )
}
fn mv(m: Mat, z: Option<u64>, r: u64) -> Option<u64> {
    let (n, d) = if let Some(z) = z {
        (fa(fm(m[0], z, r), m[1], r), fa(fm(m[2], z, r), m[3], r))
    } else {
        (m[0], m[2])
    };
    if d == 0 {
        assert_ne!(n, 0);
        None
    } else {
        Some(fm(n, fi(d, r), r))
    }
}
fn collision(a: Mat, b: Mat, r: u64) -> Poly {
    [
        fsu(fm(a[0], b[2], r), fm(b[0], a[2], r), r),
        fsu(
            fa(fm(a[0], b[3], r), fm(a[1], b[2], r), r),
            fa(fm(b[0], a[3], r), fm(b[1], a[2], r), r),
            r,
        ),
        fsu(fm(a[1], b[3], r), fm(b[1], a[3], r), r),
    ]
}
fn sqrt_mod(n: u64, r: u64) -> Option<u64> {
    if n == 0 {
        return Some(0);
    }
    if fpow(n, (r - 1) / 2, r) != 1 {
        return None;
    }
    let mut q = r - 1;
    let mut s = 0;
    while q % 2 == 0 {
        q /= 2;
        s += 1
    }
    let z = (2..r).find(|&z| fpow(z, (r - 1) / 2, r) == r - 1).unwrap();
    let mut c = fpow(z, q, r);
    let mut x = fpow(n, (q + 1) / 2, r);
    let mut t = fpow(n, q, r);
    let mut m = s;
    while t != 1 {
        let mut u = t;
        let mut i = 0;
        while u != 1 && i < m {
            u = fm(u, u, r);
            i += 1
        }
        assert!(i < m);
        let b = fpow(c, 1u64 << (m - i - 1), r);
        x = fm(x, b, r);
        let b2 = fm(b, b, r);
        t = fm(t, b2, r);
        c = b2;
        m = i
    }
    assert_eq!(fm(x, x, r), n);
    Some(x)
}
fn roots(p: Poly, r: u64) -> Vec<u64> {
    let [a, b, c] = p;
    if a == 0 {
        return if b == 0 {
            vec![]
        } else {
            vec![fm(fsu(0, c, r), fi(b, r), r)]
        };
    }
    let d = fsu(fm(b, b, r), fm(4, fm(a, c, r), r), r);
    let Some(s) = sqrt_mod(d, r) else {
        return vec![];
    };
    let inv = fi(fm(2, a, r), r);
    let mut v = vec![
        fm(fa(fsu(0, b, r), s, r), inv, r),
        fm(fsu(fsu(0, b, r), s, r), inv, r),
    ];
    v.sort();
    v.dedup();
    for &z in &v {
        assert_eq!(fa(fm(fa(fm(a, z, r), b, r), z, r), c, r), 0)
    }
    v
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Enc<const P: i64, const D: i64> {
    Finite(Pt<P, D>),
    Infinity,
}
fn ek<const P: i64, const D: i64>(w: Enc<P, D>) -> (u8, u8, i64, i64, i64, i64) {
    match w {
        Enc::Infinity => (1, 0, 0, 0, 0, 0),
        Enc::Finite(None) => (0, 0, 0, 0, 0, 0),
        Enc::Finite(Some((x, y))) => (0, 1, x.0, x.1, y.0, y.1),
    }
}
fn ec<const P: i64, const D: i64>(w: Enc<P, D>) -> Value {
    match w {
        Enc::Infinity => json!({"infinity":true}),
        Enc::Finite(p) => json!({"finite":pc(p)}),
    }
}
fn se<const P: i64, const D: i64>(pts: &[Pt<P, D>], z: Option<u64>) -> Enc<P, D> {
    z.map(|z| Enc::Finite(pts[z as usize]))
        .unwrap_or(Enc::Infinity)
}
struct Ctx<const P: i64, const D: i64> {
    a: i64,
    b: i64,
    r: u64,
    g: Pt<P, D>,
    shift: Pt<P, D>,
    gg: Vec<Pt<P, D>>,
    norms: u64,
    canon: u64,
    public_charts: u64,
}
fn nt<const P: i64, const D: i64>(ctx: &mut Ctx<P, D>, w: Enc<P, D>) -> Enc<P, D> {
    let q = match w {
        Enc::Infinity => {
            ctx.public_charts += 1;
            return Enc::Finite(None);
        }
        Enc::Finite(q) => q,
    };
    let out = if q.is_none() {
        Some(Enc::Finite(ctx.g))
    } else if q == ctx.g {
        Some(Enc::Infinity)
    } else if q == neg(ctx.g) {
        Some(Enc::Finite(mul((ctx.r + 1) / 2, ctx.g, ctx.a)))
    } else if q == mul(2, ctx.g, ctx.a) {
        Some(Enc::Finite(neg(ctx.g)))
    } else if add(q, q, ctx.a) == ctx.g {
        Some(Enc::Finite(mul(2, ctx.g, ctx.a)))
    } else {
        None
    };
    if let Some(z) = out {
        ctx.public_charts += 1;
        return z;
    }
    ctx.norms += 1;
    let qq = graph(q, ctx.a, ctx.r);
    let (z, _) = action(&ctx.gg, &qq, ctx.shift, ctx.g, ctx.a, ctx.b, ctx.r, false);
    Enc::Finite(z)
}
fn rt<const P: i64, const D: i64>(pts: &[Pt<P, D>], w: Enc<P, D>, r: u64) -> Enc<P, D> {
    let z = match w {
        Enc::Infinity => None,
        Enc::Finite(q) => Some(pts.iter().position(|&p| p == q).unwrap() as u64),
    };
    let t = [0, 1, r - 1, 1];
    se(pts, mv(t, z, r))
}
#[derive(Clone)]
struct Can<const P: i64, const D: i64> {
    rep: Enc<P, D>,
    h: Mat,
    size: usize,
    images: Vec<(Enc<P, D>, Mat)>,
}
fn canon<const P: i64, const D: i64>(
    ctx: &mut Ctx<P, D>,
    w: Enc<P, D>,
    pts: &[Pt<P, D>],
    reference: bool,
) -> Can<P, D> {
    ctx.canon += 1;
    let r = ctx.r;
    let t = [0, 1, r - 1, 1];
    let s = [r - 1, 1, 0, 1];
    let id = [1, 0, 0, 1];
    let w1 = if reference { rt(pts, w, r) } else { nt(ctx, w) };
    let w2 = if w1 == w {
        w1
    } else if reference {
        rt(pts, w1, r)
    } else {
        nt(ctx, w1)
    };
    let comp = |v| match v {
        Enc::Infinity => Enc::Infinity,
        Enc::Finite(q) => Enc::Finite(add(ctx.g, neg(q), ctx.a)),
    };
    let mut images = vec![
        (w, id),
        (w1, mn(t, r)),
        (w2, mm(t, t, r)),
        (comp(w), mn(s, r)),
        (comp(w1), mm(s, t, r)),
        (comp(w2), mm(s, mm(t, t, r), r)),
    ];
    for (_, h) in &mut images {
        *h = mn(*h, r)
    }
    let (rep, h) = *images.iter().min_by_key(|(p, m)| (ek(*p), *m)).unwrap();
    let size = images
        .iter()
        .map(|(p, _)| ek(*p))
        .collect::<HashSet<_>>()
        .len();
    Can {
        rep,
        h,
        size,
        images,
    }
}
fn recover<const P: i64, const D: i64>(ctx: &Ctx<P, D>, p: Poly, target: Pt<P, D>) -> Option<u64> {
    if p == [0, 0, 0] {
        return None;
    }
    let hits: Vec<_> = roots(p, ctx.r)
        .into_iter()
        .filter(|&z| mul(z, ctx.g, ctx.a) == target)
        .collect();
    assert!(hits.len() <= 1);
    hits.first().copied()
}
fn shortcut<const P: i64, const D: i64>(
    ctx: &Ctx<P, D>,
    c: &Can<P, D>,
    m: Mat,
) -> Option<(Poly, &'static str)> {
    let [a, b, cc, d] = m;
    let r = ctx.r;
    if c.rep == Enc::Infinity {
        return Some(([0, cc, d], "projective_pole"));
    }
    for v in [0, 1, r - 1, 2, (r + 1) / 2] {
        if c.rep == Enc::Finite(mul(v, ctx.g, ctx.a)) {
            return Some((
                [0, fsu(a, fm(v, cc, r), r), fsu(b, fm(v, d, r), r)],
                "known_orbit_value",
            ));
        }
    }
    if c.size == 2 {
        return Some((
            [
                fa(fsu(fm(a, a, r), fm(a, cc, r), r), fm(cc, cc, r), r),
                fa(
                    fsu(fm(2, fm(a, b, r), r), fa(fm(a, d, r), fm(b, cc, r), r), r),
                    fm(2, fm(cc, d, r), r),
                    r,
                ),
                fa(fsu(fm(b, b, r), fm(b, d, r), r), fm(d, d, r), r),
            ],
            "order3_stabilizer",
        ));
    }
    None
}
fn hash_key(k: (u8, u8, i64, i64, i64, i64), restart: u64) -> u64 {
    let mut h = 14695981039346656037u64;
    for v in [
        k.0 as u64, k.1 as u64, k.2 as u64, k.3 as u64, k.4 as u64, k.5 as u64, restart,
    ] {
        for b in v.to_le_bytes() {
            h ^= b as u64;
            h = h.wrapping_mul(1099511628211)
        }
    }
    h
}
fn walk<const P: i64, const D: i64>(
    ctx: &mut Ctx<P, D>,
    target: Pt<P, D>,
    pts: &[Pt<P, D>],
    reference: bool,
) -> Value {
    let mut attempts = vec![];
    let r = ctx.r;
    for (restart, offset) in [0u64, 1, 3, 7].into_iter().enumerate() {
        let mut w = Enc::Finite(add(target, mul(offset, ctx.g, ctx.a), ctx.a));
        let mut m = mn([1, offset % r, 0, 1], r);
        let mut seen = HashMap::new();
        let mut trace = vec![];
        let mut reason = "step_cap";
        for step in 0..r + 2 {
            let c = canon(ctx, w, pts, reference);
            m = mm(c.h, m, r);
            let mut eq = shortcut(ctx, &c, m);
            let mut prior = None;
            if eq.is_none() {
                if let Some(&(old, old_step)) = seen.get(&ek(c.rep)) {
                    let p = collision(old, m, r);
                    prior = Some(old_step);
                    if p == [0, 0, 0] {
                        reason = "tautological_collision";
                        trace.push(json!({"step":step,"point":ec(c.rep),"M":m,"status":reason,"prior_step":old_step}));
                        break;
                    } else {
                        eq = Some((p, "nontrivial_collision"))
                    }
                }
            }
            if let Some((p, why)) = eq {
                let k = recover(ctx, p, target).expect("true encoded equation must recover target");
                let mut bad = p;
                bad[2] = fa(bad[2], 1, r);
                assert!(recover(ctx, bad, target).is_none());
                trace.push(json!({"step":step,"point":ec(c.rep),"M":m,"status":why,"polynomial":p,"field_roots":roots(p,r),"prior_step":prior,"verified_scalar":k,"changed_polynomial_rejected":true}));
                attempts.push(
                    json!({"restart":restart,"offset":offset,"status":"recovered","trace":trace}),
                );
                return json!({"status":"recovered","scalar":k,"reason":why,"attempts":attempts});
            }
            seen.insert(ek(c.rep), (m, step));
            let choice = ((hash_key(ek(c.rep), restart as u64) >> 32) & 3) as usize;
            let (aa, bb) = [(1, 1), (2, 3), (3, 5), (5, 7)][choice];
            let mmove = mn([aa % r, bb % r, 0, 1], r);
            let q = match c.rep {
                Enc::Finite(q) => q,
                Enc::Infinity => panic!("infinity must shortcut"),
            };
            w = Enc::Finite(add(mul(aa, q, ctx.a), mul(bb, ctx.g, ctx.a), ctx.a));
            trace.push(
                json!({"step":step,"point":ec(c.rep),"M":m,"orbit_size":c.size,"move":[aa,bb]}),
            );
            m = mm(mmove, m, r);
        }
        attempts.push(json!({"restart":restart,"offset":offset,"status":reason,"trace":trace}));
    }
    json!({"status":"unresolved_within_frozen_restarts","attempts":attempts})
}
fn panel<const P: i64, const D: i64>(
    a: i64,
    b: i64,
    r: u64,
    g: Pt<P, D>,
    shift: Pt<P, D>,
    targets: &[u64],
    reference: bool,
) -> (Value, Value) {
    let on = |p: Pt<P, D>| {
        p.map(|(x, y)| y * y == x * x * x + F::c(a) * x + F::c(b))
            .unwrap_or(true)
    };
    assert!(on(g) && on(shift));
    assert_ne!(mul(2, shift, a).unwrap().1 .1, 0);
    let count = 1
        + (0..P)
            .flat_map(|x| (0..P).map(move |y| (x, y)))
            .filter(|&(x, y)| (y * y - x * x * x - a * x - b).rem_euclid(P) == 0)
            .count();
    assert_eq!(count, r as usize);
    let pts = graph(g, a, r);
    let mut ctx = Ctx {
        a,
        b,
        r,
        g,
        shift,
        gg: pts.clone(),
        norms: 0,
        canon: 0,
        public_charts: 0,
    };
    let t = [0, 1, r - 1, 1];
    let s = [r - 1, 1, 0, 1];
    let id = [1, 0, 0, 1];
    assert_eq!(mm(s, s, r), id);
    assert_eq!(mm(mm(t, t, r), t, r), id);
    assert_eq!(mm(mm(s, t, r), s, r), mm(t, t, r));
    let mut all = Vec::new();
    let mut by_key = HashMap::new();
    let mut buckets: BTreeMap<_, Vec<Option<u64>>> = BTreeMap::new();
    let mut checks = 0;
    for z in (0..r).map(Some).chain(std::iter::once(None)) {
        let w = se(&pts, z);
        let c = canon(&mut ctx, w, &pts, reference);
        for &(image, h) in &c.images {
            assert_eq!(image, se(&pts, mv(h, z, r)));
            checks += 1
        }
        buckets.entry(ek(c.rep)).or_default().push(z);
        by_key.insert(ek(w), c.clone());
        all.push(json!({"input_scalar_fixture":z,"input":ec(w),"representative":ec(c.rep),"H":c.h,"orbit_size":c.size,"images":c.images.iter().map(|(p,h)|json!({"point":ec(*p),"H":h})).collect::<Vec<_>>()}));
    }
    let mut free_checks = 0;
    for z in (0..r).map(Some).chain(std::iter::once(None)) {
        let c = &by_key[&ek(se(&pts, z))];
        let cs = &by_key[&ek(se(&pts, mv(s, z, r)))];
        let ct = &by_key[&ek(se(&pts, mv(t, z, r)))];
        assert_eq!(c.rep, cs.rep);
        assert_eq!(c.rep, ct.rep);
        if c.size == 6 {
            assert_eq!(c.h, mm(cs.h, s, r));
            assert_eq!(c.h, mm(ct.h, t, r));
            assert_eq!(collision(c.h, mm(cs.h, s, r), r), [0, 0, 0]);
            free_checks += 2
        }
    }
    let mut hist = BTreeMap::new();
    for members in buckets.values() {
        *hist.entry(members.len()).or_insert(0usize) += 1
    }
    assert_eq!(hist.get(&3), Some(&2));
    let fixed = (0..r)
        .filter(|&z| fa(fsu(fm(z, z, r), z, r), 1, r) == 0)
        .count();
    assert_eq!(hist.get(&2).copied().unwrap_or(0), fixed / 2);
    assert_eq!(
        hist.get(&6).copied().unwrap_or(0),
        (r as usize + 1 - 6 - fixed) / 6
    );
    assert!(recover(&ctx, [0, 0, 0], g).is_none());
    let mut runs = vec![];
    let mut recovered = 0;
    let mut collision_recoveries = 0;
    for &k in targets {
        let target = pts[k as usize];
        let v = walk(&mut ctx, target, &pts, reference);
        if v["status"] == "recovered" {
            assert_eq!(v["scalar"], k);
            recovered += 1;
            if v["reason"] == "nontrivial_collision" {
                collision_recoveries += 1
            }
        }
        runs.push(json!({"target_fixture":k,"Q":pc(target),"result":v}));
    }
    let result = json!({"p":P,"D":D,"a":a,"b":b,"r":r,"G":pc(g),"shift":pc(shift),"all_projective_states":r+1,"matrix_transform_checks":checks,"free_orbit_transporter_tautology_checks":free_checks,"orbit_histogram":hist.iter().map(|(&size,&count)|json!({"size":size,"orbits":count})).collect::<Vec<_>>(),"order3_scalar_roots":(0..r).filter(|&z|fa(fsu(fm(z,z,r),z,r),1,r)==0).collect::<Vec<_>>(),"canonicalizations":all,"walks":runs,"walk_targets":targets.len(),"recovered_targets":recovered,"nontrivial_collision_recoveries":collision_recoveries,"tautology_control_rejected":true});
    let receipt = json!({"p":P,"r":r,"backend":if reference{"scalar_reference_only"}else{"full_transverse_norm"},"norm_calls":ctx.norms,"target_constructors":ctx.norms,"norm_factor_terms":ctx.norms*r,"canonicalizer_calls":ctx.canon,"public_T_charts":ctx.public_charts,"target_independent_G_graph_points":r,"walk_restart_cap":4,"step_cap_per_restart":r+2});
    (result, receipt)
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let reference = args.iter().any(|s| s == "--scalar-reference");
    if args.iter().any(|s| s == "--large-residue-control") {
        let mut selection = vec![];
        let mut fixture = None;
        for b in 1i64..211 {
            if (-108 + 27 * b * b).rem_euclid(211) == 0 {
                selection.push(json!({"b":b,"status":"singular_skipped"}));
                continue;
            }
            let r = (1
                + (0i64..211)
                    .flat_map(|x| (0i64..211).map(move |y| (x, y)))
                    .filter(|&(x, y)| (y * y - x * x * x + 3 * x - b).rem_euclid(211) == 0)
                    .count()) as u64;
            let prime = r >= 2 && (2..).take_while(|d| d * d <= r).all(|d| r % d != 0);
            let eligible = prime && r >= 100 && r % 3 == 2;
            selection.push(json!({"b":b,"order":r,"prime":prime,"eligible":eligible}));
            if eligible {
                fixture = Some((b, r));
                break;
            }
        }
        let (b, r) = fixture.expect("bounded fixture scan exhausted");
        let mut g = None;
        let mut shift = None;
        for x in 0i64..211 {
            let rhs = (x * x * x - 3 * x + b).rem_euclid(211);
            if g.is_none() {
                if let Some(y) = (1i64..211).find(|&y| y * y % 211 == rhs) {
                    g = Some((F::<211, 210>::c(x), F::c(y)))
                }
            }
            if shift.is_none() {
                if let Some(y) = (1i64..211).find(|&y| y * y % 211 == (-rhs).rem_euclid(211)) {
                    let s = Some((F::c(x), F::new(0, y)));
                    if mul(2, s, -3).map(|(_, y)| y.1 != 0).unwrap_or(false) {
                        shift = s
                    }
                }
            }
            if g.is_some() && shift.is_some() {
                break;
            }
        }
        assert!(g.is_some() && shift.is_some());
        let targets = [1, 2, 3, 5, 7, 11, 17, 31, 63, 97, r - 1];
        let (p, receipt) = panel::<211, 210>(-3, b, r, g, shift, &targets, reference);
        let out = json!({"all_checks_pass":true,"fixture_selection":selection,"panel":p,"scope":"amended larger scalar residue2mod3 control; fixture selected before recovery outcomes"});
        if let Some(i) = args.iter().position(|x| x == "--check") {
            let expected: Value = serde_json::from_slice(&fs::read(&args[i + 1]).unwrap()).unwrap();
            assert!(out == expected, "certificate mismatch")
        }
        eprintln!("{}", receipt);
        println!("{}", serde_json::to_string(&out).unwrap());
        return;
    }
    if args.iter().any(|s| s == "--residue-control") {
        let (p, receipt) = panel::<11, 2>(
            8,
            1,
            17,
            Some((F::c(0), F::c(1))),
            Some((F::c(1), F::new(0, 4))),
            &(1..17).collect::<Vec<_>>(),
            reference,
        );
        let out = json!({"all_checks_pass":true,"panel":p,"scope":"amended no-order3-stabilizer control; no timing or mixing claim"});
        if let Some(i) = args.iter().position(|x| x == "--check") {
            let expected: Value = serde_json::from_slice(&fs::read(&args[i + 1]).unwrap()).unwrap();
            assert!(out == expected, "certificate mismatch")
        }
        eprintln!("{}", receipt);
        println!("{}", serde_json::to_string(&out).unwrap());
        return;
    }
    let sa = Some((F::<11, 2>::c(0), F::new(0, 3)));
    let sb = Some((F::<17, 3>::c(1), F::new(0, 8)));
    let rt = (1..211).find(|x| x * x % 211 == 204).unwrap();
    let sc = Some((F::<211, 210>::c(0), F::new(0, rt)));
    let (a, ar) = panel::<11, 2>(
        2,
        7,
        7,
        Some((F::c(6), F::c(2))),
        mul(6, sa, 2),
        &(1..7).collect::<Vec<_>>(),
        reference,
    );
    let (b, br) = panel::<17, 3>(
        2,
        2,
        19,
        Some((F::c(5), F::c(1))),
        mul(6, sb, 2),
        &(1..19).collect::<Vec<_>>(),
        reference,
    );
    let (c, cr) = panel::<211, 210>(
        0,
        7,
        199,
        Some((F::c(3), F::c(33))),
        sc,
        &[1, 2, 3, 5, 7, 11, 17, 31, 63, 97, 128, 198],
        reference,
    );
    let out = json!({"all_checks_pass":true,"panels":[a,b,c],"scope":"exact Mobius orbit and bounded-walk qualification; no timing/mixing claim or full P192 norm"});
    if let Some(i) = args.iter().position(|x| x == "--check") {
        let v: Value = serde_json::from_slice(&fs::read(&args[i + 1]).unwrap()).unwrap();
        assert!(out == v, "certificate mismatch");
    }
    eprintln!("{}", json!({"backend_receipts":[ar,br,cr]}));
    println!("{}", serde_json::to_string(&out).unwrap());
}
