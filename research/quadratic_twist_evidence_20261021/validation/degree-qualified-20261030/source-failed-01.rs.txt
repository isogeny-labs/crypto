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
fn lead(f:&Poly)->(usize,usize){*f.keys().max_by_key(|&&(x,y)|(x+y,x)).unwrap()}
fn divides(a:(usize,usize),b:(usize,usize))->bool{a.0<=b.0&&a.1<=b.1}
fn scale_poly(f:&mut Poly,c:I){for a in f.values_mut(){*a=red(*a*c,13);}f.retain(|_,a|*a!=0);}
fn sub(a:&mut ProofRow,b:&ProofRow,c:I){for(&m,&v)in &b.f{add_term(&mut a.f,m,-c*v,13);}for j in 0..3{for(&m,&v)in &b.w[j]{add_term(&mut a.w[j],m,-c*v,13);}}}
fn moved(r:&ProofRow,s:(usize,usize))->ProofRow{ProofRow{f:shifted(&r.f,s),w:std::array::from_fn(|j|shifted(&r.w[j],s))}}
fn basis(pool:&[ProofRow],d:usize,orig:&[Poly;3],peak:&mut usize)->Vec<ProofRow>{
 let mut piv:BTreeMap<(usize,usize),ProofRow>=BTreeMap::new();
 for g in pool.iter().filter(|g|!g.f.is_empty()&&degree(&g.f)<=d){for s in desc_monomials(d-degree(&g.f)){
  let mut row=moved(g,s);*peak=(*peak).max(exp_degree(&row,orig));assert!(*peak<=d);
  while !row.f.is_empty(){let l=lead(&row.f);let c=row.f[&l];if let Some(h)=piv.get(&l){sub(&mut row,h,c);}else{let z=inv(c,13);scale_poly(&mut row.f,z);for w in &mut row.w{scale_poly(w,z);}piv.insert(l,row);break;}}
 }}
 let mut keys:Vec<_>=piv.keys().copied().collect();keys.sort_by_key(|&(x,y)|(x+y,x));
 for &m in &keys{for &q in &keys{if (m.0+m.1,m.0)<(q.0+q.1,q.0){if let Some(&c)=piv[&q].f.get(&m){let h=piv[&m].clone();sub(piv.get_mut(&q).unwrap(),&h,c);}}}}
 keys.reverse();keys.into_iter().map(|m|piv.remove(&m).unwrap()).collect()
}
fn normal(mut r:ProofRow,g:&[ProofRow],orig:&[Poly;3],cap:usize,peak:&mut usize,denied:&mut usize)->bool{
 let e=exp_degree(&r,orig);if e>cap{*denied=(*denied).max(e);return false;}*peak=(*peak).max(e);
 for _ in 0..1000{if r.f.is_empty(){return true;}let m=lead(&r.f);let Some(h)=g.iter().find(|h|divides(lead(&h.f),m))else{return false;};let l=lead(&h.f);let s=(m.0-l.0,m.1-l.1);let bound=exp_degree(h,orig)+s.0+s.1;if bound>cap{*denied=(*denied).max(bound);return false;}*peak=(*peak).max(bound);let z=moved(h,s);let c=r.f[&m];sub(&mut r,&z,c);
 }panic!("division limit")
}
fn row_json(r:&ProofRow,f:&[Poly;3])->Value{assert_eq!(expansion(r,f,13),r.f,"original-input ideal witness");json!({"poly":poly_json(&r.f),"witness":r.w.iter().map(poly_json).collect::<Vec<_>>(),"expanded_degree":exp_degree(r,f)})}
fn solve_cap(f:[Poly;3],xs:&[I],cap:usize)->Value{
 let pool:Vec<_>=(0..3).map(|j|ProofRow{f:f[j].clone(),w:std::array::from_fn(|k|if k==j{[((0,0),1)].into()}else{Poly::new()})}).collect();let(mut peak,mut trace)=(0,vec![]);
 for d in 0..=cap{let b=basis(&pool,d,&f,&mut peak);let g:Vec<_>=b.iter().filter(|r|!b.iter().any(|h|lead(&h.f)!=lead(&r.f)&&divides(lead(&h.f),lead(&r.f)))).cloned().collect();let unit=g.iter().any(|r|r.f==[((0,0),1)].into());let(mut ok,mut denied)=(unit,0);
  if !unit&&!g.is_empty(){ok=true;for r in &pool{if !normal(r.clone(),&g,&f,d,&mut peak,&mut denied){ok=false;break;}}
   if ok{'pairs:for i in 0..g.len(){for j in i+1..g.len(){let(a,b)=(lead(&g[i].f),lead(&g[j].f));let l=(a.0.max(b.0),a.1.max(b.1));let(u,v)=((l.0-a.0,l.1-a.1),(l.0-b.0,l.1-b.1));let bound=(exp_degree(&g[i],&f)+u.0+u.1).max(exp_degree(&g[j],&f)+v.0+v.1);if bound>d{denied=denied.max(bound);ok=false;break 'pairs;}let mut z=moved(&g[i],u);let w=moved(&g[j],v);peak=peak.max(bound);sub(&mut z,&w,1);if !normal(z,&g,&f,d,&mut peak,&mut denied){ok=false;break 'pairs;}}}}
  }
  assert!(peak<=d);trace.push(json!({"d":d,"rank":b.len(),"peak":peak,"denied_bound":denied,"certified":ok}));
  if ok{let mut roots=vec![];if !unit{for &x in xs{for &y in xs{if f.iter().all(|h|ev(h,x,y,13)==0){roots.push(vec![x,y]);}}}roots.sort();}
   // Additional exhaustive oracle is an AUDIT ONLY and never controls acceptance.
   assert_eq!(roots,root_set(&f,13).iter().map(|&(x,y)|vec![x,y]).collect::<Vec<_>>(),"certified root completeness");
   return json!({"status":if unit{"unit"}else{"gb"},"matrix_degree":d,"expanded_peak":peak,"certificate":g.iter().map(|r|row_json(r,&f)).collect::<Vec<_>>(),"roots":roots,"trace":trace});
  }
 }json!({"status":"cap","matrix_degree":Value::Null,"expanded_peak":peak,"certificate":[],"roots":[],"trace":trace})
}
fn rows_for(c:Curve,q:Point,base:&[Point],roots:Option<&Value>)->Value{let(mut rows,mut lifts)=(BTreeSet::new(),vec![]);
 for i in 0..base.len(){for j in 0..base.len(){let xy=vec![base[i].unwrap().0,base[j].unwrap().0];if roots.is_some_and(|rr|!rr.as_array().unwrap().contains(&json!(xy))){continue;}
  for si in [-1,1]{for sj in [-1,1]{let a=if si<0{c.neg(base[i])}else{base[i]};let b=if sj<0{c.neg(base[j])}else{base[j]};if c.add(a,b)==q{let mut v=vec![0;base.len()];v[i]=red(v[i]+si,7);v[j]=red(v[j]+sj,7);rows.insert(v);lifts.push(vec![i as I,j as I,si,sj]);}}}
 }}json!({"rows":rows,"lifts":lifts})}
const E:Curve=Curve{p:13,a:1,b:4};const T:Curve=Curve{p:13,a:4,b:6};const P:Point=Some((0,2));
fn image(q:Point)->Point{q.and_then(|(x,y)|if x==10{None}else{let v=inv(x-10,13);Some((red(12*(x+2*v),13),red(8*y*(1-2*v*v),13)))})}
fn query(c:Curve,q:Point,base:&[Point],reduced:bool,cap:usize)->Value{let xs:Vec<_>=base.iter().map(|b|b.unwrap().0).collect();let f=s3_poly(c,q.unwrap().0);let f=if reduced{divide_base(f,&xs,13)}else{f};let cert=solve_cap([f,base_poly(&xs,0,13),base_poly(&xs,1,13)],&xs,cap);let rows=rows_for(c,q,base,Some(&cert["roots"]));let reference=rows_for(c,q,base,None);if cert["status"]!="cap"{assert_eq!(rows,reference,"point lift completeness");}let rank=rref(rows["rows"].as_array().unwrap().iter().map(ints).collect(),base.len(),7).1.len();json!({"point":point_json(q),"solver":cert,"admitted":rows,"reference":reference,"rank":rank})}
fn as_point(v:&Value)->Point{let v=ints(v);if v==[-1,-1]{None}else{assert_eq!(v.len(),2);Some((v[0],v[1]))}}
fn configuration(input:&Value,label:&str,reduced:bool,cap:usize)->Value{
 let mapped=label.starts_with("twist")||label.starts_with("matched");let c=if mapped{T}else{E};let g=as_point(&input["generator"]);let base:Vec<_>=input["base_points"].as_array().unwrap().iter().map(as_point).collect();let b=base.len();for &q in &base{assert!(c.contains(q));assert_eq!(c.mul(7,q),None);}
 let queries:Vec<_>=(1..7).map(|m|query(c,c.mul(m,g),&base,reduced,cap)).collect();
 let fresh=|q:Point|{let v=query(c,q,&base,reduced,cap);let m=(1..7).find(|&m|c.mul(m,g)==q).unwrap();assert_eq!(v,queries[(m-1)as usize],"fresh query binding");(v,m)};
 let(mut equations,mut setup,mut rank)=(vec![],vec![],0);for m in 1..7{if rank==b{break;}let a=fresh(c.mul(m,g)).0;for r in a["admitted"]["rows"].as_array().unwrap(){let mut v=ints(r);v.push(m);equations.push(v);}let rr=rref(equations.clone(),b+1,7);assert!(!rr.1.contains(&b));rank=rr.1.len();equations=rr.0;setup.push(json!({"m":m,"rank":rank}));}
 let ready=rank==b;let logs=if ready{solve_linear(equations,b,7)}else{vec![]};for(i,&l)in logs.iter().enumerate(){assert_eq!(c.mul(l,g),base[i]);}
 let recover=|q:Point|{let mut answer=None;let mut attempts=vec![];let m=if mapped{image(q)}else{q};if ready{for shift in 0..7{if answer.is_some(){break;}let r=c.add(m,c.mul(shift,g));if r.is_none(){attempts.push(json!({"shift":shift,"identity":true}));continue;}let(a,m)=fresh(r);attempts.push(json!({"shift":shift,"identity":false,"m":m}));if let Some(l)=a["admitted"]["lifts"].as_array().unwrap().first(){let l=ints(l);let a=red(l[2]*logs[l[0]as usize]+l[3]*logs[l[1]as usize]-shift,7);assert_eq!(E.mul(a,P),q,"ORIGINAL main scalar");answer=Some(a);}}}json!({"source":point_json(q),"answer":answer,"status":if !ready{"setup-blocked"}else if answer.is_none(){"target-failed"}else{"verified"},"attempts":attempts})};
 let targets:Vec<_>=(1..7).map(|n|recover(E.mul(n,P))).collect();let direct:Vec<_>=(1..7).map(|n|{let q=E.mul(n,P);let a=(1..7).find(|&a|E.mul(a,P)==q).unwrap();json!({"source":point_json(q),"answer":a,"tests":a})}).collect();
 json!({"arm":label,"b":b,"mode":if reduced{"reduced"}else{"original"},"cap":cap,"source_sha256":input["input_sha256"],"queries":queries,"setup":setup,"ready":ready,"logs":logs,"targets":targets,"direct_generic_reference":direct})
}
fn load(p:&Path)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn write(p:&Path,v:&Value){fs::write(p,format!("{}\n",serde_json::to_string(v).unwrap())).unwrap()}
fn audit(root:&Path,out:&Path,receipt:&Path){assert!(!receipt.exists(),"fresh receipt");let index=load(&out.join("index.json"));let files=index["files"].as_array().unwrap();assert_eq!(files.len(),288);let observed:BTreeSet<_>=fs::read_dir(out).unwrap().map(|x|x.unwrap().file_name().into_string().unwrap()).collect();let wanted:BTreeSet<_>=files.iter().map(|x|x["name"].as_str().unwrap().to_owned()).chain(["index.json".to_owned()]).collect();assert_eq!(observed,wanted,"exact case membership");let src=root.join("research/quadratic_twist_evidence_20261021/native/uniform-20261029/cases");let mut all=BTreeMap::new();let(mut verified,mut blocked,mut failures)=(0,0,0);
 for pin in files{let name=pin["name"].as_str().unwrap();let bytes=fs::read(out.join(name)).unwrap();assert_eq!(sha(&bytes),pin["sha256"]);let got:Value=serde_json::from_slice(&bytes).unwrap();let arm=got["arm"].as_str().unwrap();let b=n(&got["b"]);let source=src.join(format!("p13-b{b}-{arm}.json"));let raw=fs::read(&source).unwrap();let mut input:Value=serde_json::from_slice(&raw).unwrap();input["input_sha256"]=json!(sha(&raw));let expected=configuration(&input,arm,got["mode"]=="reduced",n(&got["cap"])as usize);assert_eq!(got,expected,"independent bounded solver/setup/recovery {name}");blocked+=usize::from(!got["ready"].as_bool().unwrap());for t in got["targets"].as_array().unwrap(){verified+=usize::from(t["status"]=="verified");}for q in got["queries"].as_array().unwrap(){failures+=usize::from(q["solver"]["status"]=="cap");}all.insert(name.to_owned(),got);}
 for s in index["sources"].as_array().unwrap(){assert_eq!(sha(&fs::read(root.join(s["path"].as_str().unwrap())).unwrap()),s["sha256"],"source binding");}assert_eq!(n(&index["configurations"]),288);assert_eq!(n(&index["source_targets"]),1728);assert_eq!(n(&index["verified_recoveries"]),verified as I);assert_eq!(n(&index["blocked_setups"]),blocked as I);
 let(mut comparisons,mut joint,mut strongest)=(0,0,0);let mut panels=vec![];
 for mode in ["original","reduced"]{for b in [2,3]{let count=if b==2{3}else{1};for tc in 0..9{for mc in 0..9{for ti in 0..count{for mi in 0..count{let t=&all[&format!("p13-b{b}-twist{ti}-{mode}-{tc}.json")];let m=&all[&format!("p13-b{b}-main{mi}-{mode}-{mc}.json")];for q in 0..6{comparisons+=1;let tr=n(&t["queries"][q]["rank"]);let mr=n(&m["queries"][q]["rank"]);if tc<mc&&tr>mr&&t["targets"][q]["status"]=="verified"{joint+=1;let best=(0..count).map(|k|n(&all[&format!("p13-b{b}-main{k}-{mode}-{mc}.json")]["queries"][q]["rank"])).max().unwrap();strongest+=usize::from(tr>best);}}}}}}}
  for cap in 0..9{let mut panel=json!({"mode":mode,"cap":cap});for role in ["main","twist","matched","pullback"]{let group:Vec<_>=all.values().filter(|v|v["mode"]==mode&&n(&v["cap"])==cap&&v["arm"].as_str().unwrap().starts_with(role)).collect();panel[role]=json!({"accepted_queries":group.iter().map(|v|v["queries"].as_array().unwrap().iter().filter(|q|q["solver"]["status"]!="cap").count()).sum::<usize>(),"verified_targets":group.iter().map(|v|v["targets"].as_array().unwrap().iter().filter(|t|t["status"]=="verified").count()).sum::<usize>()});}panels.push(panel);}
 }
 write(receipt,&json!({"schema":"quadratic-twist-degree-qualified-audit/v1","configurations":288,"source_targets":1728,"verified_recoveries":verified,"blocked_setups":blocked,"capped_response_queries":failures,"policy_target_cap_comparisons":comparisons,"strict_joint_recovery_admitted":joint,"strongest_source_joint_pair_records":strongest,"panels":panels}));println!("288 configurations independently replayed; {verified} verified,{blocked} blocked setups; joint {joint},strongest {strongest}");
}
fn corrupt(kind:&str,out:&Path){let p=out.join("index.json");let mut index=load(&p);let pins=index["files"].as_array_mut().unwrap();if kind=="missing-arm"{let x=pins.remove(0);fs::remove_file(out.join(x["name"].as_str().unwrap())).unwrap();write(&p,&index);return;}let pin=if kind=="missing-failed-query"{pins.iter_mut().find(|s|load(&out.join(s["name"].as_str().unwrap()))["queries"].as_array().unwrap().iter().any(|q|q["solver"]["status"]=="cap")).unwrap()}else{pins.iter_mut().find(|s|load(&out.join(s["name"].as_str().unwrap()))["ready"]==true).unwrap()};let file=out.join(pin["name"].as_str().unwrap());let mut v=load(&file);
 match kind{"cap"=>v["cap"]=json!(0),"root"=>{v["queries"][0]["solver"]["roots"].as_array_mut().unwrap().push(json!([12,12]));},"row"=>{v["queries"][0]["admitted"]["rows"].as_array_mut().unwrap().push(json!([1,1]));},"witness"=>{let q=v["queries"].as_array_mut().unwrap().iter_mut().find(|q|!q["solver"]["certificate"].as_array().unwrap().is_empty()).unwrap();q["solver"]["certificate"][0]["witness"][0]=json!([[0,0,1]]);},"log"=>v["logs"][0]=json!((n(&v["logs"][0])+1)%7),"scalar"=>v["targets"][0]["answer"]=json!(0),"source"=>v["source_sha256"]=json!("00".repeat(32)),"missing-failed-query"=>{let i=v["queries"].as_array().unwrap().iter().position(|q|q["solver"]["status"]=="cap").unwrap();v["queries"].as_array_mut().unwrap().remove(i);},_=>panic!("unknown corruption")};write(&file,&v);pin["sha256"]=json!(sha(&fs::read(&file).unwrap()));write(&p,&index);
}
fn main(){let a:Vec<_>=std::env::args().collect();match a[1].as_str(){"audit"=>{assert_eq!(a.len(),5);audit(Path::new(&a[2]),Path::new(&a[3]),Path::new(&a[4]));},"corrupt"=>{assert_eq!(a.len(),4);corrupt(&a[2],Path::new(&a[3]));},_=>panic!("audit ROOT RESULTS RECEIPT | corrupt KIND RESULTS")}}
