from pathlib import Path
import json,hashlib,re,subprocess,posixpath
import argparse,tomllib,signal
ap=argparse.ArgumentParser();ap.add_argument('--repo',required=True);ap.add_argument('--commit',required=True);ap.add_argument('--inventory',required=True);ap.add_argument('--inventory-sha256',required=True);ap.add_argument('--out',required=True);args=ap.parse_args()
D=Path(args.out);R=args.repo;I=Path(args.inventory);assert D.is_absolute() and str(D).startswith('/var/tmp/') and D.is_dir();assert re.fullmatch('[0-9a-f]{40}',args.commit);assert hashlib.sha256(I.read_bytes()).hexdigest()==args.inventory_sha256
for n in ['selected.json','omitted.json','include-scan.json','sparse-patterns.txt','summary.json','package-closure.json']:assert not (D/n).exists(),n
assert subprocess.run(['git','rev-parse',args.commit+'^{commit}'],cwd=R,capture_output=True,check=True,text=True,timeout=20).stdout.strip()==args.commit

rows={}
for l in I.read_text().splitlines():
 a,p=l.split('\t',1);m,t,h,z=a.split();rows[p]={'path':p,'mode':m,'object':h,'bytes':int(z) if z!='-' else 0}
def exact_blob(p):
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
(D/'package-closure.json').write_text(json.dumps({'packages':packages,'manifests':manifests,'edges':edges,'scope':'all local dependency tables including optional and target variants; workspace inheritance fails closed'},indent=2)+'\n')
selected={};reasons={}
def add(p,reason):
 if p in rows and rows[p]['mode'] in ['100644','100755']:selected[p]=rows[p];reasons.setdefault(p,[]).append(reason);return True
 return False
for p in rows:
 n=Path(p).name
 if p.endswith(('.rs','.c','.h','.cc','.cpp','.S','.s')) or n in ['Cargo.toml','Cargo.lock','build.rs','rust-toolchain','rust-toolchain.toml'] or '/.cargo/' in '/'+p:add(p,'tracked code or Rust build/config metadata')
for pre in packages:
 for p in rows:
  if p.startswith((pre+'/' if pre else '')+'src/'):add(p,'complete local package source tree')
for p in ['.gitmodules','.gitattributes','.gitignore','AGENTS.md']:add(p,'root metadata')
proc=subprocess.Popen(['git','cat-file','--batch'],cwd=R,stdin=subprocess.PIPE,stdout=subprocess.PIPE)
def blob(p):
 proc.stdin.write((rows[p]['object']+'\n').encode());proc.stdin.flush();h=proc.stdout.readline().split();size=int(h[-1]);b=proc.stdout.read(size);assert proc.stdout.read(1)==b'\n';return b.decode(errors='replace')
includes=[];unresolved=[]; scanned=[]
for p in sorted(rows):
 if not p.endswith('.rs') or not any(p.startswith((q+'/' if q else '')+'src/') for q in packages):continue
 scanned.append(p);txt=blob(p)
 for mt in re.finditer(r'include(?:_str|_bytes)?!\s*\(',txt):
  start=mt.end();j=start;depth=1;instr=False;esc=False
  while j<len(txt) and depth:
   c=txt[j]
   if instr:
    if esc:esc=False
    elif c=='\\':esc=True
    elif c=='"':instr=False
   elif c=='"':instr=True
   elif c=='(':depth+=1
   elif c==')':depth-=1
   j+=1
  expr=txt[start:j-1].strip();entry={'source':p,'line':txt.count('\n',0,mt.start())+1,'expression':expr}
  literals=re.findall(r'"([^"\\]*(?:\\.[^"\\]*)*)"',expr)
  target=None
  if re.fullmatch(r'"[^"\\]*"\s*,?',expr):target=posixpath.normpath(posixpath.join(posixpath.dirname(p),literals[0]))
  elif expr.startswith('concat!') and 'env!("CARGO_MANIFEST_DIR")' in expr:
   pkg=max((q for q in packages if not q or p.startswith(q+'/')),key=len);ss=literals.copy();ss.remove('CARGO_MANIFEST_DIR');target=posixpath.normpath(pkg+''.join(ss)).lstrip('/')
  if target is not None:
   entry['target']=target;entry['selected']=add(target,'embedded by '+p+':'+str(entry['line']))
  if target is None or not entry.get('selected'):unresolved.append(entry)
  includes.append(entry)
proc.stdin.close();proc.wait()
(D/'selected.json').write_text(json.dumps({'commit':args.commit,'inventory_sha256':hashlib.sha256(I.read_bytes()).hexdigest(),'files':[dict(v,reasons=reasons[k]) for k,v in sorted(selected.items())]},indent=2)+'\n')
(D/'omitted.json').write_text(json.dumps({'files':[v for k,v in rows.items() if k not in selected]},indent=2)+'\n')
(D/'include-scan.json').write_text(json.dumps({'scanned_sources':scanned,'includes':includes,'unresolved':unresolved},indent=2)+'\n')
assert all(not any(c.isspace() or c in '*?[]\\!#' for c in p) and not p.startswith('/') and '..' not in Path(p).parts for p in selected),'unsafe sparse pattern'
(D/'sparse-patterns.txt').write_text(''.join('/'+p+'\n' for p in sorted(selected)))
summary={'selected_files':len(selected),'selected_bytes':sum(v['bytes'] for v in selected.values()),'omitted_files':len(rows)-len(selected),'include_expressions':len(includes),'unresolved':unresolved};(D/'summary.json').write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps(summary,indent=2))
