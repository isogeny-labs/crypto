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
fn pj(q:Point)->Value{point_json(q)}
fn pt(v:&Value)->Point{let a=ints(v);assert_eq!(a.len(),2,"false dual point shape");if a==[-1,-1]{None}else{assert!(a.iter().all(|&x|(0..13).contains(&x)),"false dual field coordinate");Some((a[0],a[1]))}}
fn phi(q:Point)->Point{q.and_then(|(x,y)|if x==10{None}else{let w=inv(x-10,13);Some((red(12*(x+2*w),13),red(8*y*(1-2*w*w),13)))})}
fn dual(q:Point)->Point{q.and_then(|(x,y)|if x==7{None}else{let v=red(12*x-6,13);let w=inv(v,13);Some((red(10*(v+5+8*w)+10,13),red(12*y*(1-8*w*w),13)))})}
fn load(p:&Path)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn write(p:&Path,v:&Value){fs::write(p,format!("{}\n",serde_json::to_string(v).unwrap())).unwrap()}
fn request(s:I,f:I,d:I,c:I,l:I)->Value{json!({"scalar":s,"phi":f,"dual":d,"comparisons":c,"cache_lookup":l})}
fn audit(root:&Path,out:&Path,receipt:&Path){
 assert!(!receipt.exists(),"fresh receipt");let bytes=fs::read(out.join("dual.json")).unwrap();let manifest=fs::read_to_string(out.join("dual.sha256")).unwrap();assert_eq!(manifest.split_whitespace().collect::<Vec<_>>(),vec![sha(&bytes),"dual.json".into()],"false dual payload hash");let v:Value=serde_json::from_slice(&bytes).unwrap();
 assert_eq!(v["schema"],"quadratic-twist-cm8-dual/v1");let e=Curve{p:13,a:1,b:4};let t=Curve{p:13,a:4,b:6};let p=Some((0,2));let u=phi(p);
 let ep=e.points();let tp=t.points();assert_eq!((ep.len(),tp.len()),(14,14));let mult=n(&v["multiplier"]);assert_eq!(red(2*mult,7),1,"false dual inverse multiplier");assert_eq!(mult,4);
 let prefix="research/quadratic_twist_evidence_20261021/";
 let expected:BTreeSet<_>=["native/cm8-dual.cpp","native/cm8-transfer-prior.hpp","native/cm8-transfer.cpp","native/cm8-dual-audit/main.rs","CM8_DUAL_20261029_PROTOCOL.md","lean-cm8/CM8Transfer.lean"].map(|x|format!("{prefix}{x}")).into();
 let source=v["sources"].as_array().unwrap();assert_eq!(source.len(),expected.len());assert_eq!(source.iter().map(|x|x["path"].as_str().unwrap().to_owned()).collect::<BTreeSet<_>>(),expected);
 for s in source{assert_eq!(s["sha256"],sha(&fs::read(root.join(s["path"].as_str().unwrap())).unwrap()),"false dual source binding");}
 let old=fs::read_to_string(root.join(format!("{prefix}native/cm8-transfer.cpp"))).unwrap();let adapter=fs::read_to_string(root.join(format!("{prefix}native/cm8-transfer-prior.hpp"))).unwrap();assert_eq!(adapter,old.replace("int main(","int cm8_reference_main(").replace("\n}catch", "\nreturn 0;\n}catch"),"false reference adapter");
 let em=v["source_points"].as_array().unwrap();let tm=v["target_points"].as_array().unwrap();assert_eq!((em.len(),tm.len()),(14,14),"false dual missing point");
 for(&q,row)in ep.iter().zip(em){assert_eq!(pt(&row["point"]),q);let f=phi(q);assert!(t.contains(f));assert_eq!(pt(&row["phi"]),f,"false dual forward");assert_eq!(dual(f),e.mul(2,q));assert_eq!(pt(&row["dual_phi"]),dual(f),"false dual composition");assert_eq!(pt(&row["double"]),e.mul(2,q));}
 let mut accepted=0;for(&r,row)in tp.iter().zip(tm){assert_eq!(pt(&row["point"]),r);let d=dual(r);assert!(e.contains(d));assert_eq!(pt(&row["dual"]),d,"false dual value/kernel");assert_eq!(phi(d),t.mul(2,r));assert_eq!(pt(&row["phi_dual"]),phi(d),"false dual composition");assert_eq!(pt(&row["double"]),t.mul(2,r));let ok=t.mul(7,r).is_none();assert_eq!(row["inverse_accepted"],ok,"false dual subgroup acceptance");if ok{accepted+=1;let a=e.mul(mult,d);assert_eq!(pt(&row["inverse"]),a);assert_eq!(phi(a),r);assert!(e.mul(7,a).is_none());}else{assert!(pt(&row["inverse"]).is_none());}}
 assert_eq!(accepted,7);assert_eq!(ep.iter().filter(|&&q|phi(q).is_none()).copied().collect::<Vec<_>>(),vec![None,Some((10,0))]);assert_eq!(tp.iter().filter(|&&q|dual(q).is_none()).copied().collect::<Vec<_>>(),vec![None,Some((7,0))]);
 for(curve,points,key,back)in[(e,&ep,"forward_addition",false),(t,&tp,"dual_addition",true)]{let rows=v[key].as_array().unwrap();assert_eq!(rows.len(),196);for(i,&a)in points.iter().enumerate(){for(j,&b)in points.iter().enumerate(){let row=&rows[i*14+j];let s=curve.add(a,b);let f=if back{dual}else{phi};let other=if back{e}else{t};assert_eq!(pt(&row["a"]),a);assert_eq!(pt(&row["b"]),b);assert_eq!(pt(&row["sum"]),s);assert_eq!(pt(&row["image_sum"]),f(s));assert_eq!(pt(&row["sum_images"]),other.add(f(a),f(b)));assert_eq!(f(s),other.add(f(a),f(b)));}}}
 let subgroup=v["subgroup"].as_array().unwrap();assert_eq!(subgroup.len(),7);for(i,row)in subgroup.iter().enumerate(){let a=e.mul(i as I,p);let r=t.mul(i as I,u);assert_eq!(n(&row["n"]),i as I);assert_eq!(pt(&row["source"]),a);assert_eq!(pt(&row["target"]),r);assert_eq!(pt(&row["inverse"]),e.mul(mult,dual(r)));assert_eq!(e.mul(mult,dual(r)),a);}
 let cache=v["cache"].as_array().unwrap();assert_eq!(cache.len(),7);for(i,row)in cache.iter().enumerate(){assert_eq!(n(&row["i"]),i as I);assert_eq!(pt(&row["source"]),e.mul(i as I,p));assert_eq!(pt(&row["image"]),phi(e.mul(i as I,p)));}assert_eq!(v["cache_setup_requests"],request(7,7,0,0,0));
 let xs=t.xs(u,7);let pairs=[(0,1),(0,2),(1,2)];let pulls=v["pullbacks"].as_array().unwrap();assert_eq!(pulls.len(),6);for(policy,(i,j))in pairs.into_iter().enumerate(){let base=t.reps(&[xs[i],xs[j]]);for(col,&r)in base.iter().enumerate(){let row=&pulls[policy*2+col];assert_eq!(n(&row["policy"]),policy as I);assert_eq!(n(&row["column"]),col as I);assert_eq!(pt(&row["target"]),r);let hit=(0..7).find(|&z|phi(e.mul(z,p))==r).unwrap();let trials=row["trials"].as_array().unwrap();assert_eq!(trials.len(),(hit+1)as usize);for(z,tr)in trials.iter().enumerate(){let a=e.mul(z as I,p);assert_eq!(n(&tr["i"]),z as I);assert_eq!(pt(&tr["source"]),a);assert_eq!(pt(&tr["image"]),phi(a));assert_eq!(tr["match"],z as I==hit);}
 let a=e.mul(hit,p);let d=dual(r);let cand=e.mul(mult,d);for key in["reference","cached","candidate"]{assert_eq!(pt(&row[key]),a,"false dual pullback");}assert_eq!(pt(&row["dual"]),d);assert_eq!(cand,a);assert_eq!(phi(cand),r);
 assert_eq!(row["reference_requests"],request(hit+1,hit+1,0,hit+1,0));assert_eq!(row["cached_requests"],request(0,0,0,hit+1,1));assert_eq!(row["candidate_requests"],request(1,0,1,0,0));assert_eq!(row["boundary_requests_per_route"],json!({"scalar":2,"phi":1,"curve_checks":2,"comparisons":3}));}}
 let arms=v["arms"].as_array().unwrap();assert_eq!(arms.len(),12);let names:BTreeSet<_>=["main","twist","matched","pullback"].into_iter().flat_map(|x|(0..3).map(move|i|format!("{x}{i}"))).collect();assert_eq!(arms.iter().map(|a|a["name"].as_str().unwrap().to_owned()).collect::<BTreeSet<_>>(),names);
 let mut recoveries=0;for a in arms{let name=a["name"].as_str().unwrap();let cb=fs::read(out.join("candidate").join(format!("{name}.json"))).unwrap();assert_eq!(a["sha256"],sha(&cb));let arm:Value=serde_json::from_slice(&cb).unwrap();let mapped=name.starts_with("twist")||name.starts_with("matched");let curve=if mapped{t}else{e};let g=if mapped{u}else{p};assert_eq!(pt(&arm["generator"]),g);let bs=arm["base_points"].as_array().unwrap();assert_eq!(bs.len(),2);for(i,b)in bs.iter().enumerate(){assert_eq!(curve.mul(n(&arm["base_logs"][i]),g),pt(b));}
 let records=arm["recoveries"].as_array().unwrap();assert_eq!(records.len(),6);for(i,q)in records.iter().enumerate(){let source=pt(&q["source"]);assert_eq!(source,e.mul(i as I+1,p));let answer=n(&q["answer"]);assert!((0..7).contains(&answer));assert_eq!(e.mul(answer,p),source,"false dual source scalar");assert_eq!(pt(&q["mapped"]),if mapped{phi(source)}else{source});recoveries+=1;}
 assert_eq!(cb,fs::read(out.join("reference").join(format!("{name}.json"))).unwrap(),"false dual complete arm parity");}
 assert_eq!(recoveries,72);assert_eq!(n(&v["source_recoveries"]),recoveries,"false dual declared count");
 let expected_files:BTreeSet<_>=names.iter().map(|x|format!("{x}.json")).chain(std::iter::once("index.json".into())).collect();for d in["candidate","reference"]{let files=fs::read_dir(out.join(d)).unwrap().map(|x|x.unwrap().file_name().to_str().unwrap().to_owned()).collect::<BTreeSet<_>>();assert_eq!(files,expected_files,"false dual case membership");}assert_eq!(fs::read(out.join("candidate/index.json")).unwrap(),fs::read(out.join("reference/index.json")).unwrap());
 write(receipt,&json!({"status":"passed","independent_arithmetic":true,"source_points":14,"target_points":14,"addition_pairs":392,"subgroup_inverse_points":7,"rejected_valid_non_r7_points":7,"pullbacks":6,"arms":12,"source_recoveries":72,"dual_sha256":sha(&bytes)}));
}
fn corrupt(kind:&str,out:&Path){let path=out.join("dual.json");let mut v=load(&path);let old=v.clone();match kind{
 "false-dual"=>v["target_points"][1]["dual"]=pj(Some((0,2))),
 "false-multiplier"=>v["multiplier"]=json!(3),
 "false-composition"=>v["target_points"][1]["phi_dual"]=v["target_points"][1]["point"].clone(),
 "false-kernel"=>{let row=v["target_points"].as_array_mut().unwrap().iter_mut().find(|r|r["point"]==json!([7,0])).unwrap();row["dual"]=pj(Some((0,2)));},
 "missing-point"=>{v["target_points"].as_array_mut().unwrap().pop().unwrap();},
 "false-count"=>v["source_recoveries"]=json!(71),
 "false-subgroup"=>{let row=v["target_points"].as_array_mut().unwrap().iter_mut().find(|r|r["point"]==json!([7,0])).unwrap();row["inverse_accepted"]=json!(true);},
 "false-source"=>v["sources"][0]["sha256"]=json!("0".repeat(64)),
 "false-scalar"=>{let name=v["arms"][0]["name"].as_str().unwrap().to_owned();let p=out.join("candidate").join(format!("{name}.json"));let mut a=load(&p);a["recoveries"][0]["answer"]=json!((n(&a["recoveries"][0]["answer"])+1)%7);write(&p,&a);let h=sha(&fs::read(&p).unwrap());v["arms"][0]["sha256"]=json!(h.clone());let ip=out.join("candidate/index.json");let mut ix=load(&ip);for a in ix["arms"].as_array_mut().unwrap(){if a["name"]==name{a["sha256"]=json!(h.clone());}}write(&ip,&ix);},
 _=>panic!("unknown control")};assert!(old!=v,"control must change payload");write(&path,&v);fs::write(out.join("dual.sha256"),format!("{}  dual.json\n",sha(&fs::read(&path).unwrap()))).unwrap();println!("corrupted-and-rehashed");}
fn main(){let a:Vec<_>=std::env::args().collect();match a.get(1).map(String::as_str){Some("audit")=>audit(Path::new(&a[2]),Path::new(&a[3]),Path::new(&a[4])),Some("corrupt")=>corrupt(&a[2],Path::new(&a[3])),_=>panic!("audit ROOT OUT RECEIPT | corrupt KIND OUT")}}
