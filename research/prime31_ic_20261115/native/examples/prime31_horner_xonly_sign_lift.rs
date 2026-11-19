//! Exact signed x-only Horner/F3 chart and post-solve sign lifting, on original Prime31.
//! Original Arithmetic/PRIME31 imported, not copied. Fixtures are planted controls:
//! this does NOT perform public-target-only unknown-index extraction or solve ECDLP.
//! Protocol: ../rr9/HORNER_XONLY_SIGN_LIFT_PROTOCOL_20261119.md
use prime31_algebraic_ic::{check_curve, Arithmetic, Point, PRIME31};
use serde_json::{json, Value};

const P: u64 = 2_147_483_647;
const SEED: u64 = 0x584f_4e4c_595f_4348;
const WINDOWS: [[usize; 8]; 4] = [
    [0, 1, 2, 3, 4, 5, 6, 7],
    [8, 9, 10, 11, 12, 13, 14, 15],
    [16, 17, 18, 19, 20, 21, 22, 23],
    [23, 24, 25, 26, 27, 28, 29, 30],
];
type Aff = (u64, u64);

#[inline] fn add(x: u64, y: u64) -> u64 { (x+y)%P }
#[inline] fn sub(x: u64, y: u64) -> u64 { (x+P-y)%P }
#[inline] fn mul(x: u64, y: u64) -> u64 { ((x as u128*y as u128)%P as u128) as u64 }
fn rhs(x: u64) -> u64 { add(add(mul(mul(x,x),x),mul(PRIME31.a,x)),PRIME31.b) }
fn numerator(x: u64) -> u64 {
    let x2=mul(x,x);
    add(sub(sub(mul(x2,x2),mul(2,mul(PRIME31.a,x2))),
        mul(8,mul(PRIME31.b,x))),mul(PRIME31.a,PRIME31.a))
}
fn denominator(x: u64) -> u64 { mul(4,rhs(x)) }
/// Semaev F3(u,v,r), quadratic in v; valid with full Fp signs on E.
fn f3(u: u64, v: u64, r: u64) -> u64 {
    let dr=sub(r,u);
    let ru=mul(r,u);
    let beta=sub(0,mul(2,add(mul(add(r,u),add(ru,PRIME31.a)),mul(2,PRIME31.b))));
    let dr2=sub(ru,PRIME31.a);
    let gamma=sub(mul(dr2,dr2),mul(4,mul(PRIME31.b,add(r,u))));
    add(add(mul(mul(dr,dr),mul(v,v)),mul(beta,v)),gamma)
}
fn splitmix64(state: &mut u64) -> u64 {
    *state=state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z=*state;
    z=(z^(z>>30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z=(z^(z>>27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z^(z>>31)
}
fn base31(ar: &mut Arithmetic) -> Vec<Aff> {
    let mut base=Vec::new();
    for x in 0..10_000 {
        if let Some(y)=ar.sqrt(ar.rhs(PRIME31,x),P) {
            if y!=0 {
                base.push((x,y.min(P-y)));
                if base.len()==31 { break; }
            }
        }
    }
    assert_eq!(base.len(),31);
    assert_eq!(base.iter().take(8).map(|a| a.0).collect::<Vec<_>>(),
        vec![0,1,2,3,5,6,8,9]);
    base
}
fn planted(state: &mut u64, base: &[Aff], win: &[usize;8]) -> [Aff;8] {
    std::array::from_fn(|_| {
        let r=splitmix64(state);
        let (x,y)=base[win[(r&7) as usize]];
        (x,if r&8==0 {y}else {sub(0,y)})
    })
}
fn neg(p: Aff) -> Aff { (p.0,sub(0,p.1)) }
fn canonical_leaf(x: u64, base: &[Aff], win: &[usize;8]) -> Option<Aff> {
    win.iter().map(|&j|base[j]).find(|&(z,_)|z==x)
}
fn xonly_residuals(xleaf: &[u64;8], x_suffix: &[u64;8], x_double: &[u64;7], qx:u64,
                   base:&[Aff], win:&[usize;8]) -> (u64,u64,u64,u64) {
    let mut bad_leaf=0u64;
    for &x in xleaf {
        if canonical_leaf(x,base,win).is_none() { bad_leaf+=1; }
    }
    let mut bad_d=0u64; let mut bad_f3=0u64;
    for i in 0..7 {
        if sub(mul(denominator(x_suffix[i+1]),x_double[i]),
               numerator(x_suffix[i+1]))!=0 {bad_d+=1;}
        if f3(xleaf[i],x_double[i],x_suffix[i])!=0 {bad_f3+=1;}
    }
    let bad_terminal=u64::from(x_suffix[0]!=qx || x_suffix[7]!=xleaf[7]);
    (bad_leaf,bad_d,bad_f3,bad_terminal)
}
/// Sign recovery takes only public B8 canonical points, an x-only root,
/// and the actual public target Q. It deliberately has NO planted-sign input.
/// Group operations here are *post-solve verification*, not an extraction oracle.
fn recover(xleaf:&[u64;8], x_suffix:&[u64;8], target: Aff, base:&[Aff], win:&[usize;8],
           ar:&mut Arithmetic) -> Option<([Aff;8],u64)> {
    if x_suffix[0]!=target.0 || x_suffix[7]!=xleaf[7] {return None;}
    let mut leaves=[(0u64,0u64);8];
    let mut tail=canonical_leaf(xleaf[7],base,win)?;
    leaves[7]=tail;
    let mut ambiguous=0u64;
    for i in (0..7).rev() {
        let doubled=ar.add(PRIME31,Some(tail),Some(tail))?;
        let canonical=canonical_leaf(xleaf[i],base,win)?;
        let mut chosen=None;
        for candidate in [canonical,neg(canonical)] {
            if let Some(result)=ar.add(PRIME31,Some(candidate),Some(doubled)) {
                if result.0==x_suffix[i] {
                    if chosen.is_some() {ambiguous+=1;}
                    else {chosen=Some((candidate,result));}
                }
            }
        }
        let (leaf,new_tail)=chosen?;
        leaves[i]=leaf;
        tail=new_tail;
    }
    if tail!=target {
        if neg(tail)!=target {return None;}
        for leaf in &mut leaves {*leaf=neg(*leaf);}
    }
    let mut check=None;
    for (i,&leaf) in leaves.iter().enumerate() {
        let weighted=ar.scalar(PRIME31,1u64<<i,Some(leaf));
        check=ar.add(PRIME31,check,weighted);
    }
    (check==Some(target)).then_some((leaves,ambiguous))
}
fn run(cases_per_window:usize) -> Value {
    assert!((1..=256).contains(&cases_per_window));
    let mut setup=Arithmetic::default();
    check_curve(PRIME31,&mut setup).expect("original full-prime-order curve");
    let base=base31(&mut setup);
    let mut state=SEED;
    let mut tested=0u64; let mut ordinary=0u64; let mut exceptional=0u64;
    let mut ambiguities=0u64;
    let mut poisoned_double_rejected=0u64; let mut poisoned_target_rejected=0u64;
    let mut fixture_group_ops=0u64; let mut recovery_group_ops=0u64;
    let mut certificates=Vec::new(); let mut per_window=Vec::new();
    for (wi,win) in WINDOWS.iter().enumerate() {
        let mut good=0u64;let mut skipped=0u64;
        for ci in 0..cases_per_window {
            tested+=1;
            let signed=planted(&mut state,&base,win);
            let mut gen=Arithmetic::default();
            let mut suffix=[None;8];
            suffix[7]=Some(signed[7]);
            for i in (0..7).rev() {
                let doubled=gen.add(PRIME31,suffix[i+1],suffix[i+1]);
                suffix[i]=gen.add(PRIME31,Some(signed[i]),doubled);
            }
            fixture_group_ops+=gen.counts.group_additions;
            if suffix.iter().any(Option::is_none) {
                exceptional+=1;skipped+=1;continue;
            }
            let target=suffix[0].expect("finite");
            let x_suffix=std::array::from_fn(|i|suffix[i].expect("finite").0);
            let xleaf=std::array::from_fn(|i|signed[i].0);
            let mut dig=Arithmetic::default();
            let x_double=std::array::from_fn(|i| {
                dig.add(PRIME31,suffix[i+1],suffix[i+1]).expect("odd order, finite").0
            });
            // Pure polynomial root check: no group operations or supplied signs.
            assert_eq!(xonly_residuals(&xleaf,&x_suffix,&x_double,target.0,&base,win),
                       (0,0,0,0),"x-only polynomial failed");
            let mut poisoned=x_double;
            poisoned[0]=add(poisoned[0],1);
            if xonly_residuals(&xleaf,&x_suffix,&poisoned,target.0,&base,win).1!=0 {
                poisoned_double_rejected+=1;
            } else {panic!("poisoned doubling survived");}
            if xonly_residuals(&xleaf,&x_suffix,&x_double,add(target.0,1),&base,win).3!=0 {
                poisoned_target_rejected+=1;
            } else {panic!("poisoned public target survived");}
            let mut rec_ar=Arithmetic::default();
            let (recovered,ambiguous)=recover(&xleaf,&x_suffix,target,&base,win,&mut rec_ar)
                .expect("lost signed lifting");
            recovery_group_ops+=rec_ar.counts.group_additions;
            ambiguities+=ambiguous;
            good+=1;ordinary+=1;
            if certificates.len()<16 {
                certificates.push(json!({"window":wi,"case":ci,"x_leaves":xleaf,
                   "x_suffix":x_suffix,"x_doubled":x_double,"target":target,
                   "recovered_signed_leaves":recovered}));
            }
        }
        per_window.push(json!({"window":wi,"ordinary":good,"exceptional":skipped}));
    }
    assert_eq!(ordinary+exceptional,tested);
    json!({"schema":"prime31.xonly_horner_sign_lift.v1",
       "curve":PRIME31,"seed":format!("0x{SEED:016x}"),
       "cases_per_window":cases_per_window,"tested":tested,"ordinary":ordinary,
       "exceptional":exceptional,"sign_recovery_ambiguities":ambiguities,
       "poisoned_double_rejected":poisoned_double_rejected,
       "poisoned_target_rejected":poisoned_target_rejected,
       "fixture_group_additions":fixture_group_ops,
       "post_solve_recovery_group_additions":recovery_group_ops,
       "variables_unspecialized":22,"equations_unspecialized":23,
       "variables_after_public_x_target":21,"equations_after_public_x_target":22,
       "f3_and_doubling_max_total_degree":4,"leaf_max_degree":8,
       "prior_xy_variables_unspecialized":72,"prior_xy_equations":74,
       "per_window":per_window,"certificates":certificates,
       "generic_sum_table_calls":0,"public_only_unknown_index_extraction":false,
       "complete_ecdlp_solved":false,"independent_signed_pcc_verified":false})
}
#[test]
fn planted_polynomial_identities_and_sign_lift() {
    let r=run(8);
    assert_eq!(r["tested"].as_u64(),Some(32));
    assert_eq!(r["ordinary"].as_u64(),Some(32));
    assert_eq!(r["poisoned_double_rejected"].as_u64(),Some(32));
}
#[test]
fn no_hidden_sign_used_in_recovery() {
    let mut ar=Arithmetic::default();
    let b=base31(&mut ar); let mut seed=SEED;
    let leaves=planted(&mut seed,&b,&WINDOWS[0]);
    let mut chain=[None;8];chain[7]=Some(leaves[7]);
    for i in (0..7).rev() {
        let z=ar.add(PRIME31,chain[i+1],chain[i+1]);
        chain[i]=ar.add(PRIME31,Some(leaves[i]),z);
    }
    let q=chain[0].unwrap();
    let xs=std::array::from_fn(|i|chain[i].unwrap().0);
    let xl=std::array::from_fn(|i|leaves[i].0);
    let recovered=recover(&xl,&xs,q,&b,&WINDOWS[0],&mut ar);
    assert!(recovered.is_some());
    assert!(recover(&xl,&xs,(add(q.0,1),q.1),&b,&WINDOWS[0],&mut ar).is_none());
}
fn main() {
    let r=run(256);
    assert!(r["ordinary"].as_u64().unwrap()>=1_000,
      "predeclared ordinary-chart threshold failed");
    println!("{}",serde_json::to_string_pretty(&r).unwrap());
}
