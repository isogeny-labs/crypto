"""Record already completed deterministic qualification; performs no research arithmetic."""
import hashlib,json
from pathlib import Path
p=Path(__file__).resolve().parent
sha=lambda f:hashlib.sha256(f.read_bytes()).hexdigest()
write=lambda f,x:f.write_text(json.dumps(x,indent=2)+'\n')
records=[]
for f in sorted((p.parent/'feature-native-triple-modes').glob('*.rs')):
    g=p/f.name
    expected=f.read_text()
    if f.name=='triple-checks.rs':expected=expected.replace('fn main()','fn signed_predecessor_triple_main()')
    assert g.read_text()==expected,f.name
    records.append(dict(source=str(f.relative_to(p.parent)),output=g.name,source_sha256=sha(f),output_sha256=sha(g)))
write(p/'generation.json',dict(source_commit='f534da3bdc6b7f7b253fe9535151913af1a59d02',records=records,change='Rename triple checker main; driver appends signed checks.'))
line=Path('/tmp/t1896-check.d').read_text().splitlines()[0]
deps=[Path(x).resolve() for x in line.split(': ',1)[1].split()]
for f in deps:assert f.is_file(),f
v=p/'validation03'
ref=p.parent/'feature-native-planned-width/validation01/check.stdout'
assert (v/'inherited.stdout').read_bytes()==ref.read_bytes()
assert not (v/'check.stderr').read_bytes()
assert 'SignedReciprocal.norm_identity' in (v/'lean.stdout').read_text()
write(v/'receipt.json',dict(scope='Deterministic signed-mode observations, norm quotient and support controls; no timed panel',
 build_exit=0,focused_exit=0,inherited_exit=0,lean_exit=0,compile_timeout_seconds=90,check_timeout_seconds=60,lean_timeout_seconds=30,
 binary_sha256=sha(Path('/tmp/t1896-check')),source_hashes={str(f):sha(f) for f in deps},
 logs={f.name:sha(f) for f in v.iterdir() if f.suffix in ['.stdout','.stderr']},inherited_byte_equal=True,reference_sha256=sha(ref),
 copied_source_records=len(records),lean_version='4.34.1',lean_commit='5045d0056413266e57c625dcd7c365b10e377c52',lean_source_sha256=sha(p/'Norm.lean'),
 commands=['rustc --edition=2021 -O -C overflow-checks=yes --emit=link,dep-info driver.rs -o /tmp/t1896-check','/tmp/t1896-check','/tmp/t1896-check --inherited','lean Norm.lean']))
for n in [1,2]:write(p/f'validation0{n}'/'receipt.json',dict(build_exit=1,reason='Duplicate main' if n==1 else 'Duplicate predecessor_main',compiler_log_sha256=sha(p/f'validation0{n}'/'build.stderr')))
write(p/'report-inspection.json',dict(pages=1,inspected=True,pdf_sha256=sha(p/'report.pdf'),notes='Diagram and text readable; scalar inputs confined to fixtures.'))
print('copied_sources',len(records),'compiler_dependencies',len(deps))
