"""Prospective durable receipt primitives; importing this module runs no payload."""
import hashlib,json,os,tempfile
from pathlib import Path

def atomic_json(path, value):
    path=Path(path)
    # Caller must create and own the output directory before writing.
    fd,name=tempfile.mkstemp(prefix='.'+path.name+'.',dir=path.parent)
    try:
        with os.fdopen(fd,'w') as f:
            json.dump(value,f,indent=2);f.write('\n');f.flush();os.fsync(f.fileno())
        os.replace(name,path)
        d=os.open(path.parent,os.O_RDONLY|os.O_DIRECTORY)
        try:os.fsync(d)
        finally:os.close(d)
    finally:
        if os.path.exists(name):os.unlink(name)

def flush_logs(directory):
    rows={}
    for path in sorted(Path(directory).glob('*.std*')):
        if not path.is_file() or path.is_symlink():raise RuntimeError('unexpected log path')
        with path.open('rb') as f:
            os.fsync(f.fileno());h=hashlib.sha256();size=0
            for b in iter(lambda:f.read(1024**2),b''):h.update(b);size+=len(b)
        # Running files can grow; each row certifies this prefix only.
        rows[path.name]={'prefix_bytes':size,'prefix_sha256':h.hexdigest(),'sealed':False}
    atomic_json(Path(directory)/'log-prefix-checkpoint.json',rows)
    return rows

def load_admission(packet):
    p=Path(packet);a=json.loads((p/'admission.json').read_text())
    assert a['schema']=='durable-root-library-admission/v1' and a['admitted'] is True
    assert a['attempts']==1 and a['automatic_retry'] is False
    assert a['cost']['finite_dollar_ceiling_usd']>0
    pub=a['publication'];assert pub['verified_github_commit'] and pub['receipt_sha256']
    # The release must bind the exact previously published recipe, source,
    # inputs, fixture/seeds, launch command and this module.
    for n,digest in a['recipe_files'].items():
        assert Path(n).name==n and hashlib.sha256((p/n).read_bytes()).hexdigest()==digest,n
    assert {'durable_io.py','durable-launcher.py','durable-worker.py','source-bindings.json','fixture.json','candidate.patch','recipe-proposal.json'}<=set(a['recipe_files'])
    assert a['inputs_sha256'] and a['lease'] and a['absolute_deadline']
    assert set(a['lease'])=={'task_id','attempt_id','lease_id','fencing_epoch'}
    assert a['lease']['fencing_epoch']>0
    assert 'materialized-inputs.json' in a['recipe_files']
    assert a['recipe_files']['materialized-inputs.json']==a['inputs_sha256']
    receipt=p/pub['receipt_file']
    assert receipt.parent==p and hashlib.sha256(receipt.read_bytes()).hexdigest()==pub['receipt_sha256']
    w=Path(a['workspace']);d=Path(a['durable_run'])
    assert w.is_absolute() and d.is_absolute() and not w.is_relative_to(d) and not d.is_relative_to(w)
    assert str(d).startswith('/home/ec2-user/pencil-validation-receipts/')
    assert a['unit'].startswith('pencil-root-durable-') and a['unit'].endswith('.service')
    return a


def seal_outputs(directory):
    directory=Path(directory);rows={}
    for path in sorted(directory.rglob('*')):
        if path.is_dir():continue
        if path.is_symlink():raise RuntimeError('symlink in durable output')
        if path.name=='terminal-output-manifest.json':continue
        with path.open('rb') as stream:
            os.fsync(stream.fileno());h=hashlib.sha256();size=0
            for chunk in iter(lambda:stream.read(1024**2),b''):h.update(chunk);size+=len(chunk)
        rows[str(path.relative_to(directory))]={'bytes':size,'sha256':h.hexdigest()}
    atomic_json(directory/'terminal-output-manifest.json',{'schema':'durable-output-manifest/v1','sealed_after_owned_cgroup_empty':True,'files':rows})
