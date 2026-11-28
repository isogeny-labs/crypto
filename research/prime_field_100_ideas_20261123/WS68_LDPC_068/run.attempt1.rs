// Partial #84: exact block-Krylov projection on a 9x9 independent-row core.
// This is not block Lanczos diagonalization. The same natural EC matrix feeds
// the block arm and Gaussian control. All source/witness rows are replayed.
use std::fs;use std::time::Instant;
const P:i64=1009;const MOD:i64=1013;const S:usize=8;const L:usize=16;const Q:usize=24;const RHS:usize=25;const N:usize=9;
#[derive(Clone,Copy,Debug,Eq,PartialEq)]enum Pt{O,A(i64,i64)}
fn m(x:i64,n:i64)->i64{x.rem_euclid(n)}
fn inv(x:i64,n:i64)->i64{let(mut a,mut b,mut u,mut v)=(m(x,n),n,1,0);while b!=0{let q=a/b;(a,b)=(b,a-q*b);(u,v)=(v,u-q*v)}assert_eq!(a,1);m(u,n)}
fn neg(p:Pt)->Pt{match p{Pt::O=>p,Pt::A(x,y)=>Pt::A(x,m(-y,P))}}
fn add(a:Pt,b:Pt)->Pt{match(a,b){(Pt::O,t)|(t,Pt::O)=>t,(Pt::A(x,y),Pt::A(u,v))=>{if x==u&&m(y+v,P)==0{return Pt::O}let s=if x==u{m((3*x*x+1)*inv(2*y,P),P)}else{m((v-y)*inv(u-x,P),P)};let xx=m(s*s-x-u,P);Pt::A(xx,m(s*(x-xx)-y,P))}}}
fn mul(mut k:i64,mut p:Pt)->Pt{if k<0{k=-k;p=neg(p)}let mut o=Pt::O;while k>0{if k&1!=0{o=add(o,p)}p=add(p,p);k>>=1}o}
fn base()->Vec<Pt>{let mut v=Vec::new();for x in 0..P{for y in 0..P{if m(y*y-x*x*x-x-14,P)==0{v.push(Pt::A(x,y));break}}if v.len()==S+L{break}}v}
#[derive(Default,Clone,Copy)]struct Ops{adds:u64,muls:u64,invs:u64,matvec:u64,nonzeros:u64}
fn plus(x:i64,y:i64,o:&mut Ops)->i64{o.adds+=1;m(x+y,MOD)}
fn prod(x:i64,y:i64,o:&mut Ops)->i64{o.muls+=1;m(x*y,MOD)}
fn inverse(x:i64,o:&mut Ops)->i64{o.invs+=1;inv(x,MOD)}
#[derive(Clone)]struct Row{v:Vec<i64>,w:Vec<i64>}
fn rank(a:&[Vec<i64>],width:usize,o:&mut Ops)->usize{let mut z=a.to_vec();let mut h=0;for c in 0..width{if let Some(k)=(h..z.len()).find(|&i|z[i][c]!=0){z.swap(h,k);let p=inverse(z[h][c],o);for j in c..width{z[h][j]=prod(z[h][j],p,o)}let pivot=z[h].clone();for i in 0..z.len(){if i!=h{let q=z[i][c];if q!=0{for j in c..width{z[i][j]=plus(z[i][j],-prod(q,pivot[j],o),o)}}}}h+=1}}h}
fn solve(a:&[Vec<i64>],b:&[i64],o:&mut Ops)->Option<Vec<i64>>{let n=b.len();let mut z=vec![vec![0;n+1];n];for i in 0..n{z[i][..n].copy_from_slice(&a[i]);z[i][n]=b[i]}for c in 0..n{let k=(c..n).find(|&i|z[i][c]!=0)?;z.swap(c,k);let p=inverse(z[c][c],o);for j in c..=n{z[c][j]=prod(z[c][j],p,o)}let pivot=z[c].clone();for i in 0..n{if i!=c{let q=z[i][c];if q!=0{for j in c..=n{z[i][j]=plus(z[i][j],-prod(q,pivot[j],o),o)}}}}}Some(z.iter().map(|r|r[n]).collect())}
fn matvec(a:&[Vec<i64>],x:&[i64],o:&mut Ops)->Vec<i64>{o.matvec+=1;let n=x.len();let mut y=vec![0;n];for i in 0..n{for j in 0..n{if a[i][j]!=0{o.nonzeros+=1;y[i]=plus(y[i],prod(a[i][j],x[j],o),o)}}}y}
fn parse()->Vec<Row>{let raw=fs::read_to_string("WS68_NATURAL_PARTIAL.stdout").unwrap();let base=base();let g=base[0];let q=Pt::A(779,357);let mut sources=Vec::new();let mut block=0;let mut seen_first=false;for line in raw.lines(){if !line.starts_with("source "){continue}let t:Vec<&str>=line.split_whitespace().collect();assert_eq!(t.len(),10);let id:usize=t[1][3..].parse().unwrap();if id==0{if seen_first{block+=1;sources.clear()}else{seen_first=true}}if block==0{continue}assert_eq!(block,1);assert_eq!(id,sources.len());let a:i64=t[3][2..].parse().unwrap();let b:i64=t[4][2..].parse().unwrap();let x:i64=t[5][4..].trim_end_matches(',').parse().unwrap();let y:i64=t[6].trim_end_matches(')').parse().unwrap();let r=Pt::A(x,y);let i:usize=t[8][2..].parse().unwrap();let j:usize=t[9][2..].parse().unwrap();assert_eq!(add(base[i],base[j]),r);assert_eq!(add(mul(a,g),mul(b,q)),r);let mut v=vec![0;RHS+1];v[i]+=1;v[j]+=1;v[Q]=m(-b,MOD);v[RHS]=a;sources.push(v)}assert_eq!(sources.len(),152);let mut pivots:Vec<Option<Row>>=vec![None;L];let mut out=Vec::new();let mut op=Ops::default();for (id,src) in sources.iter().enumerate(){let mut w=vec![0;sources.len()];w[id]=1;let mut row=Row{v:src.clone(),w};let mut pivoted=false;for j in 0..L{let col=S+j;if row.v[col]==0{continue}if let Some(p)=&pivots[j]{let z=row.v[col];for k in 0..=RHS{row.v[k]=plus(row.v[k],-prod(z,p.v[k],&mut op),&mut op)}for k in 0..row.w.len(){row.w[k]=plus(row.w[k],-prod(z,p.w[k],&mut op),&mut op)}}else{let z=inverse(row.v[col],&mut op);for v in &mut row.v{*v=prod(*v,z,&mut op)}for v in &mut row.w{*v=prod(*v,z,&mut op)}pivots[j]=Some(row.clone());pivoted=true;break}}if !pivoted{assert!(row.v[S..Q].iter().all(|&v|v==0));out.push(row)}}assert_eq!(out.len(),136);for row in &out{let mut replay=vec![0;RHS+1];for(i,&z)in row.w.iter().enumerate(){if z!=0{for j in 0..=RHS{replay[j]=m(replay[j]+z*sources[i][j],MOD)}}}assert_eq!(replay,row.v);let mut lhs=Pt::O;for j in 0..S+L{if row.v[j]!=0{lhs=add(lhs,mul(row.v[j],base[j]))}}lhs=add(lhs,mul(row.v[Q],q));assert_eq!(lhs,mul(row.v[RHS],g))}println!("source_rows={} complete_rows={} witness_and_group_replays={} large_pivots={} filter_mod_adds={} filter_mod_muls={} filter_mod_invs={}",sources.len(),out.len(),out.len(),pivots.iter().filter(|p|p.is_some()).count(),op.adds,op.muls,op.invs);out}
fn vector(row:&[i64])->Vec<i64>{row[..S].iter().copied().chain([row[Q]]).collect()}
fn rows_for(completed:&[Row])->Vec<Vec<i64>>{
 let mut rows=Vec::with_capacity(completed.len()+1);
 let mut anchor=vec![0;RHS+1];anchor[0]=1;anchor[RHS]=1;rows.push(anchor);
 rows.extend(completed.iter().map(|r|r.v.clone()));rows
}
fn peel(label:&str,rows:&[Vec<i64>],g:Pt,q:Pt){
 let mut known=[None;N];let mut ops=Ops::default();let mut visits=0u64;let mut nz_visits=0u64;let mut assigned=0usize;let mut rounds=0usize;
 let start=Instant::now();
 loop {
  rounds+=1;let mut changed=false;
  for row in rows{
   visits+=1;let mut residual=row[RHS];let mut missing=0usize;let mut last=0usize;let mut coef=0i64;
   for j in 0..N{
    let c=if j<S{row[j]}else{row[Q]};if c==0{continue}nz_visits+=1;
    if let Some(v)=known[j]{residual=plus(residual,-prod(c,v,&mut ops),&mut ops)}else{missing+=1;last=j;coef=c}
   }
   if missing==0{assert_eq!(m(residual,MOD),0,"inconsistent checked row")}
   else if missing==1{
    let v=prod(residual,inverse(coef,&mut ops),&mut ops);known[last]=Some(v);assigned+=1;changed=true;
    println!("peel arm={label} assigned_col={last} assigned_value={v} round={rounds}");
   }
  }
  if !changed{break}
 }
 let wall=start.elapsed().as_micros();
 let target=known[S];if let Some(v)=target{assert_eq!(mul(v,g),q)}
 if assigned==N{for row in rows{let mut lhs=0;for j in 0..N{let c=if j<S{row[j]}else{row[Q]};lhs=m(lhs+c*known[j].unwrap(),MOD)}assert_eq!(lhs,m(row[RHS],MOD))}}
 println!("peel_summary arm={label} assigned={assigned}/{N} rounds={rounds} target_pinned={} target={} row_visits={visits} nonzero_visits={nz_visits} field_adds={} field_muls={} field_invs={} solve_wall_us={wall}",target.is_some(),target.unwrap_or(-1),ops.adds,ops.muls,ops.invs);
}
fn gaussian(label:&str,rows:&[Vec<i64>],g:Pt,q:Pt){
 let start=Instant::now();let mut selected=Vec::new();let mut rhs=Vec::new();let mut selection=Ops::default();
 for row in rows{let v=vector(row);let old=rank(&selected,N,&mut selection);let mut next=selected.clone();next.push(v.clone());if rank(&next,N,&mut selection)>old{selected.push(v);rhs.push(row[RHS])}if selected.len()==N{break}}
 assert_eq!(selected.len(),N);let mut solveops=Ops::default();let x=solve(&selected,&rhs,&mut solveops).unwrap();
 assert_eq!(mul(x[S],g),q);for row in rows{let mut lhs=0;for j in 0..N{let c=if j<S{row[j]}else{row[Q]};lhs=m(lhs+c*x[j],MOD)}assert_eq!(lhs,m(row[RHS],MOD))}
 println!("gaussian arm={label} rank={N} target={} selected={} selection_adds={} selection_muls={} selection_invs={} solve_adds={} solve_muls={} solve_invs={} wall_us={}",x[S],selected.len(),selection.adds,selection.muls,selection.invs,solveops.adds,solveops.muls,solveops.invs,start.elapsed().as_micros());
}
fn arm(label:&str,completed:&[Row],g:Pt,q:Pt){
 let rows=rows_for(completed);let mut a=Ops::default();let rank_base=rank(&rows.iter().map(|r|r[..S].to_vec()).collect::<Vec<_>>(),S,&mut a);
 let rank_aug=rank(&rows.iter().map(|r|vector(r)).collect::<Vec<_>>(),N,&mut a);
 println!("arm={label} source_completed={} rows_with_anchor={} rank_base={rank_base} rank_aug={rank_aug} rank_adds={} rank_muls={} rank_invs={}",completed.len(),rows.len(),a.adds,a.muls,a.invs);
 peel(label,&rows,g,q);gaussian(label,&rows,g,q)
}
fn main(){
 let all=parse();assert_eq!(all.len(),136);
 assert!(all[..17].iter().all(|r|r.w.iter().filter(|&&x|x!=0).count()==1));
 let base=base();let g=base[0];let q=Pt::A(779,357);assert_eq!(base.len(),24);
 arm("full_only",&all[..17],g,q);arm("partial_filter",&all,g,q);
 println!("RUN_OK=1");
}
