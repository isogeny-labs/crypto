#!/usr/bin/env python3
"""Bounded independent storage fault fixtures. No materializer top-level executes."""
import argparse, ast, hashlib, importlib.util, json, os, signal, time, types
from pathlib import Path
from unittest.mock import patch

SOURCE_SHA = '6bfc278cdfb8cd5f2adad13f5e920a2baee131be9f7dedc4b472329f2f0da4f7'
READER_SHA = '021215e4b46f84c63e0b297acc427fce244bbe675647fe13b88eb60a0326e2e9'
BASE_SHA = '085234b563a2c766040706ed7d805b2971074efba14da022265a6aa6db33ca63'
NAMES = ('record_cleanup', 'checkpoint')

def checked(path, digest):
    raw = path.read_bytes()
    assert len(raw) <= 1024 * 1024 and hashlib.sha256(raw).hexdigest() == digest
    return raw

def functions(raw):
    tree = ast.parse(raw)
    nodes = [node for node in tree.body if isinstance(node, ast.FunctionDef) and node.name in NAMES]
    assert [node.name for node in nodes] == list(NAMES)
    return ast.Module(body=nodes, type_ignores=[])

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--output', type=Path, required=True)
    args = ap.parse_args()
    here = Path(__file__).resolve().parent
    base = here.parent
    source = base / 'materializer-admission-20261109/materialize.py'
    reader_path = base / 'materializer-journal-reader-20261107/receipt_reader.py'
    code = functions(checked(source, SOURCE_SHA))
    old = functions(checked(base / 'materializer-source-successor-20261107/materialize.py', BASE_SHA))
    assert ast.dump(code) == ast.dump(old)
    checked(reader_path, READER_SHA)
    spec = importlib.util.spec_from_file_location('reviewed_reader', reader_path)
    reader = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(reader)
    assert args.output.is_absolute() and not args.output.exists()
    args.output.mkdir(mode=0o700)
    started = time.monotonic()
    rows = []
    class WorkDeadline(BaseException):
        pass
    def timed_out(*_):
        raise WorkDeadline('qualification 50-second work bound')
    signal.signal(signal.SIGALRM, timed_out)
    signal.alarm(50)
    def setup(name):
        out = args.output / name
        out.mkdir()
        env = dict(hashlib=hashlib, json=json, os=types.SimpleNamespace(**{k:getattr(os,k) for k in dir(os)}),
                   time=time, OUT=out, OWN=True, START=started, CLEANUP_HASH=hashlib.sha256(), CLEANUP_BYTES=0,
                   R={'status':'failed','errors':[]})
        exec(compile(code, str(source), 'exec'), env)
        return env, out
    def verify(out):
        result = reader.verify(out)
        assert not result['terminal_owner_authenticated'] and not result['terminal_established']
        assert not result['cleanup_semantics_verified']
        return result
    first = {'pid':1, 'errors':[], 'synthetic':True}
    second = {'pid':2, 'errors':[], 'synthetic':True}
    def case(name, test):
        env, out = setup(name)
        try:
            details = test(env, out)
            rows.append({'case':name,'passed':True,'details':details})
        except Exception as error:
            rows.append({'case':name,'passed':False,'error':repr(error)})
    def normal(e,o):
        e['record_cleanup'](first); e['checkpoint']()
        r = verify(o); assert r['committed_records']==1 and r['consistency']=='storage_incomplete'
        e['record_cleanup'](second); e['R']['status']='passed'; e['checkpoint'](True)
        r = verify(o); assert r['committed_records']==2 and r['consistency']=='storage_complete'
        return r
    case('normal_intermediate_and_final', normal)
    def tail(e,o):
        e['record_cleanup'](first); e['checkpoint']()
        with (o/'child-cleanup.jsonl').open('ab') as f: f.write(b'{"partial":')
        r=verify(o); assert r['committed_records']==1 and r['unreferenced_tail_bytes']==11
        return r
    case('interrupted_unacknowledged_tail',tail)
    def fsync_fault(e,o,number,prior):
        if prior: e['record_cleanup'](first); e['checkpoint']()
        real=e['os'].fsync; calls=0
        def fail(fd):
            nonlocal calls
            calls+=1
            if calls==number: raise OSError('injected fsync failure')
            return real(fd)
        e['os'].fsync=fail
        try:
            try: e['record_cleanup'](second)
            except OSError: pass
            else: raise AssertionError('fault did not raise')
        finally: e['os'].fsync=real
        assert e['CLEANUP_BYTES']==(len((json.dumps(first,separators=(',',':'))+'\n').encode()) if prior else 0)
        e['checkpoint'](True)
        r=verify(o); assert r['consistency']=='storage_failed' and r['committed_records']==int(prior)
        return r
    case('first_file_fsync_failure',lambda e,o:fsync_fault(e,o,1,False))
    case('first_directory_fsync_failure',lambda e,o:fsync_fault(e,o,2,False))
    case('append_fsync_failure',lambda e,o:fsync_fault(e,o,1,True))
    def write_fault(e,o,kind):
        e['record_cleanup'](first); e['checkpoint']()
        original=Path.open
        class Stream:
            def __init__(self,s): self.s=s
            def __enter__(self): self.s.__enter__(); return self
            def __exit__(self,*a): return self.s.__exit__(*a)
            def write(self,b):
                if kind=='write': raise OSError('injected write failure')
                if kind=='short': return self.s.write(b[:3])
                return self.s.write(b)
            def flush(self):
                if kind=='flush': raise OSError('injected flush failure')
                return self.s.flush()
            def fileno(self): return self.s.fileno()
        def opening(p,*a,**k):
            s=original(p,*a,**k)
            return Stream(s) if p.name=='child-cleanup.jsonl' else s
        with patch.object(Path,'open',opening):
            try:e['record_cleanup'](second)
            except (OSError,AssertionError):pass
            else:raise AssertionError('fault did not raise')
        assert e['R']['child_cleanup_journal']['records']==1
        e['checkpoint'](True)
        r=verify(o); assert r['consistency']=='storage_failed' and r['inline_records']==2 and r['committed_records']==1
        return r
    for kind in ('write','short','flush'):case('append_'+kind+'_failure',lambda e,o,k=kind:write_fault(e,o,k))
    def replace_fault(e,o):
        e['record_cleanup'](first); e['checkpoint'](); old=(o/'receipt.json').read_bytes()
        e['record_cleanup'](second)
        def fail(*a):raise OSError('injected replace failure')
        e['os'].replace=fail
        try:e['checkpoint']()
        except OSError:pass
        else:raise AssertionError('fault did not raise')
        assert (o/'receipt.json').read_bytes()==old
        r=verify(o); assert r['committed_records']==1 and r['unreferenced_tail_bytes']>0
        return r
    case('checkpoint_replace_failure',replace_fault)
    def reject_corruption(e,o):
        e['record_cleanup'](first); e['R']['status']='passed';e['checkpoint'](True)
        p=o/'child-cleanup.jsonl';p.write_bytes(p.read_bytes().replace(b'"pid":1',b'"pid":9'))
        try:reader.verify(o)
        except reader.InvalidReceipt:return {'rejected':True}
        raise AssertionError('corruption accepted')
    case('acknowledged_prefix_corruption_rejected',reject_corruption)
    case('normal_repeat_control', normal)
    def before_append(e,o):
        e['checkpoint']()
        r=verify(o); assert r['committed_records']==0 and not r['journal_verified']
        return r
    case('before_first_append_interruption',before_append)
    def durable_uncheckpointed(e,o):
        e['record_cleanup'](first); e['checkpoint']()
        e['record_cleanup'](second)
        r=verify(o); assert r['committed_records']==1 and r['unreferenced_tail_bytes']>0
        return r
    case('durable_append_before_checkpoint',durable_uncheckpointed)
    def checkpoint_dir_fsync(e,o):
        e['record_cleanup'](first); e['checkpoint'](); e['record_cleanup'](second)
        real=e['os'].fsync; calls=0
        def fail(fd):
            nonlocal calls
            calls+=1
            if calls==2:raise OSError('injected checkpoint directory fsync failure')
            return real(fd)
        e['os'].fsync=fail
        try:e['checkpoint']()
        except OSError:pass
        else:raise AssertionError('fault did not raise')
        finally:e['os'].fsync=real
        r=verify(o); assert r['committed_records']==2 and r['consistency']=='storage_incomplete'
        return dict(r, persistence_after_crash_established=False)
    case('checkpoint_directory_fsync_failure',checkpoint_dir_fsync)
    def pointer_count(e,o):
        e['record_cleanup'](first); e['checkpoint']()
        p=o/'receipt.json';r=json.loads(p.read_text());r['child_cleanup_journal']['records']=2
        p.write_text(json.dumps(r))
        try:reader.verify(o)
        except reader.InvalidReceipt:return {'rejected':True}
        raise AssertionError('count mismatch accepted')
    case('pointer_count_mismatch_rejected',pointer_count)
    signal.alarm(0)
    result={'schema':'materializer.storage-fault-qualification/v1','source_sha256':SOURCE_SHA,'reader_sha256':READER_SHA,
            'elapsed_seconds':time.monotonic()-started,'cases':rows,'passed':len(rows)==15 and all(r['passed'] for r in rows),
            'materializer_executed':False,'compiler_executed':False,'power_loss_durability_established':False,
            'terminal_custody_established':False,'semantics':'bounded deterministic injected storage faults; not OS/power-loss crash testing'}
    (args.output/'result.json').write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps(result,sort_keys=True))
    return 0 if result['passed'] else 1

if __name__=='__main__':raise SystemExit(main())
