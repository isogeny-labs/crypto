#!/usr/bin/env python3
"""Writes the item-10 revision (r2) task cards for one stage.

The first run of item 10 was stopped after card 04. Cards 01 and 04 have
completed receipts and are not repeated. Every other card is revised here as
`card-NN-r2.yaml`, with all outputs under `raw-r2/` so nothing from the first
run is overwritten.

The searches resume from their own archive, so a chain of 256 evaluations has
to be cut into pieces that each finish inside eight minutes. A piece reads the
archive written by the piece before it, which a card can do only when that file
is committed and bound by sha256 in `binds`. The cards are therefore written in
stages, each stage only after the files it binds exist and are committed:

  stage 1  first 128 evaluations of every chain, plus the end of the n=31
           incumbent-seeded chain, which continues from card 04's committed archive
  stage 2  evaluations 128..255 of every chain, binding stage 1's archives
  stage 3  finalists, held-out, ablation and conservation cards, binding stage 2's
  stage 4  summarize, binding stage 3's held-out and conservation files

    python3 make_r2_cards.py STAGE        # writes that stage's cards, then lints them

Commands never name a binary built by another card: every card compiles the
instrument itself and deletes the binary at the end.
"""
import hashlib
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[3]
R = "research/blindspots_20261018/10-kernel-program-search"
RAW2 = f"{R}/raw-r2"
S1 = f"{RAW2}/state-1"
ST = f"{RAW2}/state"
INSTR = "examples/blindspots_10_kernel_search.rs"
OLD_B31 = f"{R}/raw/state/n31-b.jsonl"
PRLIMIT = ["prlimit", "--as=4294967296"]
NOTES = "The repository root is /home/user/crypto. Run the command through the Bash tool with timeout 600000."
STOP = [
    "The tool exits with status 2.",
    "The status line in the printed summary does not read status completed.",
    "Any command line in the printed summary ends with FAILED.",
    "Any output line in the printed summary ends with MISSING.",
]


def sha(path):
    return hashlib.sha256((REPO / path).read_bytes()).hexdigest()


def q(s):
    return '"' + str(s) + '"'


def flow(argv):
    return "[" + ", ".join(q(a) for a in argv) + "]"


class Card:
    def __init__(self, nn, binds=()):
        self.nn = nn
        self.id = f"CARD-BLINDSPOT-10-{nn}-r2"
        self.binds = [INSTR] + list(binds)
        self.cmds = []
        self.dir = f"target/blindspots-10/card-{nn}-r2"
        self.bin = f"{self.dir}/b10"
        self.add("make-bin-dir", ["mkdir", "-p", self.dir], 5)
        self.add(
            "build",
            PRLIMIT + ["rustc", "--edition", "2021", "-C", "opt-level=3", "-C", "codegen-units=1",
                       "-C", "target-cpu=native", "-o", self.bin, INSTR],
            40, stdout=f"{RAW2}/card-{nn}-r2-build.stdout.log", stderr=f"{RAW2}/card-{nn}-r2-build.stderr.log")

    def add(self, cid, argv, timeout, stdout=None, stderr=None, outputs=None):
        self.cmds.append(dict(id=cid, argv=argv, timeout=timeout, stdout=stdout, stderr=stderr, outputs=outputs))

    def run(self, cid, args, timeout, log=None, outputs=None, stdout=None):
        self.add(cid, PRLIMIT + [self.bin] + args, timeout,
                 stdout=stdout or (f"{RAW2}/{log}.stdout.log" if log else None),
                 stderr=f"{RAW2}/{log}.stderr.log" if log else None, outputs=outputs)

    def text(self):
        self.add("remove-bin", ["rm", "-f", self.bin], 5)
        self.add("remove-bin-dir", ["rmdir", self.dir], 5)
        total = sum(c["timeout"] for c in self.cmds)
        assert total <= 475, (self.id, total)
        out = [
            "schema: crypto.autoresearch.mechanical-task-card.v1",
            f"card_id: {self.id}",
            "experiment_id: BLINDSPOT-10",
            "author_role: experiment-director",
            "executor_role: executor-mechanical",
            "kind: frozen_command",
            "",
            f"receipt: {RAW2}/receipt-card-{self.nn}-r2.json",
            "binds:",
        ]
        for b in self.binds:
            out.append(f"  - {{path: {b}, sha256: {sha(b)}}}")
        out.append("commands:")
        for c in self.cmds:
            out.append(f"  - id: {c['id']}")
            out.append(f"    argv: {flow(c['argv'])}")
            out.append(f"    timeout_seconds: {c['timeout']}")
            out.append("    success_exit_codes: [0]")
            if c["stdout"]:
                out.append(f"    stdout: {c['stdout']}")
            if c["stderr"]:
                out.append(f"    stderr: {c['stderr']}")
            if c["outputs"]:
                out.append("    outputs: [" + ", ".join(c["outputs"]) + "]")
        out += [
            "",
            "bus_address: executor-mechanical-blindspot-10",
            "background: false",
            "write_scope:",
            f"  - {RAW2}/",
            "stop_when:",
        ]
        out += [f"  - {s}" for s in STOP]
        out.append(f"notes: {q(NOTES)}")
        return "\n".join(out) + "\n"


def search(c, label, n, arm, state, max_new, timeout):
    c.run(f"search-{label}",
          ["search", "--n", str(n), "--ell", "9", "--arm", arm, "--state", state,
           "--budget", "256", "--max-new", str(max_new)],
          timeout, log=f"search-{label}", outputs=[f"{state}/n{n}-{arm}.jsonl"])


def first_half(nn, n, arm):
    c = Card(nn)
    search(c, f"n{n}-{arm}-a", n, arm, S1, 128, 400)
    return c


def second_half(nn, n, arm):
    f = f"n{n}-{arm}.jsonl"
    c = Card(nn, binds=[f"{S1}/{f}"])
    c.add("make-state-dir", ["mkdir", "-p", ST], 5)
    # `cp -t DIR SRC`: no argv token names the destination file, so a lint run after the
    # card has run (check-relay) does not see an unpinned repository file (card 05-r2 did).
    c.add("copy-first-half", ["cp", "-t", ST, f"{S1}/{f}"], 10)
    search(c, f"n{n}-{arm}-b", n, arm, ST, 128, 380)
    return c


STATE_FILES = [f"{ST}/n31-b.jsonl", f"{ST}/n31-c.jsonl", f"{ST}/n31-p.jsonl",
               f"{ST}/n37-b.jsonl", f"{ST}/n37-c.jsonl", f"{ST}/n41-b.jsonl", f"{ST}/n41-c.jsonl"]


def finalists(c, capture, ns=(31, 37, 41)):
    for n in ns:
        if capture:
            c.run(f"finalists-n{n}", ["finalists", "--n", str(n), "--state", ST], 10,
                  stdout=f"{RAW2}/finalists-n{n}.json", outputs=[f"{ST}/finalists-n{n}.json"])
        else:
            c.run(f"finalists-n{n}", ["finalists", "--n", str(n), "--state", ST], 10,
                  stdout=f"{RAW2}/card-{c.nn}-r2-finalists-n{n}.stdout.log")


def heldout(c, n, timeout):
    c.run(f"heldout-n{n}", ["heldout", "--n", str(n), "--ell", "9", "--state", ST,
                            "--out", f"{RAW2}/heldout-n{n}.jsonl"],
          timeout, stdout=f"{RAW2}/heldout-n{n}.log", outputs=[f"{RAW2}/heldout-n{n}.jsonl"])


ABLATE_TIMEOUT = {31: 300, 37: 300, 41: 400}  # full-size smoke alone: 143, 120, 216 s


def ablate(nn, n):
    # an ablation reads only its own cell's archives and finalists
    binds = [f"{ST}/n{n}-{arm}.jsonl" for arm in ("b", "c", "p") if f"{ST}/n{n}-{arm}.jsonl" in STATE_FILES]
    c = Card(nn, binds=binds)
    finalists(c, False, (n,))
    c.run(f"ablate-n{n}", ["ablate", "--n", str(n), "--ell", "9", "--state", ST,
                           "--out", f"{RAW2}/ablate-n{n}.jsonl"],
          ABLATE_TIMEOUT[n], stdout=f"{RAW2}/ablate-n{n}.log", outputs=[f"{RAW2}/ablate-n{n}.jsonl"])
    return c


def conserve(c, tag, args, timeout):
    c.run(f"conserve-{tag}", ["conserve"] + args + ["--out", f"{RAW2}/conserve-{tag}.jsonl", "--states", ST],
          timeout, stdout=f"{RAW2}/conserve-{tag}.log", outputs=[f"{RAW2}/conserve-{tag}.jsonl"])


def stage(k):
    cards = []
    if k == 1:
        cards.append(first_half("02", 31, "p"))
        c = Card("05", binds=[OLD_B31])
        c.add("make-state-dir", ["mkdir", "-p", ST], 5)
        c.add("copy-card-04-archive", ["cp", OLD_B31, f"{ST}/n31-b.jsonl"], 10)
        search(c, "n31-b-end", 31, "b", ST, 64, 300)
        cards.append(c)
        cards.append(first_half("06", 37, "b"))
        cards.append(first_half("07", 41, "b"))
        cards.append(first_half("08", 31, "c"))
        cards.append(first_half("10", 37, "c"))
        cards.append(first_half("12", 41, "c"))
    elif k == 2:
        cards += [second_half("03", 31, "p"), second_half("19", 37, "b"), second_half("20", 41, "b"),
                  second_half("09", 31, "c"), second_half("11", 37, "c"), second_half("13", 41, "c")]
    elif k == 3:
        c = Card("14", binds=STATE_FILES)
        finalists(c, True)
        heldout(c, 31, 190)
        heldout(c, 37, 190)
        cards.append(c)
        c = Card("15", binds=STATE_FILES)
        finalists(c, False)
        heldout(c, 41, 150)
        conserve(c, "17-6", ["--n", "17", "--ell", "6", "--targets", "4000", "--families", "1"], 200)
        cards.append(c)
        cards += [ablate("16", 31), ablate("17", 37), ablate("18", 41)]
        c = Card("21", binds=STATE_FILES)
        finalists(c, False)
        conserve(c, "19-7", ["--n", "19", "--ell", "7", "--targets", "3000", "--families", "1", "--skip-other", "1"], 300)
        cards.append(c)
    elif k == 4:
        bind = [f"{RAW2}/heldout-n{n}.jsonl" for n in (31, 37, 41)] + [f"{RAW2}/conserve-17-6.jsonl"] + \
               [f"{ST}/finalists-n{n}.json" for n in (31, 37, 41)]
        c = Card("22", binds=bind)
        c.run("summarize", ["summarize", "--dir", RAW2, "--ns", "31,37,41", "--resamples", "2000",
                            "--out", f"{RAW2}/summary.json", "--state", ST],
              60, stdout=f"{RAW2}/summary.txt", outputs=[f"{RAW2}/summary.json"])
        cards.append(c)
    return cards


if __name__ == "__main__":
    k = int(sys.argv[1])
    paths = []
    for c in stage(k):
        p = REPO / R / f"card-{c.nn}-r2.yaml"
        p.write_text(c.text())
        paths.append(p)
        print("wrote", p.relative_to(REPO))
    for p in paths:
        r = subprocess.run([sys.executable, "tools/task_card.py", "lint", str(p.relative_to(REPO))],
                           cwd=REPO, capture_output=True, text=True)
        print(p.name, (r.stdout + r.stderr).strip())
