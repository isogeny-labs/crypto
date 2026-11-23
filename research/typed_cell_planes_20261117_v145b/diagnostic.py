import time
START=time.monotonic()
import json
from pathlib import Path
from .common import P,finish
from .kernel import context,make_planes,plane_rows,prefix_code
from research.residue_quotient_20261116_v99b.kernel import apply
from research.precision_filtration_20261117_v132b.kernel import evaluate_family
from research.delayed_tangent_20261117_v133b.kernel import jet,bitpolys
from research.odd_cached_quadratic_20261116_v94b.kernel import Arithmetic
out={'mapped_states':0,'coefficient_checks':0,'row_checks':0,'old_mapping_counterexample':None,'all_registered_contexts_complete':False,'original_complete':False};error=None
try:
 data=json.loads((P/'DATA.json').read_bytes());cfg=json.loads(Path(data['cfg']).read_bytes());chart=json.loads(Path(data['cover']).read_bytes())['records'][1]['chart'];base,cols,images,selectors,Ds=context(cfg,chart,int(data['known_unit'],16));r=data['diagnostic_rank'];images=images[:r];selectors=selectors[:r];F,full=make_planes(base,images);trace=[];rows=plane_rows(F,Ds,full,trace=trace);f=Arithmetic(2,[0,1]);expected0=prefix_code(base)
 for code in range(1<<r):
  actual=evaluate_family(base,cols,apply(selectors,code));encoded=expected0^apply(images,code);assert prefix_code(actual)==encoded;out['mapped_states']+=1
  old_bad=expected0^apply(selectors,code)
  if old_bad!=encoded and out['old_mapping_counterexample']is None:out['old_mapping_counterexample']={'state':code,'selector_witness_hex':hex(apply(selectors,code)),'correct_coefficient_image_hex':hex(apply(images,code)),'actual_prefix_hex':hex(encoded),'old_misencoded_prefix_hex':hex(old_bad)}
  refs=[jet(bitpolys(actual),bitpolys(D),18,23,f)for D in Ds]
  for kind,k,lane,values in trace:
   expected=refs[0][k-1]['correction']if kind=='primal'else refs[lane][k-1]['tangent'];assert [(v>>code)&1 for v in values]==expected;out['coefficient_checks']+=len(values)
  for i in range(6):
   expected=sum(refs[j][17+i]['tangent'][1]<<j for j in range(7));assert sum(((rows[i][j]>>code)&1)<<j for j in range(7))==expected;out['row_checks']+=1
 assert out['old_mapping_counterexample']is not None and rows[0][0]==full;out['all_registered_contexts_complete']=True;out['total_seconds']=time.monotonic()-START
except Exception as exc:error=repr(exc)
if error:out['all_registered_contexts_complete']=False
finish(out,error,START,'All64typed early states and every traced primal/tangent coefficient versus original-selector scalar jets','If passed,execute the separately preregistered repaired24rankcensus with independent row/column elimination;otherwise preserve the exact counterexample and change the failed assumption.')
