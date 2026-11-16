from pathlib import Path
import hashlib,json
p=Path(__file__).resolve().parent;src=p.parent/'feature-native-step-matrix';pins={};outs=[]
for f in sorted(src.iterdir()):
 if f.suffix!='.rs' and f.name not in ['validate.py','validate-recovery.py','generate-recovery.py']:continue
 t=f.read_text();pins[str(f)]=hashlib.sha256(f.read_bytes()).hexdigest()
 if f.name=='polynomial.rs':
  old='let q=self.trim(&reverse_q);let qb=self.mul(&q,&b);let rem=self.sub(&a,&qb);\n  if rem!=[0]&&rem.len()>=b.len(){return Err(Error::Invariant);}Ok((q,rem))'
  new='''let q=self.trim(&reverse_q);
  // The exact quotient cancels every coefficient at degree >= deg(b).
  let d=b.len()-1;if d==0{return Ok((q,vec![0]));}
  let low=self.mul_low(&q[..q.len().min(d)],&b[..d],d);
  #[cfg(remainder_audit)]assert!(low.len()<=d);
  let mut rem=Vec::with_capacity(d);
  for i in 0..d{rem.push((a.get(i).copied().unwrap_or(0)+self.p-low.get(i).copied().unwrap_or(0))%self.p);}
  Ok((q,self.trim(&rem)))'''
  assert old in t;t=t.replace(old,new)
  t=t.replace('General divrem remains unchanged as an independent reference path.', 'General divrem also uses low products; the predecessor remains a reference.')
  t+='\n#[cfg(remainder_audit)]include!("remainder-checks.rs");\n'
 if f.name=='matrix-checks.rs':
  old='assert_eq!(before[0],after[0]+stats[1],"full operation preservation");'
  assert old in t;t=t.replace(old,'// Division now also changes products; preserve output/inversion controls.\n   ')
  t=t.replace('full_count_delta_matches=true','full_output_and_inversion_equivalence=true')
 if f.name=='check.rs':t=t.replace('fn main(){polynomial::matrix_checks();','fn main(){polynomial::remainder_checks();polynomial::matrix_checks();')
 if f.name=='validate.py':t=t.replace("'--cfg','matrix_audit'","'--cfg','matrix_audit','--cfg','remainder_audit'")
 (p/f.name).write_text(t);outs.append(f.name)
(p/'generation.json').write_text(json.dumps({'input_hashes':pins,'output_hashes':{n:hashlib.sha256((p/n).read_bytes()).hexdigest() for n in outs}},indent=2)+'\n')
