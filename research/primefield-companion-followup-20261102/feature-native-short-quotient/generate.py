from pathlib import Path
import hashlib,json
p=Path(__file__).resolve().parent;src=p.parent/'feature-native-low-remainder';pins={};outs=[]
for f in sorted(src.iterdir()):
 if f.suffix!='.rs' and f.name not in ['validate.py','validate-recovery.py','generate-recovery.py']:continue
 t=f.read_text();pins[str(f)]=hashlib.sha256(f.read_bytes()).hexdigest()
 if f.name=='polynomial.rs':
  old='  let n=a.len()-b.len()+1;let rev_a:Vec<_>=a.iter().rev().take(n).copied().collect();let rev_b:Vec<_>=b.iter().rev().copied().collect();let inv=self.inv_series(&rev_b,n)?;let mut reverse_q=self.mul_low(&rev_a,&inv,n);reverse_q.truncate(n);reverse_q.resize(n,0);reverse_q.reverse();let q=self.trim(&reverse_q);'
  assert old in t;t=t.replace(old,'  let q=self.quotient_canonical(&a,&b)?;',1)
  helper=''' /// Canonical nonzero divisor, len(a)>=len(b). Short quotient dispatch.
 fn quotient_canonical(&mut self,a:&[u64],b:&[u64])->Result<Vec<u64>,Error>{
  let n=a.len()-b.len()+1;
  if a!=[0] && n<=2{
   let m=a.len()-1;let d=b.len()-1;let leading=b[d];
   let inverse=if leading==1{None}else{Some(self.inverse(leading)?)};
   let high=match inverse{None=>a[m],Some(v)=>self.fm(a[m],v)};
   if n==1{return Ok(vec![high]);}
   let mut low=a[m-1];
   if d>0 && b[d-1]!=0{low=(low+self.p-self.fm(high,b[d-1]))%self.p;}
   if let Some(v)=inverse{low=self.fm(low,v);}
   return Ok(vec![low,high]);
  }
  let rev_a:Vec<_>=a.iter().rev().take(n).copied().collect();let rev_b:Vec<_>=b.iter().rev().copied().collect();let inv=self.inv_series(&rev_b,n)?;let mut reverse_q=self.mul_low(&rev_a,&inv,n);reverse_q.truncate(n);reverse_q.resize(n,0);reverse_q.reverse();Ok(self.trim(&reverse_q))
 }
'''
  t=t.replace(' pub fn divrem(',helper+' pub fn divrem(',1)
  old='  let n=a.len()-b.len()+1;let reverse_a:Vec<_>=a.iter().rev().take(n).copied().collect();let reverse_b:Vec<_>=b.iter().rev().copied().collect();let inv=self.inv_series(&reverse_b,n)?;let mut q=self.mul_low(&reverse_a,&inv,n);q.truncate(n);q.resize(n,0);q.reverse();'
  assert old in t;t=t.replace(old,'  let q=self.quotient_canonical(&a,b)?;',1)
  t+='\n#[cfg(quotient_audit)]include!("quotient-checks.rs");\n'
 if f.name=='check.rs':t=t.replace('fn main(){polynomial::remainder_checks();','fn main(){polynomial::quotient_checks();polynomial::remainder_checks();')
 if f.name=='validate.py':t=t.replace("'--cfg','remainder_audit'","'--cfg','remainder_audit','--cfg','quotient_audit'")
 (p/f.name).write_text(t);outs.append(f.name)
(p/'generation.json').write_text(json.dumps({'input_hashes':pins,'output_hashes':{n:hashlib.sha256((p/n).read_bytes()).hexdigest() for n in outs}},indent=2)+'\n')
