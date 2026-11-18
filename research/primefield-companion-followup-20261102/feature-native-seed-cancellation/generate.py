from pathlib import Path
import hashlib,json
D=Path(__file__).resolve().parent;R=D.parent/'feature-native-projected-factors'
def once(s,a,b):
 assert s.count(a)==1,(a,s.count(a))
 return s.replace(a,b)
for n in ['lib.rs','source.rs','source-extra.rs','transport-extra.rs','tests.rs']:(D/n).write_bytes((R/n).read_bytes())
s=(R/'transport.rs').read_text()
s=once(s,'let powers=powers.into_iter().map(', 'let powers:Vec<u64>=powers.into_iter().map(')
s=once(s,'  let mut out=Self{p,a,b,r,base,curve,partition,indices,lanes,base_x,squares:vec![],powers,anchors:vec![],max_window};', '''  if !seed_closes(p,&indices,&powers){return Err(Error::Domain);}
  let mut out=Self{p,a,b,r,base,curve,partition,indices,lanes,base_x,squares:vec![],powers,anchors:vec![],max_window};''')
s=once(s,'let c2=mul(pow(c,4,p),mul(2,base[1],p),p);let twice=out.values(double,c2)?;', 'let twice=out.values(double,1)?;')
s=once(s,'fn values(&self,q:[u64;2],c:u64)', 'fn values(&self,q:[u64;2],_c:u64)')
s=once(s,'let exponent=((n as u128*n as u128)%(self.p-1)as u128)as u64;let v=mul(d.value(n),pow(c,exponent,self.p),self.p);', 'let v=d.value(n);')
s+='''
fn seed_closes(p:u64,indices:&[u64],powers:&[u64])->bool{
 if p<=3||indices.len()!=powers.len(){return false;}
 let m=(p-1)as u128;
 indices.iter().zip(powers).fold(0u128,|sum,(&n,&e)|(sum+(e as u128)*((n as u128*n as u128)%m)%m)%m)==0
}
#[cfg(test)]mod seed_tests{
 use super::*;
 #[test]fn closure_and_mutation(){
  let mut checked=0;
  for p in [5u64,7,13,29,61,101,509]{
   let odd=(p-1)/(1u64<<(p-1).trailing_zeros());
   for r in (7..1001).step_by(2){
    let[w,_,s,e]=partition(r).unwrap();let[d,a,b,g]=super::super::exponents(p,r).unwrap();
    let mut indices=vec![r/2+1,2*w];let mut powers=vec![a,b];if s!=0{indices.push(s+e);powers.push(g);}indices.push(1);powers.push(d);
    let mut powers:Vec<u64>=powers.into_iter().map(|v|((v as u128*odd as u128)%(p-1)as u128)as u64).collect();
    assert!(seed_closes(p,&indices,&powers));
    let last=powers.len()-1;powers[last]=(powers[last]+1)%(p-1);assert!(!seed_closes(p,&indices,&powers));checked+=1;
   }
  }
  println!("algebraic_partitions={checked} changed_seed_exponents_rejected={checked}");
 }
}
'''
(D/'transport.rs').write_text(s)
h=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
(D/'generation.json').write_text(json.dumps({'inputs':{n:h(R/n)for n in ['lib.rs','source.rs','source-extra.rs','transport.rs','transport-extra.rs','tests.rs']},'outputs':{n:h(D/n)for n in ['lib.rs','source.rs','source-extra.rs','transport.rs','transport-extra.rs','tests.rs']}},indent=2)+'\n')
