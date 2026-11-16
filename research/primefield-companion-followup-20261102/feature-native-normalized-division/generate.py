from pathlib import Path
import json,hashlib
p=Path(__file__).resolve().parent;src=p.parent/'feature-native-dyadic-classes';base=p.parent/'feature-native-value-division/lib.rs'
s=base.read_text().split('#[cfg(test)]',1)[0]
a=s.index(' fn inv(');b=s.index('\n}\nimpl Add',a);s=s[:a]+s[b:]
s=s.replace('inverse_two_y:F,cache:', 'two_y:F,two_y_fourth:F,cache:')
s=s.replace('let w4=c(4)*y*(', 'let w4=c(2)*(')
s=s.replace('inverse_two_y:two_y.inv().ok_or(Error::Nonunit)?,','two_y,two_y_fourth:two_y.sq().sq(),')
s=s.replace('(2,two_y)', '(2,c(1))')
s=s.replace('a*b.cube()-c*d.cube() // Six field multiplications after dependencies.', 'let left=a*b.cube();let right=c*d.cube();\n   if t&1==0{self.two_y_fourth*left-right}else{left-self.two_y_fourth*right} // Seven multiplications.')
s=s.replace('(a*self.inverse_two_y)*(b*c.sq()-d*e.sq()) // Six after dependencies.', 'a*(b*c.sq()-d*e.sq()) // Five multiplications; no division.')
s=s.replace('pub fn value(&mut self,n:u64)->u64{self.field_value(n).v}', 'pub fn value(&mut self,n:u64)->u64{let v=self.field_value(n);if n&1==0{(v*self.two_y).v}else{v.v}}')
s=s.replace('//! Value-only division polynomials for source paths with a supplied seed.', '//! Parity-normalized division polynomials: inversion-free value-only cache.')
(p/'division.rs').write_text(s)
pins={str(base):hashlib.sha256(base.read_bytes()).hexdigest()};outs=['division.rs']
for name in ['lib.rs','polynomial.rs','class-source.rs','class-checks.rs','check.rs','generate-recovery.py','validate.py','validate-recovery.py','new-recovery.rs']:
 f=src/name;s=f.read_text();pins[str(f)]=hashlib.sha256(f.read_bytes()).hexdigest()
 if name=='lib.rs':s=s.replace('#[path="../feature-native-value-division/lib.rs"] mod division;', 'mod division;')
 if name=='check.rs':s=s.replace('fn main(){class_checks();','include!("division-checks.rs");\nfn main(){division_checks();class_checks();')
 (p/name).write_text(s);outs.append(name)
(p/'generation.json').write_text(json.dumps({'input_hashes':pins,'output_hashes':{n:hashlib.sha256((p/n).read_bytes()).hexdigest() for n in outs}},indent=2)+'\n')
