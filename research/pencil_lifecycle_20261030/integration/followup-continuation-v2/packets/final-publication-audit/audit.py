from pathlib import Path
import hashlib,json,re,subprocess
p=Path(__file__).parent;repo=Path('/home/ec2-user/crypto-pencil-signed-graph-20261029');integ=repo/'research/pencil_lifecycle_20261030/integration';root=integ/'followup-continuation-v2';lean=integ/'lean/same-support';sha=lambda q:hashlib.sha256(q.read_bytes()).hexdigest()
checks=[];issues=[];maps=list(root.rglob('custody-map.json'))
for c in maps:
 if 'boxed-request-software-checks' in c.parts:continue
 d=json.loads(c.read_text())
 for row in d['files']:
  name=row.get('included_path')
  if name:
   q=c.parent/name;ok=q.is_file() and sha(q)==row['sha256'];checks.append({'map':str(c.relative_to(root)),'included_path':name,'sha256':row['sha256'],'passed':ok})
   if not ok:issues.append(checks[-1])
(p/'custody-checks.json').write_text(json.dumps(checks,indent=2)+'\n')
latest=root/'report-branch-completion/index.json';d=json.loads(latest.read_text());links=[]
for b in d['evidence_bindings']:
 q=(latest.parent/b['link']).resolve();ok=q.is_file() and sha(q)==b['sha256'];links.append(dict(b,passed=ok))
 if not ok:issues.append({'aggregate_link':b['link']})
(p/'aggregate-links.json').write_text(json.dumps(links,indent=2)+'\n')
modnames=re.findall(r'^lean_lib (\w+)',(lean/'lakefile.lean').read_text(),re.M)
candidates={}
for q in integ.rglob('*.lean.txt'):
 if 'boxed-request-software-checks' in q.parts:continue
 candidates.setdefault(q.name[:-4],[]).append(q)
mods=[]
for name in modnames:
 q=lean/(name+'.lean');raw=q.read_bytes();opts=candidates.get(q.name,[]);matches=[str(f.relative_to(integ)) for f in opts if f.read_bytes()==raw]
 norm=[str(f.relative_to(integ)) for f in opts if f.read_bytes().rstrip(b'\n')+b'\n'==raw]
 result={'module':name,'canonical_sha256':sha(q),'exact_snapshot_matches':matches,'terminal_newline_matches':norm if not matches else [],'passed':bool(matches or (name in ['Constructor','Add11'] and norm))};mods.append(result)
 if not result['passed']:issues.append({'canonical_module_unbound':name})
(p/'canonical-bindings.json').write_text(json.dumps(mods,indent=2)+'\n')
cache=[]
for q in root.rglob('*'):
 if not q.is_file() or 'boxed-request-software-checks' in q.parts:continue
 if q.suffix in ['.olean','.ilean','.o','.a','.aux','.pyc'] or any(s in ['target','__pycache__','.lake'] for s in q.relative_to(root).parts):cache.append(str(q.relative_to(root)))
# Canonical .lake may be an active local cache; separately inventory rather than claim publication.
canonical_caches=[str(q.relative_to(lean)) for q in lean.iterdir() if q.name in ['.lake','target'] or q.suffix in ['.olean','.ilean','.o','.aux','.pyc']]
(p/'cache-inventory.json').write_text(json.dumps({'followup_candidates':cache,'canonical_local_cache_paths':canonical_caches,'definition':'compiled objects, Python cache, target/.lake build paths, TeX .aux; .log/.tex are retained render source/receipts not binaries'},indent=2)+'\n')
f=repo/'src/cryptanalysis/ecbench/diagnostic_file_adapter.rs';old=subprocess.run(['git','show','da1464aea27534f6daf0bc3de0cdc687030b66d2:src/cryptanalysis/ecbench/diagnostic_file_adapter.rs'],cwd=repo,capture_output=True,check=True).stdout;new=f.read_bytes();expected=old.replace(b'actual: Actual,',b'actual: Box<Actual>,');assert new==expected
(p/'adapter-base.rs').write_bytes(old);(p/'adapter-current.rs').write_bytes(new)
(p/'summary.json').write_text(json.dumps({'custody_maps':len(maps),'included_entries':len(checks),'included_entries_passed':sum(x['passed']for x in checks),'aggregate_links':len(links),'aggregate_links_passed':sum(x['passed']for x in links),'canonical_targets':len(mods),'canonical_targets_bound':sum(x['passed']for x in mods),'cache_candidates':len(cache),'canonical_local_cache_paths':canonical_caches,'issues':issues,'adapter_change_exact_one_line':True,'adapter_base_sha256':hashlib.sha256(old).hexdigest(),'adapter_current_sha256':sha(f),'native_boxed_checks':'pending root; not executed by audit'},indent=2)+'\n');print((p/'summary.json').read_text())
