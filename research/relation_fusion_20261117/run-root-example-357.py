from pathlib import Path
import subprocess,json,os,hashlib,datetime
repo=Path('/workspace/relation-supervisor-live');s=repo/'research/relation_fusion_20261117';assert (s/'ROOT_API_OUTCOME_356.json').is_file()
ctx=json.loads((s/'ROOT_EXAMPLE_CONTEXT_357.json').read_text());assert all(hashlib.sha256((repo/p).read_bytes()).hexdigest()==h for p,h in ctx['source_sha256'].items())
env=dict(os.environ,CARGO_PROFILE_RELEASE_OPT_LEVEL='0',CARGO_PROFILE_RELEASE_LTO='off',CARGO_PROFILE_RELEASE_CODEGEN_UNITS='16',CARGO_BUILD_JOBS='1',CARGO_TARGET_DIR='/workspace/crypto-root-unoptimized-181',RAYON_NUM_THREADS='1',OMP_NUM_THREADS='1',OPENBLAS_NUM_THREADS='1',RUST_TEST_THREADS='1');rows=[]
for i,args in enumerate(ctx['original_documented_arguments']):
 cmd=['taskset','-c','0','/home/agent/.cargo/bin/cargo','run','--release','--locked','--manifest-path','research/relation_fusion_20261117/root-example-tests/Cargo.toml','--bin','prime-pair-table-check','--',*args]
 with (s/f'development-01/root-example-{i}-357.stdout').open('xb') as out,(s/f'development-01/root-example-{i}-357.stderr').open('xb') as err:r=subprocess.run(cmd,cwd=repo,env=env,stdout=out,stderr=err)
 raw=(s/f'development-01/root-example-{i}-357.stdout').read_bytes();data=json.loads(raw) if raw.strip() else None;rows.append({'args':args,'exit_code':r.returncode,'actual_output':data,'stdout_sha256':hashlib.sha256(raw).hexdigest(),'complete':r.returncode==0 and data['identical'] is True and data['word_path_used'] is True})
 (s/'ROOT_EXAMPLE_OUTCOME_357.json').write_text(json.dumps({'schema':'actual-root-original-example-outcome/v1','context':'ROOT_EXAMPLE_CONTEXT_357.json','calls':rows,'all_declared_calls_complete':len(rows)==2 and all(x['complete'] for x in rows),'signed_execution_verdict':False,'performance_comparison':False,'15x_admitted':False},indent=2)+'\n')
 if not rows[-1]['complete']:print(json.dumps(rows));raise SystemExit(r.returncode or 1)
print(json.dumps(rows))
