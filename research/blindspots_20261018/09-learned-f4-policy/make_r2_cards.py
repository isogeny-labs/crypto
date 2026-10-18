#!/usr/bin/env python3
"""Writes the item-09 repair ("r2") task cards.

The first run of item 09 (26 cards) stopped at card 13 with only cards 02, 09,
10 and 12 completed. Cards 01 (timeout too short), 05 to 08 (inputs the unrun
card 04 should have produced), 03, 04, 11 and 13 to 26 have no completed
receipt. This script writes one revision card per unfinished work unit,
`card-<id>-r2.yaml`, in three stages, because a card may read a file produced
by an earlier card only when that file is committed and its sha256 is bound,
and the file must exist when the consuming card is frozen:

  stage1  cards whose inputs exist now (first-run outputs of completed cards,
          the instrument, controls.tsv) or that are self-contained:
          01, 01b, 03, 03b, 04, 05, 11, 13, 24.
  stage2  after every stage-1 receipt is committed: the evaluation cards of
          cells (19,4,3), (21,5,3), (13,3,4). Each re-fits its cell's model
          from the bound sample files (the fit is deterministic and takes
          0.02 to 0.2 s: the cell-(17,4,3) refit reproduced the committed
          model byte for byte) and then evaluates, so no model file crosses a
          card boundary.
  stage3  after every stage-2 receipt is committed: the report card 26.

Usage:  python3 make_r2_cards.py stage1|stage2|stage3 [--in-dir DIR] [--out-dir DIR]
`--in-dir` / `--out-dir` exist only to test the stage-2 and stage-3 templates
against stand-in input files before the real ones exist.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[3]
ITEM = "research/blindspots_20261018/09-learned-f4-policy"
RAW = f"{ITEM}/raw"
INSTR = "examples/blindspots_09_learned_f4.rs"
CONTROLS = "research/blindspots_20261018/07-blocked-f4-degree-cells/controls.tsv"
NOTES = ("The repository root is /home/user/crypto. "
         "Run the command through the Bash tool with timeout 600000.")
PRLIMIT = ["prlimit", "--as=4294967296"]
SEED = "20261018"

ARMS_FULL = "a,b50,b25,c,d,p,r50,r25"
ARMS_21 = "a,b50,c,d,p,r50"
ARMS_P3 = "a,b50,b25,c,d,p,r50,r25,o"
CAPS = {"17": 6_000_000_000, "19": 10_000_000_000, "21": 30_000_000_000,
        "13": 3_000_000_000}
CELLS = {"17": (17, 4, 3), "19": (19, 4, 3), "21": (21, 5, 3), "13": (13, 3, 4)}


def sha(path: str) -> str:
    return hashlib.sha256((REPO / path).read_bytes()).hexdigest()


def q(s: str) -> str:
    return json.dumps(s)


class Card:
    def __init__(self, cid: str):
        self.cid = cid
        self.dir = f"target/blindspots-09/card-{cid}-r2"
        self.bin = f"{self.dir}/b09"
        self.binds: dict[str, str] = {}
        self.cmds: list[dict] = []
        self.bind(INSTR)

    def bind(self, path: str, in_dir: str | None = None) -> str:
        real = path
        self.binds[real] = sha(real)
        return real

    def add(self, cmd_id, argv, timeout, stdout=None, stderr=None, outputs=None):
        self.cmds.append(dict(id=cmd_id, argv=argv, timeout=timeout, stdout=stdout,
                              stderr=stderr, outputs=outputs))

    def start(self):
        self.add("make-bin-dir", ["mkdir", "-p", self.dir], 5)
        self.add("build", PRLIMIT + ["rustc", "--edition", "2021", "-C", "opt-level=3",
                                     "-C", "codegen-units=1", "-C", "target-cpu=native",
                                     "-o", self.bin, INSTR], 45,
                 f"{RAW}/card-{self.cid}-r2-build.stdout.log",
                 f"{RAW}/card-{self.cid}-r2-build.stderr.log")

    def finish(self):
        self.add("remove-bin", ["rm", "-f", self.bin], 5)
        self.add("remove-bin-dir", ["rmdir", self.dir], 5)

    def b09(self, cmd_id, args, timeout, stdout, stderr, outputs=None):
        self.add(cmd_id, PRLIMIT + [self.bin] + args, timeout, stdout, stderr, outputs)

    def total_timeout(self):
        return sum(c["timeout"] for c in self.cmds)

    def yaml(self) -> str:
        o = ["schema: crypto.autoresearch.mechanical-task-card.v1",
             f"card_id: CARD-BLINDSPOT-09-{self.cid}-r2",
             "experiment_id: BLINDSPOT-09",
             "author_role: experiment-director",
             "executor_role: executor-mechanical",
             "kind: frozen_command",
             "",
             f"receipt: {RAW}/receipt-card-{self.cid}-r2.json",
             "binds:"]
        for p, h in self.binds.items():
            o.append(f"  - {{path: {p}, sha256: {h}}}")
        o.append("commands:")
        for c in self.cmds:
            o.append(f"  - id: {c['id']}")
            o.append("    argv: [" + ", ".join(q(a) for a in c["argv"]) + "]")
            o.append(f"    timeout_seconds: {c['timeout']}")
            o.append("    success_exit_codes: [0]")
            if c["stdout"]:
                o.append(f"    stdout: {c['stdout']}")
            if c["stderr"]:
                o.append(f"    stderr: {c['stderr']}")
            if c["outputs"]:
                o.append("    outputs: [" + ", ".join(c["outputs"]) + "]")
        o += ["",
              "bus_address: executor-mechanical-blindspot-09",
              "background: false",
              "write_scope:",
              f"  - {RAW}/",
              "stop_when:",
              "  - The tool exits with status 2.",
              "  - The status line in the printed summary does not read status completed.",
              "  - Any command line in the printed summary ends with FAILED.",
              "  - Any output line in the printed summary ends with MISSING.",
              f"notes: {q(NOTES)}",
              ""]
        return "\n".join(o)


def cell_args(cell: str):
    n, l, m = CELLS[cell]
    return ["--n", str(n), "--ell", str(l), "--m", str(m), "--seed", SEED]


def train(c: Card, cell: str, lo: int, hi: int, prefix: str, timeout: int,
          cap: int | None = None, sfx: str = "r2"):
    n, l, m = CELLS[cell]
    tag = f"{n}-{l}-{m}-{lo:02d}-{hi:02d}"
    c.b09(f"train-{tag}",
          ["train"] + cell_args(cell) + ["--from", str(lo), "--to", str(hi), "--cap",
                                           str(cap or CAPS[cell]), "--out", f"{RAW}/{prefix}"],
          timeout, f"{RAW}/train-{tag}-{sfx}.jsonl", f"{RAW}/train-{tag}-{sfx}.stderr.log",
          [f"{RAW}/{prefix}.samples.{lo}", f"{RAW}/{prefix}.trace.{lo}"])


def fit(c: Card, cell: str, tag: str, prefix: str, samples: list[str], timeout=20):
    c.b09(f"fit-{tag}", ["fit"] + cell_args(cell) + ["--out", f"{RAW}/{prefix}", "--samples",
                                                    ",".join(samples)],
          timeout, f"{RAW}/fit-{tag}-r2.json", f"{RAW}/fit-{tag}-r2.stderr.log",
          [f"{RAW}/{prefix}.model"])


def evaluate(c: Card, cell: str, role: str, lo: int, hi: int, arms: str, model: str,
             traces: list[str], cap: int, rel_cap: str, tag: str, timeout: int):
    c.b09(f"eval-{tag}",
          ["eval"] + cell_args(cell) + ["--role", role, "--from", str(lo), "--to", str(hi),
                                        "--arms", arms, "--model", model, "--trace",
                                        ",".join(traces), "--cap", str(cap), "--rel-cap",
                                        rel_cap],
          timeout, f"{RAW}/eval-{tag}-r2.jsonl", f"{RAW}/eval-{tag}-r2.stderr.log")


# ------------------------------------------------------------------ stage 1

def stage1() -> list[Card]:
    cards = []
    # 01: selftest + P1 (the first run's selftest took 78 s against a 62 s timeout)
    c = Card("01"); c.bind(CONTROLS); c.start()
    c.b09("selftest", ["selftest"], 150, f"{RAW}/selftest-r2.txt",
          f"{RAW}/selftest-r2.stderr.log")
    c.b09("control-p1", ["control", "--table", CONTROLS, "--max-vars", "34", "--wall", "20",
                         "--cap", "3000000000"], 70, f"{RAW}/control-p1-r2.jsonl",
          f"{RAW}/control-p1-r2.stderr.log")
    c.finish(); cards.append(c)
    # 01b: P3 control cells, self-contained train -> fit -> eval
    c = Card("01b"); c.start()
    for l, m in ((3, 3), (2, 4)):
        tag = f"p3-8-{l}-{m}"
        pre = f"model-{tag}-r2"
        base = ["--n", "8", "--ell", str(l), "--m", str(m), "--seed", SEED]
        c.b09(f"train-{tag}", ["train"] + base + ["--from", "0", "--to", "32", "--cap",
                                                 "3000000000", "--out", f"{RAW}/{pre}"],
              30, f"{RAW}/train-{tag}-r2.jsonl", f"{RAW}/train-{tag}-r2.stderr.log",
              [f"{RAW}/{pre}.samples.0", f"{RAW}/{pre}.trace.0"])
        c.b09(f"fit-{tag}", ["fit"] + base + ["--out", f"{RAW}/{pre}", "--samples",
                                              f"{RAW}/{pre}.samples.0"], 20,
              f"{RAW}/fit-{tag}-r2.json", f"{RAW}/fit-{tag}-r2.stderr.log",
              [f"{RAW}/{pre}.model"])
        for role, hi, t1 in (("test", 32, 80 if (l, m) == (2, 4) else 60),
                             ("planted", 8, 60 if (l, m) == (2, 4) else 40)):
            c.b09(f"eval-{tag}-{role}",
                  ["eval"] + base + ["--role", role, "--from", "0", "--to", str(hi), "--arms",
                                     ARMS_P3, "--model", f"{RAW}/{pre}.model", "--trace",
                                     f"{RAW}/{pre}.trace.0", "--cap", "3000000000",
                                     "--rel-cap", "1000"],
                  t1, f"{RAW}/eval-{tag}-{role}-r2.jsonl",
                  f"{RAW}/eval-{tag}-{role}-r2.stderr.log")
    c.finish(); cards.append(c)
    # 03 / 03b: cell (17,4,3) evaluation against the completed card 02's model
    m17 = f"{RAW}/model-17-4-3.model"
    t17 = [f"{RAW}/model-17-4-3.trace.0", f"{RAW}/model-17-4-3.trace.16"]
    for cid, lo, hi in (("03", 0, 16), ("03b", 16, 32)):
        c = Card(cid)
        for p in [m17] + t17:
            c.bind(p)
        c.start()
        evaluate(c, "17", "test", lo, hi, ARMS_FULL, m17, t17, CAPS["17"], "1.5",
                 f"17-4-3-test-{lo:02d}-{hi:02d}", 270)
        c.finish(); cards.append(c)
    # 04: oracle arm at cell 17 + cell-19 training 0-16
    c = Card("04")
    for p in [m17] + t17:
        c.bind(p)
    c.start()
    evaluate(c, "17", "test", 0, 3, "o", m17, t17, CAPS["17"], "1.5",
             "17-4-3-test-00-03-o", 50)
    train(c, "19", 0, 8, "model-19-4-3-r2", 130)
    train(c, "19", 8, 16, "model-19-4-3-r2", 130)
    c.finish(); cards.append(c)
    # 05: cell-19 training 16-32
    c = Card("05"); c.start()
    train(c, "19", 16, 24, "model-19-4-3-r2", 130)
    train(c, "19", 24, 32, "model-19-4-3-r2", 130)
    c.finish(); cards.append(c)
    # 11 / 13: cell-21 training chunks missing from the first run (12-18, 24-32)
    c = Card("11"); c.start()
    train(c, "21", 12, 18, "model-21-5-3-r2", 280)
    c.finish(); cards.append(c)
    c = Card("13"); c.start()
    train(c, "21", 24, 30, "model-21-5-3-r2", 280)
    train(c, "21", 30, 32, "model-21-5-3-r2", 100)
    c.finish(); cards.append(c)
    # 24: cell-13 training
    c = Card("24"); c.start()
    train(c, "13", 0, 16, "model-13-3-4-r2", 200)
    train(c, "13", 16, 32, "model-13-3-4-r2", 200)
    c.finish(); cards.append(c)
    return cards


# ------------------------------------------------------------------ stage 2

def sample_trace_files(cell: str, in_dir: str):
    """(samples, traces) bound for a cell's refit and trace-transfer arm."""
    def r2(prefix, los):
        return ([f"{in_dir}/{prefix}.samples.{lo}" for lo in los],
                [f"{in_dir}/{prefix}.trace.{lo}" for lo in los])
    if cell == "19":
        return r2("model-19-4-3-r2", (0, 8, 16, 24))
    if cell == "13":
        return r2("model-13-3-4-r2", (0, 16))
    if cell == "21":
        first = ([f"{RAW}/model-21-5-3.samples.{lo}" for lo in (0, 6, 18)],
                 [f"{RAW}/model-21-5-3.trace.{lo}" for lo in (0, 6, 18)])
        new = r2("model-21-5-3-r2", (12, 24, 30))
        # fit and trace order: ascending train index
        order = (0, 6, 12, 18, 24, 30)
        s = {int(p.rsplit(".", 1)[1]): p for p in first[0] + new[0]}
        t = {int(p.rsplit(".", 1)[1]): p for p in first[1] + new[1]}
        return [s[k] for k in order], [t[k] for k in order]
    raise ValueError(cell)


def eval_card(cid: str, cell: str, lo: int, hi: int, timeout: int, in_dir: str) -> Card:
    n, l, m = CELLS[cell]
    cellname = f"{n}-{l}-{m}"
    samples, traces = sample_trace_files(cell, in_dir)
    c = Card(cid)
    for p in samples + traces:
        c.bind(p)
    c.start()
    pre = f"model-{cellname}-r2-c{cid}"
    fit(c, cell, f"{cellname}-c{cid}", pre, samples)
    arms = ARMS_21 if cell == "21" else ARMS_FULL
    evaluate(c, cell, "test", lo, hi, arms, f"{RAW}/{pre}.model", traces, CAPS[cell], "1.5",
             f"{cellname}-test-{lo:02d}-{hi:02d}", timeout)
    c.finish()
    return c


def stage2(in_dir: str) -> list[Card]:
    cards = []
    for cid, lo, hi in (("05b", 0, 8), ("06", 8, 16), ("07", 16, 24), ("08", 24, 32)):
        cards.append(eval_card(cid, "19", lo, hi, 380, in_dir))
    for cid, lo, hi in (("14", 0, 3), ("15", 3, 6), ("16", 6, 9), ("17", 9, 12),
                        ("18", 12, 15), ("19", 15, 18), ("20", 18, 21), ("21", 21, 24),
                        ("22", 24, 27), ("23", 27, 30)):
        cards.append(eval_card(cid, "21", lo, hi, 380, in_dir))
    cards.append(eval_card("23b", "21", 30, 32, 250, in_dir))
    # Cell (13,3,4) evaluation cards (25, 25b) are NOT written here: their bound
    # sample files do not exist until stage2b has run (see stage2b, then stage2c).
    return cards


# ------------------------------------------------------------------ stage 2b
# Cell (13,3,4) training, re-frozen with the cap raised from 3e9 to 6e10 touches
# (PROTOCOL.md "Repair pass r3"): the card-24-r2 run showed 13 of 13 incumbent
# runs at cap_touch 3.0e9 to 3.9e9; a scratch run of train target 0 at cap 4e10
# refuted it at 2.19e10 touches (20 s). New output names carry the suffix r2b.

CAP13_R2B = 60_000_000_000


def stage2b() -> list[Card]:
    cards = []
    for cid, lo, hi in (("24b", 0, 8), ("24c", 8, 16), ("24d", 16, 24), ("24e", 24, 32)):
        c = Card(cid); c.start()
        train(c, "13", lo, hi, "model-13-3-4-r2b", 380, cap=CAP13_R2B, sfx="r2b")
        c.finish(); cards.append(c)
    return cards


# ------------------------------------------------------------------ stage 3

def stage3(in_dir: str) -> list[Card]:
    def ev(cell, ranges):
        return [f"{in_dir}/eval-{cell}-test-{lo:02d}-{hi:02d}-r2.jsonl" for lo, hi in ranges]
    groups = {
        "17-4-3": ev("17-4-3", [(0, 16), (16, 32)]) +
                  [f"{in_dir}/eval-17-4-3-test-00-03-o-r2.jsonl"],
        "19-4-3": ev("19-4-3", [(0, 8), (8, 16), (16, 24), (24, 32)]),
        "21-5-3": ev("21-5-3", [(i, i + 3) for i in range(0, 30, 3)] + [(30, 32)]),
        "13-3-4": ev("13-3-4", [(0, 32), (32, 64)]),
        "p3": [f"{in_dir}/eval-p3-8-3-3-test-r2.jsonl", f"{in_dir}/eval-p3-8-3-3-planted-r2.jsonl",
               f"{in_dir}/eval-p3-8-2-4-test-r2.jsonl", f"{in_dir}/eval-p3-8-2-4-planted-r2.jsonl"],
    }
    c = Card("26")
    for files in groups.values():
        for p in files:
            c.bind(p)
    c.start()
    for name, files in groups.items():
        c.b09(f"report-{name}", ["report"] + files, 60, f"{RAW}/report-{name}-r2.txt",
              f"{RAW}/report-{name}-r2.stderr.log")
    c.finish()
    return [c]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("stage", choices=["stage1", "stage2", "stage2b", "stage3"])
    ap.add_argument("--in-dir", default=RAW)
    ap.add_argument("--out-dir", default=ITEM)
    a = ap.parse_args()
    cards = {"stage1": lambda: stage1(), "stage2": lambda: stage2(a.in_dir),
             "stage2b": lambda: stage2b(),
             "stage3": lambda: stage3(a.in_dir)}[a.stage]()
    out = REPO / a.out_dir
    out.mkdir(parents=True, exist_ok=True)
    for c in cards:
        path = out / f"card-{c.cid}-r2.yaml"
        path.write_text(c.yaml())
        print(f"{path.relative_to(REPO)}  timeouts sum {c.total_timeout()} s", file=sys.stderr)


if __name__ == "__main__":
    main()
