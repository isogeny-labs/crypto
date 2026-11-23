from pathlib import Path
import json,hashlib
p=Path(__file__).resolve().parent;root=p.parents[2];sha=lambda f:hashlib.sha256(f.read_bytes()).hexdigest();v=p/'validation03'
rows=json.loads((p/'copied-sources.json').read_text())['files']
for row in rows:assert sha(p/row['file'])==row['sha256']==sha(p.parent/'feature-packed-sections'/row['file'])
deps=[(root/x).resolve() for x in Path('/tmp/t1932-v3-check.d').read_text().splitlines()[0].split(': ',1)[1].split()]
assert not(v/'check.stderr').read_bytes();assert 'image_membership_equalities=200048' in (v/'check.stdout').read_text();assert 'public_transcripts=277' in (v/'check.stdout').read_text()
receipt={'scope':'Exact implicit three-mode support and conditional phase; no unknown-image extraction or timing claim','build_exit':0,'check_exit':0,'limits_seconds':{'compile':90,'focused':60},'commands':['rustc --edition=2021 -O -C overflow-checks=yes --emit=link,dep-info research/primefield-companion-followup-20261102/feature-implicit-oriented-circuit/driver.rs -o /tmp/t1932-v3-check','/tmp/t1932-v3-check'],'binary_sha256':sha(Path('/tmp/t1932-v3-check')),'copied_sources':len(rows),'dependencies':{str(f.relative_to(root)):sha(f) for f in deps},'logs':{f.name:sha(f) for f in v.iterdir() if f.suffix in ['.stdout','.stderr']},'fixture_limits':{'node_cap':100000,'max_radii':4,'public_call_cap':'1+4*m'},'public_phase_requires_fixture_Y':True}
(v/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n');print('copied',len(rows),'dependencies',len(deps))
