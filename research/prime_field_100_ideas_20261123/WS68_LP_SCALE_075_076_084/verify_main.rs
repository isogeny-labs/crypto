// Independent candidate, source, closure, rank, target and rho replay.
include!("verify.rs");
fn modpow(mut a:i64,mut k:i64)->i64{let mut z=1;while k>0{if k&1!=0{z=red(z*a,P)}a=red(a*a,P);k>>=1}z}
fn curve_count()->i64{let mut n=1;for x in 0..P{let f=red(x*x*x+x+27,P);if f==0{n+=1}else if modpow(f,(P-1)/2)==1{n+=2}}n}
fn verify_case(case:usize){
 assert!(case<12);assert_eq!(curve_count(),R);assert!((2..64).all(|d|R%d!=0));
 let target_index=case/3;let replicate=case%3;let seed=SEEDS[replicate];
 let targets=[Pt::A(3280,1514),Pt::A(1419,683),Pt::A(3957,905),Pt::A(1671,799)];
 let q=targets[target_index];let b=base();let g=b[0];assert_eq!(g,Pt::A(1,921));assert_eq!(mul(R,g),Pt::O);
 let path=format!("WS68_LP_SCALE_075_076_084/case{case}.stdout");let (full,all,summaries)=read(&path);
 let raw=fs::read_to_string(&path).unwrap();let rho=raw.lines().find(|l|l.starts_with("rho ")).unwrap();
 if rho.contains("target_pinned=true"){let d:i64=rho.split_whitespace().find_map(|s|s.strip_prefix("scalar=")).unwrap().parse().unwrap();assert_eq!(mul(d,g),q)}
 let mut stream=Vec::new();let mut state=seed;let mut transcript=String::new();
 for trial in 0..768{let a=(next(&mut state)%R as u64)as i64;let bb=1+(next(&mut state)%(R-1)as u64)as i64;let r=add(mul(a,g),mul(bb,q));stream.push((a,bb,r));transcript.push_str(&format!("trial={trial} a={a} b={bb} r={r:?}\n"))}
 let candidates=fs::read_to_string(format!("WS68_LP_SCALE_075_076_084/candidate{case}.stdout")).unwrap();assert_eq!(candidates,transcript);
 for (arm,src) in [&full,&all].iter().enumerate(){
  let table=entries(&b,arm);let mut expected=Vec::new();for (trial,&(a,bb,r)) in stream.iter().enumerate(){if let Some(&(kind,i,j))=table.get(&r){expected.push((trial,a,bb,r,kind,i,j))}}
  assert_eq!(src.len(),expected.len());for (id,s) in src.iter().enumerate(){let &(trial,a,bb,r,kind,i,j)=&expected[id];assert_eq!((s.trial,s.a,s.b,s.r,s.kind,s.i,s.j),(trial,a,bb,r,kind,i,j));assert_eq!(add(b[i],b[j]),r);assert_eq!(add(mul(a,g),mul(bb,q)),r)}
  let mut complete=filter(src,&b,q,g);assert_eq!(complete.len(),summaries[arm].0);let mut anchor=vec![0;B+1];anchor[0]=1;anchor[B]=1;complete.insert(0,anchor);
  let (rb,_)=rank_answer(&complete,false);let (ra,ans)=rank_answer(&complete,true);assert_eq!((rb,ra),(summaries[arm].1,summaries[arm].2));assert_eq!(ans.unwrap_or(-1),summaries[arm].3);if let Some(d)=ans{assert_eq!(mul(d,g),q)}
  println!("VERIFY case={case} arm={arm} source={} complete={} base_rank={rb} augmented_rank={ra} target={} candidate_and_source_exact=true completed_point_checks=true",src.len(),complete.len()-1,ans.unwrap_or(-1));
 }
 assert_eq!(full.len(),all.iter().filter(|s|s.kind==0).count());assert!(full.iter().zip(all.iter().filter(|s|s.kind==0)).all(|(x,y)|x.trial==y.trial&&x.row==y.row));
 println!("VERIFY_OK=1 case={case} curve_order={R} target={target_index} replicate={replicate}");
}
fn main(){let case:usize=std::env::args().nth(1).unwrap().parse().unwrap();verify_case(case)}
