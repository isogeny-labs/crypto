#![allow(dead_code)]
use std::io::Write;
#[path="../feature-native-resultant-accounting/meter.rs"]mod meter;
#[path="../feature-native-resultant-accounting/generated/scalar.rs"]mod scalar;
#[path="../feature-native-resultant-accounting/generated/euclid.rs"]mod euclid;
#[path="../feature-native-resultant-accounting/generated/half.rs"]mod half;
#[path="../feature-native-resultant-panel/panel.rs"]mod panel;
include!("journal.rs");
const ARMS:[&str;5]=["scalar","euclid","half","aa","positive"];
fn evaluate(arm:&str,c:&panel::Case)->Result<(u64,meter::Snapshot),&'static str>{
 let(p,roots,points)=(c.p,&c.roots[..],&c.points[..]);
 match arm{
  "scalar"|"aa"|"positive"=>meter::measure(||{let mut poly=scalar::Poly::new(p).map_err(|_|"domain")?;let raw=poly.root_resultant(roots,points).map_err(|_|"resultant")?;let mut v=meter::project(raw,p);if arm=="positive"{meter::hit(0);v=((v as u128*std::hint::black_box(1u128))%p as u128)as u64;}Ok(v)}),
  "euclid"=>meter::measure(||{let mut poly=euclid::Poly::new(p).map_err(|_|"domain")?;poly.root_projected_resultant(roots,points).map_err(|_|"resultant")}),
  "half"=>meter::measure(||{let mut poly=half::Poly::new(p).map_err(|_|"domain")?;poly.root_projected_resultant(roots,points).map_err(|_|"resultant")}),
  _=>Err("unknown arm")
 }
}
fn self_test(){
 panel::self_test();meter::self_test();journal_self_test();
 for points in [vec![3,4],vec![1,4]]{
  let c=panel::Case{id:0,p:13,kind:"software_fixture",roots:vec![1,2],points};
  let(v,s)=evaluate("scalar",&c).unwrap();assert_eq!(v,panel::oracle(&c));assert_eq!((v,s.clone()),evaluate("aa",&c).unwrap());
  let(vp,mut sp)=evaluate("positive",&c).unwrap();assert_eq!(v,vp);assert_eq!(sp.ops[0],s.ops[0]+1);sp.ops[0]-=1;assert_eq!(sp,s);
  for arm in ["euclid","half"]{assert_eq!(evaluate(arm,&c).unwrap().0,v);}
 }
 println!("RESULTANT_PANEL_SOFTWARE_TESTS_PASS");
}
fn is_digest(x:&str)->bool{x.len()==64&&x.bytes().all(|b|b.is_ascii_digit()||(b'a'..=b'f').contains(&b))}
fn run_panel(inputs:&str,protocol:&str,input_hash:&str,output:&str){
 assert!(is_digest(protocol)&&is_digest(input_hash));
 let cases=panel::cases();assert_eq!(std::fs::read_to_string(inputs).unwrap(),panel::input_text(&cases),"input file differs from deterministic panel");
 // The external admitted launcher binds actual file/binary hashes; hashes here identify rows only.
 let mut out=std::fs::OpenOptions::new().write(true).create_new(true).open(output).unwrap();
 let mut events=std::fs::OpenOptions::new().write(true).create_new(true).open(format!("{}.events",output)).unwrap();
 for c in cases{let expected=panel::oracle(&c);for offset in 0..5{
  let arm=ARMS[(offset+c.id)%5];journal_event(&mut events,c.id*5+offset,"start",protocol,input_hash);let(v,s)=evaluate(arm,&c).expect("valid accounting boundary");assert_eq!(v,expected,"oracle disagreement");
  write!(out,"{{\"case_id\":{},\"arm\":\"{}\",\"ordinal\":{},\"p\":{},\"kind\":\"{}\",\"n\":{},\"m\":{},\"value\":{},\"expected\":{},\"protocol_sha256\":\"{}\",\"input_sha256\":\"{}\",\"metrics\":{{",c.id,arm,c.id*5+offset,c.p,c.kind,c.roots.len(),c.points.len(),v,expected,protocol,input_hash).unwrap();
  for(i,(name,value))in meter::NAMES.iter().zip(s.ops).chain(meter::HEAP_NAMES.iter().zip(s.heap)).enumerate(){if i>0{write!(out,",").unwrap();}write!(out,"\"{}\":{}",name,value).unwrap();}
  writeln!(out,"}}}} ").unwrap();out.flush().unwrap();out.sync_all().unwrap();journal_event(&mut events,c.id*5+offset,"complete",protocol,input_hash);
 }}
 out.sync_all().unwrap();
}
fn main(){let args:Vec<_>=std::env::args().collect();match args.get(1).map(String::as_str){
 Some("--self-test")if args.len()==2=>self_test(),
 Some("--inputs")if args.len()==2=>print!("{}",panel::input_text(&panel::cases())),
 Some("--metrics")if args.len()==2=>{for n in meter::NAMES.iter().chain(&meter::HEAP_NAMES){println!("{}",n);}},
 Some("--panel")if args.len()==6=>run_panel(&args[2],&args[3],&args[4],&args[5]),
 _=>{eprintln!("--self-test | --inputs | --metrics | --panel INPUTS PROTOCOL_SHA INPUT_SHA NEW_OUTPUT (external frozen-protocol admission required)");std::process::exit(2);}
}}
