from pathlib import Path
import hashlib,json
p=Path(__file__).resolve().parent;src=p.parent/'feature-native-singleton-resultant';pins={};outs=[]
for f in sorted(src.iterdir()):
 if f.suffix!='.rs' and f.name not in ['validate.py','validate-recovery.py','generate-recovery.py']:continue
 t=f.read_text();pins[str(f)]=hashlib.sha256(f.read_bytes()).hexdigest()
 if f.name=='polynomial.rs':
  marker=' fn step_matrix(&self,q:&[u64])->Matrix{'
  addition=''' /// Left Euclidean action on a canonical matrix; move the retained row.
 fn step_left(&mut self,q:&[u64],matrix:Matrix)->Matrix{
  #[cfg(matrix_audit)]STEP_AUDIT.with(|a|{let mut v=a.get();v[0]+=1;v[1]+=matrix.iter().flatten().map(|c|if c==&[0]{0}else{c.len()}).sum::<usize>();a.set(v)});
  let [[a,b],[c,d]]=matrix;
  let qc=self.mul(q,&c);let qd=self.mul(q,&d);
  let left=self.sub(&a,&qc);let right=self.sub(&b,&qd);
  [[c,d],[left,right]]
 }
 #[cfg(matrix_audit)]
'''
  assert marker in t;t=t.replace(marker,addition+marker)
  old='let step=self.step_matrix(&q);matrix=self.matrix_mul(&step,&matrix);'
  assert old in t;t=t.replace(old,'matrix=self.step_left(&q,matrix);')
  old='let step=self.step_matrix(&q);let prefix=self.matrix_mul(&step,&first.matrix);'
  assert old in t;t=t.replace(old,'let prefix=self.step_left(&q,first.matrix);')
  t+='\n#[cfg(matrix_audit)]std::thread_local!{static STEP_AUDIT:std::cell::Cell<[usize;2]>=const{std::cell::Cell::new([0;2])};}\n#[cfg(matrix_audit)]include!("matrix-checks.rs");\n'
 if f.name=='check.rs':t=t.replace('fn main(){singleton_checks();','fn main(){polynomial::matrix_checks();singleton_checks();')
 if f.name=='validate.py':t=t.replace("'--cfg','resultant_audit'","'--cfg','resultant_audit','--cfg','matrix_audit'")
 (p/f.name).write_text(t);outs.append(f.name)
(p/'generation.json').write_text(json.dumps({'input_hashes':pins,'output_hashes':{n:hashlib.sha256((p/n).read_bytes()).hexdigest() for n in outs}},indent=2)+'\n')
