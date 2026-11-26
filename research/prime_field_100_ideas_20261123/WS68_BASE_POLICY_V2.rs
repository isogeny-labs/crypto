// Anchored correction of WS68_BASE_POLICY.rs for #96 and #97.
// Adds the public known-log row log_G(G)=1; all target streams/policies match V1.
// E/F_1009: y^2=x^3+x+14; G=(0,425), order 1013.
// First 8/16 x-ordered canonical points, target scalars fixed below.
// Same 512 ordered R=[a]G+Q candidates replayed by all three policies.
use std::collections::HashMap;
const P:i64=1009;const N:i64=1013;const CAP:usize=512;const EXPAND_AT:usize=128;
const TARGETS:[i64;8]=[123,277,411,509,701,827,941,1003];
#[derive(Clone,Copy,Eq,PartialEq,Hash,Debug)]enum Pt{O,A(i64,i64)}
fn red(x:i64,m:i64)->i64{x.rem_euclid(m)}
fn inv(x:i64,m:i64)->i64{let(mut a,mut b,mut u,mut v)=(red(x,m),m,1,0);while b!=0{let q=a/b;(a,b)=(b,a-q*b);(u,v)=(v,u-q*v);}assert_eq!(a,1);red(u,m)}
fn neg(q:Pt)->Pt{match q{Pt::O=>q,Pt::A(x,y)=>Pt::A(x,red(-y,P))}}
fn add(q:Pt,t:Pt)->Pt{match(q,t){(Pt::O,v)|(v,Pt::O)=>v,(Pt::A(x,y),Pt::A(u,v))=>{if x==u&&red(y+v,P)==0{return Pt::O;}let s=if x==u{red((3*x*x+1)*inv(2*y,P),P)}else{red((v-y)*inv(u-x,P),P)};let xx=red(s*s-x-u,P);Pt::A(xx,red(s*(x-xx)-y,P))}}}
fn mul(mut k:i64,mut q:Pt)->Pt{if k<0{k=-k;q=neg(q);}let mut out=Pt::O;while k>0{if k&1==1{out=add(out,q);}q=add(q,q);k>>=1;}out}
fn mul_cost(k:i64)->u64{if k==0{0}else{(64-(k as u64).leading_zeros() as u64)+(k as u64).count_ones() as u64}}
fn base()->(Vec<Pt>,Vec<u64>,Vec<i64>){let mut out=Vec::new();let mut y_tests=Vec::new();let mut x_scans=Vec::new();let mut tested=0;
 for x in 0..P{for y in 0..P{tested+=1;if red(y*y-x*x*x-x-14,P)==0{out.push(Pt::A(x,y));y_tests.push(tested);x_scans.push(x+1);break;}}if out.len()==16{break;}}
 assert_eq!(out.len(),16);(out,y_tests,x_scans)}
fn table(base:&[Pt],b:usize)->HashMap<Pt,(usize,usize)>{let mut t=HashMap::new();for i in 0..b{for j in i..b{t.entry(add(base[i],base[j])).or_insert((i,j));}}t}
fn reduce(mut rows:Vec<Vec<i64>>,cols:usize)->(usize,Vec<Vec<i64>>){let mut h=0;for c in 0..cols{if let Some(k)=(h..rows.len()).find(|&k|red(rows[k][c],N)!=0){rows.swap(h,k);let z=inv(rows[h][c],N);for v in &mut rows[h]{*v=red(*v*z,N);}let piv=rows[h].clone();for i in 0..rows.len(){if i!=h{let z=rows[i][c];for j in c..rows[i].len(){rows[i][j]=red(rows[i][j]-z*piv[j],N);}}}h+=1;}}(h,rows)}
fn stats(rows:&[Vec<i64>])->(usize,usize,Option<i64>){if rows.is_empty(){return(0,0,None);}let rb=reduce(rows.iter().map(|r|r[..16].to_vec()).collect(),16).0;let (ra,rref)=reduce(rows.to_vec(),17);let mut answer=None;
 for row in rref{if row[..16].iter().all(|&x|x==0)&&row[16]!=0{answer=Some(red(row[17]*inv(row[16],N),N));break;}}
 (rb,ra,answer)}
fn stream(g:Pt,q:Pt,seed:u64)->Vec<(i64,Pt,u64)>{let mut state=seed;let mut out=Vec::new();for _ in 0..CAP{state^=state<<13;state^=state>>7;state^=state<<17;let a=(state%N as u64)as i64;out.push((a,add(mul(a,g),q),mul_cost(a)+1));}out}
fn run(label:&str,d:i64,g:Pt,base:&[Pt],y_tests:&[u64],x_scans:&[i64],s:&[(i64,Pt,u64)]){
 let mut b=if label=="fixed16"{16}else{8};let mut t=table(base,b);let mut setup_adds=(b*(b+1)/2)as u64;let mut trial_adds=0;let mut rows:Vec<Vec<i64>>=Vec::new();let mut attempts=0;let mut answer=None;let mut accepted=0;
 assert_eq!(base[0],g);let mut anchor=vec![0;18];anchor[0]=1;anchor[17]=1;rows.push(anchor);
 for (idx,&(a,r,cost)) in s.iter().enumerate(){if label=="expand128"&&idx==EXPAND_AT{b=16;t=table(base,b);setup_adds+=(b*(b+1)/2)as u64;}
  attempts+=1;trial_adds+=cost;if let Some(&(i,j))=t.get(&r){assert_eq!(add(base[i],base[j]),r);let mut row=vec![0;18];row[i]+=1;row[j]+=1;row[16]=-1;row[17]=a;rows.push(row);accepted+=1;let (_,_,v)=stats(&rows);if v.is_some(){answer=v;break;}}
 }
 let (rb,ra,check)=stats(&rows);assert_eq!(answer,check);if let Some(x)=answer{assert_eq!(x,d);assert_eq!(mul(x,g),mul(d,g));}
 println!("target={d} policy={label} attempts={attempts} hits={accepted} rank_base={rb} rank_aug={ra} pinned={} recovered={} setup_pair_adds={setup_adds} trial_group_adds={trial_adds} total_group_adds={} base_y_tests={} base_x_scans={}",answer.is_some(),answer.unwrap_or(-1),setup_adds+trial_adds,y_tests[b-1],x_scans[b-1]);
}
fn main(){let g=Pt::A(0,425);assert_eq!(mul(N,g),Pt::O);let count=1+(0..P).flat_map(|x|(0..P).map(move|y|(x,y))).filter(|&(x,y)|red(y*y-x*x*x-x-14,P)==0).count();assert_eq!(count,N as usize);
 let(base,y_tests,x_scans)=base();println!("curve_p={P} group_order={N} group_count={count} cap={CAP} expand_at={EXPAND_AT} factor_base_sizes=8,16");
 for &d in &TARGETS{let q=mul(d,g);let s=stream(g,q,0x6e71_2026_1124u64^d as u64);for label in ["fixed8","fixed16","expand128"]{run(label,d,g,&base,&y_tests,&x_scans,&s);}}
}
