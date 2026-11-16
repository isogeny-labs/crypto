#!/usr/bin/env sage
"""Rank and solve the checkpointed s=7 twist relation matrix."""
import hashlib
import json
import time
from pathlib import Path

HERE=Path(__file__).resolve().parent
source=HERE/'relations_checkpoint.json'
d=json.loads(source.read_text())
assert d['schema']=='ghs.odd5.twist-s7-relations/v1'
n=int(d['factor_base_size'])
r=ZZ(d['r'])
rows=d['rows']
rhs=d['rhs']
assert len(rows)==len(rhs)==d['accepted']
F=GF(r)
t0=time.process_time_ns()
M=matrix(F,len(rows),n,implementation='flint')
for i,row in enumerate(rows):
    for j,c in row:
        M[i,j]+=F(c)
v=vector(F,rhs)
build_cpu_s=float((time.process_time_ns()-t0)/1e9)
print(json.dumps({'phase':'matrix_built','rows':len(rows),'columns':n,'build_cpu_s':build_cpu_s},default=int),flush=True)
t0=time.process_time_ns()
rank=int(M.rank())
rank_cpu_s=float((time.process_time_ns()-t0)/1e9)
print(json.dumps({'phase':'rank','rank':rank,'rank_cpu_s':rank_cpu_s},default=int),flush=True)
if rank<n-1:
    out={'schema':'ghs.odd5.twist-s7-relations-analysis/v1',
         'checkpoint_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),
         'rows':len(rows),'columns':n,'rank':rank,'status':'insufficient_rank',
         'build_cpu_s':build_cpu_s,'rank_cpu_s':rank_cpu_s}
else:
    t0=time.process_time_ns()
    logs=M.solve_right(v)
    solve_cpu_s=float((time.process_time_ns()-t0)/1e9)
    assert M*logs==v
    out={'schema':'ghs.odd5.twist-s7-relations-analysis/v1',
         'checkpoint_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),
         'rows':len(rows),'columns':n,'rank':rank,'status':'solved',
         'build_cpu_s':build_cpu_s,'rank_cpu_s':rank_cpu_s,
         'solve_cpu_s':solve_cpu_s,'logs':[int(x) for x in logs]}
    if rank==n-1:
        t0=time.process_time_ns()
        ker=M.right_kernel().basis()
        assert len(ker)==1 and M*ker[0]==0
        out['right_kernel']=[int(x) for x in ker[0]]
        out['kernel_cpu_s']=float((time.process_time_ns()-t0)/1e9)
(HERE/'relations_analysis.json').write_text(json.dumps(out,indent=2,sort_keys=True,default=int)+'\n')
print(json.dumps({k:v for k,v in out.items() if k not in ('logs','right_kernel')},sort_keys=True,default=int),flush=True)
