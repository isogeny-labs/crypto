"""Copy/source layout/hash manifests only. All new scientific logic is Rust/Lean."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,shutil,subprocess,os,difflib
r=Path('/workspace/crypto-chained-balanced-main');s=r/'research/chained_cubic_inverse_20261117';old=r/'research/chained_richelot_map_20261117/compiler-revision-F'
assert Path('/workspace/scratch/ct774-protocol15-exact-readback.txt').read_text().split()[0]=='8dfcbd23ec70b3aa8baa497fe35dd3dfa1e57e74'
assert not (s/'src').exists()
for name in ['src','vendor','reused-core']:shutil.copytree(old/name,s/name)
for name in ['Cargo.toml','Cargo.lock','fixtures-12.csv']:
 shutil.copy2(old/name,s/name)
 if name.startswith('Cargo.'):
  p=s/name;p.write_text(p.read_text().replace('chained-richelot-operational-map-14','chained-cubic-inverse-15'))
backup=s/'unchanged-baseline-inputs';backup.mkdir()
for name in ['nonsplit.rs','nonsplit_map.rs']:shutil.copy2(old/'src'/name,backup/name)
p=s/'src/nonsplit.rs';before=p.read_text()
text=before.replace('    q: FpPoly,\n','    q: FpPoly,\n    mul_calls15: std::cell::Cell<u64>,\n    nonzero_mul_calls15: std::cell::Cell<u64>,\n    inverse_calls15: std::cell::Cell<u64>,\n',1)
assert text!=before
text=text.replace('Ok(Self { p, q })','Ok(Self { p, q, mul_calls15: std::cell::Cell::new(0), nonzero_mul_calls15: std::cell::Cell::new(0), inverse_calls15: std::cell::Cell::new(0) })',1)
text=text.replace('    fn mul(&self, a: &E, b: &E) -> E {\n','    fn mul(&self, a: &E, b: &E) -> E {\n        self.mul_calls15.set(self.mul_calls15.get() + 1);\n        if !a.is_zero() && !b.is_zero() { self.nonzero_mul_calls15.set(self.nonzero_mul_calls15.get() + 1); }\n',1)
text=text.replace('    fn inv(&self, a: &E) -> E {\n','    fn inv(&self, a: &E) -> E {\n        self.inverse_calls15.set(self.inverse_calls15.get() + 1);\n',1);p.write_text(text)
p=s/'src/nonsplit_map.rs';before=p.read_text();start=before.index('    fn inv(&self, a: &Element) -> Element {');end=before.index('    fn pow(',start);inverse=before[start:end]
wrapper='''    fn inv(&self, a: &Element) -> Element {
        if self.cubic_first15 && a.b.is_zero() {
            assert!(!self.zero(a), "zero in compositum inverse");
            self.shortcut_calls15.set(self.shortcut_calls15.get() + 1);
            return self.lift3(&self.k.inv(&a.a));
        }
        self.general_calls15.set(self.general_calls15.get() + 1);
        self.inv_original15(a)
    }
'''
text=before[:start]+wrapper+inverse.replace('fn inv(', 'fn inv_original15(',1)+before[end:]
text=text.replace('    nonresidue: E,\n','    nonresidue: E,\n    cubic_first15: bool,\n    shortcut_calls15: std::cell::Cell<u64>,\n    general_calls15: std::cell::Cell<u64>,\n',1)
text=text.replace('Self { k, nonresidue }','Self { k, nonresidue, cubic_first15: false, shortcut_calls15: std::cell::Cell::new(0), general_calls15: std::cell::Cell::new(0) }',1)
marker='type Result<T> = std::result::Result<T, String>;';assert text.count(marker)==1;text=text.replace(marker,marker+'\n#[path = "cubic_inverse15.rs"]\nmod cubic_inverse15;',1)
marker='mod tests_14 {\n';assert text.count(marker)==1;text=text.replace(marker,marker+'    #[test]\n    fn frozen_cubic_inverse_15() { super::cubic_inverse15::run().unwrap(); }\n',1);p.write_text(text)
shutil.copy2(s/'new-code/cubic_inverse15.rs',s/'src/cubic_inverse15.rs')
lib=s/'src/lib.rs';lib.write_text(lib.read_text().replace('Frozen operational joint-chart divisor map14; no new field/group engine.','Frozen cubic-first inverse15; retained field/group arithmetic plus guarded inverse.'))
E=dict(os.environ,PATH='/home/agent/.cargo/bin:'+os.environ['PATH'])
fmt=subprocess.run(['rustup','run','1.99.1','rustfmt','--edition','2021',str(lib)],env=E,stdout=subprocess.PIPE,stderr=subprocess.PIPE);assert fmt.returncode==0,(fmt.stdout,fmt.stderr)
guard=subprocess.run(['rustup','run','1.99.1','rustfmt','--check','--edition','2021',str(lib)],env=E,stdout=subprocess.PIPE,stderr=subprocess.PIPE);assert guard.returncode==0,(guard.stdout,guard.stderr)
(s/'new-code/cubic_inverse15.rs').write_bytes((s/'src/cubic_inverse15.rs').read_bytes())
diff=''
for name in ['nonsplit.rs','nonsplit_map.rs']:
 diff+=''.join(difflib.unified_diff((old/'src'/name).read_text().splitlines(True),(s/'src'/name).read_text().splitlines(True),fromfile='original14F/'+name,tofile='unit15/'+name))
(s/'ACTUAL_EXPLICIT_NATIVE_CHANGES.diff').write_text(diff)
for name in ['balanced.rs','closed_fibres.rs','shifted.rs']:assert (s/'src'/name).read_bytes()==(old/'src'/name).read_bytes()
for name in ['fp2.rs','fp_poly.rs']:assert (s/'vendor'/name).read_bytes()==(old/'vendor'/name).read_bytes()
data={'time':datetime.now(timezone.utc).isoformat(),'prospective_public_protocol':'8dfcbd23ec70b3aa8baa497fe35dd3dfa1e57e74','prospective_claim':'PCC-20261117-b0ba92','old_F_source':'f35867831245560527428e3562ab55d74eefb5a0','original_compositum_inverse_body_preserved_with_only_method_rename':True,'changes':['CubicField counters only; arithmetic expressions unchanged','Field guarded policy plus exact retained original inverse','new Rust paired unit and exact filter registration','Cargo package/doc label only'],'all_other_field_polynomial_group_closed_and_chart_methods_unchanged':True,'no_new_lint_allowance_or_resource_change':True,'format_exit':guard.returncode,'scientific_compile_calls':0,'scientific_arithmetic_calls':0,'formal_compile_calls':0}
(s/'ACTUAL_NATIVE_SOURCE_PREPARATION_15.json').write_text(json.dumps(data,indent=2)+'\n');print(json.dumps(data,indent=2))
