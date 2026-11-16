"""Replay recorded projection and old witnesses only; generate no new points."""
from pathlib import Path
import gzip, hashlib, json, sys, time
from rootfinder.binaryfield import BinaryField
from research.quartic_cell_20261114_v1 import worker as old
from research.regular_ore_20261115_v3 import worker as fixture_source

start=time.monotonic();root=Path(__file__).resolve().parents[2]
newpath=root/'research/gold_projection_20261115_v14/raw.json.gz'
new=json.loads(gzip.decompress(newpath.read_bytes()))['result']['actual']
baseline_path=root/'research/full_quintic_block_20261115_v9/raw.json.gz'
base=json.loads(gzip.decompress(baseline_path.read_bytes()))['out']['profile']['proofs']['baseline166']
f,A,B,z,y,h,_,_,vs=fixture_source.fixture()
H0=old.literal(f,A,B,z,y,h,0,base['particular']&((1<<83)-1),base['particular']>>83)
Hzero=old.literal(f,A,B,z,y,h,0,0,0)
columns=[old.padd(old.literal(f,A,B,z,y,h,0,v&((1<<83)-1),v>>83),Hzero) for v in base['basis']]
b=H0[4];g0=f.mul(b,H0[3])^H0[2];gs=[f.mul(b,H[3])^H[2] for H in columns]
# Third representation: literal reduced generators and83-bit residual rows,
# rather than the worker's four parity masks or four-output dynamic program.
G=old.basis(f.mul(f.square(vs[i]),vs[j])^f.mul(vs[i],f.square(vs[j]))
            for i in range(18) for j in range(i+1,18))
assert len(G)==48
rhs=old.residual(g0,G);projected=[old.residual(v,G) for v in gs]
rows=[(sum(((v>>bit)&1)<<i for i,v in enumerate(projected)),(rhs>>bit)&1)
      for bit in range(83)]
third=old.solve(rows,73);candidate=new['candidate'];K=old.basis(candidate['basis'])
assert third['rank']==candidate['rank']==4 and third['consistent']==candidate['consistent']
assert len(K)==len(third['basis'])==69
assert old.residual(third['particular']^candidate['particular'],K)==0
assert all(old.residual(v,K)==0 for v in third['basis'])
oldraw=root/'research/dual_obstruction_20261115_v6/raw.json.gz'
dual=json.loads(gzip.decompress(oldraw.read_bytes()))['out']['actual']
admitted=[]
for point in dual['points']:
    H=point[2]
    admitted.append(old.residual(f.mul(H[4],H[3])^H[2],G)==0)
cells=[]
for cell in dual['cells']:
    witness=cell['selected_table_indices']
    cells.append(dict(depth=cell['depth'],prefix=cell['prefix'],
                      old_points=len(cell['point_indices']),
                      gold_admitted_points=sum(admitted[i] for i in cell['point_indices']),
                      old_witness_size=len(witness),
                      witness_points_in_new_domain=sum(admitted[i] for i in witness),
                      old_witness_wholly_in_new_domain=all(admitted[i] for i in witness)))
assert time.monotonic()-start<5,'recorded replay limit'
report=dict(schema='recorded-gold-projection-replay/v1',verified=True,
            method='existing independent BinaryField/literalnorm code; explicit153Goldgenerators/83residualrows; recordedkernel/coset andoldwitnessmembership only',
            rank_increment=4,new_parameter_bits=69,old_unique_points=len(admitted),
            old_unique_gold_admitted=sum(admitted),cells=cells,
            no_new_points_or_root_search=True,external_reviewer=False,
            new_raw_sha256=hashlib.sha256(newpath.read_bytes()).hexdigest(),
            old_witness_raw_sha256=hashlib.sha256(oldraw.read_bytes()).hexdigest(),
            seconds=time.monotonic()-start,qualified_speedup=None)
with (Path(__file__).with_name('RECORDED_REPLAY.json')).open('x') as stream:json.dump(report,stream,indent=2)
print(json.dumps(report,sort_keys=True))
