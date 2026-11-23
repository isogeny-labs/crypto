from pathlib import Path
import hashlib,json
D=Path(__file__).resolve().parent;R=D.parent/'feature-native-projected-transport'
def once(s,a,b):
 assert s.count(a)==1,(a,s.count(a))
 return s.replace(a,b)
for n in ['lib.rs','source.rs','source-extra.rs','transport-extra.rs','tests.rs']:(D/n).write_bytes((R/n).read_bytes())
s=(R/'transport.rs').read_text()
s=once(s,'let mut d=division::ValueDivision::new(self.p,self.a,self.b,q[0],q[1])', '''if self.powers.iter().all(|&v|v==0){return Ok(vec![[1,1]]);}
 #[cfg(test)]FACTOR_ENGINES.with(|n|n.set(n.get()+1));
 let mut d=division::ValueDivision::new(self.p,self.a,self.b,q[0],q[1])''')
s=once(s,'for (&n,&power) in self.indices.iter().zip(&self.powers){let exponent=', '''for (&n,&power) in self.indices.iter().zip(&self.powers){if power==0{continue;}
 #[cfg(test)]FACTOR_VALUES.with(|n|n.set(n.get()+1));
 let exponent=''')
s+='''
#[cfg(test)]thread_local!{
 static FACTOR_VALUES:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};
 static FACTOR_ENGINES:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};
}
#[cfg(test)]mod factor_tests{
 use super::*;
 #[test]fn omitted_direct_factors(){
  for(p,a,b,r,base,active)in[(5,2,1,7,[0,1],1),(7,0,3,13,[1,2],0),(509,2,10,479,[3,68],2)]{
   let s=Source::new(p,a,b,r,base,128,32).unwrap();let c=s.phase(base).unwrap();
   let before=(FACTOR_VALUES.with(|n|n.get()),FACTOR_ENGINES.with(|n|n.get()));
   let v=s.values(base,c).unwrap();let after=(FACTOR_VALUES.with(|n|n.get()),FACTOR_ENGINES.with(|n|n.get()));
   assert_eq!(after.0-before.0,active);assert_eq!(after.1-before.1,usize::from(active>0));
   assert_eq!(s.powers.iter().filter(|&&v|v!=0).count(),active);
   if active==0{assert_eq!(v,vec![[1,1]]);}
   println!("p={p} r={r} retained_direct_factor_requests={active} available_factors={} value_engine_created={}",s.indices.len(),after.1-before.1);
  }
 }
}
'''
(D/'transport.rs').write_text(s)
h=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
(D/'generation.json').write_text(json.dumps({'inputs':{n:h(R/n)for n in ['lib.rs','source.rs','source-extra.rs','transport.rs','transport-extra.rs','tests.rs']},'outputs':{n:h(D/n)for n in ['lib.rs','source.rs','source-extra.rs','transport.rs','transport-extra.rs','tests.rs']}},indent=2)+'\n')
