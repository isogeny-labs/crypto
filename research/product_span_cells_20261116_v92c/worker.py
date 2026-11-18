import json,time,sys,subprocess,os,select,statistics
from .common import P,finish
from .kernel import columns,certificate
from research.disjoint_separator_20261116_v92.kernel import Prefix,columns as fullcolumns
START=time.monotonic();out={'tiny_checks':0,'large_checks':0,'runs':[],'complete':False,'qualified_speedup':None};error=None
try:
 d=json.load(open(P/'DATA.json'))
 for n,mod,dim in d['tiny']:
  S=Prefix(n,mod,dim)
  for k in range(1,n-dim+2):
   for y in range(1<<n):
    for u0 in range(0,1<<n,1<<k):
     full,c=fullcolumns(S,y,u0,k,1);compact,cc=columns(S,y,u0,k,1);assert c==cc;A=S.prepare(full)[0];B=S.prepare(compact)[0];assert len(A)==len(B)and all(S.particular(v,A)is not None for v in compact)and all(S.particular(v,B)is not None for v in full);v=certificate(S,y,u0,k,1,'full');w=certificate(S,y,u0,k,1,'compressed');assert v==w;out['tiny_checks']+=1
 spec=d['actual'];S=Prefix(spec['n'],spec['mod'],spec['d']);proof=[]
 for k in d['dimensions']:
  for q in spec['queries']:
   u0=q['u']&~((1<<k)-1);a=certificate(S,q['y'],u0,k,q['B'],'full');b=certificate(S,q['y'],u0,k,q['B'],'compressed');assert a==b and a['rank']<=2*S.d+2*k-1
   if 'planted_x'in q:assert not a['empty']
   proof.append({'id':q['id'],'kind':q['kind'],'y':q['y'],'u0':u0,'k':k,'full_columns':(k+1)*S.d+k,'compressed_columns':2*S.d+2*k-1,**a});out['large_checks']+=1
 out['certificates']=proof
 for rep in range(3):
  for method in(['full','compressed']if rep%2==0 else['compressed','full']):
   assert time.monotonic()-START<65;name=f'cold_{rep}_{method}';cmd=[sys.executable,'-m','research.product_span_cells_20261116_v92c.child',str(P/'DATA.json'),method];start=time.monotonic_ns()
   with(P/(name+'.stdout')).open('xb')as stdout,(P/(name+'.stderr')).open('xb')as stderr:
    child=subprocess.Popen(cmd,stdout=stdout,stderr=stderr);fd=os.pidfd_open(child.pid)
    try:
     ready=select.select([fd],[],[],12)[0]
     if not ready:child.kill()
     pid,status,usage=os.wait4(child.pid,0);child.returncode=os.waitstatus_to_exitcode(status)
    finally:os.close(fd)
   end=time.monotonic_ns();v=json.loads((P/(name+'.stdout')).read_text());assert ready and child.returncode==0 and end-start<12e9 and v['complete']and len(v['cpu_affinity'])==1;expected=[{k:r[k]for k in['id','kind','k','u0','y','empty','rank','dual']}for r in proof];assert v['certificates']==expected
   r={'rep':rep,'method':method,'wall_seconds':(end-start)/1e9,'peak_rss_kib':usage.ru_maxrss,'user_seconds':usage.ru_utime,'system_seconds':usage.ru_stime,'parsed':v,'stderr':(P/(name+'.stderr')).read_text()}
   with(P/(name+'.json')).open('x')as w:json.dump(r,w,separators=(',',':'))
   out['runs'].append({k:r[k]for k in['rep','method','wall_seconds','peak_rss_kib','user_seconds','system_seconds']})
 out['medians']={m:statistics.median(r['wall_seconds']for r in out['runs']if r['method']==m)for m in['full','compressed']};out['complete']=True
except Exception as e:error=repr(e)
finish(out,error,START,'Productsubspacecompression preservescompletecertificate spans andcanonicalduals;boundedcoldcertificatecost compared.','Reuseu-independentmixedspan acrossdisjointcovercells,profileactualouterparametercoverandcomparestrongreverseenumeration;noglobalgainclaimfromcertificatecomponent.')
