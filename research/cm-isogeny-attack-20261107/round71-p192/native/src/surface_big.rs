mod surface;
struct BigOps<'a>(&'a E);
impl surface::Ops for BigOps<'_>{type El=K;fn c(&self,n:i64)->K{if n>=0{self.0.c(n as u32)}else{self.0.negk(&self.0.c((-n)as u32))}}fn add(&self,a:&K,b:&K)->K{self.0.addk(a,b)}fn mul(&self,a:&K,b:&K)->K{self.0.mulk(a,b)}}
fn surface_contract_big(e:&E,c:&TargetCache,gb:&[Pt],giants:&[(K,K,K,K)])->([K;2],Value){let babies:Vec<_>=gb.iter().zip(&c.qb).skip(1).map(|(p,q)|{let(t,y)=p.as_ref().unwrap();let(v,w)=q.as_ref().unwrap();(t.clone(),v.clone(),e.mulk(y,w))}).collect();surface::contract(&BigOps(e),&(e.a.clone(),Z::zero()),&(e.b.clone(),Z::zero()),&babies,giants,2)}
