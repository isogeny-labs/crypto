from pathlib import Path
import hashlib,json
D=Path(__file__).resolve().parent;R=D.parent/'feature-native-torsion-validation'
def once(s,a,b):
 assert s.count(a)==1,(a,s.count(a))
 return s.replace(a,b)
for n in ['lib.rs','source.rs','source-extra.rs','tests.rs','subgroup-tests.rs']:(D/n).write_bytes((R/n).read_bytes())
s=(R/'subgroup.rs').read_text()
s=once(s,'pub fn validate(p:u64,a:u64,b:u64,r:u64,q:[u64;2])->Result<(),Error>{', '''pub struct Validated{p:u64,a:u64,b:u64,r:u64,q:[u64;2],values:division::ValueDivision}
impl Validated{
 pub fn matches(&self,p:u64,a:u64,b:u64,r:u64,q:[u64;2])->bool{(self.p,self.a,self.b,self.r,self.q)==(p,a,b,r,q)}
 pub fn value(&mut self,n:u64)->u64{self.values.value(n)}
 #[cfg(test)]pub fn retained(&self)->usize{self.values.retained_indices()}
}
pub fn validate(p:u64,a:u64,b:u64,r:u64,q:[u64;2])->Result<(),Error>{validated(p,a,b,r,q).map(|_|())}
pub fn validated(p:u64,a:u64,b:u64,r:u64,q:[u64;2])->Result<Validated,Error>{''')
s=once(s,' let mut values=division::ValueDivision::new', ' #[cfg(test)]ENGINES.with(|n|n.set(n.get()+1));\n let mut values=division::ValueDivision::new')
s=once(s,'Ok(())\n}', 'Ok(Validated{p,a,b,r,q,values})\n}')
s+='\n#[cfg(test)]thread_local!{static ENGINES:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};}\n#[cfg(test)]pub fn engines()->usize{ENGINES.with(|n|n.get())}\n'
(D/'subgroup.rs').write_text(s)
s=(R/'transport.rs').read_text()
start=s.index(' fn values(');end=s.index('\n pub fn prepare',start);old=s[start:end]
cached=once(old,'fn values(&self,q:[u64;2],_c:u64)', 'fn cached_values(&self,q:[u64;2],mut d:super::super::subgroup::Validated)')
cached=once(cached,'{#[cfg(test)]VALUE_CALLS','{if !d.matches(self.p,self.a,self.b,self.r,q){return Err(Error::Domain);}\n #[cfg(test)]VALUE_CALLS')
cached=once(cached,' #[cfg(test)]FACTOR_ENGINES.with(|n|n.set(n.get()+1));\n let mut d=division::ValueDivision::new(self.p,self.a,self.b,q[0],q[1]).map_err(|_|Error::Division)?;', '')
initial=''' fn initial(&self,q:[u64;2])->Result<Vec<Pair>,Error>{
 #[cfg(test)]SUBGROUP_CHECKS.with(|n|n.set(n.get()+1));
 let d=super::super::subgroup::validated(self.p,self.a,self.b,self.r,q).map_err(|_|Error::Phase)?;
 self.cached_values(q,d)
 }
'''
s=s[:end]+ '\n'+cached+'\n'+initial+s[end:]
s=once(s,'out.validate(base)?;let first=out.values(base,1)?;', 'let first=out.initial(base)?;')
s=once(s,'s.validate(point)?;out.direct_initializations+=1;(s.values(point,1)?,"direct")','out.direct_initializations+=1;(s.initial(point)?,"direct")')
s+='''
#[cfg(test)]mod cache_tests{
 use super::*;
 #[test]fn one_engine_and_binding(){
  for(p,a,b,r,base)in[(5,2,1,7,[0,1]),(7,0,3,13,[1,2]),(509,2,10,479,[3,68])]{
   let s=Source::new(p,a,b,r,base,128,32).unwrap();
   let want=s.values(base,1).unwrap();
   let before=(super::super::super::subgroup::engines(),FACTOR_ENGINES.with(|n|n.get()));
   assert_eq!(s.initial(base).unwrap(),want);
   let after=(super::super::super::subgroup::engines(),FACTOR_ENGINES.with(|n|n.get()));
   assert_eq!(after.0-before.0,1);assert_eq!(after.1-before.1,0);
   let mut cache=super::super::super::subgroup::validated(p,a,b,r,base).unwrap();
   let retained=cache.retained();assert_eq!(cache.value(r),0);assert_eq!(retained,cache.retained());
   assert!(!cache.matches(p,a,b,r,[base[0],p-base[1]]));assert!(!cache.matches(p,a,b,r+2,base));
   assert!(!cache.matches(p+2,a,b,r,base));assert!(!cache.matches(p,(a+1)%p,b,r,base));assert!(!cache.matches(p,a,(b+1)%p,r,base));
   assert!(matches!(s.cached_values([base[0],p-base[1]],cache),Err(Error::Domain)));
   println!("p={p} r={r} validation_engines=1 extra_factor_engines=0 cached_r_request_adds_entries=0");
  }
 }
}
'''
(D/'transport.rs').write_text(s)
s=(R/'transport-extra.rs').read_text();s=once(s,'s.validate(q)?;(s.values(q,1)?,"direct")','(s.initial(q)?,"direct")');(D/'transport-extra.rs').write_text(s)
h=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
names=['lib.rs','source.rs','source-extra.rs','transport.rs','transport-extra.rs','tests.rs','subgroup.rs','subgroup-tests.rs']
(D/'generation.json').write_text(json.dumps({'inputs':{n:h(R/n)for n in names},'outputs':{n:h(D/n)for n in names}},indent=2)+'\n')
