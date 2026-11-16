import tempfile
from pathlib import Path
from analyze import analyze,FIXTURES
lines=[]
for p,r in FIXTURES:
 for k in range(r):lines.extend([f'target,{p},{r},{k},1,1,0',f'quotient,{p},{r},{k},0,4,4,2,3,3,4,0,true,{p}'])
with tempfile.TemporaryDirectory() as d:
 p=Path(d)/'synthetic.csv';p.write_text('\n'.join(lines));a=analyze(p);assert len(a['targets'])==617;assert sum(a['histogram_p_n_d_support_weight_route'].values())==617
 mutations=[lines[:-1],lines+lines[:1],[lines[0].replace(',1,1,0',',1,1,1')]+lines[1:],[lines[0],lines[1].replace(',true,',',false,')]+lines[2:],[lines[0],lines[1].replace(',3,4,0,true',',3,99,0,true')]+lines[2:],[lines[0],lines[1].replace(',0,4,4,',',1,4,4,')]+lines[2:]]
 for rows in mutations:
  p.write_text('\n'.join(rows))
  try:analyze(p)
  except ValueError:pass
  else:raise AssertionError('malformed panel accepted')
print('synthetic_parser_targets=617 rejected_mutations=6 workload_measurement=false')
