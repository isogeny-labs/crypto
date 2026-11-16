from pathlib import Path
import hashlib,json
D=Path(__file__).resolve().parent
src=D.parent/'feature-native-standalone-source/lib.rs'
s=src.read_text().split('#[cfg(test)]#[path="../feature-native-source/lib.rs"]')[0]
s=s.replace('//!','//',1)
s=s.replace('pub struct Source {p:u64,','pub struct Source {base:[u64;2],p:u64,').replace('Self{p,a,b,r,part,','Self{base,p,a,b,r,part,')
# Isolate precisely the existing value and resultant computation for supplied geometry.
start=s.index('  let mut d=division::ValueDivision::new',s.index(' pub fn raw('))
end=s.index('\n /// Public point input only.',start)
body=s[start:end]
grid='  let grid=self.curve.grid(self.r,t,w,k,s,e).map_err(|_|Error::Geometry)?;\n'
assert body.count(grid)==1
body=body.replace(grid,'').replace('&grid.affine','affine')
s += '\nimpl Source {\n fn raw_on_grid(&self,t:[u64;2],affine:&[[u64;2]])->Result<u64,Error>{\n'+body+'}\n'
(D/'source.rs').write_text(s)
(D/'generation.json').write_text(json.dumps({'source_sha256':hashlib.sha256(src.read_bytes()).hexdigest(),'output_sha256':hashlib.sha256(s.encode()).hexdigest(),'transformation':'retain production source, bind original public base, duplicate raw arithmetic as raw_on_grid with only grid construction removed'},indent=2)+'\n')
