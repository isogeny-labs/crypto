"""Artifact infrastructure only. Credentials enter hidden stdin, never receipts."""
import base64
import datetime
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys

root = Path(__file__).resolve().parents[3]
study = Path(__file__).resolve().parent
packed = Path('/home/ec2-user/scratch/twist-zero-complete-archive-v2')
archive = packed / 'runs.tar.xz'
dest = study / 'results/publication'
attempt = sys.argv[1] if len(sys.argv) == 2 else 'v1'
assert attempt in ('v1', 'v2')
receipt = dest / ('store-publication-' + attempt + '.json')
name = 'evidence/twist-index-calculus/20261024-zero-fiber-runs.tar.xz'
cli = Path('/home/ec2-user/crypto-twist-ic/target/release/ecbench')
check = json.loads((study / 'results/archive-verification-v2.json').read_text())
assert check['status'] == 'pass'
assert check['byte_identical_permission_preserving_repack'] is True
assert check['native_file_comparisons'] == 1112
assert (study / 'results/pack-permission-repair-v1/exit').read_text().strip() == '0'
assert (study / 'results/pack-v1/exit').read_text().strip() == '1'
assert (study / 'results/pack-study-date-v2/exit').read_text().strip() == '0'
assert check['normalized_tar_date'] == '2026-10-24 00:00:00Z'
digest, size = check['archive_sha256'], check['archive_bytes']
assert size == archive.stat().st_size
with archive.open('rb') as stream:
    h = hashlib.sha256()
    for chunk in iter(lambda: stream.read(1024 * 1024), b''):
        h.update(chunk)
    assert h.hexdigest() == digest
assert not receipt.exists(), 'Reconcile saved publication before another attempt'
dest.mkdir(parents=True, exist_ok=True)
out = {
    'schema': 'zero-fiber-raw-store-publication/v1',
    'recorded_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
    'scope': 'Ten reconciled raw directories and frozen source/reference slice; publication only',
    'archive_sha256': digest, 'archive_bytes': size, 'store_name': name,
    'archive_verification': check,
    'infrastructure_script_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
    'native_store_cli_sha256': hashlib.sha256(cli.read_bytes()).hexdigest(),
    'python_version': sys.version.split()[0],
}


def save():
    receipt.write_text(json.dumps(out, indent=2) + '\n')


def call(argv, env, limit=120):
    p = subprocess.run(argv, env=env, cwd=root, capture_output=True, text=True, timeout=limit)
    match = re.search(r'An error occurred \(([^)]+)\)', p.stderr)
    return p, match.group(1) if match else ('none' if p.returncode == 0 else 'command-failed')


def remote_head(env):
    p, code = call(['aws', 's3api', 'head-object', '--bucket', 'crypto-autoresearcher',
                    '--key', 'store/sha256/' + digest[:2] + '/' + digest,
                    '--checksum-mode', 'ENABLED', '--output', 'json'], env)
    if p.returncode == 0:
        remote = json.loads(p.stdout)
        assert remote['ContentLength'] == size
        assert base64.b64decode(remote['ChecksumSHA256']).hex() == digest
        return p.returncode, code, {k: remote[k] for k in ('ContentLength', 'ChecksumSHA256', 'ETag') if k in remote}
    return p.returncode, code, None


print('READY: verified original payload; hidden credential input for existing canonical store.', flush=True)
pair = json.loads(sys.stdin.readline())
assert len(pair) == 2
env = os.environ.copy()
for key in ('AWS_PROFILE', 'AWS_SESSION_TOKEN', 'AWS_SECURITY_TOKEN', 'AWS_ACCESS_KEY_ID', 'AWS_SECRET_ACCESS_KEY'):
    env.pop(key, None)
env.update(AWS_ACCESS_KEY_ID=pair[0], AWS_SECRET_ACCESS_KEY=pair[1],
           AWS_REGION='us-east-1', AWS_DEFAULT_REGION='us-east-1', AWS_PAGER='',
           AWS_EC2_METADATA_DISABLED='true', CRYPTO_STORE_REMOTE='s3://crypto-autoresearcher/store',
           CRYPTO_STORE_INDEX=str(root / 'store/index.json'),
           CRYPTO_STORE_CACHE='/home/ec2-user/scratch/twist-zero-store-cache-v1')
pair = None
try:
    sts, code = call(['aws', 'sts', 'get-caller-identity', '--output', 'json'], env)
    out.update(sts_exit=sts.returncode, sts_error_code=code)
    assert sts.returncode == 0
    out['account'] = json.loads(sts.stdout)['Account']
    assert out['account'] == '590183823895'
    index_before = json.loads((root / 'store/index.json').read_text())
    prior = index_before['entries'].get(name)
    if prior is not None:
        assert prior['sha256'] == digest and prior['bytes'] == size
    out['existing_alias'] = prior
    head_exit, head_code, remote = remote_head(env)
    out.update(preflight_head_exit=head_exit, preflight_head_error_code=head_code,
               existing_remote_object=remote)
    assert head_exit == 0 or head_code in ('404', 'NotFound', 'NoSuchKey')
    save()
    backup = dest / ('store-index-before-' + attempt + '.json')
    assert not backup.exists()
    backup.write_text(json.dumps(index_before, indent=2) + '\n')
    if prior is None:
        push, code = call([str(cli), 'store', 'push', str(archive), '--name', name,
                           '--description', 'Complete 42-arm zero-fiber polynomial panel, independent pair/set/rank audits and preserved failed native/lock attempts; no full IC price',
                           '--provenance', 'pack-with-study-date.sh; preserved first permission mismatch and repair; study-date/mode-normalized archive; all 1112 byte comparisons and identical repack; producer ' + check['frozen_compiled_source_commit'] + '; SHA-256 ' + digest,
                           '--json'], env, 1200)
        out.update(push_exit=push.returncode, push_error_code=code)
        save()
        assert push.returncode == 0
        out['native_receipt'] = json.loads(push.stdout)
    else:
        out.update(push_exit=0, publication_already_complete=True)
    index_after = json.loads((root / 'store/index.json').read_text())
    entry = index_after['entries'][name]
    assert entry['sha256'] == digest and entry['bytes'] == size
    assert {k: v for k, v in index_before['entries'].items() if k != name} == {k: v for k, v in index_after['entries'].items() if k != name}
    out['unrelated_index_entries_unchanged'] = True
    out['existing_object_upload_repeated'] = False
    save()
    verify, code = call([str(cli), 'store', 'verify', name, '--remote', '--json'], env, 1200)
    out.update(verify_exit=verify.returncode, verify_error_code=code)
    if verify.returncode == 0:
        out['native_verify_receipt'] = {'stdout': verify.stdout, 'stderr': verify.stderr}
    save()
    assert verify.returncode == 0
    head_exit, head_code, remote = remote_head(env)
    out.update(head_exit=head_exit, head_error_code=head_code, head_receipt=remote)
    assert head_exit == 0
    out['head_digest_verified_via'] = 'SHA256-object-checksum'
    out['status'] = 'pass'
    save()
    print(json.dumps({'status': 'pass', 'store_name': name, 'bytes': size, 'sha256': digest}), flush=True)
except Exception as exc:
    out.update(status='interrupted-or-failed', error_type=type(exc).__name__)
    save()
    print(json.dumps({'status': out['status'], 'error_type': type(exc).__name__}), flush=True)
    raise SystemExit(2)
finally:
    env = None
