#!/usr/bin/env python3
"""Bounded, two-phase local CLI software validation; never builds or submits cloud work."""
import argparse
import datetime as dt
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import signal
import subprocess
import sys
import time

SCHEMA = "crypto.ml-dataset.koblitz-cli-smoke-driver/v1"
REQUIRED_SOURCE = {"Cargo.toml", "Cargo.lock", "src/lib.rs", "src/main.rs", "src/ml_dataset/mod.rs", "src/ml_dataset/solvers.rs", "src/ml_dataset/koblitz_m3.rs"}
REQUIRED_SOURCE |= {"src/ml_dataset/"+name+".rs" for name in ["types","factor_bases","curves","export","report","restore","storage","prime_chain_prefix","algebraic_targets","prime_f4_wide","prime_f4_wide_m3"]}
SLOTS = {"koblitz-m3-input.json", "koblitz-m3-events.jsonl", "koblitz-m3-result.json", "koblitz-m3-group-replay.json", "koblitz-m3-parent-delivery.json"}

def require(ok, reason):
    if not ok:
        raise RuntimeError(reason)

def stamp():
    return dt.datetime.now(dt.timezone.utc).isoformat().replace("+00:00", "Z")

def sha(path):
    h = hashlib.sha256()
    with Path(path).open("rb") as f:
        for block in iter(lambda: f.read(1024 * 1024), b""):
            h.update(block)
    return h.hexdigest()

def load(path):
    def unique(pairs):
        out={}
        for k,v in pairs:
            require(k not in out,'duplicate JSON key: '+k);out[k]=v
        return out
    return json.loads(Path(path).read_text(),object_pairs_hook=unique)

def save(path, obj):
    with Path(path).open("x") as f:
        json.dump(obj, f, indent=2, sort_keys=True, allow_nan=False)
        f.write("\n")
        f.flush()
        os.fsync(f.fileno())

def env():
    return {"PATH": "/usr/local/bin:/usr/bin:/bin", "HOME": str(Path.home()), "LANG": "C", "LC_ALL": "C", "RAYON_NUM_THREADS": "1", "OMP_NUM_THREADS": "1", "PYTHONDONTWRITEBYTECODE": "1"}

class DriverInterrupted(RuntimeError):
    pass


def interrupted(signum, _frame):
    raise DriverInterrupted(f"driver interrupted by signal {signum}")


def parse_time(value):
    require(isinstance(value, str), "timestamp must be an ISO8601 string")
    try:
        value = dt.datetime.fromisoformat(value.replace("Z", "+00:00"))
    except ValueError as exc:
        raise RuntimeError("malformed execution timestamp") from exc
    require(value.tzinfo is not None, "timestamp needs explicit timezone")
    return value


def output_belongs(argv, fixture):
    values = []
    for i, arg in enumerate(argv):
        if arg == "--out" and i + 1 < len(argv):
            values.append(argv[i + 1])
        elif arg.startswith("--out="):
            values.append(arg[len("--out="):])
    if len(values) != 1 or not Path(values[0]).is_absolute():
        return False
    output = Path(values[0]).resolve()
    root = (Path(fixture) / "run").resolve()
    return output == root or root in output.parents


def process_identity(pid):
    try:
        stat = (Path("/proc") / str(pid) / "stat").read_text().rsplit(")", 1)[1].split()
        if stat[0] == "Z":
            return None  # exited, waiting for its owner to reap; cannot generate
        return {"pid": pid, "process_group": int(stat[2]), "start_ticks": stat[19]}
    except (OSError, ValueError, IndexError):
        return None


def live_processes(binary, fixture):
    found = []
    for p in Path("/proc").iterdir():
        if not p.name.isdigit() or int(p.name) == os.getpid():
            continue
        try:
            args = [a.decode(errors="replace") for a in (p / "cmdline").read_bytes().split(b"\0") if a]
        except OSError:
            continue
        if args and args[0] == str(binary) and "dataset" in args and any(x in args for x in ["worker", "generate"]):
            identity = process_identity(int(p.name))
            if identity is not None:
                found.append({**identity, "argv": args, "belongs_to_fixture": output_belongs(args, fixture)})
    return found


def owned_session_members(child):
    if child is None:return []
    rows=[]
    for directory in Path('/proc').iterdir():
        if not directory.name.isdigit():continue
        try:
            fields=(directory/'stat').read_text().rsplit(')',1)[1].split()
            if fields[0]=='Z' or int(fields[3])!=child.pid:continue
            identity=process_identity(int(directory.name))
            if identity is not None:rows.append({**identity,'argv':['owned-session-member'],'belongs_to_fixture':True})
        except (OSError,ValueError,IndexError):continue
    return rows


def cleanup_owned(child, binary, fixture, grace=5.0, kill_wait=5.0):
    """Recheck exact output ownership and PID identity before every signal."""
    actions, errors = [], []
    for sig, seconds in [(signal.SIGTERM, grace), (signal.SIGKILL, kill_wait)]:
        until = time.monotonic() + seconds
        signalled = set()
        while True:
            owned = [p for p in live_processes(binary, fixture) if p["belongs_to_fixture"]] + owned_session_members(child)
            for p in owned:
                key = (p["pid"], p["start_ticks"])
                if key in signalled or process_identity(p["pid"]) != {k:p[k] for k in ["pid", "process_group", "start_ticks"]}:
                    continue
                try:
                    if p["process_group"] == p["pid"]:
                        os.killpg(p["pid"], sig)
                    else:
                        os.kill(p["pid"], sig)
                    actions.append({"pid":p["pid"],"start_ticks":p["start_ticks"],"signal":int(sig),"argv":p["argv"]})
                except ProcessLookupError:
                    pass
                except OSError as exc:
                    errors.append(str(exc))
                signalled.add(key)
            if child is not None and child.poll() is None:
                key = ("direct", child.pid)
                if key not in signalled:
                    try: os.killpg(child.pid, sig)
                    except ProcessLookupError: pass
                    except OSError as exc: errors.append(str(exc))
                    actions.append({"direct_child":child.pid,"signal":int(sig)})
                    signalled.add(key)
            remaining = [p for p in live_processes(binary, fixture) if p["belongs_to_fixture"]] + owned_session_members(child)
            direct_live = child is not None and child.poll() is None
            if not remaining and not direct_live:
                # A second scan after reaping closes the ordinary parent/worker
                # exit race; no status file alone is used as liveness evidence.
                remaining = [p for p in live_processes(binary, fixture) if p["belongs_to_fixture"]] + owned_session_members(child)
                if not remaining:
                    return {"settled":True,"actions":actions,"errors":errors,"unresolved":[],"direct_child_live":False}
            if time.monotonic() >= until:
                break
            time.sleep(0.05)
    return {"settled":False,"actions":actions,"errors":errors,"unresolved":remaining,"direct_child_live":direct_live}


class Runner:
    def __init__(self, fixture, binary, expected, expected_protocol):
        self.fixture = fixture
        self.binary = Path(binary)
        self.expected = expected
        self.expected_protocol = expected_protocol
        self.deadline = time.monotonic() + 1800
        self.commands = fixture / "commands"
        self.commands.mkdir(exist_ok=True)
        self.sequence = len(list(self.commands.iterdir()))
        self.last_cleanup = {"settled":True,"unresolved":[]}

    def command(self, name, argv, timeout=45, cwd=None, expected_error=None):
        require(time.monotonic() < self.deadline, "1800-second driver ceiling reached")
        before = sha(self.binary)
        require(before == self.expected, "validated binary changed before command")
        self.sequence += 1
        d = self.commands / name
        d.mkdir()
        argv = [str(a) for a in argv]
        save(d / "argv.json", argv)
        (d / "argv.nul").write_bytes(b"\0".join(os.fsencode(a) for a in argv) + b"\0")
        protocol = self.fixture / "protocol.json"
        protocol_before = sha(protocol) if protocol.exists() else None
        checker_hashes = {arg:sha(arg) for arg in argv if arg.endswith("/experiment_protocol.py") and Path(arg).is_file()}
        started = stamp()
        timed_out, child, interruption = False, None, None
        code, error = None, None
        cleanup = {"settled":True,"actions":[],"errors":[],"unresolved":[],"direct_child_live":False}
        handlers = {sig:signal.getsignal(sig) for sig in [signal.SIGINT, signal.SIGTERM]}
        for sig in handlers: signal.signal(sig, interrupted)
        try:
            with (d / "stdout").open("xb") as stdout, (d / "stderr").open("xb") as stderr:
                try:
                    require(protocol_before == self.expected_protocol and sha(protocol) == self.expected_protocol, "frozen protocol changed before command")
                    child = subprocess.Popen(argv, cwd=cwd or self.fixture, env=env(), stdout=stdout, stderr=stderr, start_new_session=True)
                    code = child.wait(timeout=min(timeout, max(0.01, self.deadline-time.monotonic())))
                except subprocess.TimeoutExpired:
                    timed_out, code = True, 124
                except (KeyboardInterrupt, DriverInterrupted) as exc:
                    interruption, error, code = type(exc).__name__, str(exc), 130
                except Exception as exc:
                    error, code = str(exc), 125
                finally:
                    for sig in handlers: signal.signal(sig, signal.SIG_IGN)
                    remaining = [p for p in live_processes(self.binary, self.fixture) if p["belongs_to_fixture"]] + owned_session_members(child)
                    if code != 0 or remaining:
                        if code == 0:
                            error, code = "successful command left live fixture workers", 125
                        cleanup = cleanup_owned(child, self.binary, self.fixture)
                    self.last_cleanup = cleanup
            after = sha(self.binary)
            receipt = {"argv":argv,"started_at":started,"finished_at":stamp(),"exit_code":code,
                       "timed_out":timed_out,"exception":error,"interruption":interruption,"cleanup":cleanup,
                       "protocol_sha256_before":protocol_before,"protocol_sha256_after":sha(protocol) if protocol.exists() else None,
                       "checker_sha256":checker_hashes,"binary_sha256_before":before,"binary_sha256_after":after,
                       "stdout_sha256":sha(d/"stdout"),"stderr_sha256":sha(d/"stderr")}
            save(d/"receipt.json", receipt)
        finally:
            for sig, old in handlers.items(): signal.signal(sig, old)
        require(cleanup["settled"], f"{name} failed with unresolved worker ownership; see {d}/receipt.json")
        require(after == self.expected, "binary changed during command")
        require(receipt["protocol_sha256_after"] == self.expected_protocol, "frozen protocol changed during command")
        if expected_error is None:
            require(code == 0, f"{name} failed ({code}); evidence retained at {d}; no retry")
        else:
            require(type(code) is int and 0 < code <= 255 and code not in [124, 125, 130] and not timed_out and error is None and interruption is None,
                    "negative command did not produce an ordinary terminal refusal")
            require(expected_error in (d/"stderr").read_text(), "negative command failed for the wrong reason")
        return d

    def git(self, name, source, *args):
        d = self.command(name, ["git", "-C", source, *args])
        return (d / "stdout").read_bytes()

# Fixed two-cell SAT initial-prefix and local full-custody acceptance.
ORDER=['plan','preflight','generate','verify','status','report','export','export-repeat','custody-create','custody-verify','custody-restore','restored-verify']
REPLAY=[n for n in ORDER if n not in ['plan','preflight','custody-verify']]
PLAN_FIELDS=['channels','schema','field_sizes','arities','families','solvers','curve_seeds','generators','keys','seed','factor_base_size','degree_bound','work_limit','twists','protocol_sha256','split_rule','factor_base_policies']

def file_inventory(root):
    rows=[]
    for p in sorted(root.rglob('*')):
        require(not p.is_symlink(),'symlink in retained output: '+str(p))
        if p.is_file():rows.append({'path':str(p.relative_to(root)),'bytes':p.stat().st_size,'sha256':sha(p)})
    return rows

def post_verify_inventory(root):
    """Read-only inventory immediately after successful native verify, before export/custody."""
    rows=[]
    for index in [0,1]:
        cell=root/'run/cells'/f'{index:012d}'
        manifest=load(cell/'manifest.json');require(manifest['cell_index']==index,'snapshot cell mismatch')
        paths=set()
        for artifact in manifest['artifacts']:
            rel=Path(artifact['path'])
            require(not rel.is_absolute() and rel.parts and '..' not in rel.parts and '.' not in rel.parts and str(rel)==artifact['path'],'unsafe sealed artifact path')
            require(artifact['path'] not in paths,'duplicate sealed artifact');paths.add(artifact['path'])
            for segment in [cell/Path(*rel.parts[:n]) for n in range(1,len(rel.parts)+1)]:require(not segment.is_symlink(),'sealed artifact symlink')
            path=cell/rel;require(path.is_file() and type(artifact['bytes']) is int and artifact['bytes']>=0 and path.stat().st_size==artifact['bytes'] and sha(path)==artifact['sha256'],'sealed artifact bytes differ')
        require(paths,'empty sealed artifact registry')
        rows.append({'cell_index':index,'manifest_sha256':sha(cell/'manifest.json'),'files':file_inventory(cell)})
    return {'schema':'local-affine-post-verify-inventory/v1','run_sha256':sha(root/'run/run.json'),'verify_command_receipt_sha256':sha(root/'commands/verify/receipt.json'),'verify_replay_receipt_sha256':sha(root/'replay/verify.json'),'cells':rows}

def authorize_policies(root,ready):
    plan=load(root/'plan.json');require(set(plan)==set(PLAN_FIELDS),'typed plan fields changed; refuse predicted identity')
    require(plan['channels']==load(root/'channels.json'),'plan channel mismatch')
    expected={'channels':load(root/'channels.json'),'schema':'crypto.ml-dataset.plan/v3','field_sizes':[23],'arities':[3],'families':['binary'],'solvers':['f4'],'curve_seeds':1,'generators':1,'keys':1,'seed':42,'factor_base_size':4,'degree_bound':3,'work_limit':16384,'twists':True,'protocol_sha256':ready['protocol_sha256'],'split_rule':'curve-twist-component-sha256-v1','factor_base_policies':[{'kind':'legacy'}]}
    require(exact_json(plan,expected),'actual plan differs from frozen full workload')
    # Exact Rust struct order, including the nested struct's declared field order.
    typed={k:plan[k] for k in PLAN_FIELDS};typed['channels']={'schema':plan['channels']['schema'],'affine_boolean_row':{'mode':'affine_unit_row','pattern':1,'row_index':0}}
    plan_sha=hashlib.sha256(json.dumps(typed,separators=(',',':'),ensure_ascii=False).encode()).hexdigest()
    binding={'schema':'crypto.ml-dataset.run/v1','plan_sha256':plan_sha,'source_commit':ready['source_commit'],'binary_sha256':ready['binary_sha256'],'shard_index':0,'shard_count':1,'seconds_per_case':60,'memory_mib':256,'trace_mib':128,'execution_context_required':True,'output_budget_scope':'all_cell_files_before_manifest_seal','clock_valid_for_comparison':False}
    expected=(json.dumps(binding,sort_keys=True,indent=2,ensure_ascii=False)+'\n').encode()
    (root/'expected-run.json').write_bytes(expected)
    digest=hashlib.sha256(expected).hexdigest();limits=load(root/'limits.json')
    (root/'policies').mkdir();(root/'replay').mkdir()
    for name in REPLAY:
        save(root/'policies'/f'{name}.json',{'schema':'crypto.ml-dataset.affine-replay-policy/v1','mode':'checked_affine_unit_identity_v1','run_sha256':digest,'receipt_path':str(root/'replay'/f'{name}.json'),'cell_indices':[0,1],'seconds':limits['replay_seconds_per_command'],'work_per_cell':limits['replay_work_per_cell'],'bytes_per_cell':limits['replay_bytes_per_cell'],'stream_bytes_per_cell':limits['replay_stream_bytes_per_cell'],'payload_bytes_per_cell':limits['replay_payload_bytes_per_cell'],'aggregate_work':limits['replay_aggregate_work_per_command'],'aggregate_bytes':limits['replay_aggregate_bytes_per_command']})
    save(root/'policy-bindings.json',{'expected_run_sha256':digest,'plan_binding_sha256':plan_sha,'policy_files':file_inventory(root/'policies')})

def exact_json(a,b):
    return json.dumps(a,sort_keys=True,separators=(',',':'))==json.dumps(b,sort_keys=True,separators=(',',':'))

def check_cell_schedule(m,scheduled,channels):
    expected={k:v for k,v in scheduled.items() if k!='index'};expected['channels']=channels
    require(type(m['cell_index']) is int and m['cell_index']==scheduled['index'],'cell index differs from schedule')
    require(exact_json(m['config'],expected),'actual full cell config differs from frozen schedule')

def check_replay_receipt(name,receipt,policy,run_sha):
    require(receipt['schema']=='crypto.ml-dataset.affine-replay-invocation/v1','wrong replay receipt schema')
    require(receipt['run_sha256']==run_sha==policy['run_sha256'] and receipt['verification_returned_ok'] is True,'replay invocation failed or unbound')
    require(receipt['solver_completed'] is False and receipt['training_export_admitted'] is False,'replay receipt overclaims completion/export')
    slots=receipt['slots'];require(len(slots)==2,'wrong slot count')
    require([s['cell_index'] for s in slots]==[0,1] and all(type(s['cell_index']) is int for s in slots),'wrong/duplicate slot identities')
    for slot in slots:
        require(slot['consumed'] is True and slot['failed'] is False and slot['verdict']=='checked_finite_identity','positive native finite identity missing')
        require(slot['mode']=='checked_affine_unit_identity_v1','wrong mode')
        charged=slot['charged'];require(len(charged)==2 and all(type(v) is int and v>0 for v in charged),'missing cumulative actual charges')
        require(charged[0]<=policy['work_per_cell'] and charged[1]<=policy['bytes_per_cell'],'cumulative cell cap exceeded')
        require(slot['finite_identity_projections']==(1 if name in ['export','export-repeat'] else 0),'unexpected projection count')
        if name in ['export','export-repeat']:require(slot['visits']>=2,'projection missed same-slot revalidation')
        require(type(slot['visits']) is int and 1<=slot['visits']<=8,'invalid bounded visit count')
    reads=sum(slot['visits'] for slot in slots)+(0 if name=='custody-restore' else 1)
    # Generate consumes one encoding; other existing roots consume one admission read.
    reserved_reads=2+8*len(slots)
    require(type(receipt['caller_charged_work']) is int and receipt['caller_charged_work']==65536+8226*reads,'caller charged work differs from actual admitted reads')
    require(type(receipt['caller_charged_bytes']) is int and receipt['caller_charged_bytes']==8192*reads,'caller charged bytes differ from actual admitted reads')
    expected_work=2*policy['work_per_cell']+65536+8226*reserved_reads
    require(type(receipt['reserved_work']) is int and receipt['reserved_work']==expected_work<=policy['aggregate_work'],'wrong work reservation')
    reserved_bytes=receipt['reserved_bytes'];floor=2*policy['bytes_per_cell']+65536+8192*reserved_reads
    require(type(reserved_bytes) is int and floor+2*4096<reserved_bytes<=policy['aggregate_bytes'],'invalid byte reservation')
    require((reserved_bytes-floor)%2==0,'nonintegral per-slot owner reservation')
    # sizeof retained owner is instrument-dependent: retain inferred tariff and require
    # cross-command equality, without pretending to know a Rust layout from Python.
    return (reserved_bytes-floor)//2

def analyze(root, expected_projection_sha):
    rows=[];schedule=load(root/'schedule.json');channels=load(root/'channels.json')
    require([s['index'] for s in schedule]==[0,1],'frozen schedule index mismatch')
    require(sha(root/'run/run.json')==sha(root/'expected-run.json'),'generated run differs from exact pre-dispatch authorization')
    cells=sorted((root/'run/cells').glob('*/manifest.json'))
    require(len(cells)==2,'two sealed native cells required')
    for path in cells:
        m=load(path);c=m['config'];o=m['outcome']
        require(type(m['cell_index']) is int and 0<=m['cell_index']<2,'unexpected cell index')
        check_cell_schedule(m,schedule[m['cell_index']],channels)
        require(path.parent.name==f"{m['cell_index']:012d}",'cell directory index differs')
        require(exact_json(load(path.parent/'config.json'),c),'retained config differs from manifest')
        require(exact_json(load(path.parent/'plan.json'),load(root/'plan.json')),'retained plan differs')
        require(sha(path.parent/'run.json')==sha(root/'expected-run.json'),'cell run binding differs')
        attempt=load(path.parent/'attempt.json');binding=load(root/'policy-bindings.json')
        require(attempt['schema']=='crypto.ml-dataset.attempt/v1' and type(attempt['cell_index']) is int and attempt['cell_index']==m['cell_index'] and attempt['plan_sha256']==m['plan_sha256']==binding['plan_binding_sha256'],'attempt/manifest plan identity differs')
        require(c['family']=='binary' and c['solver']=='f4' and c['n']==23 and c['m']==3 and c['factor_base_size']==4,'wrong actual cell profile')
        require(c['channels']==load(root/'channels.json'),'wrong actual request schema')
        require(o['status']=='partial' and o['solver_completed'] is False and o['scalar_recovered_without_witness'] is False,'finite row promoted or incomplete')
        rows.append({'cell_index':m['cell_index'],'twist':c['twist'],'manifest_sha256':sha(path),'actual_outcome':o})
    require(sorted(r['cell_index'] for r in rows)==[0,1] and sorted(r['twist'] for r in rows)==[False,True],'actual twist/index coverage mismatch')
    slot_tariffs=[]
    for name in REPLAY:
        receipt=load(root/'replay'/f'{name}.json');policy=load(root/'policies'/f'{name}.json')
        limits=load(root/'limits.json')
        expected_policy={'schema':'crypto.ml-dataset.affine-replay-policy/v1','mode':'checked_affine_unit_identity_v1','run_sha256':sha(root/'expected-run.json'),'receipt_path':str(root/'replay'/f'{name}.json'),'cell_indices':[0,1],'seconds':limits['replay_seconds_per_command'],'work_per_cell':limits['replay_work_per_cell'],'bytes_per_cell':limits['replay_bytes_per_cell'],'stream_bytes_per_cell':limits['replay_stream_bytes_per_cell'],'payload_bytes_per_cell':limits['replay_payload_bytes_per_cell'],'aggregate_work':limits['replay_aggregate_work_per_command'],'aggregate_bytes':limits['replay_aggregate_bytes_per_command']}
        require(exact_json(policy,expected_policy),'actual policy differs from frozen replay allowance')
        inventory=load(root/'policy-bindings.json')['policy_files'];entry=[e for e in inventory if e['path']==name+'.json']
        require(len(entry)==1 and entry[0]['sha256']==sha(root/'policies'/f'{name}.json'),'pre-command policy bytes changed')
        slot_tariffs.append(check_replay_receipt(name,receipt,policy,sha(root/'expected-run.json')))
    require(len(set(slot_tariffs))==1,'instrument owner layout changed between commands')
    for item in file_inventory(root/'run'):
        target=root/'restored/run'/item['path'];require(target.is_file() and sha(target)==item['sha256'],'restored original bytes differ: '+item['path'])
    export=load(root/'export/manifest.json')
    require(export['sealed_source_cells']==2 and export['source_run_sha256']==sha(root/'run/run.json'),'export original binding/cell count differs')
    for artifact in export['artifacts']:
        require(sha(root/'export'/artifact['path'])==artifact['sha256'],'export artifact digest mismatch')
    identity=export['affine_unit_identities']
    require(identity['verified_restricted_records']==2 and identity['public_consequence_features']==0,'wrong identity scope/count')
    require(identity['finite_identity_training_admitted'] is True and all(identity[k] is False for k in ['causal_solver_training_admitted','full_span_search','degree_labels_available']),'export claim inflation')
    records=[json.loads(line) for line in (root/'export/restricted/prefix-records.jsonl').read_text().splitlines()]
    require(len(records)==2,'wrong restricted record count')
    for record in records:
        r=record['record'];require(record['feature_id'] is None,'proof-linked public feature')
        require(r['schema']=='ml-dataset.restricted-affine-unit-identity/v1' and r['status']=='checked_finite_identity','wrong restricted identity')
        require(r['pattern']==1 and r['row']==0 and bool(r['wire']) and bool(r['encoding']) and bool(r['original_binding']),'wrong original/row identity')
        require(r['finite_identity_training_admitted'] is True and all(r[k] is False for k in ['causal_solver_training_admitted','solver_completed','full_span_search','scalar_recovered_without_witness']),'restricted claim inflation')
    # Execute exactly the already-authorized helper bytes, never sys.path or pyc.
    module_path=root/'public_projection.py'
    import stat
    with os.fdopen(os.open(module_path,os.O_RDONLY|os.O_NOFOLLOW|os.O_NONBLOCK),'rb') as stream:
        metadata=os.fstat(stream.fileno())
        require(stat.S_ISREG(metadata.st_mode) and metadata.st_size<=65536,'projection source cap/type')
        source_bytes=stream.read(65537)
        require(len(source_bytes)==metadata.st_size and len(source_bytes)<=65536,'projection source changed size')
    require(hashlib.sha256(source_bytes).hexdigest()==expected_projection_sha,'exact projection source changed')
    namespace={'__name__':'verified_public_projection','__file__':str(module_path)}
    exec(compile(source_bytes,str(module_path),'exec'),namespace)
    namespace['check'](root,load)
    require(file_inventory(root/'export')==file_inventory(root/'export-repeat'),'repeat export bytes differ')
    return {'schema':'local-affine-cli-custody-analysis/v1','status':'PASS','cells':rows,'checked_finite_identities':2,'solver_completed_cells':0,'actual_export_manifest':export,'remote_custody_status':load(root/'s3-readback/PASS.json')['status'],'remote_requests':len(load(root/'s3-readback/receipts.json'))}

def run_acceptance(root,ready_path):
    root=Path(root).resolve();ready=load(ready_path);binary=Path(ready['binary_path']).resolve();source=Path(ready['source_path']).resolve()
    require(str(root)==ready['root']=='/var/tmp/affine-cli-custody-acceptance-20261105-v3','wrong unique root')
    require(ready.get('execution_allowed') is True,'readiness not admitted')
    pub=ready['publication'];require(sha(pub['path'])==pub['sha256'],'publication receipt changed')
    evidence=load(pub['path']);require(evidence['github_verified'] is True and evidence['source_commit']==ready['source_commit'],'source not verified published')
    require(evidence['protocol_sha256']==ready['protocol_sha256'] and exact_json(evidence['input_sha256'],ready['input_sha256']),'published exact inputs differ')
    require(evidence['repository']=='isogeny-labs/crypto','publication differs from existing authorized source repository')
    require(evidence['publication_scope']=='source_protocol_configuration_only','restricted datasets are not source publication inputs')
    require(ready['native_regressions']['status']=='PASS' and ready['native_regressions']['source_commit']==ready['source_commit'],'current affine, raw, SAT controls not passed')
    require(sha(ready['native_regressions']['path'])==ready['native_regressions']['sha256'],'native regression evidence changed')
    regression=load(ready['native_regressions']['path'])
    require(regression['status']=='PASS' and regression['exit_code']==0 and regression['source_commit']==ready['source_commit'] and regression['source_tree_before']==ready['source_tree']==regression['source_tree_after'] and regression['source_unchanged'] is True,'native receipt does not prove exact-source PASS')
    expected=load(root/'required-regressions.json');checks=regression['tests']
    names=[c['name'] for c in checks];require(len(names)==len(set(names)) and set(expected)<=set(names),'missing or duplicate named controls')
    for check in checks:
        require(check['status']=='PASS' and check['exit_code']==0 and check['passed']==1 and check['failed']==0 and check['ignored']==0,'nonpassing or filtered-only native control')
        for artifact in ['stdout','stderr']:
            require(sha(check[artifact+'_path'])==check[artifact+'_sha256'],'native output changed')
    require(bool(regression['evidence']),'regression underlying evidence missing')
    for item in regression['evidence']:require(sha(item['path'])==item['sha256'],'regression underlying receipt changed')


    require(sha(binary)==ready['binary_sha256'],'wrong exact binary')
    require(sha(root/'protocol.json')==ready['protocol_sha256'],'wrong frozen protocol')
    protocol=load(root/'protocol.json');require(protocol['freeze']['status']=='frozen' and protocol['freeze']['filed_before_affected_data'] is True,'protocol is not frozen before data')
    require(protocol['extensions']['dataset_channels']==load(root/'channels.json'),'protocol channel authorization differs')
    require(set(ready['input_sha256'])=={'driver.py','commands.json','channels.json','factor-base-policies.json','schedule.json','limits.json','s3_custody.py','public_projection.py','required-regressions.json'},'incomplete frozen input inventory')
    for name,digest in ready['input_sha256'].items():
        require('/' not in name and name not in ['.','..'],'input filename escape');require(sha(root/name)==digest,'frozen input changed '+name)
    require(ready['input_sha256']['driver.py']==sha(Path(__file__)),'executor not bound')
    for kind in ['release_binary','full_library']:
        g=ready['gates'][kind];require(sha(g['path'])==g['sha256'],'gate changed');v=load(g['path'])
        require(v['status']=='PASS' and v['exit_code']==0 and v['source_commit']==ready['source_commit'] and v['source_tree_before']==ready['source_tree'] and v['source_tree_after']==ready['source_tree'] and v['source_unchanged'] is True,'gate is not exact-source passing')
        require(v['evidence'],'gate lacks underlying evidence')
        for item in v['evidence']:require(sha(item['path'])==item['sha256'],'underlying gate evidence changed')
        if kind=='release_binary':require(v['binary_sha256']==ready['binary_sha256'],'release gate binary mismatch')
    for name in ['commands','attempt-start.json','attempt-result.json','run','export','export-repeat','report','bundle','s3-readback','restored','policies','replay']:
        require(not (root/name).exists(),'existing side effect: reconcile, do not repeat '+name)
    require(not live_processes(binary,root),'existing native generator/worker: reconcile first')
    save(root/'attempt-start.json',{'schema':'local-affine-cli-custody-attempt/v1','started_at':stamp(),'ready_sha256':sha(ready_path),'source_commit':ready['source_commit'],'binary_sha256':ready['binary_sha256'],'protocol_sha256':ready['protocol_sha256'],'max_generate_invocations':1})
    runner=None;result={'status':'incomplete','generate_invocations':0};source_started=False;source_finished=False
    def source_check(label):
        records=[]
        try:
            for argv_tail,expected in [(['rev-parse','HEAD'],ready['source_commit']),(['rev-parse',ready['source_commit']+'^{tree}'],ready['source_tree']),(['status','--porcelain=v1','--untracked-files=all'],'')]:
                record={'argv':['git','-C',str(source),*argv_tail],'exit_code':None,'stdout':'','stderr':'','exception':None};records.append(record)
                try:
                    remaining=runner.deadline-time.monotonic();require(remaining>0,'source check deadline')
                    p=subprocess.run(record['argv'],capture_output=True,text=True,timeout=min(180,remaining),env=env());record.update(exit_code=p.returncode,stdout=p.stdout,stderr=p.stderr)
                    require(p.returncode==0 and p.stdout.strip()==expected,'source identity/cleanliness changed')
                except subprocess.TimeoutExpired as exc:
                    decode=lambda v:v.decode(errors='replace') if isinstance(v,bytes) else v or ''
                    record.update(stdout=decode(exc.stdout),stderr=decode(exc.stderr),exception=str(exc),timed_out=True);raise
                except BaseException as exc:record['exception']=str(exc);raise
        finally:save(root/(label+'.json'),records)
    try:
        runner=Runner(root,binary,ready['binary_sha256'],ready['protocol_sha256']);source_check('source-before');source_started=True
        commands=load(root/'commands.json');require(list(commands)==ORDER,'command order changed')
        values={'${ROOT}':str(root),'${CRYPTO}':str(binary),'${PROTOCOL_SHA}':ready['protocol_sha256']}
        for name in ORDER:
            if name=='custody-restore':
                runner.command('s3-custody', [sys.executable,root/'s3_custody.py','--root',root,'--authorization',ready['custody_authorization']],timeout=190)
                runner.command('s3-readback-custody-verify',[binary,'dataset','custody','verify','--bundle',root/'s3-readback/bundle'],timeout=60)
            if name=='generate':authorize_policies(root,ready);result['generate_invocations']+=1;require(result['generate_invocations']==1,'duplicate generation')
            argv=[]
            for arg in commands[name]:
                for old,new in values.items():arg=arg.replace(old,new)
                require('${' not in arg,'unbound command');argv.append(arg)
            if name in ['custody-restore']:
                argv=[a.replace(str(root/'bundle'),str(root/'s3-readback/bundle')) for a in argv]
            require(argv[0]==str(binary) and argv[1]=='dataset','unexpected command binary')
            runner.command(name,argv,timeout=300 if name not in ['plan','preflight'] else 60)
            if name=='verify':
                save(root/'post-verify-inventory.json',post_verify_inventory(root))
                result['post_verify_inventory_sha256']=sha(root/'post-verify-inventory.json')
        source_check('source-after');source_finished=True
        save(root/'analysis.json',analyze(root,ready['input_sha256']['public_projection.py']));result['status']='PASS'
    except BaseException as exc:
        result.update(status='FAIL',error_type=type(exc).__name__,error=str(exc))
        if not (root/'analysis.json').exists():save(root/'analysis.json',{'status':'FAIL','error':str(exc),'planned_cells':2,'no_retry':True})
        raise
    finally:
        try:
            if source_started and not source_finished and not (root/'source-after.json').exists():source_check('source-after')
        except BaseException as exc:result.update(status='FAIL',source_after_error=str(exc))
        result.update(finished_at=stamp(),cleanup=runner.last_cleanup if runner else {'settled':True,'unresolved':[]});save(root/'attempt-result.json',result);save(root/'attempt-inventory.json',file_inventory(root))
    require(result['status']=='PASS','terminal source check failed')

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--root',required=True);p.add_argument('--ready',required=True);a=p.parse_args();run_acceptance(a.root,Path(a.ready))
