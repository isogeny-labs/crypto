from pathlib import Path
import os,json,hashlib,subprocess,datetime,re
out=Path(__file__).parent
repo=Path('/home/ec2-user/crypto-ml-ordinary-ownership-20261101')
env=os.environ.copy();env.update(CARGO_TARGET_DIR='/var/tmp/crypto-ml-dynamic-adapter-target',CARGO_BUILD_JOBS='8',RAYON_NUM_THREADS='8')
for key in ['CRYPTO_BUILD_GIT_COMMIT','CRYPTO_BUILD_GIT_DIRTY']:env.pop(key,None)
def stamp():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def snapshot():return {str(p.relative_to(repo)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted((repo/'src').rglob('*')) if p.is_file()} | {n:hashlib.sha256((repo/n).read_bytes()).hexdigest() for n in ['Cargo.toml','Cargo.lock']}
initial=snapshot();(out/'source-hashes.json').write_text(json.dumps(initial,indent=2)+'\n')
commit=subprocess.check_output(['git','rev-parse','HEAD'],cwd=repo,text=True).strip()
argv=['cargo','+1.98','test','--release','--lib','--locked','--offline','--','--test-threads=1']
row={'argv':argv,'source_base':commit,'source_has_uncommitted_ordinary_patches':True,'started_at':stamp(),'source_hash_manifest':'source-hashes.json'}
(out/'dataset.started.json').write_text(json.dumps(row,indent=2)+'\n')
with (out/'dataset.log').open('xb') as h:result=subprocess.run(argv,cwd=repo,env=env,stdout=h,stderr=subprocess.STDOUT)
log=(out/'dataset.log').read_text();m=re.findall(r'test result: (?:ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out',log)
counts=list(map(int,m[-1])) if m else None
row.update(finished_at=stamp(),exit_code=result.returncode,source_unchanged=snapshot()==initial,counts=counts,nonempty_summary=bool(counts and counts[0]),log_sha256=hashlib.sha256((out/'dataset.log').read_bytes()).hexdigest())
(out/'dataset.command.json').write_text(json.dumps(row,indent=2)+'\n');(out/'dataset.exit').write_text(str(result.returncode)+'\n');print(json.dumps(row),flush=True)
raise SystemExit(result.returncode or (0 if row['source_unchanged'] and row['nonempty_summary'] else 1))
