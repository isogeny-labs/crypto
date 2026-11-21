from pathlib import Path
import json,hashlib
p=Path(__file__).resolve().parent; root=p.parents[2]
sha=lambda f:hashlib.sha256(f.read_bytes()).hexdigest()
copied=['lib.rs','geometry.rs','division.rs','polynomial.rs','schedule.rs','allocation.rs','class-source.rs','circuit.rs','transcript.rs','projector.rs']
for n in copied: assert sha(p/n)==sha(p.parent/'feature-selected-moments'/n)
for v in [1,2]:
 d=p/f'validation{v:02d}'; binary=Path(f'/tmp/t1940-v{v}-check')
 deps=[(root/x).resolve() for x in Path(str(binary)+'.d').read_text().splitlines()[0].split(': ',1)[1].split()]
 hashes={str(f.relative_to(root)):sha(d/'checks.rs.snapshot' if v==1 and f.name=='checks.rs' else f) for f in deps}
 assert not(d/'check.stderr').read_bytes()
 out=(d/'check.stdout').read_text();assert 'public_cases=29 recovered=18' in out and 'large_refusal=Some("terms")' in out
 receipt={'scope':'Exact word projector, all-mode phase and bounded public recovery; deterministic software checks','build_exit':0,'check_exit':0,'limits_seconds':{'compile':90,'check':60},'commands':[f'timeout 90 rustc --edition=2021 -O -C overflow-checks=yes --emit=link,dep-info {p.relative_to(root)}/driver.rs -o {binary}',f'timeout 60 {binary}'],'binary_sha256':sha(binary),'dependencies':hashes,'copied_sources':{n:sha(p/n) for n in copied},'logs':{f.name:sha(f) for f in d.iterdir() if f.suffix in ['.stdout','.stderr']},'source_snapshot':'checks.rs.snapshot' if v==1 else None,'small_caps':{'nodes':100000,'terms':128,'stored':500000,'pairs':2000000},'large_caps':{'nodes':100000,'terms':64,'stored':100000,'pairs':2000000},'public_queries':{'nominal':332,'guards':27,'total_excluding_setup':359},'original_cohort_modified':False}
 (d/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
 print(d.name,'dependencies',len(deps),'copied',len(copied))
