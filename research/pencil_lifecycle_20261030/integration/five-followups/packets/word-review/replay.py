from pathlib import Path
import os,subprocess,json,time,hashlib
D=Path(__file__).parent; S=Path('/var/tmp/pencil-five-leads-word-20261030'); L=Path('/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain/bin/lean')
manifest=json.loads((S/'manifest.json').read_text()); checks=[]
for e in manifest['files']:
 p=S/e['path'];checks.append({'path':e['path'],'match':p.exists() and hashlib.sha256(p.read_bytes()).hexdigest()==e['sha256']})
(D/'custody.json').write_text(json.dumps({'producer_manifest_sha256':hashlib.sha256((S/'manifest.json').read_bytes()).hexdigest(),'checks':checks},indent=2))
for n in ['CanonicalResidue','UInt64Residue']:(D/(n+'.lean')).write_bytes((S/(n+'.lean')).read_bytes())
names=['canonical_product_bound','uint64_mul_mod','uint64_mul_decode','uint64_square_decode','uint64_mul_canonical']
(D/'ReviewReadback.lean').write_text('import UInt64Residue\n'+''.join('#check PencilWordRefinement.'+n+'\n#print axioms PencilWordRefinement.'+n+'\n' for n in names)+'#check UInt64.toNat_mul\n#check UInt64.toNat_mod\n')
env=os.environ.copy();env['LEAN_SYSROOT']=str(L.parent.parent);env['LEAN_PATH']=str(D)+':/var/tmp/independent-same-support-replay-ow8hrtl9:/var/tmp/pencil-word-refinement-v1/dependency-union'
records=[]
for n in ['CanonicalResidue','UInt64Residue','ReviewReadback']:
 cmd=['timeout','-k','5s','120s',str(L),'-j1','-M4096','-o',str(D/(n+'.olean')),str(D/(n+'.lean'))];t=time.time()
 r=subprocess.run(cmd,cwd=D,env=env,capture_output=True);(D/(n+'.stdout')).write_bytes(r.stdout);(D/(n+'.stderr')).write_bytes(r.stderr)
 records.append({'command':cmd,'exit_code':r.returncode,'start':t,'end':time.time(),'source_sha256':hashlib.sha256((D/(n+'.lean')).read_bytes()).hexdigest(),'LEAN_PATH':env['LEAN_PATH']});print(n,r.returncode)
 if r.returncode:break
(D/'receipts.json').write_text(json.dumps(records,indent=2))
