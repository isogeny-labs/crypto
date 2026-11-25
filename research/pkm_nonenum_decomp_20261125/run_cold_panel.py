#!/usr/bin/env python3
"""Run the frozen PKM F4 panel as cold, single-target ic bench processes.

Every attempted arm has an append-only launch and result receipt. A launch
without a result is unresolved and must be inspected before resuming.
"""

import argparse
import datetime as dt
import fcntl
import hashlib
import json
import math
import os
import platform
import signal
import subprocess
import sys
import time
from pathlib import Path


ROSTER_SHA256 = "271fdaf1b23b87fb5b2b8a6ecb7ac1f6492e146d7e8b4deb2f6f251d618d9123"
SOURCE_COMMIT = "4d5019bba92f075e2abdc961b1a86bcd07738cdf"
MANIFEST_SHA256 = "a2236a912fd094548d14125d4280053610c59de1898a709b4ef777b525ae9d1b"
TESTS = (
    "kummer_m3_basis_descent_complete_small_curve_census",
    "zero_grid_crossover_uses_public_basis_descent_path",
    "descent_roots_match_ground_truth_at_m3",
)
ARMS = (
    ("candidate_a", "pkm-semaev:m=3,mode=chain,engine=f4,descent=1,grid_crossover=0,trace=0,guess=0,degree=12,max_nnz=1000000"),
    ("exhaustive", "pkm-semaev:m=3,mode=chain,engine=exhaustive"),
    ("mitm", "mitm:m=3"),
    ("candidate_b", "pkm-semaev:m=3,mode=chain,engine=f4,descent=1,grid_crossover=0,trace=0,guess=0,degree=12,max_nnz=1000000"),
)
WALL_CAP_S = 60
RSS_CAP_KIB = 2 * 1024 * 1024
MAX_TRIALS = 2_000_000
LINALG = "incremental-gauss-full-rank-checked"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def read_json(path):
    return json.loads(path.read_bytes())


def append(path, row):
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("a", encoding="utf-8") as stream:
        stream.write(json.dumps(row, sort_keys=True) + "\n")
        stream.flush()
        os.fsync(stream.fileno())


def records(path):
    found = {}
    if path.exists():
        for line in path.read_text(encoding="utf-8").splitlines():
            row = json.loads(line)
            if row["id"] in found:
                raise ValueError(f"duplicate receipt {row['id']}")
            found[row["id"]] = row
    return found


def preserve(path, data):
    path.parent.mkdir(parents=True, exist_ok=True)
    if path.exists():
        if path.read_bytes() != data:
            raise ValueError(f"frozen input differs: {path}")
        return
    fd = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o644)
    with os.fdopen(fd, "wb") as stream:
        stream.write(data)
        stream.flush()
        os.fsync(stream.fileno())


def quiet(exclude=()):
    lines = subprocess.run(["ps", "-axo", "pid,pcpu,comm,args"], check=True,
                           capture_output=True, text=True).stdout.splitlines()[1:]
    busy = []
    for line in lines:
        parts = line.strip().split(None, 3)
        if len(parts) != 4:
            continue
        pid, cpu, comm, args = parts
        if int(pid) in exclude or int(pid) == os.getpid():
            continue
        name = Path(comm).name
        system = args.startswith(("/System/", "/Applications/", "/usr/libexec/"))
        if name in {"cargo", "rustc", "ic", "msolve", "p61_parity_probe", "pkm_mixed_ecdlp"} or (
            float(cpu) >= 20 and not system and name != "ps"
        ):
            busy.append({"pid": int(pid), "cpu_percent": float(cpu), "comm": name[:48]})
    return {"at_utc": dt.datetime.now(dt.timezone.utc).isoformat(),
            "loadavg": list(os.getloadavg()), "logical_cpus": os.cpu_count(),
            "competing_processes": busy}


def rss_kib(pid):
    value = subprocess.run(["ps", "-p", str(pid), "-o", "rss="], check=False,
                           capture_output=True, text=True).stdout.strip()
    return int(value) if value.isdigit() else 0


def run_one(command, stdout_path, stderr_path):
    started = time.perf_counter()
    peak_sampled = 0
    cap_hit = None
    contamination = None
    with stdout_path.open("xb") as out, stderr_path.open("xb") as err:
        proc = subprocess.Popen(command, stdout=out, stderr=err, start_new_session=True)
        last_load_check = started
        while True:
            child, status, usage = os.wait4(proc.pid, os.WNOHANG)
            if child:
                proc.returncode = os.waitstatus_to_exitcode(status)
                break
            now = time.perf_counter()
            peak_sampled = max(peak_sampled, rss_kib(proc.pid))
            if peak_sampled > RSS_CAP_KIB:
                cap_hit = "memory_cap"
            elif now - started > WALL_CAP_S:
                cap_hit = "wall_cap"
            if now - last_load_check >= 2:
                check = quiet(exclude=(proc.pid,))
                if check["competing_processes"]:
                    contamination = check
                last_load_check = now
            if cap_hit or contamination:
                try:
                    os.killpg(proc.pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
                _, status, usage = os.wait4(proc.pid, 0)
                proc.returncode = os.waitstatus_to_exitcode(status)
                break
            time.sleep(0.25)
    stdout, stderr = stdout_path.read_bytes(), stderr_path.read_bytes()
    os_peak = usage.ru_maxrss // 1024 if sys.platform == "darwin" else usage.ru_maxrss
    peak = max(peak_sampled, os_peak)
    if peak > RSS_CAP_KIB:
        cap_hit = "memory_cap"
    return {
        "exit_code": proc.returncode,
        "wall_s": time.perf_counter() - started,
        "process_cpu_s": usage.ru_utime + usage.ru_stime,
        "peak_rss_kib": peak,
        "cap_hit": cap_hit,
        "load_contamination": contamination,
        "stdout": {"path": str(stdout_path), "sha256": sha(stdout), "bytes": len(stdout)},
        "stderr": {"path": str(stderr_path), "sha256": sha(stderr), "bytes": len(stderr)},
    }


def report_summary(stdout_path, workload_sha, scalar, r, arm):
    try:
        report = read_json(stdout_path)
    except (OSError, ValueError):
        return {"parsed": False}
    rows = report.get("rows") or []
    rho = report.get("rho_reference") or {}
    per_run = rho.get("per_run") or []
    identity = (
        report.get("operation") == "bench"
        and report.get("prime_workload_sha256") == workload_sha
        and report.get("frozen_target_scalars") == [scalar]
        and len(rows) == 1
        and len(per_run) == 1
        and per_run[0].get("planted") == scalar
    )
    if not identity:
        return {"parsed": True, "identity_match": False,
                "status": report.get("status"), "rows": len(rows)}
    row = rows[0]
    fb = row.get("factor_base") or {}
    la = row.get("linear_algebra") or {}
    dec = row.get("decomposition") or {}
    native = ((dec.get("cost") or {}).get("native") or {})
    full_rank = (la.get("rank") == fb.get("columns", -1) + 1
                 and ((la.get("cost") or {}).get("native") or {}).get("full_rank_reached") == 1)
    rho_gae = rho.get("mean_s", 0) * math.sqrt(r)
    gae = row.get("total_gae")
    return {
        "parsed": True, "identity_match": True, "status": report.get("status"),
        "all_verified": report.get("all_verified"),
        "planted": scalar, "recovered": row.get("recovered"),
        "verified": row.get("verified"), "exhausted": row.get("exhausted"),
        "full_rank": full_rank, "rank": la.get("rank"), "columns": fb.get("columns"),
        "factor_base_abscissae": fb.get("abscissae"),
        "relation_targets_tried": dec.get("targets_tried"),
        "relations_found": dec.get("relations_found"),
        "solver_calls": (dec.get("solver") or {}).get("calls"),
        "solver_ops": (dec.get("solver") or {}).get("ops"),
        "deferred_to_grid": native.get("pkm_descent_deferred_to_grid", 0),
        "cost_qualification": row.get("cost_qualification"),
        "total_gae": gae, "rho_gae": rho_gae,
        "s_over_rho": gae / rho_gae if isinstance(gae, (int, float)) and rho_gae > 0 else None,
        "rho_all_verified": rho.get("all_verified"),
        "rho_recovered": per_run[0].get("recovered"),
        "pipeline_wall_s": row.get("wall_seconds"),
        "protocol_violation": bool(arm.startswith("candidate") and native.get("pkm_descent_deferred_to_grid", 0)),
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--checkout", type=Path, required=True)
    parser.add_argument("--bin", type=Path, required=True)
    parser.add_argument("--build-receipt", type=Path, required=True)
    parser.add_argument("--gate-store", type=Path, required=True)
    parser.add_argument("--roster", type=Path)
    parser.add_argument("--manifest", type=Path)
    parser.add_argument("--store", type=Path, required=True)
    parser.add_argument("--max-new", type=int, default=0,
                        help="maximum new arm invocations; 0 runs the whole frozen panel")
    args = parser.parse_args()
    if args.max_new < 0:
        parser.error("--max-new must be nonnegative")
    checkout = args.checkout.resolve(strict=True)
    args.roster = args.roster or checkout / "research/pkm_nonenum_decomp_20261125/roster.json"
    args.manifest = args.manifest or checkout / "research/pkm_nonenum_decomp_20261125/workloads/manifest.json"
    args.store.parent.mkdir(parents=True, exist_ok=True)
    lock = (args.store.parent / (args.store.name + ".lock")).open("a+")
    try:
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
    except BlockingIOError:
        parser.error("another runner holds this append-only store")
    source_commit = subprocess.check_output(["git", "-C", str(checkout), "rev-parse", "HEAD"],
                                            text=True).strip()
    if source_commit != SOURCE_COMMIT:
        parser.error("checkout differs from the frozen PKM panel source commit")
    dirty = subprocess.check_output(["git", "-C", str(checkout), "status", "--porcelain=v1",
                                     "--untracked-files=no"], text=True)
    if dirty:
        parser.error("pinned checkout has tracked changes")
    build = read_json(args.build_receipt)
    binary = args.bin.resolve(strict=True)
    binary_sha = sha(binary.read_bytes())
    if build.get("source_commit") != source_commit or build.get("binary_sha256") != binary_sha:
        parser.error("binary build receipt differs from pinned source or executable")
    gate = records(args.gate_store)
    if gate.get("header", {}).get("source_commit") != source_commit or any(
        gate.get(f"test:{test}", {}).get("outcome") != "pass" for test in TESTS
    ):
        parser.error("source-matched native witness gate has not passed")
    roster_bytes = args.roster.read_bytes()
    manifest_bytes = args.manifest.read_bytes()
    if sha(roster_bytes) != ROSTER_SHA256:
        parser.error("roster differs from frozen selection")
    roster = json.loads(roster_bytes)
    manifest = json.loads(manifest_bytes)
    if sha(manifest_bytes) != MANIFEST_SHA256 or manifest.get("roster_sha256") != ROSTER_SHA256 or len(manifest.get("entries", [])) != 9:
        parser.error("workload manifest differs from frozen roster")
    rows = {(row["t"], row["curve_index"]): row for row in roster["curves"]}
    expected_keys = {(t, curve_index) for t in (3, 4, 5) for curve_index in (0, 1, 2)}
    manifest_keys = {(entry["t"], entry["curve_index"]) for entry in manifest["entries"]}
    if set(rows) != expected_keys or manifest_keys != expected_keys:
        parser.error("roster or manifest lacks a frozen independent curve")
    header = {
        "id": "header", "schema": "pkm-f4-cold-single-target/v1",
        "source_commit": source_commit, "binary_sha256": binary_sha,
        "build_receipt_sha256": sha(args.build_receipt.read_bytes()),
        "gate_store_sha256": sha(args.gate_store.read_bytes()),
        "runner_sha256": sha(Path(__file__).read_bytes()),
        "roster_sha256": ROSTER_SHA256, "manifest_sha256": sha(manifest_bytes),
        "platform": platform.platform(), "arms": list(ARMS),
        "linalg": LINALG, "max_trials": MAX_TRIALS,
        "wall_cap_s": WALL_CAP_S, "rss_cap_kib": RSS_CAP_KIB,
        "order": "t,curve,target; candidate/exhaustive/mitm/candidate",
    }
    prior = records(args.store)
    if not prior:
        append(args.store, header)
        prior["header"] = header
    elif prior.get("header") != header:
        raise ValueError("existing store has different frozen inputs or runner")
    new = 0
    for entry in manifest["entries"]:
        key = (entry["t"], entry["curve_index"])
        row = rows[key]
        full_path = args.manifest.parent / entry["path"]
        full_bytes = full_path.read_bytes()
        if sha(full_bytes) != entry["sha256"]:
            raise ValueError(f"workload hash differs: {full_path}")
        full = json.loads(full_bytes)
        if len(full["targets"]) != 16 or entry["target_scalars"] != [
            target["scalar"] for target in full["targets"]
        ]:
            raise ValueError(f"target list differs: {full_path}")
        for target_index, target in enumerate(full["targets"]):
            frozen_target = row["targets"][target_index]
            if target["scalar"] != frozen_target["scalar"] or target["point"] != {
                "x": hex(frozen_target["point"][0]), "y": hex(frozen_target["point"][1])
            }:
                raise ValueError("roster and ic target differ")
            scalar = target["scalar"]
            single = dict(full)
            single["targets"] = [target]
            single["sampling"] += f"; target_index={target_index}"
            workload_bytes = (json.dumps(single, indent=2, sort_keys=True) + "\n").encode()
            stem = f"t{key[0]}_c{key[1]}_q{target_index}"
            input_path = args.store.parent / "inputs" / f"{stem}.json"
            preserve(input_path, workload_bytes)
            workload_sha = sha(workload_bytes)
            seed = int.from_bytes(bytes.fromhex(frozen_target["relation_seed_sha256"])[:8], "little")
            for arm, oracle in ARMS:
                case = f"{stem}:{arm}"
                if f"result:{case}" in prior:
                    continue
                if f"launch:{case}" in prior:
                    raise RuntimeError(f"unresolved launch {case}; inspect its process and logs")
                if args.max_new and new >= args.max_new:
                    return
                load = quiet()
                load_id = f"load:{case}:{1 + sum(k.startswith('load:'+case+':') for k in prior)}"
                append(args.store, {"id": load_id, **load})
                prior[load_id] = load
                if load["competing_processes"]:
                    raise RuntimeError(f"host busy before {case}; no target child started")
                command = [str(binary), "bench", "--prime-workload", str(input_path),
                           "--factor-base", f"prime-kummer:t={key[0]},g={entry['g']}",
                           "--oracle", oracle, "--targets", "walk", "--linalg", LINALG,
                           "--audit-rank", "--max-trials", str(MAX_TRIALS),
                           "--solver-budget-seconds", str(WALL_CAP_S), "--rho-runs", "1",
                           "--seed", str(seed), "--json"]
                stdout_path = args.store.parent / "raw" / f"{case.replace(':', '_')}.stdout"
                stderr_path = args.store.parent / "raw" / f"{case.replace(':', '_')}.stderr"
                stdout_path.parent.mkdir(parents=True, exist_ok=True)
                launch = {"id": f"launch:{case}", "load_check": load_id,
                          "command": command, "workload_sha256": workload_sha,
                          "target_scalar": scalar, "relation_seed": seed,
                          "stdout_path": str(stdout_path), "stderr_path": str(stderr_path)}
                append(args.store, launch)
                prior[launch["id"]] = launch
                result = run_one(command, stdout_path, stderr_path)
                summary = report_summary(stdout_path, workload_sha, scalar, row["r"], arm)
                outcome = "censored" if result["cap_hit"] or result["load_contamination"] else (
                    "parsed" if result["exit_code"] == 0 and summary.get("identity_match") else "failed"
                )
                receipt = {"id": f"result:{case}", "outcome": outcome,
                           "arm": arm, "t": key[0], "curve_index": key[1],
                           "target_index": target_index, **result, "summary": summary}
                append(args.store, receipt)
                prior[receipt["id"]] = receipt
                new += 1
                print(json.dumps({"case": case, "outcome": outcome,
                                  "verified": summary.get("verified"),
                                  "full_rank": summary.get("full_rank"),
                                  "wall_s": result["wall_s"]}), flush=True)
                if result["load_contamination"]:
                    raise RuntimeError(f"host contamination during {case}; preserve censored row")


if __name__ == "__main__":
    main()
