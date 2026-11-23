from pathlib import Path
import hashlib,itertools,json,subprocess,sys,tempfile
P=Path(__file__).resolve().parent;R=P.parents[1];raws={}
for name,digest in json.loads((P/'EVIDENCE_SHA256.json').read_text()).items():
 data=(R/name).read_bytes();assert hashlib.sha256(data).hexdigest()==digest;x=json.loads(data);assert x['error']is None;raws[name]=x['out']
native=raws['research/root_compressed_native_20261115_v48/raw.json'];old=json.loads((R/'research/root_conditioned_native_20261115_v45/raw.json').read_text())['out'];key=lambda x:{(r,int(t,16))for r,t in x['native']['candidates']};assert key(native)==key(old)and len(key(old))==249 and native['norm_family_complete']and native['supported']==[]
cold=raws['research/root_compressed_cold_20261115_v48/raw.json'];assert len(cold['records'])==6 and len({r['candidate_sha256']for r in cold['records']})==1 and all(r['complete']and not r['norms']for r in cold['records'])
polys=set()
for ones in range(6):
 H=[1]
 for root in [1]*ones+[0]*(5-ones):
  Q=[0]*(len(H)+1)
  for i,v in enumerate(H):Q[i]^=root*v;Q[i+1]^=v
  H=Q
 polys.add(tuple(H))
want=[tuple(cs)+(1,)in polys for cs in itertools.product(range(2),repeat=5)];want += [r['result']['supported']for r in json.loads((R/'research/subspace_factor_oracles_20261115_v30/raw.json').read_text())['out']['actual']['records']];want +=[False]*249
assert raws['research/native_quintic_support_20261115_v48/raw.json']['flags']==want
if '--native'in sys.argv:
 with tempfile.TemporaryDirectory(dir='/dev/shm')as folder:
  exe=Path(folder)/'scan';subprocess.run(['g++','-O3','-std=c++17',str(R/'research/root_compressed_native_20261115_v48/scan.cpp'),'-o',str(exe)],check=True,timeout=15,capture_output=True);run=subprocess.run([str(exe)],input=(R/'research/root_compressed_native_20261115_v48/matrix.txt').read_text(),capture_output=True,text=True,timeout=60);assert run.returncode==0;v=json.loads(run.stdout);assert v['complete']and v['visited']==262143 and key({'native':v})==key(old)
print('Verified full candidate receipts, six cold-call fingerprints and all329 support flags; optional --native rebuilds the complete compressed scan.')
