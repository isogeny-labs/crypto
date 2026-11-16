"""Run the frozen 24-cell trace-22 quadratic-square screen locally."""
import concurrent.futures
import hashlib
import json
import subprocess
import time
from pathlib import Path

HERE=Path(__file__).resolve().parent
SCRIPT=HERE/'search_all.sage'
SOURCE_SHA=hashlib.sha256(SCRIPT.read_bytes()).hexdigest()
FMASKS=(32280,35349,36380,51009,67186,106121,
        112490,113379,127268,168265,175537,175771)
BMASKS=(68,84)
CELLS=tuple((f,b) for f in FMASKS for b in BMASKS)
assert len(CELLS)==24

def check_receipt(f,b):
    p=HERE/f'allcell_{f}_{b}.json'
    if not p.exists():return None
    d=json.loads(p.read_text())
    assert d['schema']=='ghs.trace22.degree8-quadratic-square-all/v1'
    assert d['source_sha256']==SOURCE_SHA and (d['fmask'],d['bmask'])==(f,b)
    assert d['counts']['systems']==243 and d['counts']['u_specializations']==59049
    assert d['counts']['exact_hits']==len(d['hits'])
    return d

def run_cell(cell):
    f,b=cell
    old=check_receipt(f,b)
    if old is not None:return {'fmask':f,'bmask':b,'status':'preexisting',
                               'hits':len(old['hits']),'cpu_seconds':old['cpu_seconds']}
    t0=time.perf_counter()
    p=subprocess.run(['sage',str(SCRIPT),str(f),str(b)],cwd=HERE,
                     capture_output=True,text=True,timeout=900)
    (HERE/f'allcell_{f}_{b}.stdout').write_text(p.stdout)
    (HERE/f'allcell_{f}_{b}.stderr').write_text(p.stderr)
    d=check_receipt(f,b) if p.returncode==0 else None
    return {'fmask':f,'bmask':b,'status':'complete' if d is not None else 'producer_failure',
            'returncode':p.returncode,'hits':None if d is None else len(d['hits']),
            'cpu_seconds':None if d is None else d['cpu_seconds'],
            'wall_seconds':time.perf_counter()-t0}

t0=time.perf_counter()
rows=[]
with concurrent.futures.ThreadPoolExecutor(max_workers=6) as pool:
    futures={pool.submit(run_cell,cell):cell for cell in CELLS}
    for future in concurrent.futures.as_completed(futures):
        cell=futures[future]
        try:row=future.result()
        except Exception as exc:row={'fmask':cell[0],'bmask':cell[1],
                                     'status':'producer_failure','error':repr(exc)}
        rows.append(row)
        print(json.dumps({'done':len(rows),'cell':cell,'status':row['status'],
                          'hits':row.get('hits'),'wall_seconds':time.perf_counter()-t0},
                         sort_keys=True),flush=True)

rows.sort(key=lambda r:(r['fmask'],r['bmask']))
out={'schema':'ghs.trace22.degree8-quadratic-all-manifest/v1',
     'source_sha256':SOURCE_SHA,
     'runner_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
     'cells':rows,'wall_seconds':time.perf_counter()-t0,
     'status':'complete' if all(r['status'] in ('complete','preexisting') for r in rows) else 'producer_failure'}
(HERE/'manifest.json').write_text(json.dumps(out,indent=2,sort_keys=True)+'\n')
print(json.dumps({'status':out['status'],'cells':len(rows),
                  'total_hits':sum(r.get('hits') or 0 for r in rows),
                  'wall_seconds':out['wall_seconds']},sort_keys=True),flush=True)
