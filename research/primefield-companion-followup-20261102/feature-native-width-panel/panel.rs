const ARMS:[&str;5]=["old","planned","separate","aa","positive"];
// Reference: repeated multiplication by (X-1), independent of interval/binomial code.
fn dense_oracle(r:u64,m:usize)->Vec<u64>{
 let mut a=vec![1u64];for _ in 0..(r-1)/2{let mut b=vec![0;a.len()+1];for (i,&x)in a.iter().enumerate(){b[i]=(b[i]+r-x)%r;b[i+1]=(b[i+1]+x)%r;}a=b;}
 let mut values=Vec::new();for e in 0..3{let v:Vec<_>=a.iter().skip(e).step_by(m).copied().collect();values.extend(v.into_iter().rev());}values
}
fn counted_stream(r:u64,m:usize,cap:usize,p:&RecoveryProducts,positive:bool)->(Vec<u64>,meter::Snapshot){
 meter::capture(||{let v=rescue_stream_values(r,m,cap,p);if positive{let _g=meter::Region::enter();std::hint::black_box(mul(1,1,r));}v}).unwrap()
}
fn panel_case(id:usize,r:u64,m:usize,cap:usize,emit:bool){
 let refs=[dense_oracle(r,m),dense_oracle(r,2*m)];
 let eta=(2..r).find(|&z|pow(z,(2*m)as u64,r)==1&&(1..2*m).all(|j|pow(z,j as u64,r)!=1)).unwrap();
 let stream=StreamModes::new(r,2*m,eta,2*m,r as usize,cap).unwrap();
 let mut seen:Vec<Option<[meter::Snapshot;2]>>=vec![None;5];
 for offset in 0..5{let arm=(offset+id)%5;let p=if arm==1{RecoveryProducts::planned_doubling(r,m,cap).unwrap()}else{RecoveryProducts::new(r,m,cap).unwrap()};
  let(v0,s0)=counted_stream(r,m,cap,&p,arm==4);assert_eq!(v0,refs[0]);
  let(v1,s1)=if arm==2{let q=RecoveryProducts::new(r,2*m,cap).unwrap();counted_stream(r,2*m,cap,&q,false)}else{let q=p.for_doubled(&stream).unwrap();counted_stream(r,2*m,cap,&q,arm==4)};assert_eq!(v1,refs[1]);
  for (phase,v,s)in[("initial",&v0,&s0),("doubled",&v1,&s1)]{assert!(!s.overflow&&s.counts[8]==0);if emit{println!("{{\"case\":{id},\"r\":{r},\"m\":{m},\"cap\":{cap},\"arm\":\"{}\",\"phase\":\"{phase}\",\"values\":{v:?},\"counts\":{:?},\"overflow\":false}}",ARMS[arm],s.counts);}}
  seen[arm]=Some([s0,s1]);
 }
 assert_eq!(seen[0],seen[3]);
 for phase in 0..2{let mut expected=seen[0].as_ref().unwrap()[phase].clone();expected.counts[0]+=1;assert_eq!(&expected,&seen[4].as_ref().unwrap()[phase]);}
}
fn main(){
 match std::env::args().nth(1).as_deref(){
 Some("--self-test")=>{meter_checks();panel_case(0,41,5,2,false);println!("width_panel_small_fixture=pass r=41 m=5 arms=5 phases=2 dense_oracle=pass controls=pass");},
 Some("--panel")=>{let mut id=0;for(r,m)in[(281,35),(1009,63),(2017,63),(2521,315)]{for cap in[1,2,7,16,64]{panel_case(id,r,m,cap,true);id+=1;}}},
 _=>{eprintln!("explicit --self-test or protocol-authorized --panel required");std::process::exit(2)}
 }
}
