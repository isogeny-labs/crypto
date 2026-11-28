#!/usr/bin/env python3
"""One externally authorized local attempt. No default authorization or retry."""
import ctypes
import fcntl
import stat
import hashlib
import json
import os
from pathlib import Path
import re
import resource
import signal
import subprocess
import sys
import time

REGISTRY = [
    "ml_dataset::original_registry::tests::" + name for name in [
        "collector_recovers_actual_checkpoint_without_terminal_registry",
        "complete_registry_coexists_without_duplicate_or_replacement",
        "absent_checkpoint_keeps_legacy_partial_public_role",
        "malformed_extra_duplicate_and_wrong_role_checkpoint_refuse",
        "hash_mutation_and_checkpoint_capacity_refuse",
        "final_missing_duplicate_and_role_conflicts_refuse",
        "path_reserved_and_receipt_publish_failure_is_returned",
    ]
] + ["ml_dataset::solvers::verified_worker_original_tests::original_registry_storage_failure_prevents_worker_token"]
ACCEPT = "ml_dataset::solvers::authentic_historical_child::authentic_historical_supervised_acceptance"


def digest(path, deadline=None):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        while True:
            if deadline is not None and time.monotonic() >= deadline:
                raise RuntimeError("overall deadline during hashing")
            part = f.read(1024 * 1024)
            if not part:
                break
            h.update(part)
    return h.hexdigest()


def write_json(path, value):
    with open(path, "x", encoding="utf-8") as f:
        json.dump(value, f, indent=2)
        f.write("\n")
        f.flush()
        os.fsync(f.fileno())
    fd = os.open(str(Path(path).parent), os.O_RDONLY | os.O_DIRECTORY)
    try:
        os.fsync(fd)
    finally:
        os.close(fd)


def proc_table():
    table = {}
    for entry in Path("/proc").iterdir():
        if not entry.name.isdigit():
            continue
        try:
            raw = (entry / "stat").read_text()
            fields = raw[raw.rfind(")") + 2:].split()
            table[int(entry.name)] = {"start": int(fields[19]), "ppid": int(fields[1]), "pgid": int(fields[2]), "state": fields[0]}
        except (OSError, ValueError, IndexError):
            continue  # Processes can disappear between enumeration and read.
    return table


def descendants():
    table = proc_table()
    owned = {os.getpid()}
    changed = True
    while changed:
        changed = False
        for pid, row in table.items():
            if pid not in owned and row["ppid"] in owned:
                owned.add(pid)
                changed = True
    return {pid: table[pid] for pid in owned if pid != os.getpid() and pid in table}


def cleanup():
    """Subreaper ancestry includes children orphaned into independent groups."""
    killed = []
    end = time.monotonic() + 5
    while time.monotonic() < end:
        owned = descendants()
        if not owned:
            break
        for pid, row in owned.items():
            current = proc_table().get(pid)
            if not current or current["start"] != row["start"]:
                continue
            pgid = row["pgid"]
            leader = owned.get(pgid)
            live_leader = proc_table().get(pgid)
            if pgid != os.getpgrp() and leader and live_leader and leader["start"] == live_leader["start"]:
                try:
                    os.killpg(pgid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
            try:
                os.kill(pid, signal.SIGKILL)
                killed.append({"pid": pid, "start": row["start"], "pgid": pgid})
            except ProcessLookupError:
                pass
        # Only called after the Popen direct child has been waited, or during
        # abort after that child's explicit wait. Reap adopted descendants.
        while True:
            try:
                pid, _ = os.waitpid(-1, os.WNOHANG)
                if pid == 0:
                    break
            except ChildProcessError:
                break
        time.sleep(.05)
    return {"killed": killed, "survivors": descendants()}


def main(auth_path):
    auth_path = Path(auth_path).resolve(strict=True)
    a = json.loads(auth_path.read_text())
    if a.get("approved") is not True:
        raise RuntimeError("authorization is false: no attempt started")
    fixed = {"attempt_limit": 1, "build_jobs": 2, "compile_seconds": 1200, "registry_seconds": 60,
             "acceptance_seconds": 300, "overall_seconds": 1560, "native_cells": 1,
             "snapshot": 1, "cloud_jobs": 0, "retries": 0, "acceptance_memory_bytes": 2147483648}
    for key, value in fixed.items():
        if a.get(key) != value:
            raise RuntimeError("authorization differs from frozen runner: " + key)
    if not a.get("authorization_text") or not re.fullmatch(r"[0-9a-f]{40}", a.get("source_commit", "")):
        raise RuntimeError("exact source and explicit authorization text required")
    if digest(Path(__file__)) != a["runner_sha256"]:
        raise RuntimeError("runner bytes differ from reviewed authorization")
    started = time.monotonic()
    hard_end = started + 1560
    overall = hard_end - 6  # Reserve descendant cleanup and final receipt time.
    repo = Path(a["repository"]).resolve(strict=True)
    out = Path(a["output"]).resolve()
    target = Path(a["cargo_target_dir"]).resolve()
    marker = Path(a["attempt_marker"]).resolve()
    if repo == out or repo in out.parents or repo == target or repo in target.parents:
        raise RuntimeError("output and build target must be outside source checkout")
    if any(x == y or x in y.parents or y in x.parents for x, y in ((out, target), (marker, out), (marker, target), (marker, repo))):
        raise RuntimeError("attempt paths overlap")
    fixed_target = Path("/var/tmp/crypto-ml-search-recovery-target-20261107")
    if target != fixed_target or not target.is_dir() or fixed_target.is_symlink():
        raise RuntimeError("only the predeclared existing release cache is allowed")
    if a.get("reuse_existing_cache") is not True or a.get("build_lock") != "/var/tmp/spot-host-heavy-build.lock":
        raise RuntimeError("cache reuse and host lock must be explicitly bound")
    if out.exists() or marker.exists():
        raise RuntimeError("prior side effects exist: reconcile; no automatic retry")
    source_manifest = Path(a["source_manifest"]).resolve(strict=True)
    if digest(source_manifest, overall) != a["source_manifest_sha256"]:
        raise RuntimeError("frozen input manifest hash mismatch")
    inputs = json.loads(source_manifest.read_text())
    if len(inputs) != a["source_manifest_entries"] or len(inputs) < 1:
        raise RuntimeError("frozen input count mismatch")
    libc = ctypes.CDLL(None, use_errno=True)
    if libc.prctl(36, 1, 0, 0, 0) != 0:  # Linux PR_SET_CHILD_SUBREAPER.
        raise RuntimeError("cannot establish descendant cleanup ownership")
    lock_fd = os.open(a["build_lock"], os.O_RDWR | os.O_CREAT | os.O_NOFOLLOW, 0o600)
    if not stat.S_ISREG(os.fstat(lock_fd).st_mode):
        raise RuntimeError("host lock must be a regular file")
    try:
        fcntl.flock(lock_fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
    except BlockingIOError:
        os.close(lock_fd)
        raise RuntimeError("host heavy-build lock has an active owner; no attempt started")
    # Also refuse a non-cooperating process explicitly using this target.
    for proc in Path("/proc").iterdir():
        if not proc.name.isdigit() or int(proc.name) == os.getpid():
            continue
        try:
            argv = (proc / "cmdline").read_bytes()
            executable = (proc / "exe").resolve()
            environment = (proc / "environ").read_bytes().split(b"\0")
            cache_env = ("CARGO_TARGET_DIR=" + str(target)).encode() in environment
            if str(target).encode() in argv or target in executable.parents or cache_env:
                raise RuntimeError("existing cache process active: PID " + proc.name)
        except (FileNotFoundError, PermissionError, ProcessLookupError):
            continue
    out.mkdir()
    env = {k: os.environ[k] for k in ("HOME", "RUSTUP_HOME", "CARGO_HOME") if k in os.environ}
    env.update({"PATH": a["trusted_path"], "CARGO_BUILD_JOBS": "2", "CARGO_TARGET_DIR": str(target), "RAYON_NUM_THREADS": "1", "RUST_TEST_THREADS": "1", "CARGO_INCREMENTAL": "0", "GIT_CONFIG_NOSYSTEM": "1", "GIT_CONFIG_GLOBAL": "/dev/null"})
    commands = []
    receipt = {"source_commit": a["source_commit"], "authorization_sha256": digest(auth_path), "source_manifest_sha256": a["source_manifest_sha256"], "commands": commands, "success": False, "cloud_jobs": 0, "retries": 0, "cache_reused": str(target), "host_build_lock": a["build_lock"], "lock_acquired_exclusive_nonblocking": True}
    direct = None

    def command(label, argv, seconds, extra=None, memory=None, required=True):
        nonlocal direct
        remaining = min(float(seconds), overall - time.monotonic())
        if remaining <= 0:
            raise RuntimeError("overall local attempt deadline exhausted")
        local_env = dict(env)
        if extra:
            local_env.update(extra)
        before = time.monotonic()
        log = out / (label + ".stdout")
        err = out / (label + ".stderr")
        parent_pid = os.getpid()
        def child_setup():
            if memory is not None:
                resource.setrlimit(resource.RLIMIT_AS, (memory, memory))
            if libc.prctl(1, signal.SIGKILL, 0, 0, 0) != 0 or os.getppid() != parent_pid:
                os._exit(125)
        item = {"label": label, "argv": argv, "timeout_seconds": remaining, "memory_limit_bytes": memory}
        commands.append(item)
        with open(log, "xb") as stdout, open(err, "xb") as stderr:
            direct = subprocess.Popen(argv, cwd=repo, env=local_env, stdout=stdout, stderr=stderr,
                                      start_new_session=True, preexec_fn=child_setup)
            item["pid"] = direct.pid
            item["start_identity"] = proc_table().get(direct.pid)
            deadline = before + remaining
            timed_out = False
            excessive_log = False
            while direct.poll() is None:
                timed_out = time.monotonic() >= deadline
                excessive_log = log.stat().st_size + err.stat().st_size > 64 * 1024 * 1024
                if timed_out or excessive_log:
                    try:
                        os.killpg(direct.pid, signal.SIGKILL)
                    except ProcessLookupError:
                        pass
                    break
                time.sleep(.1)
            code = direct.wait()
            timed_out = timed_out or time.monotonic() >= deadline
            excessive_log = excessive_log or log.stat().st_size + err.stat().st_size > 64 * 1024 * 1024
            direct = None
        reap = cleanup()
        item.update({"exit_code": code, "timed_out": timed_out, "log_limit_exceeded": excessive_log,
                     "elapsed_seconds": time.monotonic() - before, "cleanup": reap,
                     "stdout_sha256": digest(log, overall), "stderr_sha256": digest(err, overall)})
        if reap["survivors"] or timed_out or excessive_log or (required and code != 0):
            raise RuntimeError("bounded command failed: " + label)
        return log.read_text(errors="replace")

    def source_check(label):
        head = command(label + "-head", ["/usr/bin/git", "rev-parse", "HEAD"], 60).strip()
        clean = command(label + "-status", ["/usr/bin/git", "status", "--porcelain", "--untracked-files=all"], 60)
        if head != a["source_commit"] or clean:
            raise RuntimeError("source revision/cleanliness changed")
        for name, expected in inputs.items():
            relative = Path(name)
            if relative.is_absolute() or ".." in relative.parts:
                raise RuntimeError("unsafe source-manifest path")
            path = repo / relative
            if path.is_symlink() or not path.is_file():
                raise RuntimeError("missing/nonregular frozen input: " + name)
            sha = expected if isinstance(expected, str) else expected["sha256"]
            if digest(path, overall) != sha:
                raise RuntimeError("frozen input changed: " + name)

    def expired(signum, frame):
        raise RuntimeError("overall attempt watchdog expired")
    signal.signal(signal.SIGALRM, expired)
    signal.setitimer(signal.ITIMER_REAL, max(.001, overall - time.monotonic()))
    try:
        write_json(marker, {"authorization_sha256": digest(auth_path), "source_commit": a["source_commit"], "attempt": 1, "pid": os.getpid(), "started_unix": time.time()})
        previous = target / "release" / "crypto"
        if previous.is_symlink() or not previous.is_file() or digest(previous, overall) != a["prior_binary_sha256"]:
            raise RuntimeError("prior release binary custody mismatch")
        archive = out / "prior-release-crypto"
        with previous.open("rb") as source, archive.open("xb") as destination:
            while True:
                if time.monotonic() >= overall:
                    raise RuntimeError("deadline while archiving prior binary")
                block = source.read(1024 * 1024)
                if not block:
                    break
                destination.write(block)
            destination.flush()
            os.fsync(destination.fileno())
        if digest(archive, overall) != a["prior_binary_sha256"] or digest(previous, overall) != a["prior_binary_sha256"]:
            raise RuntimeError("prior binary changed during archive")
        receipt["prior_binary"] = {"path": str(previous), "sha256": a["prior_binary_sha256"], "archive": str(archive), "bytes": archive.stat().st_size}
        # The old CLI is archival evidence only; it can never satisfy libtest admission.
        source_check("before")
        publication = Path(a["publication_receipt"]).resolve(strict=True)
        if digest(publication, overall) != a["publication_receipt_sha256"]:
            raise RuntimeError("parent-verified publication receipt changed")
        published = json.loads(publication.read_text())
        if published.get("source_commit") != a["source_commit"] or published.get("verified") is not True or published.get("repository") != "isogeny-labs/crypto":
            raise RuntimeError("exact GitHub publication prerequisite absent")
        build = [a["cargo_path"], "+1.98", "test", "--release", "--lib", "--no-run", "--locked", "--offline", "--message-format=json"]
        messages = command("cargo-lib-tests", build, 1200)
        artifacts = []
        for line in messages.splitlines():
            try:
                row = json.loads(line)
            except json.JSONDecodeError:
                continue
            if row.get("reason") == "compiler-artifact" and row.get("target", {}).get("name") == "crypto" and "lib" in row.get("target", {}).get("kind", []) and row.get("profile", {}).get("test") is True and row.get("executable"):
                if Path(row["manifest_path"]).resolve() != repo / "Cargo.toml":
                    continue
                artifacts.append(Path(row["executable"]).resolve(strict=True))
        artifacts = sorted(set(artifacts))
        if len(artifacts) != 1 or target not in artifacts[0].parents:
            raise RuntimeError("Cargo JSON did not identify one exact local libtest artifact")
        binary = artifacts[0]
        source_check("after-build")
        binary_sha = digest(binary, overall)
        build_receipt = out / "exact-source-build.json"
        write_json(build_receipt, {"schema": "owned-authentic-exact-source-build/v1", "source_commit": a["source_commit"], "source_manifest_sha256": a["source_manifest_sha256"], "authorization_sha256": receipt["authorization_sha256"], "parent_verified_github_publication_receipt_sha256": digest(publication, overall), "cargo_command": build, "cargo_json_sha256": digest(out / "cargo-lib-tests.stdout", overall), "binary": str(binary), "binary_sha256": binary_sha, "binary_bytes": binary.stat().st_size, "source_clean_before_and_after": True})
        listing = command("libtest-list", [str(binary), "--list", "--format", "terse"], 15)
        available = [line.removesuffix(": test") for line in listing.splitlines() if line.endswith(": test")]
        for name in REGISTRY + [ACCEPT]:
            if available.count(name) != 1:
                raise RuntimeError("required exact test missing or duplicated: " + name)
        write_json(out / "selected-tests.json", {"registry": REGISTRY, "acceptance": ACCEPT, "listed_count": len(available)})
        registry_deadline = min(overall, time.monotonic() + 60)
        for index, name in enumerate(REGISTRY):
            if digest(binary, overall) != binary_sha:
                raise RuntimeError("libtest binary changed")
            text = command("registry-" + str(index), [str(binary), "--exact", name, "--test-threads=1", "--nocapture"], registry_deadline - time.monotonic())
            if not re.search(r"test result: ok\. 1 passed; 0 failed; 0 ignored;", text):
                raise RuntimeError("exact registry test did not execute once: " + name)
        if time.monotonic() >= registry_deadline:
            raise RuntimeError("registry controls exceeded shared 60-second allowance")
        source_check("before-native")
        if digest(binary, overall) != binary_sha:
            raise RuntimeError("libtest binary changed before native acceptance")
        native_root = out / "native-cell"
        control = out / "native-observation.json"
        extra = {"OWNED_HISTORICAL_CHILD_ROOT": str(native_root), "OWNED_HISTORICAL_CONTROL_PATH": str(control), "OWNED_HISTORICAL_SNAPSHOT": "1", "OWNED_HISTORICAL_BOUNDARY": "complete", "OWNED_HISTORICAL_BUILD_RECEIPT": str(build_receipt), "OWNED_HISTORICAL_BUILD_RECEIPT_SHA256": digest(build_receipt, overall)}
        text = command("authentic-acceptance", [str(binary), "--ignored", "--exact", ACCEPT, "--test-threads=1", "--nocapture"], 300, extra, 2 * 1024**3)
        if not re.search(r"test result: ok\. 1 passed; 0 failed; 0 ignored;", text):
            raise RuntimeError("acceptance test did not execute exactly once")
        acceptance = native_root.with_name(native_root.name + ".supervisor") / "public-acceptance.json"
        if not acceptance.is_file() or acceptance.is_symlink() or acceptance.stat().st_size > 1024 * 1024:
            raise RuntimeError("invalid acceptance receipt")
        if json.loads(acceptance.read_text()).get("passed") is not True:
            raise RuntimeError("actual public acceptance success receipt absent")
        source_check("after-native")
        if digest(binary, overall) != binary_sha:
            raise RuntimeError("libtest binary changed after native acceptance")
        receipt.update({"success": True, "binary_sha256": binary_sha, "build_receipt_sha256": digest(build_receipt, overall), "public_acceptance_sha256": digest(acceptance, overall)})
    except BaseException as e:
        receipt["error"] = type(e).__name__ + ": " + str(e)
        raise
    finally:
        signal.setitimer(signal.ITIMER_REAL, 0)
        if direct is not None:
            try:
                os.killpg(direct.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
            direct.wait()
        receipt["final_cleanup"] = cleanup()
        receipt["elapsed_seconds"] = time.monotonic() - started
        receipt["overall_deadline_exceeded"] = time.monotonic() > hard_end
        if receipt["final_cleanup"]["survivors"] or receipt["overall_deadline_exceeded"]:
            receipt["success"] = False
        receipt["host_lock_release"] = "after final cleanup and receipt serialization"
        try:
            write_json(out / "receipt.json", receipt)
        finally:
            fcntl.flock(lock_fd, fcntl.LOCK_UN)
            os.close(lock_fd)
        if not receipt["success"] and "error" not in receipt:
            raise RuntimeError("cleanup or overall deadline failed; see receipt")
        # No automatic resume/retry path: the marker, build, failed controls and
        # native artifacts remain even when this process exits unsuccessfully.


if __name__ == "__main__":
    if len(sys.argv) != 2:
        raise SystemExit("usage: run.py EXTERNALLY_BOUND_AUTHORIZATION.json")
    main(sys.argv[1])
