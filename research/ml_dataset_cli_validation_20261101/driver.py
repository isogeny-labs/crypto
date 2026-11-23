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
    return json.loads(Path(path).read_text())

def save(path, obj):
    with Path(path).open("x") as f:
        json.dump(obj, f, indent=2, sort_keys=True, allow_nan=False)
        f.write("\n")
        f.flush()
        os.fsync(f.fileno())

def env():
    return {"PATH": "/usr/local/bin:/usr/bin:/bin", "HOME": str(Path.home()), "LANG": "C", "LC_ALL": "C", "RAYON_NUM_THREADS": "1", "PYTHONDONTWRITEBYTECODE": "1"}

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


def cleanup_owned(child, binary, fixture, grace=5.0, kill_wait=5.0):
    """Recheck exact output ownership and PID identity before every signal."""
    actions, errors = [], []
    for sig, seconds in [(signal.SIGTERM, grace), (signal.SIGKILL, kill_wait)]:
        until = time.monotonic() + seconds
        signalled = set()
        while True:
            owned = [p for p in live_processes(binary, fixture) if p["belongs_to_fixture"]]
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
            remaining = [p for p in live_processes(binary, fixture) if p["belongs_to_fixture"]]
            direct_live = child is not None and child.poll() is None
            if not remaining and not direct_live:
                # A second scan after reaping closes the ordinary parent/worker
                # exit race; no status file alone is used as liveness evidence.
                remaining = [p for p in live_processes(binary, fixture) if p["belongs_to_fixture"]]
                if not remaining:
                    return {"settled":True,"actions":actions,"errors":errors,"unresolved":[],"direct_child_live":False}
            if time.monotonic() >= until:
                break
            time.sleep(0.05)
    return {"settled":False,"actions":actions,"errors":errors,"unresolved":remaining,"direct_child_live":direct_live}


class Runner:
    def __init__(self, fixture, binary, expected):
        self.fixture = fixture
        self.binary = Path(binary)
        self.expected = expected
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
        d = self.commands / f"{self.sequence:03}-{name}"
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
                    remaining = [p for p in live_processes(self.binary, self.fixture) if p["belongs_to_fixture"]]
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

def source_gate(runner, source, commit, tree):
    require(re.fullmatch(r"[0-9a-f]{40}", commit) is not None, "source must be full immutable commit")
    require(runner.git("source-head", source, "rev-parse", "HEAD").decode().strip() == commit, "source HEAD differs from validated commit")
    require(runner.git("source-tree", source, "rev-parse", commit + "^{tree}").decode().strip() == tree, "source tree differs from gate")
    require(not runner.git("source-clean", source, "status", "--porcelain=v1", "--untracked-files=all").strip(), "source must be clean, including untracked files")

def gate_file(root, value):
    path = Path(value)
    return path if path.is_absolute() else Path(root) / path


def terminal(log, unfiltered=False):
    lines = [s.strip() for s in log.splitlines() if s.strip()]
    require(lines and not any('FAILED' in s or re.match(r'(?i)^(error:|error\[|fatal:)', s) for s in lines), 'failure in test log')
    summary = r'test result: ok\. (\d+) passed; 0 failed; (\d+) ignored; (\d+) measured; (\d+) filtered out; finished in .+'
    m = re.fullmatch(summary, lines[-1])
    require(m and int(m[1]) > 0 and (not unfiltered or int(m[4]) == 0), 'empty/filtered/failing test suite')
    # These two exact inherited-output child routes are present in the pinned
    # Rust sources. Other subprocess tests capture or suppress their output.
    # Do not broadly ignore intermediate summaries: require the entire known
    # parent-start/child-run/child-result/parent-acknowledgement sequence.
    fifo = 'ml_dataset::telemetry::checked::tests::fifo_replacement_is_rejected_without_waiting_for_writer'
    checkpoint = 'cryptanalysis::koblitz_groebner::checkpoint::tests::'
    children = {fifo:fifo, checkpoint+'real_solver_recovers_after_sigkill_and_rejects_corruption':checkpoint+'solver_checkpoint_child'}
    normalized = []; seen = set(); i = 0
    while i < len(lines):
        pending = re.fullmatch(r'test (\S+) \.\.\.', lines[i])
        if pending:
            parent = pending[1]
            require(parent in children and parent not in seen, 'unknown/duplicate nested test')
            offset = 0
            if i+2 < len(lines) and lines[i+2].startswith('Restoring native checkpoint:'):
                require(parent == checkpoint+'real_solver_recovers_after_sigkill_and_rejects_corruption'
                        and lines[i+2] == 'Restoring native checkpoint: degree=2, next_column=1',
                        'unexpected checkpoint restore diagnostic')
                offset = 1
            require(i+4+offset < len(lines), 'truncated nested test')
            require(lines[i+1] == 'running 1 test' and lines[i+2+offset] == 'test '+children[parent]+' ... ok' and lines[i+4+offset] == 'ok', 'malformed nested test boundaries')
            child = re.fullmatch(summary, lines[i+3+offset])
            total = sum(int(m[j]) for j in [1,2,3,4])
            require(child and [int(child[j]) for j in [1,2,3]] == [1,0,0] and int(child[4])+1 == total, 'child summary does not match parent inventory')
            normalized.append('test '+parent+' ... ok'); seen.add(parent); i += 5+offset
        else:
            require(not lines[i].startswith('Restoring native checkpoint:'), 'checkpoint diagnostic outside known child')
            normalized.append(lines[i]); i += 1
    require(sum(s.startswith('test result:') for s in normalized) == 1, 'unattributed/concatenated summaries')
    require(sum(s.startswith('Running unittests src/lib.rs (') for s in normalized) == 1, 'missing/concatenated cargo invocation')
    runs = [(i,re.fullmatch(r'running (\d+) tests?',s)) for i,s in enumerate(normalized)]
    runs = [(i,r) for i,r in runs if r]
    require(len(runs) == 1 and int(runs[0][1][1]) == sum(int(m[j]) for j in [1,2,3]), 'parent test count mismatch')
    rows = []
    for line in normalized[runs[0][0]+1:-1]:
        if line.startswith('test '):
            row = re.fullmatch(r'test (\S+)(?: - should panic)? \.\.\. (ok|ignored(?:,.*)?)',line)
            require(row is not None, 'unrecognized parent test outcome')
            rows.append((row[1],row[2]))
    require(len({name for name,_ in rows}) == len(rows), 'duplicate parent test')
    require(sum(status == 'ok' for _,status in rows) == int(m[1]) and sum(status.startswith('ignored') for _,status in rows) == int(m[2]) and int(m[3]) == 0, 'parent outcomes do not match summary')


def validate_gate_records(g, expected, root):
    commands = {
        "full_library":["cargo","test","--release","--lib","--locked","--offline","--","--test-threads=1"],
        "cli_build":["cargo","build","--release","--bin","crypto","--locked","--offline"],
    }
    for name, argv in commands.items():
        row = g[name]
        require(row["argv"] == argv and row["source_commit"] == g["source_commit"], f"{name}: wrong command/source")
        log, exit_file, receipt_file = [gate_file(root,row[key]) for key in ["log_file","exit_file","command_receipt_file"]]
        require(exit_file.read_text().strip() == "0", f"{name}: nonzero exit")
        require(sha(log) == row["log_sha256"] and sha(exit_file) == row["exit_sha256"], f"{name}: log/exit changed")
        require(sha(receipt_file) == row["command_receipt_sha256"], f"{name}: captured command receipt changed")
        receipt = load(receipt_file)
        require(receipt["exit_code"] == 0, f"{name}: captured command failed")
        for key in ["argv","source_commit","source_snapshot","started_at","finished_at","log_sha256","exit_sha256"]:
            require(receipt[key] == row[key], f"{name}: command receipt does not bind {key}")
        require(row["source_snapshot"] == g["source_snapshot"], f"{name}: mismatched source snapshot")
        require(parse_time(row["finished_at"]) >= parse_time(row["started_at"]), f"{name}: reversed execution times")
        lines = [line.strip() for line in log.read_text().splitlines() if line.strip()]
        require(lines, f"{name}: empty log")
        require(not any("FAILED" in line or re.match(r"(?i)^(error:|error\[|fatal:)",line) for line in lines), f"{name}: failure/error in log")
        if name == "full_library":
            terminal(log.read_text(), True)
        else:
            require(row["binary_sha256"] == expected and row["embedded_source_dirty"] is False, "CLI dirty/wrong binary")
            require(row["embedded_source_commit"] == g["source_commit"], "CLI embedded source mismatch")
            for key in ["binary_sha256","embedded_source_commit","embedded_source_dirty"]:
                require(receipt[key] == row[key], "CLI captured receipt differs from build binding")


def archive_gate_evidence(fixture, external, g):
    dest = Path(fixture) / "gate-evidence"
    dest.mkdir()
    shutil.copyfile(external, dest/"original-contract.json")
    require(sha(external) == sha(dest/"original-contract.json") and load(dest/"original-contract.json") == g, "gate contract changed during archival")
    archived = json.loads(json.dumps(g))
    origins = []
    for name in ["full_library","cli_build"]:
        for field, suffix in [("log_file","log"),("exit_file","exit"),("command_receipt_file","command.json")]:
            source = gate_file(Path(external).parent,g[name][field])
            target = dest / f"{name}.{suffix}"
            shutil.copyfile(source,target)
            require(sha(source) == sha(target), "gate archival readback mismatch")
            archived[name][field] = target.name
            origins.append({"original":str(source),"copy":target.name,"sha256":sha(target)})
    save(dest/"contract.json",archived)
    save(dest/"custody.json",{"original_contract_sha256":sha(external),"files":origins})
    validate_gate_records(archived, archived["cli_build"]["binary_sha256"], dest)
    return dest/"contract.json"


def verify_source_archive(path, entries):
    import tarfile
    from pathlib import PurePosixPath
    expected=set(entries);parents=set()
    for name in expected:
        require(name and not PurePosixPath(name).is_absolute() and '..' not in PurePosixPath(name).parts
                and str(PurePosixPath(name))==name and not any(c in name for c in '\n\r\0'), 'unsafe archive source path')
        parents.update(str(p) for p in PurePosixPath(name).parents if str(p)!='.')
    seen=set();files=set()
    with path.open('rb') as raw, tarfile.open(fileobj=raw,mode='r:') as archive:
        for member in archive:
            name=member.name.rstrip('/') if member.isdir() else member.name
            require(name not in seen,'duplicate source archive member');seen.add(name)
            require(name and not PurePosixPath(name).is_absolute() and '..' not in PurePosixPath(name).parts
                    and str(PurePosixPath(name))==name,'unsafe source archive member')
            if member.isdir():
                require(name in parents and member.size==0,'unexpected source archive directory')
                continue
            require(member.isfile() and name in expected,'extra/nonregular source archive member')
            digest,size=entries[name]
            require(member.size==size,'committed source size changed: '+name)
            stream=archive.extractfile(member);observed=hashlib.sha256();remaining=size
            while remaining:
                chunk=stream.read(min(remaining,1024*1024));require(chunk,'truncated source archive blob')
                observed.update(chunk);remaining-=len(chunk)
            require(stream.read(1)==b'','extra source archive blob bytes');stream.close()
            require(observed.hexdigest()==digest,'source not bound to committed blob: '+name);files.add(name)
        require(files==expected,'missing committed source archive members')
        # TarFile stops at the first zero header; do not accept hidden trailing archives.
        raw.seek(archive.offset)
        while True:
            tail=raw.read(1024*1024)
            if not tail:break
            require(not any(tail),'nonzero trailing source archive data')

def committed_source_archive(runner,source,commit,entries):
    require(re.fullmatch(r'[0-9a-f]{40}',commit) is not None,'invalid source archive commit')
    directory=runner.command('source-archive',['git','--literal-pathspecs','-C',source,'archive','--format=tar',commit,'--',*entries])
    verify_source_archive(directory/'stdout',entries)
    return directory


def gates(runner, source, commit, expected, path):
    g = load(path)
    require(g.get("schema") == "crypto.ml-dataset.cli-smoke-gates/v1", "missing explicit full-library/build gate contract")
    require(g.get("source_commit") == commit, "gate source mismatch")
    source_gate(runner, source, commit, g["source_tree"])
    require(REQUIRED_SOURCE <= set(g["source_snapshot"]), "incomplete source snapshot gate")
    entries = {}
    for rel, digest in g["source_snapshot"].items():
        require(not Path(rel).is_absolute() and ".." not in Path(rel).parts, "unsafe source snapshot path")
        p = source / rel
        require(p.is_file() and not p.is_symlink() and sha(p) == digest, f"changed source {rel}")
        entries[rel]=(digest,p.stat().st_size)
    committed_source_archive(runner,source,commit,entries)
    validate_gate_records(g, expected, Path(path).parent)
    return g

def inventory(fixture, name, settled=True, unresolved=None):
    rows = []
    for p in sorted(fixture.rglob("*")):
        require(not p.is_symlink(), f"unexpected symlink in evidence: {p}")
        if p.is_file():
            rows.append({"path": str(p.relative_to(fixture)), "bytes": p.stat().st_size, "sha256": sha(p)})
    save(fixture / name, {"schema": SCHEMA, "recorded_at": stamp(), "stable_after_cleanup":settled, "unresolved_processes":unresolved or [], "files": rows})

def prepare(a):
    fixture = Path(a.fixture).absolute()
    require(fixture.parent.resolve() == Path("/var/tmp") and not fixture.exists(), "prepare requires a new direct /var/tmp destination; never reuse/delete output")
    binary = Path(a.binary).resolve(strict=True)
    require(binary.is_file() and os.access(binary, os.X_OK) and not binary.stat().st_mode & 0o222, "binary must be retained, executable and read-only")
    require(sha(binary) == a.binary_sha256, "binary hash mismatch")
    require(not live_processes(binary, fixture), "existing generator/worker for retained binary; reconcile before preparing")
    require(not Path(__file__).stat().st_mode & 0o222, "retain a read-only exact driver copy before freezing its hash")
    fixture.mkdir()
    runner = Runner(fixture, binary, a.binary_sha256)
    try:
        fs_type = runner.command("local-filesystem", ["stat", "-f", "-c", "%T", fixture])
        require((fs_type / "stdout").read_text().strip() == "xfs", "smoke requires local XFS output")
        source = Path(a.source_dir).resolve(strict=True)
        retained_gates = archive_gate_evidence(fixture, a.gates, load(a.gates))
        g = gates(runner, source, a.source_commit, a.binary_sha256, retained_gates)
        checker = Path(a.checker).resolve(strict=True)
        require(sha(checker) == a.checker_sha256, "canonical checker hash mismatch")
        checker_source = load(checker.parent.parent / "SOURCE.json")
        require(checker_source["commit"] == "37e193bf8b3e673e0942709794f44d28e6ab66fb", "unexpected canonical checker pin")
        for rel, digest in checker_source["sha256"].items():
            require(sha(checker.parent.parent / rel) == digest, "canonical tool/schema hash mismatch")
        validator = json.loads(a.validator_prefix)
        require(validator == ["/home/ec2-user/.local/bin/uv", "run", "--offline", "--no-project", "--with", "jsonschema==4.26.0", "python"], "use the reviewed pinned offline canonical validator route")
        draft = Path(a.protocol_draft).resolve(strict=True)
        protocol = fixture / "protocol.json"
        body = load(draft)
        require(body.get("schema") == "experiment.protocol/v1" and body.get("freeze", {}).get("status") == "draft", "supply complete canonical draft, not a minimal or pre-frozen protocol")
        driver = str(Path(__file__).resolve())
        driver_sha = sha(driver)
        for arm in body["arms"]:
            arm["instrument"]["commit"] = a.source_commit
            arm["instrument"]["binary_sha256"] = a.binary_sha256
        body["analysis_plan"]["script"] = {"path":driver,"sha256":driver_sha}
        body["analysis_plan"]["frozen_before_data"] = True
        for factor in body["variable_ledger"]["factors"]:
            if factor["name"] == "instrument": factor["value"] = {"source_commit":a.source_commit,"binary_sha256":a.binary_sha256}
            if factor["name"] == "analysis": factor["value"] = {"path":driver,"sha256":driver_sha}
        binding = body["extensions"]["software_binding"]
        binding.update(source_commit=a.source_commit,binary_path=str(binary),binary_sha256=a.binary_sha256,
                       driver_path=driver,driver_sha256=driver_sha,analysis_path=driver,analysis_sha256=driver_sha,
                       checker_sha256=a.checker_sha256,validator_argv_prefix=validator)
        body["extensions"]["binding_state"] = "BOUND_DRAFT_PENDING_FREEZE"
        require(body["extensions"].get("policy_panels") == POLICY_PANELS, "wrong frozen policy extension")
        require(body["extensions"]["scientific_corpus_rows"] == 0 and body["extensions"]["verified_cloud_cells"] == 0, "protocol exceeds software scope")
        save(protocol, body)
        binding_text = json.dumps(body, sort_keys=True)
        for value in [a.source_commit, a.binary_sha256, sha(Path(__file__))]:
            require(value in binding_text, "protocol must bind exact source, binary, and driver/analysis SHA")
        runner.command("protocol-validate-draft", [*validator, checker, "validate", protocol], 120)
        runner.command("protocol-freeze", [*validator, checker, "freeze", protocol, "--commit", a.source_commit, "--filed-before-affected-data"], 120)
        runner.command("protocol-validate-frozen", [*validator, checker, "validate", protocol], 120)
        p = load(protocol)
        require(p["freeze"]["status"] == "frozen" and p["freeze"]["filed_before_affected_data"] is True, "official freeze incomplete")
        state = {"schema": SCHEMA, "phase": "ready_to_commit", "prepared_at": stamp(), "source": str(source), "source_commit": a.source_commit,
                 "source_tree": g["source_tree"], "binary": str(binary), "binary_sha256": a.binary_sha256,
                 "gates": str(retained_gates.relative_to(fixture)), "gates_sha256": sha(retained_gates), "checker": str(checker), "checker_sha256": a.checker_sha256,
                 "driver_sha256": sha(Path(__file__)), "validator_argv_prefix":validator,
                 "checker_source_sha256":sha(checker.parent.parent / "SOURCE.json"), "protocol_file_sha256": sha(protocol), "protocol_body_sha256": p["freeze"]["protocol_sha256"],
                 "scientific_corpus_rows": 0, "verified_cloud_cells": 0, "cloud_launches": 0}
        save(fixture / "prepared.json", state)
        inventory(fixture, "prepare-inventory.json")
        print(json.dumps({"phase": "ready_to_commit", "fixture": str(fixture), "protocol_file_sha256": sha(protocol), "next": "Commit this exact frozen protocol and instruments before execute. No dataset commands ran."}))
    except BaseException as exc:
        save(fixture / "prepare-failed.json", {"at": stamp(), "error": str(exc), "no_retry": True, "cleanup":runner.last_cleanup})
        inventory(fixture, "prepare-failed-inventory.json", runner.last_cleanup["settled"], runner.last_cleanup.get("unresolved"))
        raise

def json_output(directory):
    lines = [x for x in (directory / "stdout").read_text().splitlines() if x.strip()]
    require(len(lines) == 1, f"expected one JSON result at {directory}")
    return json.loads(lines[0])

def expected_cli_argv(state, fixture):
    b, protocol = state["binary"], str(fixture/"protocol.json")
    plan, run = str(fixture/"plan.json"), str(fixture/"run")
    return {
        "plan":[b,"dataset","plan","--out",plan,"--n","23","--m","3","--families","koblitz","--solvers","f4,f5","--curve-seeds","1","--generators","1","--keys","1","--seed","42","--factor-base-size","4","--degree-bound","3","--work-limit","16384","--protocol-sha256",state["protocol_file_sha256"]],
        "preflight":[b,"dataset","preflight","--plan",plan,"--protocol",protocol],
        "generate":[b,"dataset","generate","--plan",plan,"--protocol",protocol,"--out",run,"--max-cases","4","--shard-index","0","--shard-count","1","--seconds-per-case","60","--memory-mib","256","--trace-mib","128"],
        "verify":[b,"dataset","verify","--out",run],
        "status":[b,"dataset","status","--out",run],
        "export":[b,"dataset","export","--out",run,"--destination",str(fixture/"export")],
        "report":[b,"dataset","report","--out",run,"--destination",str(fixture/"report")],
    }


def checked_receipt(directory, argv, state, expected_error=None):
    row = load(directory/"receipt.json")
    require(row["argv"] == argv and load(directory/"argv.json") == argv, "command argv mismatch")
    require((directory/"argv.nul").read_bytes() == b"\0".join(os.fsencode(a) for a in argv)+b"\0", "NUL argv mismatch")
    require(row.get("timed_out") is False and row.get("exception") is None and row.get("interruption") is None, "command interrupted")
    if expected_error is None: require(row["exit_code"] == 0, "command did not complete successfully")
    else:
        require(type(row["exit_code"]) is int and 0 < row["exit_code"] <= 255 and row["exit_code"] not in [124,125,130], "negative command not an ordinary refusal")
        require(expected_error in (directory/"stderr").read_text(), "negative command failed for wrong reason")
    require(row.get("cleanup",{}).get("settled") is True and not row["cleanup"].get("unresolved"), "command has unresolved workers")
    require(row["binary_sha256_before"] == row["binary_sha256_after"] == state["binary_sha256"], "command binary provenance mismatch")
    require(row["protocol_sha256_before"] == row["protocol_sha256_after"] == state["protocol_file_sha256"], "command protocol provenance mismatch")
    require(sha(directory/"stdout") == row["stdout_sha256"] and sha(directory/"stderr") == row["stderr_sha256"], "command output changed")
    require(parse_time(row["finished_at"]) >= parse_time(row["started_at"]), "command has reversed timestamps")
    return row


def validate_pipeline_evidence(fixture):
    """Offline analysis cannot manufacture pipeline success from cell files alone."""
    require(not (fixture/"execution-failed.json").exists(), "failed attempt cannot become a successful standalone analysis")
    state, started = load(fixture/"prepared.json"), load(fixture/"execution-started.json")
    require(state["schema"] == SCHEMA and state["phase"] == "ready_to_commit", "missing prepared provenance")
    require(sha(fixture/"protocol.json") == state["protocol_file_sha256"], "analysis protocol changed")
    require(sha(state["binary"]) == state["binary_sha256"], "retained binary unavailable or changed")
    require(sha(Path(__file__)) == state["driver_sha256"], "analysis instrument changed after freeze")
    gpath = fixture/state["gates"]
    require(sha(gpath) == state["gates_sha256"], "archived gate contract changed")
    gate_contract = load(gpath)
    require(gate_contract["source_commit"] == state["source_commit"] and gate_contract["source_tree"] == state["source_tree"], "archived source binding mismatch")
    validate_gate_records(gate_contract,state["binary_sha256"],gpath.parent)
    protocol = load(fixture/"protocol.json")
    require(protocol["freeze"]["status"] == "frozen" and protocol["freeze"]["filed_before_affected_data"] is True, "analysis protocol was not frozen")
    binding = protocol["extensions"]["software_binding"]
    require(binding["source_commit"] == state["source_commit"] and binding["binary_sha256"] == state["binary_sha256"] and binding["analysis_sha256"] == state["driver_sha256"], "protocol instrument binding mismatch")
    commits = list((fixture/"commands").glob("*-committed-protocol"))
    require(len(commits) == 1, "missing/duplicate committed protocol receipt")
    checked_receipt(commits[0],["git","-C",started["protocol_repository"],"show",started["protocol_commit"]+":"+started["protocol_git_path"]],state)
    require((commits[0]/"stdout").read_bytes() == (fixture/"protocol.json").read_bytes(), "committed protocol blob mismatch")
    validations = list((fixture/"commands").glob("*-protocol-validate-committed"))
    require(len(validations) == 1, "missing/duplicate committed protocol validation receipt")
    validation = checked_receipt(validations[0],[*state["validator_argv_prefix"],state["checker"],"validate",str(fixture/"protocol.json")],state)
    require(validation["checker_sha256"].get(state["checker"]) == state["checker_sha256"], "protocol validator receipt checker mismatch")
    expected = expected_cli_argv(state,fixture)
    actual = []
    for directory in sorted((fixture/"commands").iterdir()):
        if (directory/"argv.json").is_file():
            argv = load(directory/"argv.json")
            if len(argv)>1 and argv[1]=="dataset":
                actual.append((directory,argv))
    actual = checked_dataset_inventory(state,fixture,actual)
    results = {}
    previous = parse_time(protocol["freeze"]["frozen_at"])
    for (name,argv),(directory,seen) in zip(expected.items(),actual):
        require(seen == argv and directory.name.endswith("-"+name), "CLI order, identity or fixed arguments changed")
        receipt = checked_receipt(directory,argv,state)
        require(parse_time(receipt["started_at"]) >= previous, "command ordering/freeze boundary inconsistent")
        previous = parse_time(receipt["finished_at"])
        results[name] = None if name == "generate" else json_output(directory)
    pre = results["preflight"]
    require(pre["source_commit"] == state["source_commit"] and pre["binary_sha256"] == state["binary_sha256"] and pre["source_dirty"] is False, "preflight source/binary mismatch")
    require(pre["plan_file_sha256"] == sha(fixture/"plan.json") and pre["protocol_file_sha256"] == state["protocol_file_sha256"], "preflight input hashes mismatch")
    require(pre["planned_cells"] == 4 and pre["cloud_permissions_checked"] is False, "preflight scope changed")
    require(results["plan"]["planned_cells"] == 4, "plan count changed")
    for key in ["verify","status"]:
        r = results[key]
        require(r["completed_receipts"] == 4 and r["incomplete_attempts"] == [] and r["campaign_attempts_complete"] is True and r["artifact_hashes_verified"] is True, "central verification/status incomplete")
    require(results["verify"]["known_answer_replay_performed"] is True, "verify receipt did not request replay")
    report = results["report"]
    summary = load(fixture/"report/summary.json")
    require(summary == report["summary"], "report summary changed")
    summary_artifact = report["summary_artifact"]
    require(summary_artifact["path"] == "summary.json" and summary_artifact["sha256"] == sha(fixture/"report/summary.json"), "report receipt not bound to summary")
    require(summary["campaign_attempts_complete"] is True and summary["planned_method_cells"] == 4, "report incomplete")
    for a in summary["artifacts"]:
        path = fixture/"report"/a["path"]
        require(not Path(a["path"]).is_absolute() and ".." not in Path(a["path"]).parts, "report artifact escaped root")
        require(path.stat().st_size == a["bytes"] and sha(path) == a["sha256"], "report artifact changed")
    return state


def analyze(fixture):
    fixture = Path(fixture)
    validate_pipeline_evidence(fixture)
    cells = sorted((fixture / "run" / "cells").iterdir())
    require(len(cells) == 4, "expected four actual cell directories")
    rows = []
    for i, cell in enumerate(cells):
        config = load(cell / "config.json")
        require((config["solver"], config["twist"]) == (["f4", "f5"][i % 2], i >= 2), "unexpected solver/twist mapping")
        for key, value in {"family":"koblitz", "n":23, "m":3, "factor_base_size":4, "degree_bound":3, "work_limit":16384}.items():
            require(config[key] == value, f"changed actual cell {key}")
        launch = load(cell / "execution-context.json")
        require((launch["seconds_per_case"], launch["memory_bytes"], launch["trace_bytes"]) == (60, 256*1024*1024, 128*1024*1024), "changed original limits")
        require(launch["deadline_monotonic_ns"] - launch["start_monotonic_ns"] == 60*10**9, "renewed/wrong original deadline")
        policy = load(cell / "measurement-policy.json")
        require(all(policy.get(k) is False for k in ["training_cost_eligible", "whole_worker_coverage_verified", "hardware_independently_verified", "platform_admitted", "clock_valid_for_comparison"]), "telemetry scope changed; review rather than claim training costs")
        process = load(cell / "worker-process.json")
        require(process["worker_pid"] > 0 and process["supervisor_pid"] > 0 and process["process_group"] == process["worker_pid"], "missing real supervisor process fields")
        manifest = load(cell / "manifest.json")
        outcome = load(cell / "result.json")
        require(manifest["config"] == config, "manifest config mismatch")
        require(outcome["strategy"] == outcome["adapter_id"] == "koblitz-m3-native-v1", "ordinary fallback is not native smoke coverage")
        require(outcome["solver_completed"] is True, "failed/censored cell retained; no resampling")
        finalization = outcome["capture_finalization"]
        require(all(finalization[k] is True for k in ["event_attempted", "event_acknowledged", "finish_attempted", "finish_acknowledged"]) and finalization["stop"] is None, "capture not completed")
        for key in ["first_fall_degree", "last_fall_degree", "retired_hardware_instructions"]:
            require(outcome.get(key) is None, f"unsupported exact {key}")
        paths = [a["path"] for a in manifest["artifacts"]]
        require(all(paths.count(s) == 1 for s in SLOTS), "canonical roots missing/duplicated")
        rows.append({"cell": cell.name, "solver": config["solver"], "twist": config["twist"], "manifest_sha256": sha(cell / "manifest.json"), "status": outcome["status"]})
    indexes = [json.loads(x) for x in (fixture / "export/restricted/index.jsonl").read_text().splitlines() if x]
    require(len(indexes) == 4 and len({x["record_id"] for x in indexes}) == 4, "export lost source identities")
    for index in indexes:
        paths = {x["path"] for x in index["source_artifacts"]}
        require(SLOTS <= paths, "export lost restricted native artifact references")
    features = [json.loads(x) for x in (fixture / "export/features.jsonl").read_text().splitlines() if x]
    require(features, "missing model-input rows")
    forbidden = {"private_witness", "known_answer_scalar", "scalar", "solver", "outcome", "labels", "seed", "curve_seed", "generator_index", "key_index", "configuration", "order_certificate", "construction_trace", "construction_candidates", "source_artifacts", "source_paths", "solution_certificates", "events", "trace", "witness", "proof", "certificate"}
    def public_only(value):
        if isinstance(value, dict):
            require(not forbidden.intersection(value), "restricted key leaked into public features")
            for v in value.values(): public_only(v)
        elif isinstance(value, list):
            for v in value: public_only(v)
    for feature in features:
        require(set(feature) == {"schema", "feature_id", "split_component", "split", "inputs"}, "unexpected public row shape")
        require(set(feature["inputs"]) == {"pdp_domain", "family", "n", "m", "twist", "field", "curve", "generator", "target", "symmetry", "factor_base"}, "unexpected Koblitz public input projection")
        public_only(feature["inputs"])
    native_causal_check(fixture,indexes)
    return {"schema": SCHEMA, "status":"passed", "software_cells":4, "scientific_corpus_rows":0, "verified_cloud_cells":0,
            "timing_comparison_claim":False, "hardware_instruction_claim":False, "source_index_rows":len(indexes),
            "unique_features":len(features), "cells":rows, "pipeline_contract_pass":1, "telemetry_training_cost_eligible":False}

def execute(a):
    fixture = Path(a.fixture).resolve(strict=True)
    state = load(fixture / "prepared.json")
    require(state["phase"] == "ready_to_commit", "fixture was not prepared")
    require(sha(Path(__file__)) == state["driver_sha256"], "driver changed after freeze")
    require(not (fixture / "execution-started.json").exists(), "one attempt only; reconcile existing outputs instead of retry")
    runner = Runner(fixture, state["binary"], state["binary_sha256"])
    try:
        require(sha(fixture / state["gates"]) == state["gates_sha256"], "gate receipt changed")
        gates(runner, Path(state["source"]), state["source_commit"], state["binary_sha256"], fixture / state["gates"])
        protocol = fixture / "protocol.json"
        require(sha(protocol) == state["protocol_file_sha256"], "frozen protocol changed")
        require(sha(state["checker"]) == state["checker_sha256"], "canonical checker changed")
        checker_root = Path(state["checker"]).parent.parent
        require(sha(checker_root / "SOURCE.json") == state["checker_source_sha256"], "canonical source receipt changed")
        for rel, digest in load(checker_root / "SOURCE.json")["sha256"].items():
            require(sha(checker_root / rel) == digest, "canonical tool/schema changed after freeze")
        for row in load(fixture / "prepare-inventory.json")["files"]:
            require(sha(fixture / row["path"]) == row["sha256"], "prepared receipt/artifact changed")
        require(re.fullmatch(r"[0-9a-f]{40}", a.protocol_commit) is not None, "protocol commit must be full immutable SHA")
        blob = runner.git("committed-protocol", a.protocol_repo, "show", a.protocol_commit + ":" + a.protocol_git_path)
        require(blob == protocol.read_bytes(), "commit does not contain exact frozen protocol")
        runner.command("protocol-validate-committed", [*state["validator_argv_prefix"], state["checker"], "validate", protocol], 120)
        require(not live_processes(Path(state["binary"]), fixture), "existing generator/worker; reconcile rather than duplicate")
        require(not any((fixture / p).exists() for p in ["plan.json", "run", "export", "report"]), "prior CLI outputs exist; do not restart")
        save(fixture / "execution-started.json", {"at": stamp(), "pid": os.getpid(), "protocol_commit":a.protocol_commit, "protocol_repository":a.protocol_repo, "protocol_git_path":a.protocol_git_path})
        b = state["binary"]
        runner.command("plan", [b,"dataset","plan","--out",fixture/"plan.json","--n","23","--m","3","--families","koblitz","--solvers","f4,f5","--curve-seeds","1","--generators","1","--keys","1","--seed","42","--factor-base-size","4","--degree-bound","3","--work-limit","16384","--protocol-sha256",state["protocol_file_sha256"]])
        pre = json_output(runner.command("preflight", [b,"dataset","preflight","--plan",fixture/"plan.json","--protocol",protocol]))
        require(pre["source_commit"] == state["source_commit"] and pre["binary_sha256"] == state["binary_sha256"] and pre["source_dirty"] is False, "embedded binary provenance mismatch")
        require(pre["planned_cells"] == 4 and pre["cloud_permissions_checked"] is False, "preflight wrong scope")
        runner.command("generate", [b,"dataset","generate","--plan",fixture/"plan.json","--protocol",protocol,"--out",fixture/"run","--max-cases","4","--shard-index","0","--shard-count","1","--seconds-per-case","60","--memory-mib","256","--trace-mib","128"],300)
        runner.command("verify", [b,"dataset","verify","--out",fixture/"run"],120)
        status = json_output(runner.command("status",[b,"dataset","status","--out",fixture/"run"]))
        require(status["completed_receipts"] == 4 and status["incomplete_attempts"] == [] and status["campaign_attempts_complete"] is True, "status incomplete")
        runner.command("export",[b,"dataset","export","--out",fixture/"run","--destination",fixture/"export"],120)
        runner.command("report",[b,"dataset","report","--out",fixture/"run","--destination",fixture/"report"],120)
        require(sha(protocol) == state["protocol_file_sha256"], "protocol mutated during CLI")
        source_gate(runner, Path(state["source"]), state["source_commit"], state["source_tree"])
        analysis = analyze(fixture)
        checkpoint_analysis(fixture,state,"native-analysis-checkpoint.json",analysis)
        policy_analysis = run_policy_extension(runner,state,fixture)
        checkpoint_analysis(fixture,state,"policy-analysis-checkpoint.json",policy_analysis)
        validate_pipeline_evidence(fixture)
        source_gate(runner, Path(state["source"]), state["source_commit"], state["source_tree"])
        save(fixture/"policy-analysis.json",policy_analysis)
        save(fixture / "analysis.json", analysis)
        write_phase_rows(fixture,state,analysis,policy_analysis)
        runner.command("experiment-export", [*state["validator_argv_prefix"], state["checker"], "from-rows", fixture/"functional-rows.jsonl",
            "--protocol",protocol,"--out",fixture/"experiment.export.json","--export-id","koblitz-m3-cli-functional-smoke"],120)
        bind_success_case_metadata(fixture)
        runner.command("experiment-export-check", [*state["validator_argv_prefix"], state["checker"], "check-export",fixture/"experiment.export.json",
            "--protocol",protocol,"--verify-files"],120)
        inventory(fixture, "inventory.json")
        print(json.dumps({"phase":"complete", "analysis":str(fixture/"analysis.json"), "cloud_launches":0}))
    except BaseException as exc:
        save(fixture / "execution-failed.json", {"at":stamp(), "error":str(exc), "no_retry":True, "scientific_corpus_rows":0, "cleanup":runner.last_cleanup})
        inventory(fixture, "execution-failed-inventory.json", runner.last_cleanup["settled"], runner.last_cleanup.get("unresolved"))
        try:
            failure_export(fixture)
        except Exception as diagnostic_error:
            save(fixture/"failure-export-error.json",{"error":str(diagnostic_error),"original_failure_preserved":True})
        raise

# Insert into the reviewed driver; no standalone entry point or alternate gates.
POLICY_RECIPES = [
    {"kind":"legacy"},
    {"kind":"sampled_x_projected_control","candidate_limit":128},
    {"kind":"binary_subspace_intersection","basis":[1,2,4],"candidate_limit":128},
    {"kind":"binary_frobenius_union_intersection","basis":[1,2,4],"candidate_limit":128},
]
POLICY_PANELS = [
    {"id":"legacy","n":7,"m":2,"families":["koblitz","binary","prime"],"solvers":["enumerate"],"policies":None,"cells":6},
    {"id":"policies","n":7,"m":2,"families":["koblitz","binary","prime"],"solvers":["enumerate"],"policies":POLICY_RECIPES,"cells":24},
    {"id":"domains","n":7,"m":2,"families":["prime"],"solvers":["pkm","double-large-prime"],"policies":POLICY_RECIPES[:2],"cells":8},
    {"id":"causal","n":7,"m":3,"families":["prime"],"solvers":["f4"],"policies":POLICY_RECIPES[:2],"cells":4},
]
POLICY_FIELDS_FORBIDDEN = {"sampling_seed","construction_context","construction_candidates","construction_counters","construction_status","factor_base_membership_logs","private_witness","known_answer_scalar","generator_multiplier","seed","curve_seed","generator_index","key_index","solver_outcome","outcome","labels","source_artifacts","solution_certificates","order_certificate","construction_trace"}

def policy_json_bytes(value, pretty=False):
    return (json.dumps(value, sort_keys=pretty, ensure_ascii=False,
                       indent=2 if pretty else None, separators=None if pretty else (",", ":")) + ("\n" if pretty else "")).encode()

def policy_rows(path):
    return [json.loads(line) for line in path.read_text().splitlines() if line]

def policy_projection_check(value):
    if isinstance(value, dict):
        require(not POLICY_FIELDS_FORBIDDEN.intersection(value), "restricted construction field in model input")
        for item in value.values(): policy_projection_check(item)
    elif isinstance(value, list):
        for item in value: policy_projection_check(item)

def policy_expected(panel, index):
    require(type(index) is int and 0 <= index < panel["cells"], "cell index outside frozen policy panel")
    solvers, policies = panel["solvers"], panel["policies"]
    solver = solvers[index % len(solvers)]; index //= len(solvers)
    recipe = policies[index % len(policies)] if policies else None
    if policies: index //= len(policies)
    twist = bool(index % 2); index //= 2
    return panel["families"][index], twist, solver, recipe

def policy_commands(state, fixture, panel):
    root = fixture/"policy"/panel["id"]
    b, plan, run = state["binary"], str(root/"plan.json"), str(root/"run")
    plan_args = [b,"dataset","plan","--out",plan,"--n",str(panel["n"]),"--m",str(panel["m"]),"--families",",".join(panel["families"]),"--solvers",",".join(panel["solvers"]),"--curve-seeds","1","--generators","1","--keys","1","--seed","42","--factor-base-size","4","--degree-bound","3","--work-limit","16384","--protocol-sha256",state["protocol_file_sha256"]]
    if panel["policies"] is not None: plan_args += ["--factor-base-policies",str(root/"recipes.json")]
    return {
        "plan":plan_args,
        "preflight":[b,"dataset","preflight","--plan",plan,"--protocol",str(fixture/"protocol.json")],
        "generate":[b,"dataset","generate","--plan",plan,"--protocol",str(fixture/"protocol.json"),"--out",run,"--max-cases",str(panel["cells"]),"--shard-index","0","--shard-count","1","--seconds-per-case","60","--memory-mib","256","--trace-mib","128"],
        "verify":[b,"dataset","verify","--out",run],
        "status":[b,"dataset","status","--out",run],
        "export":[b,"dataset","export","--out",run,"--destination",str(root/"export")],
        "report":[b,"dataset","report","--out",run,"--destination",str(root/"report")],
        "publish":[b,"dataset","publish","--out",run,"--remote","file://"+str(root/"remote")],
    }

def policy_check_features(panel, channel, rows):
    for row in rows:
        inputs=row["inputs"]; policy_projection_check(inputs)
        require(row["feature_id"]==hashlib.sha256(json.dumps(inputs,sort_keys=True,separators=(",",":"),ensure_ascii=False).encode()).hexdigest(),"public feature identity mismatch")
        initial=inputs if channel=="features.jsonl" else inputs["initial"]
        if panel["policies"]:
            require(initial["source_factor_base_policy"] in panel["policies"], "lost source recipe")
        else: require("source_factor_base_policy" not in initial, "legacy identity changed")
    if panel["id"]=="causal" and channel!="features.jsonl":
        require(len(rows)>0, "non-vacuous causal/algebraic CLI coverage missing; retain failure, no resample")

def policy_check_channel_bindings(rows, records, targets, static_rows):
    features={row["feature_id"]:row for row in rows}
    source_targets={row["target_id"]:row for row in targets}
    originals={row["feature_id"]:row["inputs"] for row in static_rows}
    require(len(features)==len(rows),"duplicate channel feature identity")
    require(len({row["record_id"] for row in records})==len(records),"duplicate channel record")
    used=set()
    for record in records:
        target=source_targets[record["source_target_id"]]
        identifier=record["feature_id"]
        if identifier is None: continue
        require(identifier in features and target["feature_id"] in originals,"unbound channel feature/source")
        require(features[identifier]["inputs"]["initial"]==originals[target["feature_id"]],"channel initial differs from exact source recipe/problem")
        used.add(identifier)
    require(used==set(features),"channel feature has no admitted source record")

def policy_dataset_schedule(state, fixture):
    schedule=[]
    for panel in POLICY_PANELS:
        root=fixture/"policy"/panel["id"]
        schedule.extend(("policy-"+panel["id"]+"-"+phase,argv,None)
            for phase,argv in policy_commands(state,fixture,panel).items())
        receipts=sorted((root/"run/receipts").glob("*.json"))
        require(len(receipts)==panel["cells"],"missing restore source receipt")
        for i,receipt in enumerate(receipts):
            restored=root/"restored"/str(i)
            schedule += [
                (f"policy-{panel['id']}-restore-{i}",[state["binary"],"dataset","restore","--receipt",str(receipt),"--out",str(restored)],None),
                (f"policy-{panel['id']}-restored-verify-{i}",[state["binary"],"dataset","verify","--out",str(restored)],None),
            ]
    wrong=fixture/"policy/policies";error="factor-base recipe or control seed differs from frozen config"
    schedule += [
        ("policy-wrong-recipe-verify",[state["binary"],"dataset","verify","--out",str(wrong/"wrong-recipe-run")],error),
        ("policy-wrong-recipe-restore",[state["binary"],"dataset","restore","--receipt",str(wrong/"adversarial-wrong-recipe-receipt.json"),"--out",str(wrong/"wrong-recipe-restore")],error),
    ]
    return schedule

def checked_dataset_inventory(state, fixture, native_actual):
    schedule=[(name,argv,None) for name,argv in expected_cli_argv(state,fixture).items()]
    if (fixture/"policy").exists(): schedule += policy_dataset_schedule(state,fixture)
    require(len(native_actual)==len(schedule),"seven unique native CLI receipts plus the exact policy command schedule required")
    previous=parse_time(load(fixture/"protocol.json")["freeze"]["frozen_at"])
    for (name,argv,error),(directory,seen) in zip(schedule,native_actual):
        require(directory.name.endswith("-"+name) and seen==argv,"dataset command order or identity changed")
        receipt=checked_receipt(directory,argv,state,expected_error=error)
        require(parse_time(receipt["started_at"])>=previous,"dataset command order/freeze boundary violated")
        previous=parse_time(receipt["finished_at"])
    return native_actual[:7]

def policy_check_targets(panel, source_run_sha, bindings, indexes, targets, features):
    require(len(indexes)==len(targets)==panel["cells"],"missing target/index rows")
    by_id={row["target_id"]:row for row in targets}
    by_cell={row["cell"]:row for row in indexes}
    by_feature={row["feature_id"]:row for row in features}
    require(len(by_id)==len(targets) and len(by_cell)==len(indexes) and len(by_feature)==len(features),"duplicate export identities")
    used=set()
    for i,binding in enumerate(bindings):
        manifest_sha,artifacts=binding
        identifier=hashlib.sha256(f"{source_run_sha}\n{manifest_sha}\n{i}".encode()).hexdigest()
        index=by_cell[f"cells/{i:012}"];target=by_id[identifier]
        require(index["record_id"]==index["target_id"]==identifier,"source/target identity mismatch")
        require(index["source_manifest_sha256"]==manifest_sha and index["source_run_sha256"]==source_run_sha,"source provenance mismatch")
        require(index["source_artifacts"]==artifacts and index["status"]=="verified_sealed_cell","lost source artifacts")
        family,twist,solver,recipe=policy_expected(panel,i)
        require(target["solver"]==solver and target["feature_id"]==index["feature_id"],"target routing mismatch")
        unsupported=family=="prime" and recipe is not None and recipe["kind"].startswith("binary_")
        if recipe is not None: require(target["factor_base_construction"]["policy"]==recipe,"target recipe substitution")
        else: require("factor_base_construction" not in target,"legacy target identity changed")
        if unsupported:
            require(target["feature_status"]=="unsupported_factor_base_policy" and target["feature_id"] is None and target["pdp_domain"] is None,"unsupported public feature")
            require(target["factor_base_construction"]["status"]=="unsupported","unsupported provenance missing")
        else:
            require(target["feature_status"]=="available" and target["feature_id"] in by_feature,"supported target lost public feature")
            public=by_feature[target["feature_id"]]["inputs"];used.add(target["feature_id"])
            require(public["family"]==family and public["twist"]==twist and public["n"]==panel["n"] and public["m"]==panel["m"],"public feature problem mismatch")
            require(public.get("source_factor_base_policy")==recipe,"target bound to different recipe feature")
            domain={"pkm":"prime-kummer-full-curve","double-large-prime":"cyclic-subgroup-identity-signed-large-prime"}.get(solver,"cyclic-subgroup-factor-base")
            require(target["pdp_domain"]==public["pdp_domain"]==domain,"solver public domain mismatch")
    require(used==set(by_feature),"unreferenced or missing public feature")

def policy_analyze(fixture, state):
    results=[]
    for panel in POLICY_PANELS:
        root=fixture/"policy"/panel["id"]
        outputs={phase:json_output(next((fixture/"commands").glob("*-policy-"+panel["id"]+"-"+phase))) for phase in ["plan","preflight","verify","status","export","report"]}
        require(outputs["plan"]["planned_cells"]==panel["cells"],"plan output count mismatch")
        pre=outputs["preflight"]
        require(pre["source_commit"]==state["source_commit"] and pre["binary_sha256"]==state["binary_sha256"] and pre["source_dirty"] is False,"offline policy provenance mismatch")
        require(pre["plan_file_sha256"]==sha(root/"plan.json") and pre["protocol_file_sha256"]==state["protocol_file_sha256"] and pre["planned_cells"]==panel["cells"] and pre["cloud_permissions_checked"] is False,"offline preflight mismatch")
        for phase in ["verify","status"]:
            report=outputs[phase]
            require(report["completed_receipts"]==panel["cells"] and report["incomplete_attempts"]==[] and report["campaign_attempts_complete"] is True and report["artifact_hashes_verified"] is True,"offline policy campaign incomplete")
        require(outputs["verify"]["known_answer_replay_performed"] is True,"offline verify skipped replay")
        summary=load(root/"report/summary.json")
        require(outputs["report"]["summary"]==summary,"offline report substitution")
        require(summary["campaign_attempts_complete"] is True and summary["planned_method_cells"]==panel["cells"],"offline report incomplete")
        require(outputs["report"]["summary_artifact"]["sha256"]==sha(root/"report/summary.json"),"offline report receipt mismatch")
        for artifact in summary["artifacts"]:
            require(not Path(artifact["path"]).is_absolute() and ".." not in Path(artifact["path"]).parts,"unsafe report path")
            path=root/"report"/artifact["path"]
            require(sha(path)==artifact["sha256"] and path.stat().st_size==artifact["bytes"],"offline report artifact substitution")
        for phase,argv in policy_commands(state,fixture,panel).items():
            matches=list((fixture/"commands").glob("*-policy-"+panel["id"]+"-"+phase))
            require(len(matches)==1,"missing or duplicate policy command receipt")
            checked_receipt(matches[0],argv,state)
        plan=load(root/"plan.json")
        require(plan["schema"] == ("crypto.ml-dataset.plan/v2" if panel["policies"] else "crypto.ml-dataset.plan/v1"), "policy version mismatch")
        require(plan.get("factor_base_policies") == panel["policies"], "recipe axis mismatch")
        require(sorted(p.name for p in (root/"run/cells").iterdir()) == [f"{i:012}" for i in range(panel["cells"])], "missing or extra policy cells")
        bindings=[]
        for i in range(panel["cells"]):
            cell=root/"run/cells"/f"{i:012}"
            m=load(cell/"manifest.json"); c=m["config"]
            bindings.append((sha(cell/"manifest.json"),m["artifacts"]))
            family,twist,solver,recipe=policy_expected(panel,i)
            expected_config={"family":family,"n":panel["n"],"m":panel["m"],"twist":twist,
                "curve_seed":0,"generator_index":0,"key_index":0,"seed":42,"factor_base_size":4,
                "degree_bound":3,"work_limit":16384,"solver":solver}
            if recipe is not None: expected_config["factor_base_policy"]=recipe
            require(c==expected_config and load(cell/"config.json")==c,"complete policy configuration mismatch")
            for artifact in m["artifacts"]:
                require(not Path(artifact["path"]).is_absolute() and ".." not in Path(artifact["path"]).parts,"unsafe cell artifact path")
                path=cell/artifact["path"]
                require(sha(path)==artifact["sha256"] and path.stat().st_size==artifact["bytes"],"changed retained cell artifact")
            require((c["family"],c["twist"],c["solver"],c.get("factor_base_policy")) == (family,twist,solver,recipe), "policy cell mapping mismatch")
            base=load(cell/"factor-base.json")
            unsupported=family=="prime" and recipe is not None and recipe["kind"].startswith("binary_")
            require((base["construction_status"]=="unsupported") == unsupported, "unexpected unsupported recipe")
            if unsupported: require(m["outcome"]["solver_completed"] is False, "unsupported solver completion")
            require(base.get("policy") == recipe, "base/config recipe mismatch")
            results.append({"panel":panel["id"],"cell":i,"family":family,"twist":twist,"solver":solver,"policy":recipe,"explicit_unsupported":unsupported,"manifest_sha256":sha(cell/"manifest.json")})
        receipts=sorted((root/"run/receipts").glob("*.json"))
        require(len(receipts)==panel["cells"],"missing genuine source publication")
        for i,receipt in enumerate(receipts):
            restored=root/"restored"/str(i)
            for phase,argv in {
                "restore":[state["binary"],"dataset","restore","--receipt",str(receipt),"--out",str(restored)],
                "restored-verify":[state["binary"],"dataset","verify","--out",str(restored)],
            }.items():
                matches=list((fixture/"commands").glob(f"*-policy-{panel['id']}-{phase}-{i}"))
                require(len(matches)==1,"missing restore command evidence")
                checked_receipt(matches[0],argv,state)
                if phase=="restore":
                    observed=json_output(matches[0])
                    require(observed["native_replay_performed"] is True and observed["remote_rechecked"] is True,"restore did not independently replay")
                else:
                    restored_verify_report(json_output(matches[0]))
            original=load(receipt)
            require(sha(restored/"cells"/f"{i:012}"/"manifest.json")==original["manifest_sha256"],"restored cell identity mismatch")
        metadata=load(root/"export/manifest.json")
        require(outputs["export"]["export"]==metadata,"offline export metadata substitution")
        require(metadata["feature_policy"]["version"] == ("strict-structural-allowlist-v7" if panel["policies"] else "strict-structural-allowlist-v6"), "feature policy version mismatch")
        for artifact in metadata["artifacts"]:
            path=root/"export"/artifact["path"]
            require(not Path(artifact["path"]).is_absolute() and ".." not in Path(artifact["path"]).parts,"unsafe export artifact")
            require(sha(path)==artifact["sha256"] and path.stat().st_size==artifact["bytes"],"changed export artifact")
        policy_check_targets(panel,sha(root/"run/run.json"),bindings,
            policy_rows(root/"export/restricted/index.jsonl"),policy_rows(root/"export/restricted/targets.jsonl"),
            policy_rows(root/"export/features.jsonl"))
        for channel in ["features.jsonl","prefix-features.jsonl","algebraic-features.jsonl"]:
            rows=policy_rows(root/"export"/channel)
            policy_check_features(panel,channel,rows)
            if channel!="features.jsonl":
                records="restricted/"+("prefix-records.jsonl" if channel=="prefix-features.jsonl" else "algebraic-records.jsonl")
                policy_check_channel_bindings(rows,policy_rows(root/"export"/records),
                    policy_rows(root/"export/restricted/targets.jsonl"),policy_rows(root/"export/features.jsonl"))
        if panel["id"]=="domains":
            domains={r["inputs"]["pdp_domain"] for r in policy_rows(root/"export/features.jsonl")}
            require(domains=={"prime-kummer-full-curve","cyclic-subgroup-identity-signed-large-prime"}, "both exact alternative domains must be retained")
    error="factor-base recipe or control seed differs from frozen config"
    wrong=fixture/"policy/policies"
    negatives={
        "verify":[state["binary"],"dataset","verify","--out",str(wrong/"wrong-recipe-run")],
        "restore":[state["binary"],"dataset","restore","--receipt",str(wrong/"adversarial-wrong-recipe-receipt.json"),"--out",str(wrong/"wrong-recipe-restore")],
    }
    for phase,argv in negatives.items():
        matches=list((fixture/"commands").glob("*-policy-wrong-recipe-"+phase))
        require(len(matches)==1,"missing or duplicate negative command")
        checked_receipt(matches[0],argv,state,expected_error=error)
    require(not (wrong/"wrong-recipe-restore/cells/000000000000/manifest.json").exists(),"invalid restore activation")
    # Explicit legacy must reproduce the entire legacy public problem and actual signed base.
    for i in range(6):
        family=i//2;twist=i%2
        old=fixture/"policy/legacy/run/cells"/f"{i:012}"
        new=fixture/"policy/policies/run/cells"/f"{family*8+twist*4:012}"
        require(load(old/"public.json")==load(new/"public.json"), "explicit legacy changed public problem")
        a,b=load(old/"factor-base.json"),load(new/"factor-base.json")
        for field in ["points","negation_action","frobenius_action","frobenius_power","orbits"]:
            require(a[field]==b[field], "explicit legacy changed actual domain")
    require(sum(row["explicit_unsupported"] for row in results)==4,"explicit unsupported coverage mismatch")
    return {"software_cells":42,"scientific_corpus_rows":0,"verified_cloud_cells":0,"rows":results}

def run_policy_extension(runner, state, fixture):
    # Called only after the original four-cell pipeline and native analyze pass.
    # Uses the SAME runner/deadline, source gates, binary, protocol and cleanup owner.
    require(load(fixture/"protocol.json")["extensions"]["policy_panels"] == POLICY_PANELS, "unfrozen policy schedule")
    (fixture/"policy").mkdir()
    for panel in POLICY_PANELS:
        root=fixture/"policy"/panel["id"];root.mkdir()
        if panel["policies"] is not None: save(root/"recipes.json",panel["policies"])
        for phase,argv in policy_commands(state,fixture,panel).items():
            result=runner.command("policy-"+panel["id"]+"-"+phase,argv,timeout= min(1200, panel["cells"]*60+60) if phase=="generate" else 120)
            if phase=="preflight":
                pre=json_output(result)
                require(pre["planned_cells"]==panel["cells"] and pre["cloud_permissions_checked"] is False,"wrong policy preflight")
                require(pre["source_commit"]==state["source_commit"] and pre["binary_sha256"]==state["binary_sha256"] and pre["source_dirty"] is False,"policy preflight source/binary mismatch")
            if phase=="status":
                status=json_output(result)
                require(status["completed_receipts"]==panel["cells"] and status["incomplete_attempts"]==[] and status["campaign_attempts_complete"] is True,"incomplete policy attempt")
        receipts=sorted((root/"run/receipts").glob("*.json"))
        require(len(receipts)==panel["cells"],"missing publication receipts")
        for i,receipt in enumerate(receipts):
            restored=root/"restored"/str(i)
            runner.command(f"policy-{panel['id']}-restore-{i}",[state["binary"],"dataset","restore","--receipt",receipt,"--out",restored],120)
            runner.command(f"policy-{panel['id']}-restored-verify-{i}",[state["binary"],"dataset","verify","--out",restored],120)
    policy_negative_commands(runner,state,fixture)
    return policy_analyze(fixture,state)

def policy_negative_fixture(fixture):
    # Adversarial test objects are labelled and never counted as producer receipts.
    # The replacement is a genuine base generated for another frozen recipe on
    # the same public problem; no curve arithmetic or resampling in the driver.
    root=fixture/"policy/policies"
    copied=root/"wrong-recipe-run"
    shutil.copytree(root/"run",copied)
    cell=copied/"cells/000000000000"
    donor=root/"run/cells/000000000001/factor-base.json"
    require(load(cell/"public.json")==load(donor.parent/"public.json"),"negative fixture must share exact public problem")
    original_sha=sha(cell/"factor-base.json")
    (cell/"factor-base.json").write_bytes(donor.read_bytes())
    manifest=load(cell/"manifest.json")
    for artifact in manifest["artifacts"]:
        if artifact["path"]=="factor-base.json":
            artifact["sha256"]=sha(donor);artifact["bytes"]=donor.stat().st_size
    (cell/"manifest.json").write_bytes(policy_json_bytes(manifest,True))
    original_receipts=[p for p in (root/"run/receipts").glob("*.json") if p.name.startswith("000000000000-")]
    require(len(original_receipts)==1,"negative fixture needs genuine source publication")
    receipt=load(original_receipts[0]); receipt.pop("catalog",None)
    remote=root/"remote"
    require(receipt["remote"]=="file://"+str(remote),"negative remote not fixture owned")
    def object_for(artifact, data):
        artifact={k:artifact[k] for k in ["role","path","sha256","bytes","encoding"]}
        artifact["sha256"]=hashlib.sha256(data).hexdigest();artifact["bytes"]=len(data)
        key="sha256/"+artifact["sha256"][:2]+"/"+artifact["sha256"]
        path=remote/key;path.parent.mkdir(parents=True,exist_ok=True)
        if path.exists(): require(path.read_bytes()==data,"negative content-address collision")
        else:
            with path.open("xb") as f: f.write(data);f.flush();os.fsync(f.fileno())
        require(sha(path)==artifact["sha256"],"negative fixture local readback mismatch")
        return {"name":receipt["namespace"]+"/"+artifact["path"],"artifact":artifact,"object_key":key,"roundtrip_verified":True}
    objects=[]
    for obj in receipt["objects"]:
        artifact=obj["artifact"]
        objects.append(object_for(artifact,(cell/artifact["path"]).read_bytes()))
    receipt["objects"]=objects;receipt["manifest_sha256"]=sha(cell/"manifest.json")
    record={k:receipt[k] for k in ["schema","remote","namespace","manifest_sha256","objects"]}
    rid=hashlib.sha256(policy_json_bytes(record)).hexdigest()
    completion={"role":"publication_completion","path":"publication/"+rid+".json","sha256":"", "bytes":0,"encoding":"json"}
    receipt["completion"]=object_for(completion,policy_json_bytes(record,True))
    confirmation={"schema":"ml-dataset.publication-confirmation/v1","remote":receipt["remote"],"namespace":receipt["namespace"],"manifest_sha256":receipt["manifest_sha256"],"completion":receipt["completion"]}
    payload=policy_json_bytes(confirmation,True)
    receipt["confirmation"]={"role":"publication_confirmation","path":"publication/verified-"+rid+".json","sha256":hashlib.sha256(payload).hexdigest(),"bytes":len(payload),"encoding":"json"}
    dest=root/"adversarial-wrong-recipe-receipt.json";save(dest,receipt)
    save(root/"adversarial-mutation.json",{"purpose":"negative software fixture; not a genuine producer publication acknowledgement","original_receipt_sha256":sha(original_receipts[0]),"source_factor_base_sha256":original_sha,"replacement_factor_base_sha256":sha(donor),"mutated_manifest_sha256":sha(cell/"manifest.json"),"mutated_receipt_sha256":sha(dest),"generator_invocations":0})
    return copied,dest

def policy_negative_commands(runner,state,fixture):
    copied,receipt=policy_negative_fixture(fixture)
    error="factor-base recipe or control seed differs from frozen config"
    runner.command("policy-wrong-recipe-verify",[state["binary"],"dataset","verify","--out",copied],120,expected_error=error)
    restored=fixture/"policy/policies/wrong-recipe-restore"
    runner.command("policy-wrong-recipe-restore",[state["binary"],"dataset","restore","--receipt",receipt,"--out",restored],120,expected_error=error)
    require(not (restored/"cells/000000000000/manifest.json").exists(),"wrong recipe activated on restore")


# Embedded in the existing guarded driver. Diagnostics only; never runs dataset work.
def failure_schedule(fixture):
    cases=[]
    for i in range(4):
        cases.append({"id":"native-"+str(i),"panel":"native","index":i,"root":fixture,
            "family":"koblitz","n":23,"m":3,"solver":["f4","f5"][i%2],"twist":i>=2,"recipe":None})
    for panel in POLICY_PANELS:
        for i in range(panel["cells"]):
            family,twist,solver,recipe=policy_expected(panel,i)
            cases.append({"id":panel["id"]+"-"+str(i),"panel":panel["id"],"index":i,
                "root":fixture/"policy"/panel["id"],"family":family,"n":panel["n"],"m":panel["m"],
                "solver":solver,"twist":twist,"recipe":recipe})
    return cases

def diagnostic_json(path,cap=4*1024*1024):
    require(path.is_file() and not path.is_symlink() and path.stat().st_size<=cap,"diagnostic JSON absent, symlinked or over4MiB cap")
    def reject_constant(value): raise ValueError("nonfinite diagnostic JSON "+value)
    return json.loads(path.read_text(),parse_constant=reject_constant)

def diagnostic_file(fixture,path,kind="raw"):
    require(path.is_file() and not path.is_symlink(),"diagnostic artifact not a regular file")
    require(path.resolve().is_relative_to(fixture.resolve()),"diagnostic artifact escapes fixture")
    return {"path":str(path.relative_to(fixture)),"sha256":sha(path),"bytes":path.stat().st_size,"kind":kind}

def diagnostic_command(fixture,state,name,argv):
    matches=list((fixture/"commands").glob("*-"+name))
    if not matches: return None
    require(len(matches)==1,"duplicate diagnostic command")
    path=matches[0];row=diagnostic_json(path/"receipt.json")
    require(row["argv"]==argv and diagnostic_json(path/"argv.json")==argv,"diagnostic argv mismatch")
    require((path/"argv.nul").read_bytes()==b"\0".join(os.fsencode(a) for a in argv)+b"\0","diagnostic NUL argv mismatch")
    require(row["binary_sha256_before"]==row["binary_sha256_after"]==state["binary_sha256"],"diagnostic binary substitution")
    require(row["protocol_sha256_before"]==row["protocol_sha256_after"]==state["protocol_file_sha256"],"diagnostic protocol substitution")
    require(sha(path/"stdout")==row["stdout_sha256"] and sha(path/"stderr")==row["stderr_sha256"],"diagnostic command output substitution")
    require(parse_time(row["finished_at"])>=parse_time(row["started_at"]),"diagnostic reversed command times")
    return row

def diagnostic_checkpoint(fixture,state,case,manifest_sha):
    name="native-analysis-checkpoint.json" if case["panel"]=="native" else "policy-analysis-checkpoint.json"
    path=fixture/name
    if not path.exists():return False
    checkpoint=diagnostic_json(path)
    require(checkpoint["source_commit"]==state["source_commit"] and checkpoint["driver_sha256"]==state["driver_sha256"] and checkpoint["protocol_file_sha256"]==state["protocol_file_sha256"],"checkpoint instrument substitution")
    data=checkpoint["analysis"]
    if case["panel"]=="native":
        require(data["status"]=="passed" and data["software_cells"]==4,"invalid native checkpoint")
        rows=[row for row in data["cells"] if int(row["cell"])==case["index"]]
    else:
        require(data["software_cells"]==42,"invalid policy checkpoint")
        rows=[row for row in data["rows"] if row["panel"]==case["panel"] and row["cell"]==case["index"]]
    require(len(rows)==1 and rows[0]["manifest_sha256"]==manifest_sha,"checkpoint manifest substitution")
    return True

def checkpoint_analysis(fixture,state,name,analysis):
    save(fixture/name,{"source_commit":state["source_commit"],"driver_sha256":state["driver_sha256"],
        "protocol_file_sha256":state["protocol_file_sha256"],"analysis":analysis})

def failure_export(fixture):
    # Called after failure receipt/inventory retention. No subprocess, cleanup,
    # arithmetic replay, solver resume, deadline change or remote I/O here.
    state=diagnostic_json(fixture/"prepared.json");protocol=diagnostic_json(fixture/"protocol.json")
    require(sha(fixture/"protocol.json")==state["protocol_file_sha256"],"failure protocol changed")
    require(sha(Path(__file__))==state["driver_sha256"],"failure driver changed")
    require([a["id"] for a in protocol["arms"]]==list(PHASE_ARMS),"failure adapter needs the frozen source/replay phases")
    failure=diagnostic_json(fixture/"execution-failed.json")
    settled=failure.get("cleanup",{}).get("settled") is True and not failure.get("cleanup",{}).get("unresolved")
    files=[];seen=set();cases=[];runs=[]
    def retain(path,kind="raw"):
        if path.exists() and path.is_file() and not path.is_symlink():
            row=diagnostic_file(fixture,path,kind)
            if row["path"] not in seen:seen.add(row["path"]);files.append(row)
            return row
        return None
    # Snapshot references preserve raw charges, caps, logs and failures even when
    # they cannot be projected into truthful numerical canonical measurements.
    inventory_path=fixture/"execution-failed-inventory.json"
    retain(inventory_path,"receipt");retain(fixture/"execution-failed.json","receipt")
    if inventory_path.exists():
        for item in diagnostic_json(inventory_path)["files"]:
            relative=Path(item["path"])
            if relative.is_absolute() or ".." in relative.parts:continue
            path=fixture/relative
            if path.is_file() and not path.is_symlink():retain(path)
    for case in failure_schedule(fixture):
        cell=case["root"]/"run/cells"/f"{case['index']:012}"
        observation={k:v for k,v in case.items() if k!="root"}
        observation.update(state="attempt_status_unknown",canonical_row=False,evidence=[],observed_outcome=None,reason=None)
        for name in ["attempt.json","worker-process.json","execution-context.json","result.json","manifest.json","config.json"]:
            artifact=retain(cell/name)
            if artifact:observation["evidence"].append(artifact)
        try:
            if case["panel"]=="native":name="generate";argv=expected_cli_argv(state,fixture)[name]
            else:
                panel=next(p for p in POLICY_PANELS if p["id"]==case["panel"])
                name="policy-"+case["panel"]+"-generate";argv=policy_commands(state,fixture,panel)["generate"]
            command=diagnostic_command(fixture,state,name,argv)
            if command is None:
                observation["state"]="not_attempted" if settled and not cell.exists() else "attempt_status_unknown"
                observation["reason"]="no generator command receipt; no worker state invented"
            elif not (cell/"attempt.json").exists():
                observation["state"]="not_attempted" if settled and not cell.exists() else "attempt_status_unknown"
                observation["reason"]="no retained attempt marker"
            else:
                attempt=diagnostic_json(cell/"attempt.json");config=diagnostic_json(cell/"config.json")
                expected={"family":case["family"],"n":case["n"],"m":case["m"],"twist":case["twist"],"curve_seed":0,"generator_index":0,"key_index":0,"seed":42,"factor_base_size":4,"degree_bound":3,"work_limit":16384,"solver":case["solver"]}
                if case["recipe"] is not None:expected["factor_base_policy"]=case["recipe"]
                require(config==expected,"attempt config differs from frozen case")
                binding=diagnostic_json(case["root"]/"run/run.json")
                require(binding["schema"]=="crypto.ml-dataset.run/v1" and binding["source_commit"]==state["source_commit"] and binding["binary_sha256"]==state["binary_sha256"],"run instrument substitution")
                require((binding["shard_index"],binding["shard_count"],binding["seconds_per_case"],binding["memory_mib"],binding["trace_mib"])==(0,1,60,256,128),"run limits changed")
                pre_name="preflight" if case["panel"]=="native" else "policy-"+case["panel"]+"-preflight"
                pre_argv=expected_cli_argv(state,fixture)["preflight"] if case["panel"]=="native" else policy_commands(state,fixture,panel)["preflight"]
                pre=diagnostic_command(fixture,state,pre_name,pre_argv)
                require(pre is not None and type(pre["exit_code"]) is int and pre["exit_code"]==0,"preflight missing or failed")
                pre_dir=next((fixture/"commands").glob("*-"+pre_name));preflight=diagnostic_json(pre_dir/"stdout")
                require(preflight["plan_sha256"]==binding["plan_sha256"] and preflight["plan_file_sha256"]==sha(case["root"]/"plan.json") and preflight["protocol_file_sha256"]==state["protocol_file_sha256"] and preflight["source_commit"]==state["source_commit"] and preflight["binary_sha256"]==state["binary_sha256"],"preflight plan/instrument substitution")
                require(attempt["schema"]=="crypto.ml-dataset.attempt/v1" and attempt["cell_index"]==case["index"] and attempt["plan_sha256"]==binding["plan_sha256"] and attempt["status"]=="attempt_started","invalid attempt marker")
                observation["state"]="attempted_unverified"
                if (cell/"result.json").exists():observation["observed_result"]=diagnostic_json(cell/"result.json")
                if not settled:
                    observation["reason"]="unresolved cleanup; no terminal row claimed"
                elif not (cell/"manifest.json").exists():
                    observation["reason"]="attempt retained without terminal seal; command status not attributed to cell"
                else:
                    manifest=diagnostic_json(cell/"manifest.json")
                    require(manifest["schema"]=="crypto.ml-dataset.cell/v1" and manifest["config"]==config and manifest["cell_index"]==case["index"] and manifest["plan_sha256"]==binding["plan_sha256"],"manifest attempt/config mismatch")
                    outcome=manifest["outcome"];observation["observed_outcome"]=outcome
                    require(isinstance(outcome.get("supervisor"),dict) and "exit_code" in outcome["supervisor"],"terminal supervisor observation missing")
                    for artifact in manifest["artifacts"]:
                        relative=Path(artifact["path"])
                        require(not relative.is_absolute() and ".." not in relative.parts,"unsafe manifest member")
                        actual=retain(cell/relative)
                        require(actual and actual["sha256"]==artifact["sha256"] and actual["bytes"]==artifact["bytes"],"manifest member changed")
                        observation["evidence"].append(actual)
                    verified=diagnostic_checkpoint(fixture,state,case,sha(cell/"manifest.json"))
                    if verified:status="succeeded";classification="verified_completed"
                    elif outcome.get("status")=="timeout":status="timeout";classification="attempted_timeout"
                    elif outcome.get("solver_completed") is False:status="failed";classification="attempted_terminal_failure"
                    else:
                        observation["reason"]="sealed worker outcome lacks retained full contract checkpoint; no cancellation invented"
                        observation["phase_states"]={"producer-source":"attempted_unverified","verifier-replay":"attempt_status_unknown"}
                        cases.append(observation)
                        continue
                    observation["state"]=classification;observation["canonical_row"]=True
                    counters={k:v for k,v in outcome.get("counters",{}).items() if type(v) in [int,str]}
                    row={"run_id":"producer-source-"+case["id"],"arm":"producer-source","instance_id":case["id"],"seed":42,"replicate":0,"warmup":False,"status":status,"outcome_class":"completed" if status in ["succeeded","failed"] else "not_completed","attempts":1,"resumed":0,"checkpoint":None,
                        "measurements":{"counted":{"cell_contract_pass":1} if verified else {},"wall_seconds":None,"wall_valid":False,"cpu_user_seconds":None,"cpu_sys_seconds":None,"max_rss_bytes":None,"counters":counters},
                        "isolation":{"tier":None,"contended":None,"blockers":["failure diagnostics; no timing claim"]},"verification":{"status":"verified" if verified else "not_run","verifier":"retained source-bound driver checkpoint" if verified else None},"exclusion":{"excluded":not verified,"rule":"functional validation incomplete or failed" if not verified else None},"artifacts":observation["evidence"],"started_at":None,"finished_at":None,"worker":None,"backend_ref":str(cell.relative_to(fixture))}
                    runs.append(row)
                    observation["phase_states"]={"producer-source":classification,"verifier-replay":"attempt_status_unknown"}
                    if verified:
                        replay_artifacts=phase_replay_evidence(fixture,state,case)
                        replay=json.loads(json.dumps(row));replay.update(run_id="verifier-replay-"+case["id"],arm="verifier-replay",artifacts=replay_artifacts)
                        replay["measurements"]["counters"]={}
                        runs.append(replay);observation["phase_states"]["verifier-replay"]="verified_completed"
        except Exception as error:
            observation["reason"]=str(error)
            if not observation["canonical_row"]:observation["state"]="attempt_status_unknown"
        observation.setdefault("phase_states",{"producer-source":observation["state"],"verifier-replay":"attempt_status_unknown"})
        cases.append(observation)
    sidecar={"schema":"crypto.ml-dataset.failure-flow/v1","planned_software_cells":46,"planned_validation_observations":92,"cleanup":failure.get("cleanup"),"settled":settled,"cases":cases,
        "counts":{key:sum(c["state"]==key for c in cases) for key in sorted({c["state"] for c in cases})},"scope":"all46 planned cases; absent/unknown/nonterminal cases are not fabricated canonical attempts","scientific_corpus_rows":0,"verified_cloud_cells":0}
    sidecar_path=fixture/"failure-flow.json";save(sidecar_path,sidecar);retain(sidecar_path,"summary")
    script=diagnostic_file(fixture,fixture/"retained-analysis.py","analysis") if (fixture/"retained-analysis.py").exists() else None
    # Analysis code is captured by content, not a mutable absolute external path.
    if script is None:
        target=fixture/"retained-analysis.py"
        with target.open("xb") as out:out.write(Path(__file__).read_bytes());out.flush();os.fsync(out.fileno())
        script=diagnostic_file(fixture,target,"analysis")
    require(script["sha256"]==state["driver_sha256"],"retained analysis script substitution")
    exclusions={}
    for row in runs:
        if row["exclusion"]["excluded"]:exclusions.setdefault(row["exclusion"]["rule"],[]).append(row["run_id"])
    export={"schema":"experiment.export/v1","export_id":"software-validation-partial-failure","created":stamp(),"protocol":{"path":"protocol.json","sha256":protocol["freeze"]["protocol_sha256"],"protocol_id":protocol["protocol_id"],"version":protocol["version"]},
        "producer":{"runtime":"codex_cli","model":None,"session":None,"role":"diagnostic-adapter"},"source":{"repo":protocol["repository"],"commit":state["source_commit"],"dirty":None},"backend":"local","host":{"path":None,"sha256":None,"class_id":None,"summary":{"cpu_model":None,"logical_cpus":None,"memory_bytes":None,"kernel":None,"hypervisor":None,"isolation_tier_max":None}},
        "arms":[{"id":a["id"],"role":a["role"],"binary_sha256":a["instrument"]["binary_sha256"],"commit":a["instrument"]["commit"]} for a in protocol["arms"]],"runs":runs,
        "flow":{"planned":len(runs),"attempted":len(runs),"completed":sum(row["outcome_class"]=="completed" for row in runs),"not_completed":sum(row["outcome_class"]=="not_completed" for row in runs),"interrupted":0,"resumed":0,"excluded":[{"rule":key,"run_ids":values} for key,values in sorted(exclusions.items())]},
        "files":files,"analysis":{"script":script,"blinded":False,"blinding_key_sha256":None,"outputs":[diagnostic_file(fixture,sidecar_path,"summary")],"unmasked_at":None},"notes":[{"kind":"observation","by":"failure adapter","text":"Canonical flow.planned counts represented terminal attempt rows under the pinned checker. Actual frozen workload is46, bound in failure-flow.json and backend_record. Unknown/unattempted cases have no invented attempt, status or measurements. The official checker will report shortfall where rows are missing. A failed row is outcome_class completed under the pinned checker, not a verified solver result."}],"reviews_dir":"reviews","backend_record":sidecar}
    save(fixture/"failure.experiment.export.json",export)
    save(fixture/"failure-export-status.json",{"status":"written_not_yet_officially_checked","represented_rows":len(runs),"planned_software_cells":46,"cleanup_settled":settled,"no_dataset_commands":True})
    return export

def validate_failure_export(fixture):
    # Run explicitly in the pinned offline jsonschema environment; no dataset,
    # generator, subprocess, ownership cleanup or clock extension is performed.
    import importlib.util,importlib.metadata
    require(importlib.metadata.version("jsonschema")=="4.26.0","wrong validator dependency")
    state=diagnostic_json(fixture/"prepared.json");checker=Path(state["checker"])
    source=diagnostic_json(checker.parent.parent/"SOURCE.json")
    require(source["commit"]=="37e193bf8b3e673e0942709794f44d28e6ab66fb" and sha(checker)==state["checker_sha256"],"checker pin changed")
    for rel,digest in source["sha256"].items():require(sha(checker.parent.parent/rel)==digest,"checker source/schema changed")
    spec=importlib.util.spec_from_file_location("pinned_failure_export_checker",checker)
    tool=importlib.util.module_from_spec(spec);spec.loader.exec_module(tool)
    export=diagnostic_json(fixture/"failure.experiment.export.json",128*1024*1024);protocol=diagnostic_json(fixture/"protocol.json")
    schema_findings=tool.schema_errors(export,"experiment.export/v1")
    findings=schema_findings+tool.schema_errors(protocol,"experiment.protocol/v1")
    if not findings:findings=tool.lint_export(export,protocol,fixture,True)
    result={"checker_sha256":sha(checker),"checker_commit":source["commit"],"export_sha256":sha(fixture/"failure.experiment.export.json"),"protocol_file_sha256":sha(fixture/"protocol.json"),"schema_findings":schema_findings,"findings":findings,"accepted":not findings,"method":"pinned official schema_errors plus lint_export(... verify_files=True)","scientific_execution":False}
    save(fixture/"failure-export-validation.json",result)
    return result


def native_causal_check(fixture,indexes):
    features=policy_rows(fixture/"export/prefix-features.jsonl")
    records=policy_rows(fixture/"export/restricted/prefix-records.jsonl")
    by_feature={row["feature_id"]:row for row in features}
    require(features and len(by_feature)==len(features),"empty/duplicate native prefix channel")
    by_target={row["target_id"]:row for row in indexes}
    require(len(by_target)==4,"native causal source targets missing")
    used=set();counts={key:0 for key in by_target};identities=set()
    forbidden={"label","labels","sum","matches_target","seed","curve_seed","generator_index","key_index","scalar","known_answer_scalar","solver","outcome","source_artifacts","source_capture_sequence","signed_sequence","source_factor_base_policy"}
    def check_public(value):
        if isinstance(value,dict):
            require(not forbidden.intersection(value),"native causal restricted/recipe leakage")
            for item in value.values():check_public(item)
        elif isinstance(value,list):
            for item in value:check_public(item)
    for row in features:
        require(set(row)=={"schema","feature_id","split_component","split","inputs"},"native prefix public shape")
        require(row["schema"]=="ml-dataset.prefix-features/v1","native prefix schema")
        inputs=row["inputs"];check_public(inputs)
        require(set(inputs)=={"schema","task","static_inputs","action_query","prefix_state"},"native causal input shape")
        query=inputs["action_query"];prefix=inputs["prefix_state"]
        require(set(query)=={"tuple","indices","summand","operand"} and set(prefix)=={"completed_additions","accumulator","infinity"},"native causal action/state shape")
        for point in [query["operand"],prefix["accumulator"]]:
            require(point is None or isinstance(point,dict) and set(point)=={"x","y"} and all(isinstance(v,str) and v.isdecimal() for v in point.values()),"native causal point shape")
        require(type(query["tuple"]) is int and 0<=query["tuple"]<64 and type(query["summand"]) is int and 0<=query["summand"]<3,"native causal query bounds")
        require(query["indices"]==[query["tuple"]//16,(query["tuple"]//4)%4,query["tuple"]%4],"native causal tuple mapping")
        require(type(prefix["completed_additions"]) is int and prefix["completed_additions"]==query["tuple"]*3+query["summand"] and type(prefix["infinity"]) is bool and prefix["infinity"]==(prefix["accumulator"] is None),"native causal prefix bounds")
        require(inputs["schema"]=="crypto.ml-dataset.koblitz-causal-input/v1" and inputs["task"]=="signed-addition-outcome","native causal task mismatch")
        require(hashlib.sha256(json.dumps(inputs,sort_keys=True,separators=(",",":"),ensure_ascii=False).encode()).hexdigest()==row["feature_id"],"native prefix digest mismatch")
    source_run_sha=sha(fixture/"run/run.json")
    for record in records:
        identifier=record["record_id"];require(identifier not in identities,"duplicate native prefix record");identities.add(identifier)
        target=record["source_target_id"];require(target in by_target,"native prefix dangling target")
        index=by_target[target];relative=Path(index["cell"])
        require(not relative.is_absolute() and ".." not in relative.parts,"unsafe native source path")
        cell=fixture/"run"/relative;manifest=load(cell/"manifest.json");config=load(cell/"config.json")
        require("factor_base_policy" not in config and manifest["config"]==config,"native legacy recipe changed")
        expected_target=hashlib.sha256(f"{source_run_sha}\n{sha(cell/'manifest.json')}\n{manifest['cell_index']}".encode()).hexdigest()
        require(target==expected_target and index["source_run_sha256"]==source_run_sha and index["source_manifest_sha256"]==sha(cell/"manifest.json") and index["source_artifacts"]==manifest["artifacts"],"native prefix source binding mismatch")
        for artifact in manifest["artifacts"]:
            relative=Path(artifact["path"])
            require(not relative.is_absolute() and ".." not in relative.parts,"unsafe source artifact")
            require(sha(cell/relative)==artifact["sha256"] and (cell/relative).stat().st_size==artifact["bytes"],"native source artifact changed")
        require(any(a["path"]=="public.json" for a in manifest["artifacts"]),"unauthenticated native public input")
        artifacts=[a for a in manifest["artifacts"] if a["path"] in {"live/events.jsonl","live/committed.json","capture-binding.json","capture-limits.json"}]
        require(len(artifacts)==4 and record["source_event_artifacts"]==artifacts,"native prefix custody references missing")
        for artifact in artifacts:
            require(sha(cell/artifact["path"])==artifact["sha256"] and (cell/artifact["path"]).stat().st_size==artifact["bytes"],"native prefix custody changed")
        fid=record["feature_id"]
        if fid is None:continue
        require(fid in by_feature,"native prefix missing public feature")
        feature=by_feature[fid];require(record["split"]==feature["split"] and record["split_component"]==feature["split_component"],"native prefix partition mismatch")
        r=record["record"];require(r["kind"]=="koblitz-signed-addition" and r["admission_status"]=="verified_group_step","native prefix not verified group evidence")
        public=load(cell/"public.json");static=feature["inputs"]["static_inputs"]
        require(static["family"]=="koblitz" and static["n"]==config["n"] and static["m"]==3 and static["twist"]==config["twist"],"native prefix original shape mismatch")
        require(static["target"]==public["target"] and static["curve"]["a"]==public["curve"]["a"] and static["curve"]["b"]==public["curve"]["b"],"native prefix original math substitution")
        require(any(a["path"]=="koblitz-m3-input.json" for a in manifest["artifacts"]),"native canonical input absent")
        original=load(cell/"koblitz-m3-input.json")
        expected_static={"family":"koblitz","m":3,"n":original["field_degree"],"twist":config["twist"],
            "field":{"characteristic":"2","degree":original["field_degree"],"basis":"polynomial","irreducible_low_terms":original["irreducible_low_terms"]},
            "curve":{"model":"y2+xy=x3+a*x2+b","a":str(original["curve_a"]),"b":str(original["curve_b"])},
            "target":{"x":str(original["target"][0]),"y":str(original["target"][1])},
            "signed_domain":[{"x":str(x),"y":str(y)} for x,y in original["signed_domain"]],"x_codes":[str(x) for x in original["codes"]]}
        require(static==expected_static,"native causal static input differs from authenticated original")
        used.add(fid);counts[target]+=1
    require(used==set(by_feature) and all(counts.values()),"missing native per-cell group prefix channel")

def bind_success_case_metadata(fixture):
    cases=[{key:value for key,value in c.items() if key!="root"} for c in failure_schedule(fixture)]
    path=fixture/"software-cases.json";save(path,{"schema":"crypto.ml-dataset.software-cases/v1","cases":cases,"planned":46,"scientific_corpus_rows":0})
    output=fixture/"experiment.export.json";value=load(output)
    require(len(value["runs"])==92 and {r["instance_id"] for r in value["runs"]}=={c["id"] for c in cases},"canonical success export lost exact schedule")
    state=load(fixture/"prepared.json");mapping={case["id"]:case for case in failure_schedule(fixture)}
    require({(r["arm"],r["instance_id"]) for r in value["runs"]}=={(arm,key) for arm in PHASE_ARMS for key in mapping},"canonical phase rectangle changed")
    for row in value["runs"]:
        case=mapping[row["instance_id"]];cell=case["root"]/"run/cells"/f"{case['index']:012}"
        if row["arm"]=="producer-source":
            row["artifacts"]=[diagnostic_file(fixture,cell/"manifest.json","receipt")]
            outcome=load(cell/"manifest.json")["outcome"]
            row["measurements"]["counters"]={k:v for k,v in outcome.get("counters",{}).items() if type(v) in [int,str]}
        else:
            row["artifacts"]=phase_replay_evidence(fixture,state,case);row["measurements"]["counters"]={}
    script=fixture/"retained-analysis.py"
    if not script.exists():
        with script.open("xb") as target:
            target.write(Path(__file__).read_bytes());target.flush();os.fsync(target.fileno())
    require(sha(script)==state["driver_sha256"],"success analysis script substitution")
    value["analysis"]["script"]=diagnostic_file(fixture,script,"analysis")
    value["backend_record"]={"software_cases":cases,"functional_validation_only":True,"planned_dataset_cells":46,"planned_validation_observations":92,"additional_solver_runs":0}
    value["files"].append(diagnostic_file(fixture,path,"receipt"))
    # Preserve the canonical converter output before attaching phase evidence.
    raw=fixture/"experiment.export.unbound.json"
    with raw.open("xb") as target:
        target.write(output.read_bytes());target.flush();os.fsync(target.fileno())
    value["files"].append(diagnostic_file(fixture,raw,"raw"))
    staged=fixture/"experiment.export.bound.tmp.json";save(staged,value)
    require(sha(output)==sha(raw),"canonical export changed during evidence binding")
    os.replace(staged,output)


def restored_verify_report(report):
    require(type(report.get("completed_receipts")) is int and report["completed_receipts"]==1
        and report.get("incomplete_attempts")==[] and report.get("artifact_hashes_verified") is True
        and report.get("known_answer_replay_performed") is True,"restored verify lacks one complete authenticated replay")

# Two actual existing execution phases, not two solver populations.
PHASE_ARMS=("producer-source","verifier-replay")
def phase_replay_evidence(fixture,state,case):
    panel=None if case["panel"]=="native" else next(p for p in POLICY_PANELS if p["id"]==case["panel"])
    name="verify" if panel is None else "policy-"+panel["id"]+"-verify"
    argv=expected_cli_argv(state,fixture)["verify"] if panel is None else policy_commands(state,fixture,panel)["verify"]
    evidence=[]
    def checked(name,argv):
        paths=list((fixture/"commands").glob("*-"+name));require(len(paths)==1,"missing/duplicate phase replay receipt")
        path=paths[0];checked_receipt(path,argv,state)
        for leaf in ["receipt.json","argv.json","argv.nul","stdout","stderr"]:evidence.append(diagnostic_file(fixture,path/leaf,"receipt"))
        return json_output(path)
    report=checked(name,argv);count=4 if panel is None else panel["cells"]
    require(report["completed_receipts"]==count and report["incomplete_attempts"]==[] and report["campaign_attempts_complete"] is True and report["artifact_hashes_verified"] is True and report["known_answer_replay_performed"] is True,"phase replay receipt incomplete")
    if panel is not None:
        root=case["root"];i=case["index"];receipts=sorted((root/"run/receipts").glob("*.json"))
        require(len(receipts)==count,"phase publication receipts incomplete")
        receipt=receipts[i];restored=root/"restored"/str(i)
        result=checked(f"policy-{panel['id']}-restore-{i}",[state["binary"],"dataset","restore","--receipt",str(receipt),"--out",str(restored)])
        require(result["native_replay_performed"] is True and result["remote_rechecked"] is True,"phase restore replay absent")
        restored_verify_report(checked(f"policy-{panel['id']}-restored-verify-{i}",[state["binary"],"dataset","verify","--out",str(restored)]))
        source=case["root"]/"run/cells"/f"{i:012}"/"manifest.json"
        require(sha(restored/"cells"/f"{i:012}"/"manifest.json")==sha(source)==load(receipt)["manifest_sha256"],"phase restore source identity mismatch")
        evidence.append(diagnostic_file(fixture,receipt,"receipt"))
    return evidence

def write_phase_rows(fixture,state,analysis,policy_analysis):
    native={int(r["cell"]):r for r in analysis["cells"]}
    policy={(r["panel"],r["cell"]):r for r in policy_analysis["rows"]}
    with (fixture/"functional-rows.jsonl").open("x") as output:
        for case in failure_schedule(fixture):
            observed=native[case["index"]] if case["panel"]=="native" else policy[(case["panel"],case["index"])]
            source=case["root"]/"run/cells"/f"{case['index']:012}"/"manifest.json"
            require(sha(source)==observed["manifest_sha256"],"phase source changed after complete analysis")
            phase_replay_evidence(fixture,state,case)
            for arm in PHASE_ARMS:
                output.write(json.dumps({"run_id":arm+"-"+case["id"],"arm":arm,"instance_id":case["id"],"seed":42,"replicate":0,"warmup":False,"status":"succeeded","verification":"verified","cell_contract_pass":1,"pipeline_contract_pass":1,"source_manifest_sha256":observed["manifest_sha256"]},sort_keys=True)+"\n")


def main():
    p = argparse.ArgumentParser(description=__doc__)
    sub = p.add_subparsers(dest="mode", required=True)
    a = sub.add_parser("prepare")
    for name in ["fixture","source-dir","source-commit","binary","binary-sha256","gates","protocol-draft","checker","checker-sha256","validator-prefix"]:
        a.add_argument("--"+name, required=True)
    a = sub.add_parser("execute")
    for name in ["fixture","protocol-repo","protocol-commit","protocol-git-path"]:
        a.add_argument("--"+name, required=True)
    a = sub.add_parser("analyze")
    a.add_argument("--fixture", required=True)
    a = sub.add_parser("check-failure-export")
    a.add_argument("--fixture", required=True)
    args = p.parse_args()
    if args.mode == "prepare": prepare(args)
    elif args.mode == "execute": execute(args)
    elif args.mode == "check-failure-export":
        print(json.dumps(validate_failure_export(Path(args.fixture)),indent=2))
    else:
        fixture=Path(args.fixture)
        print(json.dumps({"native":analyze(fixture),"policy":policy_analyze(fixture,load(fixture/"prepared.json"))},indent=2))

if __name__ == "__main__":
    signal.signal(signal.SIGTERM, interrupted)
    try: main()
    except (Exception, KeyboardInterrupt) as exc:
        print(f"REFUSED/FAILED: {exc}", file=sys.stderr)
        sys.exit(1)
