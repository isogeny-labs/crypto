"""Unexecuted prospective durable receipt/admission helpers."""
import hashlib,json,math,os,tempfile,re
from pathlib import Path

def atomic_json(path,value):
    path=Path(path);raw=(json.dumps(value,indent=2,allow_nan=False)+'\n').encode()
    if len(raw)>16*1024**2:raise RuntimeError('JSON receipt size cap')
    fd,name=tempfile.mkstemp(prefix='.'+path.name+'.',dir=path.parent)
    try:
        with os.fdopen(fd,'wb') as f:f.write(raw);f.flush();os.fsync(f.fileno())
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
            before=os.fstat(f.fileno());remaining=before.st_size;h=hashlib.sha256()
            while remaining:
                chunk=f.read(min(1024**2,remaining))
                if not chunk:raise RuntimeError('log truncated during prefix snapshot')
                remaining-=len(chunk);h.update(chunk)
            # Flush after hashing the fixed prefix; later appends are excluded.
            os.fsync(f.fileno());after=os.fstat(f.fileno())
            if after.st_size<before.st_size:raise RuntimeError('log truncated')
        rows[path.name]={'prefix_bytes':before.st_size,'prefix_sha256':h.hexdigest(),'sealed':False}
    atomic_json(Path(directory)/'log-prefix-checkpoint.json',rows)
    return rows

def finite_number(value):
    return type(value) in (int,float) and math.isfinite(value)

def load_admission(packet):
    p=Path(packet);a=json.loads((p/'admission.json').read_text())
    assert a['schema']=='durable-root-library-admission/owner-waiver-v3' and a['admitted'] is True
    recipe=json.loads((p/'recipe-proposal.json').read_text())
    for name in ['limits','uid','gid','exact_tests','attempts','automatic_retry','launch_command']:
        assert a[name]==recipe[name],name
    assert a['attempts']==1 and a['automatic_retry'] is False
    assert len(a['exact_tests'])==4 and len(set(a['exact_tests']))==4
    waiver=a['owner_dollar_waiver']
    assert waiver['scope']=='dollar-accounting-admission-only'
    assert waiver['approved'] is True
    for n,digest in a['recipe_files'].items():
        assert Path(n).name==n and hashlib.sha256((p/n).read_bytes()).hexdigest()==digest,n
    required={'durable_io.py','durable-launcher.py','durable-worker.py','source-bindings.json','fixture.json','candidate.patch','recipe-proposal.json','materialized-inputs.json'}
    assert required<=set(a['recipe_files'])
    for field,obj in [('publication',a['publication']),('owner_dollar_waiver',waiver)]:
        path=p/obj['receipt_file'];assert path.parent==p
        assert hashlib.sha256(path.read_bytes()).hexdigest()==obj['receipt_sha256']
    pub=a['publication'];assert pub['verified_github_commit']
    approval=json.loads((p/waiver['receipt_file']).read_text())
    assert approval['approved'] is True and approval['scope']=='dollar-accounting-admission-only'
    assert approval['owner_instruction']=='ignore dollar facts and run'
    # Parent supplies the exact recorded instruction receipt; no dollar values invented.
    assert a['absolute_deadline']=='2026-11-06T04:00:00+00:00'
    assert a['campaign_remaining_worker_seconds']>=a['limits']['total_seconds']
    assert type(a['campaign_remaining_worker_seconds']) is int
    assert a['campaign_remaining_worker_seconds']<=46500
    assert a['inputs_sha256']==a['recipe_files']['materialized-inputs.json']
    assert set(a['lease'])=={'task_id','attempt_id','lease_id','fencing_epoch'}
    assert type(a['lease']['fencing_epoch']) is int and a['lease']['fencing_epoch']>0
    assert a['absolute_deadline']
    w=Path(a['workspace']);d=Path(a['durable_run'])
    assert w.is_absolute() and d.is_absolute() and not w.is_relative_to(d) and not d.is_relative_to(w)
    assert str(d).startswith('/home/ec2-user/pencil-validation-receipts/')
    assert a['unit'].startswith('pencil-root-durable-') and a['unit'].endswith('.service')
    return a

def durable_limits(directory,limits):
    total=0;metadata=0
    for path in Path(directory).rglob('*'):
        if path.is_dir():continue
        if path.is_symlink():raise RuntimeError('symlink in output')
        try:n=path.stat().st_size
        except FileNotFoundError:
            # Only our atomic checkpoint temp names may disappear mid-enumeration.
            if re.fullmatch(r'\.(?:launch-receipt|worker-receipt|log-prefix-checkpoint|supervisor-permit|terminal-output-manifest)\.json\.[A-Za-z0-9_-]+',path.name):continue
            raise
        total+=n
        if not path.name.endswith(('.stdout','.stderr')):metadata+=n
    if total>limits['durable_total_stop_bytes'] or metadata>limits['durable_metadata_stop_bytes']:
        raise RuntimeError('durable output size stop threshold')
    return {'total_bytes':total,'metadata_bytes':metadata}

def seal_outputs(directory):
    directory=Path(directory);rows={}
    for path in sorted(directory.rglob('*')):
        if path.is_dir():continue
        if path.is_symlink():raise RuntimeError('symlink in durable output')
        if path.name=='terminal-output-manifest.json':continue
        with path.open('rb') as stream:
            h=hashlib.sha256();size=0
            for chunk in iter(lambda:stream.read(1024**2),b''):h.update(chunk);size+=len(chunk)
            os.fsync(stream.fileno())
        rows[str(path.relative_to(directory))]={'bytes':size,'sha256':h.hexdigest()}
    atomic_json(directory/'terminal-output-manifest.json',{'schema':'durable-output-manifest/v1','sealed_after_owned_cgroup_empty':True,'files':rows})
