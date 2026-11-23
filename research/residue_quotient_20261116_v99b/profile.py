import json,time,hashlib
from .common import P,finish
from .problem import build
from .kernel import count
START=time.monotonic();out={};error=None
try:
 E,eb,widths,tables,F,fb,last,details=build(P);results={}
 for method in['dp','walsh']:
  t=time.monotonic();v=count(E,eb,widths,tables,F,fb,last,method);results[method]={'seconds':time.monotonic()-t,'result':v}
 assert results['dp']['result']['distribution']==results['walsh']['result']['distribution']and results['dp']['result']['exact_necessary_parameter_count']==results['walsh']['result']['exact_necessary_parameter_count'];out={'tables':details,'methods':results,'original_complete':False,'original_exact_root_count':None,'scope':'Exact first8necessary residue/lift parameter count;not fullsupport/originalroots','qualified_speedup':None}
except Exception as e:error=repr(e)
finish(out,error,START,'Exact quotient count ofall first8residue-compatible liftedparameters,retainingglobalqcoupling.','Addresiduesorfullsupportwithcompleteprovenance;candidatecountisnot originalall-roots completion.')
