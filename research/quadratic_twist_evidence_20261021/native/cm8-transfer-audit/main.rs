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

// Independent request-accounting replay of the precisely named incremental solver.
const KEYS:[&str;38]=["solver_calls","membership_evals","constraint_evals","eval_term_visits",
 "basis_calls","generated_rows","dense_column_slots","monomial_copies","pivot_normalizations",
 "scaled_terms","subtraction_steps","term_updates","membership_subtractions","evaluation_entries",
 "evaluation_row_updates","evaluation_row_scales","evaluation_inverses","linear_row_updates",
 "linear_row_scales","linear_inverses","linear_backsub_updates","point_add_requests","point_negations",
 "scalar_iterations","signed_lifts","map_multiplications","membership_builds","remainder_builds","scalar_recoveries","root_entries_validated","root_order_checks","root_polynomial_bindings","cache_entries_built","cache_entry_validations","cache_binding_checks","cache_reads","sign_operand_reads","sign_dispatches"];
#[derive(Clone)]
struct Demand(BTreeMap<String,I>);
impl Demand{
 fn new()->Self{Self(KEYS.into_iter().map(|s|(s.to_owned(),0)).collect())}
 fn inc(&mut self,k:&str,v:usize){*self.0.get_mut(k).expect("known counter")+=v as I;}
 fn merge(&mut self,b:&Self){for k in KEYS{*self.0.get_mut(k).unwrap()+=b.0[k];}}
 fn value(&self)->Value{json!(self.0)}
}
struct Space{p:I,label:&'static str,pivots:BTreeMap<usize,Vec<I>>}
impl Space{
 fn new(p:I,label:&'static str)->Self{Self{p,label,pivots:BTreeMap::new()}}
 fn insert(&mut self,mut r:Vec<I>,ct:&mut Demand)->bool{
  for j in 0..r.len(){if r[j]==0{continue;}
   if let Some(b)=self.pivots.get(&j){let a=r[j];for k in j..r.len(){ct.inc(&format!("{}_row_updates",self.label),1);r[k]=red(r[k]-a*b[k],self.p);}}
   else{ct.inc(&format!("{}_inverses",self.label),1);let a=inv(r[j],self.p);for x in &mut r{ct.inc(&format!("{}_row_scales",self.label),1);*x=red(*x*a,self.p);}self.pivots.insert(j,r);return true;}
  }false
 }
}
fn leader(f:&Poly)->(usize,usize){*f.keys().max_by_key(|&&(i,j)|(i+j,i)).unwrap()}
fn proof_size(r:&ProofRow)->usize{r.f.len()+r.w.iter().map(|f|f.len()).sum::<usize>()}
fn scaled_poly(f:&Poly,c:I,p:I)->Poly{f.iter().map(|(&m,&a)|(m,red(a*c,p))).filter(|(_,a)|*a!=0).collect()}
fn normalized(mut r:ProofRow,c:I,p:I,ct:&mut Demand)->ProofRow{ct.inc("pivot_normalizations",1);ct.inc("scaled_terms",proof_size(&r));r.f=scaled_poly(&r.f,c,p);for w in &mut r.w{*w=scaled_poly(w,c,p);}r}
fn row_sub(a:&mut ProofRow,b:&ProofRow,c:I,p:I,ct:&mut Demand){ct.inc("subtraction_steps",1);ct.inc("term_updates",proof_size(b));for(&m,&v)in &b.f{add_term(&mut a.f,m,-c*v,p);}for j in 0..3{for(&m,&v)in &b.w[j]{add_term(&mut a.w[j],m,-c*v,p);}}}
fn request_basis(pool:&[ProofRow],d:usize,p:I,orig:&[Poly;3],peak:&mut usize,ct:&mut Demand)->Vec<ProofRow>{
 ct.inc("basis_calls",1);let cols=desc_monomials(d).len();let mut piv:BTreeMap<(usize,usize),ProofRow>=BTreeMap::new();
 for g in pool.iter().filter(|g|!g.f.is_empty()&&degree(&g.f)<=d){for s in desc_monomials(d-degree(&g.f)){
  ct.inc("generated_rows",1);ct.inc("dense_column_slots",cols);ct.inc("monomial_copies",proof_size(g));let mut r=ProofRow{f:shifted(&g.f,s),w:std::array::from_fn(|j|shifted(&g.w[j],s))};
  *peak=(*peak).max(exp_degree(&r,orig));assert!(*peak<=32,"expanded cap");
  while !r.f.is_empty(){let l=leader(&r.f);let c=r.f[&l];if let Some(b)=piv.get(&l){row_sub(&mut r,b,c,p,ct);}else{piv.insert(l,normalized(r,inv(c,p),p,ct));break;}}
 }}
 let mut ms:Vec<_>=piv.keys().copied().collect();ms.sort_by_key(|&(i,j)|(i+j,i));
 for &m in &ms{for &u in &ms{if(m.0+m.1,m.0)<(u.0+u.1,u.0){if let Some(&c)=piv[&u].f.get(&m){let b=piv[&m].clone();row_sub(piv.get_mut(&u).unwrap(),&b,c,p,ct);}}}}
 ms.reverse();ms.into_iter().map(|m|piv.remove(&m).unwrap()).collect()
}
fn request_member(mut f:Poly,b:&[ProofRow],p:I,ct:&mut Demand)->bool{for r in b{if let Some(&c)=f.get(&leader(&r.f)){ct.inc("membership_subtractions",1);ct.inc("term_updates",r.f.len());for(&m,&v)in &r.f{add_term(&mut f,m,-c*v,p);}}}f.is_empty()}
fn proof_json(r:&ProofRow,orig:&[Poly;3])->Value{json!({"poly":poly_json(&r.f),"witness":r.w.iter().map(poly_json).collect::<Vec<_>>(),"expanded_degree":exp_degree(r,orig)})}
fn request_eval(f:&Poly,x:I,y:I,p:I,is_mem:bool,ct:&mut Demand)->I{ct.inc(if is_mem{"membership_evals"}else{"constraint_evals"},1);ct.inc("eval_term_visits",f.len());ev(f,x,y,p)}
fn request_solve(f:[Poly;3],p:I,ct:&mut Demand,known:bool,xs:&[I])->Value{
 ct.inc("solver_calls",1);let mut pool:Vec<_>=(0..3).map(|j|ProofRow{f:f[j].clone(),w:std::array::from_fn(|i|if i==j{[((0,0),1)].into()}else{Poly::new()})}).collect();let input_max=f.iter().filter(|f|!f.is_empty()).map(degree).max().unwrap_or(0);
 let mut rr=vec![];if known{let mut roots=xs.to_vec();roots.sort();for &x in &roots{for &y in &roots{if request_eval(&f[0],x,y,p,false,ct)==0{rr.push((x,y));}}}}else{for x in 0..p{if request_eval(&f[1],x,0,p,true,ct)==0{for y in 0..p{if request_eval(&f[2],0,y,p,true,ct)==0&&request_eval(&f[0],x,y,p,false,ct)==0{rr.push((x,y));}}}}}
 let roots:Vec<_>=rr.iter().map(|&(x,y)|vec![x,y]).collect();let(mut trace,mut peak)=(vec![],0);
 for d in 0..=8{for round in 0..64{
  let before=pool.len();let mut b=request_basis(&pool,d,p,&f,&mut peak,ct);let ms=desc_monomials(d);let mut evaluation=Space::new(p,"evaluation");
  for &(x,y) in &rr{let row=ms.iter().map(|&(i,j)|{ct.inc("evaluation_entries",1);red(pow(x,i,p)*pow(y,j,p),p)}).collect();evaluation.insert(row,ct);}
  let unit=b.iter().any(|r|r.f==[((0,0),1)].into());let closure=d>=input_max&&b.len()+evaluation.pivots.len()==ms.len()&&evaluation.pivots.len()==rr.len();let mut promoted=vec![];
  if unit||closure{assert_eq!(unit,rr.is_empty());trace.push(json!({"degree":d,"round":round,"pool_before":before,"rank":b.len(),"columns":ms.len(),"evaluation_rank":evaluation.pivots.len(),"promoted":promoted}));
   let cert:Vec<_>=b.iter().filter(|r|!unit||r.f==[((0,0),1)].into()).collect();let final_degree=cert.iter().map(|r|exp_degree(r,&f)).max().unwrap_or(0);
   return json!({"status":if unit{"unit"}else{"closure"},"matrix_degree":d,"input_max_degree":input_max,"expanded_peak":peak,"certificate_expanded_degree":final_degree,"roots":roots,"certificate":cert.into_iter().map(|r|proof_json(r,&f)).collect::<Vec<_>>(),"trace":trace});
  }
  let rank=b.len();b.sort_by_key(|r|(degree(&r.f),serde_json::to_string(&poly_json(&r.f)).unwrap()));
  for r in b.into_iter().filter(|r|degree(&r.f)<d){let low=request_basis(&pool,degree(&r.f),p,&f,&mut peak,ct);if !request_member(r.f.clone(),&low,p,ct){promoted.push(proof_json(&r,&f));pool.push(r);}}
  let stop=promoted.is_empty();trace.push(json!({"degree":d,"round":round,"pool_before":before,"rank":rank,"columns":ms.len(),"evaluation_rank":evaluation.pivots.len(),"promoted":promoted}));if stop{break;}assert!(round<63,"round cap");
 }}panic!("solver cap");
}
fn phi(q:Point)->Point{q.and_then(|(x,y)|{if x==10{None}else{let v=inv(x-10,13);Some((red(12*(x+2*v),13),red(8*y*(1-2*v*v),13)))}})}
fn inverse7(e:Curve,p:Point,q:Point)->Point{(0..7).map(|i|e.mul(i,p)).find(|&v|phi(v)==q).expect("subgroup inverse")}
fn cm_query(c:Curve,r:Point,base:&[Point])->Value{
 let xs:Vec<_>=base.iter().map(|b|b.unwrap().0).collect();let z=r.unwrap().0;let f=s3_poly(c,z);let h=divide_base(f.clone(),&xs,13);let gx=base_poly(&xs,0,13);let gy=base_poly(&xs,1,13);let mut demand=Demand::new();let raw=request_solve([f.clone(),gx.clone(),gy.clone()],13,&mut demand,false,&xs);let reduced=request_solve([h.clone(),gx.clone(),gy.clone()],13,&mut demand,false,&xs);assert_eq!(raw["roots"],reduced["roots"]);audit_solve(&raw,[f,gx.clone(),gy.clone()],13);audit_solve(&reduced,[h,gx,gy],13);
 let(mut tested,mut lifts,mut rows,mut actual)=(vec![],vec![],BTreeSet::new(),vec![]);
 for i in 0..2{for j in 0..2{let mut any=false;for si in [-1,1]{for sj in [-1,1]{let a=if si<0{c.neg(base[i])}else{base[i]};let b=if sj<0{c.neg(base[j])}else{base[j]};let sum=c.add(a,b);tested.push(json!({"i":i,"j":j,"si":si,"sj":sj,"A":point_json(a),"B":point_json(b),"sum":point_json(sum),"accepted":sum==r}));if sum==r{any=true;lifts.push(vec![i as I,j as I,si,sj]);let mut row=vec![0,0];row[i]=red(row[i]+si,7);row[j]=red(row[j]+sj,7);rows.insert(row);}}}if any{actual.push(vec![xs[i],xs[j]]);}}}actual.sort();assert_eq!(json!(actual),reduced["roots"],"actual lifts");json!({"target":point_json(r),"original":raw,"reduced":reduced,"tested":tested,"lifts":lifts,"rows":rows})
}
fn cm_arm(e:Curve,c:Curve,p:Point,g:Point,base:Vec<Point>,mapped:bool)->Value{
 let(mut targets,mut setup_queries)=(vec![],vec![]);let(mut rank,mut equations)=(Space::new(7,"linear"),Space::new(7,"linear"));let mut ct=Demand::new();
 for z in 1..7{let mut q=cm_query(c,c.mul(z,g),&base);let before=rank.pivots.len();for v in q["rows"].as_array().unwrap(){let mut row=ints(v);rank.insert(row.clone(),&mut ct);if equations.pivots.len()<2{row.push(z);equations.insert(row,&mut ct);}}q["n"]=json!(z);q["before"]=json!(before);q["after"]=json!(rank.pivots.len());if setup_queries.last().is_none_or(|v:&Value|n(&v["rank"])<2){setup_queries.push(json!({"n":z,"query":q,"rank":equations.pivots.len()}));}targets.push(q);}
 assert_eq!(equations.pivots.len(),2);assert!(!equations.pivots.contains_key(&2));let mut logs=vec![0,0];for(&j,row)in equations.pivots.iter().rev(){let mut v=row[2];for k in j+1..2{v-=row[k]*logs[k];}logs[j]=red(v,7);}for i in 0..2{assert_eq!(c.mul(logs[i],g),base[i]);}
 let recover=|q:Point|{let m=if mapped{phi(q)}else{q};let(mut attempts,mut answer)=(vec![],None);for shift in 0..7{if answer.is_some(){break;}let r=c.add(m,c.mul(shift,g));if r.is_none(){attempts.push(json!({"shift":shift,"identity":true}));continue;}let mut v=cm_query(c,r,&base);v["shift"]=json!(shift);v["identity"]=json!(false);if let Some(l)=v["lifts"].as_array().unwrap().first(){let a=ints(l);answer=Some(red(a[2]*logs[a[0]as usize]+a[3]*logs[a[1]as usize]-shift,7));}attempts.push(v);}let a=answer.expect("recovery");assert_eq!(e.mul(a,p),q);json!({"source":point_json(q),"mapped":point_json(m),"answer":a,"map_calls":if mapped{1}else{0},"attempts":attempts})};
 json!({"generator":point_json(g),"base_points":base.iter().copied().map(point_json).collect::<Vec<_>>(),"targets":targets,"setup_queries":setup_queries,"base_logs":logs,"recoveries":(1..7).map(|n|recover(e.mul(n,p))).collect::<Vec<_>>()})
}
fn cm_audit(root:&Path,out:&Path,receipt:&Path){
 let e=Curve{p:13,a:1,b:4};let t=Curve{p:13,a:4,b:6};let p=e.generator(7);let u=phi(p);assert!(u.is_some()&&t.mul(7,u).is_none());let pts=e.points();assert_eq!(pts.len(),14);assert_eq!(t.points().len(),14);let mut maps=vec![];let mut pairs=0;
 for &a in &pts{let b=phi(a);assert!(t.contains(b));if b.is_none(){assert!(a.is_none()||a==Some((10,0)));}maps.push(json!({"source":point_json(a),"image":point_json(b)}));for &b in &pts{assert_eq!(phi(e.add(a,b)),t.add(phi(a),phi(b)));pairs+=1;}}
 for n in 0..7{assert_eq!(phi(e.mul(n,p)),t.mul(n,u));}assert!(!(1..13).any(|c|pow(c,4,13)==4&&pow(c,6,13)==8));let mut arms=BTreeMap::new();
 for(name,c,g)in [("main",e,p),("twist",t,u)]{let xs=c.xs(g,7);assert_eq!(xs.len(),3);for(i,xs)in subs(&xs,2).into_iter().enumerate(){let bs=c.reps(&xs);arms.insert(format!("{name}{i}"),cm_arm(e,c,p,g,bs.clone(),name=="twist"));let(other,c2,g2,mapped)=if name=="main"{("matched",t,u,true)}else{("pullback",e,p,false)};let bs=bs.into_iter().map(|b|if name=="main"{phi(b)}else{inverse7(e,p,b)}).collect();arms.insert(format!("{other}{i}"),cm_arm(e,c2,p,g2,bs,mapped));}}
 let mut comparisons=vec![];let(mut original,mut reduced)=(0,0);for i in 0..3{for j in 0..3{for z in 0..6{let a=&arms[&format!("main{i}")]["targets"][z];let b=&arms[&format!("twist{j}")]["targets"][z];let da=n(&a["after"])-n(&a["before"]);let db=n(&b["after"])-n(&b["before"]);let mut v=json!({"main":i,"twist":j,"n":z+1,"main_useful":da,"twist_useful":db});for mode in ["original","reduced"]{let joint=db>da&&n(&b[mode]["matrix_degree"])<n(&a[mode]["matrix_degree"]);v[mode]=json!({"main_degree":a[mode]["matrix_degree"],"twist_degree":b[mode]["matrix_degree"],"joint_gain":joint});if joint{if mode=="original"{original+=1;}else{reduced+=1;}}}comparisons.push(v);}}}
 for i in 0..3{for(a,b)in [("main","matched"),("twist","pullback")]{for z in 0..6{for k in ["rows","lifts","before","after"]{assert_eq!(arms[&format!("{a}{i}")]["targets"][z][k],arms[&format!("{b}{i}")]["targets"][z][k]);}}}}
 let study="research/quadratic_twist_evidence_20261021";
 let prior:Value=serde_json::from_slice(&fs::read(root.join(format!("{study}/native/reduced-s3-20261025/cases/p13-a1-b4-r7-size2-policy0.json"))).unwrap()).unwrap();
 let adaptive:Value=serde_json::from_slice(&fs::read(root.join(format!("{study}/native/adaptive-s3-20261025/cases/p13-a1-b4-r7-size2-policy0.json"))).unwrap()).unwrap();
 let mut prior_certificates=0;
 for role in ["main","twist"]{let fresh=&arms[&format!("{role}0")];assert_eq!(fresh["base_points"],prior[role]["base_points"]);for(i,old)in prior[role]["targets"].as_array().unwrap().iter().enumerate(){let q=fresh["targets"].as_array().unwrap().iter().find(|q|q["target"][0]==old["target"][0]).unwrap();for mode in ["original","reduced"]{assert_eq!(q[mode],adaptive[role]["targets"][i][mode],"prior prefix solver by actual x");prior_certificates+=1;}}}
 let mut matched_degree_equal=0;let mut matched_degree_different=0;let mut matched_changes=vec![];
 for i in 0..3{for(a,b)in [("main","matched"),("twist","pullback")]{for z in 0..6{for mode in ["original","reduced"]{if arms[&format!("{a}{i}")]["targets"][z][mode]["matrix_degree"]==arms[&format!("{b}{i}")]["targets"][z][mode]["matrix_degree"]{matched_degree_equal+=1;}else{matched_degree_different+=1;matched_changes.push(json!({"from":format!("{a}{i}"),"to":format!("{b}{i}"),"n":z+1,"mode":mode,"from_degree":arms[&format!("{a}{i}")]["targets"][z][mode]["matrix_degree"],"to_degree":arms[&format!("{b}{i}")]["targets"][z][mode]["matrix_degree"],"from_expanded":arms[&format!("{a}{i}")]["targets"][z][mode]["expanded_peak"],"to_expanded":arms[&format!("{b}{i}")]["targets"][z][mode]["expanded_peak"]}));}}}}}
 let setup_query_count:usize=arms.values().map(|v|v["setup_queries"].as_array().unwrap().len()).sum();
 let online_query_count:usize=arms.values().flat_map(|v|v["recoveries"].as_array().unwrap()).map(|v|v["attempts"].as_array().unwrap().len()).sum();
 let actual_map_calls:I=arms.values().flat_map(|v|v["recoveries"].as_array().unwrap()).map(|v|n(&v["map_calls"])).sum();
 let failed_shifts:usize=arms.values().flat_map(|v|v["recoveries"].as_array().unwrap()).map(|v|v["attempts"].as_array().unwrap().len()-1).sum();
 let mut pins=vec![];for(name,v)in &arms{let bytes=fs::read(out.join(format!("{name}.json"))).expect("missing arm");let got:Value=serde_json::from_slice(&bytes).unwrap();assert_eq!(&got,v,"false cm8 map, subgroup, coverage, witness, row, query, log or scalar");pins.push(json!({"name":name,"sha256":sha(&bytes)}));}
 let study="research/quadratic_twist_evidence_20261021";let sources:Vec<_>=["native/cm8-transfer.cpp","CM8_TRANSFER_20261025_PROTOCOL.md","native/phase-adaptive-prior.hpp","native/reduced-s3-20261025/cases/p13-a1-b4-r7-size2-policy0.json","native/adaptive-s3-20261025/cases/p13-a1-b4-r7-size2-policy0.json"].into_iter().map(|p|{let path=format!("{study}/{p}");json!({"path":path,"sha256":sha(&fs::read(root.join(&path)).unwrap())})}).collect();
 let expected=json!({"schema":"quadratic-twist-cm8-transfer/v1","P":point_json(p),"U":point_json(u),"point_map":maps,"addition_pairs":pairs,"arms":pins,"comparisons":comparisons,"joint_original":original,"joint_reduced":reduced,"sources":sources});let bytes=fs::read(out.join("index.json")).unwrap();let index:Value=serde_json::from_slice(&bytes).unwrap();assert_eq!(index,expected,"false cm8 map, subgroup, coverage, witness, row, query, log or scalar");let report=json!({"status":"passed","arms":12,"targets":72,"verified":72,"setup_queries":setup_query_count,"online_queries":online_query_count,"map_calls":actual_map_calls,"addition_pairs":pairs,"joint_original":original,"joint_reduced":reduced,"P":point_json(p),"U":point_json(u),"index_sha256":sha(&bytes),"independent_arithmetic":true,"semantic_witness_audit":true,"prior_prefix_certificates":prior_certificates,"matched_degree_equal":matched_degree_equal,"matched_degree_different":matched_degree_different,"matched_changes":matched_changes,"failed_online_shifts":failed_shifts,"missing_failed_query_control":if failed_shifts==0{"null-no-failed-shift"}else{"applicable"}});fs::write(receipt,format!("{}\n",serde_json::to_string(&report).unwrap())).unwrap();println!("PASS:12arms/72recoveries/196rational addition pairs, all independent arithmetic and semantic witnesses");
}
fn cm_corrupt(kind:&str,out:&Path){let path=out.join("index.json");let mut index:Value=serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();if kind=="false-map"{index["point_map"][1]["image"]=json!([-1,-1]);}else if kind=="false-subgroup"{index["P"]=json!([10,0]);}else if kind=="false-source"{index["sources"][0]["sha256"]=json!("0".repeat(64));}else if kind=="missing-arm"{index["arms"].as_array_mut().unwrap().pop();}else{let name="twist0";let f=out.join(format!("{name}.json"));let mut v:Value=serde_json::from_slice(&fs::read(&f).unwrap()).unwrap();match kind{"false-witness"=>v["targets"][0]["reduced"]["certificate"][0]["witness"][0]=json!([[0,0,99]]),"missing-target"=>{v["targets"].as_array_mut().unwrap().pop();},"false-degree"=>v["targets"][0]["reduced"]["matrix_degree"]=json!(99),"false-row"=>v["targets"][0]["rows"]=json!([]),"false-log"=>v["base_logs"][0]=json!(99),"false-scalar"=>v["recoveries"][0]["answer"]=json!(99),"missing-query"=>{v["recoveries"][0]["attempts"].as_array_mut().unwrap().pop();},_=>panic!("unknown control")};let bytes=format!("{}\n",serde_json::to_string(&v).unwrap());fs::write(f,&bytes).unwrap();for a in index["arms"].as_array_mut().unwrap(){if a["name"]==name{a["sha256"]=json!(sha(bytes.as_bytes()));}}}fs::write(path,format!("{}\n",serde_json::to_string(&index).unwrap())).unwrap();}
const PROOF:&str="research/quadratic_twist_evidence_20261021/lean-cm8/CM8Transfer.lean";
fn cv(n:I)->Ex{Ex::Const(n)}
fn vv(s:&str)->Ex{Ex::var(s)}
fn ep(xs:Vec<Ex>)->Ex{Ex::product(xs)}
fn es(xs:Vec<Ex>)->Ex{xs.into_iter().fold(cv(0),Ex::plus)}
fn po(x:Ex,n:usize)->Ex{ep(vec![x;n])}
fn certificate_statement()->IdentityStatement{
 let u=vv("u");let a=vv("A");let b=vv("B");let u2=po(u.clone(),2);let w=es(vec![u2.clone(),ep(vec![a.clone(),u.clone()]),b.clone()]);let source=es(vec![po(u.clone(),3),ep(vec![a.clone(),u2.clone()]),ep(vec![b.clone(),u.clone()])]);
 let lhs=ep(vec![source,po(es(vec![u2,ep(vec![cv(-1),b.clone()])]),2),po(u.clone(),2)]);
 let rhs=es(vec![ep(vec![po(w.clone(),3),po(u.clone(),3)]),ep(vec![cv(-2),a.clone(),po(w.clone(),2),po(u.clone(),4)]),ep(vec![es(vec![po(a,2),ep(vec![cv(-4),b])]),w,po(u,5)])]);
 IdentityStatement{name:"f13-degree-two-universal-cleared-map".into(),source:format!("{PROOF}:CM8Transfer.cleared_map_identity; polynomial only"),variables:vec!["u".into(),"A".into(),"B".into()],degree_bound:12,lhs,rhs}
}
fn verify_certificate(root:&Path,c:&IdentityCertificate)->Result<(),String>{
 if serde_json::to_value(&c.statement).unwrap()!=serde_json::to_value(certificate_statement()).unwrap(){return Err("false statement".into());}
 let p=c.proof.as_ref().ok_or("missing proof")?;if p.kind!="lean4"||p.path!=PROOF||p.sha256!=sha(&fs::read(root.join(PROOF)).unwrap()){return Err("false source hash".into());}
 match identity_certificate::check_with_extra(c,Some((2381,16))).map_err(|e|e.to_string())?{Verdict::Accept{points:32,extra_points:16,..}=>Ok(()),v=>Err(format!("rejected:{v:?}"))}
}
fn certificate(mode:&str,root:&Path,out:&Path){
 if mode=="issue"{assert!(!out.exists());fs::create_dir_all(out).unwrap();}
 let c:IdentityCertificate=if mode=="issue"{identity_certificate::issue(certificate_statement(),20261025,32,"Polynomial only; finite point/scalar map separately checked",Some(ProofRef{kind:"lean4".into(),path:PROOF.into(),sha256:sha(&fs::read(root.join(PROOF)).unwrap())})).unwrap()}else{serde_json::from_slice(&fs::read(out.join("identity.json")).unwrap()).unwrap()};verify_certificate(root,&c).unwrap();let mut bad=c.clone();bad.proof.as_mut().unwrap().sha256="0".repeat(64);let source_rejection=verify_certificate(root,&bad).unwrap_err();let mut wrong=certificate_statement();wrong.rhs=Ex::plus(wrong.rhs,cv(1));let witness=match identity_certificate::issue(wrong.clone(),20261025,32,"false plus one",None).unwrap_err(){identity_certificate::IssueError::False(w)=>w,e=>panic!("wrong failure:{e:?}")};
 let controls=json!({"false_source":bad,"source_rejection":source_rejection,"false_identity":wrong,"counterexample":{"index":witness.index,"assignment":witness.assignment,"lhs":witness.lhs,"rhs":witness.rhs}});let receipt=json!({"id":c.id,"status":"passed","issue_points":32,"extra_points":16,"false_identity":"rejected","false_source":"rejected","source":c.proof});
 for(name,value)in [("identity.json",serde_json::to_value(&c).unwrap()),("controls.json",controls),("receipt.json",receipt)]{let bytes=format!("{}\n",serde_json::to_string(&value).unwrap());if mode=="issue"{fs::write(out.join(name),bytes).unwrap();}else{assert_eq!(fs::read_to_string(out.join(name)).unwrap(),bytes);}}println!("PASS: source-bound universal coordinate identity{},32+16points and controls",c.id);
}
fn main(){let a:Vec<_>=std::env::args().collect();match a.get(1).map(String::as_str){Some("certificate")=>certificate(&a[2],Path::new(&a[3]),Path::new(&a[4])),Some("audit")=>cm_audit(Path::new(&a[2]),Path::new(&a[3]),Path::new(&a[4])),Some("corrupt")=>cm_corrupt(&a[2],Path::new(&a[3])),_=>panic!("audit ROOT RESULTS RECEIPT | corrupt KIND RESULTS")}}
