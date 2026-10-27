#!/usr/bin/env python3
"""Independent recomputation of research/postgb_linear_stage_20261026.

Written by the validator from PROTOCOL.md and its two amendments, without
reading scripts/summary.py.  Every input is read from the committed snapshot
with `git show <commit>:<path>`, so an uncommitted working tree cannot leak in.

    python3 -I recompute.py --repo /home/user/crypto --commit <snapshot> --out validator-report.json

Checks (all from raw data):
  * receipts: card sha256, exit statuses, binds = pins, output/stdout/stderr
    hashes, commit ancestry, dirty_before, host (lscpu);
  * run JSON: meta vs card argv/env, skipped lists, rounds, medians from raw
    `ms`, dense arm present iff delta <= 1024, coverage of the protocol corpus
    (every system exactly once, none dropped);
  * gates: agreement, planted points, constructed Nagao systems have a point,
    fast give-ups, sparse fallbacks, ablation agreement;
  * counted-mul determinism: a system at the same index `si` in two runs has
    identical sparse and fast `muls` (the arm seed is 0x5EED ^ si);
  * metrics: per-family geometric means (sparse/fast, dense/fast), min, max,
    counted ratios, own bootstrap CI, Spearman rho (P2), give-up fraction (P3),
    ablation geometric means (P5), portable/SIMD fast, host-A ratios, DLP S,
    S/rho, lin_ms and ratios (P4) rebuilt from their counters;
  * every numeric leaf of analysis/summary.json is matched against the
    recomputation; a leaf that cannot be matched is reported.
"""
import argparse
import hashlib
import json
import math
import random
import re
import statistics
import subprocess
from pathlib import Path

STUDY = "research/postgb_linear_stage_20261026"
PIN = {"./target/release/examples/postgb_la_bench": "c5612e2eb33393ca4d446b482685179a8418ff9d52682dd70c53d62f0e5ba1d3",
       "./target/release/examples/jv_cover": "afa830fc99087f12978f1e072c16b3f6e164c99b2b15e17fd6a3a7cc92bb050f"}
BENCH = ["R2-L1", "R2-L2", "R2-L3", "R2-L4", "R2-L5", "R2-L6"]
ABL = ["R2-A1", "R2-A2"]
DLP = ["R2-D1", "R2-D2"]
HOST_A = ["L1", "L2", "L3", "L4"]
FAMILIES = ["jv-nagao", "gaudry-s4", "gaudry-d4", "gaudry-s5", "dense"]
SHAPES = [(2, 8), (2, 16), (2, 32), (3, 4), (3, 6), (3, 8), (3, 10), (4, 3), (4, 4), (4, 5),
          (5, 2), (5, 3), (5, 4), (6, 2), (6, 3), (7, 2), (8, 2)]
DENSE_P = [65521, 2147483647, 4294967291]
TOL = 1e-9


class Snap:
    def __init__(self, repo: Path, commit: str):
        self.repo, self.commit = repo, commit

    def raw(self, rel: str) -> bytes:
        return subprocess.run(["git", "show", f"{self.commit}:{rel}"], cwd=self.repo,
                              capture_output=True, check=True).stdout

    def json(self, rel: str):
        return json.loads(self.raw(rel))

    def text(self, rel: str) -> str:
        return self.raw(rel).decode()

    def sha(self, rel: str) -> str:
        return hashlib.sha256(self.raw(rel)).hexdigest()

    def git(self, *args) -> subprocess.CompletedProcess:
        return subprocess.run(["git", *args], cwd=self.repo, capture_output=True, text=True)


def card_yaml_fields(text: str) -> dict:
    """The few card fields needed, parsed without a YAML dependency."""
    binds = re.findall(r"- path: (\S+)\n  sha256: ([0-9a-f]{64})", text)
    run = text[text.index("- id: run\n"):]
    run = run[:run.index("bus_address:")]
    argv = re.findall(r"^  - '?([^'\n]*)'?$", run[run.index("argv:"):run.index("timeout_seconds")], re.M)
    env = dict(re.findall(r"^    (\w+): (\S+)$", run[run.index("env:"):], re.M)) if "  env:" in run else {}
    timeout = int(re.search(r"timeout_seconds: (\d+)", run).group(1))
    return {"binds": dict(binds), "argv": argv, "env": env, "timeout": timeout}


def argv_opts(argv: list) -> dict:
    out, i = {}, 1
    while i < len(argv):
        out[argv[i]] = argv[i + 1]
        i += 2
    return out


def gmean(xs):
    return math.exp(sum(math.log(x) for x in xs) / len(xs))


def upper_median(xs):
    v = sorted(xs)
    return v[len(v) // 2]


def avg_ranks(xs):
    order = sorted(range(len(xs)), key=lambda i: xs[i])
    ranks = [0.0] * len(xs)
    i = 0
    while i < len(order):
        j = i
        while j + 1 < len(order) and xs[order[j + 1]] == xs[order[i]]:
            j += 1
        for k in range(i, j + 1):
            ranks[order[k]] = (i + j) / 2 + 1
        i = j + 1
    return ranks


def pearson(a, b):
    ma, mb = statistics.fmean(a), statistics.fmean(b)
    num = sum((x - ma) * (y - mb) for x, y in zip(a, b))
    return num / math.sqrt(sum((x - ma) ** 2 for x in a) * sum((y - mb) ** 2 for y in b))


def spearman(a, b):
    return pearson(avg_ranks(a), avg_ranks(b))


def bootstrap_ci(ratios, n=4000, seed=20261027):
    """Own percentile bootstrap of the geometric mean over systems (stdlib RNG;
    the producer's RNG is not reproduced, so this is a stochastic check)."""
    rng = random.Random(seed)
    logs = [math.log(r) for r in ratios]
    k = len(logs)
    means = sorted(math.exp(sum(rng.choice(logs) for _ in range(k)) / k) for _ in range(n))
    return [means[int(0.025 * n)], means[int(0.975 * n) - 1]]


def arm(row, name):
    for a in row["arms"]:
        if a["arm"] == name:
            return a
    return None


def expected_corpus():
    exp = set()
    for p in [53, 101, 251, 1009]:
        for k in range(12):
            exp.add(("jv-nagao", f"p{p}-{k}{'c' if k >= 8 else ''}"))
    for p in [271, 1039, 8191]:
        for k in range(8):
            exp.add(("gaudry-s4", f"p{p}-{k}"))
    for p in [97, 1033, 4093]:
        for k in range(8):
            exp.add(("gaudry-d4", f"p{p}-{k}"))
    for p in [13, 101]:
        for k in range(2):
            exp.add(("gaudry-s5", f"p{p}-{k}"))
    for n, d in SHAPES:
        for p in DENSE_P:
            for k in range(3):
                exp.add(("dense", f"n{n}-d{d}-p{p.bit_length()}-{k}"))
    return exp


class Report:
    def __init__(self):
        self.checks, self.problems = [], []

    def check(self, name, ok, detail=None):
        self.checks.append({"check": name, "ok": bool(ok), **({"detail": detail} if detail is not None else {})})
        if not ok:
            self.problems.append({"check": name, "detail": detail})
        return ok


def rel(a, b):
    if a == b:
        return 0.0
    return abs(a - b) / max(abs(a), abs(b))


# ---------------------------------------------------------------------------

def receipts(snap: Snap, rep: Report, cards: list) -> dict:
    hosts = {}
    for c in cards:
        cid = f"CARD-postgb-la-20261026-{c}"
        base = f"{STUDY}/cards/{cid}"
        card_text = snap.text(f"{base}.yaml")
        card = card_yaml_fields(card_text)
        r = snap.json(f"{base}.receipt.json")
        rep.check(f"{c}: receipt card_sha256 = sha256(card)", r["card_sha256"] == snap.sha(f"{base}.yaml"))
        rep.check(f"{c}: receipt status completed", r["status"] == "completed", r["status"])
        rep.check(f"{c}: every command ran, exit 0, succeeded, not timed out",
                  r["commands_total"] == len(r["commands"]) and all(
                      x["exit_code"] == 0 and x["succeeded"] and not x["timed_out"] and x["attempts"] == 1
                      for x in r["commands"]),
                  [(x["id"], x["exit_code"]) for x in r["commands"]])
        for path, digest in PIN.items():
            rep.check(f"{c}: card binds {path.rsplit('/', 1)[1]} at the pin", card["binds"].get(path) == digest,
                      card["binds"].get(path))
        rep.check(f"{c}: card binds PROTOCOL.md at its committed sha256",
                  card["binds"].get(f"{STUDY}/PROTOCOL.md") == snap.sha(f"{STUDY}/PROTOCOL.md"))
        run = [x for x in r["commands"] if x["id"] == "run"][0]
        rep.check(f"{c}: receipt run argv = card run argv", run["argv"] == card["argv"], run["argv"])
        rep.check(f"{c}: run argv[0] is a pinned binary", run["argv"][0] in PIN, run["argv"][0])
        out_rel = f"{STUDY}/runs/{c}.json"
        rep.check(f"{c}: receipt output sha256 = committed runs/{c}.json",
                  run["outputs"].get(out_rel) == snap.sha(out_rel), run["outputs"])
        for stream in ("stdout", "stderr"):
            rep.check(f"{c}: run {stream} sha256 = committed file",
                      run[f"{stream}_sha256"] == snap.sha(f"{base}.run.{stream}"))
        lscpu = [x for x in r["commands"] if x["id"] == "lscpu-record"][0]
        rep.check(f"{c}: lscpu sha256 = committed file", lscpu["stdout_sha256"] == snap.sha(f"{base}.lscpu.stdout"))
        text = snap.text(f"{base}.lscpu.stdout")
        model = re.search(r"Model name:\s+(.*)", text).group(1).strip()
        l2 = re.search(r"L2 cache:\s+(.*)", text).group(1).strip()
        hosts[c] = f"{model} | L2 {l2}"
        anc = snap.git("merge-base", "--is-ancestor", r["commit"], snap.commit).returncode == 0
        rep.check(f"{c}: receipt commit is an ancestor of the snapshot", anc, r["commit"])
        rep.check(f"{c}: dirty_before only under cards/",
                  all(p.startswith(f"{STUDY}/cards/") for p in r["dirty_before"]), r["dirty_before"])
        rep.check(f"{c}: dirty_after only the card's own outputs",
                  all(p.startswith(f"{STUDY}/cards/{cid}") or p == out_rel for p in r["dirty_after"]),
                  r["dirty_after"])
        try:
            log = snap.text(f"{base}.launcher.log")
            m = re.search(r"receipt: \S+ sha256 ([0-9a-f]{64})", log)
            rep.check(f"{c}: launcher log names the committed receipt's sha256",
                      m and m.group(1) == snap.sha(f"{base}.receipt.json"))
        except subprocess.CalledProcessError:
            pass
        ids = [x["id"] for x in r["commands"]]
        rep.check(f"{c}: card ran git-clean, disk/lscpu records and the quiet-CPU wait before the run",
                  ids == ["git-clean", "output-absent", "mkdir-runs", "disk-record", "lscpu-record",
                          "disk-before-run", "quiet-before-run", "run"], ids)
        if c.startswith("R2-"):
            want_timeout = 1700 if c in ("R2-L1", "R2-L2") else 7000
            rep.check(f"{c}: run timeout {want_timeout} s (amendment 2)", card["timeout"] == want_timeout,
                      card["timeout"])
        added = snap.git("log", "--diff-filter=A", "--format=%H", snap.commit, "--",
                         f"{base}.receipt.json").stdout.split()
        card["receipt_commit"] = r["commit"]
        card["receipt_added_in"] = added[-1] if added else None
        card["run_seconds"] = run["seconds"]
        hosts[c + ":card"] = card
    # one card at a time: each card started at the commit that added the previous card's receipt
    for prev, nxt in zip(cards, cards[1:]):
        a_, b_ = hosts[prev + ":card"], hosts[nxt + ":card"]
        rep.check(f"{nxt} started after {prev}'s receipt was committed (no overlap)",
                  b_["receipt_commit"] == a_["receipt_added_in"],
                  {"prev_receipt_added_in": a_["receipt_added_in"], "next_started_at": b_["receipt_commit"]})
    return hosts


def bench_rows(snap: Snap, rep: Report, cards: list, hosts: dict, reps_expected: int, simd: str, skipped_field=True):
    rows = []
    for c in cards:
        doc = snap.json(f"{STUDY}/runs/{c}.json")
        card = hosts[c + ":card"]
        opts = argv_opts(card["argv"])
        rep.check(f"{c}: meta matches card argv/env",
                  doc["families"] == opts["--families"] and doc["per"] == int(opts["--per"])
                  and doc["reps"] == int(opts["--reps"]) == reps_expected
                  and doc["arms"] == opts.get("--arms", "dense,sparse,fast")
                  and doc["shapes"] == opts.get("--shapes")
                  and doc["primes"] == ([int(x) for x in opts["--p"].split(",")] if "--p" in opts else None)
                  and doc["simd"] == simd and doc["fast_threads"] == 1
                  and card["env"] == ({"ZERO_DIM_LA_SIMD": "portable"} if simd == "portable" else {}),
                  {k: doc.get(k) for k in ("families", "per", "reps", "arms", "shapes", "primes", "simd",
                                           "fast_threads")} | {"env": card["env"]})
        if skipped_field:
            rep.check(f"{c}: skipped list is empty", doc.get("skipped") == [], doc.get("skipped"))
        want_arms = doc["arms"].split(",")
        for si, row in enumerate(doc["rows"]):
            names = [a["arm"] for a in row["arms"]]
            exp = [a for a in want_arms if not (a == "dense" and row["delta"] > 1024)]
            if names != exp:
                rep.check(f"{c}:{row['label']}: arms present", False, names)
            for a in row["arms"]:
                if len(a["ms"]) != reps_expected or a["median_ms"] != upper_median(a["ms"]):
                    rep.check(f"{c}:{row['label']}:{a['arm']}: rounds and median from raw ms", False,
                              [len(a["ms"]), a["median_ms"], upper_median(a["ms"])])
            rows.append({"card": c, "si": si, **row})
    return rows


def family_block(rows, num, den, fams):
    out = {}
    for fam in fams:
        rs = [r for r in rows if r["family"] == fam and arm(r, num) and arm(r, den)]
        if not rs:
            continue
        ratios = [arm(r, num)["median_ms"] / arm(r, den)["median_ms"] for r in rs]
        # counted ratio in the block's own direction (num.muls / den.muls)
        counted = [arm(r, num)["muls"] / arm(r, den)["muls"] for r in rs]
        out[fam] = {
            "systems": len(rs),
            "delta": [min(r["delta"] for r in rs), max(r["delta"] for r in rs)],
            "n": sorted({r["n"] for r in rs}),
            "p": sorted({r["p"] for r in rs}),
            "geomean": gmean(ratios),
            "ci95": bootstrap_ci(ratios),
            "min": min(ratios),
            "max": max(ratios),
            "median_ms": {num: upper_median([arm(r, num)["median_ms"] for r in rs]),
                          den: upper_median([arm(r, den)["median_ms"] for r in rs])},
            "median_ms_mean_of_middle": {num: statistics.median([arm(r, num)["median_ms"] for r in rs]),
                                         den: statistics.median([arm(r, den)["median_ms"] for r in rs])},
            "counted_ratio": {"min": min(counted), "max": max(counted), "geomean": gmean(counted)},
            "_ratios": ratios,
        }
    return out


def dlp_block(snap: Snap, rep: Report):
    d1 = {(r["p"], r["seed"]): r for r in snap.json(f"{STUDY}/runs/R2-D1.json")}
    d2 = {(r["p"], r["seed"]): r for r in snap.json(f"{STUDY}/runs/R2-D2.json")}
    rep.check("DLP: D1 and D2 cover the same (p, seed) = {53,101} x {1,2}",
              set(d1) == set(d2) == {(53, 1), (53, 2), (101, 1), (101, 2)}, sorted(d1))
    out = []
    for key in sorted(d1):
        rec = {"p": key[0], "seed": key[1]}
        for tag, r in (("sparse", d1[key]), ("fast", d2[key])):
            oracle = r["weil_muls"] + r["f4_muls"] + r["lin_muls"] + r["post_muls"]
            la_muls = r["la_ops"] * 16
            total = r["setup_muls"] + r["base_muls"] + r["stream_muls"] + oracle + r["verify_muls"] + la_muls
            s = total / (r["c_add_e"] * math.sqrt(r["l"]))
            rho_s = [x["group_ops"] / math.sqrt(r["l"]) for x in r["rho"]]
            rho_mean = statistics.fmean(rho_s)
            rep.check(f"DLP {tag} {key}: oracle_muls, la_muls, total_muls rebuilt from parts",
                      oracle == r["oracle_muls"] and la_muls == r["la_muls"] and total == r["total_muls"],
                      [oracle, r["oracle_muls"], la_muls, r["la_muls"], total, r["total_muls"]])
            rep.check(f"DLP {tag} {key}: S = total/(c_add_E sqrt l)", rel(s, r["s"]) <= TOL, [s, r["s"]])
            rep.check(f"DLP {tag} {key}: rho S = group_ops/sqrt l, mean", all(
                rel(a, x["s"]) <= TOL for a, x in zip(rho_s, r["rho"])) and rel(rho_mean, r["rho_s_mean"]) <= TOL)
            rep.check(f"DLP {tag} {key}: S/rho = S / rho mean", rel(s / rho_mean, r["s_over_rho"]) <= TOL,
                      [s / rho_mean, r["s_over_rho"]])
            rep.check(f"DLP {tag} {key}: solved, correct, every rho correct, timed_out 0, mismatches 0",
                      r["solved"] and r["correct"] and all(x["correct"] for x in r["rho"])
                      and r["timed_out"] == 0 and r["mismatches"] == 0)
            rec[tag] = {"s": s, "s_over_rho": s / rho_mean, "rho_s_mean": rho_mean, "lin_ms": r["lin_ms"],
                        "lin_muls": r["lin_muls"], "f4_ms": r["f4_ms"], "wall_ms": r["wall_ms"],
                        "relations": r["relations"], "correct": r["correct"]}
        same = {k: (d1[key][k], d2[key][k]) for k in
                ("l", "base", "residuals", "decompositions", "relations", "unknowns", "filtered_out", "la_ops",
                 "incomplete", "cross_checked")}
        rec["arm_invariant_counters"] = same
        rep.check(f"DLP {key}: instance and relation counters identical across JV_LIN arms",
                  all(a == b for a, b in same.values()), same)
        rec["s_ratio_sparse_over_fast"] = rec["sparse"]["s"] / rec["fast"]["s"]
        rec["lin_ms_ratio_sparse_over_fast"] = rec["sparse"]["lin_ms"] / rec["fast"]["lin_ms"]
        rec["lin_muls_ratio_fast_over_sparse"] = rec["fast"]["lin_muls"] / rec["sparse"]["lin_muls"]
        out.append(rec)
    return out


def leafs(obj, path=""):
    if isinstance(obj, dict):
        for k, v in obj.items():
            yield from leafs(v, f"{path}.{k}" if path else k)
    elif isinstance(obj, list):
        for i, v in enumerate(obj):
            yield from leafs(v, f"{path}[{i}]")
    else:
        yield path, obj


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--repo", default="/home/user/crypto")
    ap.add_argument("--commit", required=True)
    ap.add_argument("--out", required=True)
    a = ap.parse_args()
    snap = Snap(Path(a.repo), a.commit)
    rep = Report()

    hosts = receipts(snap, rep, BENCH + ABL + DLP)
    host_b = {hosts[c] for c in BENCH + ABL + DLP}
    rep.check("every R2 card ran on one host (host B)", len(host_b) == 1, sorted(host_b))
    hosts_a = receipts(snap, Report(), HOST_A)  # host A: recorded, not scored
    rep.check("L1-L4 ran on a different host (host A), not pooled with R2",
              len({hosts_a[c] for c in HOST_A}) == 1 and not ({hosts_a[c] for c in HOST_A} & host_b),
              sorted({hosts_a[c] for c in HOST_A}))
    for c in DLP:
        card = hosts[c + ":card"]
        rep.check(f"{c}: env JV_LIN = {'sparse' if c == 'R2-D1' else 'fast'} and argv as protocol",
                  card["env"] == {"JV_LIN": "sparse" if c == "R2-D1" else "fast"}
                  and card["argv"][1:] == ["--exp", "dlp", "--sizes", "53,101", "--seeds", "2", "--rho-runs", "4",
                                           "--check-every", "64", "--json", f"{STUDY}/runs/{c}.json"],
                  [card["env"], card["argv"]])

    main_rows = bench_rows(snap, rep, BENCH, hosts, 7, "avx512f")
    abl_rows = bench_rows(snap, rep, ABL, hosts, 3, "portable")

    # coverage
    exp = expected_corpus()
    keys = [(r["family"], r["label"]) for r in main_rows]
    rep.check("main runs: no system appears twice", len(keys) == len(set(keys)), len(keys))
    rep.check("main runs: exactly the protocol corpus (253 systems, none dropped, none extra)",
              set(keys) == exp, {"missing": sorted(exp - set(keys)), "extra": sorted(set(keys) - exp)})
    akeys = [(r["family"], r["label"]) for r in abl_rows]
    rep.check("ablation runs: exactly the protocol corpus, each once", sorted(akeys) == sorted(exp),
              {"n": len(akeys), "missing": sorted(exp - set(akeys))})
    by_key = {(r["family"], r["label"]): r for r in main_rows}

    # gates
    disagree = [f"{r['card']}:{r['label']}" for r in main_rows if not r["agree"]]
    planted_lost = [f"{r['card']}:{r['label']}" for r in main_rows if r["family"] == "dense" and r["planted_found"] is not True]
    planted_non_dense = [f"{r['card']}:{r['label']}" for r in main_rows if r["family"] != "dense" and r["planted_found"] is not None]
    constructed_empty = [f"{r['card']}:{r['label']}" for r in main_rows
                         if r["family"] == "jv-nagao" and r["label"].endswith("c") and arm(r, "fast")["sols"] < 1]
    sols_differ = [f"{r['card']}:{r['label']}" for r in main_rows if len({x["sols"] for x in r["arms"]}) != 1]
    gave_up = [f"{r['card']}:{r['label']}:{arm(r, 'fast')['gave_up']}" for r in main_rows if arm(r, "fast")["gave_up"]]
    sparse_fb = [f"{r['card']}:{r['label']}" for r in main_rows if arm(r, "sparse")["fallback"]]
    fast_fb_without_giveup = [r["label"] for r in main_rows if arm(r, "fast")["fallback"] != bool(arm(r, "fast")["gave_up"])]
    abl_disagree = [f"{r['card']}:{r['label']}" for r in abl_rows if not r["agree"]
                    or {x["sols"] for x in r["arms"]} != {x["sols"] for x in by_key[(r["family"], r["label"])]["arms"]}]
    rep.check("gate: every arm agrees on every system", not disagree, disagree)
    rep.check("gate: every planted point (dense family) found", not planted_lost, planted_lost)
    rep.check("planted_found is null outside the dense family (bench plants no checkable point there)",
              not planted_non_dense, planted_non_dense)
    rep.check("constructed (c) Nagao systems each have >= 1 solution", not constructed_empty, constructed_empty)
    rep.check("solution counts equal across arms", not sols_differ, sols_differ)
    rep.check("fast fallback flag set iff fast gave up", not fast_fb_without_giveup, fast_fb_without_giveup)
    rep.check("ablation: agrees, and solution counts equal the main runs", not abl_disagree, abl_disagree)

    # counted-mul determinism across runs at equal si
    same_si, diff_si, viol = 0, 0, []
    for r in abl_rows:
        m = by_key[(r["family"], r["label"])]
        if r["si"] == m["si"]:
            same_si += 1
            for name in ("sparse", "fast"):
                if arm(r, name)["muls"] != arm(m, name)["muls"]:
                    viol.append(f"{r['label']}:{name}")
        else:
            diff_si += 1
    rep.check("counted muls identical between ablation and main run wherever the system index si is the same",
              not viol, {"same_si": same_si, "different_si": diff_si, "violations": viol})

    # metrics
    rec = {
        "systems": len(main_rows),
        "sparse_over_fast": family_block(main_rows, "sparse", "fast", FAMILIES),
        "dense_over_fast": family_block(main_rows, "dense", "fast", FAMILIES),
        "ablation_sparse_over_portable_fast": family_block(abl_rows, "sparse", "fast", FAMILIES),
    }
    port = {}
    for fam in FAMILIES:
        pairs = [(arm(r, "fast")["median_ms"], arm(by_key[(r["family"], r["label"])], "fast")["median_ms"])
                 for r in abl_rows if r["family"] == fam]
        port[fam] = {"geomean_of_ratios": gmean([x / y for x, y in pairs]),
                     "ratio_of_upper_medians": upper_median([x for x, _ in pairs]) / upper_median([y for _, y in pairs])}
    rec["portable_over_simd_fast"] = port
    dense_rows = [r for r in main_rows if r["family"] == "dense"]
    rho = spearman([arm(r, "sparse")["median_ms"] / arm(r, "fast")["median_ms"] for r in dense_rows],
                   [r["delta"] for r in dense_rows])
    rec["P2_spearman_rho"] = rho
    rec["P3_gave_up_fraction"] = len(gave_up) / len(main_rows)

    # host A, beside (not pooled)
    host_a_rows = []
    for c in HOST_A:
        doc = snap.json(f"{STUDY}/runs/{c}.json")
        host_a_rows += [{"card": c, **row} for row in doc["rows"]]
    a_keys = {(r["family"], r["label"]) for r in host_a_rows}
    a_missing = sorted(exp - a_keys)
    named = {("dense", "n3-d10-p32-1"), ("dense", "n3-d10-p32-2")} | {
        ("dense", f"n6-d3-p{p.bit_length()}-{k}") for p in DENSE_P for k in range(3)}
    named |= {("gaudry-s5", f"p{p}-{k}") for p in (13, 101) for k in range(2)}  # L5/L6 never ran on host A
    rep.check("host A (L1-L4, beside only): missing systems are exactly amendment 1's 11 plus gaudry-s5 (not run)",
              set(a_missing) == named and len(host_a_rows) == len(a_keys) == 238, a_missing)
    for c in HOST_A:
        ra = snap.json(f"{STUDY}/cards/CARD-postgb-la-20261026-{c}.receipt.json")
        rep.check(f"{c} (host A): receipt completed, every command exit 0",
                  ra["status"] == "completed" and all(x["exit_code"] == 0 for x in ra["commands"]))
    rec["host_a_sparse_over_fast"] = family_block(host_a_rows, "sparse", "fast", FAMILIES)
    rec["host_a_dense_over_fast"] = family_block(host_a_rows, "dense", "fast", FAMILIES)
    rec["dlp"] = dlp_block(snap, rep)

    # predictions, from the recomputation
    sof = rec["sparse_over_fast"]
    preds = {
        "P1": all(sof[f]["geomean"] >= 1.5 and sof[f]["ci95"][0] > 1 for f in FAMILIES),
        "P2": rho > 0,
        "P3": rec["P3_gave_up_fraction"] <= 0.05,
        "P4_logs_correct": all(d[t]["correct"] for d in rec["dlp"] for t in ("sparse", "fast")),
        # protocol wording: fast's counted muls within +-25% of sparse's, i.e. fast/sparse in [0.75, 1.25];
        # summary.json states the range as sparse/fast; the verdict is checked in both directions
        "P4_counted_within_25pct_every_system": all(0.75 <= 1 / x <= 1.25 for f in FAMILIES for x in
                                                     [sof[f]["counted_ratio"]["min"], sof[f]["counted_ratio"]["max"]]),
        "P4_counted_within_25pct_sparse_over_fast_direction": all(
            0.75 <= x <= 1.25 for f in FAMILIES for x in [sof[f]["counted_ratio"]["min"], sof[f]["counted_ratio"]["max"]]),
        "P4_counted_ratio_range_fast_over_sparse": [1 / max(sof[f]["counted_ratio"]["max"] for f in FAMILIES),
                                                    1 / min(sof[f]["counted_ratio"]["min"] for f in FAMILIES)],
        "P4_counted_ratio_range": [min(sof[f]["counted_ratio"]["min"] for f in FAMILIES),
                                   max(sof[f]["counted_ratio"]["max"] for f in FAMILIES)],
        "P4_s_within_10pct": all(abs(d["fast"]["s"] / d["sparse"]["s"] - 1) <= 0.10 for d in rec["dlp"]),
        "P4_lin_ms_lower_every_run": all(d["fast"]["lin_ms"] < d["sparse"]["lin_ms"] for d in rec["dlp"]),
        "P5": all(rec["ablation_sparse_over_portable_fast"][f]["geomean"] >= 1.2 for f in FAMILIES),
    }
    preds["P4"] = (preds["P4_logs_correct"] and preds["P4_counted_within_25pct_every_system"]
                   and preds["P4_s_within_10pct"] and preds["P4_lin_ms_lower_every_run"])
    rec["predictions"] = preds
    rec["gates"] = {"disagree": disagree, "planted_lost": planted_lost, "fast_gave_up": gave_up,
                    "sparse_fell_back": sparse_fb, "ablation_disagree": abl_disagree}

    # ---- compare with analysis/summary.json ------------------------------
    claimed = snap.json(f"{STUDY}/analysis/summary.json")
    headline, unmatched, mismatched, stochastic = [], [], [], []

    def cmp(name, c, r, tol=TOL, head=False):
        ok = (c == r) if not (isinstance(c, float) or isinstance(r, float)) else rel(float(c), float(r)) <= tol
        item = {"name": name, "claimed": c, "recomputed": r, "rel_tol": tol}
        if head:
            headline.append(item)
        if not ok:
            mismatched.append(item)
        return ok

    for block in ("sparse_over_fast", "dense_over_fast", "ablation_sparse_over_portable_fast",
                  "host_a_sparse_over_fast", "host_a_dense_over_fast"):
        for fam, cv in claimed[block].items():
            rv = rec[block].get(fam)
            if rv is None:
                unmatched.append(f"{block}.{fam}")
                continue
            head = not block.startswith("host_a")
            for k in ("systems", "delta", "n", "p"):
                cmp(f"{block}.{fam}.{k}", cv[k], rv[k])
            cmp(f"{block}.{fam}.geomean", cv["geomean"], rv["geomean"], head=head)
            cmp(f"{block}.{fam}.min", cv["min"], rv["min"], head=head)
            cmp(f"{block}.{fam}.max", cv["max"], rv["max"], head=head)
            for arm_name, val in cv["median_ms"].items():
                cmp(f"{block}.{fam}.median_ms.{arm_name}", val, rv["median_ms"][arm_name])
            for k, val in cv["counted_ratio"].items():
                cmp(f"{block}.{fam}.counted_ratio.{k}", val, rv["counted_ratio"][k])
            for i, val in enumerate(cv["ci95"]):
                stochastic.append({"name": f"{block}.{fam}.ci95[{i}]", "claimed": val,
                                   "recomputed_own_bootstrap": rv["ci95"][i],
                                   "rel_diff": rel(val, rv["ci95"][i])})
                # reproduced exactly by the validator's own seeded stdlib bootstrap -> compared at 1e-9 too
                cmp(f"{block}.{fam}.ci95[{i}]", val, rv["ci95"][i], head=head)
        extra = set(rec[block]) - set(claimed[block])
        if extra:
            unmatched.append(f"{block}: recomputed families absent from summary: {sorted(extra)}")
    for fam, val in claimed["portable_over_simd_fast"].items():
        cand = rec["portable_over_simd_fast"][fam]
        best = min(cand, key=lambda k: rel(val, cand[k]))
        cmp(f"portable_over_simd_fast.{fam} ({best})", val, cand[best], head=True)
    cmp("gates.systems", claimed["gates"]["systems"], rec["systems"], head=True)
    for k in ("disagree", "planted_lost", "fast_gave_up", "sparse_fell_back", "ablation_disagree"):
        cmp(f"gates.{k}", claimed["gates"][k], rec["gates"][k])
    cmp("gates.pass", claimed["gates"]["pass"], not disagree and not planted_lost)
    cmp("P2.spearman_rho", claimed["predictions"]["P2"]["spearman_rho"], rho, head=True)
    cmp("P2.systems", claimed["predictions"]["P2"]["systems"], len(dense_rows))
    cmp("P3.gave_up_fraction", claimed["predictions"]["P3"]["gave_up_fraction"], rec["P3_gave_up_fraction"], head=True)
    for p_name in ("P1", "P2", "P3", "P5"):
        cmp(f"{p_name}.held", claimed["predictions"][p_name]["held"], preds[p_name])
    for fam, (g, ci) in claimed["predictions"]["P1"]["by_family"].items():
        cmp(f"P1.by_family.{fam}.geomean", g, sof[fam]["geomean"])
        cmp(f"P1.by_family.{fam}.geomean = sparse_over_fast.geomean", g, claimed["sparse_over_fast"][fam]["geomean"])
        cmp(f"P1.by_family.{fam}.ci95 = sparse_over_fast.ci95", ci, claimed["sparse_over_fast"][fam]["ci95"])
    for fam, g in claimed["predictions"]["P5"]["by_family"].items():
        cmp(f"P5.by_family.{fam}", g, rec["ablation_sparse_over_portable_fast"][fam]["geomean"])
    p4c, p4r = claimed["predictions"]["P4"], preds
    cmp("P4.logs_correct", p4c["logs_correct"], p4r["P4_logs_correct"])
    cmp("P4.counted_within_25pct_every_system", p4c["counted_within_25pct_every_system"],
        p4r["P4_counted_within_25pct_every_system"])
    for i in range(2):
        cmp(f"P4.counted_ratio_range[{i}]", p4c["counted_ratio_range"][i], p4r["P4_counted_ratio_range"][i])
    cmp("P4.s_within_10pct", p4c["s_within_10pct"], p4r["P4_s_within_10pct"])
    cmp("P4.lin_ms_lower_every_run", p4c["lin_ms_lower_every_run"], p4r["P4_lin_ms_lower_every_run"])
    cmp("P4.held", p4c["held"], p4r["P4"])
    rdlp = {(d["p"], d["seed"]): d for d in rec["dlp"]}
    for cd in claimed["dlp"]:
        d = rdlp[(cd["p"], cd["seed"])]
        tag = f"dlp[p={cd['p']},seed={cd['seed']}]"
        for i, arm_name in enumerate(("sparse", "fast")):
            cmp(f"{tag}.S.{arm_name}", cd["s"][i], d[arm_name]["s"], head=True)
            cmp(f"{tag}.S_over_rho.{arm_name}", cd["s_over_rho"][i], d[arm_name]["s_over_rho"], head=True)
            cmp(f"{tag}.lin_ms.{arm_name}", cd["lin_ms"][i], d[arm_name]["lin_ms"], head=True)
            cmp(f"{tag}.lin_muls.{arm_name}", cd["lin_muls"][i], d[arm_name]["lin_muls"])
            cmp(f"{tag}.f4_ms.{arm_name}", cd["f4_ms"][i], d[arm_name]["f4_ms"])
            cmp(f"{tag}.wall_ms.{arm_name}", cd["wall_ms"][i], d[arm_name]["wall_ms"])
            cmp(f"{tag}.rho_s_mean.{arm_name}", cd["rho_s_mean"][i], d[arm_name]["rho_s_mean"])
            cmp(f"{tag}.relations.{arm_name}", cd["relations"][i], d[arm_name]["relations"])
            cmp(f"{tag}.correct.{arm_name}", cd["correct"][i], d[arm_name]["correct"])
        cmp(f"{tag}.lin_ms_ratio_sparse_over_fast", cd["lin_ms_ratio_sparse_over_fast"],
            d["lin_ms_ratio_sparse_over_fast"], head=True)
        cmp(f"{tag}.s_ratio_sparse_over_fast", cd["s_ratio_sparse_over_fast"], d["s_ratio_sparse_over_fast"], head=True)
        cmp(f"{tag}.rho_pooled_mean_s", cd["rho_pooled_mean_s"],
            statistics.fmean([d["sparse"]["rho_s_mean"], d["fast"]["rho_s_mean"]]), tol=1e-12)
        cmp(f"{tag}.rho_all_correct", cd["rho_all_correct"], True)

    # summary.systems: one entry per main-run row, copied exactly
    sys_claim = {(s["family"], s["label"]): s for s in claimed["systems"]}
    cmp("summary.systems: count and keys = main-run rows", sorted(sys_claim), sorted(by_key))
    bad_sys = []
    for k, s in sys_claim.items():
        r = by_key.get(k)
        if r is None:
            continue
        fa = arm(r, "fast")["fast"] or {}
        want = {"card": r["card"], "n": r["n"], "p": r["p"], "delta": r["delta"], "f4_ms": r["f4_ms"],
                "sols": r["arms"][0]["sols"], "agree": r["agree"], "planted_found": r["planted_found"],
                "ms": {x["arm"]: x["median_ms"] for x in r["arms"]}, "muls": {x["arm"]: x["muls"] for x in r["arms"]},
                "fast_phases_ms": {f: fa.get(f) for f in ("nf_ms", "krylov_ms", "bm_ms", "roots_ms", "readout_ms")}}
        got = {f: s.get(f) for f in want}
        if got != want:
            bad_sys.append({"system": k, "claimed": got, "raw": want})
    rep.check("summary.systems: every entry equals its raw row (card, n, p, delta, f4_ms, sols, agree, planted, "
              "median ms, muls, fast phases)", not bad_sys, bad_sys[:5])
    for block in ("meta", "meta_ablation", "meta_host_a", "skipped"):
        for c, v in claimed[block].items():
            doc = snap.json(f"{STUDY}/runs/{c}.json")
            if block == "skipped":
                cmp(f"skipped.{c}", v, doc.get("skipped"))
            else:
                cmp(f"{block}.{c}", v, {k: doc.get(k) for k in v})
    cmp("summary.host", claimed["host"], "B (amendment 2)")
    known = {"schema", "host", "meta", "meta_ablation", "meta_host_a", "skipped", "gates", "sparse_over_fast",
             "dense_over_fast", "ablation_sparse_over_portable_fast", "host_a_sparse_over_fast",
             "host_a_dense_over_fast", "portable_over_simd_fast", "systems", "dlp", "predictions"}
    for k in set(claimed) - known:
        unmatched.append(f"top-level key {k} not recomputed")
    for name in ("P1", "P2", "P3", "P4", "P5"):
        for k in set(claimed["predictions"][name]) - {"held", "by_family", "spearman_rho", "systems",
                                                       "gave_up_fraction", "logs_correct",
                                                       "counted_within_25pct_every_system", "counted_ratio_range",
                                                       "s_within_10pct", "lin_ms_lower_every_run"}:
            unmatched.append(f"predictions.{name}.{k} not recomputed")

    worst_ci = max(x["rel_diff"] for x in stochastic)
    ci_above_one = all(rec["sparse_over_fast"][f]["ci95"][0] > 1 for f in FAMILIES)
    rep.check("own bootstrap CI (sparse/fast, every family) lies above 1", ci_above_one)
    rep.check("claimed bootstrap CIs within 5% of an independent bootstrap", worst_ci <= 0.05, worst_ci)
    rep.check("every compared summary.json number matches the recomputation", not mismatched, mismatched[:10])
    rep.check("every summary.json block was recomputed", not unmatched, unmatched)

    raw = [f"{STUDY}/runs/{c}.json" for c in BENCH + ABL + DLP + HOST_A]
    raw += [f"{STUDY}/cards/CARD-postgb-la-20261026-{c}.{s}" for c in BENCH + ABL + DLP
            for s in ("yaml", "receipt.json", "lscpu.stdout", "run.stderr", "run.stdout")]
    raw += [f"{STUDY}/PROTOCOL.md", f"{STUDY}/PROTOCOL_AMENDMENT_1.md", f"{STUDY}/PROTOCOL_AMENDMENT_2.md",
            f"{STUDY}/analysis/summary.json"]
    status = "pass" if not rep.problems else "fail"
    for fam in FAMILIES:
        for block in ("sparse_over_fast", "dense_over_fast", "ablation_sparse_over_portable_fast"):
            rec[block].get(fam, {}).pop("_ratios", None)
    for block in ("host_a_sparse_over_fast", "host_a_dense_over_fast"):
        for v in rec[block].values():
            v.pop("_ratios", None)
    report = {
        "schema": "postgb-la-validator-report/v1",
        "status": status,
        "independent": True,
        "independence": "recompute.py was written by the validator from PROTOCOL.md and amendments 1-2; "
                        "scripts/summary.py was not read or run",
        "snapshot_commit": snap.commit,
        "inputs_read_via": "git show <snapshot_commit>:<path> (working tree not read)",
        # paths relative to this report's directory (validation/), as the gate resolves them
        # against the evidence directory holding VALIDATION.json; repo_path is the same file
        "raw_data_paths_relative_to": f"{STUDY}/validation",
        "raw_data": [{"path": "../" + p[len(STUDY) + 1:], "repo_path": p, "sha256": snap.sha(p)} for p in raw],
        "headline": headline,
        "hosts": {c: hosts[c] for c in BENCH + ABL + DLP} | {c: hosts_a[c] for c in HOST_A},
        "checks": rep.checks,
        "problems": rep.problems,
        "mismatched": mismatched,
        "unmatched": unmatched,
        "stochastic_ci_comparison": stochastic,
        "recomputed": rec,
    }
    Path(a.out).write_text(json.dumps(report, indent=1) + "\n")
    print(f"status {status}: {len(rep.checks)} checks, {len(rep.problems)} problems, "
          f"{len(headline)} headline numbers, {len(mismatched)} mismatches, {len(unmatched)} unmatched, "
          f"worst CI rel diff {worst_ci:.4f}")
    for p in rep.problems:
        print("PROBLEM", json.dumps(p)[:600])
    for m in mismatched[:20]:
        print("MISMATCH", json.dumps(m)[:400])
    for u in unmatched:
        print("UNMATCHED", u)


if __name__ == "__main__":
    main()
