import json,time,tracemalloc,resource
from .common import P,finish
from .kernel import trace_jump,trace_sequential
from research.semilinear_trace_20261116_v82.cases import fields,cases
START=time.monotonic()
class Count:
 def __init__(self,f):self.f=f;self.M=0;self.S=0
 def __getattr__(self,k):return getattr(self.f,k)
 def mul(self,a,b):self.M+=1;return self.f.mul(a,b)
 def sq(self,a):self.S+=1;return self.f.sq(a)
def main():
 out={'profiles':[],'qualified_speedup':None,'timing_scope':'single-worker component diagnostics; no A/A or exclusive isolation qualification'};error=None
 try:
  d=json.loads((P/'DATA.json').read_text())
  for f,name in fields(d):
   for m,kind,label,H,t in cases(f,d,name):
    if kind!='arbitrary'or label!='dense':continue
    for k in d['profile_lengths']:
     row={'field':name,'degree':m,'trace_length':k};results=[]
     for method in ['sequential','jump']:
      c=Count(f);stats={};tracemalloc.start();start=time.perf_counter_ns()
      v=trace_sequential(t,k,H,c)if method=='sequential'else trace_jump(t,k,H,c,stats=stats)
      elapsed=(time.perf_counter_ns()-start)/1e9;current,peak=tracemalloc.get_traced_memory();tracemalloc.stop();results.append(v);row[method]={'scalar_mul':c.M,'scalar_square':c.S,'seconds':elapsed,'python_peak_bytes':peak,'stats':stats}
     assert results[0]==results[1];out['profiles'].append(row)
     assert time.monotonic()-START<65
  out['complete']=True;out['worker_peak_rss_kib']=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
 except Exception as e:error=repr(e)
 return out,error
