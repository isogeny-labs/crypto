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
    assert a['schema']=='signed-kummer-combined-admission/v1' and a['released'] is True
    assert a['total_seconds']==2400 and a['build_phase_seconds']==1800 and a['execution_phase_seconds']==600
    assert a['cpu']==2 and a['memory_bytes']==16*1024**3 and a['swap_bytes']==0
    assert a['attempts']==1 and a['automatic_retry'] is False and a['prior_v4_terminal_confirmed'] is True
    assert isinstance(a['absolute_deadline'],str) and a['absolute_deadline'], 'absolute deadline binding pending'
    import datetime
    deadline=datetime.datetime.fromisoformat(a['absolute_deadline'].replace('Z','+00:00'))
    assert deadline.tzinfo is not None and (deadline-datetime.datetime.now(datetime.timezone.utc)).total_seconds()>0
    assert a['campaign_ceiling_seconds']==129600
    assert set(a['generated_locks'])=={'driver','adapter'}
    for lock in a['generated_locks'].values():
        assert isinstance(lock['path'],str) and Path(lock['path']).is_absolute(), 'generated lock absent'
        assert type(lock['bytes']) is int and lock['bytes']>0
        assert re.fullmatch('[a-f0-9]{64}',lock['sha256'] or ''), 'generated lock hash absent'
        assert Path(lock['path']).stat().st_size==lock['bytes']
        assert hashlib.sha256(Path(lock['path']).read_bytes()).hexdigest()==lock['sha256']
    assert re.fullmatch('[a-f0-9]{40}',a['published_source_revision'] or ''), 'source publication pending' 
    assert type(a['campaign_charged_seconds']) is int and type(a['campaign_remaining_seconds']) is int
    assert a['campaign_charged_seconds']>=0 and a['campaign_charged_seconds']+a['campaign_remaining_seconds']==129600
    assert a['campaign_remaining_seconds']>=2400
    assert set(a['lease'])=={'task_id','attempt_id','lease_id','fencing_epoch'}
    assert type(a['lease']['fencing_epoch']) is int and a['lease']['fencing_epoch']>0
    assert len(a['exact_tests'])==2 and len(set(a['exact_tests']))==2
    commands=json.loads((p/'commands.json').read_text());assert a['exact_tests']==commands['exact_tests']
    assert a['uid']==a['gid']==1000
    assert a['toolchain']=='/home/ec2-user/.rustup/toolchains/stable-x86_64-unknown-linux-gnu'
    for name,digest in a['packet_files'].items():
        q=p/name;assert not Path(name).is_absolute() and '..' not in Path(name).parts
        assert hashlib.sha256(q.read_bytes()).hexdigest()==digest,name
    required={'durable_io.py','durable-launcher.py','durable-worker.py','materialize.py','commands.json','overlays.json','expected-source-manifest.json','driver-inputs.json','stage_driver.py','panel_acceptance.py'}
    assert required<=set(a['packet_files'])
    assert set(a['release_receipts'])=={'prior_v4_terminal','fresh_cache_snapshot','ledger','lease_readback','independent_review','publication','lock_publication','campaign_authorization'}
    for name,row in a['release_receipts'].items():
        q=Path(row['path']);assert q.is_absolute() and hashlib.sha256(q.read_bytes()).hexdigest()==row['sha256'],name
    pub=json.loads(Path(a['release_receipts']['publication']['path']).read_text());assert pub['verified_github_commit']==a['published_source_revision']
    lockpub=json.loads(Path(a['release_receipts']['lock_publication']['path']).read_text())
    assert lockpub['verified_github_commit'] and set(lockpub['locks'])=={'driver','adapter'}
    for kind,lock in a['generated_locks'].items():
        assert lockpub['locks'][kind]['sha256']==lock['sha256'] and lockpub['locks'][kind]['bytes']==lock['bytes']
    authorization=json.loads(Path(a['release_receipts']['campaign_authorization']['path']).read_text())
    assert authorization['absolute_deadline']==a['absolute_deadline'] and authorization['worker_seconds']==129600
    assert a['fresh_cache_manifest']['sha256'] and a['fresh_cache_manifest']['path']
    assert a['source_manifest_sha256']==hashlib.sha256((p/'expected-source-manifest.json').read_bytes()).hexdigest()
    assert a['immutable_manifest']['sha256']=='c5178d2138d79f1ea8cac1fc96cb8d968913ba403d68ee66fa36afdd60fdf3f2'
    assert a['workspace']=='/var/tmp/pencil-signed-kummer-combined-v2-20261111'
    assert a['durable_run']=='/home/ec2-user/pencil-validation-receipts/signed-kummer-combined-v2-20261111'
    assert a['unit']=='pencil-signed-kummer-combined-v2-20261111.service'
    assert a['limits']=={'target_stop_threshold_bytes':6*1024**3,'workspace_stop_threshold_bytes':8*1024**3,'logs_stop_threshold_bytes':256*1024**2,'minimum_free_bytes':6*1024**3,'durable_metadata_stop_bytes':64*1024**2,'durable_total_stop_bytes':384*1024**2,'supervisor_permit_seconds':10}
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
