from pathlib import Path
import json,hashlib
p=Path(__file__).resolve().parent;src=p.parent/'feature-native-packed-products';pins={};outs=[]
for f in sorted(src.iterdir()):
 if f.suffix!='.rs' and f.name!='validate-recovery.py':continue
 t=f.read_text();pins[str(f)]=hashlib.sha256(f.read_bytes()).hexdigest()
 if f.name in ('packed-helper.rs','interval.rs'):
  t=t.replace('evaluation_batches:usize}','evaluation_batches:usize,progression:Option<ProgressionValues>,convolution_batches:usize,query_trees:usize}')
  t=t.replace('evaluation_batches:0})','evaluation_batches:0,progression:None,convolution_batches:0,query_trees:0})')
  old='self.max_points=self.max_points.max(points.len());let tree=self.p.product_tree(points).map_err(|_|Error::Arithmetic)?;\n  let values=self.p.values(self.polynomial.as_ref().ok_or(Error::Arithmetic)?,&tree).map_err(|_|Error::Arithmetic)?;'
  new='self.max_points=self.max_points.max(points.len());let step=self.width as u64;\n  let eligible=points.len()>=2&&points.windows(2).all(|v|v[1]==(v[0]+step)%self.r);\n  let accelerated=if eligible{if self.progression.is_none(){self.progression=Some(ProgressionValues::new(self.r,&mut self.p,self.polynomial.as_ref().ok_or(Error::Arithmetic)?,step)?);}self.progression.as_ref().unwrap().evaluate(&mut self.p,points[0],points.len())?}else{None};\n  let values=if let Some(values)=accelerated{self.convolution_batches+=1;values}else{let tree=self.p.product_tree(points).map_err(|_|Error::Arithmetic)?;self.query_trees+=1;self.p.values(self.polynomial.as_ref().ok_or(Error::Arithmetic)?,&tree).map_err(|_|Error::Arithmetic)?};'
  assert old in t;t=t.replace(old,new)
  if f.name=='interval.rs':t='include!("progression.rs");\n'+t
 if f.name=='packed-checks.rs':
  t=t.replace('fewer_tree_panels=','fewer_batch_panels=').replace('equal_tree_panels=','equal_batch_panels=').replace('coefficient_chunk_trees=','coefficient_chunk_batches=').replace('separate_chunk_trees=','separate_chunk_batches=')
 if f.name=='new-recovery.rs':t=t.replace('fn main(){','fn main(){progression_checks();',1)
 (p/f.name).write_text(t);outs.append(f.name)
(p/'generation.json').write_text(json.dumps({'input_hashes':pins,'output_hashes':{n:hashlib.sha256((p/n).read_bytes()).hexdigest() for n in outs}},indent=2)+'\n')
