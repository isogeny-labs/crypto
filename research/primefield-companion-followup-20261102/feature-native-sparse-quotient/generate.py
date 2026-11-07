from pathlib import Path
import hashlib,json
p=Path(__file__).resolve().parent;src=p.parent/'feature-native-short-quotient';pins={};outs=[]
for f in sorted(src.iterdir()):
 if f.suffix!='.rs' and f.name not in ['validate.py','validate-recovery.py','generate-recovery.py']:continue
 t=f.read_text();pins[str(f)]=hashlib.sha256(f.read_bytes()).hexdigest()
 if f.name=='polynomial.rs':
  marker='  let rev_a:Vec<_>=a.iter().rev().take(n).copied().collect();let rev_b:'
  insert='''  if a!=[0]{
   let d=b.len()-1;let(mut term,mut sparse)=(None,true);
   for j in 1..=d.min(n-1){
    #[cfg(sparse_audit)]SPARSE_AUDIT.with(|a|{let mut x=a.get();x[0]+=1;a.set(x)});
    if b[d-j]!=0{if term.is_some(){sparse=false;break;}term=Some((j,b[d-j]));}
   }
   if sparse{
    #[cfg(sparse_audit)]SPARSE_AUDIT.with(|a|{let mut x=a.get();x[1]+=1;a.set(x)});
    let inverse=if b[d]==1{None}else{Some(self.inverse(b[d])?)};
    let mut reverse_q=Vec::with_capacity(n);
    for i in 0..n{
     let mut value=a[a.len()-1-i];
     if let Some((j,c))=term{if i>=j && reverse_q[i-j]!=0{value=(value+self.p-self.fm(c,reverse_q[i-j]))%self.p;}}
     if value!=0{if let Some(v)=inverse{value=self.fm(value,v);}}
     reverse_q.push(value);
    }
    reverse_q.reverse();return Ok(reverse_q);
   }
   #[cfg(sparse_audit)]SPARSE_AUDIT.with(|a|{let mut x=a.get();x[2]+=1;a.set(x)});
  }
'''
  assert marker in t;t=t.replace(marker,insert+marker,1)
  t=t.replace('/// obtained by Newton reversal;', '/// obtained by the shared quotient dispatcher;')
  t+='\n#[cfg(sparse_audit)]std::thread_local!{static SPARSE_AUDIT:std::cell::Cell<[usize;3]>=const{std::cell::Cell::new([0;3])};}\n#[cfg(sparse_audit)]include!("sparse-checks.rs");\n'
 if f.name=='quotient-checks.rs':
  old='assert_eq!(audit(),before);fallback+=1;'
  new='if b.iter().rev().skip(1).take(n-1).filter(|&&x|x!=0).count()>1{assert_eq!(audit(),before);}else{assert_eq!(audit()[1],before[1]);}fallback+=1;'
  assert old in t;t=t.replace(old,new).replace('unchanged_large_quotient_cases={fallback}','wider_quotient_cases={fallback} dense_fallback_counts_preserved=true')
 if f.name=='check.rs':t=t.replace('fn main(){polynomial::quotient_checks();','fn main(){polynomial::sparse_checks();polynomial::quotient_checks();')
 if f.name=='validate.py':t=t.replace("'--cfg','quotient_audit'","'--cfg','quotient_audit','--cfg','sparse_audit'")
 (p/f.name).write_text(t);outs.append(f.name)
(p/'generation.json').write_text(json.dumps({'input_hashes':pins,'output_hashes':{n:hashlib.sha256((p/n).read_bytes()).hexdigest() for n in outs}},indent=2)+'\n')
