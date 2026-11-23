// Geometry instrument only. Do not execute a study before the official checkpoint.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, fs, io::Write, path::Path};

const PRODUCER: &str = "research/quadratic_twist_chord_proof_20261105/native-preflight-20261113/producer/src/main.rs";
const AUDITOR: &str = "research/quadratic_twist_chord_proof_20261105/native-preflight-20261113/auditor.cpp";
const CONTRACT: &str = "research/quadratic_twist_chord_proof_20261105/native-preflight-20261113/CONTRACT.json";
fn f(x: i64) -> i64 { x.rem_euclid(37) }
fn power(mut x: i64, mut n: u32) -> i64 {
    let mut y=1; while n>0 { if n&1==1 {y=f(y*x)} x=f(x*x); n>>=1; } y
}
#[derive(Clone,Copy,Debug,PartialEq,Eq,PartialOrd,Ord)]
struct K(i64,i64);
impl K {
    fn b(x:i64)->Self {Self(f(x),0)}
    fn add(self,z:Self)->Self {Self(f(self.0+z.0),f(self.1+z.1))}
    fn neg(self)->Self {Self(f(-self.0),f(-self.1))}
    fn sub(self,z:Self)->Self {self.add(z.neg())}
    fn mul(self,z:Self)->Self {Self(f(self.0*z.0+35*self.1*z.1),f(self.0*z.1+self.1*z.0))}
    fn inv(self)->Self {let n=f(self.0*self.0-35*self.1*self.1); assert_ne!(n,0,"zero inverse"); let a=power(n,35);Self(f(self.0*a),f(-self.1*a))}
    fn div(self,z:Self)->Self {self.mul(z.inv())}
    fn sq(self)->Self {self.mul(self)}
    fn cube(self)->Self {self.sq().mul(self)}
    fn conj(self)->Self {Self(self.0,f(-self.1))}
}
#[derive(Clone,Copy,Debug,PartialEq,Eq,PartialOrd,Ord)]
struct Pt {x:K,y:K}
type Point=Option<Pt>;
fn neg(p:Point)->Point {p.map(|p|Pt{x:p.x,y:p.y.neg()})}
fn conj(p:Point)->Point {p.map(|p|Pt{x:p.x.conj(),y:p.y.conj()})}
fn on(p:Point,a:i64,b:i64)->bool {p.map_or(true,|p|p.y.sq()==p.x.cube().add(K::b(a).mul(p.x)).add(K::b(b)))}
fn plus(p:Point,q:Point)->Point {
    let (p,q)=match(p,q){(None,q)=>return q,(p,None)=>return p,(Some(p),Some(q))=>(p,q)};
    if p.x==q.x && p.y.add(q.y)==K::b(0) {return None}
    let l=if p.x==q.x {assert_eq!(p.y,q.y);p.x.sq().mul(K::b(3)).add(K::b(7)).div(p.y.mul(K::b(2)))}
      else {q.y.sub(p.y).div(q.x.sub(p.x))};
    let x=l.sq().sub(p.x).sub(q.x);let y=l.mul(p.x.sub(x)).sub(p.y);Some(Pt{x,y})
}
fn times(mut p:Point,mut n:u32)->Point {let mut a=None;while n>0 {if n&1==1 {a=plus(a,p)} p=plus(p,p);n>>=1;}a}
fn pj(p:Point)->Value {p.map_or(Value::Null,|p|json!([p.x.0,p.x.1,p.y.0,p.y.1]))}
fn qj(p:Point)->Value {p.map_or(Value::Null,|p|{assert_eq!(p.x.1,0);assert_eq!(p.y.1,0);json!([p.x.0,p.y.0])})}
fn inverse_twist(c:Point)->Value {
    c.map_or(Value::Null,|p|{let x=p.x.mul(K::b(35));let y=p.y.mul(K(0,35));assert_eq!(x.1,0);assert_eq!(y.1,0);
      let t=Some(Pt{x,y});assert!(on(t,28,33));json!([x.0,y.0])})
}
fn record(q:Point,r:Point,kind:&str,parameters:Value)->Value {
    assert!(on(r,7,19));assert_eq!(plus(r,conj(r)),q);
    let c=plus(r,neg(times(q,10)));assert_eq!(conj(c),neg(c));
    json!({"branch":kind,"parameters":parameters,"point":pj(r),"twist":inverse_twist(c)})
}
fn fiber(q:Point)->Value {
    let mut outcomes=Vec::with_capacity(1369);let mut rows=Vec::new();
    if let Some(qp)=q {
      let a=qp.x.0;let b=qp.y.0;
      for l in 0..37 {for t in 0..37 {
        let disc=f(l*l*l*l-6*a*l*l-8*b*l-3*a*a-4*7);
        let ok=f(35*t*t)==disc;outcomes.push(ok);
        if ok {let s=f(l*l-a);let x=K(s,t).div(K::b(2));let y=K::b(l).mul(x.sub(K::b(a))).sub(K::b(b));
          rows.push(record(q,Some(Pt{x,y}),if t==0 {"tangent"} else {"secant"},json!([l,t])));
        }
      }}
    } else {
      rows.push(record(None,None,"infinity",Value::Null));
      for x in 0..37 {for y in 0..37 {
        let ok=f(y*y)==f(x*x*x+28*x+33);outcomes.push(ok);
        if ok {let r=Some(Pt{x:K::b(x).div(K::b(35)),y:K::b(y).div(K(0,35))});rows.push(record(None,r,"identity",json!([x,y])));}
      }}
    }
    let mut seen=BTreeSet::new();for r in &rows {assert!(seen.insert(r["point"].to_string()),"duplicate geometric witness")}
    json!({"query":qj(q),"pair_outcomes":outcomes,"records":rows})
}
fn fixture()->Value {json!({"p":37,"a":7,"b":19,"d":35,"twist_a":28,"twist_b":33,"r":19,"generator":[1,8],"half_scalar":10})}
fn targets(scope:&str)->Vec<Point> {
    let p=Some(Pt{x:K::b(1),y:K::b(8)});assert!(on(p,7,19));assert_eq!(times(p,19),None);assert_ne!(p,None);
    let mut nonzero=Vec::new();let mut q=p;
    for _ in 1..19 {assert!(q.is_some());nonzero.push(q);q=plus(q,p)}
    assert_eq!(q,None);nonzero.sort();assert!(nonzero.windows(2).all(|w|w[0]!=w[1]));
    match scope {"fixed"=>vec![p],"full"=>{let mut all=vec![None];all.extend(nonzero);all},_=>panic!("unknown scope")}
}
fn digest(b:&[u8])->String {format!("{:x}",Sha256::digest(b))}
fn manifest(repo:&Path,path:&Path,expected:&str)->Value {
    let b=fs::read(path).expect("manifest read");assert_eq!(digest(&b),expected,"manifest pin");
    let v:Value=serde_json::from_slice(&b).expect("manifest JSON");assert_eq!(v["schema"],"twist.trace-fiber-inputs/v1");
    for (k,p) in [("producer",PRODUCER),("auditor",AUDITOR),("contract",CONTRACT)] {
      assert_eq!(v[k]["path"],p);assert_eq!(v[k]["sha256"],digest(&fs::read(repo.join(p)).expect("source read")),"source pin");
    } v
}
fn write_new(path:&Path,v:&Value) {
    let side=path.with_extension("sha256");assert!(!path.exists()&&!side.exists(),"fresh output required");
    let mut b=serde_json::to_vec_pretty(v).expect("serialize");b.push(b'\n');
    fs::OpenOptions::new().write(true).create_new(true).open(path).expect("create output").write_all(&b).expect("write output");
    fs::OpenOptions::new().write(true).create_new(true).open(side).expect("create digest").write_all((digest(&b)+"\n").as_bytes()).expect("write digest");
}
fn mutate(input:&Path,output:&Path,mode:&str) {
    assert_ne!(input,output);let mut v:Value=serde_json::from_slice(&fs::read(input).expect("read input")).expect("JSON");let old=v.clone();
    match mode {
      "wrong-d"=>v["fixture"]["d"]=json!(1),
      "wrong-target"=>{let c=v["queries"].as_array_mut().unwrap().iter_mut().find(|q|!q["query"].is_null()).expect("nonidentity");c["query"][0]=json!((c["query"][0].as_i64().unwrap()+1)%37);},
      "missing-tangent"=>{let c=v["queries"].as_array_mut().unwrap().iter_mut().find(|q|q["records"].as_array().unwrap().iter().any(|r|r["branch"]=="tangent")).expect("control inapplicable: no tangent");let rows=c["records"].as_array_mut().unwrap();let i=rows.iter().position(|r|r["branch"]=="tangent").unwrap();rows.remove(i);},
      "duplicate-record"=>{let rows=v["queries"][0]["records"].as_array_mut().unwrap();let r=rows[0].clone();rows.push(r);},
      "wrong-inverse"=>{let c=v["queries"].as_array_mut().unwrap().iter_mut().find(|q|q["records"].as_array().unwrap().iter().any(|r|!r["twist"].is_null())).unwrap();let r=c["records"].as_array_mut().unwrap().iter_mut().find(|r|!r["twist"].is_null()).unwrap();r["twist"][0]=json!((r["twist"][0].as_i64().unwrap()+1)%37);},
      "missing-attempt"=>{v["queries"][0]["pair_outcomes"].as_array_mut().unwrap().pop();},
      "false-root"=>{let bits=v["queries"][0]["pair_outcomes"].as_array_mut().unwrap();let i=bits.iter().position(|b|b==false).expect("inapplicable: no failed candidate");bits[i]=json!(true);},
      "missing-case"=>{v["queries"].as_array_mut().unwrap().pop();},
      "source-pin"=>v["sources"]["producer"]["sha256"]=json!("0".repeat(64)),
      "spurious-infinity"=>{let c=v["queries"].as_array_mut().unwrap().iter_mut().find(|q|!q["query"].is_null()).unwrap();c["records"].as_array_mut().unwrap().push(json!({"branch":"infinity","parameters":null,"point":null,"twist":null}));},
      _=>panic!("unknown mutation")
    }
    assert_ne!(v,old,"mutation must change data");write_new(output,&v)
}
fn main(){
 let a:Vec<String>=std::env::args().collect();
 if a.len()==2 && a[1]=="--help" {println!("produce REPO MANIFEST MANIFEST_SHA256 fixed|full NEW_OUTPUT.json\nmutate INPUT.json NEW_OUTPUT.json MODE\nNo execution without the approved official checkpoint.");return}
 if a.len()==6 && a[1]=="mutate" {eprintln!("usage: mutate INPUT NEW_OUTPUT MODE");std::process::exit(2)}
 if a.len()==5 && a[1]=="mutate" {mutate(Path::new(&a[2]),Path::new(&a[3]),&a[4]);return}
 if a.len()!=7 || a[1]!="produce" {eprintln!("use --help");std::process::exit(2)}
 let m=manifest(Path::new(&a[2]),Path::new(&a[3]),&a[4]);
 let cases=targets(&a[5]).into_iter().map(fiber).collect::<Vec<_>>();
 write_new(Path::new(&a[6]),&json!({"schema":"twist.trace-fiber-geometry/v1","fixture":fixture(),"scope":a[5],"sources":m,"queries":cases}));
}
