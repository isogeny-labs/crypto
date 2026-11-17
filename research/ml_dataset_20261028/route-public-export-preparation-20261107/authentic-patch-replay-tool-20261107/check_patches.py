"""Replay combined patch application and Rust syntax checks; never builds or runs solvers."""
import argparse, hashlib, json, pathlib, re, subprocess, tempfile

def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
def main():
    ap=argparse.ArgumentParser();ap.add_argument('--repo',required=True);ap.add_argument('--packet',required=True);ap.add_argument('--receipt',required=True);a=ap.parse_args()
    root=pathlib.Path(a.packet).resolve(); out=pathlib.Path(a.receipt).resolve()
    if out.exists(): raise SystemExit('Receipt exists; inspect prior result rather than overwrite')
    expected=json.loads((root/'COMBINED_PATCH_CHECKS.json').read_text())
    names=['owned-original-registry-checkpoint-20261107/implementation.patch','owned-authentic-supervisor-20261107/supervisor.patch']
    patches=[root/n for n in names]
    for p,e in zip(patches,expected['patches']):
        if sha(p)!=e['sha256']: raise SystemExit('Patch digest mismatch: '+str(p))
    commit='3517ae554787690472e3ed9801623de27730e20d'
    result={'base_commit':commit,'compiled':False,'solver_executed':False,'checks':[]}
    with tempfile.TemporaryDirectory(prefix='authentic-patch-source-') as temp:
        work=pathlib.Path(temp)
        paths=set()
        for p in patches: paths.update(re.findall(r'^--- a/(.+)$',p.read_text(),re.M))
        for name in sorted(paths):
            q=pathlib.PurePosixPath(name)
            if q.is_absolute() or '..' in q.parts: raise SystemExit('Unsafe patch path')
            dst=work/name;dst.parent.mkdir(parents=True,exist_ok=True)
            dst.write_bytes(subprocess.check_output(['git','-C',a.repo,'show',commit+':'+name]))
        for p in patches: subprocess.run(['git','apply',str(p)],cwd=work,check=True)
        for item in expected['files']:
            p=work/item['file']; h=sha(p)
            if h!=item['sha256']: raise SystemExit('Assembled source mismatch: '+item['file'])
            c=subprocess.run(['rustfmt','+1.98','--edition','2021','--config','skip_children=true','--emit','stdout',str(p)],stdout=subprocess.DEVNULL,stderr=subprocess.PIPE,text=True)
            result['checks'].append({'path':item['file'],'sha256':h,'parse_exit':c.returncode,'stderr':c.stderr})
        result['passed']=all(x['parse_exit']==0 for x in result['checks'])
    with out.open('x') as f: json.dump(result,f,indent=2);f.write('\n')
    return 0 if result['passed'] else 1
if __name__=='__main__': raise SystemExit(main())
