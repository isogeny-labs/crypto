from pathlib import Path
import hashlib,json
D=Path(__file__).resolve().parent;R=D.parent/'feature-native-projected-euclid'
def once(s,a,b):
 assert s.count(a)==1,(a,s.count(a))
 return s.replace(a,b)
names=['lib.rs','source.rs','source-extra.rs','transport.rs','transport-extra.rs','tests.rs','subgroup.rs','subgroup-tests.rs','residual.rs','residual-tests.rs','polynomial.rs','product-tests.rs','euclid-tests.rs']
for n in names:
 s=(R/n).read_text()
 if n=='polynomial.rs':
  s=once(s,'impl Poly{',(D/'class.rs').read_text()+'impl Poly{')
  a=s.index(' fn project_power(');b=s.index(' /// Build only the product polynomial:',a)
  method=s[a:b];method=method[method.index(' /// Standard Res'):]
  method=once(method,' pub fn projected_resultant(&mut self,a:&[u64],b:&[u64])->Result<u64,Error>{',' pub fn resultant_class(&mut self,a:&[u64],b:&[u64])->Result<ResultantClass,Error>{')
  assert method.count('return Ok(0);')==2;method=method.replace('return Ok(0);','return self.image_class(0);')
  method=once(method,'  let v=self.project_power(acc,(self.p-1)/order);Ok(if negative{self.p-v}else{v})','  let class=self.image_class(acc)?;Ok(if negative{class.negate()}else{class})')
  method+=' pub fn image_class(&self,v:u64)->Result<ResultantClass,Error>{if 1u64<<(self.p-1).trailing_zeros()>4{return Err(Error::Domain);}Ok(ResultantClass{p:self.p,representative:v%self.p})}\n'
  method+=' pub fn projected_resultant(&mut self,a:&[u64],b:&[u64])->Result<u64,Error>{Ok(self.resultant_class(a,b)?.project())}\n'
  s=s[:a]+method+s[b:]
  anchor=' pub fn root_projected_resultant(&mut self,roots:&[u64],points:&[u64])->Result<u64,Error>{'
  s=once(s,anchor,' pub fn root_resultant_class(&mut self,roots:&[u64],points:&[u64])->Result<ResultantClass,Error>{let f=self.root_product(roots)?;let g=self.root_product(points)?;self.resultant_class(&g,&f)}\n'+anchor)
  s+='\n#[cfg(test)]mod class_tests{use super::*;include!("class-tests.rs");}\n'
 if n=='residual.rs':
  assert s.count('root_projected_resultant(')==2;s=s.replace('root_projected_resultant(','root_resultant_class(')
  s=once(s,'resultant=mul(resultant,tail,p);','resultant=resultant.combine(tail).map_err(|_|Error::Polynomial)?;')
  s=once(s,' if resultant==0{return Err(Error::Degenerate);}',' if resultant.is_zero(){return Err(Error::Degenerate);}')
  s=once(s,' let mut out=resultant;\n if order==4{out=mul(out,pow(square_class,(p-1)/2,p),p);}', ' if order==4{let correction=poly.image_class(square_class).map_err(|_|Error::Polynomial)?.square();resultant=resultant.combine(correction).map_err(|_|Error::Polynomial)?;}\n let mut out=resultant.project();')
  s=once(s,'use super::{mul,pow,Error,Pair};','use super::{mul,Error,Pair};')
  s+='\n#[cfg(test)]pub(crate) fn reset_projection_count(){polynomial::reset_projections();}\n#[cfg(test)]pub(crate) fn projection_count()->usize{polynomial::projections()}\n'
 if n=='residual-tests.rs':
  s=once(s,'let got=residual::geometry_ratio(p,part,q,&grid);','residual::reset_projection_count();let got=residual::geometry_ratio(p,part,q,&grid);assert_eq!(residual::projection_count(),usize::from(got.is_ok()));')
  s=once(s,'synthetic_geometry_cases={} valid={valid} rejected={rejected}','synthetic_geometry_cases={} valid={valid} rejected={rejected} one_projection_per_valid=true')
 (D/n).write_text(s)
h=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
(D/'generation.json').write_text(json.dumps({'inputs':{str(p):h(p)for p in [*(R/n for n in names),D/'class.rs',D/'class-tests.rs']},'outputs':{n:h(D/n)for n in names}},indent=2)+'\n')
