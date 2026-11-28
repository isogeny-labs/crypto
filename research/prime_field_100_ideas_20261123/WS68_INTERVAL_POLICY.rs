// #97 partial control: time-ordered scalar intervals on E(F_1009).
// This is NOT a Semaev or integer smoothness sieve interval. The same 512
// Q+[a]G candidates and pair-sum EC relation oracle feed all policies.
// Run stops only after a full interval, with censored arms retained.
use std::collections::HashMap;
const P:i64=1009;const N:i64=1013;const CAP:usize=512;
const TARGETS:[i64;8]=[123,277,411,509,701,827,941,1003];
#[derive(Clone,Copy,Eq,PartialEq,Hash,Debug)]enum Pt{O,A(i64,i64)}
fn m(x:i64,n:i64)->i64{x.rem_euclid(n)}
fn inv(x:i64,n:i64)->i64{let(mut a,mut b,mut u,mut v)=(m(x,n),n,1,0);while b!=0{let q=a/b;(a,b)=(b,a-q*b);(u,v)=(v,u-q*v)}assert_eq!(a,1);m(u,n)}
fn neg(q:Pt)->Pt{match q{Pt::O=>q,Pt::A(x,y)=>Pt::A(x,m(-y,P))}}
fn add(q:Pt,t:Pt)->Pt{match(q,t){(Pt::O,v)|(v,Pt::O)=>v,(Pt::A(x,y),Pt::A(u,v))=>{if x==u&&m(y+v,P)==0{return Pt::O}let s=if x==u{m((3*x*x+1)*inv(2*y,P),P)}else{m((v-y)*inv(u-x,P),P)};let xx=m(s*s-x-u,P);Pt::A(xx,m(s*(x-xx)-y,P))}}}
fn mul(mut k:i64,mut q:Pt)->Pt{if k<0{k=-k;q=neg(q)}let mut out=Pt::O;while k>0{if k&1!=0{out=add(out,q)}q=add(q,q);k>>=1}out}
fn valid(q:Pt)->bool{match q{Pt::O=>true,Pt::A(x,y)=>m(y*y-x*x*x-x-14,P)==0}}
fn base()->(Vec<Pt>,u64,i64){let mut out=Vec::new();let mut y_tests=0;let mut last_x=0;for x in 0..P{for y in 0..P{y_tests+=1;let q=Pt::A(x,y);if valid(q){out.push(q);last_x=x;break}}if out.len()==16{break}}assert_eq!(out.len(),16);(out,y_tests,last_x+1)}
fn table(base:&[Pt])->HashMap<Pt,(usize,usize)>{let mut t=HashMap::new();for i in 0..16{for j in i..16{t.entry(add(base[i],base[j])).or_insert((i,j));}}t}
fn reduce(mut rows:Vec<Vec<i64>>,cols:usize)->(usize,Vec<Vec<i64>>){let mut h=0;for c in 0..cols{if let Some(k)=(h..rows.len()).find(|&k|m(rows[k][c],N)!=0){rows.swap(h,k);let z=inv(rows[h][c],N);for v in &mut rows[h]{*v=m(*v*z,N)}let piv=rows[h].clone();for i in 0..rows.len(){if i!=h{let z=rows[i][c];for j in c..rows[i].len(){rows[i][j]=m(rows[i][j]-z*piv[j],N)}}}h+=1}}(h,rows)}
fn stats(rows:&[Vec<i64>])->(usize,usize,Option<i64>){let rb=reduce(rows.iter().map(|r|r[..16].to_vec()).collect(),16).0;let(ra,rref)=reduce(rows.to_vec(),17);let mut ans=None;for row in rref{if row[..16].iter().all(|&x|x==0)&&row[16]!=0{ans=Some(m(row[17]*inv(row[16],N),N));break}}(rb,ra,ans)}
fn stream(q:Pt,g:Pt,d:i64)->Vec<Pt>{let mut r=q;let mut out=Vec::with_capacity(CAP);for a in 0..CAP{assert_eq!(r,mul(d+a as i64,g));assert!(valid(r));out.push(r);r=add(r,g)}out}
fn fmt(q:Pt)->String{match q{Pt::O=>"O".into(),Pt::A(x,y)=>format!("{x},{y}")}}
fn run(label:&str,d:i64,g:Pt,q:Pt,base:&[Pt],t:&HashMap<Pt,(usize,usize)>,s:&[Pt]){
 let mut rows:Vec<Vec<i64>>=Vec::new();let mut anchor=vec![0;18];anchor[0]=1;anchor[17]=1;rows.push(anchor);
 let mut cursor=0;let mut width=if label=="fixed32"{32}else{8};let mut integral=0i64;let mut previous=0i64;let mut batches=0;let mut hits=0;let mut width_changes=0;let mut answer=None;
 while cursor<CAP{
  let end=(cursor+width).min(CAP);let mut batch_hits=0;
  for a in cursor..end{let r=s[a];if let Some(&(i,j))=t.get(&r){assert_eq!(add(base[i],base[j]),r);assert_eq!(r,add(q,mul(a as i64,g)));let mut row=vec![0;18];row[i]+=1;row[j]+=1;row[16]=-1;row[17]=a as i64;rows.push(row);hits+=1;batch_hits+=1}}
  let used=end-cursor;cursor=end;batches+=1;let(_,_,found)=stats(&rows);if let Some(x)=found{assert_eq!(x,d);assert_eq!(mul(x,g),q);answer=Some(x);break}
  if label=="pid"{let error=128-(1024*batch_hits as i64/used as i64);integral=(integral+error).clamp(-2048,2048);let derivative=error-previous;previous=error;let delta=(8*error+integral+4*derivative)/1024;let next=(width as i64+delta).clamp(4,64) as usize;if next!=width{width_changes+=1}width=next}
 }
 let(rb,ra,check)=stats(&rows);assert_eq!(answer,check);let setup_adds=136u64;let candidate_adds=cursor.saturating_sub(1) as u64;let lookup_count=cursor;let group_add_total=setup_adds+candidate_adds;
 println!("arm target={d} policy={label} attempts={cursor} batches={batches} hits={hits} rank_base={rb} rank_aug={ra} pinned={} recovered={} final_width={width} width_changes={width_changes} setup_pair_adds={setup_adds} candidate_group_adds={candidate_adds} total_group_adds={group_add_total} relation_lookups={lookup_count} rank_checks={batches} dispatches={batches} controller_updates={}",answer.is_some(),answer.unwrap_or(-1),if label=="pid"{batches as i64-if answer.is_some(){1}else{0}}else{0});
}
fn main(){let g=Pt::A(0,425);assert!(valid(g));assert_eq!(mul(N,g),Pt::O);let count=1+(0..P).flat_map(|x|(0..P).map(move|y|Pt::A(x,y))).filter(|&q|valid(q)).count();assert_eq!(count,N as usize);let(base,y_tests,x_scans)=base();assert_eq!(base[0],g);let t=table(&base);
 println!("curve_p={P} group_order={N} group_count={count} cap={CAP} target_count={} pair_table_entries={} base_y_tests={y_tests} base_x_scans={x_scans} policies=fixed8,fixed32,pid pid_target_rate_1024=128 pid_kp=8 pid_ki=1 pid_kd=4 pid_width_bounds=4,64",TARGETS.len(),t.len());
 for &d in &TARGETS{let q=mul(d,g);let s=stream(q,g,d);for(a,&r)in s.iter().enumerate(){println!("candidate target={d} a={a} point={}",fmt(r))}for label in ["fixed8","fixed32","pid"]{run(label,d,g,q,&base,&t,&s)}}
 println!("claim_boundary=contiguous_scalar_interval_schedule_only not_a_sieve_interval=true modular_elimination_ops_unpriced=true wall_time_unpriced=true");
}
