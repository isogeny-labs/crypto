//! Independent point, polynomial, exact Macaulay and rank replay. No timings.
#![allow(dead_code)]
#[path = "../../../../src/hash/sha256.rs"]
pub mod sha_backend;
pub mod hash {
    pub use crate::sha_backend as sha256;
}
#[path = "../../../../src/cryptanalysis/identity_certificate.rs"]
mod identity_certificate;
use identity_certificate::{Expr as Ex, IdentityCertificate, IdentityStatement, ProofRef, Verdict};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};
type I = i64;
type Point = Option<(I, I)>;
type Poly = BTreeMap<(usize, usize), I>;
fn red(x: I, p: I) -> I {
    x.rem_euclid(p)
}
fn pow(mut a: I, mut n: usize, p: I) -> I {
    let mut r = 1;
    while n > 0 {
        if n & 1 == 1 {
            r = red(r * a, p);
        }
        a = red(a * a, p);
        n >>= 1;
    }
    r
}
fn inv(a: I, p: I) -> I {
    let (mut r, mut rr, mut s, mut ss) = (p, red(a, p), 0, 1);
    while rr != 0 {
        let q = r / rr;
        (r, rr) = (rr, r - q * rr);
        (s, ss) = (ss, s - q * ss);
    }
    assert_eq!(r, 1, "noninvertible denominator");
    red(s, p)
}
fn sha(bytes: &[u8]) -> String {
    hex::encode(sha_backend::sha256(bytes))
}
fn n(v: &Value) -> I {
    v.as_i64().unwrap()
}
fn ints(v: &Value) -> Vec<I> {
    v.as_array().unwrap().iter().map(n).collect()
}
fn point_json(q: Point) -> Value {
    match q {
        None => json!([-1, -1]),
        Some((x, y)) => json!([x, y]),
    }
}
#[derive(Clone, Copy)]
struct Curve {
    p: I,
    a: I,
    b: I,
}
impl Curve {
    fn contains(self, q: Point) -> bool {
        q.is_none_or(|(x, y)| red(y * y - x * x * x - self.a * x - self.b, self.p) == 0)
    }
    fn points(self) -> Vec<Point> {
        let mut out = vec![None];
        for x in 0..self.p {
            for y in 0..self.p {
                let q = Some((x, y));
                if self.contains(q) {
                    out.push(q);
                }
            }
        }
        out
    }
    fn neg(self, q: Point) -> Point {
        q.map(|(x, y)| (x, red(-y, self.p)))
    }
    fn add(self, q: Point, r: Point) -> Point {
        let (Some((x, y)), Some((u, v))) = (q, r) else {
            return q.or(r);
        };
        if x == u && red(y + v, self.p) == 0 {
            return None;
        }
        let m = if x == u {
            red((3 * x * x + self.a) * inv(2 * y, self.p), self.p)
        } else {
            red((v - y) * inv(u - x, self.p), self.p)
        };
        let xx = red(m * m - x - u, self.p);
        Some((xx, red(m * (x - xx) - y, self.p)))
    }
    fn mul(self, z: I, q: Point) -> Point {
        // Linear chain deliberately differs from the producer's binary chain.
        let q = if z < 0 { self.neg(q) } else { q };
        let mut out = None;
        for _ in 0..z.abs() {
            out = self.add(out, q);
        }
        out
    }
    fn generator(self, r: I) -> Point {
        self.points()
            .into_iter()
            .skip(1)
            .find(|&q| self.mul(r, q).is_none())
            .unwrap()
    }
    fn xs(self, q: Point, r: I) -> Vec<I> {
        (1..r)
            .map(|i| self.mul(i, q).unwrap().0)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
    }
    fn reps(self, xs: &[I]) -> Vec<Point> {
        xs.iter()
            .map(|&x| {
                (0..self.p)
                    .map(|y| Some((x, y)))
                    .find(|&q| self.contains(q))
                    .unwrap()
            })
            .collect()
    }
}
fn scale(c: Curve, q: Point, s: I) -> Point {
    q.map(|(x, y)| (red(s * s * x, c.p), red(s * s * s * y, c.p)))
}
fn primes(mut x: I) -> Vec<I> {
    let mut ps = vec![];
    let mut p = 2;
    while p * p <= x {
        if x % p == 0 {
            ps.push(p);
            while x % p == 0 {
                x /= p;
            }
        }
        p += 1;
    }
    if x > 1 {
        ps.push(x);
    }
    ps
}
fn nonsquare(p: I) -> I {
    (1..p)
        .find(|&d| !(0..p).any(|s| red(s * s, p) == d))
        .unwrap()
}
fn gcd(mut a: I, mut b: I) -> I {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}
fn subs(xs: &[I], b: usize) -> Vec<Vec<I>> {
    let mut out = vec![];
    for mask in 0..1usize << xs.len() {
        if mask.count_ones() as usize == b {
            out.push(
                xs.iter()
                    .enumerate()
                    .filter(|(i, _)| mask & (1 << i) != 0)
                    .map(|(_, x)| *x)
                    .collect(),
            );
        }
    }
    out.sort();
    out
}
fn add_term(f: &mut Poly, m: (usize, usize), c: I, p: I) {
    let c = red(f.get(&m).copied().unwrap_or(0) + c, p);
    if c == 0 {
        f.remove(&m);
    } else {
        f.insert(m, c);
    }
}
fn s3_poly(c: Curve, z: I) -> Poly {
    // Explicit coefficient table, independent of the producer's convolutions.
    let mut f = Poly::new();
    for (m, v) in [
        ((2, 2), 1),
        ((2, 1), -2 * z),
        ((1, 2), -2 * z),
        ((2, 0), z * z),
        ((1, 1), -2 * z * z - 2 * c.a),
        ((0, 2), z * z),
        ((1, 0), -2 * c.a * z - 4 * c.b),
        ((0, 1), -2 * c.a * z - 4 * c.b),
        ((0, 0), c.a * c.a - 4 * c.b * z),
    ] {
        add_term(&mut f, m, v, c.p);
    }
    f
}
fn s3_value(c: Curve, x: I, y: I, z: I) -> I {
    let t = x * y + x * z + y * z - c.a;
    red(t * t - 4 * (x + y + z) * (x * y * z + c.b), c.p)
}
fn base_poly(xs: &[I], variable: usize, p: I) -> Poly {
    let mut coeff = vec![1];
    for &x in xs {
        let mut next = vec![0; coeff.len() + 1];
        for (i, &a) in coeff.iter().enumerate() {
            next[i] = red(next[i] - x * a, p);
            next[i + 1] = red(next[i + 1] + a, p);
        }
        coeff = next;
    }
    coeff
        .into_iter()
        .enumerate()
        .filter(|(_, a)| *a != 0)
        .map(|(i, a)| (if variable == 0 { (i, 0) } else { (0, i) }, a))
        .collect()
}
fn monomials(d: usize) -> Vec<(usize, usize)> {
    (0..=d)
        .flat_map(|k| (0..=k).map(move |i| (i, k - i)))
        .collect()
}
fn rref(mut rows: Vec<Vec<I>>, cols: usize, p: I) -> (Vec<Vec<I>>, Vec<usize>) {
    // Batch RREF with full elimination and ascending monomial degrees; neither
    // order nor reduction strategy is shared with the incremental C++ basis.
    let mut pivots = vec![];
    for j in 0..cols {
        let k = pivots.len();
        let Some(i) = (k..rows.len()).find(|&i| rows[i][j] != 0) else {
            continue;
        };
        rows.swap(k, i);
        let t = inv(rows[k][j], p);
        for x in &mut rows[k] {
            *x = red(*x * t, p);
        }
        for i in 0..rows.len() {
            if i != k {
                let t = rows[i][j];
                if t != 0 {
                    for l in j..cols {
                        rows[i][l] = red(rows[i][l] - t * rows[k][l], p);
                    }
                }
            }
        }
        pivots.push(j);
    }
    rows.truncate(pivots.len());
    (rows, pivots)
}
fn solve_linear(equations: Vec<Vec<I>>, vars: usize, p: I) -> Vec<I> {
    let (rows, pivots) = rref(equations, vars + 1, p);
    assert!(
        !pivots.contains(&vars),
        "linear certificate system inconsistent"
    );
    let mut solution = vec![0; vars];
    for (row, &j) in rows.iter().zip(&pivots) {
        solution[j] = row[vars];
    }
    solution
}

// The above arithmetic is inherited verbatim from the independently written
// prior Rust auditor. No C++ producer field/group/polynomial code is shared.
fn degree(f: &Poly) -> usize {
    f.keys().map(|(i, j)| i + j).max().unwrap_or(0)
}
fn ev(f: &Poly, x: I, y: I, p: I) -> I {
    f.iter().fold(0, |s, (&(i, j), &c)| {
        red(s + c * pow(x, i, p) * pow(y, j, p), p)
    })
}
fn divide_base(mut f: Poly, xs: &[I], p: I) -> Poly {
    let (s, t) = (red(xs[0] + xs[1], p), red(xs[0] * xs[1], p));
    for variable in 0..2 {
        loop {
            let found = f
                .iter()
                .rev()
                .find(|((i, j), _)| if variable == 0 { *i >= 2 } else { *j >= 2 })
                .map(|(&m, &c)| (m, c));
            let Some(((i, j), c)) = found else { break };
            f.remove(&(i, j));
            let (u, v) = if variable == 0 {
                ((i - 1, j), (i - 2, j))
            } else {
                ((i, j - 1), (i, j - 2))
            };
            add_term(&mut f, u, s * c, p);
            add_term(&mut f, v, -t * c, p);
        }
    }
    f
}
fn poly_json(f: &Poly) -> Value {
    json!(f
        .iter()
        .map(|(&(i, j), &c)| vec![i as I, j as I, c])
        .collect::<Vec<_>>())
}
fn homogeneous(eqs: &[Poly], p: I) -> Value {
    let (mut hs, mut cutoff) = (vec![], Value::Null);
    for d in 0..=8 {
        let ms: Vec<_> = (0..=d).map(|i| (i, d - i)).collect();
        let index: BTreeMap<_, _> = ms.iter().enumerate().map(|(i, &m)| (m, i)).collect();
        let mut rows = vec![];
        for f in eqs.iter().filter(|f| !f.is_empty() && degree(f) <= d) {
            let df = degree(f);
            for sx in 0..=d - df {
                let sy = d - df - sx;
                let mut row = vec![0; ms.len()];
                for (&(i, j), &c) in f {
                    if i + j == df {
                        row[index[&(i + sx, j + sy)]] = c;
                    }
                }
                rows.push(row);
            }
        }
        let h = ms.len() - rref(rows, ms.len(), p).1.len();
        hs.push(h);
        if h == 0 && cutoff.is_null() {
            cutoff = json!(d);
        }
    }
    json!({"hilbert":hs,"cutoff":cutoff})
}

#[derive(Clone)]
struct ProofRow {f:Poly,w:[Poly;3]}
fn pmul(a:&Poly,b:&Poly,p:I)->Poly{let mut f=Poly::new();for(&(i,j),&c)in a{for(&(u,v),&d)in b{add_term(&mut f,(i+u,j+v),c*d,p);}}f}
fn shifted(f:&Poly,s:(usize,usize))->Poly{f.iter().map(|(&(i,j),&c)|((i+s.0,j+s.1),c)).collect()}
fn expansion(row:&ProofRow,original:&[Poly;3],p:I)->Poly{let mut out=Poly::new();for j in 0..3{for(m,c)in pmul(&row.w[j],&original[j],p){add_term(&mut out,m,c,p);}}out}
fn exp_degree(row:&ProofRow,original:&[Poly;3])->usize{(0..3).filter(|&j|!row.w[j].is_empty()&&!original[j].is_empty()).map(|j|degree(&row.w[j])+degree(&original[j])).max().unwrap_or(0)}
fn parse_poly(v:&Value,p:I)->Poly{let mut f=Poly::new();for t in v.as_array().unwrap(){let a=ints(t);assert_eq!(a.len(),3);assert!((0..=32).contains(&a[0])&&(0..=32).contains(&a[1])&&a[2]>0&&a[2]<p);assert!(f.insert((a[0]as usize,a[1]as usize),a[2]).is_none());}f}
fn parse_row(v:&Value,original:&[Poly;3],p:I,bound:usize)->ProofRow{
    let w=v["witness"].as_array().unwrap();assert_eq!(w.len(),3);let row=ProofRow{f:parse_poly(&v["poly"],p),w:std::array::from_fn(|j|parse_poly(&w[j],p))};
    assert_eq!(expansion(&row,original,p),row.f,"false original-input witness");
    assert_eq!(n(&v["expanded_degree"]),exp_degree(&row,original)as I,"false expanded degree");assert!(exp_degree(&row,original)<=bound,"hidden higher-degree witness");row
}
fn desc_monomials(d:usize)->Vec<(usize,usize)>{let mut m=monomials(d);m.reverse();m}
fn matrix_space(pool:&[ProofRow],d:usize,p:I,original:&[Poly;3],peak:&mut usize)->Vec<Poly>{
    let ms=desc_monomials(d);let index:BTreeMap<_,_>=ms.iter().enumerate().map(|(i,&m)|(m,i)).collect();let mut rows=vec![];
    for row in pool.iter().filter(|r|!r.f.is_empty()&&degree(&r.f)<=d){for shift in desc_monomials(d-degree(&row.f)){
        let proof=ProofRow{f:shifted(&row.f,shift),w:std::array::from_fn(|j|shifted(&row.w[j],shift))};*peak=(*peak).max(exp_degree(&proof,original));assert!(*peak<=32,"expanded cap");
        let mut v=vec![0;ms.len()];for(&m,&c)in &proof.f{v[index[&m]]=c;}rows.push(v);
    }}
    rref(rows,ms.len(),p).0.into_iter().map(|v|ms.iter().copied().zip(v).filter(|(_,c)|*c!=0).collect()).collect()
}
fn member(f:&Poly,basis:&[Poly],p:I)->bool{let mut rows:Vec<_>=basis.to_vec();rows.push(f.clone());let d=rows.iter().map(degree).max().unwrap_or(0);let ms=desc_monomials(d);
    let build=|polys:&[Poly]|polys.iter().map(|f|ms.iter().map(|m|f.get(m).copied().unwrap_or(0)).collect()).collect();
    rref(build(&rows),ms.len(),p).1.len()==rref(build(basis),ms.len(),p).1.len()
}
fn root_set(f:&[Poly;3],p:I)->Vec<(I,I)>{(0..p).filter(|&x|ev(&f[1],x,0,p)==0).flat_map(|x|(0..p).filter(move|&y|ev(&f[2],0,y,p)==0&&ev(&f[0],x,y,p)==0).map(move|y|(x,y))).collect()}
fn audit_solve(v:&Value,original:[Poly;3],p:I)->Vec<(I,I)>{
    let rr=root_set(&original,p);assert_eq!(v["roots"],json!(rr.iter().map(|&(x,y)|vec![x,y]).collect::<Vec<_>>()));
    let input_max=original.iter().filter(|f|!f.is_empty()).map(degree).max().unwrap_or(0);assert_eq!(n(&v["input_max_degree"]),input_max as I);
    let mut pool:Vec<_>=(0..3).map(|j|ProofRow{f:original[j].clone(),w:std::array::from_fn(|i|if i==j{[((0,0),1)].into()}else{Poly::new()})}).collect();
    let trace=v["trace"].as_array().unwrap();let(mut ti,mut peak)=(0,0);
    for d in 0..=8{for round in 0..64{
        let before=pool.len();let b=matrix_space(&pool,d,p,&original,&mut peak);let ms=desc_monomials(d);
        let evaluation=rr.iter().map(|&(x,y)|ms.iter().map(|&(i,j)|red(pow(x,i,p)*pow(y,j,p),p)).collect()).collect();let erank=rref(evaluation,ms.len(),p).1.len();
        let unit=b.iter().any(|f|*f==[((0,0),1)].into());let closure=d>=input_max&&b.len()+erank==ms.len()&&erank==rr.len();
        let step=&trace[ti];ti+=1;
        for(key,value)in[("degree",d),("round",round),("pool_before",before),("rank",b.len()),("columns",ms.len()),("evaluation_rank",erank)]{assert_eq!(n(&step[key]),value as I,"false solver trace {key}");}
        let promoted=step["promoted"].as_array().unwrap();
        if unit||closure{
            assert_eq!(unit,rr.is_empty());assert!(promoted.is_empty());assert_eq!(ti,trace.len());assert_eq!(v["status"],if unit{"unit"}else{"closure"});assert_eq!(n(&v["matrix_degree"]),d as I,"false operational degree");
            let wanted:Vec<_>=b.iter().filter(|f|!unit||**f==[((0,0),1)].into()).collect();let cert=v["certificate"].as_array().unwrap();assert_eq!(cert.len(),wanted.len());let mut final_degree=0;
            for(got,expected)in cert.iter().zip(wanted){let row=parse_row(got,&original,p,peak);assert_eq!(&row.f,expected,"false terminal basis");final_degree=final_degree.max(exp_degree(&row,&original));}
            assert_eq!(n(&v["expanded_peak"]),peak as I,"false expanded peak");assert_eq!(n(&v["certificate_expanded_degree"]),final_degree as I);return rr;
        }
        let mut candidates=b.clone();candidates.sort_by_key(|f|(degree(f),serde_json::to_string(&poly_json(f)).unwrap()));let mut pi=0;
        for f in candidates.iter().filter(|f|degree(f)<d){let low=matrix_space(&pool,degree(f),p,&original,&mut peak);if !member(f,&low,p){let row=parse_row(&promoted[pi],&original,p,peak);pi+=1;assert_eq!(&row.f,f,"false promotion decision");pool.push(row);}}
        assert_eq!(pi,promoted.len());if pi==0{break;}assert!(round<63,"round cap");
    }}
    panic!("operational cap");
}
fn as_point(v:&Value)->Point{let a=ints(v);if a==[-1,-1]{None}else{assert_eq!(a.len(),2);Some((a[0],a[1]))}}
fn load(p:&Path)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn write(p:&Path,v:&Value){fs::write(p,format!("{}\n",serde_json::to_string(v).unwrap())).unwrap()}
fn audit_arm(v:&Value,c:Curve,src:&Value,r:I){
    let generator=as_point(&src["generator"]);assert!(generator.is_some()&&c.contains(generator)&&c.mul(r,generator).is_none());let xs=ints(&src["base_x"]);assert_eq!(xs.len(),2);assert_ne!(xs[0],xs[1]);
    let base:Vec<_>=src["base_points"].as_array().unwrap().iter().map(as_point).collect();for(i,&q)in base.iter().enumerate(){assert!(c.contains(q));assert_eq!(q.unwrap().0,xs[i]);}
    let logs:Vec<_>=base.iter().map(|&q|(0..r).find(|&z|c.mul(z,generator)==q).unwrap()).collect();
    let targets=v["targets"].as_array().unwrap();assert_eq!(targets.len(),(r-1)as usize,"missing target coverage");let mut all_rows=vec![];
    for scalar in 1..r{let target=c.mul(scalar,generator).unwrap();let f=s3_poly(c,target.0);let h=divide_base(f.clone(),&xs,c.p);let gx=base_poly(&xs,0,c.p);let gy=base_poly(&xs,1,c.p);let record=&targets[(scalar-1)as usize];assert_eq!(n(&record["n"]),scalar);
        let roots=audit_solve(&record["original"],[f,gx.clone(),gy.clone()],c.p);assert_eq!(roots,audit_solve(&record["reduced"],[h,gx,gy],c.p));
        let before=rref(all_rows.clone(),2,r).1.len();let mut rows=BTreeSet::new();let mut actual=BTreeSet::new();
        for i in 0..2{for j in 0..2{for si in [-1,1]{for sj in [-1,1]{if c.add(c.mul(si,base[i]),c.mul(sj,base[j]))==Some(target){let mut row=vec![0,0];row[i]=red(row[i]+si,r);row[j]=red(row[j]+sj,r);assert_eq!(red(row[0]*logs[0]+row[1]*logs[1],r),scalar);rows.insert(row.clone());all_rows.push(row);actual.insert((xs[i],xs[j]));}}}}}
        assert_eq!(roots,actual.into_iter().collect::<Vec<_>>());assert_eq!(record["rows"],json!(rows));assert_eq!(n(&record["before"]),before as I);assert_eq!(n(&record["after"]),rref(all_rows.clone(),2,r).1.len()as I);
        let old=&src["targets"][(scalar-1)as usize];for key in ["before","after","rows"]{assert_eq!(record[key],old[key],"frozen useful rows/rank");}
    }
}
fn check_recoveries(v:&Value){
    let e=Curve{p:43,a:1,b:0};let t=Curve{p:43,a:4,b:0};let generator=Some((31,18));let u=Some((24,26));let base=[Some((15,9)),Some((17,6))];let xs=[15,17];let mut equations=vec![];
    for i in 0..2{for j in 0..2{for si in [-1,1]{for sj in [-1,1]{if t.add(t.mul(si,base[i]),t.mul(sj,base[j]))==u{let mut row=vec![0,0,1];row[i]=red(row[i]+si,11);row[j]=red(row[j]+sj,11);equations.push(row);}}}}}
    let logs=solve_linear(equations,2,11);assert_eq!(v["base_logs"],json!(logs));for i in 0..2{assert_eq!(t.mul(logs[i],u),base[i]);}
    let targets=v["targets"].as_array().unwrap();assert_eq!(targets.len(),11);
    for (known,rec) in targets.iter().enumerate(){let q=e.mul(known as I,generator);let mapped=q.map(|(x,y)|(red(41*x,43),red(11*y,43)));assert_eq!(rec["target"],point_json(q));assert_eq!(rec["mapped"],point_json(mapped));
        let attempts=rec["attempts"].as_array().unwrap();let(mut ai,mut answer)=(0,None);if q.is_none(){answer=Some(0);}
        for shift in 0..11{if answer.is_some(){break;}let r=t.add(mapped,t.mul(shift,u));let attempt=&attempts[ai];ai+=1;assert_eq!(n(&attempt["shift"]),shift);
            let Some((z,_))=r else{assert_eq!(attempt["identity"],true);continue;};assert_eq!(attempt["identity"],false);
            let f=divide_base(s3_poly(t,z),&xs,43);let roots=audit_solve(&attempt["solver"],[f,base_poly(&xs,0,43),base_poly(&xs,1,43)],43);
            for(x,y)in roots{for i in 0..2{for j in 0..2{if xs[i]==x&&xs[j]==y{for si in [-1,1]{for sj in [-1,1]{if answer.is_none()&&t.add(t.mul(si,base[i]),t.mul(sj,base[j]))==r{answer=Some(red(si*logs[i]+sj*logs[j]-shift,11));}}}}}}}
        }
        assert_eq!(ai,attempts.len());let answer=answer.unwrap();assert_eq!(n(&rec["recovered"]),answer,"false recovered scalar");assert_eq!(e.mul(answer,generator),q);
    }
}
fn comparison(left:&str,right:&str,l:&Value,r:&Value,out:&mut Vec<Value>){for(a,b)in l["targets"].as_array().unwrap().iter().zip(r["targets"].as_array().unwrap()){
    let da=n(&a["after"])-n(&a["before"]);let db=n(&b["after"])-n(&b["before"]);let mut v=json!({"main":left,"twist":right,"n":a["n"],"main_useful":da,"twist_useful":db});
    for mode in ["original","reduced"]{let md=n(&a[mode]["matrix_degree"]);let td=n(&b[mode]["matrix_degree"]);v[mode]=json!({"main_degree":md,"twist_degree":td,"joint_gain":db>da&&td<md});}out.push(v);
}}
fn audit(source:&Path,results:&Path,receipt:&Path){
    assert!(!receipt.exists());let mut files:Vec<_>=fs::read_dir(source).unwrap().map(|e|e.unwrap().path()).filter(|p|p.extension().is_some_and(|e|e=="json")&&p.file_name().unwrap()!="index.json").collect();files.sort();assert_eq!(files.len(),98);
    assert_eq!(fs::read_dir(results).unwrap().filter(|e|e.as_ref().unwrap().path().extension().is_some_and(|x|x=="json")).count(),99,"case coverage");
    let(mut cases,mut pop,mut fixture,mut pins)=(BTreeMap::new(),vec![],vec![],vec![]);
    for file in files{let bytes=fs::read(&file).unwrap();let src:Value=serde_json::from_slice(&bytes).unwrap();let name=file.file_name().unwrap().to_str().unwrap().to_owned();let got=load(&results.join(&name));
        assert_eq!(got["source_sha256"],sha(&bytes),"false source binding");for key in ["p","a","b","d","r"]{assert_eq!(got[key],src[key]);}
        let(p,a,b,d,r)=(n(&src["p"]),n(&src["a"]),n(&src["b"]),n(&src["d"]),n(&src["r"]));let e=Curve{p,a,b};let t=Curve{p,a:red(d*d*a,p),b:red(d*d*d*b,p)};
        for role in ["main","twist","matched","pulled_back_twist_base"]{if !src[role].is_null(){audit_arm(&got[role],if role=="twist"||role=="matched"{t}else{e},&src[role],r);}else{assert!(got[role].is_null());}}
        pins.push(json!({"case":name,"sha256":sha(&fs::read(results.join(&name)).unwrap())}));(if p==43{&mut fixture}else{&mut pop}).push(name.clone());cases.insert(name,got);
    }
    assert_eq!((pop.len(),fixture.len()),(88,10));let mut comparisons=vec![];for s in &pop{comparison(s,s,&cases[s]["main"],&cases[s]["twist"],&mut comparisons);}for l in &fixture{for r in &fixture{comparison(l,r,&cases[l]["main"],&cases[r]["twist"],&mut comparisons);}}
    let raw=comparisons.iter().filter(|v|v["original"]["joint_gain"]==true).count();let red=comparisons.iter().filter(|v|v["reduced"]["joint_gain"]==true).count();let index=load(&results.join("index.json"));check_recoveries(&index["recoveries"]);
    assert_eq!(index,json!({"schema":"quadratic-twist-adaptive-s3/v1","cases":pins,"comparisons":comparisons,"comparison_count":188,"target_comparisons":comparisons.len(),"joint_original":raw,"joint_reduced":red,"recoveries":index["recoveries"]}));
    write(receipt,&json!({"status":"passed","cases":98,"comparisons":188,"target_comparisons":comparisons.len(),"joint_original":raw,"joint_reduced":red,"main_recoveries":11,"index_sha256":sha(&fs::read(results.join("index.json")).unwrap()),"witnesses":"all symbolic original-input expansions checked","problems":0}));println!("PASS independent adaptive spaces, witnesses, degrees and11 point recoveries; raw joint{raw}, reduced joint{red}");
}
fn corrupt(kind:&str,dir:&Path){let path=dir.join("index.json");let mut index=load(&path);if kind=="false-recovery"{index["recoveries"]["targets"][1]["recovered"]=json!(999);write(&path,&index);return;}
    let name=index["cases"][0]["case"].as_str().unwrap().to_owned();let file=dir.join(&name);let mut v=load(&file);match kind{
        "false-witness"=>{let cert=&mut v["main"]["targets"][0]["original"]["certificate"][0];cert["witness"][0].as_array_mut().unwrap().push(json!([31,0,1]));},
        "false-degree"=>v["main"]["targets"][0]["original"]["matrix_degree"]=json!(0),
        "missing-target"=>{v["main"]["targets"].as_array_mut().unwrap().remove(0);},
        "false-source"=>v["source_sha256"]=json!("0".repeat(64)),
        _=>panic!("unknown corruption")}
    write(&file,&v);index["cases"][0]["sha256"]=json!(sha(&fs::read(&file).unwrap()));write(&path,&index);
}
const PROOF:&str="research/quadratic_twist_evidence_20261021/lean-adaptive/AdaptiveS3.lean";
fn cv(n:I)->Ex{Ex::Const(n)}
fn vv(s:&str)->Ex{Ex::var(s)}
fn es(xs:Vec<Ex>)->Ex{xs.into_iter().fold(cv(0),Ex::plus)}
fn ep(xs:Vec<Ex>)->Ex{Ex::product(xs)}
fn esq(x:Ex)->Ex{ep(vec![x.clone(),x])}
fn certificate_statement()->IdentityStatement{
    let x=vv("x");let y=vv("y");let xy=ep(vec![x.clone(),y.clone()]);let xx=esq(x.clone());let yy=esq(y.clone());
    let w0=es(vec![cv(37),ep(vec![cv(16),x.clone()]),ep(vec![cv(16),y.clone()])]);
    let w1=es(vec![cv(21),ep(vec![cv(32),y.clone()])]);let w2=es(vec![cv(21),ep(vec![cv(32),x.clone()])]);
    let hm=es(vec![ep(vec![cv(41),xy.clone()]),ep(vec![cv(39),x.clone()]),ep(vec![cv(39),y.clone()]),cv(27)]);
    let gx=es(vec![xx.clone(),ep(vec![cv(26),x.clone()]),cv(9)]);
    let gy=es(vec![yy.clone(),ep(vec![cv(26),y.clone()]),cv(9)]);
    let q=es(vec![cv(32),ep(vec![cv(63),x.clone()]),ep(vec![cv(63),y.clone()]),ep(vec![cv(15),xx.clone()]),ep(vec![cv(15),yy.clone()]),ep(vec![cv(103),xy.clone()]),ep(vec![cv(16),xx,y.clone()]),ep(vec![cv(16),x,yy])]);
    IdentityStatement{name:"adaptive-S3-cleared-main-degree3-unit".into(),source:format!("{PROOF}:AdaptiveS3.F43.cleared_main_degree3_unit; polynomial only"),variables:["x","y"].map(str::to_string).into(),degree_bound:3,lhs:es(vec![ep(vec![w0,hm]),ep(vec![w1,gx]),ep(vec![w2,gy]),cv(-1)]),rhs:ep(vec![cv(43),q])}
}
fn verify_certificate(root:&Path,c:&IdentityCertificate)->Result<(),String>{
    if serde_json::to_value(&c.statement).unwrap()!=serde_json::to_value(certificate_statement()).unwrap(){return Err("wrong statement".into());}
    let p=c.proof.as_ref().ok_or("missing source")?;
    if p.kind!="lean4"||p.path!=PROOF||p.sha256!=sha(&fs::read(root.join(PROOF)).unwrap()){return Err("false source hash".into());}
    match identity_certificate::check_with_extra(c,Some((2381,16))).map_err(|e|e.to_string())?{
        Verdict::Accept{points:32,extra_points:16,..}=>Ok(()),other=>Err(format!("rejected: {other:?}"))
    }
}
fn certificate(mode:&str,root:&Path,out:&Path){
    if mode=="issue"{assert!(!out.exists());fs::create_dir_all(out).unwrap();}
    let c:IdentityCertificate=if mode=="issue"{
        identity_certificate::issue(certificate_statement(),20261025,32,"Cleared polynomial unit only; solver traces and DLP separately audited",Some(ProofRef{kind:"lean4".into(),path:PROOF.into(),sha256:sha(&fs::read(root.join(PROOF)).unwrap())})).unwrap()
    }else{serde_json::from_slice(&fs::read(out.join("identity.json")).unwrap()).unwrap()};
    verify_certificate(root,&c).unwrap();
    let mut wrong=c.clone();wrong.proof.as_mut().unwrap().sha256="0".repeat(64);let rejection=verify_certificate(root,&wrong).unwrap_err();
    let mut bad=certificate_statement();bad.rhs=Ex::plus(bad.rhs,cv(1));
    let witness=match identity_certificate::issue(bad.clone(),20261025,32,"false plus one",None).unwrap_err(){identity_certificate::IssueError::False(w)=>w,e=>panic!("wrong failure {e:?}")};
    let controls=json!({"false_source":wrong,"source_rejection":rejection,"false_identity":bad,"counterexample":{"index":witness.index,"assignment":witness.assignment,"lhs":witness.lhs,"rhs":witness.rhs}});
    let receipt=json!({"id":c.id,"status":"passed","issue_points":32,"extra_points":16,"false_identity":"rejected","false_source":"rejected","source":c.proof});
    for(name,value)in [("identity.json",serde_json::to_value(&c).unwrap()),("controls.json",controls),("receipt.json",receipt)]{
        let bytes=format!("{}\n",serde_json::to_string(&value).unwrap());if mode=="issue"{fs::write(out.join(name),bytes).unwrap();}else{assert_eq!(fs::read_to_string(out.join(name)).unwrap(),bytes);}
    }
    println!("Source-bound adaptive unit certificate {}, 32+16 points and controls pass",c.id);
}
fn main(){let a:Vec<_>=std::env::args().collect();match a.get(1).map(String::as_str){Some("audit")=>{assert_eq!(a.len(),5);audit(Path::new(&a[2]),Path::new(&a[3]),Path::new(&a[4]));},Some("corrupt")=>{assert_eq!(a.len(),4);corrupt(&a[2],Path::new(&a[3]));},Some("certificate")=>{assert_eq!(a.len(),5);certificate(&a[2],Path::new(&a[3]),Path::new(&a[4]));},_=>panic!("audit SOURCE RESULTS FRESH_RECEIPT | corrupt KIND RESULTS | certificate issue|check ROOT DIRECTORY")}}
