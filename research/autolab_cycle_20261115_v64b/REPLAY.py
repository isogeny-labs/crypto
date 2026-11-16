from pathlib import Path
import sys,json,hashlib
R=Path(__file__).resolve().parents[2];P=Path(__file__).resolve().parent;sys.path.insert(0,str(R))
for n,h in json.loads((P/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256((R/n).read_bytes()).hexdigest()==h,n
full=json.loads((R/'research/full_constant_cubic_20261115_v64b/raw.json').read_text());cold=json.loads((R/'research/constant_cubic_cold_20261115_v64b/raw.json').read_text());assert full['error']is None and cold['error']is None
assert full['out']['complete_original_three_source']and full['out']['original_tuple_count_exact']==6
assert len(cold['out']['runs'])==8 and all(r['exact_root_count']==6 for r in cold['out']['runs'])
assert cold['out']['qualified_speedup']is None
if '--oracles'in sys.argv:
 from rootfinder.binaryfield import BinaryField
 from research.general_binary_cubic_repair_20261115_v60.oracles import family
 from research.norm_block_reuse_20261115_v36 import backend as linear
 from research.original_chain_lift_20261115_v55.worker import eq
 f=BinaryField(83,0x800000000200000000007);source=R/'research/constant_cubic_gray_20261115_v64b'
 for name in ['calibration_B1','calibration_ordinary']:
  v=json.loads((source/(name+'_instance.json')).read_text());raw=json.loads((source/(name+'_raw.json')).read_text())
  for rec in raw['native']['conditional_family_records']:
   ref=family(f,v,int(rec['b'],16));assert rec['consistent']==(ref is not None)
   if ref is not None:assert linear.same(ref,{'particular':int(rec['particular'],16),'basis':[int(x,16)for x in rec['kernel']]})
 v=json.loads((source/'full_ordinary_instance.json').read_text())
 for a,b,c,u in full['out']['original_tuples']:assert not eq(a,b,u,v['B'],f.modulus)and not eq(u,c,v['z'],v['B'],f.modulus)
print('Hashes, complete original roots,8cold runs and optional independent fiber replay verified. Ratio is own-reference only.')
