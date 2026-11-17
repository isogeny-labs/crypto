"""Mechanical instrumentation only; does not execute a cryptographic algorithm."""
from pathlib import Path
import hashlib,json,re,os
D=Path(__file__).resolve().parent;R=D.parent
names={'geometry':'joint-window','jets':'division','value':'value-division','polynomial':'polynomial','source':'shared-seeds','decoder':'signmask','recovery':'recovery'}
files={name:R/f'feature-native-{part}/lib.rs' for name,part in names.items()}
files['polynomial']=R/'feature-native-low-product/polynomial.rs'
reverse={p.resolve():name+'.rs'for name,p in files.items()}
reverse[(R/'feature-native-polynomial/lib.rs').resolve()]='polynomial.rs'
ledger=[]
for name,path in files.items():
 original=path.read_text();s=original;edits=[]
 def change(old,new):
  global s
  n=s.count(old)
  if not n:raise ValueError((name,'missing',old))
  s=s.replace(old,new);edits.append({'old':old,'new':new,'occurrences':n})
 def hit(c,o):return f'crate::meter::hit(crate::meter::Component::{c},crate::meter::Op::{o});'
 def wrap(expr,c,o):change(expr,f'crate::meter::value(crate::meter::Component::{c},crate::meter::Op::{o},{expr})')
 # Keep fixtures relative and route production dependencies to generated copies.
 for match in list(re.finditer(r'#\[path="([^"]+)"\]',s)):
  old=match.group(0);p=(path.parent/match.group(1)).resolve();new=os.path.relpath(p,D/'generated') if name=='polynomial' else reverse.get(p,os.path.relpath(p,D/'generated'));change(old,'#[path="'+new+'"]')
 for match in {m.group(0):m for m in re.finditer(r'include_str!\("([^"]+)"\)',s)}.values():
  old=match.group(0);p=(path.parent/match.group(1)).resolve();change(old,'include_str!("'+os.path.relpath(p,D/'generated')+'")')
 if name in ('geometry','jets','value'):
  c={'geometry':'Geometry','jets':'Phase','value':'Value'}[name]
  # Only F operations, not Jet product/addition wrappers.
  begin=s.index('impl Add for F');end=s.index('pub enum Error') if name!='jets'else s.index('fn inverse(a:')
  section=s[begin:end]
  for signature,o in [('fn add(self,b:Self)->Self{','AddSub'),('fn sub(self,b:Self)->Self{','AddSub'),('fn mul(self,b:Self)->Self{','Product')]:
   assert section.count(signature)==1
   pos=s.index(signature,begin);s=s[:pos]+s[pos:].replace(signature,signature+hit(c,o),1);edits.append({'old':signature,'new':signature+hit(c,o),'occurrences':1,'first_only':True})
  if name=='jets':
   change('fn neg(self)->Self{Self::new','fn neg(self)->Self{'+hit(c,'Neg')+'Self::new')
  change('fn inv(self)->Option<Self>{','fn inv(self)->Option<Self>{'+hit(c,'Inverse')) if name!='jets' else change('fn inv(self)->Option<Self>{ inverse','fn inv(self)->Option<Self>{'+hit(c,'Inverse')+' inverse')
  loop='while s!=0 {'if name=='jets'else 'while s!=0{';change(loop,loop+hit(c,'EuclidStep'))
  if name=='jets':
   change('let rr=((r as u128','let rr={'+hit(c,'ScalarProduct')+'((r as u128')
   change('*r as u128)%modulus as u128)as u64;','*r as u128)%modulus as u128)as u64};')
   change('let exponent=inverse(rr,modulus)','let exponent={'+hit(c,'ScalarInverse')+'inverse(rr,modulus)}')
 if name=='polynomial':
  c='Polynomial'
  for sig,o in [('fn fm(&self,a:u64,b:u64)->u64{','Product'),('fn inverse(&self,a:u64)->Result<u64,Error>{','Inverse'),('while s!=0{','EuclidStep')]:change(sig,sig+hit(c,o))
  change('c[i]=(c[i]+x)%self.p;',hit(c,'AddSub')+'c[i]=(c[i]+x)%self.p;')
  change('c[i+j]=(c[i+j]+self.fm(x,y))%self.p;',hit(c,'AddSub')+'c[i+j]=(c[i+j]+self.fm(x,y))%self.p;')
  wrap('(self.p-x%self.p)%self.p',c,'Neg');wrap('(self.p-points[0]%self.p)%self.p',c,'Neg')
  wrap('(self.p-tree.polynomial[0])%self.p',c,'Neg')
  wrap('(self.fm(value,x)+coefficient%self.p)%self.p',c,'AddSub')
  wrap('(a.get(i).copied().unwrap_or(0)+self.p-low.get(i).copied().unwrap_or(0))%self.p',c,'AddSub')
  wrap('(result[k+i]+v)%self.p',c,'AddSub')
  wrap('(result[2*k+i]+v)%self.p',c,'AddSub')
 if name=='source':
  c='Source'
  for sig,o in [('fn mul(a:u64,b:u64,p:u64)->u64{','Product'),('fn inverse(a:u64,p:u64)->Result<u64,Error>{','Inverse'),('while s!=0{','EuclidStep')]:change(sig,sig+hit(c,o))
  for expr in ['(a[0]+p-b[0])%p','(mul(3,mul(x,x,p),p)+self.a)%p','(mul(slope,slope,p)+p-mul(2,x,p))%p','(mul(slope,(x+p-xx)%p,p)+p-y)%p','(x+p-xx)%p','(self.source.base[0]+p-middle[0])%p']:wrap(expr,c,'AddSub')
  for expr in ['(p-u)%p','(p-previous[0])%p','(p-scaled(s+e))%p','(p-y)%p','(p-self.base_seed)%p']:wrap(expr,c,'Neg')
  wrap('((n as u128*n as u128)%(p-1)as u128)as u64',c,'ScalarProduct')
 if name=='decoder':
  c='Decoder'
  for old,o in [('support |= 1u64 << a;','Bit'),('odd ^= 1u64 << a;','Bit'),('value += c;','AddSub')]:change(old,hit(c,o)+old)
  change('for t in &self.terms {','for t in &self.terms {'+hit(c,'Term')+hit(c,'Bit'))
  change('let c = if (t.odd & negative)','let c = {'+hit(c,'Bit')+hit(c,'Popcount')+'if (t.odd & negative)')
  change('} else { self.modulus - t.coefficient };','} else { '+hit(c,'Neg')+'self.modulus - t.coefficient }};')
  change('0 => zero |= 1u64 << a,','0 => {'+hit(c,'Bit')+'zero |= 1u64 << a},')
  change('-1 => negative |= 1u64 << a,','-1 => {'+hit(c,'Bit')+'negative |= 1u64 << a},')
 if name=='recovery':
  c='Verifier'
  for sig,o in [('fn fm(a:u64,b:u64,p:u64)->u64{','Product'),('fn inv(a:u64,p:u64)->Result<u64,Error>{','Inverse'),('while s!=0{','EuclidStep')]:change(sig,sig+hit(c,o))
  for expr in ['(y+v)%p','(fm(3,fm(x,x,p),p)+c.a)%p','(v+p-y)%p','(u+p-x)%p','(fm(slope,slope,p)+p-x)%p','(xx+p-u)%p','(fm(slope,(x+p-xx)%p,p)+p-y)%p','(x+p-xx)%p']:wrap(expr,c,'AddSub')
  for sig,stage in [('pub fn new(curve:CurveSpec,width:usize,terms:&[(u64,Vec<usize>)])->Result<Self,Error>{','Setup'),("pub fn prepare(&self)->Result<PreparedDecoder<'_>,Error>{",'Prepare'),('pub fn repaired_479()->Result<Self,Error>{','Setup'),('pub fn recover(&self,q:Option<[u64;2]>)->Result<Recovery,Error>{','Query')]:change(sig,sig+f'let _meter_stage=crate::meter::enter(crate::meter::Stage::{stage});')
  change('let(point,verification_add_calls,verification_double_calls)=multiply(self.decoder.curve,scalar)?;','let(point,verification_add_calls,verification_double_calls)={let _meter_verify=crate::meter::enter(crate::meter::Stage::Verify);multiply(self.decoder.curve,scalar)?};')
 if name in ('geometry','jets','value'):
  c={'geometry':'Geometry','jets':'Phase','value':'Value'}[name]
  marker=hit(c,'Inverse');change(marker,marker+f'if self.v==1{{{hit(c,"UnitInverse")}}}')
 if name in ('polynomial','source','recovery'):
  c={'polynomial':'Polynomial','source':'Source','recovery':'Verifier'}[name]
  marker=hit(c,'Inverse');condition='a%self.p==1'if name=='polynomial'else'a==1';change(marker,marker+f'if {condition}{{{hit(c,"UnitInverse")}}}')
 # Removing only added hooks and path routing must recover all original bytes.
 reverted=s
 for edit in reversed(edits):
  if edit.get('first_only'):reverted=reverted.replace(edit['new'],edit['old'],1)
  else:reverted=reverted.replace(edit['new'],edit['old'])
 assert reverted==original,(name,'inverse transform failed')
 out=D/'generated'/f'{name}.rs';out.write_text(s)
 ledger.append({'module':name,'original':str(path.relative_to(R)),'original_sha256':hashlib.sha256(original.encode()).hexdigest(),'generated_sha256':hashlib.sha256(s.encode()).hexdigest(),'reversed_exactly':True,'edits':edits})
(D/'instrumentation.json').write_text(json.dumps({'schema':'native-instrumentation-map/v1','modules':ledger},indent=2)+'\n')
print('7 generated modules; every reverse transform recovers original bytes')
