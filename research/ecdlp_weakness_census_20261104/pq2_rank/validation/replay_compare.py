#!/usr/bin/env python3
"""Assemble the validator's replay certificate (Task B).

Compares every instance line of the validator's own re-executions of pq2_rank
(binary rebuilt from the committed source) with the producer's committed logs
and JSON, timings excluded, and folds in the independent closure checks
(closure_check.py) and an independent |F'| recount. Producer files are read from
the git object store at the validated commit (recompute.COMMIT); the validator's
own outputs are read from validation/.

  python3 -I validation/replay_compare.py
"""
from __future__ import annotations

import datetime as dt
import hashlib
import json
import platform
import subprocess
import sys
from pathlib import Path

import numpy as np

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import closure_check as cc  # noqa: E402
import recompute as rc      # noqa: E402

PKG = rc.PKG
REPO = rc.REPO
SP_BIN = Path("/tmp/claude-0/-home-user/0f5ee4e8-cecf-515b-8fba-6c8312e48a0f/scratchpad/target/release/pq2_rank")
NS = lambda lo, hi: ",".join(str(n) for n in range(lo, hi + 1))
BIN = "./target/release/pq2_rank"  # byte-identical to the validator's fresh build (sha256 below)
VALUE_KEYS = rc.VALUE_KEYS


def sha(p: Path) -> str:
    return hashlib.sha256(p.read_bytes()).hexdigest()


def own_log(p: Path):
    hdr, rows = [], []
    for line in p.read_text().splitlines():
        r = rc.parse_line(line)
        if r is not None:
            rows.append(r)
        elif line.strip():
            hdr.append(line)
    return hdr, rows


def producer_log(relpath: str):
    hdr, rows = [], []
    for line in rc.read_text(PKG / relpath).splitlines():
        r = rc.parse_line(line)
        if r is not None:
            rows.append(r)
        elif line.strip():
            hdr.append(line)
    return hdr, rows


def producer_json(relpath: str):
    return json.loads(rc.read_text(PKG / relpath))["rows"]


runs: list[dict] = []
problems: list[str] = []

main_hdr, main_rows = producer_log("results/run.log")
main_by = {(r["n"], r["inst"]): r for r in main_rows}


def compare_log_run(name: str, cmd: str, lo: int, hi: int, extra_env: str = ""):
    d = HERE / name
    log = d / "run.log"
    if not log.exists():
        problems.append(f"{name}: no run.log")
        return
    hdr, rows = own_log(log)
    expect_keys = [k for k in main_by if lo <= k[0] <= hi]
    got = {(r["n"], r["inst"]): r for r in rows}
    mism = []
    fields = 0
    for k in expect_keys:
        if k not in got:
            mism.append({"key": k, "missing": True})
            continue
        for f in VALUE_KEYS:
            fields += 1
            if got[k][f] != main_by[k][f]:
                mism.append({"key": k, "field": f, "replay": got[k][f], "producer": main_by[k][f]})
    extra = sorted(set(got) - set(expect_keys))
    seed_ok = bool(hdr) and hdr[0] == main_hdr[0]
    complete = any(h.startswith("done in") for h in hdr)
    entry = {
        "id": name, "command": (extra_env + " " if extra_env else "") + cmd, "cwd": rc.rel(PKG),
        "reference": f"results/run.log instance lines with {lo} <= n <= {hi}",
        "compared_lines": len(expect_keys), "compared_fields": fields, "mismatches": len(mism),
        "mismatch_detail": mism[:20], "extra_lines": len(extra), "seed_line_equal": seed_ok,
        "run_completed": complete, "timings_excluded": True,
        "outputs": {p.name: sha(p) for p in sorted(d.iterdir()) if p.is_file()},
    }
    t = d / "time.txt"
    if t.exists():
        entry["time"] = " ".join(t.read_text().split())
    runs.append(entry)
    if mism or extra or not seed_ok or not complete:
        problems.append(f"{name}: mismatches {len(mism)}, extra {len(extra)}, seed_ok {seed_ok}, complete {complete}")


compare_log_run("replay_n8_20", f"{BIN} {NS(8, 20)} 3 20 validation/replay_n8_20/pq2_rank 44 16", 8, 20)
compare_log_run("replay_n8_20_dump", f"{BIN} {NS(8, 20)} 3 20 validation/replay_n8_20_dump/pq2_rank 44 16", 8, 20,
                "PQ2_DUMP=validation/replay_n8_20_dump/inst")
compare_log_run("replay_n8_24_dump", f"{BIN} {NS(8, 24)} 3 20 validation/replay_n8_24_dump/pq2_rank 44 16", 8, 24,
                "PQ2_DUMP=validation/replay_n8_24_dump/inst")
compare_log_run("replay_n8_44", f"{BIN} {NS(8, 44)} 3 20 validation/replay_n8_44/pq2_rank 44 16", 8, 44)


# ---- closure checks (validator's independent implementation vs Rust) -------
def closure_entry(name: str, cmd: str, ref: str):
    p = HERE / name
    if not p.exists():
        problems.append(f"{name}: missing")
        return None
    d = json.loads(p.read_text())
    s = d["summary"]
    failed = [(r["n"], r["instance"], r["failed_checks"]) for r in d["results"] if r["failed_checks"]]
    mism = [(r["n"], r["instance"], r["value_mismatches"]) for r in d["results"] if r["value_mismatches"]]
    entry = {"id": name, "command": cmd, "reference": ref, "instances": s["instances"],
             "compared_values": s["compared_values"], "mismatches": s["value_mismatches"],
             "failed_invariant_checks": len(failed), "failed_detail": failed[:10], "mismatch_detail": mism[:10],
             "iter2_values": sorted({r["mine"]["iter2"] for r in d["results"]}),
             "output_sha256": sha(p)}
    runs.append(entry)
    if s["value_mismatches"] or failed:
        problems.append(f"{name}: {s['value_mismatches']} value mismatches, {len(failed)} failed checks")
    return d


rust15 = f"PQ2_DUMP=validation/dump_n15_17/inst {BIN} 15,16,17 1 0 validation/dump_n15_17/pq2_rank 17 17"
runs.append({"id": "dump_n15_17", "command": rust15, "cwd": rc.rel(PKG),
             "note": "the dispatch's dump command; its Rust values are compared in closure_dump_n15_17",
             "outputs": {p.name: sha(p) for p in sorted((HERE / "dump_n15_17").iterdir()) if p.is_file()}})
CC = "python3 -I validation/closure_check.py"
closure_entry("closure_dump_n15_17.json",
              f"{CC} --run-json validation/dump_n15_17/pq2_rank.json --dump-dir validation/dump_n15_17/inst "
              "--out validation/closure_dump_n15_17.json --iter4-max-n 17 --low4-max-n 0",
              "Rust values in validation/dump_n15_17/pq2_rank.json (same run as the dumps)")
closure_entry("closure_replay_n8_20.json",
              f"{CC} --run-json validation/replay_n8_20_dump/pq2_rank.json --dump-dir validation/replay_n8_20_dump/inst "
              "--out validation/closure_replay_n8_20.json --iter4-max-n 16 --low4-max-n 20",
              "Rust values of the replayed main sweep n = 8..20 (lines equal to results/run.log, see replay_n8_20_dump)")
closure_entry("closure_replay_n21_24.json",
              f"{CC} --run-json validation/replay_n8_24_dump/pq2_rank.json --dump-dir validation/replay_n8_24_dump/inst "
              "--out validation/closure_replay_n21_24.json --iter4-max-n 16 --low4-max-n 20 --only 21:0,...,24:2",
              "Rust values of the replayed main sweep n = 21..24 (lines equal to results/run.log)")
d1418 = closure_entry("closure_dump_n14_18.json",
                      f"{CC} --run-json validation/dump_n14_18/pq2_rank.json --dump-dir validation/dump_n14_18/inst "
                      "--out validation/closure_dump_n14_18.json --iter4-max-n 18 --low4-max-n 16",
                      "Rust values of the crosscheck instances, re-run with iter4 enabled "
                      f"(PQ2_DUMP=validation/dump_n14_18/inst {BIN} 14,15,16,17,18 1 16 validation/dump_n14_18/pq2_rank 18 18)")
closure_entry("closure_iter4_n21_24.json",
              f"{CC} --run-json validation/iter4_n21_24_dump/pq2_rank.json --dump-dir validation/iter4_n21_24_dump/inst "
              "--out validation/closure_iter4_n21_24.json --iter4-max-n 24 --low4-max-n 0 "
              "--expect-json results/iter4_n21_24/pq2_rank.json",
              "producer results/iter4_n21_24/pq2_rank.json (instances regenerated by "
              f"PQ2_DUMP=validation/iter4_n21_24_dump/inst {BIN} 21,22,23,24 3 0 validation/iter4_n21_24_dump/pq2_rank 24 0)")
closure_entry("closure_iter4_n17_20.json",
              f"{CC} --run-json validation/iter4_n17_20_replay/pq2_rank.json --dump-dir validation/iter4_n17_20_replay/inst "
              "--out validation/closure_iter4_n17_20.json --iter4-max-n 20 --low4-max-n 0 "
              "--expect-json results/iter4_n17_20/pq2_rank.json",
              "producer results/iter4_n17_20/pq2_rank.json (instances regenerated by the producer's own command, below)")


# ---- Rust replays of the degree-4 sweeps vs the producer's JSON -------------
PARAMS = ["n", "nprime", "N", "instance", "a", "b", "modulus", "basis", "x_R", "F_size"]
NONTIME = PARAMS + ["Z_size", "affdim", "k", "rho", "low2", "low3", "low4", "d_lin", "iter3", "iter3_rounds",
                    "iter4", "iter4_rounds", "d_iter"]


def json_vs_json(name: str, mine_path: Path, ref_rel: str, fields: list[str], cmd: str, allow_extra: bool = False):
    if not mine_path.exists():
        problems.append(f"{name}: missing {mine_path}")
        return
    mine = json.loads(mine_path.read_text())["rows"]
    ref = producer_json(ref_rel)
    by = {(r["n"], r["instance"]): r for r in mine}
    mism, n = [], 0
    for r in ref:
        m = by.get((r["n"], r["instance"]))
        if m is None:
            mism.append({"key": (r["n"], r["instance"]), "missing": True})
            continue
        for f in fields:
            n += 1
            if m[f] != r[f]:
                mism.append({"key": (r["n"], r["instance"]), "field": f, "replay": m[f], "producer": r[f]})
    runs.append({"id": name, "command": cmd, "reference": ref_rel, "compared_lines": len(ref),
                 "compared_fields": n, "mismatches": len(mism), "mismatch_detail": mism[:20],
                 "extra_lines": len(set(by) - {(r["n"], r["instance"]) for r in ref}), "timings_excluded": True,
                 "output_sha256": sha(mine_path)})
    extra = len(set(by) - {(r["n"], r["instance"]) for r in ref})
    if mism or (extra and not allow_extra):
        problems.append(f"{name}: {len(mism)} mismatches, {extra} extra lines")


json_vs_json("iter4_n17_20_replay", HERE / "iter4_n17_20_replay/pq2_rank.json", "results/iter4_n17_20/pq2_rank.json",
             NONTIME, f"PQ2_DUMP=validation/iter4_n17_20_replay/inst {BIN} 17,18,19,20 3 0 "
             "validation/iter4_n17_20_replay/pq2_rank 20 20   (producer's COMMAND.txt with the out prefix moved)")
json_vs_json("iter4_n21_24_dump", HERE / "iter4_n21_24_dump/pq2_rank.json", "results/iter4_n21_24/pq2_rank.json",
             [f for f in NONTIME if f not in ("iter4", "iter4_rounds", "d_iter")],
             f"PQ2_DUMP=validation/iter4_n21_24_dump/inst {BIN} 21,22,23,24 3 0 "
             "validation/iter4_n21_24_dump/pq2_rank 24 0   (iter4 off; iter4 itself is checked by closure_iter4_n21_24)")
json_vs_json("replay_n8_44_vs_rerun_json", HERE / "replay_n8_44/pq2_rank.json", "results/rerun_n8_40/pq2_rank.json",
             NONTIME, "the replay_n8_44 run (instance parameters and F_size against the producer's rerun JSON; "
             "its 12 rows at n = 41..44 have no counterpart there)", allow_extra=True)

# ---- crosscheck: dumps byte-identical, values vs the validator's closure ----
cc_files = ["n14_i0", "n15_i0", "n16_i0", "n17_i0", "n18_i0"]
same = sum(rc.read_text(PKG / f"crosscheck/{f}.json") == (HERE / f"dump_n14_18/inst/{f}.json").read_text() for f in cc_files)
d3 = {}
d4 = {}
import re as _re
for fname, tgt in (("closure_d3.txt", d3), ("closure_d4.txt", d4)):
    for line in rc.read_text(PKG / f"crosscheck/{fname}").splitlines():
        m = _re.match(r"^(n\d+_i\d+)\.json \{(.*)\}$", line.strip())
        if m:
            tgt[m[1]] = {k: (v == "True" if v in ("True", "False") else int(v))
                         for k, v in _re.findall(r"'(\w+)': (\w+)", m[2])}
mine = {f"n{r['n']}_i{r['instance']}": r for r in (d1418 or {"results": []})["results"]}
cmp_n, cmp_bad = 0, []
for nm, ref in list(d3.items()) + [(k + "#d4", v) for k, v in d4.items()]:
    base, deg = (nm.split("#")[0], 4) if "#" in nm else (nm, 3)
    r = mine.get(base)
    if r is None:
        cmp_bad.append((nm, "missing"))
        continue
    pairs = [("low", r["mine"][f"iter{deg}"]), ("rank", r["mine"][f"closure{deg}_dim"]),
             ("one_in_span", r["mine"][f"closure{deg}_contains_1"]), ("k", r["mine"]["k"])]
    for key, val in pairs:
        cmp_n += 1
        if ref[key] != val:
            cmp_bad.append((nm, key, ref[key], val))
rust_iter4 = {f"n{r['n']}_i{r['instance']}": r["rust"]["iter4"] for r in (d1418 or {"results": []})["results"]}
d4_vs_rust = sum(d4[nm]["low"] == rust_iter4.get(nm) for nm in d4)
runs.append({"id": "crosscheck_instances", "command": "regenerated by the dump_n14_18 run; compared with crosscheck/",
             "reference": "crosscheck/n1{4..8}_i0.json, closure_d3.txt, closure_d4.txt",
             "dumps_byte_identical": same, "compared_values": cmp_n, "mismatches": len(cmp_bad),
             "mismatch_detail": cmp_bad, "rust_iter4_on_crosscheck_instances": rust_iter4,
             "python_d4_equal_validator_rust_iter4": f"{d4_vs_rust} of {len(d4)}"})
if same != 5 or cmp_bad:
    problems.append(f"crosscheck: {same} identical dumps, {len(cmp_bad)} mismatches")


# ---- independent |F'| recount for n <= 32 ------------------------------------
def fsize_independent(row) -> int:
    n = row["n"]
    F = cc.Field(n, int(row["modulus"], 16))
    basis = [int(v, 16) for v in row["basis"]]
    a, b = int(row["a"], 16), int(row["b"], 16)
    V = np.zeros(1, dtype=np.uint64)
    for v in basis:
        V = np.concatenate([V, V ^ np.uint64(v)])
    x = V[V != 0]
    # x^(-2) = x^(2^n - 3)
    e = (1 << n) - 3
    r = np.ones_like(x)
    p = x.copy()
    while e:
        if e & 1:
            r = F.vmul(r, p)
        p = F.vmul(p, p)
        e >>= 1
    c = x ^ np.uint64(a) ^ F.vmul(r, b)
    t = c.copy()
    s = c.copy()
    for _ in range(n - 1):
        s = F.vmul(s, s)
        t ^= s
    assert set(np.unique(t).tolist()) <= {0, 1}
    # x = 0 has one point (0, sqrt b); x != 0 has two points iff Tr(x + a + b/x^2) = 0
    return int(1 + 2 * np.count_nonzero(t == 0))


rer = producer_json("results/rerun_n8_40/pq2_rank.json")
fs_n, fs_bad = 0, []
for row in rer:
    if row["n"] <= 32:
        fs_n += 1
        got = fsize_independent(row)
        if got != row["F_size"]:
            fs_bad.append((row["n"], row["instance"], row["F_size"], got))
runs.append({"id": "F_size_independent_recount", "command": "replay_compare.py fsize_independent (numpy GF(2^n), n <= 32)",
             "reference": "results/rerun_n8_40/pq2_rank.json F_size", "compared_values": fs_n,
             "mismatches": len(fs_bad), "mismatch_detail": fs_bad})
if fs_bad:
    problems.append(f"F_size: {len(fs_bad)} mismatches")

# ---- negative controls ---------------------------------------------------------
ctl = json.loads((HERE / "controls.json").read_text())["results"] if (HERE / "controls.json").exists() else []
runs.append({"id": "negative_controls", "command": "python3 -I validation/controls.py",
             "controls": len(ctl), "caught": sum(c["caught"] for c in ctl), "mismatches": sum(not c["caught"] for c in ctl)})
if not ctl or not all(c["caught"] for c in ctl):
    problems.append("negative controls not all caught")

# ---------------------------------------------------------------------------------
env = {
    "validated_commit": rc.COMMIT,
    "head_at_assembly": subprocess.run(["git", "-C", str(REPO), "rev-parse", "HEAD"], capture_output=True, text=True).stdout.strip(),
    "src_main_rs_sha256": sha(PKG / "src/main.rs"),
    "src_main_rs_equals_commit_blob": hashlib.sha256(subprocess.run(
        ["git", "-C", str(REPO), "show", f"{rc.COMMIT}:{rc.rel(PKG / 'src/main.rs')}"], capture_output=True).stdout
    ).hexdigest() == sha(PKG / "src/main.rs"),
    "binary_fresh_build": str(SP_BIN), "binary_sha256": sha(SP_BIN) if SP_BIN.exists() else None,
    "producer_binary_sha256": sha(PKG / "target/release/pq2_rank"),
    "build": "cargo build --release -j 1 --locked --target-dir <scratch>/target (pinned to one core)",
    "rustc": subprocess.run(["rustc", "--version"], capture_output=True, text=True).stdout.strip(),
    "cargo": subprocess.run(["cargo", "--version"], capture_output=True, text=True).stdout.strip(),
    "python": platform.python_version(), "numpy": np.__version__,
    "cpu": next((l.split(":", 1)[1].strip() for l in open("/proc/cpuinfo") if l.startswith("model name")), "?"),
    "kernel": platform.release(), "seed": "0x5eed20261104 (hard-coded in src/main.rs)",
    "cpu_limit": "every validator process pinned with taskset -c 3 (one core)",
}
if env["binary_sha256"] != env["producer_binary_sha256"]:
    problems.append("fresh build differs from the producer's binary")

ok = not problems
cert = {
    "schema": "pcc.replay-certificate/v1",
    "status": "verified" if ok else "failed",
    "scalars_verified": ok,
    "runs": runs,
    "validator": rc.VALIDATOR,
    "produced_at": dt.datetime.now(dt.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
    "environment": env,
    "problems": problems,
    "producer_files_read": [{"path": p, "sha256": h} for p, h in sorted(rc.READ.items())],
}
(HERE / "replay_certificate.json").write_text(json.dumps(cert, indent=1, default=list) + "\n")
for r in runs:
    print(r["id"], {k: r[k] for k in ("compared_lines", "compared_fields", "compared_values", "mismatches",
                                       "instances", "failed_invariant_checks", "dumps_byte_identical") if k in r})
print("problems:", problems)
print("status:", cert["status"])
sys.exit(0 if ok else 1)
