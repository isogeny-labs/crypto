from pathlib import Path
p=Path('/var/tmp/pencil-musl-sparse-proposal-20261031/prepare.py');s=p.read_text()
old="D=Path(__file__).parent;R='/home/ec2-user/crypto-pencil-signed-graph-20261029';I=Path('/var/tmp/pencil-b985-tree-inventory.txt');assert hashlib.sha256(I.read_bytes()).hexdigest()=='01140e75482e40a0d7860c634c3b3bc365928db6e3b648e0e0c85d60400fe9d0'"
new="""import argparse,tomllib,signal
ap=argparse.ArgumentParser();ap.add_argument('--repo',required=True);ap.add_argument('--commit',required=True);ap.add_argument('--inventory',required=True);ap.add_argument('--inventory-sha256',required=True);ap.add_argument('--out',required=True);a=ap.parse_args()
D=Path(a.out);R=a.repo;I=Path(a.inventory);assert D.is_absolute() and str(D).startswith('/var/tmp/') and D.is_dir();assert re.fullmatch('[0-9a-f]{40}',a.commit);assert hashlib.sha256(I.read_bytes()).hexdigest()==a.inventory_sha256
for n in ['selected.json','omitted.json','include-scan.json','sparse-patterns.txt','summary.json','package-closure.json']:assert not (D/n).exists(),n
assert subprocess.run(['git','rev-parse',a.commit+'^{commit}'],cwd=R,capture_output=True,check=True,text=True,timeout=20).stdout.strip()==a.commit
"""
assert old in s;s=s.replace(old,new)
pkgline="packages=['','crates/hashed-ecdl-tensor','crates/ecdl-tensor-ic','research/pencil_large_prime_20261029','research/five_leads_e2e_20261019/native','tools/factor-base','tools/volcano-cli','isogeny_algos']"
newpkg="""def exact_blob(p):
 assert p in rows and rows[p]['mode'] in ['100644','100755'],p
 return subprocess.run(['git','cat-file','blob',rows[p]['object']],cwd=R,capture_output=True,check=True,timeout=20).stdout
packages=[];queue=[''];edges=[];manifests=[]
while queue:
 pre=queue.pop(0)
 if pre in packages:continue
 packages.append(pre);mf=(pre+'/' if pre else '')+'Cargo.toml';raw=exact_blob(mf);doc=tomllib.loads(raw.decode());manifests.append({'package':pre,'manifest':mf,'git_blob':rows[mf]['object'],'sha256':hashlib.sha256(raw).hexdigest(),'build_field':doc.get('package',{}).get('build'),'default_build_rs_present':(pre+'/' if pre else '')+'build.rs' in rows})
 def walk(x,context=''):
  if not isinstance(x,dict):return
  for key,value in x.items():
   if key in ['dependencies','dev-dependencies','build-dependencies'] and isinstance(value,dict):
    for name,dep in value.items():
     if isinstance(dep,dict) and 'path' in dep:
      target=posixpath.normpath(posixpath.join(pre,dep['path']));assert not target.startswith('../') and not target.startswith('/'),target
      target='' if target=='.' else target
      edges.append({'from':pre,'dependency':name,'path':target,'optional':bool(dep.get('optional',False)),'context':context+'/'+key});queue.append(target)
     if isinstance(dep,dict) and dep.get('workspace'):raise AssertionError('workspace inherited dependency requires explicit renewed closure review: '+name)
   elif key=='patch':
    for registry,overrides in value.items():
     walk({'dependencies':overrides},context+'/patch/'+registry)
   else:walk(value,context+'/'+key)
 walk(doc)
(D/'package-closure.json').write_text(json.dumps({'packages':packages,'manifests':manifests,'edges':edges,'scope':'all local dependency tables including optional and target variants; workspace inheritance fails closed'},indent=2)+'\\n')"""
assert pkgline in s;s=s.replace(pkgline,newpkg)
s=s.replace("'commit':'b985ddb30b46abbd9a050e5753c7e301f40b07f6'","'commit':a.commit")
s=s.replace("(D/'sparse-patterns.txt').write_text", "assert all(not any(c.isspace() or c in '*?[]\\\\!#' for c in p) and not p.startswith('/') and '..' not in Path(p).parts for p in selected),'unsafe sparse pattern'\n(D/'sparse-patterns.txt').write_text")
Path(__file__).with_name('prepare-final.py').write_text(s)
