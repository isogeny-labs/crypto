//! Public-residual-only exterior-4 RR9 algebraic candidate, original Prime31.
//! No EC additions or EC half-sum tables occur inside extract().
//! This is exhaustive algebraic pairwork, NOT a faster complete ECDLP solver.
//! Protocol: ../rr9/RR9_EXTERIOR4_PUBLIC_PROTOCOL_20261119.md
use prime31_algebraic_ic::{Arithmetic, Point, PRIME31};
use serde_json::json;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

const P: u64 = 2_147_483_647;
const G: Point = Some((625_037_715, 2_100_627_251));
const W: [u64; 8] = [1, 3, 5, 7, 11, 13, 17, 19];
const C: usize = 70;
type Word = [u8; 4];
type MinorVector = [u64; C];
type Row = [u64; 8];

#[inline] fn add(a: u64,b: u64)->u64 {
    let v=a+b;
    if v>=P {v-P} else {v}
}
#[inline] fn sub(a:u64,b:u64)->u64 {
    if a>=b {a-b} else {a+P-b}
}
#[inline] fn neg(a:u64)->u64 {if a==0 {0} else {P-a}}
#[inline] fn mul(a:u64,b:u64)->u64 {
    let v=a*b; // a,b < 2^31: no overflow
    let t=(v&P)+(v>>31);
    let t=(t&P)+(t>>31);
    if t>=P {t-P} else {t}
}
fn pow(mut a:u64,mut e:u64)->u64{
    let mut r=1u64;
    while e!=0 {
        if e&1!=0 {r=mul(r,a);}
        a=mul(a,a); e>>=1;
    }
    r
}
fn inv(a:u64)->u64{assert_ne!(a,0);pow(a,P-2)}
fn row(q:Point, r:Point)->Row{
    let (x,y)=q.expect("non-infinite signed slot leaf");
    let (rx,ry)=r.expect("non-infinite public residual");
    let u=sub(x,rx); let v=add(y,ry);
    let mut out=[0u64;8]; let mut power=1u64;
    for j in 0..4 {
        out[j]=mul(u,power);
        out[4+j]=mul(v,power);
        power=mul(power,x);
    }
    out
}
fn det4(rows:&[Row;4], cs:[usize;4])->u64{
    let mut a=[[0u64;4];4];
    for i in 0..4 {for j in 0..4 {a[i][j]=rows[i][cs[j]];}}
    let mut ans=1u64;
    for k in 0..4 {
        let Some(t)=(k..4).find(|&i|a[i][k]!=0) else{return 0;};
        if t!=k {a.swap(k,t);ans=neg(ans);}
        let pivot=a[k][k];
        ans=mul(ans,pivot);
        let ip=inv(pivot);
        for i in k+1..4 {
            let factor=mul(a[i][k],ip);
            for j in k+1..4 {
                a[i][j]=sub(a[i][j],mul(factor,a[k][j]));
            }
            a[i][k]=0;
        }
    }
    ans
}
fn det8(mut a:[Row;8])->u64{
    let mut ans=1;
    for k in 0..8{
        let Some(t)=(k..8).find(|&i|a[i][k]!=0) else{return 0;};
        if t!=k {a.swap(k,t);ans=neg(ans);}
        let pivot=a[k][k];ans=mul(ans,pivot);
        let ip=inv(pivot);
        for i in k+1..8 {
            let factor=mul(a[i][k],ip);
            for j in k+1..8 {a[i][j]=sub(a[i][j],mul(factor,a[k][j]));}
            a[i][k]=0;
        }
    }
    ans
}
// Generalized Laplace for a 4+4 split of eight target-conditioned rows.
// Sign = (-1)^sum(zero-based column indices), since the row sum is 10.
fn columns()->Vec<([usize;4],[usize;4],bool)>{
    let mut cs=Vec::with_capacity(C);
    for i in 0..8{for j in i+1..8{for k in j+1..8{for l in k+1..8{
        let lhs=[i,j,k,l];
        let mut rhs=[0usize;4];let mut n=0;
        for t in 0..8 {
            if !lhs.contains(&t) {rhs[n]=t;n+=1;}
        }
        assert_eq!(n,4);
        cs.push((lhs,rhs,(i+j+k+l)%2!=0));
    }}}}
    assert_eq!(cs.len(),70);cs
}
fn half_vectors(
    rows:&[Vec<Row>], begin:usize, cs:&[([usize;4],[usize;4],bool)]
)->Vec<(Word,MinorVector)>{
    let b=rows[0].len();
    let mut out=Vec::with_capacity(b.pow(4));
    for a in 0..b {for c in 0..b {for d in 0..b {for e in 0..b {
        let rr=[rows[begin][a],rows[begin+1][c],
                rows[begin+2][d],rows[begin+3][e]];
        let mut v=[0u64;C];
        for (i,&(cols,comp,sign)) in cs.iter().enumerate(){
            let mut z=det4(&rr, if begin==0 {cols} else {comp});
            if begin!=0 && sign {z=neg(z);}
            v[i]=z;
        }
        out.push(([a as u8,c as u8,d as u8,e as u8],v));
    }}}}
    out
}
#[inline] fn contract(a:&MinorVector,b:&MinorVector)->u64{
    let mut r=0;
    for i in 0..C {r=add(r,mul(a[i],b[i]));}
    r
}
fn distinct_and_not_target(word:[u8;8], slots:&[Vec<Point>], target:Point)->bool{
    let mut seen=HashSet::with_capacity(8);
    let (rx,ry)=target.expect("public target cannot be infinity");
    for j in 0..8 {
        let q=slots[j][word[j] as usize].expect("weighted leaf must be finite");
        if !seen.insert(q) || (q.0==rx && add(q.1,ry)==0) {return false;}
    }
    true
}
struct Extraction {
    witness:Option<[u8;8]>,pairs:u64, halves:usize, elapsed_ms:f64
}
// Function sees no scalar of public R, no secret/planted words, and no group-sum oracle.
fn extract(slots:&[Vec<Point>],target:Point)->Extraction{
    let begin=Instant::now();
    let rows:Vec<Vec<Row>>=slots.iter().map(|s|s.iter().map(|&q|row(q,target)).collect()).collect();
    let cs=columns();
    let left=half_vectors(&rows,0,&cs);
    let right=half_vectors(&rows,4,&cs);
    let mut pairs=0u64;let mut found=None;
    'outer:for (lw,lv) in &left{
        for (rw,rv) in &right {
            pairs+=1;
            if contract(lv,rv)==0 {
                let w=[lw[0],lw[1],lw[2],lw[3],rw[0],rw[1],rw[2],rw[3]];
                if distinct_and_not_target(w,slots,target){found=Some(w);break 'outer;}
            }
        }
    }
    Extraction{witness:found,pairs,halves:left.len(),elapsed_ms:begin.elapsed().as_secs_f64()*1000.0}
}
fn canonical_base(ar:&mut Arithmetic,b:usize)->Vec<Point>{
    let mut out=Vec::new();
    for x in 0..10000 {
        let rhs=ar.rhs(PRIME31,x);
        if let Some(y)=ar.sqrt(rhs,P){
            if y!=0{
                let q=Some((x,y.min(P-y)));
                assert!(ar.on_curve(PRIME31,q));
                out.push(q);
                if out.len()==b{break;}
            }
        }
    }
    assert_eq!(out.len(),b);out
}
fn supports(ar:&mut Arithmetic,base:&[Point])->Vec<Vec<Point>>{
    let b=base.len();let mut slots=Vec::with_capacity(8);
    let mut abscissae=HashSet::new();
    for &w in &W{
        let mut pos=Vec::with_capacity(b);
        for &q in base {
            let t=ar.scalar(PRIME31,w,q);
            let (x,_)=t.expect("nonzero slot point");
            assert!(abscissae.insert(x),"weighted x overlap: invalid fixture");
            pos.push(t);
        }
        let mut v=pos.clone();
        v.extend(pos.into_iter().map(|q|ar.neg(PRIME31,q)));
        slots.push(v);
    }
    slots
}
fn replay(ar:&mut Arithmetic, slots:&[Vec<Point>], word:[u8;8])->Point{
    let mut sum=None;
    for j in 0..8 {sum=ar.add(PRIME31,sum,slots[j][word[j] as usize]);}
    sum
}
fn truth_only_generic_grader(ar:&mut Arithmetic,slots:&[Vec<Point>],target:Point)->bool{
    // Strictly out-of-band grading; extract() NEVER calls this function.
    let b=slots[0].len();let mut right=HashMap::new();
    for i in 0..b {for j in 0..b {for k in 0..b {for l in 0..b{
        let mut s=None;
        for (slot,t) in [(4,i),(5,j),(6,k),(7,l)] {
            s=ar.add(PRIME31,s,slots[slot][t]);
        }
        right.entry(s).or_insert(());
    }}}}
    for i in 0..b {for j in 0..b {for k in 0..b {for l in 0..b{
        let mut s=None;
        for (slot,t) in [(0,i),(1,j),(2,k),(3,l)] {
            s=ar.add(PRIME31,s,slots[slot][t]);
        }
        let wanted=ar.add(PRIME31,target,ar.neg(PRIME31,s));
        if right.contains_key(&wanted){return true;}
    }}}}
    false
}
fn main(){
    let mut ar=Arithmetic::default();
    let start=Instant::now();
    let base=canonical_base(&mut ar,4);
    assert_eq!(base.iter().map(|q|q.unwrap().0).collect::<Vec<_>>(),vec![0,1,2,3]);
    let slots=supports(&mut ar,&base);
    let setup_ms=start.elapsed().as_secs_f64()*1000.0;
    let planted=replay(&mut ar,&slots,[0u8;8]);
    assert!(planted.is_some() && planted!=G);
    let mut result=Vec::new();
    for (name,public_target) in [("planted_B4",planted),("G_unplanted_B4",G)] {
        let x=extract(&slots,public_target);
        let truth=truth_only_generic_grader(&mut ar,&slots,public_target);
        assert_eq!(x.witness.is_some(),truth,"algebraic and independent EC graders disagree");
        if let Some(word)=x.witness{
            assert_eq!(replay(&mut ar,&slots,word),public_target,"false algebraic relation");
        }
        result.push(json!({
            "case":name,"target":public_target,
            "witness":x.witness,"independent_truth_exists":truth,
            "algebraic_pairs":x.pairs,"field_coordinate_product_muls":x.pairs*70,
            "half_table_size":x.halves,
            "candidate_public_only":true,"generic_group_table_in_candidate":false,
            "candidate_wall_ms_including_wedge_tables":x.elapsed_ms
        }));
    }
    println!("{}",serde_json::to_string_pretty(&json!({
        "scope":"B4 original curve, exhaustive algebraic pairwork; not complete ECDLP",
        "p":P,"group_order":2_147_446_831u64,"weights":W,
        "base_x":[0,1,2,3],"public_support_setup_ms":setup_ms,
        "unit_contraction_products_per_pair":70,
        "B4_full_pairs":8u64.pow(8),"B8_full_pairs":16u64.pow(8),
        "B8_full_coordinate_products":16u64.pow(8)*70,
        "B8_1024_full_coordinate_products":16u128.pow(8)*70*1024,
        "separate_truth_grader_excluded_from_candidate_time":true,
        "pcc":"unverified","cases":result
    })).unwrap());
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]fn mersenne_field_arithmetic(){
        for a in 1..400u64{
            assert_eq!(mul(a,inv(a)),1);
            for b in 1..400u64{
                assert_eq!(mul(a,b),((a as u128*b as u128)%P as u128)as u64);
            }
        }
    }
    #[test]fn laplace_exterior4_equals_det8(){
        let cs=columns();
        let mut state=0x173def1u64;
        for _ in 0..12 {
            let mut all=[[0u64;8];8];
            for r in &mut all {for x in r {state=state.wrapping_mul(6364136223846793005).wrapping_add(1);*x=(state>>32)%P;}}
            let a=[all[0],all[1],all[2],all[3]];
            let b=[all[4],all[5],all[6],all[7]];
            let mut sum=0;
            for &(left,right,sign) in &cs{
                let term=mul(det4(&a,left),det4(&b,right));
                sum=add(sum,if sign{neg(term)}else{term});
            }
            assert_eq!(sum,det8(all));
        }
    }
}


// Algebraic 4-plane graph chart for the same target-conditioned RR9 predicate.
// For H=[A|B], normalize to [I|A^{-1}B] on the open chart det(A)!=0.
// The two 4-planes intersect iff det(M_right-M_left)=0.
// This variant is still EXHAUSTIVE and not a complete non-generic DLP solver.
type Graph4 = [[u64;4];4];

fn graph_chart(mut rows:[Row;4])->Option<(Graph4,u64)> {
    let mut det=1u64;
    for k in 0..4 {
        let t=(k..4).find(|&i|rows[i][k]!=0)?;
        if t!=k {rows.swap(k,t);det=neg(det);}
        let pivot=rows[k][k];det=mul(det,pivot);
        let ip=inv(pivot);
        for j in k..8 {rows[k][j]=mul(rows[k][j],ip);}
        for i in 0..4 {
            if i==k {continue;}
            let f=rows[i][k];
            if f==0 {continue;}
            for j in k+1..8 {
                rows[i][j]=sub(rows[i][j],mul(f,rows[k][j]));
            }
            rows[i][k]=0;
        }
    }
    let mut out=[[0u64;4];4];
    for i in 0..4 {for j in 0..4 {out[i][j]=rows[i][4+j];}}
    Some((out,det))
}

// Two-by-two complementary-minor determinant of a 4x4 matrix.
// 6*(2+2+1) = 30 field multiplications, zero inversions.
#[inline]
fn det_graph_difference(l:&Graph4,r:&Graph4)->u64 {
    let mut d=[[0u64;4];4];
    for i in 0..4 {for j in 0..4 {d[i][j]=sub(r[i][j],l[i][j]);}}
    const TWO:[[usize;2];6]=[[0,1],[0,2],[0,3],[1,2],[1,3],[2,3]];
    let mut ans=0;
    for &[i,j] in &TWO {
        let mut other=[0usize;2]; let mut n=0;
        for z in 0..4 {
            if z!=i && z!=j {other[n]=z;n+=1;}
        }
        let [k,h]=other;
        let a=sub(mul(d[0][i],d[1][j]),mul(d[0][j],d[1][i]));
        let b=sub(mul(d[2][k],d[3][h]),mul(d[2][h],d[3][k]));
        let term=mul(a,b);
        // Laplace sign (-1)^(0+1+i+j).
        ans=if (i+j)%2==1 {add(ans,term)} else {sub(ans,term)};
    }
    ans
}

fn graph_halves(rows:&[Vec<Row>],start:usize)->Option<Vec<(Word,Graph4,u64)>> {
    let b=rows[0].len();
    let mut result=Vec::with_capacity(b.pow(4));
    for a in 0..b {for c in 0..b {for d in 0..b {for e in 0..b {
        let four=[rows[start][a],rows[start+1][c],
                  rows[start+2][d],rows[start+3][e]];
        // Explicitly fail the whole candidate on chart singularity: no false
        // "negative" report may silently discard singular four-plane fibres.
        let (m,det)=graph_chart(four)?;
        result.push(([a as u8,c as u8,d as u8,e as u8],m,det));
    }}}}
    Some(result)
}

struct GraphExtraction {
    witness:Option<[u8;8]>,
    pairs:u64,
    halves:usize,
    elapsed_ms:f64
}

fn extract_graph(slots:&[Vec<Point>],target:Point)->Option<GraphExtraction> {
    let begin=Instant::now();
    let rows:Vec<Vec<Row>>=slots.iter()
        .map(|s|s.iter().map(|&q|row(q,target)).collect()).collect();
    let left=graph_halves(&rows,0)?;
    let right=graph_halves(&rows,4)?;
    let mut pairs=0u64;let mut found=None;
    'outer:for (lw,lm,_) in &left {
        for (rw,rm,_) in &right {
            pairs+=1;
            if det_graph_difference(lm,rm)==0 {
                let w=[lw[0],lw[1],lw[2],lw[3],
                       rw[0],rw[1],rw[2],rw[3]];
                if distinct_and_not_target(w,slots,target) {
                    found=Some(w);break 'outer;
                }
            }
        }
    }
    Some(GraphExtraction{
        witness:found,pairs,halves:left.len(),
        elapsed_ms:begin.elapsed().as_secs_f64()*1000.0
    })
}

// Called from prime31_rr9_graph_chart.rs; reuses every original EC operation.
pub(crate) fn graph_chart_main() {
    let mut ar=Arithmetic::default();
    let start=Instant::now();
    let base=canonical_base(&mut ar,4);
    let slots=supports(&mut ar,&base);
    let setup_ms=start.elapsed().as_secs_f64()*1000.0;
    let planted=replay(&mut ar,&slots,[0u8;8]);
    assert!(planted.is_some() && planted!=G);
    let mut result=Vec::new();
    for (name,public_target) in [("planted_B4",planted),("G_unplanted_B4",G)] {
        let x=extract_graph(&slots,public_target)
            .expect("graph chart singular: do not silently discard exceptional fibre");
        let truth=truth_only_generic_grader(&mut ar,&slots,public_target);
        assert_eq!(x.witness.is_some(),truth,
            "graph and independently graded EC relation disagree");
        if let Some(word)=x.witness {
            assert_eq!(replay(&mut ar,&slots,word),public_target);
        }
        result.push(json!({
            "case":name,"target":public_target,
            "witness":x.witness,"independent_truth_exists":truth,
            "algebraic_pairs":x.pairs,
            "field_coordinate_product_muls":x.pairs*30,
            "half_table_size":x.halves,
            "candidate_public_only":true,"generic_group_table_in_candidate":false,
            "candidate_wall_ms_including_chart_tables":x.elapsed_ms
        }));
    }
    println!("{}",serde_json::to_string_pretty(&json!({
        "scope":"B4 original curve graph-chart exact determinant, still exhaustive",
        "p":P,"group_order":2_147_446_831u64,"weights":W,
        "base_x":[0,1,2,3],"public_support_setup_ms":setup_ms,
        "unit_contraction_products_per_pair":30,
        "B4_full_pairs":8u64.pow(8),"B8_full_pairs":16u64.pow(8),
        "B8_full_chart_products":16u64.pow(8)*30,
        "B8_1024_full_chart_products":16u128.pow(8)*30*1024,
        "B4_half_graph_storage_bytes":2u64*8u64.pow(4)*16*8,
        "B4_half_exterior_storage_bytes":2u64*8u64.pow(4)*70*8,
        "B8_half_graph_storage_bytes":2u64*16u64.pow(4)*16*8,
        "B8_half_exterior_storage_bytes":2u64*16u64.pow(4)*70*8,
        "separate_truth_grader_excluded_from_candidate_time":true,
        "pcc":"unverified","cases":result
    })).unwrap());
}

#[cfg(test)]
mod graph_tests {
    use super::*;
    #[test]
    fn synthetic_graph_schur_exact_residues() {
        let mut state=0x77baff03u64;
        let mut verified=0usize;
        for _ in 0..100 {
            let mut all=[[0u64;8];8];
            for rr in &mut all {
                for x in rr {
                    state=state.wrapping_mul(6364136223846793005).wrapping_add(1);
                    *x=(state>>32)%P;
                }
            }
            let l=[all[0],all[1],all[2],all[3]];
            let r=[all[4],all[5],all[6],all[7]];
            if let (Some((ml,dl)),Some((mr,dr)))=(graph_chart(l),graph_chart(r)) {
                assert_eq!(det8(all),mul(mul(dl,dr),det_graph_difference(&ml,&mr)));
                verified+=1;
            }
        }
        assert_eq!(verified,100);
    }
    #[test]
    fn original_prime31_positive_negative_chart_checks() {
        let mut ar=Arithmetic::default();
        let base=canonical_base(&mut ar,4);
        let slots=supports(&mut ar,&base);
        let planted=replay(&mut ar,&slots,[0u8;8]);
        for target in [planted,G] {
            let rows:Vec<Vec<Row>>=slots.iter()
                .map(|s|s.iter().map(|&q|row(q,target)).collect()).collect();
            let left=graph_halves(&rows,0).unwrap();
            let right=graph_halves(&rows,4).unwrap();
            assert_eq!(left.len(),8usize.pow(4));
            assert_eq!(right.len(),8usize.pow(4));
            for l in [0,1,7,101,2047,4095] {
                for r in [0,1,17,128,3071,4095] {
                    let la=left[l].0;let ra=right[r].0;
                    let a=[rows[0][la[0] as usize],rows[1][la[1] as usize],
                           rows[2][la[2] as usize],rows[3][la[3] as usize],
                           rows[4][ra[0] as usize],rows[5][ra[1] as usize],
                           rows[6][ra[2] as usize],rows[7][ra[3] as usize]];
                    let want=det8(a);
                    let got=mul(mul(left[l].2,right[r].2),
                                det_graph_difference(&left[l].1,&right[r].1));
                    assert_eq!(got,want);
                }
            }
        }
    }
}
