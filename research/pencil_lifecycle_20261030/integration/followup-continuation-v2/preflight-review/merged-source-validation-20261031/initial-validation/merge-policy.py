import subprocess,sys,importlib.util,pathlib,json
r=pathlib.Path('/home/ec2-user/crypto-pencil-signed-graph-20261029')
s=importlib.util.spec_from_file_location('artifacts',r/'tools/artifacts/artifacts.py');m=importlib.util.module_from_spec(s);sys.modules['artifacts']=m;s.loader.exec_module(m);c=m.Config.load(r)
paths=subprocess.check_output(['git','diff','--cached','9d9b3ffb7eb65349661109786121fcf2a97d3fb1','--name-only','--diff-filter=ACMR','-z'],cwd=r).split(b'\0');bad=[];count=0
paths=[p.decode() for p in paths if p];sizes=subprocess.check_output(['git','cat-file','--batch-check=%(objectsize)'],input=''.join(':'+p+'\n' for p in paths).encode(),cwd=r).decode().splitlines()
for p,z in zip(paths,sizes):
 n=int(z);count+=1
 if m.belongs_in_store(p,n,c.policy):bad.append({'path':p,'bytes':n})
print(json.dumps({'checked':count,'policy_violations':bad}));sys.exit(bool(bad))
