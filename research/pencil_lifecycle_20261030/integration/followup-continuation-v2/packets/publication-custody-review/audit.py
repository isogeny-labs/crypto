from pathlib import Path
import json,hashlib,datetime,re,importlib.util,sys
sys.dont_write_bytecode=True
R=Path('/home/ec2-user/crypto-pencil-signed-graph-20261029');B=R/'research/pencil_lifecycle_20261030/integration/followup-continuation-v2';D=Path(__file__).parent;h=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();start=datetime.datetime.now(datetime.timezone.utc).isoformat();maps=sorted(B.rglob('custody-map.json'));snap={str(p.relative_to(B)):h(p) for p in maps};(D/'snapshot.json').write_text(json.dumps({'started_utc':start,'maps':snap},indent=2));checked=[];errors=[];omitted=0;artifacts={}
for m in B.rglob('.artifacts.json'):
 for n,v in json.loads(m.read_text())['files'].items():artifacts[str(m.parent/n)]=v
for m in maps:
 for f in json.loads(m.read_text())['files']:
  n=f.get('included_path')
  if not n:omitted+=1;continue
  p=m.parent/n;row={'path':str(p.relative_to(B)),'expected':f['sha256'],'artifact_managed':str(p) in artifacts}
  if p.exists():row['actual']=h(p);row['matches']=row['actual']==f['sha256'];row['bytes_match']=p.stat().st_size==f['bytes']
  else:row['matches']=False;row['missing']=True
  if row['artifact_managed']:row['artifact_pin_matches']=artifacts[str(p)]['sha256']==f['sha256'] and artifacts[str(p)]['bytes']==f['bytes']
  if not row['matches'] or not row.get('bytes_match',False) or row.get('artifact_pin_matches') is False:errors.append(row)
  checked.append(row)
links=[]
for p in [B/'README.md',B/'report/INDEX.md']:
 if not p.exists():continue
 for dest in re.findall(r'\]\(([^)]+)\)',p.read_text()):
  if '://' in dest or dest.startswith('#'):continue
  dest=dest.split('#')[0];target=p.parent/dest
  links.append({'from':str(p.relative_to(B)),'target':dest,'exists':target.exists()})
p=B/'report/index.json'
for c in json.loads(p.read_text()).get('claims',[]):
 if 'link' in c:links.append({'from':str(p.relative_to(B)),'target':c['link'],'exists':(p.parent/c['link']).exists()})
spec=importlib.util.spec_from_file_location('artifact_policy',R/'tools/artifacts/artifacts.py');mod=importlib.util.module_from_spec(spec);sys.modules[spec.name]=mod;spec.loader.exec_module(mod);cfg=mod.Config.load(R);oversize=[]
for p in B.rglob('*'):
 if p.is_file() and mod.belongs_in_store(str(p.relative_to(R)),p.stat().st_size,cfg.policy) and str(p) not in artifacts:oversize.append({'path':str(p.relative_to(B)),'bytes':p.stat().st_size})
canonical=[]
known={'Constructor':Path('/var/tmp/pencil-weighted-constructor-invariant-20261031-v1/Constructor.lean'),'WordErrorExclusion':Path('/var/tmp/pencil-word-error-exclusion-v1-20261031/WordErrorExclusion.lean'),'CrossScaling':Path('/var/tmp/pencil-cross-scaling-v1-20261031/CrossScaling.lean'),'NatClassifier':Path('/var/tmp/pencil-word-classifier-v2-20261031/NatClassifier.lean'),'WordClassifier':Path('/var/tmp/pencil-word-classifier-v2-20261031/WordClassifier.lean'),'CanonicalResidue':Path('/var/tmp/pencil-word-refinement-v1/CanonicalResidue.lean'),'UInt64Residue':Path('/var/tmp/pencil-five-leads-word-20261030/UInt64Residue.lean'),'InvariantBranches':Path('/var/tmp/pencil-five-leads-invariants-20261030/InvariantBranches.lean')}
for p in sorted((B.parent/'lean/same-support').glob('*.lean')):
 if p.name=='lakefile.lean':continue
 ref=known.get(p.stem,Path('/var/tmp/independent-same-support-replay-ow8hrtl9')/p.name);row={'module':p.stem,'canonical_sha256':h(p),'reference':str(ref),'reference_exists':ref.exists()}
 if ref.exists():row['reference_sha256']=h(ref);row['matches']=row['canonical_sha256']==row['reference_sha256']
 canonical.append(row)
endmaps={str(p.relative_to(B)):h(p) for p in B.rglob('custody-map.json')};result={'schema':'research.publication-custody-audit/v1','started_utc':start,'ended_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'map_count':len(maps),'included_count':len(checked),'explicit_omissions':omitted,'artifact_managed_checked':sum(x['artifact_managed'] for x in checked),'errors':errors,'links':links,'canonical_lean':canonical,'oversize_without_artifact_manifest':oversize,'additive_maps_during_audit':sorted(set(endmaps)-set(snap)),'changed_maps_during_audit':[n for n in snap if endmaps.get(n)!=snap[n]],'remote_object_readback':'not performed; local exact bytes and manifest pins only'};(D/'included-checks.json').write_text(json.dumps(checked,indent=2));(D/'audit.json').write_text(json.dumps(result,indent=2));print(json.dumps(result,indent=2))
