from pathlib import Path
import hashlib,json
p=Path(__file__).resolve().parent;src=p.parent/'feature-native-sparse-quotient';pins={};outs=[]
for f in sorted(src.iterdir()):
 if f.suffix!='.rs' and f.name not in ['validate.py','validate-recovery.py','generate-recovery.py']:continue
 t=f.read_text();pins[str(f)]=hashlib.sha256(f.read_bytes()).hexdigest()
 if f.name=='polynomial.rs':
  t=t.replace('let(mut term,mut sparse)=(None,true);','let(mut terms,mut weight,mut sparse)=(Vec::new(),0usize,true);')
  t=t.replace('if b[d-j]!=0{if term.is_some(){sparse=false;break;}term=Some((j,b[d-j]));}', 'if b[d-j]!=0{let cost=n-j;if cost>n-1-weight{sparse=false;break;}weight+=cost;terms.push((j,b[d-j]));}')
  t=t.replace('if let Some((j,c))=term{if i>=j && reverse_q[i-j]!=0{value=(value+self.p-self.fm(c,reverse_q[i-j]))%self.p;}}', '''for &(j,c) in &terms{
      #[cfg(sparse_audit)]WEIGHT_AUDIT.with(|a|{let mut v=a.get();v[0]+=1;a.set(v)});
      if j>i{break;}
      #[cfg(sparse_audit)]WEIGHT_AUDIT.with(|a|{let mut v=a.get();v[1]+=1;a.set(v)});
      if reverse_q[i-j]!=0{value=(value+self.p-self.fm(c,reverse_q[i-j]))%self.p;}
     }''')
  t+='\n#[cfg(sparse_audit)]std::thread_local!{static WEIGHT_AUDIT:std::cell::Cell<[usize;2]>=const{std::cell::Cell::new([0;2])};}\n#[cfg(sparse_audit)]include!("weighted-checks.rs");\n'
 if f.name=='quotient-checks.rs':t=t.replace('b.iter().rev().skip(1).take(n-1).filter(|&&x|x!=0).count()>1','b.iter().rev().enumerate().skip(1).take(n-1).filter(|&(_,x)|*x!=0).map(|(j,_)|n-j).sum::<usize>()>n-1')
 if f.name=='check.rs':t=t.replace('fn main(){','fn main(){polynomial::weighted_checks();',1)
 (p/f.name).write_text(t);outs.append(f.name)
(p/'generation.json').write_text(json.dumps({'input_hashes':pins,'output_hashes':{n:hashlib.sha256((p/n).read_bytes()).hexdigest() for n in outs}},indent=2)+'\n')
