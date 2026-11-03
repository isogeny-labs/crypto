"""Infrastructure only. Scoped stdin credentials; no secret values in receipts."""
import datetime
import base64
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys

root = Path('/home/ec2-user/crypto-twist-ic')
study = root/'research/twist_index_calculus_20261020/joint_image_elimination'
archive = study/'runs.tar.xz'
dest = study/'results/publication'
receipt = dest/'terminal-store-publish-v2.json'
name = 'evidence/twist-index-calculus/20261024-joint-image-runs.tar.xz'
digest = 'f4ddf5ce185d0866ff02e7a4e4085065e59756cf86798aaff85f7a8398eaa39c'
size = 542528576
check = json.loads((dest/'terminal-archive-verify-v1/verification.json').read_text())
assert check['status'] == 'pass'
assert check['archive_sha256'] == digest and check['archive_bytes'] == size
assert archive.stat().st_size == size
with archive.open('rb') as stream:
    archive_hash = hashlib.sha256()
    for chunk in iter(lambda:stream.read(1024*1024),b''):
        archive_hash.update(chunk)
    assert archive_hash.hexdigest() == digest
assert not receipt.exists(), 'Reconcile the existing publication receipt before repeating'
dest.mkdir(exist_ok=True)
source_bytes = Path(__file__).read_bytes()
(dest/'terminal-store-publication-script-v2.py').write_bytes(source_bytes)
out = {'schema':'joint-terminal-raw-store-publication/v1',
       'recorded_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),
       'scope':'Canonical native artifact registration of the 22-directory terminal checkpoint only',
       'archive_sha256':digest,'archive_bytes':size,'store_name':name,
       'infrastructure_script_sha256':hashlib.sha256(source_bytes).hexdigest(),
       'python_version':sys.version.split()[0]}
def save():
    receipt.write_text(json.dumps(out,indent=2)+'\n')
def call(argv, env, limit=120):
    p = subprocess.run(argv, env=env, cwd=root, capture_output=True, text=True, timeout=limit)
    m = re.search(r'An error occurred \(([^)]+)\)', p.stderr)
    return p, m.group(1) if m else ('none' if p.returncode == 0 else 'command-failed')
print('READY: hidden credential input; public integrity checks passed.',flush=True)
pair = json.loads(sys.stdin.readline())
assert len(pair) == 2
env = os.environ.copy()
for key in ('AWS_PROFILE','AWS_SESSION_TOKEN','AWS_SECURITY_TOKEN','AWS_ACCESS_KEY_ID','AWS_SECRET_ACCESS_KEY'):
    env.pop(key,None)
env.update(AWS_ACCESS_KEY_ID=pair[0],AWS_SECRET_ACCESS_KEY=pair[1],
           AWS_REGION='us-east-1',AWS_DEFAULT_REGION='us-east-1',AWS_PAGER='',
           AWS_EC2_METADATA_DISABLED='true',CRYPTO_STORE_REMOTE='s3://crypto-autoresearcher/store')
pair = None
try:
    sts,code = call(['aws','sts','get-caller-identity','--output','json'],env)
    out.update(sts_exit=sts.returncode,sts_error_code=code)
    assert sts.returncode == 0
    out['account'] = json.loads(sts.stdout)['Account']
    assert out['account'] == '590183823895'
    save()
    cli = str(root/'target/release/ecbench')
    info,code = call([cli,'store','info',name],env)
    if info.returncode == 0:
        registered = json.loads(info.stdout)
        assert registered['sha256'] == digest and registered['entry']['bytes'] == size
        out['reconciled_existing_native_index'] = registered
        out['push_exit'] = 0
        out['upload_repeated'] = False
    else:
        push,code = call([cli,'store','push',str(archive),'--name',name,
                         '--description','Immutable selected-terminal checkpoint: 297 joint S5 constructions and reconciled regression/proof attempts; 22 literal run directories only',
                         '--provenance','pack-terminal-checkpoint.sh at 59955b238604fb3656c901ad4caaa615f2087ad0; fresh extraction/cmp pass; SHA-256 '+digest,'--json'],env,1200)
        out.update(push_exit=push.returncode,push_error_code=code)
        save()
        assert push.returncode == 0
        out['native_receipt'] = json.loads(push.stdout)
    save()
    verify,code = call([cli,'store','verify',name,'--remote','--json'],env,600)
    out.update(verify_exit=verify.returncode,verify_error_code=code)
    if verify.returncode == 0:
        out['verify_receipt'] = {'stdout':verify.stdout,'stderr':verify.stderr}
    save()
    assert verify.returncode == 0
    head,code = call(['aws','s3api','head-object','--bucket','crypto-autoresearcher',
                      '--key','store/sha256/f4/'+digest,'--checksum-mode','ENABLED','--output','json'],env)
    out.update(head_exit=head.returncode,head_error_code=code)
    assert head.returncode == 0
    h = json.loads(head.stdout)
    assert h['ContentLength'] == size
    if h.get('ChecksumSHA256'):
        assert base64.b64decode(h['ChecksumSHA256']).hex() == digest
        out['head_digest_verified_via'] = 'SHA256-object-checksum'
    else:
        assert h.get('Metadata',{}).get('sha256') == digest
        out['head_digest_verified_via'] = 'sha256-metadata'
    out['head_receipt'] = {key:h[key] for key in ('ContentLength','Metadata','ChecksumSHA256','ETag') if key in h}
    out['status'] = 'pass'
    save()
    print(json.dumps({'status':out['status'],'store_name':name,'bytes':size,'sha256':digest}),flush=True)
except Exception as exc:
    out['status'] = 'interrupted-or-failed'
    out['error_type'] = type(exc).__name__
    save()
    print(json.dumps({'status':out['status'],'error_type':type(exc).__name__}),flush=True)
    raise SystemExit(2)
finally:
    env = None
