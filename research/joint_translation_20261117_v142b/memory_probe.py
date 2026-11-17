import time
START=time.monotonic()
import json,resource,os,sys
from pathlib import Path
from .common import finish

def snapshot():
 fields={}
 for line in Path('/proc/self/status').read_text().splitlines():
  key,_,value=line.partition(':')
  if key in ['VmHWM','VmRSS','VmSize']:fields[key]=value.strip()
 fields['ru_maxrss_kib']=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss;return fields
before=snapshot();pages=bytearray(16*1024*1024)
for i in range(0,len(pages),4096):pages[i]=1
after=snapshot();out={'context':sys.argv[2],'before':before,'after':after,'allocation_bytes':len(pages),'page_checksum':sum(pages),'cpu_affinity':sorted(os.sched_getaffinity(0)),'address_limit':list(resource.getrlimit(resource.RLIMIT_AS)),'original_complete':False};assert out['page_checksum']==4096 and out['cpu_affinity']==[0];finish(out,None,START,'Same frozen memory probe under a declared launcher context','Compare root-ledger and fresh-ledger full-child RSS with post-exec VmHWM;do not infer algorithm memory gain from launcher differences.')
