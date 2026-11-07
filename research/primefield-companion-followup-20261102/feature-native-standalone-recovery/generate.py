from pathlib import Path
import hashlib,json
D=Path(__file__).resolve().parent;src=D.parent/'feature-native-monomial-recovery/lib.rs';text=src.read_text();prefix=text.split('pub struct Source{',1)[0];assert prefix.count('mod source;')==1
prefix=prefix.replace('mod source;\n','')
extra='''
impl Program {pub(crate) fn shape(&self)->(u64,usize,usize){(self.r,self.q,self.max_extra)}}
#[cfg(test)]impl Program {pub(crate) fn corrupt_for_test(&mut self){self.decoder=decoder::Program::new(self.r,self.q,&[(1,vec![])]).unwrap();self.corrections.clear();}}
'''
(D/'program.rs').write_text(prefix+extra);(D/'generation.json').write_text(json.dumps({'source':'../feature-native-monomial-recovery/lib.rs','source_sha256':hashlib.sha256(src.read_bytes()).hexdigest(),'output_sha256':hashlib.sha256((prefix+extra).encode()).hexdigest(),'transformation':'retain prefix before Source; remove mod source; append shape accessor and cfg(test) corruption control; original decoder arithmetic unchanged'},indent=2)+'\n')
