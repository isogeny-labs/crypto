from pathlib import Path
import importlib.util,json,hashlib
D=Path(__file__).resolve().parent
TRACE=D.parent/'feature-native-quotient-trace'
PREVIOUS=D.parent/'feature-native-resultant-protocol'
def load(name,path):
 spec=importlib.util.spec_from_file_location(name,path);m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);return m
base=load('quotient_previous_common',PREVIOUS/'common.py')
canonical=base.canonical
analyzer=base.analyzer
trace=load('quotient_trace_analyzer',TRACE/'analyze.py')
sha=base.sha
require=base.require
FILE_LIMIT=16*1024*1024

def profile(p):
 require([a['id']for a in p['arms']]==['trace','aa'],'wrong arms')
 require(p['sample_size']['instances']==617 and p['sample_size']['replicates']==1 and p['sample_size']['warmups']==0,'wrong sample size')
 x=p['execution'];q=x['placement'];require(x['backend']=='isolab-k8s' and x['timeout_seconds']==120 and x['retries']['max_attempts']==1 and x['checkpoint']['policy']=='none','execution envelope')
 require(q['cpus']==1 and q['memory_mib']==1024 and q['concurrency']==1 and q['max_total_usd']==1,'placement ceiling')
 require(p['extensions']['file_limit_bytes']==FILE_LIMIT,'output ceiling')

def protocol(path,expected):
 p=base.protocol(path,expected);profile(p);return p

def bindings(p,root):
 b=base.bindings(p,root)
 require(b['analysis']==(TRACE/'analyze.py').resolve(),'unexpected trace analyzer')
 for name in ['capture','export','common','pair']:
  require(b[name]==(D/(name+'.py')).resolve(),'unexpected adapter binding')
 return b

def pair(first,second):
 require(first.stat().st_size<=FILE_LIMIT and second.stat().st_size<=FILE_LIMIT,'oversize transcript')
 a=trace.analyze(first);b=trace.analyze(second)
 require(first.read_bytes()==second.read_bytes(),'A/A transcript mismatch')
 require(a==b,'A/A parsed mismatch');return a
