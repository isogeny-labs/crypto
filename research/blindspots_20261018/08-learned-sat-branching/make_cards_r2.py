#!/usr/bin/env python3
"""Writes the repair (-r2) cards of item 08, stage 1: cards 05-09b.

Stage 2 (card-10-r2, the summaries) is written by make_card_10_r2.py after the
stage-1 outputs are committed, because a card may read an earlier card's output
only when that file is committed and bound by sha256.
Run from the repository root: python3 research/blindspots_20261018/08-learned-sat-branching/make_cards_r2.py
"""
import hashlib, pathlib

ITEM = "research/blindspots_20261018/08-learned-sat-branching"
RAW = f"{ITEM}/raw"
SRC = "examples/blindspots_08_learned_branching.rs"
MODEL = f"{RAW}/model.txt"
CMS_REV = "4c9a6b6b459b7c1381744115ccfe744f1121794b"
sha = lambda p: hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
SRC_SHA, MODEL_SHA = sha(SRC), sha(MODEL)
assert MODEL_SHA == "00f0d29d6f0535b91ca3ab0bbc1c76dbb19085fce325c5b77aabed0479b60809"
PR = "prlimit --as=4294967296"

def isnum(x):
    try:
        float(x)
        return True
    except ValueError:
        return False

def q(s):
    return '"' + s + '"'

def argv(*a):
    return "[" + ", ".join(x if x.replace("-", "").replace("_", "").replace("/", "").replace(".", "").isalnum() and not isnum(x) and "=" not in x else q(x) for x in a) + "]"

def cmd(cid, a, timeout, stdout=None, stderr=None, outputs=None, retry=None):
    s = f"  - id: {cid}\n    argv: {argv(*a)}\n    timeout_seconds: {timeout}\n    success_exit_codes: [0]\n"
    if retry:
        s += f"    retry: {{exit_codes: [{retry}], max_attempts: 2}}\n"
    if stdout:
        s += f"    stdout: {RAW}/{stdout}\n"
    if stderr:
        s += f"    stderr: {RAW}/{stderr}\n"
    return s

def card(num, binds, commands, outputs_must_not_exist):
    cid = f"CARD-BLINDSPOT-08-{num}-r2"
    s = f"""schema: crypto.autoresearch.mechanical-task-card.v1
card_id: {cid}
experiment_id: BLINDSPOT-08
author_role: experiment-director
executor_role: executor-mechanical
kind: frozen_command

receipt: {RAW}/receipt-card-{num}-r2.json
must_not_exist:
""" + "".join(f"  - {RAW}/{p}\n" for p in outputs_must_not_exist) + "binds:\n"
    for p, h in binds:
        s += f"  - {{path: {p}, sha256: {h}}}\n"
    s += "commands:\n" + "".join(commands)
    s += f"""
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
    pathlib.Path(f"{ITEM}/card-{num}-r2.yaml").write_text(s)

def build_cmds(num):
    d = f"target/blindspots-08/card-{num}-r2"
    return d, [
        cmd("make-work-dir", ["mkdir", "-p", d], 5),
        cmd("build", PR.split() + ["rustc", "--edition", "2021", "-C", "opt-level=3", "-C", "codegen-units=1", "-C", "target-cpu=native", "-o", f"{d}/b08", SRC], 30,
            f"card-{num}-r2-build.stdout.log", f"card-{num}-r2-build.stderr.log"),
    ]

def cell(cid, d, n, ell, np_, nr, role, cap, timeout, out, extra=()):
    a = PR.split() + [f"{d}/b08", "cell", "--n", str(n), "--ell", str(ell), "--np", str(np_), "--nr", str(nr), "--seed", "20261018", "--role", str(role), "--models", MODEL]
    a += list(extra) + ["--cap", str(cap)]
    return cmd(cid, a, timeout, f"{out}.jsonl", f"{out}.stderr.log")

def cleanup(d):
    return cmd("remove-work-dir", ["rm", "-rf", d], 20)

BIND = [(SRC, SRC_SHA), (MODEL, MODEL_SHA)]

def l5_card(num, ns):
    d, c = build_cmds(num)
    outs = []
    for n in ns:
        out = f"test-n{n}-r2"
        outs += [f"{out}.jsonl", f"{out}.stderr.log"]
        c.append(cell(f"test-n{n}", d, n, 5, 16, 16, 7, 100000000, 200, out, ["--truth"]))
    c.append(cleanup(d))
    card(num, BIND, c, outs + [f"card-{num}-r2-build.stdout.log", f"card-{num}-r2-build.stderr.log"])

def l6_card(num, n):
    d, c = build_cmds(num)
    out = f"sec-l6-n{n}-r2"
    c.append(cell(f"sec-l6-n{n}", d, n, 6, 3, 3, 8, 150000000, 400, out, ["--truth"]))
    c.append(cleanup(d))
    card(num, BIND, c, [f"{out}.jsonl", f"{out}.stderr.log", f"card-{num}-r2-build.stdout.log", f"card-{num}-r2-build.stderr.log"])

def cms_card(num, only, out):
    d = f"target/blindspots-08/card-{num}-r2"
    c = [cmd("make-work-dir", ["mkdir", "-p", d], 5),
         cmd("clone-cms", ["git", "clone", "--quiet", "--depth", "1", "--branch", "5.11.21", "https://github.com/msoos/cryptominisat", f"{d}/src"], 25,
             f"card-{num}-r2-clone.stdout.log", f"card-{num}-r2-clone.stderr.log", retry="128"),
         cmd("pin-cms-revision", ["git", "-C", f"{d}/src", "checkout", "--quiet", "--detach", CMS_REV], 15),
         cmd("cms-revision", ["git", "-C", f"{d}/src", "rev-parse", "HEAD"], 5, f"card-{num}-r2-cms-revision.txt"),
         cmd("cms-configure", ["cmake", "-S", f"{d}/src", "-B", f"{d}/build", "-DCMAKE_BUILD_TYPE=Release", "-DENABLE_TESTING=OFF", "-DSTATS=OFF", "-DNOZLIB=ON", "-DNOM4RI=ON", "-DSTATICCOMPILE=ON"], 40,
             f"card-{num}-r2-cmake.stdout.log", f"card-{num}-r2-cmake.stderr.log"),
         cmd("cms-build", ["make", "-C", f"{d}/build", "-j2", "cryptominisat5-bin"], 120,
             f"card-{num}-r2-make.stdout.log", f"card-{num}-r2-make.stderr.log"),
         cmd("cms-version", [f"{d}/build/cryptominisat5", "--version"], 5, f"card-{num}-r2-cms-version.txt"),
         cmd("build", PR.split() + ["rustc", "--edition", "2021", "-C", "opt-level=3", "-C", "codegen-units=1", "-C", "target-cpu=native", "-o", f"{d}/b08", SRC], 30,
             f"card-{num}-r2-build.stdout.log", f"card-{num}-r2-build.stderr.log"),
         cell(f"cms-n31-{out}", d, 31, 5, 16, 16, 7, 100000000, 150, f"cms-n31-{out}-r2",
              ["--arms", "a", "--only", only, "--cms", f"{d}/build/cryptominisat5", "--cms-maxtime", "40", "--tmp", d]),
         cleanup(d)]
    names = [f"card-{num}-r2-{s}" for s in ("clone.stdout.log", "clone.stderr.log", "cms-revision.txt", "cmake.stdout.log", "cmake.stderr.log", "make.stdout.log", "make.stderr.log", "cms-version.txt", "build.stdout.log", "build.stderr.log")]
    card(num, BIND, c, names + [f"cms-n31-{out}-r2.jsonl", f"cms-n31-{out}-r2.stderr.log"])

l5_card("05", [31, 33])
l5_card("06", [35, 37])
l6_card("07", 31)
l6_card("08", 37)
cms_card("09", "0,1,2", "planted")
cms_card("09b", "16,17,18", "random")
