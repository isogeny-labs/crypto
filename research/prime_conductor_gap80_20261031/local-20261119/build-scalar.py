#!/usr/bin/env python3
"""Build orchestration only; original CLI plus actual frozen release library."""
import hashlib,json,os,pathlib,subprocess,sys
repo=pathlib.Path.cwd();s=repo/'research/prime_conductor_gap80_20261031/local-20261119';out=s/sys.argv[1];out.mkdir(exist_ok=True)
cache=pathlib.Path('/workspace/tools/conductor-wide-target-v4/release/build')
all_files=list(cache.rglob('*.rlib'))+list(cache.rglob('*.so'))+list(cache.rglob('*.a'))
dirs=sorted({str(p.parent) for p in all_files})
externs={}
choose={'num_bigint':'920d97d93b548abb','sha2':'d6bf49c17ec75546'}
for crate in ['crypto_lib','clap','serde','serde_json','num_bigint','num_traits','rand','sha2','rayon','hex']:
    candidates=sorted(cache.rglob('lib'+crate+'-*.rlib'))
    if crate in choose:candidates=[p for p in candidates if choose[crate] in p.name]
    assert len(candidates)==1,(crate,candidates)
    externs[crate]=candidates[0]
common=['/workspace/tools/rust-1.100/bin/rustc','--edition=2021','-C','opt-level=3','-C','debuginfo=0']
for d in dirs:common+=['-L','dependency='+d,'-L','native='+d]
for name,path in externs.items():common+=['--extern',name+'='+str(path)]
env=os.environ.copy();env.update(CARGO_MANIFEST_DIR=str(repo/'research/prime_conductor_gap80_20261031/wide-harness-20261118/cli'),CARGO_PKG_VERSION='0.1.0',CARGO_PKG_NAME='conductor-prime-mont4-cli')
commands=[common+['--test','tests/scalar_timing.rs','-o','/workspace/tools/conductor-local-20261119/scalar-tests'],common+['src/bin/ecbench.rs','-o','/workspace/tools/conductor-local-20261119/ecbench-scalar']]
(out/'commands.json').write_text(json.dumps({'argv':commands,'explicit_env':{k:env[k] for k in ['CARGO_MANIFEST_DIR','CARGO_PKG_VERSION','CARGO_PKG_NAME']},'root_library':'unchanged qualified release-v4 rlib','dependency_selection':{k:str(v) for k,v in externs.items()}},indent=2)+'\n')
sources=[repo/'src/bin/ecbench.rs',repo/'src/bin/ecbench/scalar_timing.rs',repo/'tests/scalar_timing.rs',pathlib.Path(__file__),*externs.values()]
(out/'build-inputs.sha256').write_text(''.join(hashlib.sha256(p.read_bytes()).hexdigest()+'  '+str(p)+'\n' for p in sources))
for i,cmd in enumerate(commands):
    with (out/f'{i}.stdout').open('wb') as stdout,(out/f'{i}.stderr').open('wb') as stderr:
        r=subprocess.run(cmd,env=env,stdout=stdout,stderr=stderr)
    (out/f'{i}.exit').write_text(str(r.returncode)+'\n')
    if r.returncode:print((out/f'{i}.stderr').read_text()[-6000:]);sys.exit(r.returncode)
print('original CLI and focused scalar adapter target compiled')
