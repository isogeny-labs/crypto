//! Independent point, polynomial, exact Macaulay and rank replay. No timings.
#![allow(dead_code)]
#[path = "../../../../src/hash/sha256.rs"]
pub mod sha_backend;
pub mod hash {
    pub use crate::sha_backend as sha256;
}
#[path = "../../../../src/cryptanalysis/identity_certificate.rs"]
mod identity_certificate;

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

// Exact rational/state arithmetic is independent of the producer's GMP and
// recursive implementation. These paths are compiled only in this preflight.
use num_bigint::BigInt;
use num_traits::{One, Zero, Signed};
#[derive(Clone,Debug,Eq,PartialEq)]
struct Fraction {num:BigInt,den:BigInt}
impl Fraction {
 fn make(mut num:BigInt,mut den:BigInt)->Self{
  assert!(!den.is_zero(),"zero rational denominator");if den.is_negative(){num=-num;den=-den;}
  let(mut a,mut b)=(num.abs(),den.clone());while !b.is_zero(){let rem=&a%&b;a=b;b=rem;}
  Self{num:num/&a,den:den/a}
 }
 fn integer(n:I)->Self{Self::make(n.into(),BigInt::one())}
 fn ratio(n:I,d:I)->Self{Self::make(n.into(),d.into())}
 fn add(&self,b:&Self)->Self{Self::make(&self.num*&b.den+&b.num*&self.den,&self.den*&b.den)}
 fn divide(&self,d:I)->Self{assert!(d>0);Self::make(self.num.clone(),&self.den*BigInt::from(d))}
 fn le(&self,b:&Self)->bool{&self.num*&b.den<=&b.num*&self.den}
 fn lt(&self,b:&Self)->bool{&self.num*&b.den<&b.num*&self.den}
 fn value(&self)->Value{json!({"num":self.num.to_string(),"den":self.den.to_string()})}
 fn parse(v:&Value)->Self{let a:BigInt=v["num"].as_str().unwrap().parse().unwrap();let b:BigInt=v["den"].as_str().unwrap().parse().unwrap();let q=Self::make(a,b);assert_eq!(q.value(),*v,"noncanonical rational");q}
}
type Rows=Vec<Vec<I>>;
fn canonical(rows:Rows,b:usize,r:I)->Rows{let(out,_)=rref(rows,b,r);out}
fn state_key(v:&Rows)->String{serde_json::to_string(v).unwrap()}
#[derive(Clone)]
struct Recurrence {calls:Fraction,original:Fraction,reduced:Fraction}
fn distribution(queries:&[Value],r:I,b:usize)->Value{
 let nquery=r-1;assert_eq!(queries.len(),nquery as usize);
 let responses:Vec<_>=queries.iter().map(|q|canonical(q["rows"].as_array().unwrap().iter().map(ints).collect(),b,r)).collect();
 let positive=responses.iter().filter(|q|!q.is_empty()).count() as I;
 let rank_sum=responses.iter().map(Vec::len).sum::<usize>() as I;
 let original_sum: I=queries.iter().map(|q|n(&q["original"]["matrix_degree"])).sum();
 let reduced_sum: I=queries.iter().map(|q|n(&q["reduced"]["matrix_degree"])).sum();
 // Breadth-first closure first; then a descending-rank solution. No recursive
 // calls or producer state keys/rationals are used as an arithmetic oracle.
 let zero:Rows=vec![];let mut bases:BTreeMap<String,Rows>=[(state_key(&zero),zero.clone())].into();let mut queue=vec![zero];let mut cursor=0;
 while cursor<queue.len(){let state=queue[cursor].clone();cursor+=1;if state.len()==b{continue;}
  for rr in &responses{let mut next=state.clone();next.extend(rr.clone());let next=canonical(next,b,r);let key=state_key(&next);if !bases.contains_key(&key){bases.insert(key,next.clone());queue.push(next);}}
 }
 let mut ordered:Vec<_>=bases.iter().map(|(k,v)|(k.clone(),v.clone())).collect();ordered.sort_by(|a,b|b.1.len().cmp(&a.1.len()).then(a.0.cmp(&b.0)));
 let mut solved:BTreeMap<String,Recurrence>=BTreeMap::new();let mut nodes:BTreeMap<String,Value>=BTreeMap::new();
 for(key,state)in ordered{
  if state.len()==b{let z=Fraction::integer(0);solved.insert(key.clone(),Recurrence{calls:z.clone(),original:z.clone(),reduced:z.clone()});nodes.insert(key.clone(),json!({"key":key,"basis":state,"absorbing":true,"calls":z.value(),"original_degree_requests":z.value(),"reduced_degree_requests":z.value()}));continue;}
  let mut stays=0;let(mut calls,mut original,mut reduced)=(Fraction::integer(nquery),Fraction::integer(original_sum),Fraction::integer(reduced_sum));let mut transitions=vec![];
  for rr in &responses{let mut next=state.clone();next.extend(rr.clone());let next=canonical(next,b,r);let next_key=state_key(&next);transitions.push(next_key.clone());if next.len()==state.len(){assert_eq!(next_key,key,"rank-preserving move must stay in same subspace");stays+=1;}else{assert!(next.len()>state.len());let e=&solved[&next_key];calls=calls.add(&e.calls);original=original.add(&e.original);reduced=reduced.add(&e.reduced);}}
  assert!(stays<nquery,"nonabsorbing reachable rank state");let e=Recurrence{calls:calls.divide(nquery-stays),original:original.divide(nquery-stays),reduced:reduced.divide(nquery-stays)};
  nodes.insert(key.clone(),json!({"key":key,"basis":state,"absorbing":false,"stays":stays,"transitions":transitions,"calls":e.calls.value(),"original_degree_requests":e.original.value(),"reduced_degree_requests":e.reduced.value()}));solved.insert(key,e);
 }
 let e=&solved["[]"];json!({"N":nquery,"positive":positive,"rank_sum":rank_sum,"original_degree_sum":original_sum,"reduced_degree_sum":reduced_sum,"useful_probability":Fraction::ratio(positive,nquery).value(),"mean_rank_gain":Fraction::ratio(rank_sum,nquery).value(),"mean_original_degree":Fraction::ratio(original_sum,nquery).value(),"mean_reduced_degree":Fraction::ratio(reduced_sum,nquery).value(),"expected_calls":e.calls.value(),"expected_original_degree_requests":e.original.value(),"expected_reduced_degree_requests":e.reduced.value(),"states":nodes.into_values().collect::<Vec<_>>()})
}
#[derive(Clone,Copy)]
struct Fixture {prime:I,r:I,e:Curve,t:Curve,p:Point,u:Point}
impl Fixture {
 fn new(prime:I)->Self{assert!([43,13].contains(&prime));Self{prime,r:if prime==43{11}else{7},e:Curve{p:prime,a:1,b:if prime==43{0}else{4}},t:Curve{p:prime,a:4,b:if prime==43{0}else{6}},p:if prime==43{Some((31,18))}else{Some((0,2))},u:if prime==43{Some((24,26))}else{Some((8,11))}}}
 fn image(self,q:Point)->Point{q.and_then(|(x,y)|if self.prime==43{Some((red(41*x,43),red(11*y,43)))}else if x==10{None}else{let v=inv(x-10,13);Some((red(12*(x+2*v),13),red(8*y*(1-2*v*v),13)))})}
 fn inverse(self,q:Point)->Point{if self.prime==43{q.map(|(x,y)|(red(21*x,43),red(4*y,43)))}else{(0..self.r).map(|n|self.e.mul(n,self.p)).find(|&a|self.image(a)==q).expect("public base inverse")}}
}
// Ordinary monic division by the ACTUAL supplied membership polynomial.
// This does not use the legacy size-two shortcut for a size-three base.
fn divide_membership(mut f: Poly, xs: &[I], p: I) -> Poly {
 assert!([2,3].contains(&xs.len()));
 assert_eq!(xs.iter().copied().collect::<BTreeSet<_>>().len(),xs.len());
 let b=xs.len();
 for variable in 0..2 {
  let g=base_poly(xs,variable,p);
  let leading=if variable==0{(b,0)}else{(0,b)};
  assert_eq!(g.get(&leading),Some(&1),"monic membership");
  loop {
   let found=f.iter().rev().find(|((i,j),_)|if variable==0{*i>=b}else{*j>=b}).map(|(&m,&c)|(m,c));
   let Some(((i,j),c))=found else{break};
   let shift=if variable==0{(i-b,j)}else{(i,j-b)};
   for (&(u,v),&a) in &g {add_term(&mut f,(u+shift.0,v+shift.1),-c*a,p);}
  }
 }
 assert!(f.keys().all(|&(i,j)|i<b&&j<b));
 f
}
fn query(c:Curve,r:Point,base:&[Point],modulus:I)->Value{
 let b=base.len();let xs:Vec<_>=base.iter().map(|q|q.unwrap().0).collect();let z=r.expect("affine query").0;let f=s3_poly(c,z);let h=divide_membership(f.clone(),&xs,c.p);if b==3{assert!(f.keys().all(|&(i,j)|i<3&&j<3));assert_eq!(h,f,"cubic membership changes no S3 monomial");}let gx=base_poly(&xs,0,c.p);let gy=base_poly(&xs,1,c.p);let mut scratch=Demand::new();
 let raw=request_solve([f.clone(),gx.clone(),gy.clone()],c.p,&mut scratch,false,&xs);let reduced=request_solve([h.clone(),gx.clone(),gy.clone()],c.p,&mut scratch,false,&xs);
 audit_solve(&raw,[f,gx.clone(),gy.clone()],c.p);audit_solve(&reduced,[h,gx,gy],c.p);assert_eq!(raw["roots"],reduced["roots"]);
 let(mut tested,mut lifts,mut rows,mut actual)=(vec![],vec![],BTreeSet::new(),vec![]);
 for i in 0..b{for j in 0..b{let mut any=false;for si in [-1,1]{for sj in [-1,1]{let a=if si<0{c.neg(base[i])}else{base[i]};let bb=if sj<0{c.neg(base[j])}else{base[j]};let sum=c.add(a,bb);tested.push(json!({"i":i,"j":j,"si":si,"sj":sj,"A":point_json(a),"B":point_json(bb),"sum":point_json(sum),"accepted":sum==r}));if sum==r{any=true;lifts.push(vec![i as I,j as I,si,sj]);let mut row=vec![0;b];row[i]=red(row[i]+si,modulus);row[j]=red(row[j]+sj,modulus);rows.insert(row);}}}if any{actual.push(vec![xs[i],xs[j]]);}}}
 actual.sort();assert_eq!(json!(actual),reduced["roots"],"actual-point root lifting");json!({"target":point_json(r),"original":raw,"reduced":reduced,"tested":tested,"lifts":lifts,"rows":rows})
}
fn arm(f:Fixture,c:Curve,g:Point,base:Vec<Point>,mapped:bool)->Value{
 let b=base.len();let(mut queries,mut setup,mut equations)=(vec![],vec![],Rows::new());
 for m in 1..f.r{let mut q=query(c,c.mul(m,g),&base,f.r);let rank=rref(equations.clone(),b+1,f.r).1.len();if rank<b{for row in q["rows"].as_array().unwrap(){let mut row=ints(row);row.push(m);equations.push(row);}let(rank_rows,piv)=rref(equations.clone(),b+1,f.r);assert!(!piv.contains(&b));equations=rank_rows;setup.push(json!({"m":m,"query":q,"rank":piv.len()}));}q["m"]=json!(m);queries.push(q);}
 let(_,piv)=rref(equations.clone(),b+1,f.r);assert_eq!(piv.len(),b);let logs=solve_linear(equations,b,f.r);for i in 0..b{assert_eq!(c.mul(logs[i],g),base[i],"actual base-log verification");}
 let recover=|q:Point|{let image=if mapped{f.image(q)}else{q};let(mut attempts,mut answer)=(vec![],None);for shift in 0..f.r{if answer.is_some(){break;}let r=c.add(image,c.mul(shift,g));if r.is_none(){attempts.push(json!({"shift":shift,"identity":true}));continue;}let mut v=query(c,r,&base,f.r);v["shift"]=json!(shift);v["identity"]=json!(false);if let Some(row)=v["lifts"].as_array().unwrap().first(){let l=ints(row);answer=Some(red(l[2]*logs[l[0] as usize]+l[3]*logs[l[1] as usize]-shift,f.r));}attempts.push(v);}let a=answer.expect("source target not recovered");assert_eq!(f.e.mul(a,f.p),q,"source scalar verification");json!({"source":point_json(q),"image":point_json(image),"answer":a,"attempts":attempts})};
 let rec:Vec<_>=(1..f.r).map(|n|recover(f.e.mul(n,f.p))).collect();let uniform=distribution(&queries,f.r,b);json!({"p":f.prime,"r":f.r,"b":b,"generator":point_json(g),"base_points":base.iter().copied().map(point_json).collect::<Vec<_>>(),"queries":queries,"setup_queries":setup,"base_logs":logs,"recoveries":rec,"uniform":uniform})
}
fn no_states(mut v:Value)->Value{v.as_object_mut().unwrap().remove("states");v}
fn frontiers(arms:&BTreeMap<String,Value>,prime:I,b:usize,count:usize)->Vec<Value>{
 ["main","twist"].into_iter().map(|role|{let mut group=json!({"p":prime,"b":b,"role":role});for mode in ["original","reduced"]{let mut kept=vec![];for i in 0..count{let a=&arms[&format!("{role}{i}")]["uniform"];let mut dominated=false;for j in 0..count{if i==j{continue;}let z=&arms[&format!("{role}{j}")]["uniform"];let(ar,zr)=(n(&a["rank_sum"]),n(&z["rank_sum"]));let(ad,zd)=(n(&a[format!("{mode}_degree_sum")]),n(&z[format!("{mode}_degree_sum")]));let(ac,zc)=(Fraction::parse(&a["expected_calls"]),Fraction::parse(&z["expected_calls"]));if zr>=ar&&zd<=ad&&zc.le(&ac)&&(zr>ar||zd<ad||zc.lt(&ac)){dominated=true;break;}}if !dominated{kept.push(i);}}group[mode]=json!(kept);}group}).collect()
}
fn audit_uniform(root:&Path,results:&Path,receipt:&Path){
 assert!(!receipt.exists(),"fresh receipt path required");let observed_files:BTreeSet<_>=fs::read_dir(results).unwrap().map(|e|{let e=e.unwrap();assert!(e.file_type().unwrap().is_file(),"unexpected directory in canonical case set");e.file_name().into_string().unwrap()}).collect();let(mut pins,mut comparisons,mut all_frontiers)=(vec![],vec![],vec![]);let(mut arm_count,mut targets,mut original_joint,mut reduced_joint)=(0,0,0,0);let mut total_states=0;let mut failed_queries=0;
 for prime in [43,13]{let f=Fixture::new(prime);for n in 0..f.r{assert_eq!(f.image(f.e.mul(n,f.p)),f.t.mul(n,f.u),"actual source reduction");}for b in [2,3]{let mut arms=BTreeMap::new();let mut count=0;for(role,c,g,mapped)in [("main",f.e,f.p,false),("twist",f.t,f.u,true)]{let xs=c.xs(g,f.r);let subsets=subs(&xs,b);count=subsets.len();for(i,xs)in subsets.iter().enumerate(){let base=c.reps(xs);arms.insert(format!("{role}{i}"),arm(f,c,g,base.clone(),mapped));let other:Vec<_>=base.into_iter().map(|q|if mapped{f.inverse(q)}else{f.image(q)}).collect();arms.insert(format!("{}{i}",if mapped{"pullback"}else{"matched"}),arm(f,if mapped{f.e}else{f.t},if mapped{f.p}else{f.u},other,!mapped));}}
  for i in 0..count{for(a,z)in [("main","matched"),("twist","pullback")]{let l=&arms[&format!("{a}{i}")];let rr=&arms[&format!("{z}{i}")];for m in 0..(f.r-1)as usize{for key in ["rows","lifts"]{assert_eq!(l["queries"][m][key],rr["queries"][m][key],"matched accepted coefficient responses");}}for key in ["positive","rank_sum","expected_calls"]{assert_eq!(l["uniform"][key],rr["uniform"][key],"matched useful profiles");}}}
  for i in 0..count{for j in 0..count{let a=&arms[&format!("main{i}")]["uniform"];let z=&arms[&format!("twist{j}")]["uniform"];let useful=n(&z["rank_sum"])>n(&a["rank_sum"]);let joint_original=useful&&n(&z["original_degree_sum"])<n(&a["original_degree_sum"]);let joint_reduced=useful&&n(&z["reduced_degree_sum"])<n(&a["reduced_degree_sum"]);original_joint+=usize::from(joint_original);reduced_joint+=usize::from(joint_reduced);comparisons.push(json!({"p":prime,"b":b,"main":i,"twist":j,"main_uniform":no_states(a.clone()),"twist_uniform":no_states(z.clone()),"joint_original":joint_original,"joint_reduced":joint_reduced}));}}
  all_frontiers.extend(frontiers(&arms,prime,b,count));for(label,v)in &arms{let name=format!("p{prime}-b{b}-{label}");let bytes=fs::read(results.join(format!("{name}.json"))).expect("missing base/model arm");let supplied:Value=serde_json::from_slice(&bytes).unwrap();assert_eq!(&supplied,v,"false uniform row,degree,witness,coverage,state,rational,query,log or scalar");pins.push(json!({"name":name,"sha256":sha(&bytes)}));arm_count+=1;targets+=f.r-1;total_states+=v["uniform"]["states"].as_array().unwrap().len();for r in v["recoveries"].as_array().unwrap(){for a in r["attempts"].as_array().unwrap(){if a.get("lifts").is_some_and(|l|l.as_array().unwrap().is_empty()){failed_queries+=1;}}}}
 }}assert_eq!(arm_count,96);assert_eq!(targets,896);assert_eq!(comparisons.len(),210);
 let study="research/quadratic_twist_evidence_20261021";let sources:Vec<_>=["native/uniform-yield-v2.cpp","native/phase-adaptive-prior.hpp","UNIFORM_YIELD_20261025_PROTOCOL.md","lean-cm8/CM8Transfer.lean","lean/S3System.lean"].into_iter().map(|p|{let path=format!("{study}/{p}");json!({"path":path,"sha256":sha(&fs::read(root.join(&path)).unwrap())})}).collect();
 let mut expected_files:BTreeSet<_>=pins.iter().map(|p|format!("{}.json",p["name"].as_str().unwrap())).collect();expected_files.insert("index.json".to_owned());assert_eq!(observed_files,expected_files,"missing or extra canonical case");
 let expected=json!({"schema":"quadratic-twist-uniform-yield/v1","arms":pins,"arm_count":arm_count,"targets":targets,"comparisons":comparisons,"frontiers":all_frontiers,"joint_original":original_joint,"joint_reduced":reduced_joint,"sources":sources});let bytes=fs::read(results.join("index.json")).unwrap();assert_eq!(serde_json::from_slice::<Value>(&bytes).unwrap(),expected,"false uniform index, source or coverage");
 write(receipt,&json!({"status":"passed","arms":arm_count,"targets":targets,"comparisons":210,"joint_original":original_joint,"joint_reduced":reduced_joint,"reachable_states":total_states,"failed_queries":failed_queries,"missing_failed_query_control":if failed_queries==0{"null-no-failed-query"}else{"applicable"},"index_sha256":sha(&bytes),"independent_arithmetic":true,"semantic_witness_audit":true}));
}
fn corrupt_uniform(kind:&str,results:&Path)->bool{
 let index_path=results.join("index.json");let mut index=load(&index_path);if kind=="false-source"{index["sources"][0]["sha256"]=json!("0".repeat(64));write(&index_path,&index);return true;}if kind=="missing-arm"{index["arms"].as_array_mut().unwrap().pop();write(&index_path,&index);return true;}if kind=="false-frontier"{index["frontiers"][0]["original"]=json!([999]);write(&index_path,&index);return true;}
 let mut name="p43-b2-main0".to_owned();let mut v=load(&results.join(format!("{name}.json")));if kind=="missing-failed-query"{let mut found=false;for pin in index["arms"].as_array().unwrap(){let label=pin["name"].as_str().unwrap();let mut q=load(&results.join(format!("{label}.json")));for target in q["recoveries"].as_array_mut().unwrap(){let a=target["attempts"].as_array_mut().unwrap();if let Some(i)=a.iter().position(|x|x.get("lifts").is_some_and(|l|l.as_array().unwrap().is_empty())){a.remove(i);found=true;break;}}if found{name=label.to_owned();v=q;break;}}if !found{return false;}}
 else{match kind{"false-row"=>v["queries"][0]["rows"]=json!([]),"false-degree"=>v["queries"][0]["reduced"]["matrix_degree"]=json!(99),"false-witness"=>v["queries"][0]["reduced"]["certificate"][0]["witness"][0]=json!([[0,0,99]]),"missing-target"=>{v["queries"].as_array_mut().unwrap().pop();},"false-transition"=>{let s=v["uniform"]["states"].as_array_mut().unwrap().iter_mut().find(|s|s["absorbing"]==false).unwrap();s["transitions"][0]=json!("[[-1]]");},"false-rational"=>v["uniform"]["expected_calls"]=json!({"num":"999999","den":"1"}),"missing-query"=>{v["recoveries"][0]["attempts"].as_array_mut().unwrap().pop();},"false-log"=>v["base_logs"][0]=json!(99),"false-scalar"=>v["recoveries"][0]["answer"]=json!(99),_=>panic!("unknown corruption")}}
 let bytes=format!("{}\n",serde_json::to_string(&v).unwrap());fs::write(results.join(format!("{name}.json")),&bytes).unwrap();for pin in index["arms"].as_array_mut().unwrap(){if pin["name"]==name{pin["sha256"]=json!(sha(bytes.as_bytes()));}}write(&index_path,&index);true
}
fn main(){let a:Vec<_>=std::env::args().collect();match a.get(1).map(String::as_str){Some("audit")=>{assert_eq!(a.len(),5);audit_uniform(Path::new(&a[2]),Path::new(&a[3]),Path::new(&a[4]));},Some("corrupt")=>{assert_eq!(a.len(),4);let applied=corrupt_uniform(&a[2],Path::new(&a[3]));println!("{}",if applied{"corrupted-and-rehashed"}else{"null-no-failed-query"});},_=>panic!("audit ROOT RESULTS FRESH_RECEIPT | corrupt KIND RESULTS")}}
