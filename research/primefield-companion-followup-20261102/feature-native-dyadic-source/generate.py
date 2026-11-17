from pathlib import Path
import hashlib,json
p=Path(__file__).resolve().parent;source=p.parent/'feature-native-source/lib.rs';s=source.read_text().split('#[cfg(test)] mod integration_tests')[0]
s=s.replace('#[path="../feature-native-division/lib.rs"] mod jets;\n','')
s=s.replace('pub fn new(p:u64,a:u64,b:u64,r:u64,base:[u64;2])->Result<Self,Error>{','''pub fn new(p:u64,a:u64,b:u64,r:u64,base:[u64;2],max_lanes:usize)->Result<Self,Error>{
  if p<5||p>u64::MAX/2||r>u64::MAX/2||r==p{return Err(Error::Domain)}
  let(mut x,mut y)=(r,p-1);while y!=0{(x,y)=(y,x%y)}if x!=1{return Err(Error::Domain)}''')
s=s.replace('let partition=partition(r)?;let curve=', '''let partition=partition(r)?;let[w,k,s,_]=partition;
  if 1+w as u128+k as u128+u128::from(s!=0)>max_lanes as u128{return Err(Error::Domain)}let curve=''')
a=s.index('  let mut jet=');b=s.index('  let mut scaled=',a)
s=s[:a]+'''  let mut values=division::ValueDivision::new(p,self.a,self.b,q[0],q[1]).map_err(|_|Error::Division)?;
  if values.value(self.r)!=0{return Err(Error::Domain)}
  let order=1u64<<(p-1).trailing_zeros();let r2=mul(self.r%order,self.r%order,order);let exponent=inverse(r2,order)?;
  let w1=values.value(self.r+1);let w2=values.value(2);let w3=values.value(self.r+2);
  if w1==0||w2==0||w3==0{return Err(Error::Degenerate)}
  let beta=mul(mul(mul(w1,w1,p),w2,p),inverse(w3,p)?,p);
  // pi(seed)=pi(beta)^(-1/r^2), all exponent inversion in Z/order Z.
  let seed=pow(inverse(beta,p)?,exponent,p);
'''+s[b:]
s=s.replace('value:mul(numerator,inverse(denominator,p)?,p)', 'value:pow(mul(numerator,inverse(denominator,p)?,p),(p-1)/order,p)')
(p/'lib.rs').write_text(s)
(p/'generation.json').write_text(json.dumps({'source':str(source),'source_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),'output_sha256':hashlib.sha256(s.encode()).hexdigest()},indent=2)+'\n')
