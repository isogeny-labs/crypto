"""In-job capture only, not a cloud submitter or an authorization service."""
from pathlib import Path
import argparse,json,subprocess,time,datetime,os,math,platform
import sys
sys.path.insert(0,str(Path(__file__).resolve().parent.parent/"feature-native-resultant-protocol"))
from common import protocol,bindings,sha,require,canonical,analyzer

def host_facts():
 # Best-effort metadata must never prevent a failure receipt. No subprocesses.
 host={'cpu_model':None,'logical_cpus':None,'memory_bytes':None,'kernel':None,'hypervisor':None,'isolation_tier_max':None}
 try:
  host['logical_cpus']=os.cpu_count();host['kernel']=os.uname().release
  host['memory_bytes']=os.sysconf('SC_PAGE_SIZE')*os.sysconf('SC_PHYS_PAGES')
  for line in Path('/proc/cpuinfo').read_text().splitlines():
   if line.startswith('model name'):
    host['cpu_model']=line.split(':',1)[1].strip();break
 except (OSError,ValueError,IndexError):pass
 return host

def admission(p,a,expected,now):
 require(a.get('admitted')is True,'not admitted')
 require(a.get('protocol_sha256')==expected,'admission protocol mismatch')
 require(a.get('source_commit')==p['arms'][0]['instrument']['commit'],'admission source mismatch')
 require(type(a.get('expires_unix'))in [int,float] and math.isfinite(a['expires_unix']) and now<a['expires_unix'],'expired admission')
 require(type(a.get('charged_and_reserved_remainder'))in [int,float] and math.isfinite(a['charged_and_reserved_remainder']) and a['charged_and_reserved_remainder']>=1,'remaining funds unknown/insufficient')
 require(type(a.get('reserved_usd'))in [int,float] and a['reserved_usd']==1 and type(a.get('max_attempts'))is int and a['max_attempts']==1,'reservation/attempt limit')
 require(all(type(a.get(k))is int for k in ['cpus','memory_mib','concurrency']) and a['cpus']==1 and a['memory_mib']==1024 and a['concurrency']==1,'resource envelope')
 require(a.get('role')=='job-submitter' and a.get('namespace')=='isolab','role/namespace')
 require(a.get('quota_readback_verified')is True and a.get('publication_verified')is True,'missing external readback/publication verification')
 for k in ['job_uid','pod_uid','image_digest','budget_evidence_sha256','capacity_evidence_sha256','publication_evidence_sha256']:
  require(isinstance(a.get(k),str)and bool(a[k]),'missing external evidence '+k)
 for k in ['budget_evidence_sha256','capacity_evidence_sha256','publication_evidence_sha256']:
  require(len(a[k])==64 and all(c in '0123456789abcdef'for c in a[k]),'invalid evidence digest')
 require(now+120<=a['expires_unix'],'insufficient admitted deadline')

def capture(p,pp,binary,a,out,execute):
 ph=p['freeze']['protocol_sha256'];b=bindings(p,pp.parent);admission(p,a,ph,time.time())
 require(sha(binary)==p['arms'][0]['instrument']['binary_sha256'],'binary changed')
 require(all(arm['instrument']['binary_sha256']==sha(binary)for arm in p['arms']),'arm binary mismatch')
 require(Path(a.get('attempt_directory','')).is_absolute() and Path(a['attempt_directory']).resolve()==out.resolve(),'attempt directory differs from admission')
 if not execute:return {'checked':True,'executed':False}
 # Caller must supply authentic operator evidence. No credentials, IAM or cluster mutation here.
 out.mkdir(parents=True,exist_ok=False)
 rows=out/'rows.jsonl';stderr=out/'stderr.log';stdout=out/'stdout.log';started=canonical.now_iso()
 command=[str(binary),'--panel',str(b['inputs']),ph,sha(b['inputs']),str(rows)]
 code=None;status='infra_error';error=None
 try:
  with stdout.open('xb')as so,stderr.open('xb')as se:
   remaining=min(120,a['expires_unix']-time.time())
   if remaining<=0:code=124;status='timeout';error='admission expired before child start'
   else:
    try:code=subprocess.run(command,stdout=so,stderr=se,timeout=remaining).returncode;status='succeeded'if code==0 else'failed'
    except subprocess.TimeoutExpired:code=124;status='timeout'
 except OSError as e:error=str(e)
 receipt={'schema':'primefield-resultant-capture/v1','protocol_sha256':ph,'input_sha256':sha(b['inputs']),'metrics_sha256':sha(b['metrics']),'binary_sha256':sha(binary),'source_commit':p['arms'][0]['instrument']['commit'],'exit_code':code,'status':status,'started_at':started,'finished_at':canonical.now_iso(),'command':command,'raw_sha256':sha(rows)if rows.exists()else None,'stdout_sha256':sha(stdout)if stdout.exists()else None,'stderr_sha256':sha(stderr)if stderr.exists()else None,'job_uid':a['job_uid'],'pod_uid':a['pod_uid'],'image_digest':a['image_digest'],'admission':a,'error':error,'attempts':1,'events_sha256':sha(Path(str(rows)+'.events'))if Path(str(rows)+'.events').exists()else None,'host':host_facts()}
 canonical.write_once(out/'receipt.json',receipt);return receipt

def main():
 ap=argparse.ArgumentParser()
 for k in ['protocol','expected-protocol','binary','admission','out']:ap.add_argument('--'+k,required=True)
 ap.add_argument('--execute',action='store_true');a=ap.parse_args();pp=Path(a.protocol).resolve();p=protocol(pp,a.expected_protocol)
 admitted=analyzer.read_json(Path(a.admission).read_text());result=capture(p,pp,Path(a.binary).resolve(),admitted,Path(a.out).resolve(),a.execute);print(json.dumps(result))
 if a.execute and result.get('exit_code')!=0:raise SystemExit(1)
if __name__=='__main__':main()
