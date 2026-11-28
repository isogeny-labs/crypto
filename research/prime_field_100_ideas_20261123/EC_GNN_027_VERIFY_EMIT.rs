// Frozen one-hop heterogeneous candidate/base/target message passing screen.
// Public prime-field EC points only; labels come from exact point sums.
use std::time::Instant;
const B:usize=16;const H:usize=8;const N:usize=B*B*B*8;const K:usize=N/4;const F:usize=6;const WIDTH:usize=8;
#[derive(Clone,Copy,PartialEq,Eq,Debug)]struct Pt{x:i64,y:i64,inf:bool}
fn idpt()->Pt{Pt{x:0,y:0,inf:true}}
fn md(x:i64,p:i64)->i64{x.rem_euclid(p)}
fn pow(mut a:i64,mut e:i64,p:i64)->i64{let mut v=1;while e>0{if e&1==1{v=md(v*a,p)}a=md(a*a,p);e>>=1}v}
fn add(a:Pt,b:Pt,p:i64)->Pt{if a.inf{return b}if b.inf{return a}if a.x==b.x&&md(a.y+b.y,p)==0{return idpt()}
let slope=if a==b{md((3*a.x*a.x+1)*pow(md(2*a.y,p),p-2,p),p)}else{md((b.y-a.y)*pow(md(b.x-a.x,p),p-2,p),p)};
let x=md(slope*slope-a.x-b.x,p);let y=md(slope*(a.x-x)-a.y,p);Pt{x,y,inf:false}}
fn neg(a:Pt,p:i64)->Pt{if a.inf{a}else{Pt{x:a.x,y:md(-a.y,p),inf:false}}}
fn order(mut a:Pt,p:i64,cap:usize)->usize{let x=a;for k in 1..=cap{if a.inf{return k}a=add(a,x,p)}panic!("order cap")}
fn points(p:i64,b:i64,expected:&[i64;H])->(Vec<Pt>,Vec<Pt>){let mut all=Vec::new();for x in 1..p{let v=md(x*x*x+x+b,p);for y in 0..p{if md(y*y,p)==v{all.push(Pt{x,y,inf:false});break}}if all.len()==B+H{break}}
assert_eq!(all.len(),B+H);assert_eq!(all[B..].iter().map(|q|q.x).collect::<Vec<_>>(),expected);(all[..B].to_vec(),all[B..].to_vec())}
fn decode(id:usize)->(usize,usize,usize,usize){let m=id%8;let t=id/8;(t/256,(t/16)%16,t%16,m)}
fn signed(p:Pt,m:usize,bit:usize,modulus:i64)->Pt{if (m>>bit)&1==1{neg(p,modulus)}else{p}}
fn sum(id:usize,base:&[Pt],p:i64)->Pt{let(i,j,k,m)=decode(id);add(add(signed(base[i],m,0,p),signed(base[j],m,1,p),p),signed(base[k],m,2,p),p)}
fn rows_rank(rows:&[Vec<i64>],r:i64)->usize{let mut a=rows.to_vec();let mut rank=0;for c in 0..B+H{if let Some(k)=(rank..a.len()).find(|&i|md(a[i][c],r)!=0){a.swap(rank,k);let d=pow(md(a[rank][c],r),r-2,r);for j in c..B+H{a[rank][j]=md(a[rank][j]*d,r)}for i in 0..a.len(){if i!=rank{let q=a[i][c];for j in c..B+H{a[i][j]=md(a[i][j]-q*a[rank][j],r)}}}rank+=1;if rank==a.len(){break}}}rank}
fn row(id:usize,u:usize)->Vec<i64>{let(i,j,k,m)=decode(id);let mut v=vec![0;B+H];for (idx,bit) in [(i,0),(j,1),(k,2)]{v[idx]+=if (m>>bit)&1==1{-1}else{1}}v[B+u]=-1;v}
fn mix(mut x:u64)->u64{x=x.wrapping_add(0x9e3779b97f4a7c15);x=(x^(x>>30)).wrapping_mul(0xbf58476d1ce4e5b9);x=(x^(x>>27)).wrapping_mul(0x94d049bb133111eb);x^(x>>31)}
fn features(id:usize,u:usize,base:&[Pt],targets:&[Pt],p:i64)->[f64;F]{let(i,j,k,m)=decode(id);let q=targets[u];let ps=[signed(base[i],m,0,p),signed(base[j],m,1,p),signed(base[k],m,2,p)];let pf=p as f64;
let sx=ps.iter().map(|v|v.x as f64/pf).sum::<f64>()/3.;let sy=ps.iter().map(|v|v.y as f64/pf).sum::<f64>()/3.;let sx2=ps.iter().map(|v|{let z=v.x as f64/pf;z*z}).sum::<f64>()/3.;[1.,sx,sy,q.x as f64/pf,q.y as f64/pf,sx2]}
struct Net{w:[[f64;F];WIDTH],v:[f64;WIDTH],c:f64}
impl Net{fn new()->Self{let mut w=[[0.;F];WIDTH];let mut v=[0.;WIDTH];for h in 0..WIDTH{for f in 0..F{w[h][f]=(((h*17+f*31)%19) as f64-9.)/50.}v[h]=(((h*13)%11) as f64-5.)/40.}Self{w,v,c:0.}}
fn forward(&self,z:&[f64;F])->(f64,[f64;WIDTH]){let mut hidden=[0.;WIDTH];let mut out=self.c;for h in 0..WIDTH{let mut s=0.;for f in 0..F{s+=self.w[h][f]*z[f]}hidden[h]=s.tanh();out+=self.v[h]*hidden[h]}(out,hidden)}
fn train_one(&mut self,z:&[f64;F],label:bool){let(logit,hid)=self.forward(z);let prob=if logit>=0.{1./(1.+(-logit).exp())}else{let e=logit.exp();e/(1.+e)};let delta=prob-if label{1.}else{0.};let lr=0.01;let old=self.v;for h in 0..WIDTH{self.v[h]-=lr*delta*hid[h];let d=delta*old[h]*(1.-hid[h]*hid[h]);for f in 0..F{self.w[h][f]-=lr*d*z[f]}}self.c-=lr*delta}
fn score(&self,z:&[f64;F])->f64{self.forward(z).0}}
fn labels(base:&[Pt],targets:&[Pt],p:i64)->Vec<u8>{let mut out=vec![0;H*N];let sums=(0..N).map(|id|sum(id,base,p)).collect::<Vec<_>>();for u in 0..H{for id in 0..N{if sums[id]==targets[u]{out[u*N+id]=1}}}out}
fn top_by(scores:Vec<(f64,usize)>)->Vec<usize>{let mut a=scores;a.sort_by(|x,y|y.0.total_cmp(&x.0).then(x.1.cmp(&y.1)));a.into_iter().take(K).map(|x|x.1).collect()}
fn evaluate(name:&str,selected:&[Vec<usize>],base:&[Pt],targets:&[Pt],lab:&[u8],p:i64,r:i64){let mut hits=0;let mut cover=0;let mut rows=Vec::new();let mut missed=0;let mut fullrows=Vec::new();let mut fullhits=0;let mut fullcover=0;
for u in 0..H{let mut this=0;let mut fullthis=0;for id in 0..N{if lab[u*N+id]!=0{fullhits+=1;fullthis+=1;fullrows.push(row(id,u))}}if fullthis>0{fullcover+=1}
for &id in &selected[u]{let verified=sum(id,base,p)==targets[u];assert_eq!(verified,lab[u*N+id]!=0);if verified{this+=1;rows.push(row(id,u))}}
if this>0{cover+=1}hits+=this;println!("arm_step name={} target_x={} selected={} hits={}",name,targets[u].x,selected[u].len(),this);
}
missed+=fullhits-hits;let rank=rows_rank(&rows,r);let fullrank=rows_rank(&fullrows,r);println!("arm_summary name={} selected_total={} exact_hits={} false_positive_checks={} false_negative_hits={} covered={}/{} full_covered={}/{} rank={}/{} full_hits={} full_rank={} rank_loss={} verification_adds={} verification_comparisons={}",name,H*K,hits,H*K-hits,missed,cover,H,fullcover,H,rank,B+H,fullhits,fullrank,fullrank-rank,2*H*K,H*K)}
fn main(){let t0=Instant::now();let(train_base,train_targets)=points(1009,14,&[42,43,44,45,48,49,54,55]);assert_eq!(order(Pt{x:0,y:425,inf:false},1009,1013),1013);assert_eq!(train_base.len(),B);
let tlabels=labels(&train_base,&train_targets,1009);let positives=tlabels.iter().filter(|&&v|v!=0).count();let label_ms=t0.elapsed().as_millis();
let mut net=Net::new();let mut updates=0;for _epoch in 0..3{for u in 0..H{let pos=(0..N).filter(|&id|tlabels[u*N+id]!=0).collect::<Vec<_>>();for (n,&id) in pos.iter().enumerate(){let z=features(id,u,&train_base,&train_targets,1009);net.train_one(&z,true);updates+=1;let mut neg_id=(mix(((u*N+n) as u64)^20261125)%N as u64) as usize;while tlabels[u*N+neg_id]!=0{neg_id=(neg_id+1)%N}let zn=features(neg_id,u,&train_base,&train_targets,1009);net.train_one(&zn,false);updates+=1}}}
println!("train curve_p=1009 order=1013 targets=8 base=16 exact_label_comparisons={} label_group_adds={} positives={} balanced_sgd_updates={} epochs=3 hidden_width=8 feature_dim=6 label_and_train_ms={}",H*N,2*N,positives,updates,t0.elapsed().as_millis());
let h0=Instant::now();let(base,targets)=points(1019,20,&[38,39,41,46,49,53,54,56]);assert_eq!(order(Pt{x:0,y:64,inf:false},1019,1033),1033);let audit=labels(&base,&targets,1019);println!("heldout_audit curve_p=1019 order=1033 targets=8 full_labels={} full_grid_group_adds={} full_grid_comparisons={} audit_ms={}",H*N,2*N,H*N,h0.elapsed().as_millis());
let mut model=Vec::new();let mut cheap=Vec::new();let mut static_arm=Vec::new();let mut random=Vec::new();let score0=Instant::now();for u in 0..H{model.push(top_by((0..N).map(|id|(net.score(&features(id,u,&base,&targets,1019)),id)).collect()));cheap.push(top_by((0..N).map(|id|{let(i,j,k,_)=decode(id);let x=base[i].x^base[j].x^base[k].x;let d=(x^targets[u].x).count_ones() as f64;(-d,id)}).collect()));static_arm.push((0..N).filter(|&id|decode(id).0<4).collect());random.push(top_by((0..N).map(|id|{let x=mix((id as u64)^((u as u64)<<32)^20261125);(-(x as f64),id)}).collect()));}
println!("selection model_scores={} cheap_scores={} random_hashes={} topk_each={} score_sort_ms={}",H*N,H*N,H*N,K,score0.elapsed().as_millis());
for u in 0..H{println!("model_ids_u{}={:?}",u+1,model[u]);}
evaluate("message_passing",&model,&base,&targets,&audit,1019,1033);evaluate("cheap_xor",&cheap,&base,&targets,&audit,1019,1033);evaluate("static_block1",&static_arm,&base,&targets,&audit,1019,1033);evaluate("seeded_hash",&random,&base,&targets,&audit,1019,1033);
println!("complete EC one-hop message passing screen total_ms={}",t0.elapsed().as_millis());}
