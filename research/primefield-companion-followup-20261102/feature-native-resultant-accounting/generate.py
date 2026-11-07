"""Reversible source instrumentation only. Does not execute crypto or panel cases."""
from pathlib import Path
import hashlib,json,re
D=Path(__file__).resolve().parent
ARMS={'scalar':'resultant-product','euclid':'resultant-class','half':'half-gcd'}
OPS=['Product','Add','Subtract','Negate','Normalize','Inverse','EuclidStep','Projection']
HELPERS=['new','fm','inverse','trim','add','sub','mul','mul_low','inv_series','divrem','product_tree','monic_remainder','values','value_product','root_resultant','combine','square','negate','is_zero','project','resultant_class','image_class','projected_resultant','root_product','root_resultant_class','root_projected_resultant','matrix_mul','matrix_apply','step_matrix','lifted','half_gcd','identity','degree','polynomial','reset_projections','projections','reset_batches','batches']
NAMES=OPS+['call_'+x for x in HELPERS]
sha=lambda b:hashlib.sha256(b).hexdigest()
def build():
 (D/'generated').mkdir(exist_ok=True); ledger=[]
 for arm,source in ARMS.items():
  path=D.parent/('feature-native-'+source)/'polynomial.rs';original=path.read_text();s=original;edits=[]
  def change(old,new,required=True):
   nonlocal s
   n=s.count(old)
   if required:assert n,(arm,old)
   if not n:return
   token=f'__INSTRUMENT_{len(edits)}__';assert token not in s
   s=s.replace(old,token);edits.append(dict(old=old,new=new,occurrences=n,token=token))
  def wrap(old,*ops,required=True):
   change(old,'{'+''.join(f'crate::meter::hit({NAMES.index(op)});'for op in ops)+old+'}',required)
  # Only the production prefix is transformed; preserve all test source unchanged.
  s,tests=s.split('#[cfg(test)]mod tests{',1)
  for old in ['((a as u128*b as u128)%self.p as u128)as u64','((self.representative as u128*other.representative as u128)%self.p as u128)as u64','((self.representative as u128*self.representative as u128)%self.p as u128)as u64','((v as u128*a as u128)%self.p as u128)as u64','((a as u128*a as u128)%self.p as u128)as u64']:
   wrap(old,'Product',required=old.startswith('((a as u128*b'))
  for old in ['(self.p-x%self.p)%self.p','(self.p-points[0]%self.p)%self.p','(self.p-roots[0]%self.p)%self.p']:
   wrap(old,'Normalize','Negate',required='roots' not in old)
  for old in ['(self.p-self.representative)%self.p','(self.p-tree.polynomial[0])%self.p']:
   wrap(old,'Negate',required='representative' not in old)
  for old in ['(c[i]+x)%self.p','(c[i+j]+self.fm(x,y))%self.p','(result[k+i]+v)%self.p','(result[2*k+i]+v)%self.p']:
   wrap(old,'Add')
  wrap('(self.fm(value,x)+coefficient%self.p)%self.p','Normalize','Add')
  wrap('(a.get(i).copied().unwrap_or(0)+self.p-low.get(i).copied().unwrap_or(0))%self.p','Subtract')
  for old in ['a%self.p','x%self.p','c%self.p','v%self.p','x.rem_euclid(self.p as i128)']:
   wrap(old,'Normalize',required=old!='v%self.p')
  assert '%self.p' not in s and 'rem_euclid' not in s,(arm,'uncovered modular operation')
  change('while s!=0{','while s!=0{crate::meter::hit(6);')
  # Inject helper-entry counts, plus semantic inverse/projection events.
  for match in list(re.finditer(r'\bfn (\w+)\([^{}]*?\)(?:->[^{}]+)?\{',s)):
   sig=match.group(0);name=match.group(1);assert name in HELPERS,name
   extra='crate::meter::hit(5);'if name=='inverse'else'crate::meter::hit(7);'if name=='project'else''
   change(sig,sig+f'crate::meter::hit({NAMES.index("call_"+name)});'+extra)
  # Replace placeholders in reverse dependency order, then restore for exact audit.
  for e in reversed(edits):s=s.replace(e['token'],e['new'])
  generated=s+'#[cfg(test)]mod tests{'+tests
  reverse=generated
  for e in reversed(edits):
   assert reverse.count(e['new'])==e['occurrences'],(arm,e['old'])
   reverse=reverse.replace(e['new'],e['old'])
  assert reverse==original,arm
  (D/'generated'/f'{arm}.rs').write_text(generated)
  ledger.append(dict(arm=arm,source=str(path.relative_to(D.parent)),source_sha256=sha(original.encode()),generated_sha256=sha(generated.encode()),edits=edits,reverse_exact=True,uncovered_modular_sites=0))
 (D/'generation.json').write_text(json.dumps(ledger,indent=2)+'\n')
 (D/'names.rs').write_text('pub const NAMES:[&str;'+str(len(NAMES))+']=['+','.join(json.dumps(x)for x in NAMES)+'];\n')
 print('Generated three reversible arms; all production modular sites covered.')
if __name__=='__main__':build()
