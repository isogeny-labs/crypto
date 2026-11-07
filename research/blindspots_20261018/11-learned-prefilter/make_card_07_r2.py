#!/usr/bin/env python3
"""Freeze card-07-r2.yaml (the repair-pass verdict card) from the committed raw files.

Run from the repository root, only after cards 03-r2, 03b-r2 and 05-r2 have run and their
outputs and receipts are committed. The verdict subcommand reads every *.jsonl in one directory,
so the card copies exactly the sixteen cell-population files (eleven receipted first-run files plus
the five -r2 files that replace the two unreceipted first-run files and fill the three missing
cells) into a throwaway directory under target/ and runs verdict there; the first-run
n19-m3-U.jsonl and n29-m3-U.jsonl are left out, so no cell is counted twice. Each source is bound
by sha256. Usage: make_card_07_r2.py [--out PATH]
"""
import hashlib, os, sys
D = "research/blindspots_20261018/11-learned-prefilter"
SHA = "e6f23662c89f4ed99dc9407817c61a8aa58412c276cccb023af46371c7bc0d66"
CELLS = [(n, m, p) for n in (17, 19, 23, 29) for m in (2, 3) for p in "US"]
R2 = {(19, 3, "U"), (19, 3, "S"), (23, 2, "U"), (29, 3, "U"), (29, 3, "S")}
srcs = []
for n, m, p in CELLS:
    suf = "-r2" if (n, m, p) in R2 else ""
    srcs.append(f"{D}/raw/n{n}-m{m}-{p}{suf}.jsonl")
for s in srcs:
    if not os.path.exists(s):
        sys.exit(f"missing {s}: freeze this card only after cards 03-r2, 03b-r2, 05-r2 have run and are committed")
    last = open(s).read().rstrip("\n").splitlines()[-1]
    if '"kind":"done"' not in last:
        sys.exit(f"{s} does not end with a done line")
out = sys.argv[sys.argv.index("--out") + 1] if "--out" in sys.argv else f"{D}/card-07-r2.yaml"
bd = "target/blindspots-11/card-07-r2"
inp = f"{bd}/in"
L = [
    "schema: crypto.autoresearch.mechanical-task-card.v1",
    "card_id: CARD-BLINDSPOT-11-07-r2",
    "experiment_id: BLINDSPOT-11", "author_role: experiment-director",
    "executor_role: executor-mechanical", "kind: frozen_command", "",
    f"receipt: {D}/raw/receipt-card-07-r2.json", "binds:",
    f"  - {{path: examples/blindspots_11_learned_prefilter.rs, sha256: {SHA}}}",
]
for s in srcs:
    L.append(f"  - {{path: {s}, sha256: {hashlib.sha256(open(s,'rb').read()).hexdigest()}}}")
L.append("commands:")
def cmd(i, argv, t, extra=""):
    L.append(f"  - id: {i}\n    argv: [{', '.join(argv)}]\n    timeout_seconds: {t}\n    success_exit_codes: [0]{extra}")
cmd("make-bin-dir", ["mkdir", "-p", inp], 10)
cmd("build", ["prlimit", "--as=4294967296", "rustc", "--edition", '"2021"', "-C", "opt-level=3", "-C", "codegen-units=1",
              "-C", "target-cpu=native", "-o", f"{bd}/b11", "examples/blindspots_11_learned_prefilter.rs"], 60,
    f"\n    stdout: {D}/raw/card-07-r2-build.stdout.log\n    stderr: {D}/raw/card-07-r2-build.stderr.log")
cmd("stage-inputs", ["cp"] + srcs + [inp], 30)
cmd("verdict", ["prlimit", "--as=4294967296", f"{bd}/b11", "verdict", "--dir", inp], 60,
    f"\n    stdout: {D}/raw/verdict-r2.txt\n    stderr: {D}/raw/verdict-r2.stderr.log")
cmd("remove-staged", ["rm", "-f"] + [f"{inp}/{os.path.basename(s)}" for s in srcs], 10)
cmd("remove-in-dir", ["rmdir", inp], 10)
cmd("remove-bin", ["rm", "-f", f"{bd}/b11"], 10)
cmd("remove-bin-dir", ["rmdir", bd], 10)
L += ["", "bus_address: executor-mechanical-blindspot-11", "background: false", "write_scope:", f"  - {D}/raw/",
      "stop_when:", "  - The tool exits with status 2.", "  - The status line in the printed summary does not read status completed.",
      "  - Any command line in the printed summary ends with FAILED.", "  - Any output line in the printed summary ends with MISSING.",
      'notes: "The repository root is /home/user/crypto. Run the command through the Bash tool with timeout 600000."']
open(out, "w").write("\n".join(L) + "\n")
print("wrote", out)
