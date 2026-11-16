"""Common-X cubic bridge, independent tiny norms then opposite-sign actual scan."""
from pathlib import Path
import hashlib,itertools,json,resource,subprocess,sys,time
from rootfinder.binaryfield import BinaryField
from research.inversion_free_oracles_20261115_v35 import worker as old
from research.inversion_free_norm_20261115_v34 import compiler as c
from research.k0_reciprocal_cells_20261115_v38.solver import coefficients
P=Path(__file__).resolve().parent;START=time.monotonic()
def cubic(f,A,B,z,h,r,s):
 F=[s,r,1];N=old.plus(old.plus(old.pmul(f,F,F),old.pmul(f,[0,h],F)),[f.mul(B,f.square(h)),0,f.mul(A,f.square(h)),f.square(h)])
 J=[0]*4;J[3]=N[4]
 for i in range(3,0,-1):J[i-1]=N[i]^f.mul(z,J[i])
 assert N[0]==f.mul(z,J[0]);return J

def controls():
 states=positive=zero_h=0;profiles=[]
 for n,m in[(3,11),(4,19),(5,37)]:
  f=old.Small(n,m);A=0;z,y,B=next((z,y,f.square(y)^f.mul(z,y)^f.mul(f.square(z),z))for z in range(1,f.order)for y in range(f.order)if old.trace(f,z)==0 and(f.square(y)^f.mul(z,y)^f.mul(f.square(z),z)))
  D=range(4);C={tuple(coefficients(f,x))for x in itertools.combinations_with_replacement(D,3)};Hset={tuple(coefficients(f,x))for x in itertools.combinations_with_replacement(D,5)}
  for ctx in old.context(f,A,z,y):
   count=0
   for h,u in itertools.product(range(f.order),repeat=2):
    r,s=c.to_original(f,ctx,h,u,0);H=c.norm_coefficients(f,ctx,h,u,0);J=cubic(f,A,B,z,h,r,s)
    assert H==[0,0]+J and H==old.literal(f,A,B,z,y,h,r,s)
    assert J==cubic(f,A,B,z,h,r^h,s)
    assert f.square(z)^f.mul(z,r)^s^f.mul(h,y)==0
    assert f.square(z)^f.mul(z,r^h)^s^f.mul(h,y^z)==0
    assert(tuple(H)in Hset)==(tuple(J)in C);positive+=tuple(J)in C;states+=1;zero_h+=h==0;count+=1
   profiles.append(dict(n=n,modulus=m,A=A,B=B,z=z,y=y,c=ctx['c'],states=count))
 return dict(complete_parameter_states=states,positive_supported_norms=positive,h0_states=zero_h,profiles=profiles)
def actual():
 data=json.loads((P/'DATA.json').read_text());f=BinaryField(83,data['modulus']);z=data['z'];Qy=data['Qy'];minus=Qy^z;assert f.square(Qy)^f.mul(z,Qy)==f.mul(f.square(z),z)^1;assert f.square(minus)^f.mul(z,minus)==f.mul(f.square(z),z)^1
 # Native CLI internally uses yt=y+z. Opposite CLI ordinate verifies the conjugate cubic family.
 cmd=[str(P/'triple_scan'),'--z',hex(z),'--y',hex(minus),'--dimension','18','--max-outer','262144','--max-candidates','16777216','--max-nullity','12','--max-accepted','1024','--max-seconds','55'];t=time.monotonic();run=subprocess.run(cmd,capture_output=True,text=True,timeout=60);wall=time.monotonic()-t;report=json.loads(run.stdout)
 result=dict(complete=run.returncode==0 and report.get('complete')is True,native=report,child_wall_seconds=wall,exit_code=run.returncode,stderr=run.stderr,native_stdout_sha256=hashlib.sha256(run.stdout.encode()).hexdigest(),source_norm_ordinate=minus,native_internal_ordinate=Qy,scope='Cubic supported norm family only; commonX bridge excludes >=2zero k0norms iff complete and zero')
 if result['complete']:
  assert report['stats']['outer_visited']==262144 and report['stats']['candidates_visited']==report['stats']['candidates_planned'];assert report['field_modulus_hex']==hex(f.modulus);result['zero_source_k0_excluded']=report['supported_norm_count_exact']==0
 return result

def main():
 out={};error=None
 try:out['controls']=controls();assert time.monotonic()-START<20;out['actual']=actual();assert out['actual']['complete'],'native cubic scan incomplete'
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 trial=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':'))
 if time.monotonic()-START>=78:
  error='late serializedreport';a=out.get('actual',{});a['complete']=False;a['zero_source_k0_excluded']=False;r=a.get('native',{});r['complete']=False
  if 'supported_norm_count_exact'in r:r['supported_norm_count_lower_bound']=r.pop('supported_norm_count_exact')
 raw=json.dumps(dict(out=out,error=error),sort_keys=True,separators=(',',':')).encode()
 with(P/'raw.json').open('xb')as f:f.write(raw)
 a=out.get('actual',{});ev=dict(error=error,controls=out.get('controls'),actual_complete=a.get('complete'),zero_source_k0_excluded=a.get('zero_source_k0_excluded',False),native_stats=a.get('native',{}).get('stats'),worker_seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,child_peak_rss_kib=resource.getrusage(resource.RUSAGE_CHILDREN).ru_maxrss,raw_sha256=hashlib.sha256(raw).hexdigest(),raw_bytes=len(raw),qualified_speedup=None,release_gate_claim=False,complete_global_root_search=False)
 result=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed'if error is None else'inconclusive',summary='CommonX target-sign bridge and complete actualcubicnorm scan; originalchain/distinctfive/otherk remainopen.',evidence=ev,next_actions=['If zero, combine with reciprocaltriple certificate to prove allsupportedk0quintics distinctnonzero; then exact squarefree support fastpath L_VmodH0 needsnoGCD. Keep allotherk/ghost/sourceclosure.'])
 text=json.dumps(result,separators=(',',':'));assert len(text)<16384 and time.monotonic()-START<84;print(text)
if __name__=='__main__':main()
