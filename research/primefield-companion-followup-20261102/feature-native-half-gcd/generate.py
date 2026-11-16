from pathlib import Path
import hashlib,json
D=Path(__file__).resolve().parent;R=D.parent/'feature-native-resultant-class'
def once(s,a,b):
 assert s.count(a)==1,(a,s.count(a))
 return s.replace(a,b)
names=['lib.rs','source.rs','source-extra.rs','transport.rs','transport-extra.rs','tests.rs','subgroup.rs','subgroup-tests.rs','residual.rs','residual-tests.rs','polynomial.rs','product-tests.rs','euclid-tests.rs','class-tests.rs']
for n in names:
 s=(R/n).read_text()
 if n=='polynomial.rs':
  s=once(s,'impl Poly{',(D/'batch-types.rs').read_text()+'impl Poly{')
  a=s.index(' pub fn resultant_class(');b=s.index(' pub fn image_class(',a);s=s[:a]+(D/'half-method.rs').read_text()+s[b:]
  s+='\n#[cfg(test)]#[path="../feature-native-resultant-class/polynomial.rs"]mod reference_euclid;\n#[cfg(test)]mod half_tests{use super::*;include!("half-tests.rs");}\n'
 if n=='residual.rs':
  s=once(s,'fn reset_projection_count(){polynomial::reset_projections();}','fn reset_projection_count(){polynomial::reset_projections();polynomial::reset_batches();}')
  s+='\n#[cfg(test)]pub(crate) fn batch_count()->usize{polynomial::batches()}\n'
 if n=='residual-tests.rs':
  s=once(s,'let(mut valid,mut rejected)=(0,0);','let(mut valid,mut rejected,mut batches)=(0,0,0);')
  s=once(s,'assert_eq!(residual::projection_count(),usize::from(got.is_ok()));','assert_eq!(residual::projection_count(),usize::from(got.is_ok()));batches+=residual::batch_count();')
  s=once(s,'assert!(valid>0&&rejected>0);','assert!(valid>0&&rejected>0&&batches>0);')
  s=once(s,'one_projection_per_valid=true','one_projection_per_valid=true recursive_batches={batches}')
 (D/n).write_text(s)
h=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
(D/'generation.json').write_text(json.dumps({'inputs':{str(p):h(p)for p in [*(R/n for n in names),D/'batch-types.rs',D/'half-method.rs',D/'half-tests.rs']},'outputs':{n:h(D/n)for n in names}},indent=2)+'\n')
