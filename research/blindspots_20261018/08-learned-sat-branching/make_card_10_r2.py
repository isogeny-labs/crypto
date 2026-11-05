#!/usr/bin/env python3
"""Writes card-10-r2.yaml (stage 2: the summaries) of item 08.

Run from the repository root AFTER the stage-1 cards (05-r2 .. 09b-r2) have run
and their raw/*-r2 outputs and receipts are committed:

    python3 research/blindspots_20261018/08-learned-sat-branching/make_card_10_r2.py

A card may read an earlier card's output only when that file is committed and
bound by sha256, so this card cannot be frozen before its inputs exist. The
script refuses an input that is not committed byte-for-byte at HEAD, and omits a
summary whose producing receipts do not all read `status: completed` (it prints
what it omitted). It chooses nothing else: files, flags and thresholds are the
frozen protocol's (the same as the first run's card-10).
"""
import hashlib, json, pathlib, subprocess, sys

ITEM = "research/blindspots_20261018/08-learned-sat-branching"
RAW = f"{ITEM}/raw"
SRC = "examples/blindspots_08_learned_branching.rs"
PR = ["prlimit", "--as=4294967296"]
allow_uncommitted = "--allow-uncommitted" in sys.argv
out_name = "card-10-r2.yaml"
if "--out" in sys.argv:
    out_name = sys.argv[sys.argv.index("--out") + 1]

GROUPS = [
    # id, input files, producing receipts, summarize flags, output stem
    ("summary-primary", [f"test-n{n}-r2.jsonl" for n in (31, 33, 35, 37)], ["05", "06"],
     ["--sizes", "31,33,35,37", "--incumbent", "a", "--cands", "b,b2,c", "--controls", "bs,b2s,d,a2", "--threshold", "1.3", "--min-pairs", "100", "--boots", "10000"], 120),
    ("summary-secondary", [f"sec-l6-n{n}-r2.jsonl" for n in (31, 37)], ["07", "08"],
     ["--sizes", "31,37", "--incumbent", "a", "--cands", "b,b2,c", "--controls", "bs,b2s,d,a2", "--threshold", "1.3", "--min-pairs", "12", "--boots", "10000"], 60),
    ("summary-cms", ["cms-n31-planted-r2.jsonl", "cms-n31-random-r2.jsonl"], ["09", "09b"],
     ["--sizes", "31", "--incumbent", "a", "--cands", "none", "--controls", "e", "--min-pairs", "6", "--boots", "10000"], 60),
]

def sha(p):
    return hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()

def committed(p):
    r = subprocess.run(["git", "rev-parse", f"HEAD:{p}"], capture_output=True, text=True)
    h = subprocess.run(["git", "hash-object", p], capture_output=True, text=True)
    return r.returncode == 0 and r.stdout.strip() == h.stdout.strip()

def isnum(x):
    try:
        float(x)
        return True
    except ValueError:
        return False

def q(s):
    return '"' + s + '"'

def flow(a):
    return "[" + ", ".join(x if x.replace("-", "").replace("/", "").replace(".", "").replace("_", "").isalnum() and not isnum(x) and "=" not in x else q(x) for x in a) + "]"

d = "target/blindspots-08/card-10-r2"
binds = [(SRC, sha(SRC))]
cmds = [f"  - id: make-work-dir\n    argv: {flow(['mkdir', '-p', d])}\n    timeout_seconds: 5\n    success_exit_codes: [0]\n",
        f"  - id: build\n    argv: {flow(PR + ['rustc', '--edition', '2021', '-C', 'opt-level=3', '-C', 'codegen-units=1', '-C', 'target-cpu=native', '-o', d + '/b08', SRC])}\n"
        f"    timeout_seconds: 30\n    success_exit_codes: [0]\n    stdout: {RAW}/card-10-r2-build.stdout.log\n    stderr: {RAW}/card-10-r2-build.stderr.log\n"]
outs = [f"{RAW}/card-10-r2-build.stdout.log", f"{RAW}/card-10-r2-build.stderr.log"]
included = 0
for gid, files, receipts, flags, timeout in GROUPS:
    bad = []
    for r in receipts:
        rp = pathlib.Path(f"{RAW}/receipt-card-{r}-r2.json")
        if not rp.is_file() or json.loads(rp.read_text()).get("status") != "completed":
            bad.append(f"receipt-card-{r}-r2.json not completed")
    for f in files:
        p = f"{RAW}/{f}"
        if not pathlib.Path(p).is_file():
            bad.append(f"{f} missing")
        elif not allow_uncommitted and not committed(p):
            bad.append(f"{f} not committed")
    if bad:
        print(f"OMITTED {gid}: " + "; ".join(bad))
        continue
    included += 1
    stem = gid + "-r2"
    binds += [(f"{RAW}/{f}", sha(f"{RAW}/{f}")) for f in files]
    a = PR + [d + "/b08", "summarize", "--files", ",".join(f"{RAW}/{f}" for f in files)] + flags
    cmds.append(f"  - id: {gid}\n    argv: {flow(a)}\n    timeout_seconds: {timeout}\n    success_exit_codes: [0]\n"
                f"    stdout: {RAW}/{stem}.txt\n    stderr: {RAW}/{stem}.stderr.log\n")
    outs += [f"{RAW}/{stem}.txt", f"{RAW}/{stem}.stderr.log"]
    print(f"INCLUDED {gid}: {len(files)} inputs bound")
if not included:
    sys.exit("no summary group has completed, committed inputs; card-10-r2 not written")
cmds.append(f"  - id: remove-work-dir\n    argv: {flow(['rm', '-rf', d])}\n    timeout_seconds: 20\n    success_exit_codes: [0]\n")

s = f"""schema: crypto.autoresearch.mechanical-task-card.v1
card_id: CARD-BLINDSPOT-08-10-r2
experiment_id: BLINDSPOT-08
author_role: experiment-director
executor_role: executor-mechanical
kind: frozen_command

receipt: {RAW}/receipt-card-10-r2.json
must_not_exist:
""" + "".join(f"  - {o}\n" for o in outs) + "binds:\n" + "".join(f"  - {{path: {p}, sha256: {h}}}\n" for p, h in binds) + "commands:\n" + "".join(cmds) + f"""
bus_address: executor-mechanical-blindspot-08
background: false
write_scope:
  - {RAW}/
stop_when:
  - The tool exits with status 2.
  - The status line in the printed summary does not read status completed.
  - Any command line in the printed summary ends with FAILED.
  - Any output line in the printed summary ends with MISSING.
notes: "The repository root is /home/user/crypto. Run the command through the Bash tool with timeout 600000."
"""
pathlib.Path(f"{ITEM}/{out_name}").write_text(s)
print(f"wrote {ITEM}/{out_name}")
