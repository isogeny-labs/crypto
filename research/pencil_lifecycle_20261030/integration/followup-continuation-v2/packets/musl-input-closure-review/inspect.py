import pathlib,subprocess,tomllib,json,hashlib,os
r=pathlib.Path('/home/ec2-user/crypto-pencil-signed-graph-20261029'); seen=set(); pending=['Cargo.toml']; out={'manifests':[],'build_scripts':[],'configs':[]}
def get(n):return subprocess.run(['git','show','b985ddb30b46abbd9a050e5753c7e301f40b07f6:'+n],cwd=r,capture_output=True)
while pending:
 n=pending.pop()
 if n in seen:continue
 seen.add(n); x=get(n)
 if x.returncode:out['manifests'].append({'path':n,'missing':True});continue
 d=tomllib.loads(x.stdout.decode()); row={'path':n,'sha256':hashlib.sha256(x.stdout).hexdigest(),'paths':[]};out['manifests'].append(row)
 def walk(v):
  if isinstance(v,dict):
   for k,w in v.items():
    if isinstance(w,dict) and 'path' in w:
     path=os.path.normpath(str(pathlib.Path(n).parent/w['path']/'Cargo.toml'));row['paths'].append({'name':k,'path':path,'optional':w.get('optional',False)});pending.append(path)
    walk(w)
  elif isinstance(v,list):
   for w in v:walk(w)
 walk({k:v for k,v in d.items() if k not in ['bin','example','bench','lib']})
 b=str(pathlib.Path(n).parent/d.get('package',{}).get('build','build.rs'));q=get(b);out['build_scripts'].append({'path':b,'present':q.returncode==0,'text':q.stdout.decode() if q.returncode==0 else None})
for p in set([pathlib.Path('/.cargo'),pathlib.Path('/home/.cargo'),pathlib.Path('/home/ec2-user/.cargo'),r/'.cargo',pathlib.Path(os.environ.get('CARGO_HOME',str(pathlib.Path.home()/'.cargo')))]):
 for f in ['config','config.toml']:
  q=p/f;v={'path':str(q),'present':q.is_file()}
  if q.is_file():
   z=q.read_bytes();v.update(sha256=hashlib.sha256(z).hexdigest(),keys=list(tomllib.loads(z.decode())))
  out['configs'].append(v)
lock=get('Cargo.lock');ld=tomllib.loads(lock.stdout.decode());out['lock']={'sha256':hashlib.sha256(lock.stdout).hexdigest(),'source_kinds':sorted(set(x.get('source','local') for x in ld['package']))}
pathlib.Path('/var/tmp/pencil-musl-input-closure-review-20261031/evidence.json').write_text(json.dumps(out,indent=2)+'\n'); print(json.dumps(out,indent=2))
