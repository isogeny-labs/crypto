"""Wait for the existing grid, validate/freeze it, then resume idle W3b tests.

No generator or cost cell is launched here. JSON configuration pins reviewed
paths, source/binary hashes and the isolated checkout's initial Git commit.
"""
from pathlib import Path
import datetime
import fcntl
import hashlib
import json
import os
import shutil
import signal
import subprocess
import sys
import time

def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()

def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def complete_gate(state, count, generator_live):
    return state.get('status') == 'generation_complete' and count == 52 and not generator_live

def processes(root, generator_only=False):
    found = []
    for proc in Path('/proc').iterdir():
        if not proc.name.isdigit() or int(proc.name) == os.getpid():
            continue
        try:
            args = (proc/'cmdline').read_bytes().split(b'\0')
            if os.readlink(proc/'cwd') == str(root):
                if not generator_only or (args and b'diem_quartic_study' in args[0] and b'gen' in args):
                    found.append(int(proc.name))
        except (FileNotFoundError, PermissionError, ProcessLookupError):
            pass
    return found

def main(config_path):
    cfg = json.loads(Path(config_path).read_text())
    root, freeze, w3b = (Path(cfg[k]) for k in ['root', 'freeze', 'w3b'])
    base = root/'research/diem_quartic_relations_20261018/resume-20261019'
    lock = (base/'.freeze-followup.lock').open('a+')
    fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
    out = base/('freeze-followup-'+datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%SZ'))
    out.mkdir()
    state = {'started_at':now(), 'pid':os.getpid(), 'status':'waiting_for_grid',
             'steps':[], 'output':str(out), 'config_sha256':digest(config_path)}

    def save():
        for path in [out/'state.json', base/'latest-freeze-followup.json']:
            temp = path.with_suffix('.tmp')
            with temp.open('w') as f:
                json.dump(state, f, indent=2); f.flush(); os.fsync(f.fileno())
            os.replace(temp, path)
            fd = os.open(path.parent, os.O_RDONLY)
            try: os.fsync(fd)
            finally: os.close(fd)

    def command(name, args, cwd, seconds=120, cores=None, output=None):
        if args[0] == 'conductor':
            pos = 3 if args[1] == 'task' else 2
            args = list(args[:pos]) + ['--project', cfg['conductor_project']] + list(args[pos:])
        row = {'name':name,'command':list(map(str,args)),'started_at':now(),'timeout_seconds':seconds}
        state['steps'].append(row)
        target = Path(output) if output else out/(name+'.log')
        prefix = ['taskset','-c',cores,'nice','-n','10'] if cores else []
        with target.open('wb') as f:
            p = subprocess.Popen(prefix+list(map(str,args)), cwd=cwd,
                env=dict(os.environ,RAYON_NUM_THREADS='1',CONDUCTOR_ENDPOINT=cfg['conductor_endpoint']), stdin=subprocess.DEVNULL,
                stdout=f, stderr=subprocess.STDOUT, start_new_session=True)
            row['pid']=p.pid; save()
            try: code=p.wait(timeout=seconds)
            except subprocess.TimeoutExpired:
                os.killpg(p.pid,signal.SIGTERM)
                try: p.wait(timeout=10)
                except subprocess.TimeoutExpired:
                    os.killpg(p.pid,signal.SIGKILL); p.wait()
                code=124; row['timed_out']=True
        row.update(exit_code=code,finished_at=now()); save()
        if code: raise RuntimeError(f'{name} exited {code}; log retained at {target}')

    def git(*args):
        return subprocess.check_output(['git','-C',str(freeze),*args],text=True).strip()

    def check_sources():
        for path, sha in cfg['source_hashes'].items():
            if digest(root/path) != sha: raise RuntimeError('generator source changed: '+path)
        if digest(cfg['validator']) != cfg['validator_sha256']:
            raise RuntimeError('validator binary changed')

    save()
    try:
        check_sources()
        deadline=time.monotonic()+86400
        absent=0
        while True:
            producer=json.loads((base/'latest-state.json').read_text())
            records=list((root/'research/diem_quartic_relations_20261018/results/instance-checkpoints-v1').glob('k*.json'))
            live=processes(root,True)
            state.update(durable_records=len(records),generator_pids=live,last_checked_at=now()); save()
            if complete_gate(producer,len(records),bool(live)): break
            if producer.get('status')=='failed': raise RuntimeError('producer failed; no frozen commit or downstream tests launched')
            absent=absent+1 if not live else 0
            if absent>=6: raise RuntimeError('generator absent without complete receipt for one minute')
            if time.monotonic()>deadline: raise RuntimeError('24-hour continuation wait exhausted')
            time.sleep(10)
        check_sources()
        command('freeze-conflict-check',['conductor','check','--summary','Validate and commit complete Diem inputs','--scope','dir:'+str(freeze)],root)
        command('freeze-claim',['conductor','task','claim',cfg['freeze_task']],root)
        if git('rev-parse','HEAD') != cfg['freeze_base'] or git('status','--porcelain'):
            raise RuntimeError('isolated freeze checkout changed; inspect before committing')
        state['status']='validating_complete_grid'; save()
        rel=Path('research/diem_quartic_relations_20261018/results')
        files=[root/rel/'instances.jsonl',root/rel/'FROZEN.sha256',
               root/rel/'instance-checkpoints-v1/manifest.json']+sorted(records)
        for src in files:
            dest=freeze/src.relative_to(root);dest.parent.mkdir(parents=True,exist_ok=True)
            with dest.open('xb') as f:
                f.write(src.read_bytes());f.flush();os.fsync(f.fileno())
        report=freeze/'research/diem_quartic_relations_20261018/resume-validation-20261019/frozen-grid-validation.json'
        command('validate-complete-grid',[cfg['validator'],freeze/rel],freeze,600,'24,25',report)
        valid=json.loads(report.read_text())
        if valid.get('status')!='COMPLETE_RECEIPTS_VALID' or valid.get('validated')!=52:
            raise RuntimeError('validator did not certify all 52 receipts')
        # The native checker holds the output lock and verifies the final list
        # against all records. Recheck the final bytes immediately before Git.
        if digest(freeze/rel/'instances.jsonl')!=valid['instances_sha256']:
            raise RuntimeError('validated final list changed')
        paths=[str(p.relative_to(root)) for p in files]+[str(report.relative_to(freeze))]
        git('add','--sparse','--',*paths)
        command('commit-frozen-grid',['git','commit','-m','Freeze all 52 validated Diem instances and preserve durable receipts'],freeze)
        state.update(status='frozen',freeze_commit=git('rev-parse','HEAD'),instances_sha256=valid['instances_sha256']);save()
        try:
            command('publish-frozen-grid',['git','push','origin','HEAD:refs/heads/'+cfg['branch']],freeze,180)
            state['publication']='pushed'
        except Exception as exc:
            state['publication']='commit_preserved_push_failed';state['publication_error']=str(exc)
        save()
        # Advance only this named, previously interrupted lane, after a fresh
        # ownership/process/source check. No study measurements are repeated.
        if processes(w3b): raise RuntimeError('W3b already has active processes; preserving existing ownership')
        if digest(w3b/'src/cryptanalysis/smith_isogeny.rs')!=cfg['w3b_source_sha256']:
            raise RuntimeError('W3b source changed; existing work needs reconciliation')
        if digest(cfg['w3b_binary'])!=cfg['w3b_binary_sha256']:
            raise RuntimeError('W3b validation binary changed')
        command('w3b-conflict-check',['conductor','check','--summary','Resume interrupted Smith divisor-map validation','--scope','dir:'+str(w3b)],w3b)
        command('w3b-claim',['conductor','task','claim',cfg['w3b_task']],w3b)
        state['status']='validating_w3b';save()
        tests=['example1_quartic_and_counts','example1_pic_group_law','example1_verify','debug_p7']
        failures=[]
        for test in tests:
            if processes(w3b): raise RuntimeError('W3b process appeared; refusing duplicate validation')
            command('w3b-renew-'+test,['conductor','task','claim',cfg['w3b_task']],root)
            try:
                command('w3b-'+test,[cfg['w3b_binary'],'--exact','cryptanalysis::smith_isogeny::tests::'+test,
                        '--nocapture','--test-threads=1'],w3b,120 if test=='debug_p7' else 600,'24,25')
            except Exception as exc:
                failures.append({'test':test,'error':str(exc)})
        state.update(status='w3b_validation_passed' if not failures else 'w3b_validation_needs_attention',w3b_failures=failures)
    except Exception as exc:
        state.update(status='needs_attention',error=str(exc))
    finally:
        state['finished_at']=now();save()

if __name__=='__main__':
    main(sys.argv[1])
