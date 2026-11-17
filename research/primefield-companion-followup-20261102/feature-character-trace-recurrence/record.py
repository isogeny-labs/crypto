from pathlib import Path
import hashlib,json,re
p=Path(__file__).resolve().parent;root=p.parents[2];sha=lambda f:hashlib.sha256(f.read_bytes()).hexdigest()
copied=['lib.rs','geometry.rs','division.rs','polynomial.rs','schedule.rs','allocation.rs','class-source.rs','circuit.rs','transcript.rs','projector.rs','word.rs','corrected.rs']
for n in copied:assert sha(p/n)==sha(p.parent/'feature-word-endpoint-correction'/n)
for attempt in [1,2]:
 v=p/f'validation{attempt:02d}';binary=Path(f'/tmp/t1944-v{attempt}-check');deps=[(root/x).resolve() for x in Path(str(binary)+'.d').read_text().splitlines()[0].split(': ',1)[1].split()]
 assert not(v/'check.stderr').read_bytes();out=(v/'check.stdout').read_text();assert 'coefficients=7965' in out and 'public_cases=29 recovered=12 identity_endpoints=11 unresolved=0 capped=6' in out
 receipt={'scope':'Streaming lifted coefficient recurrence, selected character traces, bounded subset word moments and public recovery','build_exit':0,'check_exit':0,'commands':[f'timeout 90 rustc --edition=2021 -O -C overflow-checks=yes --emit=link,dep-info {p.relative_to(root)}/driver.rs -o {binary}',f'timeout 60 {binary}'],'binary_sha256':sha(binary),'dependencies':{str(f.relative_to(root)):sha(v/'checks.rs.snapshot' if attempt==1 and f.name=='checks.rs' else f) for f in deps},'copied_sources':{n:sha(p/n) for n in copied},'logs':{f.name:sha(f) for f in v.iterdir() if f.suffix in ['.stdout','.stderr']},'caps':{'steps':500000,'subsets':4096,'modulus':2**63-1,'circuit_nodes':100000,'endpoint_correction_multiplications':100000},'source_queries':{'nominal':332,'guards':27,'sum_excluding_setup':359},'original_cohort_modified':False,'counter_scope':'recurrence modular products include initial binary power, numerator and denominator products; unit inversions and subset polynomial products are separate; window is retained entries between steps'}
 if attempt==2:
  rows=[x for x in out.splitlines() if x.startswith('word_refusal')];assert len(rows)==9
  receipt['capped_word_work']={key:sum(int(re.search(r'\b'+key+r': (\d+)',row)[1]) for row in rows) for key in ['steps','products','inversions','precision_drops','subsets','polynomial_pairs']}
  receipt['capped_word_rows']=rows
 (v/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n');print(v.name,'copied',len(copied),'dependencies',len(deps))
