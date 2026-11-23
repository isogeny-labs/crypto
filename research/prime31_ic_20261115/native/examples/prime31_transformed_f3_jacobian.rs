//! Nonsingular target-coupled Semaev F3 chart, unchanged original Prime31.
//! Frozen protocol: ../rr9/TRANSFORMED_F3_JACOBIAN_PROTOCOL_20261119.md
//! Planted-chain Jacobian certificate only; NOT a public-target-only extractor.
use prime31_algebraic_ic::{check_curve, Arithmetic, Point, PRIME31};
use serde_json::{json, Value};

type Aff = (u64,u64);
const P: u64 = 2_147_483_647;
const SEED: u64 = 0x4a41_434f_4249_414e;
const WINDOWS: [[usize;8];4] = [
    [0,1,2,3,4,5,6,7],
    [8,9,10,11,12,13,14,15],
    [16,17,18,19,20,21,22,23],
    [23,24,25,26,27,28,29,30],
];

fn add(x:u64,y:u64)->u64 {(x+y)%P}
fn sub(x:u64,y:u64)->u64 {(x+P-y)%P}
fn mul(x:u64,y:u64)->u64 {((x as u128*y as u128)%P as u128) as u64}
fn f(x:u64)->u64 {add(add(mul(mul(x,x),x),mul(PRIME31.a,x)),PRIME31.b)}
fn negate(p:Aff)->Aff {(p.0,sub(0,p.1))}
fn splitmix64(s:&mut u64)->u64 {
    *s=s.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z=*s;
    z=(z^(z>>30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z=(z^(z>>27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z^(z>>31)
}
fn s3(x:u64,y:u64,z:u64)->u64 {
    let d=sub(x,y);
    let xy=mul(x,y);
    let ax=add(x,y);
    let c=sub(mul(sub(xy,PRIME31.a),sub(xy,PRIME31.a)),
              mul(4,mul(PRIME31.b,ax)));
    sub(add(mul(mul(d,d),mul(z,z)),c),
        mul(2,mul(add(mul(ax,add(xy,PRIME31.a)),
                          mul(2,PRIME31.b)),z)))
}
fn ds3_z(x:u64,y:u64,z:u64)->u64 {
    sub(mul(2,mul(mul(sub(x,y),sub(x,y)),z)),
        mul(2,add(mul(add(x,y),add(mul(x,y),PRIME31.a)),
                  mul(2,PRIME31.b))))
}
fn hprime(slot:&[Aff;8],root:u64)->u64 {
    let mut found=0;let mut deriv=1u64;
    for p in slot {
        if p.0==root {found+=1;}
        else {deriv=mul(deriv,sub(root,p.0));}
    }
    assert_eq!(found,1,"duplicate or missing transformed slot root");
    assert_ne!(deriv,0,"selector derivative vanished");
    deriv
}
fn base31(ar:&mut Arithmetic)->Vec<Aff> {
    let mut b=Vec::new();
    for x in 0..10_000u64 {
        let y2=ar.rhs(PRIME31,x);
        if let Some(y)=ar.sqrt(y2,P) {
            if y!=0 {
                b.push((x,y.min(P-y)));
                if b.len()==31 {break;}
            }
        }
    }
    assert_eq!(b.len(),31);
    b
}
fn run(per_window:usize)->Value {
    assert!((1..=128).contains(&per_window));
    let mut ar=Arithmetic::default();
    check_curve(PRIME31,&mut ar).expect("original Prime31 curve and generator");
    let base=base31(&mut ar);
    let mut state=SEED;
    let mut ordinary=0usize;
    let mut stage_zero=0usize;
    let mut selector_zero=0usize;
    let mut pivots=0usize;
    let mut discriminant_checks=0usize;
    let mut nonordinary=0usize;
    let mut first=Vec::new();
    for (w,window) in WINDOWS.iter().enumerate() {
        let slots:[[Aff;8];8]=std::array::from_fn(|i|
            std::array::from_fn(|j|
                ar.scalar(PRIME31,1u64<<i,Some(base[window[j]]))
                  .expect("original prime-order scalar nonzero")));
        let distinct:std::collections::HashSet<u64>=slots.iter()
            .flat_map(|s|s.iter().map(|a|a.0)).collect();
        assert_eq!(distinct.len(),64,"transformed support overlap");
        for c in 0..per_window {
            let mut leaves=[(0u64,0u64);8];
            for i in 0..8 {
                let choice=(splitmix64(&mut state)%8) as usize;
                let sign=splitmix64(&mut state)&1;
                let canonical=slots[i][choice];
                leaves[i]=if sign==0 {canonical} else {negate(canonical)};
            }
            let mut suffix:[Point;8]=[None;8];
            suffix[7]=Some(leaves[7]);
            for i in (0..7).rev() {
                suffix[i]=ar.add(PRIME31,Some(leaves[i]),suffix[i+1]);
            }
            if suffix.iter().any(Option::is_none) {nonordinary+=1;continue;}
            let z:[u64;8]=std::array::from_fn(|i|suffix[i].expect("finite").0);
            let t:[u64;8]=std::array::from_fn(|i|leaves[i].0);
            let mut selector_pivots=[0u64;8];
            let mut stage_pivots=[0u64;7];
            for i in 0..8 {
                selector_pivots[i]=hprime(&slots[i],t[i]);
                assert_ne!(selector_pivots[i],0);
                selector_zero+=1;
            }
            for i in 0..7 {
                assert_eq!(s3(t[i],z[i+1],z[i]),0,"S3 identity");
                let d=ds3_z(t[i],z[i+1],z[i]);
                assert_ne!(d,0,"singular S3 chart stage");
                let expected=mul(16,mul(f(t[i]),f(z[i+1])));
                assert_eq!(mul(d,d),expected,"S3 discriminant identity");
                assert_ne!(expected,0,"rational 2-torsion / twist branch");
                stage_pivots[i]=d;
                stage_zero+=1;
                discriminant_checks+=1;
            }
            // 8 distinct leaf-selector pivots, plus the 6 suffix F3 pivots
            // for i=1..6: a 14x14 triangular Jacobian minor.
            let mut jacobian_minor=1u64;
            for v in selector_pivots {
                jacobian_minor=mul(jacobian_minor,v);
                pivots+=1;
            }
            for v in stage_pivots.iter().skip(1) {
                jacobian_minor=mul(jacobian_minor,*v);
                pivots+=1;
            }
            assert_ne!(jacobian_minor,0,"rank-14 Jacobian minor vanishes");
            if c==0 {first.push(json!({
                "window":w, "target":suffix[0].expect("finite"),
                "leaf_x":t, "suffix_x":z,
                "hprime":selector_pivots, "s3_z_derivative":stage_pivots,
                "jacobian_minor_up_to_sign":jacobian_minor
            }));}
            ordinary+=1;
        }
    }
    json!({
        "schema":"prime31.transformed_f3_jacobian.v1",
        "curve":PRIME31, "seed":format!("0x{SEED:016x}"),
        "cases_per_window":per_window, "tested":4*per_window,
        "ordinary":ordinary, "exceptional":nonordinary,
        "selector_derivatives_nonzero":selector_zero,
        "f3_equations_and_derivatives_nonzero":stage_zero,
        "f3_discriminant_identities":discriminant_checks,
        "full_rank_14x14_pivots":pivots,
        "target_conditioned_variables":14,
        "target_conditioned_equations":15,
        "untargeted_14eq_upper_degree":1_073_741_824u64,
        "first_case_per_window":first,
        "uses_original_arithmetic":true,
        "generic_sum_table_calls":0,
        "planted_fixture_gate_only":true,
        "public_only_unknown_index_extraction":false,
        "whole_solver_speedup_measured":false, "pcc_signed":false
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn original_prime31_smooth_chart() {
        let x=run(2);
        assert_eq!(x["ordinary"],8);
        assert_eq!(x["full_rank_14x14_pivots"],112);
        assert_eq!(x["f3_discriminant_identities"],56);
    }
}
fn main() {
    let v=run(128);
    println!("{}",serde_json::to_string_pretty(&v).expect("JSON"));
}
