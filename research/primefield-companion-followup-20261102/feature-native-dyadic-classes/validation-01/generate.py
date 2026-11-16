from pathlib import Path
import hashlib,json
p=Path(__file__).resolve().parent;src=p.parent/'feature-native-half-gcd/polynomial.rs';s=src.read_text().replace('#[derive(Debug)]\npub struct ResultantClass','#[derive(Debug,Clone,Copy)]\npub struct ResultantClass')
s=s.replace('if order>4{return Err(Error::Domain);}','').replace('if 1u64<<(self.p-1).trailing_zeros()>4{return Err(Error::Domain);}','')
methods=''' pub fn power(self,exponent:u64)->Self{
  if self.representative==0{return if exponent==0{Self{p:self.p,representative:1}}else{self}}
  let q=1u64<<(self.p-1).trailing_zeros();let(mut e,mut a,mut v)=(exponent%q,self.representative,1u64);
  while e>0{if e&1!=0{v=((v as u128*a as u128)%self.p as u128)as u64}a=((a as u128*a as u128)%self.p as u128)as u64;e>>=1}Self{p:self.p,representative:v}
 }
 pub fn inverse(self)->Result<Self,Error>{if self.is_zero(){return Err(Error::Nonunit)}let q=1u64<<(self.p-1).trailing_zeros();Ok(self.power(q-1))}
'''
s=s.replace('impl ResultantClass{','impl ResultantClass{\n'+methods)
s=s.replace('for _ in 0..(m as u64%order){acc=self.fm(acc,b[0]);}', 'acc=self.class_factor(acc,b[0],m as u64%order);')
s=s.replace('for _ in 0..((t.m-k)as u64%order){acc=self.fm(acc,t.leading);}', 'acc=self.class_factor(acc,t.leading,(t.m-k)as u64%order);')
s=s.replace(' pub fn resultant_class(', ' fn class_factor(&self,acc:u64,base:u64,exponent:u64)->u64{let factor=ResultantClass{p:self.p,representative:base}.power(exponent);((acc as u128*factor.representative as u128)%self.p as u128)as u64}\n pub fn resultant_class(')
(p/'polynomial.rs').write_text(s)
old=p.parent/'feature-native-dyadic-source/lib.rs';t=old.read_text();before=t[:t.index('impl Source {')]
before=before.replace('#[path="../feature-native-polynomial/lib.rs"] mod polynomial;','mod polynomial;').replace('pub struct Raw {pub value:u64,pub seed:u64,pub ratio:[u64;2],pub resultant:u64,pub recurrence_updates:u64,pub partial_value:Option<u64>}','pub struct Raw {pub value:u64}')
before=before.replace('calibration_inverse:u64','calibration:u64')
a=t.index(' pub fn new(');b=t.index(' pub fn raw(',a);ctor=t[a:b].replace('calibration_inverse:0','calibration:0').replace('out.calibration_inverse=inverse(calibration,p)?','out.calibration=calibration')
raw=t[b:t.index(' fn raw_grid(',b)]
window=t[t.index(' /// Produces Q,Q+P'):t.rfind('}')]
normal=''' fn normalize(&self,raw:u64)->Result<i8,Error>{if raw==self.calibration{Ok(1)}else if raw==self.p-self.calibration{Ok(-1)}else{Err(Error::Character)}}
 pub fn sign(&self,q:Option<[u64;2]>)->Result<i8,Error>{match q{None=>Ok(0),Some(q)=>self.normalize(self.raw(q)?.value)}}
'''
(p/'lib.rs').write_text(before+'include!("class-source.rs");\nimpl Source {\n'+ctor+raw+'include_body_placeholder'+normal+window+'}\n')
# Rust include cannot expand methods in impl; splice the handwritten method body.
s=(p/'lib.rs').read_text().replace('include_body_placeholder',(p/'raw-grid.rs').read_text());(p/'lib.rs').write_text(s)
(p/'generation.json').write_text(json.dumps({'input_hashes':{str(x):hashlib.sha256(x.read_bytes()).hexdigest() for x in [src,old,p/'raw-grid.rs']},'output_hashes':{n:hashlib.sha256((p/n).read_bytes()).hexdigest() for n in ['polynomial.rs','lib.rs']}},indent=2)+'\n')
