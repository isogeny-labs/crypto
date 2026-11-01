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
fn request_add(c:Curve,a:Point,b:Point,ct:&mut Demand)->Point{ct.inc("point_add_requests",1);c.add(a,b)}
fn request_mul(c:Curve,mut z:I,mut a:Point,ct:&mut Demand)->Point{if z<0{ct.inc("point_negations",1);a=c.neg(a);z=-z;}let mut r=None;while z!=0{ct.inc("scalar_iterations",1);if z%2==1{r=request_add(c,r,a,ct);}a=request_add(c,a,a,ct);z/=2;}r}
fn request_query(c:Curve,r:Point,src:&Value,old:&Value,base:&[Point],xs:&[I],gx:&Poly,gy:&Poly,ct:&mut Demand,checked:&mut BTreeSet<String>,known:bool,cache:Option<&Vec<[Point;2]>>,mode:I)->(Value,Vec<[I;4]>,BTreeSet<Vec<I>>){
 let z=r.unwrap().0;ct.inc("remainder_builds",1);let h=divide_base(s3_poly(c,z),xs,43);let f=[h,gx.clone(),gy.clone()];let cert=request_solve(f.clone(),43,ct,known,xs);
 let at=src["targets"].as_array().unwrap().iter().position(|q|n(&q["target"][0])==z).expect("frozen query");assert_eq!(cert,old["targets"][at]["reduced"],"source-bound full solver certificate");
 let digest=sha(serde_json::to_string(&cert).unwrap().as_bytes());if checked.insert(digest.clone()){audit_solve(&cert,f,43);}
 let(mut lifts,mut rows,mut tested)=(vec![],BTreeSet::new(),vec![]);for root in cert["roots"].as_array().unwrap(){let xy=ints(root);for i in 0..2{for j in 0..2{if xs[i]!=xy[0]||xs[j]!=xy[1]{continue;}for si in [-1,1]{for sj in [-1,1]{ct.inc("signed_lifts",1);ct.inc("sign_operand_reads",2);ct.inc("sign_dispatches",2);let(a,b)=if mode==2{let a=if si<0{ct.inc("point_negations",1);c.neg(base[i])}else{base[i]};let b=if sj<0{ct.inc("point_negations",1);c.neg(base[j])}else{base[j]};(a,b)}else if let Some(cache)=cache{ct.inc("cache_reads",2);(cache[i][usize::from(si>0)],cache[j][usize::from(sj>0)])}else{(request_mul(c,si,base[i],ct),request_mul(c,sj,base[j],ct))};let sum=request_add(c,a,b,ct);tested.push(json!({"i":i,"j":j,"si":si,"sj":sj,"A":point_json(a),"B":point_json(b),"sum":point_json(sum),"accepted":sum==r}));if sum==r{lifts.push([i as I,j as I,si,sj]);let mut row=vec![0,0];row[i]=red(row[i]+si,11);row[j]=red(row[j]+sj,11);rows.insert(row);}}}}}}
 let record=json!({"target":point_json(r),"solver_sha256":digest,"matrix_degree":cert["matrix_degree"],"expanded_peak":cert["expanded_peak"],"root_pairs":cert["roots"].as_array().unwrap().len(),"rows":rows,"lifts":lifts,"sign_trace_sha256":sha(serde_json::to_string(&tested).unwrap().as_bytes())});(record,lifts,rows)
}
fn phase_role(role:&str,src:&Value,old:&Value,checked:&mut BTreeSet<String>,mode:I)->Value{
 let cached=mode==1;let known=true;
 let e=Curve{p:43,a:1,b:0};let t=Curve{p:43,a:4,b:0};let mapped=role=="twist"||role=="matched";let c=if mapped{t}else{e};let p=Some((31,18));let g=as_point(&src["generator"]);let xs=ints(&src["base_x"]);let base:Vec<_>=src["base_points"].as_array().unwrap().iter().map(as_point).collect();let gx=base_poly(&xs,0,43);let gy=base_poly(&xs,1,43);
 let mut setup=Demand::new();setup.inc("membership_builds",2);let mut root_contract=Value::Null;
 if known{
  assert_eq!(xs.len(),2);assert_eq!(base.len(),2);
  for i in 0..2{setup.inc("root_entries_validated",1);assert!((0..43).contains(&xs[i]));assert_eq!(base[i].unwrap().0,xs[i]);}
  let mut roots=xs.clone();roots.sort();setup.inc("root_order_checks",1);assert!(roots[0]<roots[1]);
  for v in 0..2{setup.inc("root_polynomial_bindings",1);let mut expected=Poly::new();add_term(&mut expected,if v==0{(2,0)}else{(0,2)},1,43);add_term(&mut expected,if v==0{(1,0)}else{(0,1)},-xs[0]-xs[1],43);add_term(&mut expected,(0,0),xs[0]*xs[1],43);assert_eq!(expected,if v==0{gx.clone()}else{gy.clone()});}
  for &x in &roots{assert_eq!(request_eval(&gx,x,0,43,true,&mut setup),0);assert_eq!(request_eval(&gy,0,x,43,true,&mut setup),0);}
  root_contract=json!({"roots":roots,"gx":poly_json(&gx),"gy":poly_json(&gy)});
 }
 let mut cache=vec![];let mut cache_contract=vec![];
 if cached{
  for i in 0..2{let mut entries=[None;2];for j in 0..2{
   let q=if j==0{setup.inc("point_negations",1);c.neg(base[i])}else{base[i]};setup.inc("cache_entries_built",1);entries[j]=q;
   setup.inc("cache_entry_validations",1);let(x,y)=q.unwrap();assert!((0..43).contains(&x)&&(0..43).contains(&y)&&c.contains(q));
   setup.inc("cache_binding_checks",1);if j==0{assert!(request_add(c,base[i],q,&mut setup).is_none());}else{assert_eq!(q,base[i]);}
   cache_contract.push(json!({"base_index":i,"sign":if j==0{-1}else{1},"point":point_json(q)}));
  }cache.push(entries);}
 }
 let cache_ref=if cached{Some(&cache)}else{None};
 let mut linear=Space::new(11,"linear");let mut queries=vec![];
 for m in 1..=10{if linear.pivots.len()>=2{break;}let mut d=Demand::new();let r=request_mul(c,m,g,&mut d);let(mut q,_,rows)=request_query(c,r,src,old,&base,&xs,&gx,&gy,&mut d,checked,known,cache_ref,mode);let before=linear.pivots.len();for mut row in rows{row.push(m);linear.insert(row,&mut d);}q["m"]=json!(m);q["before"]=json!(before);q["after"]=json!(linear.pivots.len());q["demand"]=d.value();queries.push(q);setup.merge(&d);}
 assert_eq!(linear.pivots.len(),2,"setup rank cap");assert!(!linear.pivots.contains_key(&2));let mut logs=vec![0,0];for(&j,row)in linear.pivots.iter().rev(){let mut v=row[2];for k in j+1..2{setup.inc("linear_backsub_updates",1);v-=row[k]*logs[k];}logs[j]=red(v,11);}for j in 0..2{assert_eq!(request_mul(c,logs[j],g,&mut setup),base[j]);}
 let mut setup_json=json!({"queries":queries,"base_logs":logs,"demand":setup.value()});if known{setup_json["root_contract"]=root_contract;}if cached{setup_json["signed_cache"]=json!(cache_contract);}
 let mut recover=|q:Point|->Value{let(mut online,mut verification)=(Demand::new(),Demand::new());let image=if mapped{online.inc("map_multiplications",2);q.map(|(x,y)|(red(41*x,43),red(11*y,43)))}else{q};let(mut attempts,mut answer)=(vec![],None);
  for shift in 0..=10{if answer.is_some(){break;}let mut d=Demand::new();let s=request_mul(c,shift,g,&mut d);let r=request_add(c,image,s,&mut d);let mut record;
   if r.is_none(){record=json!({"shift":shift,"identity":true});}
   else{let(mut value,lifts,_)=request_query(c,r,src,old,&base,&xs,&gx,&gy,&mut d,checked,known,cache_ref,mode);value["shift"]=json!(shift);value["identity"]=json!(false);record=value;if let Some(l)=lifts.first(){d.inc("scalar_recoveries",1);answer=Some(red(l[2]*logs[l[0]as usize]+l[3]*logs[l[1]as usize]-shift,11));}}
   record["demand"]=d.value();attempts.push(record);online.merge(&d);
  }
  if let Some(a)=answer{assert_eq!(request_mul(e,a,p,&mut verification),q,"source scalar check");}let mut cold=setup.clone();cold.merge(&online);cold.merge(&verification);
  json!({"source":point_json(q),"mapped":point_json(image),"answer":answer.unwrap_or(-1),"status":if answer.is_some(){"verified"}else{"failed"},"attempts":attempts,"online":online.value(),"verification":verification.value(),"cold":cold.value()})
 };
 let targets:Vec<_>=(1..=10).map(|known|{let mut v=recover(e.mul(known,p));v["n"]=json!(known);v}).collect();json!({"setup":setup_json,"targets":targets})
}



const NEW_KEYS:[&str;2]=["sign_operand_reads","sign_dispatches"];
fn old_shape(mut v:Value)->Value{match &mut v{Value::Array(a)=>for q in a{*q=old_shape(q.take());},Value::Object(m)=>{for k in ["sign_operand_reads","sign_dispatches","sign_trace_sha256"]{m.remove(k);}for q in m.values_mut(){*q=old_shape(q.take());}},_=>{}}v}
fn mathematical(mut v:Value)->Value{match &mut v{Value::Array(a)=>for q in a{*q=mathematical(q.take());},Value::Object(m)=>{for k in ["demand","online","verification","cold","signed_cache"]{m.remove(k);}for q in m.values_mut(){*q=mathematical(q.take());}},_=>{}}v}
fn check_delta(binary:&Value,cached:&Value,direct:&Value,setup:bool){
 let l=n(&binary["signed_lifts"]);let extra=if setup{2}else{0};for k in KEYS{
  let d=match k{"point_add_requests"=>n(&binary[k])-4*l,"scalar_iterations"=>n(&binary[k])-2*l,"sign_operand_reads"|"sign_dispatches"=>2*l,_=>n(&binary[k])};
  assert_eq!(n(&direct[k]),d,"wrong direct delta {k}");
  let c=match k{"point_add_requests"=>d+extra,"point_negations"=>d-l+extra,"cache_entries_built"|"cache_entry_validations"|"cache_binding_checks"=>if setup{4}else{0},"cache_reads"=>2*l,_=>d};assert_eq!(n(&cached[k]),c,"wrong cached delta {k}");
 }
 for key in NEW_KEYS{assert_eq!(n(&binary[key]),2*l);}
}
fn direct_audit(source:&Path,prior:&Path,baseline:&Path,results:&Path,receipt:&Path){
 assert!(!receipt.exists());let mut files:Vec<_>=fs::read_dir(source).unwrap().map(|e|e.unwrap().path()).filter(|p|p.file_name().unwrap().to_str().unwrap().starts_with("p43-")).collect();files.sort();assert_eq!(files.len(),10);assert_eq!(fs::read_dir(results).unwrap().count(),11);
 let(mut pins,mut all,mut checked)=(vec![],vec![],BTreeSet::new());let(mut setups,mut targets,mut verified,mut direct_adds_lower,mut cache_negs_lower)=(0,0,0,0,0);
 for file in files{let name=file.file_name().unwrap().to_str().unwrap().to_owned();let sb=fs::read(&file).unwrap();let ab=fs::read(prior.join(&name)).unwrap();let pb=fs::read(baseline.join(&name)).unwrap();let src:Value=serde_json::from_slice(&sb).unwrap();let old:Value=serde_json::from_slice(&ab).unwrap();let previous:Value=serde_json::from_slice(&pb).unwrap();let mut wanted=json!({"source_sha256":sha(&sb),"adaptive_sha256":sha(&ab),"signed_sha256":sha(&pb)});
  for (arm,mode) in [("binary",0),("cached",1),("direct",2)]{for role in ["main","twist","matched","pulled_back_twist_base"]{let v=phase_role(role,&src[role],&old[role],&mut checked,mode);if mode<2{assert_eq!(old_shape(v.clone()),previous[if mode==0{"reference"}else{"signed_cache"}][role],"frozen sign-arm mismatch");}setups+=1;for t in v["targets"].as_array().unwrap(){targets+=1;verified+=usize::from(t["status"]=="verified");}wanted[arm][role]=v;}}
  for role in ["main","twist","matched","pulled_back_twist_base"]{let(b,c,d)=(&wanted["binary"][role],&wanted["cached"][role],&wanted["direct"][role]);assert_eq!(mathematical(b.clone()),mathematical(c.clone()));assert_eq!(mathematical(b.clone()),mathematical(d.clone()),"changed ordered operand tests or mathematical behavior");check_delta(&b["setup"]["demand"],&c["setup"]["demand"],&d["setup"]["demand"],true);
   for i in 0..10{for phase in ["online","verification","cold"]{check_delta(&b["targets"][i][phase],&c["targets"][i][phase],&d["targets"][i][phase],phase=="cold");}direct_adds_lower+=usize::from(n(&d["targets"][i]["cold"]["point_add_requests"])<n(&c["targets"][i]["cold"]["point_add_requests"]));cache_negs_lower+=usize::from(n(&c["targets"][i]["cold"]["point_negations"])<n(&d["targets"][i]["cold"]["point_negations"]));}
  }
  let got=load(&results.join(&name));assert!(got==wanted,"false direct-sign trace, demand, coverage, cache, query, scalar or source");pins.push(json!({"case":name,"sha256":sha(&fs::read(results.join(&name)).unwrap())}));all.push(wanted);
 }
 let index=load(&results.join("index.json"));assert_eq!(index,json!({"schema":"quadratic-twist-direct-sign/v1","cases":pins,"policies":10,"setups":setups,"targets":targets,"verified":verified,"unit":"unpriced algorithm request vector; no scalar time/GAE"}));
 let mut totals=json!({});for arm in ["binary","cached","direct"]{for role in ["main","twist","matched","pulled_back_twist_base"]{let mut cold=Demand::new();for v in &all{for t in v[arm][role]["targets"].as_array().unwrap(){for k in KEYS{*cold.0.get_mut(k).unwrap()+=n(&t["cold"][k]);}}}totals[arm][role]=cold.value();}}
 let mut first=json!({});for arm in ["binary","cached","direct"]{for role in ["main","twist","matched","pulled_back_twist_base"]{first[arm][role]=json!({"setup":all[0][arm][role]["setup"]["demand"],"online":all[0][arm][role]["targets"][0]["online"],"cold":all[0][arm][role]["targets"][0]["cold"]});}}
 let mut comparisons=json!({});for arm in ["cached","direct"]{for key in ["point_add_requests","point_negations","scalar_iterations","dense_column_slots"]{let(mut lt,mut eq,mut gt)=(0,0,0);for v in &all{for i in 0..10{let m=n(&v[arm]["main"]["targets"][i]["cold"][key]);let t=n(&v[arm]["twist"]["targets"][i]["cold"][key]);lt+=usize::from(t<m);eq+=usize::from(t==m);gt+=usize::from(t>m);}}comparisons[arm][key]=json!({"twist_lower":lt,"equal":eq,"twist_higher":gt});}}
 write(receipt,&json!({"status":"passed","policies":10,"setups":setups,"targets":targets,"verified":verified,"batch_checked_solver_certificates":checked.len(),"index_sha256":sha(&fs::read(results.join("index.json")).unwrap()),"same_ordered_tested_operands_and_behavior":true,"direct_lower_cold_additions_than_cache":direct_adds_lower,"cache_lower_cold_negations_than_direct":cache_negs_lower,"cold_vector_sums_separate_targets":totals,"first_policy_target":first,"main_twist_cold_components":comparisons,"problems":0}));println!("PASS three sign arms: {setups} setups/{targets} source checks, {} batch-audited certificates; direct lower cold additions {direct_adds_lower}/400, cache lower negations {cache_negs_lower}/400",checked.len());
}
fn direct_corrupt(kind:&str,dir:&Path){let ip=dir.join("index.json");let mut index=load(&ip);let name=index["cases"][0]["case"].as_str().unwrap().to_owned();let path=dir.join(&name);let mut v=load(&path);
 match kind{
  "false-trace"=>v["direct"]["main"]["setup"]["queries"][0]["sign_trace_sha256"]=json!("0".repeat(64)),
  "false-demand"=>v["direct"]["main"]["targets"][0]["online"]["point_add_requests"]=json!(999),
  "false-cache-point"=>v["cached"]["main"]["setup"]["signed_cache"][0]["point"]=v["cached"]["main"]["setup"]["signed_cache"][1]["point"].clone(),
  "missing-arm"=>{v.as_object_mut().unwrap().remove("direct");},
  "false-scalar"=>v["direct"]["main"]["targets"][0]["answer"]=json!(999),
  "false-source"=>v["source_sha256"]=json!("0".repeat(64)),
  "missing-failed-query"=>{let mut found=false;for role in ["main","twist","matched","pulled_back_twist_base"]{for t in v["direct"][role]["targets"].as_array_mut().unwrap(){let a=t["attempts"].as_array_mut().unwrap();if let Some(i)=a.iter().position(|q|q["root_pairs"]==0){a.remove(i);found=true;break;}}if found{break;}}assert!(found);},
  _=>panic!("unknown control")
 }write(&path,&v);index["cases"][0]["sha256"]=json!(sha(&fs::read(&path).unwrap()));write(&ip,&index);
}
const PROOF:&str="research/quadratic_twist_evidence_20261021/lean-signed-cache/SignedCache.lean";
fn cv(n:I)->Ex{Ex::Const(n)}
fn vv(s:&str)->Ex{Ex::var(s)}
fn es(xs:Vec<Ex>)->Ex{xs.into_iter().fold(cv(0),Ex::plus)}
fn ep(xs:Vec<Ex>)->Ex{Ex::product(xs)}
fn esq(x:Ex)->Ex{ep(vec![x.clone(),x])}
fn certificate_statement()->IdentityStatement{
 let y=vv("y");IdentityStatement{name:"signed-cache-inverse-coordinate".into(),source:format!("{PROOF}:SignedCache.coordinate_inverse; polynomial only"),variables:vec!["y".into()],degree_bound:1,lhs:es(vec![y.clone(),ep(vec![cv(-1),y])]),rhs:cv(0)}
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
        identity_certificate::issue(certificate_statement(),20261025,32,"Inverse coordinate polynomial only; group, list and recovery implications separately proved",Some(ProofRef{kind:"lean4".into(),path:PROOF.into(),sha256:sha(&fs::read(root.join(PROOF)).unwrap())})).unwrap()
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
    println!("Source-bound signed cache coordinate certificate {}, 32+16 points and controls pass",c.id);
}
fn main(){let a:Vec<_>=std::env::args().collect();match a.get(1).map(String::as_str){
 Some("audit")=>{assert_eq!(a.len(),7);direct_audit(Path::new(&a[2]),Path::new(&a[3]),Path::new(&a[4]),Path::new(&a[5]),Path::new(&a[6]));},
 Some("corrupt")=>{assert_eq!(a.len(),4);direct_corrupt(&a[2],Path::new(&a[3]));},
 Some("certificate")=>{assert_eq!(a.len(),5);certificate(&a[2],Path::new(&a[3]),Path::new(&a[4]));},
 _=>panic!("audit REDUCED_CASES ADAPTIVE_CASES SIGNED_CASES RESULTS RECEIPT | corrupt KIND RESULTS | certificate check ROOT DIRECTORY")}}
