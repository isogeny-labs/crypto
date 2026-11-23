#!/usr/bin/env python3
"""Writes the item 04 revision cards (card-NN-r2.yaml).  Run from the repository root after the instrument is committed."""
import hashlib, pathlib

ITEM = "research/blindspots_20261018/04-presentation-search"
SRC = "examples/blindspots_04_presentation_search.rs"
SHA = hashlib.sha256(open(SRC, "rb").read()).hexdigest()
NOTES = "The repository root is /home/user/crypto. Run the command through the Bash tool with timeout 600000."
PL = ["prlimit", "--as=4294967296"]


def q(x):
    """YAML flow scalar: integers are quoted strings, words stay bare."""
    x = str(x)
    return '"%s"' % x if x.lstrip("-").isdigit() else x


def argv(items):
    return "[" + ", ".join(q(i) for i in items) + "]"


def cell(n, l, m, arm, extra):
    return ["cell", "--n", n, "--l", l, "--m", m, "--arms", arm] + extra


def card(name, runs):
    """runs: list of (id, cell-argv-after-binary, timeout, out-stem)."""
    dirp = f"target/blindspots-04/card-{name}-r2"
    raw = f"{ITEM}/raw"
    out = []
    out.append(f"""schema: crypto.autoresearch.mechanical-task-card.v1
card_id: CARD-BLINDSPOT-04-{name}-r2
experiment_id: BLINDSPOT-04
author_role: experiment-director
executor_role: executor-mechanical
kind: frozen_command

receipt: {raw}/receipt-card-{name}-r2.json
binds:
  - {{path: {SRC}, sha256: {SHA}}}
commands:
  - id: make-bin-dir
    argv: [mkdir, -p, {dirp}]
    timeout_seconds: 10
    success_exit_codes: [0]
  - id: build
    argv: {argv(PL + ["rustc", "--edition", "2021", "-C", "opt-level=3", "-C", "codegen-units=1", "-C", "target-cpu=native", "-o", dirp + "/b04", SRC])}
    timeout_seconds: 60
    success_exit_codes: [0]
    stdout: {raw}/card-{name}-r2-build.stdout.log
    stderr: {raw}/card-{name}-r2-build.stderr.log""")
    for cid_, av, to, stem in runs:
        out.append(f"""  - id: {cid_}
    argv: {argv(PL + [dirp + "/b04"] + av)}
    timeout_seconds: {to}
    success_exit_codes: [0]
    stdout: {raw}/card-{name}-r2-{stem}.jsonl
    stderr: {raw}/card-{name}-r2-{stem}.log""")
    out.append(f"""  - id: remove-bin
    argv: [rm, -f, {dirp}/b04]
    timeout_seconds: 10
    success_exit_codes: [0]
  - id: remove-bin-dir
    argv: [rmdir, {dirp}]
    timeout_seconds: 10
    success_exit_codes: [0]

bus_address: executor-mechanical-blindspot-04
background: false
write_scope:
  - {raw}/
stop_when:
  - The tool exits with status 2.
  - The status line in the printed summary does not read status completed.
  - Any command line in the printed summary ends with FAILED.
  - Any output line in the printed summary ends with MISSING.
notes: "{NOTES}"
""")
    pathlib.Path(f"{ITEM}/card-{name}-r2.yaml").write_text("\n".join(out))


C1, C2, C3, C4, C5 = (17, 4, 3), (19, 4, 3), (13, 3, 4), (15, 3, 4), (17, 4, 4)

# 02-r2: A/A control (arm a, draw 1) on all five cells, then the recodings.
runs = []
for i, c in enumerate([C1, C2, C3, C4], 1):
    runs.append((f"aa-cell{i}", cell(*c, "a", ["--draw", 1, "--random", 8, "--planted", 0, "--time-limit", 20, "--job-time", 15]), 60, f"aa-cell{i}"))
runs.append(("aa-cell5", cell(*C5, "a", ["--draw", 1, "--random", 8, "--planted", 0, "--dmax-u", 12, "--time-limit", 20, "--job-time", 10]), 60, "aa-cell5"))
for rc in ("linear", "nonlinear"):
    for i, c in enumerate([C1, C2, C3, C4], 1):
        runs.append((f"recode-{rc}-cell{i}", cell(*c, "a", ["--recode", rc, "--random", 4, "--planted", 2, "--time-limit", 20, "--job-time", 15]), 60, f"recode-{rc}-cell{i}"))
card("02", runs)

# 04-r2: arm e, cell 3 random 4-7 and cell 4 random 0-3 (parameters of card 04).
card("04", [
    ("e-cell3-second", cell(*C3, "e", ["--first", 4, "--random", 4, "--planted", 0, "--dmax-e", 5, "--time-limit", 110, "--job-time", 80]), 130, "e-cell3-second"),
    ("e-cell4-first", cell(*C4, "e", ["--random", 4, "--planted", 0, "--dmax-e", 5, "--time-limit", 280, "--job-time", 200]), 300, "e-cell4-first"),
])

# 06-r2: arm e, cell 5, one random target (parameters of card 06).
card("06", [
    ("e-cell5", cell(*C5, "e", ["--random", 1, "--planted", 0, "--dmax-e", 5, "--time-limit", 380, "--job-time", 360]), 450, "e-cell5"),
])

# 07-r2 / 07b-r2: arm b, cell 1; card 07 split in two so each card is far below 8 minutes.
card("07", [
    ("b-cell1-random-a", cell(*C1, "b", ["--dmax-b", 3, "--random", 4, "--planted", 0, "--time-limit", 220, "--job-time", 90]), 240, "b-cell1-random-a"),
    ("b-cell1-planted-a", cell(*C1, "b", ["--dmax-b", 3, "--random", 0, "--planted", 2, "--time-limit", 140, "--job-time", 80]), 160, "b-cell1-planted-a"),
])
card("07b", [
    ("b-cell1-random-b", cell(*C1, "b", ["--dmax-b", 3, "--first", 4, "--random", 4, "--planted", 0, "--time-limit", 220, "--job-time", 90]), 240, "b-cell1-random-b"),
])

# 12/16/19-r2: arm b, cell 4, one random target each (parameters of cards 12, 16, 19).
card("12", [
    ("b-cell4-t0", cell(*C4, "b", ["--dmax-b", 3, "--first", 0, "--random", 1, "--planted", 0, "--time-limit", 330, "--job-time", 320]), 380, "b-cell4-t0"),
    ("b-cell4-planted", cell(*C4, "b", ["--dmax-b", 2, "--random", 0, "--planted", 2, "--time-limit", 40, "--job-time", 30]), 50, "b-cell4-planted"),
])
for name, t in (("16", 4), ("19", 7)):
    card(name, [
        (f"b-cell4-t{t}", cell(*C4, "b", ["--dmax-b", 3, "--first", t, "--random", 1, "--planted", 0, "--time-limit", 330, "--job-time", 320]), 380, f"b-cell4-t{t}"),
    ])
print("sha", SHA)
