#!/usr/bin/env python3
"""Freeze the item 06 repair-pass cards (card-NN-r2.yaml) for the first-run cards that did not complete.

Run from the repository root: python3 research/blindspots_20261018/06-signature-gb-trace-replay/make_cards_r2.py
The instrument and draws digests are read from the files, so the cards bind whatever is committed.
Every card builds its own binary in its own directory, runs only that binary, and removes it.
Outputs go to raw/card-NN-r2-*.log (never a first-run path). Commands and per-job limits are
the first-run ones except where PROTOCOL.md "Amendment 2" says otherwise (total-limit, timeouts,
the split of the first-run cards 05, 08 and 12 so that each card stays under 8 minutes).
"""
import hashlib
import json
import os

D = "research/blindspots_20261018/06-signature-gb-trace-replay"
RAW = f"{D}/raw"
DRAWS = f"{D}/draws.tsv"
SRC = "examples/blindspots_06_signature_gb.rs"
NOTES = "The repository root is /home/user/crypto. Run the command through the Bash tool with timeout 600000."


def sha(p):
    return hashlib.sha256(open(p, "rb").read()).hexdigest()


def run_argv(bd, cell, targets, arms, extra):
    a = ["prlimit", "--as=4294967296", f"{bd}/b06", "run", "--draws", DRAWS, "--cell", cell,
         "--targets", targets, "--arms", arms]
    for k, v in extra:
        a += [k, v]
    return a


# (file stem, card id suffix, [(command id, cell, targets, arms, extra flags, timeout)])
SIG = lambda lim: [("--touch-ratio", "2"), ("--sig-time-limit", "25"), ("--time-limit", "90"), ("--total-limit", str(lim))]
ABL = [("--touch-ratio", "2"), ("--sig-time-limit", "25"), ("--time-limit", "90"), ("--total-limit", "200")]
REP = [("--trace-target", "16"), ("--time-limit", "90"), ("--total-limit", "300")]
CARDS = [
    ("03-r2", [("long-c17-4-3", "c17-4-3", "0", "f4,sig",
                [("--touch-ratio", "100"), ("--sig-time-limit", "100"), ("--time-limit", "90"), ("--total-limit", "120")], 170)]),
    ("04-r2", [("sig-c15-5-3-a", "c15-5-3", "0-7", "f4,sig", SIG(220), 260),
               ("f4rs-c15-5-3", "c15-5-3", "0-3", "f4rs", [("--time-limit", "90"), ("--total-limit", "60")], 100)]),
    ("05-r2", [("sig-c15-5-3-b", "c15-5-3", "8-15", "f4,sig", SIG(220), 260)]),
    ("05b-r2", [("ablation-c15-5-3", "c15-5-3", "0-1", "f4,sig4,sig0,sigc", ABL, 240)]),
    ("08-r2", [("sig-c13-3-4-b", "c13-3-4", "8-15", "f4,sig", SIG(200), 240)]),
    ("08b-r2", [("ablation-c13-3-4", "c13-3-4", "0-1", "f4,sig4,sig0,sigc", ABL, 240)]),
    ("12-r2", [("replay-c15-3-4-a", "c15-3-4", "0-3", "f4r,f4n", REP, 340)]),
    ("12b-r2", [("replay-c15-3-4-b", "c15-3-4", "4-11", "f4r,f4n", REP, 340)]),
    ("12c-r2", [("replay-c15-3-4-c", "c15-3-4", "12-15", "f4r,f4n", REP, 340)]),
]


def y(argv):
    return json.dumps(argv)


def card(stem, cmds):
    num = stem.upper().replace("-R2", "-r2")
    bd = f"target/blindspots-06/card-{stem}"
    pre = f"card-{stem}"
    L = [
        "schema: crypto.autoresearch.mechanical-task-card.v1",
        f"card_id: CARD-BLINDSPOT-06-{num}",
        "experiment_id: BLINDSPOT-06",
        "author_role: experiment-director",
        "executor_role: executor-mechanical",
        "kind: frozen_command",
        "",
        f"receipt: {RAW}/receipt-{pre}.json",
        "binds:",
        f"  - {{path: {SRC}, sha256: {sha(SRC)}}}",
        f"  - {{path: {DRAWS}, sha256: {sha(DRAWS)}}}",
        "commands:",
    ]

    def cmd(i, argv, t, out=None):
        L.append(f"  - id: {i}\n    argv: {y(argv)}\n    timeout_seconds: {t}\n    success_exit_codes: [0]")
        if out:
            L.append(f"    stdout: {RAW}/{out}.stdout.log\n    stderr: {RAW}/{out}.stderr.log")

    cmd("make-bin-dir", ["mkdir", "-p", bd], 5)
    cmd("build", ["prlimit", "--as=4294967296", "rustc", "--edition", "2021", "-C", "opt-level=3", "-C",
                  "target-cpu=native", "-o", f"{bd}/b06", SRC], 60, f"{pre}-build")
    for cid, cell, tg, arms, extra, t in cmds:
        cmd(cid, run_argv(bd, cell, tg, arms, extra), t, f"{pre}-{cid}")
    cmd("remove-bin", ["rm", "-f", f"{bd}/b06"], 5)
    cmd("remove-bin-dir", ["rmdir", bd], 5)
    L += ["", "bus_address: executor-mechanical-blindspot-06", "background: false", "write_scope:", f"  - {RAW}/",
          "stop_when:", "  - The tool exits with status 2.",
          "  - The status line in the printed summary does not read status completed.",
          "  - Any command line in the printed summary ends with FAILED.",
          "  - Any output line in the printed summary ends with MISSING.",
          f'notes: "{NOTES}"']
    total = 5 + 60 + sum(c[5] for c in cmds) + 10
    assert total <= 480, (stem, total)
    path = f"{D}/card-{stem}.yaml"
    open(path, "w").write("\n".join(L) + "\n")
    return path, total


if __name__ == "__main__":
    for stem, cmds in CARDS:
        p, total = card(stem, cmds)
        print(p, "sum of timeouts", total)
