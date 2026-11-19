//! Prime31 public-constraint *construction* for signed eight-slot powers-of-two W8.
//! Plant-only witness controls. NOT a public-only index extractor or ECDLP speedup.
//! Frozen protocol: ../rr9/HORNER_XY_CHAIN_PROTOCOL_20261119.md
//! Run with the original prime31-algebraic-ic Rust arithmetic crate.
use prime31_algebraic_ic::{check_curve, Arithmetic, Point, PRIME31};
use serde_json::{json, Value};

const P: u64 = 2_147_483_647;
const G: Point = Some((625_037_715, 2_100_627_251));
const SEED: u64 = 0x4858_595f_4348_4149;
const WINDOWS: [[usize; 8]; 4] = [
    [0, 1, 2, 3, 4, 5, 6, 7],
    [8, 9, 10, 11, 12, 13, 14, 15],
    [16, 17, 18, 19, 20, 21, 22, 23],
    [23, 24, 25, 26, 27, 28, 29, 30],
];
type Aff = (u64, u64);

#[inline] fn a(x: u64, y: u64) -> u64 { (x + y) % P }
#[inline] fn s(x: u64, y: u64) -> u64 { (x + P - y) % P }
#[inline] fn m(x: u64, y: u64) -> u64 { ((x as u128 * y as u128) % P as u128) as u64 }
fn pow(mut x: u64, mut n: u64) -> u64 {
    let mut out = 1;
    while n > 0 {
        if n & 1 == 1 { out = m(out, x); }
        x = m(x, x);
        n >>= 1;
    }
    out
}
fn inv(x: u64) -> u64 { assert_ne!(x, 0); pow(x, P - 2) }
fn rhs(x: u64) -> u64 { a(a(m(m(x, x), x), m(PRIME31.a, x)), PRIME31.b) }

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}
fn base31(ar: &mut Arithmetic) -> Vec<Aff> {
    let mut b = Vec::new();
    for x in 0..10_000 {
        let y2 = ar.rhs(PRIME31, x);
        if let Some(y) = ar.sqrt(y2, P) {
            if y != 0 {
                b.push((x, y.min(P - y)));
                if b.len() == 31 { break; }
            }
        }
    }
    assert_eq!(b.len(), 31);
    assert_eq!(b.iter().take(8).map(|&(x, _)| x).collect::<Vec<_>>(),
        vec![0, 1, 2, 3, 5, 6, 8, 9]);
    b
}

fn leaves_from_seed(state: &mut u64, base: &[Aff], win: &[usize; 8]) -> [Aff; 8] {
    std::array::from_fn(|_| {
        let r = splitmix64(state);
        let (x, y) = base[win[(r & 7) as usize]];
        (x, if r & 8 == 0 { y } else { (P-y) % P })
    })
}
fn signed_base_root(x: u64, base: &[Aff], win: &[usize; 8]) -> u64 {
    win.iter().fold(1u64, |acc, &i| m(acc, s(x, base[i].0)))
}
fn direct_weighted(ar: &mut Arithmetic, leaves: &[Aff; 8]) -> Point {
    let mut sum = None;
    for (i, &p) in leaves.iter().enumerate() {
        let term = ar.scalar(PRIME31, 1u64 << i, Some(p));
        sum = ar.add(PRIME31, sum, term);
    }
    sum
}
fn horner_group(ar: &mut Arithmetic, leaves: &[Aff]) -> Point {
    assert!(!leaves.is_empty());
    let mut sum = Some(leaves[leaves.len()-1]);
    for &p in leaves[..leaves.len()-1].iter().rev() {
        sum = ar.add(PRIME31, sum, sum);
        sum = ar.add(PRIME31, Some(p), sum);
    }
    sum
}
fn split_group(ar: &mut Arithmetic, leaves: &[Aff; 8]) -> Point {
    let u = horner_group(ar, &leaves[..4]);
    let mut v = horner_group(ar, &leaves[4..]);
    for _ in 0..4 { v = ar.add(PRIME31, v, v); }
    ar.add(PRIME31, u, v)
}

#[derive(Clone, Copy, Debug)]
struct Step {
    tail: Aff,
    leaf: Aff,
    doubled: Aff,
    next: Aff,
    lambda: u64,
    rho: u64,
    mu: u64,
    sigma: u64,
}
impl Step {
    // No group operations here: derive rational slope data from the ordinary
    // affine chart, then *separately* verify polynomial identities below.
    fn create(tail: Aff, leaf: Aff) -> Option<Self> {
        let (x, y) = tail;
        let den_d = a(y, y);
        if den_d == 0 { return None; }
        let lambda = m(a(m(3, m(x, x)), PRIME31.a), inv(den_d));
        let doubled = (s(m(lambda, lambda), a(x, x)), 0);
        let doubled = (doubled.0, s(m(lambda, s(x, doubled.0)), y));
        let den_a = s(leaf.0, doubled.0);
        if den_a == 0 { return None; }
        let mu = m(s(leaf.1, doubled.1), inv(den_a));
        let nx = s(s(m(mu, mu), doubled.0), leaf.0);
        let ny = s(m(mu, s(doubled.0, nx)), doubled.1);
        Some(Self {
            tail, leaf, doubled, next: (nx, ny),
            lambda, rho: inv(den_d), mu, sigma: inv(den_a),
        })
    }
    fn residuals(&self) -> [u64; 8] {
        let (x,y)=self.tail;
        let (xi,yi)=self.leaf;
        let (xd,yd)=self.doubled;
        let (xn,yn)=self.next;
        let dd=a(y,y);
        let da=s(xi,xd);
        [
            s(m(dd,self.lambda),a(m(3,m(x,x)),PRIME31.a)),
            s(m(dd,self.rho),1),
            s(xd,s(m(self.lambda,self.lambda),a(x,x))),
            s(yd,s(m(self.lambda,s(x,xd)),y)),
            s(m(da,self.mu),s(yi,yd)),
            s(m(da,self.sigma),1),
            s(xn,s(s(m(self.mu,self.mu),xd),xi)),
            s(yn,s(m(self.mu,s(xd,xn)),yd)),
        ]
    }
}
fn polynomial_chain(leaves: &[Aff; 8]) -> Option<(Aff, Vec<Step>)> {
    let mut tail = leaves[7];
    let mut steps = Vec::with_capacity(7);
    for &leaf in leaves[..7].iter().rev() {
        let step = Step::create(tail, leaf)?;
        // Each ordinary affine formula must have an actual nonzero inverse;
        // the inverse-witness constraints prevent extraneous saturated roots.
        assert_eq!(step.residuals(), [0;8]);
        assert_eq!(m(step.next.1,step.next.1),rhs(step.next.0));
        tail = step.next;
        steps.push(step);
    }
    Some((tail, steps))
}
fn run(cases_per_window: usize) -> Value {
    assert!((1..=64).contains(&cases_per_window));
    let mut setup = Arithmetic::default();
    check_curve(PRIME31, &mut setup).expect("full-prime-order original curve");
    let base = base31(&mut setup);
    let mut state = SEED;
    let mut tested=0u64;
    let mut ordinary=0u64;
    let mut exceptional=0u64;
    let mut max_residual=0u64;
    let mut mutated_slope_rejected=0u64;
    let mut mutated_target_rejected=0u64;
    let mut naive_group=0u64;
    let mut horner_group_count=0u64;
    let mut split_group_count=0u64;
    let mut certificates=Vec::new();
    let mut per_window=Vec::new();
    for (wi, win) in WINDOWS.iter().enumerate() {
        let mut window_ordinary=0u64;
        let mut window_exceptional=0u64;
        for ci in 0..cases_per_window {
            tested+=1;
            let leaves=leaves_from_seed(&mut state,&base,win);
            assert!(leaves.iter().all(|&(x,y)| {
                signed_base_root(x,&base,win)==0 && m(y,y)==rhs(x)
            }));
            let mut naive_ar=Arithmetic::default();
            let target=direct_weighted(&mut naive_ar,&leaves);
            naive_group+=naive_ar.counts.group_additions;
            let mut horner_ar=Arithmetic::default();
            let h=horner_group(&mut horner_ar,&leaves);
            horner_group_count+=horner_ar.counts.group_additions;
            assert_eq!(h,target);
            let mut split_ar=Arithmetic::default();
            let split=split_group(&mut split_ar,&leaves);
            split_group_count+=split_ar.counts.group_additions;
            assert_eq!(split,target);
            match polynomial_chain(&leaves) {
                None => { exceptional+=1; window_exceptional+=1; },
                Some((terminal,steps)) => {
                    ordinary+=1; window_ordinary+=1;
                    assert_eq!(steps.len(),7);
                    assert_eq!(Some(terminal),target,"wrong final polynomial point");
                    let mut group_verifier=Arithmetic::default();
                    for step in &steps {
                        assert_eq!(
                            Some(step.next),
                            group_verifier.add(PRIME31,
                                Some(step.leaf),
                                group_verifier.add(PRIME31,Some(step.tail),Some(step.tail))
                            )
                        );
                        for v in step.residuals() {
                            max_residual=max_residual.max(v);
                            assert_eq!(v,0);
                        }
                    }
                    let mut changed=steps[0];
                    changed.lambda=a(changed.lambda,1);
                    assert_ne!(changed.residuals(),[0;8]);
                    mutated_slope_rejected+=1;
                    let changed_target=group_verifier.add(PRIME31,target,G);
                    assert_ne!(Some(terminal),changed_target);
                    mutated_target_rejected+=1;
                    if certificates.len()<12 {
                        certificates.push(json!({
                            "window":wi, "case":ci, "leaves":leaves,
                            "target":target,"ordinary_chart":true,
                            "first_step":{"tail":steps[0].tail,
                              "leaf":steps[0].leaf,"doubled":steps[0].doubled,
                              "next":steps[0].next,"lambda":steps[0].lambda,
                              "rho":steps[0].rho,"mu":steps[0].mu,
                              "sigma":steps[0].sigma},
                            "terminal":terminal
                        }));
                    }
                }
            }
        }
        per_window.push(json!({"window":wi,"indices":win,"ordinary":window_ordinary,
                    "exceptional":window_exceptional}));
    }
    assert_eq!(ordinary+exceptional,tested);
    assert_eq!(max_residual,0);
    assert_eq!(mutated_slope_rejected,ordinary);
    assert_eq!(mutated_target_rejected,ordinary);
    json!({
        "schema":"prime31.horner_xy_chain.v1",
        "curve":PRIME31,
        "seed":format!("0x{SEED:016x}"),
        "cases_per_window":cases_per_window,
        "tested":tested,"ordinary":ordinary,"exceptional":exceptional,
        "mutated_slope_rejected":mutated_slope_rejected,
        "mutated_target_same_witness_rejected":mutated_target_rejected,
        "max_polynomial_identity_residual":max_residual,
        "naive_group_additions":naive_group,
        "horner_group_additions":horner_group_count,
        "split_group_additions":split_group_count,
        "variables_before_terminal_elimination":72,
        "polynomial_equations":74,
        "maximum_equation_total_degree":8,
        "ordinary_chart_step_equations":56,
        "first_8_curve_and_factor_equations":16,
        "terminal_equations":2,
        "per_window":per_window,
        "certificates":certificates,
        "includes_generic_table_oracle":false,
        "planted_witness_controls_only":true,
        "public_only_unknown_index_extraction":false,
        "complete_ecdlp_solved":false,
        "pcc_verified":false
    })
}
#[test]
fn planted_horner_chain_certs() {
    let result=run(8);
    assert_eq!(result["tested"].as_u64(),Some(32));
    assert_eq!(result["max_polynomial_identity_residual"].as_u64(),Some(0));
    assert!(result["ordinary"].as_u64().unwrap() >= 28);
}
#[test]
fn poisoned_slope_and_factor_membership() {
    let mut ar=Arithmetic::default();
    let base=base31(&mut ar);
    let mut st=SEED;
    let leaves=leaves_from_seed(&mut st,&base,&WINDOWS[0]);
    let (_,steps)=polynomial_chain(&leaves).expect("ordinary frozen fixture");
    assert_eq!(steps[0].residuals(),[0;8]);
    let mut changed=steps[0];
    changed.mu=a(changed.mu,1);
    assert_ne!(changed.residuals(),[0;8]);
    assert_ne!(signed_base_root(1000,&base,&WINDOWS[0]),0);
}
fn main() {
    let result=run(64);
    assert!(result["ordinary"].as_u64().unwrap()>=240,
        "unexpected many exceptional charts");
    println!("{}",serde_json::to_string_pretty(&result).unwrap());
}
