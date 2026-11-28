"""Record completed correctness qualification; no research arithmetic."""
import hashlib,json
from pathlib import Path
p=Path(__file__).resolve().parent;old=p.parent/'feature-native-signed-image';sha=lambda f:hashlib.sha256(f.read_bytes()).hexdigest()
records=json.loads((p/'generation.json').read_text())['copied']
for row in records:assert sha(p/row['source'])==row['sha256']==sha(old/row['source'])
v=p/'validation03';deps=[Path(x).resolve() for x in Path('/tmp/t1905-v3-check.d').read_text().splitlines()[0].split(': ',1)[1].split()]
ref=p.parent/'feature-native-planned-width/validation01/check.stdout';assert(v/'inherited.stdout').read_bytes()==ref.read_bytes();assert not(v/'check.stderr').read_bytes()
assert 'powered_phase_calls=700 fallback_phase_calls=350' in(v/'check.stdout').read_text()
receipt={'scope':'Powered canonical state, public recovery and capacity fallback correctness; no timing panel',
 'build_exit':0,'check_exit':0,'inherited_exit':0,'compile_limit_seconds':90,'check_limit_seconds':60,
 'commands':['rustc --edition=2021 -O -C overflow-checks=yes --emit=link,dep-info driver.rs -o /tmp/t1905-v3-check','/tmp/t1905-v3-check','/tmp/t1905-v3-check --inherited'],
 'binary_sha256':sha(Path('/tmp/t1905-v3-check')),'source_hashes':{str(f):sha(f) for f in deps},'copied_sources':len(records),
 'logs':{f.name:sha(f) for f in v.iterdir() if f.suffix in ['.stdout','.stderr']},'inherited_byte_equal':True,'reference_sha256':sha(ref),
 'lean_exit':0,'lean_command':'lean Product.lean','lean_log':'../validation01/lean.stdout','lean_log_sha256':sha(p/'validation01/lean.stdout'),
 'lean_source_sha256':sha(p/'Product.lean'),'lean_version':'4.34.1','lean_commit':'5045d0056413266e57c625dcd7c365b10e377c52'}
(v/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
for n in [1,2]:
 v=p/f'validation0{n}';(v/'receipt.json').write_text(json.dumps({'scope':'Intermediate branch integration' if n==1 else 'Terminal phase added before coverage assertions','build_exit':0,'check_exit':0,'binary_sha256':sha(Path(f'/tmp/t1905-v{n}-check')),'snapshots':{f.name:sha(f) for f in v.glob('*.snapshot')},'stdout_sha256':sha(v/'check.stdout')},indent=2)+'\n')
print('copied_sources',len(records),'compiler_dependencies',len(deps))
