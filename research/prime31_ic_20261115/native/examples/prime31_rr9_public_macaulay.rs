//! Public-residual-only, degree-5 Macaulay feasibility gate for RR9 B8.
//! This is a negative-only algebraic certificate, NOT a relation extractor or ECDLP solver.
//! Existing prime31 Arithmetic/Curve are reused; signs and scalar labels enter test oracle only.
//! cargo +1.100.0 test --release --locked --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_rr9_public_macaulay
//! cargo +1.100.0 run --release --locked --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_rr9_public_macaulay
use prime31_algebraic_ic::{check_curve, Arithmetic, Point, PRIME31};
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::time::Instant;

const P: u64 = 2_147_483_647;
const NV: usize = 7;
const DEGREE: u8 = 5;
type Exp = [u8; NV];
type Polynomial = BTreeMap<Exp, u64>;

fn add(a: u64, b: u64) -> u64 {
    let z = a + b;
    if z >= P { z - P } else { z }
}
fn sub(a: u64, b: u64) -> u64 {
    if a >= b { a - b } else { a + P - b }
}
fn mul(a: u64, b: u64) -> u64 {
    // Mersenne folding; the exact u128 % P implementation in the original
    // native Rust library is independently checked against this below.
    let x = a * b;
    let y = (x & P) + (x >> 31);
    let z = (y & P) + (y >> 31);
    if z >= P { z - P } else { z }
}
fn pow(mut x: u64, mut e: u64) -> u64 {
    let mut v = 1;
    while e != 0 {
        if e & 1 == 1 { v = mul(v, x); }
        x = mul(x, x);
        e >>= 1;
    }
    v
}
fn term(q: &mut Polynomial, coefficient: u64, first: Option<usize>, second: Option<usize>) {
    if coefficient == 0 { return; }
    let mut e = [0u8; NV];
    for i in [first, second].into_iter().flatten() { e[i] += 1; }
    let prev = q.get(&e).copied().unwrap_or(0);
    let next = add(prev, coefficient);
    if next == 0 { q.remove(&e); } else { q.insert(e, next); }
}
fn poly_mul(a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut out = vec![0; a.len() + b.len() - 1];
    for (i, &ai) in a.iter().enumerate() {
        for (j, &bj) in b.iter().enumerate() {
            out[i+j] = add(out[i+j], mul(ai, bj));
        }
    }
    out
}
fn factor_polynomial(xs: &[u64; 8]) -> [u64; 9] {
    let mut h = vec![1];
    for &x in xs { h = poly_mul(&h, &[sub(0, x), 1]); }
    h.try_into().unwrap()
}
fn equations(r: u64, s: u64, h: &[u64; 9]) -> [Polynomial; 8] {
    let mut q: [Polynomial; 8] = std::array::from_fn(|_| BTreeMap::new());
    let d = [add(mul(r,r), PRIME31.a), r, 1];
    let vvar = |i: usize| if i == 3 { None } else { Some(i+4) };
    for i in 0..4 {
        for j in 0..4 {
            for (k, &dk) in d.iter().enumerate() {
                if i+j+k < 8 { term(&mut q[i+j+k], dk, vvar(i), vvar(j)); }
            }
            // -(X-r)A^2 = r A^2 - X A^2.
            term(&mut q[i+j], r, Some(i), Some(j));
            if i+j+1 < 8 { term(&mut q[i+j+1], P-1, Some(i), Some(j)); }
            // -2s AV.
            term(&mut q[i+j], sub(0, mul(2,s)), Some(i), vvar(j));
        }
    }
    for i in 0..8 { term(&mut q[i], sub(0, h[i]), None, None); }
    q
}
fn eval_poly(q: &Polynomial, v: &[u64;NV]) -> u64 {
    let mut total = 0;
    for (e, &coeff) in q {
        let mut z = coeff;
        for j in 0..NV { for _ in 0..e[j] { z = mul(z,v[j]); } }
        total = add(total,z);
    }
    total
}
fn independent_direct_m(v: &[u64;NV], r: u64, s: u64) -> [u64;9] {
    let a = [v[0],v[1],v[2],v[3]];
    let vv = [v[4],v[5],v[6],1];
    let d = [add(mul(r,r),PRIME31.a),r,1];
    let dvv = poly_mul(&d,&poly_mul(&vv,&vv));
    let xaa = poly_mul(&[sub(0,r),1], &poly_mul(&a,&a));
    let av = poly_mul(&a,&vv);
    std::array::from_fn(|i| {
        let mut z=dvv[i];
        if i<xaa.len() { z=sub(z,xaa[i]); }
        if i<av.len() { z=sub(z,mul(mul(2,s),av[i])); }
        z
    })
}
fn validate_quadrics(q: &[Polynomial;8], h: &[u64;9], r:u64, s:u64) {
    for case in 0..8u64 {
        let v: [u64;NV]=std::array::from_fn(|j| {
            if case==0 { 0 } else if case==1 { 1 } else { (case*37 + (j as u64)*11 + (j*j) as u64) % P }
        });
        let m=independent_direct_m(&v,r,s);
        assert_eq!(m[8],1,"leading coefficient must be fixed monic");
        for i in 0..8 {assert_eq!(eval_poly(&q[i],&v), sub(m[i],h[i]),"symbolic coefficient mismatch");}
    }
}
fn exact(n:usize, rem:u8, e:&mut Exp, output:&mut Vec<Exp>) {
    if n == NV { if rem==0 { output.push(*e); } return; }
    for v in 0..=rem { e[n]=v; exact(n+1, rem-v, e, output); }
}
fn monomials(max_degree:u8) -> Vec<Exp> {
    let mut out=Vec::new();
    let mut e=[0u8;NV];
    for d in 0..=max_degree { exact(0,d,&mut e,&mut out); }
    out
}
fn add_exp(a:&Exp,b:&Exp)->Exp {
    std::array::from_fn(|i| a[i]+b[i])
}
#[derive(Default)]
struct Ops { reductions: u64, multiplications: u64, inversions:u64 }
struct Pivot {
    row: Vec<u64>,
    input: usize,
    inverse: u64,
    dependencies: Vec<(usize,u64)>,
}
fn raw_row(q:&Polynomial, multiplier:&Exp, index:&HashMap<Exp,usize>, columns:usize)->Vec<u64> {
    let mut row=vec![0;columns];
    for (e,&c) in q {
        let at=*index.get(&add_exp(e,multiplier)).expect("bounded Macaulay monomial");
        row[at]=add(row[at],c);
    }
    row
}
fn prove_unit(
    pivots:&[Option<Pivot>], sources:&[(usize,Exp)], quadrics:&[Polynomial;8],
    index:&HashMap<Exp,usize>, columns:usize
) -> Value {
    // A normalized pivot is inverse*(input_row - sum factor*pivot_dep).
    // Every dependency has STRICTLY HIGHER monomial order than its consumer,
    // so expand recursively by ascending pivot column and replay against
    // an independently reassembled Macaulay matrix (no signed support input).
    let mut pending=vec![0u64;columns];
    pending[0]=1;
    let mut coefficients=vec![0u64;sources.len()];
    for col in 0..columns {
        let weight=pending[col];
        if weight==0 {continue;}
        let piv=pivots[col].as_ref().expect("unit pivot references an absent row");
        let w=mul(weight,piv.inverse);
        coefficients[piv.input]=add(coefficients[piv.input],w);
        for &(higher,factor) in &piv.dependencies {
            assert!(higher>col);
            pending[higher]=sub(pending[higher],mul(w,factor));
        }
    }
    let mut check=vec![0u64;columns];
    let mut witness=Vec::new();
    for (i,&weight) in coefficients.iter().enumerate() {
        if weight==0 {continue;}
        let (qidx,e)=sources[i];
        let line=raw_row(&quadrics[qidx],&e,index,columns);
        for j in 0..columns {check[j]=add(check[j],mul(weight,line[j]));}
        witness.push(json!({"equation":qidx,"multiplier_exponents":e,"coefficient":weight}));
    }
    assert_eq!(check[0],1,"unit coefficient replay differs");
    assert!(check[1..].iter().all(|&x|x==0),"unit identity has nonconstant remainder");
    json!({"identity_replayed":true,"nonzero_witness_terms":witness.len(),"witness":witness})
}

// Reduce a public coefficient-function against the degree-5 Macaulay span.
// This computes quotient normal forms, *not* support-mask enumeration.
fn normal_form(mut row:Vec<u64>, pivots:&[Option<Pivot>])->Vec<u64> {
    for col in (0..row.len()).rev() {
        let z=row[col];
        if z==0 {continue;}
        if let Some(p)=&pivots[col] {
            for j in 0..=col {row[j]=sub(row[j],mul(z,p.row[j]));}
        }
    }
    row
}
fn extract_unique_normal_form(
    pivots:&[Option<Pivot>], index:&HashMap<Exp,usize>,
    quadrics:&[Polynomial;8], columns:usize
)->Option<[u64;NV]> {
    let mut identity=vec![0u64;columns];
    identity[0]=1; // monomials() lists the constant first
    let normal_one=normal_form(identity,pivots);
    let pivot_index=normal_one.iter().position(|&x|x!=0)?;
    let inv=pow(normal_one[pivot_index],P-2);
    let mut values=[0u64;NV];
    for var in 0..NV {
        let mut e=[0u8;NV];e[var]=1;
        let mut coordinate=vec![0u64;columns];
        coordinate[*index.get(&e)?]=1;
        let rem=normal_form(coordinate,pivots);
        let scalar=mul(rem[pivot_index],inv);
        if rem.iter().zip(normal_one.iter()).any(|(&a,&b)|a!=mul(scalar,b)) {
            return None; // quotient still has a nonconstant dimension
        }
        values[var]=scalar;
    }
    if quadrics.iter().any(|q|eval_poly(q,&values)!=0) {
        return None; // a quotient diagnostic is not automatically a solution
    }
    Some(values)
}

fn public_macaulay(residual:Point, xs:&[u64;8], h:&[u64;9], degree:u8)->Result<Value,String> {
    let (r,s)=residual.ok_or("infinity residual excluded")?;
    if xs.contains(&r) {return Err("residual x lies on B8 factor base: exceptional chart excluded".into());}
    let start=Instant::now();
    let qs=equations(r,s,h);
    validate_quadrics(&qs,h,r,s);
    let basis=monomials(degree);
    let multipliers=monomials(degree-2);
    let index:HashMap<Exp,usize>=basis.iter().copied().enumerate().map(|(i,e)|(e,i)).collect();
    let inputs:Vec<(usize,Exp)>=(0..8).flat_map(|i| multipliers.iter().copied().map(move |e|(i,e))).collect();
    let columns=basis.len();
    let mut pivots:Vec<Option<Pivot>>=std::iter::repeat_with(||None).take(columns).collect();
    let mut ops=Ops::default();
    let mut rank=0usize;
    let mut unit:Option<Value>=None;
    for (seed,&(qindex,exponents)) in inputs.iter().enumerate() {
        let mut row=raw_row(&qs[qindex],&exponents,&index,columns);
        let mut dependencies=Vec::new();
        for col in (0..columns).rev() {
            let z=row[col];
            if z==0 {continue;}
            if let Some(ref prior)=pivots[col] {
                dependencies.push((col,z));
                ops.reductions+=1;
                for k in 0..=col {row[k]=sub(row[k],mul(z,prior.row[k]));ops.multiplications+=1;}
            } else {
                let inverse=pow(z,P-2);
                ops.inversions+=1;
                for value in row.iter_mut().take(col+1) {*value=mul(*value,inverse);ops.multiplications+=1;}
                pivots[col]=Some(Pivot{row,input:seed,inverse,dependencies});
                rank+=1;
                if col==0 {unit=Some(prove_unit(&pivots,&inputs,&qs,&index,columns));}
                break;
            }
        }
        if unit.is_some() {break;}
    }
    let extracted=if unit.is_none() {
        extract_unique_normal_form(&pivots,&index,&qs,columns)
    } else { None };
    if let Some(values)=extracted {
        let m=independent_direct_m(&values,r,s);
        assert_eq!(m,*h,"recovered coefficients do not reconstruct public factor polynomial");
    }
    Ok(json!({
        "candidate_chart_coefficients":extracted,
        "positive_solution_recovered_from_normal_forms":extracted.is_some(),
        "degree":degree,"columns":columns,"rows_generated":inputs.len(),
        "rank":rank,"unit_certificate":unit.is_some(),"replay":unit,
        "charged_wall_ns":start.elapsed().as_nanos().to_string(),
        "elimination_reductions":ops.reductions,
        "elimination_multiplications":ops.multiplications,
        "elimination_inversions":ops.inversions,
        "pivot_dense_storage_upper_bytes":rank*columns*8,
        "only_public_residual_and_x_factor_base":true,
        "complete_unknown_ecdlp_solves":0,
        "pcc":"unverified"
    }))
}
fn first_eight(ar:&mut Arithmetic)->[(u64,u64);8] {
    let mut pts=Vec::new();
    for x in 0..PRIME31.p {
        let rhs=ar.rhs(PRIME31,x);
        if let Some(y)=ar.sqrt(rhs,PRIME31.p) {
            if y==0 {continue;}
            pts.push((x,y.min(P-y)));
            if pts.len()==8 {break;}
        }
    }
    pts.try_into().expect("eight rational x points exist")
}
fn group_sum(ar:&mut Arithmetic,pts:&[Point;8])->Point {
    let mut total=None;
    for &pt in pts {total=ar.add(PRIME31,total,pt);}
    total
}
fn fixtures(ar:&mut Arithmetic,base:&[(u64,u64);8])->(Point,Vec<Point>,usize) {
    let mut all=HashSet::new();
    let mut positive=None;
    let mut count=0usize;
    for mask in 0..256usize {
        let selected:[Point;8]=std::array::from_fn(|i|{
            let (x,y)=base[i];Some((x,if mask&(1<<i)!=0 {sub(0,y)} else {y}))
        });
        let q=group_sum(ar,&selected);
        all.insert(q);
        if let Some((r,_))=q {
            if !base.iter().any(|b|b.0==r) && positive.is_none(){positive=q;}
        }
        count+=1;
    }
    let mut negatives=Vec::new();
    for k in 1..=256u64 {
        let r=ar.scalar(PRIME31,k,Some(PRIME31.g));
        if let Some((x,_))=r {
            if !base.iter().any(|b|b.0==x) && !all.contains(&r) {
                negatives.push(r);
                if negatives.len()==2 {break;}
            }
        }
    }
    assert_eq!(negatives.len(),2);
    (Some(positive.expect("a finite positive B8 fibre")),negatives,count)
}

// Test-only independent original-curve replay of a *public-only* coefficient
// solution: extract signs from E(F_p), validate every F-zero, then group-sum.
// This is NOT an input to the algebraic Macaulay solver.
fn check_public_coefficients(
    ar:&mut Arithmetic, r:Point, base:&[(u64,u64);8], coeff:&[u64;NV]
)->bool {
    let Some((rx,ry))=r else {return false;};
    let a=[coeff[0],coeff[1],coeff[2],coeff[3]];
    let v=[coeff[4],coeff[5],coeff[6],1];
    let mut signed=[None;8];
    for (i,&(x,known_y)) in base.iter().enumerate(){
        let mut xa=0u64;let mut xv=0u64;
        let mut power=1u64;
        for j in 0..4 {
            xa=add(xa,mul(a[j],power));
            xv=add(xv,mul(v[j],power));
            power=mul(power,x);
        }
        let term=mul(sub(x,rx),xa);
        let yes=add(term,mul(add(ry,known_y),xv))==0;
        let no=add(term,mul(sub(ry,known_y),xv))==0;
        if yes==no{return false;} // every factor chooses exactly one signed point
        let selected_y=if yes{known_y}else{sub(0,known_y)};
        signed[i]=Some((x,selected_y));
    }
    group_sum(ar,&signed)==r
}

fn run()->Value {
    let mut ar=Arithmetic::default();
    check_curve(PRIME31,&mut ar).unwrap();
    // Algebraic operations use Mersenne folding; check the prime's full
    // carry boundary as well as deterministic off-boundary products.
    for (a,b) in [(0,0),(1,1),(P-1,P-1),(P-1,P-2),(P-1,123456789),(1000000000,2000000000)] {
        assert_eq!(mul(a,b),((a as u128*b as u128)%(P as u128)) as u64);
    }
    let base=first_eight(&mut ar);
    let xs=base.map(|p|p.0);
    let h=factor_polynomial(&xs);
    let (positive,negative,formal_supports)=fixtures(&mut ar,&base);
    let pos=public_macaulay(positive,&xs,&h,DEGREE).unwrap();
    assert!(!pos["unit_certificate"].as_bool().unwrap(),"genuine positive fibre ruled out by a false algebraic unit");
    let extracted_positive=if let Some(v)=pos["candidate_chart_coefficients"].as_array() {
        let coeff:[u64;NV]=std::array::from_fn(|i|v[i].as_u64().unwrap());
        assert!(check_public_coefficients(&mut ar,positive,&base,&coeff),
            "public normal-form recovery fails independent signed original-curve group-sum replay");
        true
    } else {false};
    let mut negatives_json=Vec::new();
    for r in negative {
        let record=public_macaulay(r,&xs,&h,DEGREE).unwrap();
        assert_eq!(record["candidate_chart_coefficients"],Value::Null,
            "a signed-support-free negative residual has a fabricated coefficient solution");
        negatives_json.push(json!({"residual":r,"macaulay":record}));
    }
    assert!(public_macaulay(None,&xs,&h,DEGREE).is_err());
    assert!(public_macaulay(Some(base[0]),&xs,&h,DEGREE).is_err());
    json!({
        "schema":"prime31.rr9.public-macaulay.v1","p":P,"n":PRIME31.n,
        "curve_a":PRIME31.a,"curve_b":PRIME31.b,
        "base_x":xs,"h_ascending":h,"macaulay_degree_cap":DEGREE,
        "formal_signed_supports_used_only_by_test_oracle":formal_supports,
        "positive":{"residual":positive,"macaulay":pos},
        "positive_normal_form_solution_replayed_on_original_curve":extracted_positive,
        "negatives":negatives_json,
        "candidate_uses_no_signed_supports_or_secret_scalars":true,
        "negative_unit_is_sound_but_nonunit_is_inconclusive":true,
        "not_an_extractor":true,"complete_unknown_ecdlp_solves":0,
        "pcc":"unverified"
    })
}
fn main(){println!("{}",serde_json::to_string_pretty(&run()).unwrap());}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn prime31_public_rr9_b8_macaulay_gate() {
        let v=run();
        assert_eq!(v["p"],P);
        assert_eq!(v["n"],PRIME31.n);
        assert_eq!(v["formal_signed_supports_used_only_by_test_oracle"],256);
        assert_eq!(v["positive"]["macaulay"]["columns"],792);
        assert_eq!(v["positive"]["macaulay"]["rows_generated"],960);
        assert_eq!(v["positive"]["macaulay"]["unit_certificate"],false);
        if v["positive"]["macaulay"]["positive_solution_recovered_from_normal_forms"]==true {
            assert_eq!(v["positive_normal_form_solution_replayed_on_original_curve"],true);
        }
        for n in v["negatives"].as_array().unwrap(){
            assert_eq!(n["macaulay"]["columns"],792);
            if n["macaulay"]["unit_certificate"]==true {
                assert_eq!(n["macaulay"]["replay"]["identity_replayed"],true);
            }
        }
    }
}


/// Separately frozen degree-six B8 public-only Macaulay gate (PR #4549).
/// Reuses the EXACT preceding-degree candidate and finite-field implementation.
pub fn degree_six_controls() -> Value {
    let mut ar = Arithmetic::default();
    check_curve(PRIME31, &mut ar).expect("the original ordinary prime31 curve");
    let base = first_eight(&mut ar);
    let xs = base.map(|pt| pt.0);
    assert_eq!(xs, [0, 1, 2, 3, 5, 6, 8, 9], "frozen first eight x coordinates");
    let h = factor_polynomial(&xs);

    let positive: Point = Some((1_911_579_686, 548_820_096));
    let negatives: [Point;2] = [
        Some((625_037_715, 2_100_627_251)),
        Some((1_923_055_775, 307_731_458))
    ];
    // The test-only sign oracle checks the frozen fixtures, but passes no
    // sign masks, group lookup, or secret labels to public_macaulay.
    let (oracle_positive, oracle_negatives, supports) = fixtures(&mut ar, &base);
    assert_eq!(oracle_positive, positive, "frozen positive was changed");
    assert_eq!(oracle_negatives.as_slice(), &negatives, "frozen negative panel was changed");
    assert_eq!(supports, 256);
    let pos = public_macaulay(positive, &xs, &h, 6).expect("B8 positive degree-six chart");
    assert_eq!(pos["degree"], 6);
    assert_eq!(pos["columns"], 1716);
    assert_eq!(pos["rows_generated"], 2640);
    assert_eq!(pos["unit_certificate"], false, "cannot prove an actual signed solution impossible");
    let positive_replayed = if let Some(values) = pos["candidate_chart_coefficients"].as_array() {
        let coeff: [u64;NV] = std::array::from_fn(|i| values[i].as_u64().unwrap());
        assert!(check_public_coefficients(&mut ar, positive, &base, &coeff),
                "positive degree-six coefficients failed independent curve-group sum");
        true
    } else { false };
    let mut negative_results = Vec::new();
    for residual in negatives {
        let diag = public_macaulay(residual, &xs, &h, 6)
            .expect("B8 off-support negative degree-six chart");
        assert_eq!(diag["degree"], 6);
        assert_eq!(diag["columns"], 1716);
        assert_eq!(diag["rows_generated"], 2640);
        assert_eq!(diag["candidate_chart_coefficients"], Value::Null,
                   "off-support residual may not produce chart coefficients");
        if diag["unit_certificate"] == true {
            assert_eq!(diag["replay"]["identity_replayed"], true,
                       "unit replay must be exact");
        }
        negative_results.push(json!({"residual": residual, "macaulay": diag}));
    }
    json!({
        "schema": "prime31.rr9.public-macaulay-d6.v1",
        "preregistered_protocol": "RR9_PUBLIC_MACAULAY_D6_PROTOCOL_20261117.md",
        "p": P, "n": PRIME31.n, "curve_a": PRIME31.a,
        "curve_b": PRIME31.b, "generator": PRIME31.g,
        "base_x": xs, "h_ascending": h, "degree": 6,
        "candidate_receives_only_public_R_and_factor_polynomial": true,
        "positive": {"residual": positive, "macaulay": pos},
        "positive_coefficient_original_curve_replay": positive_replayed,
        "negatives": negative_results,
        "test_oracle_frozen_sign_supports": supports,
        "complete_unknown_ecdlp_solves": 0,
        "interpretation": "unit proves only absent B8 chart; nonunit without coefficients is inconclusive",
        "pcc": "unverified"
    })
}


/// Prospective #4550 test-only *positive* certificate for the degree-9
/// homogeneous Macaulay row-space obstruction. No negative-rank computation,
/// support-free extractor or unknown-log recovery is implemented here.
/// The signed support is restricted to the INDEPENDENT TEST ORACLE. The
/// eight public coefficient quadrics are unchanged from D5 and D6.
pub fn projective_degree_nine_witness_controls() -> Value {
    let start=Instant::now();
    let mut ar=Arithmetic::default();
    check_curve(PRIME31,&mut ar).unwrap();
    let base=first_eight(&mut ar);
    let xs=base.map(|pt|pt.0);
    assert_eq!(xs,[0,1,2,3,5,6,8,9]);
    let h=factor_polynomial(&xs);
    let positive:Point=Some((1_911_579_686,548_820_096));
    let negatives:[Point;2]=[
        Some((625_037_715,2_100_627_251)),
        Some((1_923_055_775,307_731_458))
    ];
    let (oracle_positive,oracle_negatives,formal_supports)=fixtures(&mut ar,&base);
    assert_eq!(oracle_positive,positive);
    assert_eq!(oracle_negatives.as_slice(),&negatives);
    assert_eq!(formal_supports,256);
    let (r,s)=positive.unwrap();
    assert!(!xs.contains(&r));

    // Test-oracle ONLY: find the actual eight signed elliptic-curve leaves
    // behind this *frozen positive* public residual. Never feed the sign mask
    // or the resulting coefficients to a public-only unknown-R extractor.
    let mut support=None;
    for mask in 0..256u16 {
        let chosen:[Point;8]=std::array::from_fn(|i|{
            let (x,y)=base[i];
            Some((x,if mask&(1u16<<i)==0 {y} else {sub(0,y)}))
        });
        if group_sum(&mut ar,&chosen)==positive {
            support=Some((mask,chosen));
            break;
        }
    }
    let (_test_only_sign_mask,signed)=support.expect("positive sign support missing");
    let mut rows=[[0u64;NV+1];8];
    for (i,&point) in signed.iter().enumerate() {
        let (x,y)=point.expect("finite signed leaf");
        let xr=sub(x,r);
        let ys=add(y,s);
        let mut xpow=1u64;
        for j in 0..4 {
            rows[i][j]=mul(xr,xpow);
            if j<3 {rows[i][4+j]=mul(ys,xpow);}
            if j==3 {rows[i][NV]=sub(0,mul(ys,xpow));}
            xpow=mul(xpow,x);
        }
    }
    // Eight equations, seven unknown normalized coefficients. Elementary
    // finite-field RREF with exact consistency / uniqueness checks.
    let mut next_row=0usize;
    for column in 0..NV {
        let pivot=(next_row..8).find(|&i|rows[i][column]!=0)
            .expect("frozen signed chart has deficient rank");
        rows.swap(next_row,pivot);
        let inv=pow(rows[next_row][column],P-2);
        for k in column..=NV {rows[next_row][k]=mul(rows[next_row][k],inv);}
        for i in 0..8 {
            if i==next_row {continue;}
            let factor=rows[i][column];
            if factor==0 {continue;}
            for k in column..=NV {
                rows[i][k]=sub(rows[i][k],mul(factor,rows[next_row][k]));
            }
        }
        next_row+=1;
    }
    assert_eq!(next_row,NV);
    for row in rows.iter().skip(NV) {
        assert!(row[..NV].iter().all(|&z|z==0),"unreduced overdetermined row");
        assert_eq!(row[NV],0,"signed chart has no normalized solution");
    }
    let coeff:[u64;NV]=std::array::from_fn(|i|rows[i][NV]);
    let quadrics=equations(r,s,&h);
    assert!(quadrics.iter().all(|q|eval_poly(q,&coeff)==0),
        "positive public coefficient equations not solved");
    assert_eq!(independent_direct_m(&coeff,r,s),h);
    assert!(check_public_coefficients(&mut ar,positive,&base,&coeff),
        "positive chart fails independent original-curve replay");

    // Homogenize eight affine quadrics in seven variables by a formal h.
    // Columns of degree nine in (7 coefficients,h) correspond bijectively
    // to affine monomials in coefficients of total degree <=9. Evaluation
    // at (coefficients,h=1) is a NONZERO right-null vector of the 27456
    // public Macaulay rows. Reassemble and test every sparse row, rather
    // than merely observing q(coeff)=0.
    let basis=monomials(9);
    assert_eq!(basis.len(),11_440);
    let multipliers=monomials(7);
    assert_eq!(multipliers.len(),3_432);
    let idx:HashMap<Exp,usize>=basis.iter().copied().enumerate()
        .map(|(i,e)|(e,i)).collect();
    let mut powers=[[1u64;10];NV];
    for i in 0..NV {
        for d in 1..=9 {powers[i][d]=mul(powers[i][d-1],coeff[i]);}
    }
    let evals:Vec<u64>=basis.iter().map(|ex|{
        let mut z=1u64;
        for j in 0..NV {z=mul(z,powers[j][ex[j] as usize]);}
        z
    }).collect();
    assert_eq!(evals[*idx.get(&[0u8;NV]).unwrap()],1,
        "homogeneous h^9 evaluation must be nonzero");
    let mut nonzero_rows=0usize;
    let mut sparse_terms=0u64;
    for q in &quadrics {
        for e in &multipliers {
            let mut dot=0u64;
            for (qe,&coefficient) in q {
                let combined=add_exp(e,qe);
                let column=*idx.get(&combined)
                    .expect("degree-nine Macaulay monomial not indexed");
                dot=add(dot,mul(coefficient,evals[column]));
                sparse_terms+=1;
            }
            assert_eq!(dot,0,"the positive root must annihilate every D9 row");
            nonzero_rows+=1;
        }
    }
    assert_eq!(nonzero_rows,27_456);

    // Only bounded positive-witness timing; no D9 rank or negative
    // certificate has been computed and no cost claim is appropriate.
    json!({
        "schema":"prime31.rr9.projective-d9-positive-witness.v1",
        "protocol":"RR9_PROJECTIVE_D9_PROTOCOL_20261117.md",
        "p":P,"n":PRIME31.n,"curve_a":PRIME31.a,"curve_b":PRIME31.b,
        "base_x":xs,"positive_residual":positive,"negative_controls":negatives,
        "positive_chart_coefficients_test_oracle_only":coeff,
        "oracle_supports_test_only":formal_supports,
        "positive_chart_equations_replay":true,
        "positive_original_curve_replay":true,
        "degree":9,"macaulay_columns":basis.len(),
        "macaulay_rows_checked":nonzero_rows,
        "sparse_row_terms_checked":sparse_terms,
        "evaluation_at_h_equals_one_nonzero":true,
        "degree_nine_rank_upper_bound_for_positive":basis.len()-1,
        "negative_degree_nine_rank":Value::Null,
        "all_D9_rows_annihilated":true,
        "witness_only_wall_ns":start.elapsed().as_nanos().to_string(),
        "complete_unknown_ecdlp_solves":0,
        "public_only_non_enumerative_extractor_implemented":false,
        "pcc":"unverified"
    })
}


// Public-only but explicitly sign-enumerative B8 barycentric extractor (#4556).
// Shares the original prime31 arithmetic, factor polynomial and independent
// point-group replay. This is an exact algebraic control, NOT a non-generic
// asymptotic acceleration or a full B31 discrete-logarithm solver.
#[path = "prime31_rr9_barycentric_core.rs"]
mod barycentric_core;
pub fn public_b8_barycentric_controls() -> Value { barycentric_core::run() }

// #4604: Matrix-free RR9 D9 full-rank negative gate, no supports in candidate.
#[path = "prime31_rr9_d9_blackbox_core.rs"]
mod d9_blackbox_core;
pub fn rr9_d9_blackbox_controls(index: usize) -> Value { d9_blackbox_core::run(index) }
pub fn rr9_d9_blackbox_smoke() -> Value { d9_blackbox_core::smoke() }
