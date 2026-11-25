//! Same-team bounded correctness qualifier; not a decomposition or cost experiment.
mod production;
use production::{dickson_abscissae, PrimeCurve};
use std::collections::BTreeSet;

const P: u64 = 2_147_483_647;

// Separate evaluator: none of these call production's arithmetic or square root.
fn mul(a: u64, b: u64, p: u64) -> u64 { ((a as u128 * b as u128) % p as u128) as u64 }
fn add(a: u64, b: u64, p: u64) -> u64 { ((a as u128 + b as u128) % p as u128) as u64 }
fn sub(a: u64, b: u64, p: u64) -> u64 { (a + p - b % p) % p }
fn pow(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1;
    while e > 0 { if e & 1 != 0 { r = mul(r,a,p); } a = mul(a,a,p); e >>= 1; }
    r
}
fn sqrt(n: u64, p: u64) -> Option<u64> {
    assert_eq!(p % 4, 3);
    let r = pow(n, (p+1)/4, p);
    (mul(r,r,p) == n).then_some(r)
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct F2 { a: u64, b: u64 }
impl F2 {
    fn one() -> Self { Self { a:1,b:0 } }
    fn mul(self, other: Self) -> Self {
        Self { a:sub(mul(self.a,other.a,P),mul(self.b,other.b,P),P),
               b:add(mul(self.a,other.b,P),mul(self.b,other.a,P),P) }
    }
    fn pow(mut self, mut n: u64) -> Self {
        let mut r = Self::one();
        while n > 0 { if n & 1 != 0 { r = r.mul(self); } self=self.mul(self); n >>= 1; }
        r
    }
    fn conj(self) -> Self { Self { a:self.a, b:sub(0,self.b,P) } }
    fn trace(self) -> u64 { add(self.a,self.a,P) }
}

fn terminal_and_derivative(mut x: u64, k: u32) -> (u64,u64) {
    let mut derivative=1;
    for _ in 0..k {
        derivative=mul(derivative,mul(2,x,P),P);
        x=sub(mul(x,x,P),2,P);
    }
    (x,derivative)
}

fn recurrence(a: u64, k: u32) -> Vec<u64> {
    let n=1usize << k;
    let q=sub(mul(a,a,P),2,P);
    let mut xs=vec![a];
    if n > 1 { xs.push(sub(mul(mul(a,a,P),a,P),mul(3,a,P),P)); }
    while xs.len() < n {
        let j=xs.len();
        xs.push(sub(mul(q,xs[j-1],P),xs[j-2],P));
    }
    xs
}

fn main() {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    assert!(arguments.len() <= 1, "at most one control flag is accepted");
    let mode = arguments.first().map(String::as_str).unwrap_or("normal");
    assert!(matches!(mode, "normal" | "--mutate-first-root" | "--mutate-norm-one-lift"), "unknown control flag");
    let curve=PrimeCurve { p:P,a:P-3,b:21 };
    let zero_error=dickson_abscissae(P,0,0,0).expect_err("production must reject depth zero");
    let thirty_error=dickson_abscissae(P,30,0,0).expect_err("production must reject depth thirty");
    let ramification_error=dickson_abscissae(P,2,0,2).expect_err("production must reject ramified fibre");
    assert!(ramification_error.contains("ramifies"));
    let splitting_error=dickson_abscissae(13,1,0,0).expect_err("production must reject unsplit fibre");
    assert!(splitting_error.contains("fails to split"));
    assert!(!zero_error.is_empty() && !thirty_error.is_empty());
    println!("{{\"schema\":\"pplus-native-selector-check/v2\",\"standing\":\"pcc:unverified\",\"p\":{P},\"a\":{},\"b\":21,\"rows\":[",P-3);
    let mut total_roots=0;
    for k in 0..=12 {
        let mut roots=if k==0 { vec![0] } else { dickson_abscissae(P,k,0,0).expect("production fibre") };
        if mode == "--mutate-first-root" && k == 1 { roots[0] = 2; }
        roots.sort_unstable();
        let unique: BTreeSet<_>=roots.iter().copied().collect();
        assert_eq!(roots.len(),1usize << k);
        assert_eq!(unique.len(),roots.len());
        let mut curve_columns=0usize;
        let mut nonzero_y_pairs=0usize;
        let mut zero_y=0usize;
        let mut zero_x=0usize;
        for &x in &roots {
            assert!(x<P);
            let (value,derivative)=terminal_and_derivative(x,k);
            assert_eq!(value,0,"terminal k={k} x={x}");
            assert_ne!(derivative,0,"derivative k={k} x={x}");
            let four_minus_x2=sub(4,mul(x,x,P),P);
            let s=sqrt(four_minus_x2,P).expect("norm-one coordinate lift");
            let inv2=(P+1)/2;
            let mut w=F2 { a:mul(x,inv2,P),b:mul(s,inv2,P) };
            if mode == "--mutate-norm-one-lift" && k == 0 { w.a=add(w.a,1,P); }
            assert_eq!(w.trace(),x);
            assert_eq!(w.mul(w.conj()),F2::one());
            assert_eq!(w.pow(P),w.conj());
            assert_eq!(w.pow(P+1),F2::one());
            assert_eq!(w.pow(1u64 << (k+2)),F2::one());
            assert_ne!(w.pow(1u64 << (k+1)),F2::one());
            assert_eq!(w.mul(w).trace(),sub(mul(x,x,P),2,P));
            let rhs=add(sub(mul(mul(x,x,P),x,P),mul(3,x,P),P),21,P);
            assert_eq!(curve.rhs(x),rhs);
            if let Some(y)=sqrt(rhs,P) {
                curve_columns+=1;
                assert_eq!(mul(y,y,P),rhs);
                assert_eq!(mul(sub(0,y,P),sub(0,y,P),P),rhs);
                if y==0 { zero_y+=1; } else { nonzero_y_pairs+=1; }
            }
            if x==0 { zero_x+=1; }
        }
        if k>0 {
            assert_ne!(terminal_and_derivative(2,k).0,0,"negative root control");
            let mut recurrence_roots=recurrence(roots[0],k);
            let recurrence_unique: BTreeSet<_>=recurrence_roots.iter().copied().collect();
            assert_eq!(recurrence_unique.len(),roots.len());
            recurrence_roots.sort_unstable();
            assert_eq!(recurrence_roots,roots,"standard odd-power recurrence");
        }
        total_roots+=roots.len();
        if k>0 { println!(","); }
        print!("{{\"k\":{k},\"production_constructor\":{},\"root_count\":{},\"curve_columns\":{curve_columns},\"nonzero_y_pairs\":{nonzero_y_pairs},\"signed_affine_points\":{},\"zero_y_columns\":{zero_y},\"zero_x_coordinates\":{zero_x},\"nonzero_derivative_count\":{},\"norm_one_lift_count\":{},\"exact_order\":{},\"recurrence_set_equal\":{}}}",
            k>0,roots.len(),2*nonzero_y_pairs+zero_y,roots.len(),roots.len(),1u64 << (k+2),if k>0 { "true" } else { "null" });
    }
    println!("],\"total_checked_roots\":{total_roots},\"controls\":{{\"production_k0_rejected\":true,\"production_k30_rejected\":true,\"ramified_fibre_rejected\":true,\"unsplit_fibre_rejected\":true,\"off_fibre_x2_rejected_at_all_positive_depths\":true}},\"status\":\"all_required_checks_passed\"}}");
}
