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
    assert a['schema']=='fixture-view-admission/v1' and a['admitted'] is True
    assert a['phase'] in ('materialize','suite')
    assert a['total_seconds']==(120 if a['phase']=='materialize' else 1800)
    assert a['attempts']==1 and not a['automatic_retry']
    assert a['campaign_ceiling_seconds']==43200 and a['campaign_remaining_seconds']>=a['total_seconds']
    assert a['absolute_deadline']=='2026-11-10T05:11:39Z'
    assert a['serial_predecessor_terminal_sha256'] and a['independent_review_sha256']
    assert a['publication']['verified_github_commit'] and a['publication']['readback_sha256']
    for n,digest in a['recipe_files'].items():
        assert Path(n).name==n and hashlib.sha256((p/n).read_bytes()).hexdigest()==digest
    assert {'durable-launcher.py','durable-worker.py','durable_io.py'}<=set(a['recipe_files'])
    assert a['lease'].keys()=={'task_id','attempt_id','lease_id','fencing_epoch'}
    assert a['cost_mode']=='owner-waived-dollar-facts/v1'
    if a['phase']=='suite':assert a['materialized_view_sha256'] and a['materialization_terminal_sha256']
    return a

def durable_limits(directory,limits):
    total=0;metadata=0
    for path in Path(directory).rglob('*'):
        if path.is_dir():continue
        if path.is_symlink():raise RuntimeError('symlink in output')
        try:n=path.stat().st_size
        except FileNotFoundError:
            # Only our atomic checkpoint temp names may disappear mid-enumeration.
            if re.fullmatch(r'\.(?:launch-receipt|worker-receipt|log-prefix-checkpoint|supervisor-permit|materialized-view|terminal-output-manifest)\.json\.[A-Za-z0-9_-]+',path.name):continue
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
