"""Read-only local contract joins. Never invokes a container, build or payload."""
from pathlib import Path
import hashlib,json,stat
P=Path(__file__).resolve().parent;B=P.parent
def digest(data):return hashlib.sha256(data).hexdigest()
def strict(data):
 def unique(items):
  out={}
  for k,v in items:
   if k in out:raise ValueError('duplicate key '+k)
   out[k]=v
  return out
 return json.loads(data,object_pairs_hook=unique)
def require(ok,msg):
 if not ok:raise ValueError(msg)
checks=[]
def pin(path,expected,label):
 path=Path(path)
 for c in [path,*path.parents]:require(not c.is_symlink(),'symlink '+str(c))
 st=path.stat();require(stat.S_ISREG(st.st_mode),'regular '+str(path));raw=path.read_bytes();require(digest(raw)==expected,'digest '+label)
 checks.append(dict(label=label,path=str(path),sha256=expected,bytes=len(raw),mode=oct(stat.S_IMODE(st.st_mode))));return raw
q=strict((P/'qualification-binding.draft.json').read_bytes());require(q['admitted'] is False and q['image_id'] is None and q['prospective_review_sha256'] is None,'unadmitted replacement')
staged=strict((B/'runtime-context-staging/staging-receipt.json').read_bytes());ctx=Path(staged['context_path'])
require(ctx.is_dir() and not ctx.is_symlink(),'context type');require(oct(stat.S_IMODE(ctx.stat().st_mode))==staged['root_mode'],'context mode');require(oct(stat.S_IMODE(ctx.parent.stat().st_mode))==staged['private_parent_mode'],'private parent mode')
require({str(x.relative_to(ctx)) for x in ctx.rglob('*')}=={m['context_path'] for m in staged['members']},'context inventory')
for m in staged['members']:
 f=ctx/m['context_path'];require(not f.is_symlink(),'context symlink');st=f.lstat();require(oct(stat.S_IMODE(st.st_mode))==m['mode'],'context member mode')
 if m['kind']=='file':require(st.st_size==m['bytes'],'context size');pin(f,m['sha256'],'context/'+m['context_path'])
 else:require(m['kind']=='directory' and stat.S_ISDIR(st.st_mode),'context directory')
require(digest((ctx/'Dockerfile').read_bytes())==staged['recipe_sha256'],'recipe pin')
require((ctx/'Dockerfile').read_bytes()==(B/'runtime-mountpoint-proposal/Dockerfile.runtime.proposed').read_bytes(),'reviewed recipe join')
fix=Path('/var/tmp/pencil-runtime-fixtures-20261031-v1');manifest=strict(pin(fix/'manifest.json',q['fixture_manifest_sha256'],'fixture manifest'))
require({str(f.relative_to(fix)) for f in fix.rglob('*') if f.is_file()}==set(manifest)|{'manifest.json'},'fixture closure')
for rel,sha in manifest.items():
 require(not Path(rel).is_absolute() and '..' not in Path(rel).parts,'fixture path');pin(fix/rel,sha,'fixture/'+rel)
for key in ['docker_client','curl_client','gate_binary','gate_control_receipt','gate_independent_review']:
 path=q[key] if key.endswith('client') else q[key+'_path'];pin(path,q[key+'_sha256'],key)
pin(Path(q['gate_control_receipt_path']).parent/'admission.json',q['gate_control_admission_sha256'],'gate control admission')
require(oct(stat.S_IMODE(Path(q['gate_binary_path']).stat().st_mode))=='0o555','gate mode')
pin(B/'runtime-qualifier-v5/supervise.py',q['reviewed_supervisor_sha256'],'qualifier source')
pin(B/'runtime-qualifier-v5/head_contract.py',q['head_contract_sha256'],'HEAD parser')
pin('/var/tmp/pencil-local-image-assembly-proposal-20261031-v1/runtime-members.json',q['runtime_members_sha256'],'runtime member ledger')
a=strict((B/'assembly-filled-binding/binding.json').read_bytes());br=strict(pin(a['final_build_receipt_path'],q['final_build_receipt_sha256'],'build receipt'));pin(a['binary_path'],q['binary_sha256'],'binary')
require(br['binary_sha256']==q['binary_sha256'] and br['status']=='built_static_candidate' and br['static_elf_checked'] is True,'build receipt joins')
require(br['source_manifest']['commit']==q['build_info']['source_revision']==a['source_revision'],'source revision joins')
require(q['build_info']['ecbench_sha256']==q['binary_sha256'],'build-info binary')
require(q['build_info']==a['build_info'],'historical build-info unchanged')
pin(B/'final-source-refresh-executed/new-materialized.json',q['final_source_manifest_sha256'],'materialized source manifest')
pin('/var/tmp/pencil-copy-only-assembly-execution-20261031-v1/contexts/base/build-info.json',q['build_info_sha256'],'retained build-info bytes')
pin('/var/tmp/pencil-copy-only-assembly-execution-20261031-v1/contexts/base/smoke-prime.json',q['smoke_prime_sha256'],'retained smoke fixture')
print(json.dumps(dict(schema='runtime.replacement.input-check/v1',files=checks,files_checked=len(checks),context_members=len(staged['members']),fixture_members=len(manifest),context_path=str(ctx),source_revision=a['source_revision'],binary_sha256=q['binary_sha256'],admitted=False,replacement_image=None,prospective_review=None,executed_payloads=0,executed_builds=0),indent=2))
