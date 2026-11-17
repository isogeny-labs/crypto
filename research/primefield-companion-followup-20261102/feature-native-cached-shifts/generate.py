from pathlib import Path
import json,hashlib
p=Path(__file__).resolve().parent;src=p.parent/'feature-native-progression-values';pins={};outs=[]
for f in sorted(src.iterdir()):
 if f.suffix!='.rs' and f.name!='validate-recovery.py':continue
 t=f.read_text();pins[str(f)]=hashlib.sha256(f.read_bytes()).hexdigest()
 if f.name=='progression.rs':
  t=t.replace('weights:Vec<u64>}','weights:Vec<u64>,samples:Vec<u64>}').replace('Ok(Self{r,d,inv_step,weights})','Ok(Self{r,d,inv_step,weights,samples:values})')
  old='let r=self.r;let a=mul(first,self.inv_step,r);let n=self.d.checked_add(count).ok_or(Error::Limit)?;'
  new='self.evaluate_normalized(p,mul(first,self.inv_step,self.r),count)\n }\n fn evaluate_normalized(&self,p:&mut polynomial::Poly,a:u64,count:usize)->Result<Option<Vec<u64>>,Error>{\n  if a>=self.r||count==0||count>self.d{return Err(Error::Domain)}let r=self.r;let n=self.d.checked_add(count).ok_or(Error::Limit)?;'
  assert old in t;t=t.replace(old,new);t+='\ninclude!("cached.rs");\n'
 if f.name in ('interval.rs','packed-helper.rs','progression-checks.rs'):
  t=t.replace('convolution_batches','progression_batches')
 if f.name in ('interval.rs','packed-helper.rs'):
  t=t.replace('query_trees:usize}','query_trees:usize,convolution_calls:usize,cached_copies:usize,cache_only_batches:usize}')
  t=t.replace('query_trees:0})','query_trees:0,convolution_calls:0,cached_copies:0,cache_only_batches:0})')
  old='self.progression.as_ref().unwrap().evaluate(&mut self.p,points[0],points.len())?'
  new='{let result=self.progression.as_ref().unwrap().evaluate_cached(&mut self.p,points[0],points.len())?;self.convolution_calls+=result.convolutions;self.cached_copies+=result.copied;self.cache_only_batches+=usize::from(result.convolutions==0);Some(result.values)}'
  assert old in t;t=t.replace(old,new)
 if f.name=='progression-checks.rs':
  t=t.replace('assert!(helper.query_trees>0);','')
  t=t.replace('progression_fast_batches=','old_control_nonsingular=').replace('singular_fallback_batches=','old_control_singular=').replace('dispatch_convolutions=','dispatch_progression_batches=').replace('coefficient_stream_convolutions=','coefficient_stream_progression_batches=')
 if f.name=='new-recovery.rs':t=t.replace('fn main(){','fn main(){cached_shift_checks();',1)
 (p/f.name).write_text(t);outs.append(f.name)
(p/'generation.json').write_text(json.dumps({'input_hashes':pins,'output_hashes':{n:hashlib.sha256((p/n).read_bytes()).hexdigest() for n in outs}},indent=2)+'\n')
