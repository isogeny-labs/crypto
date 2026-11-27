#!/usr/bin/env python3
"""Linux per-target cold runner; requires an external frozen admission JSON."""

import argparse
import base64
import hashlib
import json
import os
import platform
import resource
import signal
import subprocess
import sys
import tempfile
import threading
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parent
MANIFEST_SHA256 = "b9d5cb5dca1f46131616f5703284edfc769645ee48a5e187f929a1c3e187f64d"
ARMS = {
    "queued_aps": "--single-queued-aps",
    "folded_rho": "--single-rho-folded",
    "unfolded_rho": "--single-rho-unfolded",
}


def sha256(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def load_inputs():
    manifest_path = ROOT / "heldout_v2_seed_manifest.json"
    public_path = ROOT / "heldout_v2_public_targets.json"
    assert sha256(manifest_path) == MANIFEST_SHA256
    manifest = json.loads(manifest_path.read_bytes())
    assert sha256(public_path) == manifest["public_targets_sha256"]
    public = json.loads(public_path.read_bytes())
    assert len(public) == 48 and len(manifest["streams"]) == 240
    return manifest, public


def physical_core(cpu):
    base = Path(f"/sys/devices/system/cpu/cpu{cpu}/topology")
    return ((base / "physical_package_id").read_text().strip(),
            (base / "core_id").read_text().strip())


def preflight(admission, manifest, require_aa=True):
    if sys.platform != "linux":
        raise RuntimeError("held-out runner requires Linux for affinity and wait4 RSS")
    if require_aa and admission.get("admitted") is not True:
        raise RuntimeError("admission must explicitly be true")
    if admission.get("study") != manifest["study"]:
        raise RuntimeError("study mismatch")
    if admission.get("manifest_sha256") != MANIFEST_SHA256:
        raise RuntimeError("manifest hash mismatch")
    if not admission.get("source_commit") or not admission.get("compiler_and_flags"):
        raise RuntimeError("source commit and compiler flags must be recorded")
    binary = Path(admission["binary_path"]).resolve()
    if sha256(binary) != admission["binary_sha256"]:
        raise RuntimeError("binary hash mismatch")
    cpus = admission["physical_cpu_ids"]
    if len(cpus) != 4 or len(set(cpus)) != 4:
        raise RuntimeError("exactly four distinct CPU IDs required")
    if len({physical_core(cpu) for cpu in cpus}) != 4:
        raise RuntimeError("CPU IDs do not name distinct physical cores")
    if not set(cpus).issubset(os.sched_getaffinity(0)):
        raise RuntimeError("selected cores unavailable to runner")
    memory = admission["memory_limit_bytes"]
    if not isinstance(memory, int) or memory != 2 * 1024**3:
        raise RuntimeError("frozen 2 GiB address-space ceiling required")
    if admission.get("dedicated_host_evidence_sha256") is None:
        raise RuntimeError("dedicated-host evidence hash missing")
    if sha256(admission["dedicated_host_evidence_path"]) != admission[
            "dedicated_host_evidence_sha256"]:
        raise RuntimeError("dedicated-host evidence hash mismatch")
    if require_aa and sha256(admission["aa_control_evidence_path"]) != admission[
            "aa_control_evidence_sha256"]:
        raise RuntimeError("same-host A/A control evidence hash mismatch")
    if admission.get("timeout_seconds") != 30:
        raise RuntimeError("frozen 30-second per-call watchdog required")
    return binary, cpus, memory


def invoke(binary, cpus, memory, timeout_seconds, public_q, stream, arm):
    request = {"public_Q": public_q}
    if arm == "queued_aps":
        request["aps_source_seed_hex"] = stream["aps_source_seed_hex"]
    else:
        request["rho_walk_seed_hex"] = stream["rho_walk_seed_hex"]
        request["rho_lane_restart_seed_hex"] = stream[
            "rho_lane_restart_seed_hex"]
    input_bytes = (json.dumps(request, sort_keys=True, separators=(",", ":")) + "\n").encode()

    def limit_child():
        os.sched_setaffinity(0, cpus)
        resource.setrlimit(resource.RLIMIT_AS, (memory, memory))

    with tempfile.TemporaryFile() as stdout, tempfile.TemporaryFile() as stderr:
        started = time.perf_counter_ns()
        proc = subprocess.Popen([str(binary), ARMS[arm], "4"],
            stdin=subprocess.PIPE, stdout=stdout, stderr=stderr,
            preexec_fn=limit_child)
        input_error = None
        try:
            proc.stdin.write(input_bytes)
            proc.stdin.close()
        except BrokenPipeError:
            input_error = "child closed stdin before receiving the complete request"
        finally:
            if not proc.stdin.closed:
                try:
                    proc.stdin.close()
                except BrokenPipeError:
                    input_error = "child closed stdin before receiving the complete request"
        timed_out_event = threading.Event()
        child_reaped = threading.Event()

        def kill_on_timeout():
            if child_reaped.is_set():
                return
            try:
                # Popen.kill() polls and can reap the child before wait4,
                # losing the per-call CPU and peak-RSS receipt.
                os.kill(proc.pid, signal.SIGKILL)
                timed_out_event.set()
            except ProcessLookupError:
                pass

        timer = threading.Timer(timeout_seconds, kill_on_timeout)
        timer.start()
        try:
            pid, status, usage = os.wait4(proc.pid, 0)
            assert pid == proc.pid
        finally:
            child_reaped.set()
            timer.cancel()
        wall_ns = time.perf_counter_ns() - started
        exit_code = os.waitstatus_to_exitcode(status)
        proc.returncode = exit_code
        stdout.seek(0)
        stderr.seek(0)
        output_bytes = stdout.read()
        error_bytes = stderr.read()
    output = None
    try:
        output = json.loads(output_bytes)
    except (ValueError, UnicodeDecodeError):
        pass
    timed_out = timed_out_event.is_set() and exit_code == -signal.SIGKILL
    return {
        "arm": arm, "public_Q": public_q,
        "input": request, "input_sha256": hashlib.sha256(input_bytes).hexdigest(),
        "stdout": output_bytes.decode("utf-8", "replace"),
        "stdout_base64": base64.b64encode(output_bytes).decode("ascii"),
        "stdout_sha256": hashlib.sha256(output_bytes).hexdigest(),
        "stderr": error_bytes.decode("utf-8", "replace"),
        "stderr_base64": base64.b64encode(error_bytes).decode("ascii"),
        "stderr_sha256": hashlib.sha256(error_bytes).hexdigest(),
        "exit_code": exit_code, "timed_out": timed_out,
        "input_error": input_error,
        "external_cold_wall_ns": wall_ns,
        "child_cpu_ns": int((usage.ru_utime + usage.ru_stime) * 10**9),
        "peak_rss_bytes": int(usage.ru_maxrss) * 1024,
        "native_report": output,
    }


def planned_calls(manifest, public):
    for row in manifest["streams"]:
        case = row["case"]
        replicate = row["replicate"]
        for arm in row["arm_order"]:
            yield case, replicate, arm, public[case], row


def checked_prefix(path, calls, admission_sha256):
    """Accept only a complete prefix; charge an orphan start as interrupted."""
    completed = 0
    orphan = None
    with path.open() as handle:
        for line_number, line in enumerate(handle, 1):
            record = json.loads(line)
            if record.get("admission_sha256") != admission_sha256 or \
                    record.get("manifest_sha256") != MANIFEST_SHA256:
                raise RuntimeError(f"record {line_number}: admission or manifest mismatch")
            ordinal = record.get("ordinal")
            if ordinal != completed or completed >= len(calls):
                raise RuntimeError(f"record {line_number}: non-prefix ordinal")
            case, replicate, arm, q, _ = calls[completed]
            if any(record.get(k) != v for k, v in (
                    ("case", case), ("replicate", replicate), ("arm", arm),
                    ("public_Q", q))):
                raise RuntimeError(f"record {line_number}: scheduled call mismatch")
            if record.get("event") == "started":
                if orphan is not None:
                    raise RuntimeError(f"record {line_number}: nested start")
                orphan = record
            elif record.get("event") in ("completed", "interrupted"):
                if orphan is None:
                    raise RuntimeError(f"record {line_number}: missing start")
                orphan = None
                completed += 1
            else:
                raise RuntimeError(f"record {line_number}: unknown event")
    return completed, orphan


def append_record(handle, record):
    handle.write(json.dumps(record, sort_keys=True) + "\n")
    handle.flush()
    os.fsync(handle.fileno())


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--admission", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--execute", action="store_true")
    parser.add_argument("--resume", action="store_true")
    args = parser.parse_args()
    manifest, public = load_inputs()
    admission = json.loads(args.admission.read_bytes())
    binary, cpus, memory = preflight(admission, manifest)
    calls = list(planned_calls(manifest, public))
    if not args.execute:
        print(json.dumps({"preflight": "passed", "calls": len(calls),
            "binary_sha256": admission["binary_sha256"],
            "manifest_sha256": MANIFEST_SHA256, "cpus": cpus,
            "memory_limit_bytes": memory}, indent=2))
        return
    admission_sha256 = sha256(args.admission)
    if args.output.exists() and not args.resume:
        raise RuntimeError("output exists; pass --resume to check the prefix")
    if not args.output.exists() and args.resume:
        raise RuntimeError("no output exists to resume")
    args.output.parent.mkdir(parents=True, exist_ok=True)
    completed, orphan = checked_prefix(args.output, calls, admission_sha256) \
        if args.resume else (0, None)
    with args.output.open("a" if args.resume else "x") as handle:
        if orphan is not None:
            append_record(handle, {**orphan, "event": "interrupted",
                "reason": "runner stopped before child outcome was durably recorded",
                "external_cold_wall_ns": None, "child_cpu_ns": None,
                "peak_rss_bytes": None, "native_report": None})
            completed += 1
        for ordinal in range(completed, len(calls)):
            case, replicate, arm, q, stream = calls[ordinal]
            common = {"ordinal": ordinal, "case": case,
                "replicate": replicate, "manifest_sha256": MANIFEST_SHA256,
                "admission_sha256": admission_sha256,
                "binary_sha256": admission["binary_sha256"],
                "host": platform.node(), "arm": arm, "public_Q": q}
            append_record(handle, {**common, "event": "started",
                "start_wall_utc_ns": time.time_ns()})
            result = invoke(binary, cpus, memory, 30, q, stream, arm)
            append_record(handle, {**common, **result, "event": "completed"})
            print(f"{ordinal + 1}/{len(calls)} {case=} {replicate=} {arm=}",
                  file=sys.stderr, flush=True)


if __name__ == "__main__":
    main()
