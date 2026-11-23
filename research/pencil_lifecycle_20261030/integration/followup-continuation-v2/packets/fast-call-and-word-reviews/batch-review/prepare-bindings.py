import pathlib,subprocess,json,hashlib
D=pathlib.Path(__file__).parent; repo='/home/ec2-user/crypto-pencil-signed-graph-20261029'; commit=(D/'commit.txt').read_text().strip()
rows=subprocess.check_output(['git','-C',repo,'ls-tree','-r',commit],text=True).splitlines(); blobs={}
for row in rows:
 a,p=row.split('\t'); blobs.setdefault(a.split()[2],[]).append(p)
out=[]
for f in (D/'deps').rglob('*.lean'):
 sha=subprocess.check_output(['git','hash-object',str(f)],text=True).strip(); out.append({'file':str(f.relative_to(D)),'git_blob':sha,'archived_matches':blobs.get(sha,[])[:2]})
(D/'helper-source-bindings.json').write_text(json.dumps(out,indent=2)); print('matched',sum(bool(x['archived_matches']) for x in out),'of',len(out))
paths=list((D/'input').glob('*.lean'))+list((D/'deps').rglob('*'))+[D/'native-three_leads.rs',D/'lean-path.txt',D/'inventory.txt',D/'dependency-map.json']
for p in json.loads((D/'dependency-map.json').read_text()):
 if p[2]=='reused union caches': paths+=list(pathlib.Path(p[0]).rglob('*'))
lean=pathlib.Path('/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain/bin/lean'); paths.append(lean)
with (D/'before.sha256').open('w') as o:
 for f in sorted(set(paths)):
  if f.is_file(): o.write(hashlib.sha256(f.read_bytes()).hexdigest()+'  '+str(f)+'\n')
