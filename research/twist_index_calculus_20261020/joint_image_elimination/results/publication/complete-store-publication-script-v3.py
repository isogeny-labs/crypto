"""Artifact infrastructure only; credentials enter hidden stdin, never receipts."""
import base64
import datetime
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys

root = Path('/home/ec2-user/crypto-twist-ic')
study = root/'research/twist_index_calculus_20261020/joint_image_elimination'
packed = Path('/home/ec2-user/scratch/twist-joint-complete-archive-v3')
archive = packed/'runs.tar.xz'
dest = study/'results/publication'
receipt = dest/'complete-store-publish-v3.json'
name = 'evidence/twist-index-calculus/20261024-joint-image-runs.tar.xz'
old = 'f4ddf5ce185d0866ff02e7a4e4085065e59756cf86798aaff85f7a8398eaa39c'
check = json.loads((packed/'verified/complete-verification.json').read_text())
assert (packed/'exit').read_text().strip() == '0'
assert check['status'] == 'pass' and check['byte_identical_repack'] is True
digest, size = check['archive_sha256'], check['archive_bytes']
assert check['repacked_sha256'] == digest and digest != old
manifest = json.loads((packed/'runs.tar.xz.manifest.json').read_text())
assert len(manifest['included_terminal_or_reconciled_interrupted_runs']) == 43
assert archive.stat().st_size == size
with archive.open('rb') as stream:
    h = hashlib.sha256()
    for chunk in iter(lambda: stream.read(1024*1024), b''):
        h.update(chunk)
    assert h.hexdigest() == digest
assert not receipt.exists(), 'Reconcile saved publication before another attempt'
out = {'schema':'joint-complete-raw-store-publication/v1',
       'recorded_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),
       'scope':'All 43 reconciled raw run directories; content integrity and publication only',
       'archive_sha256':digest, 'archive_bytes':size, 'store_name':name,
       'prior_checkpoint_sha256':old, 'archive_verification':check,
       'infrastructure_script_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
       'python_version':sys.version.split()[0]}
def save():
    receipt.write_text(json.dumps(out,indent=2)+'\n')
def call(argv, env, limit=120):
    p = subprocess.run(argv, env=env, cwd=root, capture_output=True, text=True, timeout=limit)
    match = re.search(r'An error occurred \(([^)]+)\)',p.stderr)
    return p, match.group(1) if match else ('none' if p.returncode==0 else 'command-failed')
print('READY: all 43 directories passed exact integrity checks; hidden credential input.',flush=True)
pair = json.loads(sys.stdin.readline())
assert len(pair)==2
env = os.environ.copy()
for key in ('AWS_PROFILE','AWS_SESSION_TOKEN','AWS_SECURITY_TOKEN','AWS_ACCESS_KEY_ID','AWS_SECRET_ACCESS_KEY'):
    env.pop(key,None)
env.update(AWS_ACCESS_KEY_ID=pair[0],AWS_SECRET_ACCESS_KEY=pair[1],
           AWS_REGION='us-east-1',AWS_DEFAULT_REGION='us-east-1',AWS_PAGER='',
           AWS_EC2_METADATA_DISABLED='true',CRYPTO_STORE_REMOTE='s3://crypto-autoresearcher/store',
           CRYPTO_STORE_CACHE='/home/ec2-user/scratch/twist-joint-store-cache-v3')
pair = None
try:
    sts,code=call(['aws','sts','get-caller-identity','--output','json'],env)
    out.update(sts_exit=sts.returncode,sts_error_code=code)
    assert sts.returncode==0
    out['account']=json.loads(sts.stdout)['Account']
    assert out['account']=='590183823895'
    save()
    cli=str(root/'target/release/ecbench')
    info,code=call([cli,'store','info',name],env)
    assert info.returncode==0
    prior=json.loads(info.stdout)
    assert prior['sha256'] in (old,digest)
    out['reconciled_existing_native_index']=prior
    index_before=json.loads((root/'store/index.json').read_text())
    backup=dest/'complete-store-index-before-v3.json'
    assert not backup.exists()
    backup.write_text(json.dumps(index_before,indent=2)+'\n')
    if prior['sha256']==digest:
        assert prior['entry']['bytes']==size
        out.update(push_exit=0,upload_repeated=False)
    else:
        assert prior['entry']['bytes']==542528576
        save()
        push,code=call([cli,'store','push',str(archive),'--name',name,'--replace',
                       '--description','Complete finite joint-image checkpoint: all 43 reconciled raw run directories, including failures and interrupted attempts; no complete-cost claim',
                       '--provenance','pack-complete-checkpoint-level1.sh; fresh extraction/native cmp of all files and byte-identical repack; source '+manifest['source_commit']+'; SHA-256 '+digest,'--json'],env,1200)
        out.update(push_exit=push.returncode,push_error_code=code)
        save()
        assert push.returncode==0
        out['native_receipt']=json.loads(push.stdout)
        out['upload_repeated']=False
    index_after=json.loads((root/'store/index.json').read_text())
    entry=index_after['entries'][name]
    assert entry['sha256']==digest and entry['bytes']==size
    assert {'sha256':old,'bytes':542528576} in entry['superseded']
    assert {k:v for k,v in index_before['entries'].items() if k!=name} == {k:v for k,v in index_after['entries'].items() if k!=name}
    out['prior_digest_retained_as_superseded']=True
    save()
    verify,code=call([cli,'store','verify',name,'--remote','--json'],env,1200)
    out.update(verify_exit=verify.returncode,verify_error_code=code)
    if verify.returncode==0:
        out['verify_receipt']={'stdout':verify.stdout,'stderr':verify.stderr}
    save()
    assert verify.returncode==0
    head,code=call(['aws','s3api','head-object','--bucket','crypto-autoresearcher',
                    '--key','store/sha256/'+digest[:2]+'/'+digest,'--checksum-mode','ENABLED','--output','json'],env)
    out.update(head_exit=head.returncode,head_error_code=code)
    assert head.returncode==0
    remote=json.loads(head.stdout)
    assert remote['ContentLength']==size
    assert base64.b64decode(remote['ChecksumSHA256']).hex()==digest
    out['head_receipt']={key:remote[key] for key in ('ContentLength','ChecksumSHA256','ETag') if key in remote}
    out['head_digest_verified_via']='SHA256-object-checksum'
    old_head,code=call(['aws','s3api','head-object','--bucket','crypto-autoresearcher',
                       '--key','store/sha256/f4/'+old,'--checksum-mode','ENABLED','--output','json'],env)
    assert old_head.returncode==0
    old_remote=json.loads(old_head.stdout)
    assert old_remote['ContentLength']==542528576
    assert base64.b64decode(old_remote['ChecksumSHA256']).hex()==old
    out['prior_remote_object_retained_and_checksum_verified']=True
    out['status']='pass'
    save()
    print(json.dumps({'status':'pass','store_name':name,'bytes':size,'sha256':digest,'prior_retained':True}),flush=True)
except Exception as exc:
    out.update(status='interrupted-or-failed',error_type=type(exc).__name__)
    save()
    print(json.dumps({'status':out['status'],'error_type':type(exc).__name__}),flush=True)
    raise SystemExit(2)
finally:
    env=None
