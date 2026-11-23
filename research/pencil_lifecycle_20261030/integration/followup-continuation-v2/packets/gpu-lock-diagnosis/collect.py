from pathlib import Path
import hashlib,json,shutil,os
p=Path(__file__).parent;r=Path('/home/ec2-user/crypto-pencil-signed-graph-20261029');v=r/'research/pencil_lifecycle_20261030/integration/followup-continuation-v2/preflight-review'
refs=[]
def retain(src,dst):
 dst=p/dst;dst.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(src,dst);refs.append({'source':str(src),'retained':str(dst.relative_to(p)),'sha256':hashlib.sha256(dst.read_bytes()).hexdigest()})
for label,src in [('full-lib',v/'final-software-checks/full-lib'),('isolated-replay',v/'gpu-capsule-failure-replay')]:
 for n in ['stdout','stderr','receipt.json','command.json']:retain(src/n,Path(label)/n)
for n in ['gpu.rs','isolation.rs','runner.rs','host.rs']:retain(r/'src/cryptanalysis/ecbench'/n,Path('source')/n)
for n in ['gpu-stage.json','host.json','selftest.stdout','selftest.stderr']:retain(Path('/tmp/ecbench-gpu-capsule-2039903/out')/n,Path('capsule')/n)
(p/'source-bindings.json').write_text(json.dumps(refs,indent=2)+'\n')
c=json.loads((p/'capsule/gpu-stage.json').read_text());h=(p/'capsule/host.json').read_bytes()
checks={'status':c['status']=='selftest_failed','host_hash':c['host_sha256']==hashlib.sha256(h).hexdigest(),'env_prefix':c['env_class_id'].startswith('ECBENV3h'),'lock_path':c['lock_path']=='/tmp/ecbench-gpu-capsule-2039903/lock','elapsed_scope':'CUDA'in c['elapsed_scope']}
assert all(checks.values())
s=Path('/tmp/ecbench-gpu-capsule-2039903/lock').stat();key=f'{os.major(s.st_dev):02x}:{os.minor(s.st_dev):02x}:{s.st_ino}'
(p/'observations.json').write_text(json.dumps({'capsule_checks':checks,'lock_inode_key':key,'matching_locks_at_inspection':[x for x in Path('/proc/locks').read_text().splitlines() if key in x],'full_suite':{'passed':5792,'failed':1,'ignored':128,'failure':'gpu.rs:1270:47 nonblocking BenchLock reacquire Err'},'isolated_replay':{'exit_code':0,'passed':1},'concurrency_hypothesis_confirmed':False,'tests_or_builds_launched_by_this_audit':False},indent=2)+'\n')
