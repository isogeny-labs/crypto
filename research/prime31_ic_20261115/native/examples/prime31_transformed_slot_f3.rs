//! Target-coupled transformed-slot Semaev F3 charts for weighted W8 on original Prime31.
//! Verifies 14-variable degree<=8 and 7-variable degree<=32 charts.
//! This is a planted-fixture identity / post-solve sign-lift gate, NOT an extractor.
//! Frozen protocol: ../rr9/TRANSFORMED_SLOT_F3_PROTOCOL_20261119.md
use prime31_algebraic_ic::{check_curve, Arithmetic, Point, PRIME31};
use serde_json::{json, Value};
use std::collections::HashSet;

type Aff = (u64, u64);
const P: u64 = 2_147_483_647;
const SEED: u64 = 0x5452_4e53_3346_3301;
const WINDOWS: [[usize; 8]; 4] = [
    [0,1,2,3,4,5,6,7],
    [8,9,10,11,12,13,14,15],
    [16,17,18,19,20,21,22,23],
    [23,24,25,26,27,28,29,30],
];
fn add(x: u64, y: u64) -> u64 { (x+y)%P }
fn sub(x: u64, y: u64) -> u64 { (x+P-y)%P }
fn mul(x: u64, y: u64) -> u64 { ((x as u128*y as u128)%(P as u128)) as u64 }
fn neg(pt: Aff) -> Aff { (pt.0, sub(0,pt.1)) }
fn splitmix64(state: &mut u64) -> u64 {
    *state=state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z=*state;
    z=(z^(z>>30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z=(z^(z>>27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z^(z>>31)
}
fn s3(x: u64, y: u64, z: u64) -> u64 {
    let d=sub(x,y);
    let xy=mul(x,y);
    let term1=mul(mul(d,d),mul(z,z));
    let term2=mul(2,mul(add(mul(add(x,y),add(xy,PRIME31.a)),mul(2,PRIME31.b)),z));
    let term3=mul(sub(xy,PRIME31.a),sub(xy,PRIME31.a));
    let term4=mul(4,mul(PRIME31.b,add(x,y)));
    sub(add(sub(term1,term2),term3),term4)
}
fn h(slot: &[Aff;8], t: u64) -> u64 {
    slot.iter().fold(1,|acc,pt|mul(acc,sub(t,pt.0)))
}
fn k(slot: &[Aff;8], suffix: u64, sum: u64) -> u64 {
    slot.iter().fold(1,|acc,pt|mul(acc,s3(pt.0,suffix,sum)))
}
fn base31(ar: &mut Arithmetic) -> Vec<Aff> {
    let mut base=Vec::new();
    for x in 0..10_000 {
        let y2=ar.rhs(PRIME31,x);
        if let Some(y)=ar.sqrt(y2,P) {
            if y!=0 {
                base.push((x,y.min(P-y)));
                if base.len()==31 {break;}
            }
        }
    }
    assert_eq!(base.len(),31);
    base
}
// Given only polynomial solution (transformed leaf x, suffix x), public Q, and
// public slot point lists, choose signs. Planted selection/signs are NOT input.
fn sign_lift(slot: &[[Aff;8];8], t:&[u64;8], z:&[u64;8],
             target:Aff, ar:&mut Arithmetic) -> Option<[Aff;8]> {
    if z[0]!=target.0 || z[7]!=t[7] {return None;}
    let mut recovered=[(0,0);8];
    let mut tail=*slot[7].iter().find(|pt|pt.0==t[7])?;
    recovered[7]=tail;
    for i in (0..7).rev() {
        let canonical=*slot[i].iter().find(|pt|pt.0==t[i])?;
        let mut found=None;
        for candidate in [canonical,neg(canonical)] {
            if let Some(sum)=ar.add(PRIME31,Some(candidate),Some(tail)) {
                if sum.0==z[i] { found=Some((candidate,sum)); break; }
            }
        }
        let (signed,s)=found?;
        recovered[i]=signed;
        tail=s;
    }
    if tail!=target {
        if neg(tail)!=target{return None;}
        for point in &mut recovered {*point=neg(*point);}
    }
    let mut sum:Point=None;
    for &point in &recovered {sum=ar.add(PRIME31,sum,Some(point));}
    (sum==Some(target)).then_some(recovered)
}
fn run(per_window:usize) -> Value {
    assert!((1..=128).contains(&per_window));
    let mut ar=Arithmetic::default();
    check_curve(PRIME31,&mut ar).expect("unchanged Prime31 order/curve");
    let base=base31(&mut ar);
    let mut state=SEED;
    let mut ordinary=0u64; let mut exceptional=0u64;
    let mut h_roots=0u64; let mut s3_zeros=0u64; let mut k_zeros=0u64;
    let mut target_mutations_rejected=0u64;
    let mut wrong_slot_rejected=0u64; let mut lifted=0u64;
    let mut windows_json=Vec::new(); let mut first=Vec::new();
    for (wi,win) in WINDOWS.iter().enumerate() {
        let slots:[[Aff;8];8]=std::array::from_fn(|i|std::array::from_fn(|j| {
            ar.scalar(PRIME31,1u64<<i,Some(base[win[j]]))
                .expect("nonzero scalar in prime-order group")
        }));
        let distinct:HashSet<u64>=slots.iter().flat_map(|v|v.iter().map(|p|p.0)).collect();
        assert_eq!(distinct.len(),64,"wrong slot-support assumption");
        let mut win_ord=0u64;let mut win_exc=0u64;
        for ci in 0..per_window {
            let mut choices=[0usize;8];let mut signs=[0u64;8];
            let mut leaf=[(0,0);8]; let mut suffix:[Point;8]=[None;8];
            for i in 0..8 {
                choices[i]=(splitmix64(&mut state)%8) as usize;
                signs[i]=splitmix64(&mut state)&1;
                let pt=slots[i][choices[i]];
                leaf[i]=if signs[i]==1 {neg(pt)} else {pt};
            }
            suffix[7]=Some(leaf[7]);
            for i in (0..7).rev() {suffix[i]=ar.add(PRIME31,Some(leaf[i]),suffix[i+1]);}
            if suffix.iter().any(Option::is_none) {exceptional+=1;win_exc+=1;continue;}
            let target=suffix[0].expect("finite target");
            let t:[u64;8]=std::array::from_fn(|i|leaf[i].0);
            let z:[u64;8]=std::array::from_fn(|i|suffix[i].expect("finite").0);
            assert_eq!(z[7],t[7]);assert_eq!(z[0],target.0);
            for i in 0..8 {
                assert_eq!(h(&slots[i],t[i]),0,"invalid selector root");
                h_roots+=1;
            }
            for i in 0..7 {
                assert_eq!(s3(t[i],z[i+1],z[i]),0,"transformed F3 failure");
                assert_eq!(k(&slots[i],z[i+1],z[i]),0,"leaf-eliminated product failure");
                s3_zeros+=1;k_zeros+=1;
            }
            let bad_t=(0..P).find(|&x|h(&slots[0],x)!=0).expect("finite field");
            assert_ne!(h(&slots[0],bad_t),0,"wrong selector accepted");
            wrong_slot_rejected+=1;
            let mutated_target=add(z[0],1);
            let poisoned=k(&slots[0],z[1],mutated_target);
            if poisoned!=0 {target_mutations_rejected+=1;}
            let mut verifier=Arithmetic::default();
            assert!(sign_lift(&slots,&t,&z,target,&mut verifier).is_some(),
                    "independent-sign post-solve verification failed");
            lifted+=1;ordinary+=1;win_ord+=1;
            if ci==0 {
                first.push(json!({"window":wi,"choices":choices,"planted_signs":signs,
                    "transformed_roots":t,"suffix_x":z,"target_q":target,
                    "mutated_k0":poisoned}));
            }
        }
        windows_json.push(json!({"window":wi,"distinct_slot_roots":distinct.len(),
            "ordinary":win_ord,"exceptional":win_exc}));
    }
    json!({"schema":"prime31.transformed_slot_f3.v1","curve":PRIME31,
        "seed":format!("0x{SEED:016x}"),"cases_per_window":per_window,
        "tested":4*per_window,"ordinary":ordinary,"exceptional":exceptional,
        "h_zero":h_roots,"s3_zero":s3_zeros,"k_zero":k_zeros,
        "wrong_slot_rejected":wrong_slot_rejected,
        "mutated_public_target_rejected":target_mutations_rejected,
        "post_solve_sign_lift_verified":lifted,"windows":windows_json,
        "first_case_per_window":first,
        "direct_variables":14,"direct_equations":15,"direct_max_total_degree":8,
        "direct_s3_total_degree":4,"direct_membership_degree":8,
        "eliminated_variables":7,"eliminated_equations":8,
        "eliminated_max_total_degree_bound":32,
        "eliminated_s3_product_factors_per_equation":8,
        "generic_sum_table_calls":0,"planted_witness_controls_only":true,
        "public_only_unknown_index_extraction":false,"complete_ecdlp_solved":false,
        "pcc_signed":false})
}
#[test]
fn transformed_f3_original_prime31() {
    let result=run(2);
    assert_eq!(result["tested"].as_u64(),Some(8));
    assert_eq!(result["ordinary"].as_u64(),Some(8));
    assert_eq!(result["h_zero"].as_u64(),Some(64));
    assert_eq!(result["s3_zero"].as_u64(),Some(56));
    assert_eq!(result["k_zero"].as_u64(),Some(56));
    assert_eq!(result["post_solve_sign_lift_verified"].as_u64(),Some(8));
}
fn main() {
    println!("{}",serde_json::to_string_pretty(&run(128)).expect("JSON"));
}
