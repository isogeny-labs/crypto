// Finite scalar D=4 supplement only. The universal coefficient/degree proof
// is in Lean; this program does not sample the unbounded D theorem.
#[path = "../../../../../src/cryptanalysis/identity_certificate.rs"]
mod identity_certificate;
#[path = "../../../../../src/hash/sha256.rs"]
pub mod sha256;
pub mod hash { pub use crate::sha256; }
use identity_certificate::{Expr as E, IdentityStatement, ProofRef, Verdict, issue, check_with_extra};

fn entry(i: usize, j: usize, a: &[E], z: &E) -> E {
    let shift = if j < 4 { j } else { j - 4 };
    if i < shift { return E::Const(0); }
    let k = i - shift;
    if j < 4 {
        match k {
            0 => E::times(z.clone(), z.clone()),
            1 => E::times(E::Const(-2), z.clone()),
            2 => E::Const(1),
            _ => E::Const(0),
        }
    } else {
        a.get(k).cloned().unwrap_or(E::Const(0))
    }
}

fn determinant(a: &[E], z: &E) -> E {
    fn visit(j: usize, p: &mut Vec<usize>, a: &[E], z: &E, terms: &mut Vec<E>) {
        if j == 6 {
            let inversions = (0..6).flat_map(|i| (i+1..6).map(move |k| (i,k)))
                .filter(|&(i,k)| p[i] > p[k]).count();
            let term = E::product((0..6).map(|k| entry(p[k], k, a, z)).collect());
            terms.push(if inversions % 2 == 0 { term } else { E::minus(term) });
            return;
        }
        for i in 0..6 {
            if p.contains(&i) || entry(i,j,a,z) == E::Const(0) { continue; }
            p.push(i);
            visit(j+1,p,a,z,terms);
            p.pop();
        }
    }
    let mut terms = Vec::new();
    visit(0,&mut Vec::new(),a,z,&mut terms);
    E::sum(terms)
}

fn main() {
    let a: Vec<E> = (0..5).map(|i| E::var(&format!("a{i}"))).collect();
    let z = E::var("z");
    let lhs = determinant(&a,&z);
    let eval = a[..4].iter().rev().fold(a[4].clone(), |v,c|
        E::plus(E::times(v,z.clone()),c.clone()));
    let statement = IdentityStatement {
        name: "Fixed-degree quartic resultant against a monic square".into(),
        source: "ResultantDegreeStepAudit.resultantStep_coeff_top uses the actual resultant_X_sub_C_pow_right lemma; this finite supplement checks only Res_(4,2)(sum a_i X^i,(X-z)^2)=(sum a_i z^i)^2.".into(),
        variables: (0..5).map(|i| format!("a{i}")).chain(std::iter::once("z".into())).collect(),
        degree_bound: 10, lhs, rhs: E::times(eval.clone(),eval),
    };
    let bytes = std::fs::read("../lean/ResultantDegreeStep.lean").expect("read frozen Lean source");
    let digest = sha256::sha256(&bytes).iter().map(|b| format!("{b:02x}")).collect();
    let cert = issue(statement,0x2026_1109_1810,32,
        "Producer-issued on one host with the unchanged repository checker. Only the scalar fixed-D=4 rewrite is sampled. The unbounded-D determinant/coefficient/degree and actual S5 conclusions rest on Lean; no independent-host or signed standing.",
        Some(ProofRef { kind:"lean4".into(),
            path:"research/theorem_census_20261109/lean/ResultantDegreeStep.lean".into(),sha256:digest }))
        .expect("identity must verify before sealing");
    assert!(matches!(check_with_extra(&cert,Some((0x5e4a_e064,11))).unwrap(),Verdict::Accept{..}));
    // Preserve u64 modulus bytes; no JavaScript JSON round-trip.
    println!("{}",serde_json::to_string(&cert).unwrap());
}
