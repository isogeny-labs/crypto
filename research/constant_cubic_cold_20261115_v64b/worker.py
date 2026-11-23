from pathlib import Path
import json,hashlib,os,resource,statistics,sys,time
from research.quadratic_baseline_pilot_20261115_v63.worker import invoke
P=Path(__file__).resolve().parent;START=time.monotonic()
def main():
 import research.quadratic_baseline_pilot_20261115_v63.worker as proc
 proc.P=P
 out={'runs':[],'reference_speed_ratio':None,'qualified_speedup':None};error=None
 try:
  data=json.loads((P/'DATA.json').read_text());expected=set(map(tuple,json.loads(Path(data['expected_receipt']).read_text())['out']['original_tuples']));env=os.environ.copy();env.update(OMP_NUM_THREADS='1',OPENBLAS_NUM_THREADS='1',RAYON_NUM_THREADS='1');times={'reference':[],'constant':[]}
  for rep in range(4):
   for backend in(['reference','constant']if rep%2==0 else['constant','reference']):
    name=backend+'_'+str(rep);binary=data[backend];command=[sys.executable,'-m','research.constant_cubic_cold_20261115_v64b.child',data['descriptor'],binary,backend];run=invoke(command,env,name);parsed=json.loads(run['stdout']);complete=parsed['complete_original_three_source']is True …1054 tokens truncated…ANF coefficients exactly by subset-XOR. A zeta transform converts RHS ANF to the full truth table;this is precomputation charged in the scan. Gray steps change high columns by a single constant delta and lowGold rows by single plus active pair coefficients. No field inversion or per-h field multiplication is needed in fiber setup. The unchanged final support criterion retains repeated roots by nilpotence and the unchanged original tuple replay checks every accepted norm.

The implementation is GF83 only,nonzero target,outer dimension<=20. It refuses larger precomputation before allocation. Source domain d and all rank/nullity/candidate/accepted/time caps remain explicit. At d18 the constant rank is31;remaining coordinate count52;setup stores an8MiB RHS table plus bounded matrices. Search still visits all262144outer parameters;this is not a global quintic complexity improvement or a many-field release claim. Timings must include precompute,support,factor/lift,startup andserialization. The independent C60Python matrix oracle uses the original constraints without this elimination or Gray table.
