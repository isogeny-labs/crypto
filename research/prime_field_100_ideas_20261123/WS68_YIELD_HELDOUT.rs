// Held-out target panel for #96. One cold process per target and policy.
// Feedback rule is copied unchanged from WS68_YIELD_BASE: at trial 64,
// expand B8->B16 iff fewer than two relation hits have occurred.
use std::collections::HashMap;use std::time::Instant;
const P:i64=1009;const N:i64=1013;const CAP:usize=512;const CHECK:usize=64;const MIN_HITS:usize=2;
const TARGETS:[i64;8]=[37,83,191,349,613,769,887,997];
#[derive(Clone,Copy,Eq,PartialEq,Hash,Debug)]enum Pt{O,A(i64,i64)}
fn red(x:i64,n:i64)->i64{x.rem_euclid(n)}
fn inv(x:i64,n:i64)->i64{let(mut a,mut b,mut u,mut v)=(red(x,n),n,1,0);while b!=0{let q=a/b;(a,b)=(b,a-q*b);(u,v)=(v,u-q*v)}assert_eq!(a,1);red(u,n)}
fn neg(q:Pt)->Pt{match q{Pt::O=>q,Pt::A(x,y)=>Pt::A(x,red(-y,P))}}
fn add(q:Pt,t:Pt)->Pt{match(q,t){(Pt::O,v)|(v,Pt::O)=>v,(Pt::A(x,y),Pt::A(u,v))=>{if x==u&&red(y+v,P)==0{return Pt::O}let s=if x==u{red((3*x*x+1)*inv(2*y,P),P)}else{red((v-y)*inv(u-x,P),P)};let xx=red(s*s-x-u,P);Pt::A(xx,red(s*(x-xx)-y,P))}}}
fn ga(q:Pt,t:Pt,c:&mut u64)->Pt{*c+=1;add(q,t)}
fn mul(mut k:i64,mut q:Pt,c:&mut u64)->Pt{if k<0{k=-k;q=neg(q)}let mut out=Pt::O;while k>0{if k&1!=0{out=ga(out,q,c)}q=ga(q,q,c);k>>=1}out}
fn valid(q:Pt)->bool{match q{Pt::O=>true,Pt::A(x,y)=>red(y*y-x*x*x-x-14,P)==0}}
fn base(b:usize)->(Vec<Pt>,u64,i64){let mut out=Vec::new();let mut tests=0;let mut xmax=0;for x in 0..P{for y in 0..P{tests+=1;let q=Pt::A(x,y);if valid(q){out.push(q);xmax=x;break}}if out.len()==b{break}}assert_eq!(out.len(),b);(out,tests,xmax+1)}
fn table(base:&[Pt],c:&mut u64)->HashMap<Pt,(usize,usize)>{let mut t=HashMap::new();for i in 0..base.len(){for j in i..base.len(){t.entry(ga(base[i],base[j],c)).or_insert((i,j));}}t}
#[derive(Default)]struct FCost{add:u64,mul:u64,inv:u64,rank_calls:u64}
fn fm(a:i64,b:i64,c:&mut FCost)->i64{c.mul+=1;red(a*b,N)}
fn fa(a:i64,b:i64,c:&mut FCost)->i64{c.add+=1;red(a+b,N)}
fn fi(a:i64,c:&mut FCost)->i64{c.inv+=1;inv(a,N)}
fn reduce(mut rows:Vec<Vec<i64>>,cols:usize,c:&mut FCost)->(usize,Vec<Vec<i64>>){c.rank_calls+=1;let mut h=0;for j in 0..cols{if let Some(k)=(h..rows.len()).find(|&k|red(rows[k][j],N)!=0){rows.swap(h,k);let z=fi(rows[h][j],c);for v in &mut rows[h]{*v=fm(*v,z,c)}let pivot=rows[h].clone();for i in 0..rows.len(){if i!=h{let z=rows[i][j];if z!=0{for k in j..rows[i].len(){rows[i][k]=fa(rows[i][k],-fm(z,pivot[k],c),c)}}}}h+=1}}(h,rows)}
fn stats(rows:&[Vec<i64>],c:&mut FCost)->(usize,usize,Option<i64>){let rb=reduce(rows.iter().map(|r|r[..16].to_vec()).collect(),16,c).0;let(ra,rref)=reduce(rows.to_vec(),17,c);let mut ans=None;for row in rref{if row[..16].iter().all(|&v|v==0)&&row[16]!=0{ans=Some(fm(row[17],fi(row[16],c),c));break}}(rb,ra,ans)}
#[repr(C)]struct Timespec{sec:i64,nsec:i64}
unsafe extern "C"{fn clock_gettime(clock_id:i32,tp:*mut Timespec)->i32;}
fn cpu_ns()->u128{let mut t=Timespec{sec:0,nsec:0};let rc=unsafe{clock_gettime(12,&mut t)};assert_eq!(rc,0);t.sec as u128*1_000_000_000+t.nsec as u128}
fn step(state:&mut u64)->u64{*state^=*state<<13;*state^=*state>>7;*state^=*state<<17;*state}
fn collect(label:&str,g:Pt,q:Pt)->(usize,usize,usize,usize,Option<i64>,&'static str,usize,u64,u64,u64,u64,u64,u64,u64,FCost,u64){let initial=if label=="fixed16"{16}else{8};let(mut fb,mut tests,mut scans)=base(initial);let mut setup_adds=0;let mut t=table(&fb,&mut setup_adds);let mut b=initial;let mut decision="not_reached";let mut candidate_adds=0;let mut verify_adds=0;let mut lookups=0;let mut rows=Vec::new();let mut anchor=vec![0;18];anchor[0]=1;anchor[17]=1;rows.push(anchor);let mut hits=0;let mut attempts=0;let mut ans=None;let mut c=FCost::default();let mut state=0x6e71_2026_1124u64^match q{Pt::O=>0,Pt::A(x,y)=>((x as u64)<<32)^(y as u64)};let mut digest=0xcbf2_9ce4_8422_2325u64;
 for idx in 0..CAP{if label=="yield64"&&idx==CHECK{if hits<MIN_HITS{let(f,yt,xs)=base(16);fb=f;tests+=yt;scans+=xs;b=16;t=table(&fb,&mut setup_adds);decision="expand"}else{decision="stay"}}let a=(step(&mut state)%N as u64)as i64;digest^=a as u64;digest=digest.wrapping_mul(0x100000001b3);let ag=mul(a,g,&mut candidate_adds);let r=ga(ag,q,&mut candidate_adds);attempts+=1;lookups+=1;if let Some(&(i,j))=t.get(&r){assert_eq!(ga(fb[i],fb[j],&mut verify_adds),r);let mut row=vec![0;18];row[i]+=1;row[j]+=1;row[16]=-1;row[17]=a;rows.push(row);hits+=1;let(_,_,found)=stats(&rows,&mut c);if found.is_some(){ans=found;break}}}
 let(rb,ra,check)=stats(&rows,&mut c);assert_eq!(ans,check);(attempts,hits,rb,ra,ans,decision,b,tests,scans as u64,setup_adds,candidate_adds,verify_adds,lookups,digest,c,fb.len()as u64)}
fn main(){let args:Vec<String>=std::env::args().collect();assert_eq!(args.len(),3);let d:i64=args[1].parse().unwrap();assert!(TARGETS.contains(&d));let label=args[2].as_str();assert!(["fixed8","fixed16","yield64"].contains(&label));let wall=Instant::now();let cpu0=cpu_ns();let g=Pt::A(0,425);assert!(valid(g));let mut target_adds=0;let q=mul(d,g,&mut target_adds);assert!(valid(q));let(attempts,hits,rb,ra,ans,decision,b,tests,scans,setup,candidate,verify,lookups,digest,c,fb_len)=collect(label,g,q);if let Some(x)=ans{assert_eq!(x,d);let mut check_adds=0;assert_eq!(mul(x,g,&mut check_adds),q)}let cpu=cpu_ns()-cpu0;let elapsed=wall.elapsed().as_nanos();println!("target={d} q={q:?} policy={label} attempts={attempts} hits={hits} rank_base={rb} rank_aug={ra} pinned={} recovered={} decision={decision} final_base={b} fb_len={fb_len} setup_base_y_tests={tests} setup_base_x_scans={scans} setup_pair_group_adds={setup} target_group_adds={target_adds} candidate_group_adds={candidate} source_verify_group_adds={verify} relation_lookups={lookups} rank_calls={} rank_mod_adds={} rank_mod_muls={} rank_mod_invs={} candidate_prefix_fnv64={digest:016x} wall_ns={elapsed} process_cpu_ns={cpu}",ans.is_some(),ans.unwrap_or(-1),c.rank_calls,c.add,c.mul,c.inv);}
