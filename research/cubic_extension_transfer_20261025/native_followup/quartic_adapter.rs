// Appended to the pinned reference by build_native.sh; same Rust translation
// unit and counters. Fixed quartic stage replay, not elliptic-to-Jacobian map.

fn signed_terms(text: &str) -> Vec<(i64, String)> {
    let s = text.replace(' ', "").replace('-', "+-");
    s.split('+').filter(|t| !t.is_empty()).map(|t| {
        let (sign,t) = if let Some(x)=t.strip_prefix('-'){(-1,x)}else{(1,t)};
        let end=t.find(|c:char| c.is_alphabetic()).unwrap_or(t.len());
        let prefix=t[..end].trim_end_matches('*');
        let c=if prefix.is_empty(){1}else{prefix.parse::<i64>().unwrap()};
        (sign*c,t[end..].to_string())
    }).collect()
}
fn exponent(term:&str,var:char)->usize{
    if let Some(i)=term.find(var){
        let tail=&term[i+1..];
        if let Some(t)=tail.strip_prefix('^') {t.chars().take_while(|c|c.is_ascii_digit()).collect::<String>().parse().unwrap()}else{1}
    }else{0}
}
fn coefficients(text:&str,p:u64)->Poly {
    let mut out=vec![0;4];
    for(c,t)in signed_terms(text){let e=if t.is_empty(){0}else{exponent(&t,t.chars().find(|c|c.is_alphabetic()).unwrap())};assert!(e<4);out[e]=(out[e] as i64+c).rem_euclid(p as i64)as u64;}
    out
}
fn parse_quartic(text:&str,p:u64)->[[u64;5];5]{
    let mut fc=[[0;5];5];for(c,t)in signed_terms(text){let i=exponent(&t,'X');let j=exponent(&t,'Y');let k=exponent(&t,'Z');assert_eq!(i+j+k,4);fc[i][j]=(fc[i][j]as i64+c).rem_euclid(p as i64)as u64;}fc
}
fn fixed_instance(path:&str,seed:u64)->Instance{
    let s=std::fs::read_to_string(path).unwrap();let v:Vec<_>=s.lines().collect();assert_eq!(v.len(),10);
    let q:u64=v[0].parse().unwrap();let ell:u64=v[1].parse().unwrap();assert!(q>13&&is_prime(q)&&is_prime(ell));let fc=parse_quartic(v[2],q);let f=Fq(q);
    assert!(is_smooth(f,&fc));let mut rng=Rng::new(seed);let mut chosen=None;
    for _ in 0..10000{let o=random_point(f,&fc,&mut rng);let d=derivs(f,&fc,o.0,o.1);let dx=d[1];let dy=f.neg(d[0]);let g2=f.add(f.add(f.mul(d[2],f.mul(dx,dx)),f.mul(d[3],f.mul(dx,dy))),f.mul(d[4],f.mul(dy,dy)));if d[1]!=0&&g2!=0{chosen=Some(o);break;}}
    let o=chosen.expect("non-flex smooth reference point");let n1=rational_point_count(f,&fc,&mut rng);let curve=Curve::new(f,fc,o);let mut ctx=Ctx::new(curve,seed^0x1234);
    for _ in 0..1000 {
        if ctx.pool.len()==8 {break;}
        let c=random_class(&mut ctx,&fc);
        if let Ok(nc)=ctx.neg(&c){ctx.pool.push(c);ctx.pool.push(nc);}else{ctx.fails+=1;}
    }
    assert_eq!(ctx.pool.len(),8,"arithmetic helper pool budget exhausted");
    let mut pq=None;let k:u64=v[9].parse().unwrap();
    for _ in 0..1000 {let p=random_class(&mut ctx,&fc);if p==ctx.id()||ctx.smul(ell,&p)!=Some(ctx.id()){continue;}if let Some(qq)=ctx.smul(k,&p){if [3,5,7,11].iter().all(|&a|ctx.smul(a,&p).is_some()&&ctx.smul(a,&qq).is_some()){pq=Some((p,qq));break;}}}
    let(p,qq)=pq.expect("fixture subgroup points");Instance{ctx,q,n_group:ell,ell,cof:1,p,qq,k,n1,curve_tries:0}
}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
struct F3([u64;3]);
#[derive(Clone)]
struct Source { f:Fq, modulus:[u64;3],a:F3,b:F3,r:u64 }
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
struct EP {x:F3,y:F3,inf:bool}
impl EP{fn zero()->Self{Self{x:F3([0;3]),y:F3([0;3]),inf:true}}fn key(&self)->[u64;7]{[self.x.0[0],self.x.0[1],self.x.0[2],self.y.0[0],self.y.0[1],self.y.0[2],self.inf as u64]}}
impl Source {
 fn addf(&self,a:F3,b:F3)->F3{let mut c=[0;3];for i in 0..3{c[i]=self.f.add(a.0[i],b.0[i]);}F3(c)}
 fn negf(&self,a:F3)->F3{F3(a.0.map(|x|self.f.neg(x)))}
 fn subf(&self,a:F3,b:F3)->F3{self.addf(a,self.negf(b))}
 fn mulf(&self,a:F3,b:F3)->F3{let mut c=[0;5];for i in 0..3{for j in 0..3{c[i+j]=self.f.add(c[i+j],self.f.mul(a.0[i],b.0[j]));}}for i in(3..5).rev(){for j in 0..3{c[i-3+j]=self.f.sub(c[i-3+j],self.f.mul(c[i],self.modulus[j]));}}F3([c[0],c[1],c[2]])}
 fn powf(&self,mut a:F3,mut e:u64)->F3{let mut b=F3([1,0,0]);while e>0{if e&1==1{b=self.mulf(a,b);}a=self.mulf(a,a);e>>=1;}b}
 fn divf(&self,a:F3,b:F3)->F3{
  assert_ne!(b,F3([0;3]));
  // Adjugate of multiplication by b: one base-field inversion, rather than
  // exponentiation by p^3-2. This is important for a fair native source control.
  let ztimes=|c:F3|F3([self.f.neg(self.f.mul(self.modulus[0],c.0[2])),self.f.sub(c.0[0],self.f.mul(self.modulus[1],c.0[2])),self.f.sub(c.0[1],self.f.mul(self.modulus[2],c.0[2]))]);
  let z=ztimes(b);let zz=ztimes(z);let m=[b.0,z.0,zz.0];
  let c0=self.f.sub(self.f.mul(m[1][1],m[2][2]),self.f.mul(m[2][1],m[1][2]));
  let c1=self.f.sub(self.f.mul(m[2][1],m[0][2]),self.f.mul(m[0][1],m[2][2]));
  let c2=self.f.sub(self.f.mul(m[0][1],m[1][2]),self.f.mul(m[1][1],m[0][2]));
  let determinant=self.f.add(self.f.add(self.f.mul(m[0][0],c0),self.f.mul(m[1][0],c1)),self.f.mul(m[2][0],c2));
  let d=self.f.inv(determinant);self.mulf(a,F3([self.f.mul(c0,d),self.f.mul(c1,d),self.f.mul(c2,d)]))
 }
 fn constant(&self,n:u64)->F3{F3([n%self.f.0,0,0])}
 fn negp(&self,p:EP)->EP{if p.inf{p}else{EP{x:p.x,y:self.negf(p.y),inf:false}}}
 fn addp(&self,p:EP,q:EP)->EP{
  if p.inf{return q;}if q.inf{return p;}
  let m=if p.x==q.x{if self.addf(p.y,q.y)==F3([0;3]){return EP::zero();}self.divf(self.addf(self.mulf(self.constant(3),self.mulf(p.x,p.x)),self.a),self.mulf(self.constant(2),p.y))}else{self.divf(self.subf(p.y,q.y),self.subf(p.x,q.x))};
  let x=self.subf(self.subf(self.mulf(m,m),p.x),q.x);let y=self.subf(self.mulf(m,self.subf(p.x,x)),p.y);EP{x,y,inf:false}
 }
 fn mulp(&self,mut n:u64,mut p:EP)->EP{let mut q=EP::zero();while n>0{if n&1==1{q=self.addp(q,p);}p=self.addp(p,p);n>>=1;}q}
 fn on_curve(&self,p:EP)->bool{p.inf||self.mulf(p.y,p.y)==self.addf(self.addf(self.mulf(self.mulf(p.x,p.x),p.x),self.mulf(self.a,p.x)),self.b)}
}
fn source_fixture(path:&str)->(Source,EP,EP,u64){
 let s=std::fs::read_to_string(path).unwrap();let v:Vec<_>=s.lines().collect();let p:u64=v[0].parse().unwrap();let r=v[1].parse().unwrap();let md=coefficients(v[3],p);assert_eq!(md[3],1);
 let parts:Vec<_>=v[4].split('(').collect();let aa=parts[1].split(')').next().unwrap();let bb=parts[2].split(')').next().unwrap();let cv=|s:&str|{let c=coefficients(s,p);F3([c[0],c[1],c[2]])};
 let ec=Source{f:Fq(p),modulus:[md[0],md[1],md[2]],a:cv(aa),b:cv(bb),r};let point=EP{x:cv(v[5]),y:cv(v[6]),inf:false};let target=EP{x:cv(v[7]),y:cv(v[8]),inf:false};let k=v[9].parse().unwrap();
 assert!(ec.on_curve(point)&&ec.on_curve(target)&&ec.mulp(r,point).inf&&ec.mulp(r,target).inf&&ec.mulp(k,point)==target);(ec,point,target,k)
}
fn source_rho(ec:&Source,p:EP,q:EP,seed:u64,neg:bool)->(u64,u64,u64,usize){
 let r=ec.r;let mut rng=Rng::new(seed);let mut jumps=Vec::new();for _ in 0..32{let a=1+rng.below(r-1);let b=1+rng.below(r-1);jumps.push((ec.addp(ec.mulp(a,p),ec.mulp(b,q)),a,b));}
 let mut table:HashMap<[u64;7],(u64,u64)>=HashMap::new();let mut steps=0;let mut restarts=0;
 for _ in 0..1000{
  restarts+=1;let(mut a,mut b)=(rng.below(r),rng.below(r));let mut w=ec.addp(ec.mulp(a,p),ec.mulp(b,q));
  for _ in 0..(100*r.isqrt()+1000){
   if neg&&!w.inf{let nw=ec.negp(w);if nw.key()<w.key(){w=nw;a=(r-a)%r;b=(r-b)%r;}}
   let key=w.key();cn(1);if let Some(&(aa,bb))=table.get(&key){let denominator=(bb+r-b)%r;if denominator!=0{let k=mulmod((a+r-aa)%r,invmod(denominator,r),r);assert_eq!(ec.mulp(k,p),q);return(k,steps,restarts,table.len());}else{break;}}
   table.insert(key,(a,b));let h=mix(key[0]^mix(key[1]^mix(key[2]^mix(key[3]))));let j=(h%32)as usize;w=ec.addp(w,jumps[j].0);a=(a+jumps[j].1)%r;b=(b+jumps[j].2)%r;steps+=1;
  }
 }
 panic!("source rho budget exhausted")
}
fn main(){
 let args:Vec<String>=std::env::args().collect();if args.get(1).map(|s|s.as_str())!=Some("fixture"){reference_main();return;}
 assert_eq!(args.len(),5,"fixture FILE ARM SEED");let path=&args[2];let arm=&args[3];let seed:u64=args[4].parse().unwrap();
 // Input validation outside the measured solver interval; reported separately.
 let setup=std::time::Instant::now();let(ec,ep,eq,k)=source_fixture(path);let source_input_seconds=setup.elapsed().as_secs_f64();
 let c0=snap();let begin=std::time::Instant::now();
 if arm=="source-rho"||arm=="source-rho-neg"{
  let(answer,steps,restarts,entries)=source_rho(&ec,ep,eq,seed,arm.ends_with("neg"));let wall=begin.elapsed().as_secs_f64();let cnt=diff(snap(),c0);assert_eq!(answer,k);
  println!("{{\"arm\":\"{arm}\",\"p\":{},\"r\":{},\"seed\":{seed},\"verified\":true,\"scalar\":{answer},\"solver_seconds\":{wall:.9},\"input_validation_seconds\":{source_input_seconds:.9},\"steps\":{steps},\"restarts\":{restarts},\"table_entries\":{entries},\"M\":{},\"I\":{},\"NAT\":{},\"LA\":{},\"scope\":\"native direct source solver including jump setup and verification\"}}",ec.f.0,ec.r,cnt.m,cnt.i,cnt.nat,cnt.la);return;
 }
 let construction=std::time::Instant::now();let mut inst=fixed_instance(path,seed);let target_fixture_seconds=construction.elapsed().as_secs_f64();let c1=snap();let phase=std::time::Instant::now();
 let result=match arm.as_str(){"target-ic"=>run_ic(&mut inst,300,seed^0x1c),"target-rho"=>run_rho(&mut inst,false,seed^0x2d),"target-rho-neg"=>run_rho(&mut inst,true,seed^0x3e),_=>panic!("unknown arm")};
 let solver=phase.elapsed().as_secs_f64();let counted=diff(snap(),c1);let total=begin.elapsed().as_secs_f64();
 if let Some(answer)=result.scalar {assert_eq!(answer,k);}
 let scalar=result.scalar.map(|x|x.to_string()).unwrap_or_else(||"null".to_string());
 println!("{{\"arm\":\"{arm}\",\"p\":{},\"r\":{},\"seed\":{seed},\"scalar\":{scalar},\"found\":{},\"verified\":{},\"solver_seconds\":{solver:.9},\"target_fixture_seconds\":{target_fixture_seconds:.9},\"stage_total_seconds\":{total:.9},\"input_validation_seconds\":{source_input_seconds:.9},\"M\":{},\"I\":{},\"NAT\":{},\"LA\":{},\"reported_M\":{},\"reported_I\":{},\"steps\":{},\"adds\":{},\"extra\":\"{}\",\"scope\":\"fixed quartic synthetic subgroup stage; elliptic transfer phases absent\"}}",ec.f.0,ec.r,result.found,result.verified,counted.m,counted.i,counted.nat,counted.la,result.cnt.m,result.cnt.i,result.steps,result.adds,result.extra);
 if !result.found||!result.verified{std::process::exit(3);}
}

#[test]
fn native_fixture_field_and_scalar(){
 let path=std::env::var("CUBIC_FIXTURE").expect("CUBIC_FIXTURE");let(ec,p,q,k)=source_fixture(&path);assert_eq!(ec.addp(p,ec.negp(p)),EP::zero());assert_eq!(ec.mulp(k,p),q);for a in[1,3,17,41]{for b in[2,9,23]{assert_eq!(ec.addp(ec.mulp(a,p),ec.mulp(b,p)),ec.mulp(a+b,p));let x=F3([a%ec.f.0,b%ec.f.0,1]);assert_eq!(ec.divf(ec.constant(1),x),ec.powf(x,ec.f.0.pow(3)-2));}}
}
