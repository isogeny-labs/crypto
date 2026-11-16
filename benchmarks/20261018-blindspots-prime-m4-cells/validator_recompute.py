#!/usr/bin/env python3
"""Independent validator recompute for blind-spot item 14 (first prime-field m = 4 cost cells).

Stdlib only. Run from anywhere:

    python3 -I benchmarks/20261018-blindspots-prime-m4-cells/validator_recompute.py

It reads only committed files: the raw JSONL records, the 12 card receipts, the 12 frozen command
cards, the selftest log, and (for the claimed numbers and their print precision) the item README and
raw/summary.md. It recomputes every headline from the raw JSONL with its own arithmetic. It does not
call the producer's instrument and does not copy a number from summary.md. Then it writes
`validator-report.json` (validation-gate/v1 validator report) and `rho-reference.json` (a draft
rho_reference descriptor, not VALIDATION.json) next to itself.

Tolerance policy: each headline's rel_tol is the sum of the half-units of every rounding stage
between the exact value and the README's printed digits, divided by |claimed|. The README says its
numbers are "those printed by `summarize --dir raw` (raw/summary.md)", and summarize prints with
fixed formats ({:.1}, {:.2}, {:.3e}, {:.4e}, {:.3}, {:.4}), so a README integer copied from a {:.1}
print carries two roundings (0.05 + 0.5 units). Counts are exact (rel_tol 0). No tolerance is
widened beyond the rounding chain.
"""
from __future__ import annotations

import hashlib
import json
import math
import subprocess
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parent.parent
ITEM = REPO / "research/blindspots_20261018/14-prime-m4-cells"
RAW = ITEM / "raw"
INSTRUMENT_SHA = "f6696fc0cafff9120189072a7785452422d68c7944a2812d4667e27c8872b376"
NATIVE = ("lookup", "insert", "legendre", "sqrt", "rowop", "madd", "dptest", "build")
WS = (0.0, 0.1, 1.0)
WKEY = ("0", "0.1", "1")
DESIGN = {16: 8, 20: 15, 24: 29, 28: 63}
# PROTOCOL.md "Cells, controls, targets" table (checked against the raw cell records below).
PROTOCOL_CELLS = {
    16: (48479, 25392, 20877, 48383),
    20: (661603, 422772, 346249, 660917),
    24: (10539323, 7836717, 8828358, 10542893),
    28: (248685287, 49054657, 177851102, 248685841),
}
T975 = {2: 4.302652729911275, 4: 2.7764451051977987}  # Student t, two-sided 95%
# Committed snapshot at which this validation started; every file read must be identical there and at HEAD.
SNAPSHOT_START = "f543b5523357eb11490a30fcac84d17359ce98ad"

read_files: dict[Path, str] = {}  # path -> role


def sha(p: Path) -> str:
    return hashlib.sha256(p.read_bytes()).hexdigest()


def rel(p: Path) -> str:
    return str(Path("../..") / p.resolve().relative_to(REPO))


def load(name: str, role: str = "raw") -> list[dict]:
    p = RAW / name
    read_files[p] = role
    return [json.loads(x) for x in p.read_text().splitlines() if x.strip()]


def mean(v):
    v = list(v)
    return sum(v) / len(v)


def ols(x, y, df_expected=None):
    n = len(x)
    mx, my = mean(x), mean(y)
    sxx = sum((a - mx) ** 2 for a in x)
    m = sum((a - mx) * (b - my) for a, b in zip(x, y)) / sxx
    c = my - m * mx
    rss = sum((b - (c + m * a)) ** 2 for a, b in zip(x, y))
    df = n - 2
    if df_expected is not None:
        assert df == df_expected, (df, df_expected)
    se = math.sqrt(rss / df / sxx)
    t = T975[df]
    return m, m - t * se, m + t * se


def native(counts: dict) -> int:
    return sum(counts[k] for k in NATIVE)


checks: list[dict] = []


def check(name: str, ok: bool, detail: str = "") -> None:
    checks.append({"check": name, "ok": bool(ok), "detail": detail})


# ---------------------------------------------------------------------------------------------
# 1. Artifact binding: cards, receipts, output digests, commits, dirty state, source identity.
# ---------------------------------------------------------------------------------------------
def git_blob_sha(commit: str, path: str) -> str | None:
    try:
        out = subprocess.run(["git", "-C", str(REPO), "show", f"{commit}:{path}"],
                             capture_output=True, check=True).stdout
    except (OSError, subprocess.CalledProcessError):
        return None
    return hashlib.sha256(out).hexdigest()


receipts = {}
for i in range(1, 13):
    card = ITEM / f"card-{i:02d}.yaml"
    rp = RAW / f"receipt-card-{i:02d}.json"
    read_files[card] = "frozen command card"
    read_files[rp] = "execution receipt"
    r = json.loads(rp.read_text())
    receipts[i] = r
    cmds = r["commands"]
    bad = [c["id"] for c in cmds if not (c["succeeded"] and c["exit_code"] == 0 and not c["timed_out"])]
    outs = [(p, d) for c in cmds for p, d in c["outputs"].items()]
    mism = [p for p, d in outs if not (REPO / p).is_file() or sha(REPO / p) != d]
    dirty_src = [x for x in r["dirty_before"] if "blindspots_14_prime_m4" in str(x)]
    bind_ok = f"{{path: examples/blindspots_14_prime_m4.rs, sha256: {INSTRUMENT_SHA}}}" in card.read_text()
    src_at_commit = git_blob_sha(r["commit"], "examples/blindspots_14_prime_m4.rs")
    builds = [c for c in cmds if c["id"] == "build"]
    check(f"card-{i:02d} receipt",
          r["card_sha256"] == sha(card) and r["status"] == "completed" and not bad and not mism
          and len(cmds) == r["commands_total"] and not dirty_src and bind_ok
          and src_at_commit == INSTRUMENT_SHA and len(builds) == 1
          and builds[0]["argv"][-1] == "examples/blindspots_14_prime_m4.rs",
          f"card sha256 matches receipt; status {r['status']}; {len(cmds)} commands, failed {bad}; "
          f"{len(outs)} output digests, mismatched {mism}; commit {r['commit']} with instrument sha256 "
          f"{src_at_commit}; dirty_before {len(r['dirty_before'])} paths, instrument dirty: {dirty_src}")

# Rho and arm (a) ran in one binary: card 01 builds target/blindspots-14/card-01/b14 once.
c1 = receipts[1]["commands"]
bin1 = "target/blindspots-14/card-01/b14"
rho_ids = {f"rho-b{b}" for b in DESIGN}
a_ids = {f"fullic-a-b{b}" for b in DESIGN}
builds1 = [c for c in c1 if c["id"] == "build"]
users = {c["id"] for c in c1 if c["argv"] and bin1 in c["argv"][:3]}
order = {c["id"]: c["index"] for c in c1}
same_binary = (len(builds1) == 1 and builds1[0]["argv"][builds1[0]["argv"].index("-o") + 1] == bin1
               and rho_ids <= users and a_ids <= users
               and all(order["build"] < order[k] < order["remove-bin"] for k in rho_ids | a_ids))
check("rho and arm (a) full IC invoke one binary built once in card 01", same_binary,
      f"build index {order['build']} -> {bin1}; rho indices {sorted(order[k] for k in rho_ids)}; "
      f"arm (a) full-IC indices {sorted(order[k] for k in a_ids)}; remove-bin index {order['remove-bin']}")

st = ITEM / "raw/card-01-selftest.stdout.log"
read_files[st] = "control log"
check("selftest --deep passed", "SELFTEST OK" in st.read_text(), "card-01-selftest.stdout.log")

inst = REPO / "research/blindspots_20261018/instruments/blindspots_14_prime_m4.rs"
read_files[inst] = "instrument source (relocated byte-identically); read for field semantics only"
check("relocated instrument source is the bound source", sha(inst) == INSTRUMENT_SHA, sha(inst))

# ---------------------------------------------------------------------------------------------
# 2. Recompute from raw JSONL.
# ---------------------------------------------------------------------------------------------
P = {}
for b, bx in DESIGN.items():
    cellrec = [r for r in load(f"tdec-a-b{b}.jsonl") if r.get("kind") == "cell"][0]
    P[b] = cellrec["p"]
    check(f"cell {b} matches PROTOCOL table",
          (cellrec["p"], cellrec["a"], cellrec["b"], cellrec["n"]) == PROTOCOL_CELLS[b] and cellrec["bx"] == bx
          and cellrec["a"] % cellrec["p"] != 0 and cellrec["b"] % cellrec["p"] != 0,
          f"p={cellrec['p']} a={cellrec['a']} b={cellrec['b']} n={cellrec['n']} bx={cellrec['bx']} "
          "(a, b nonzero: j not in {0, 1728}, Aut(E) = {+-1}, negation is the only symmetry)")

# Matched rho: rho(w) = mean(gae) + w * mean(native); gae = counts.gadd, which includes setup.
rho, rho_ok = {}, {}
for b in DESIGN:
    rs = [r for r in load(f"rho-b{b}.jsonl") if r.get("kind") == "rho"]
    assert sorted(r["run"] for r in rs) == list(range(16))
    assert all(r["gae"] == r["counts"]["gadd"] and r["native"] == native(r["counts"]) for r in rs)
    assert all(r["counts"]["gadd"] > r["setup_gadd"] for r in rs)
    rho[b] = [mean(r["gae"] for r in rs) + w * mean(r["native"] for r in rs) for w in WS]
    rho_ok[b] = sum(1 for r in rs if r["ok"] is True)


def fullic(files, arm, b, bx):
    rs = [r for f in files for r in load(f) if r.get("kind") == "fullic"]
    assert all(r["arm"] == arm and r["bits"] == b and r["bx"] == bx for r in rs)
    assert sorted(r["run"] for r in rs) == list(range(16)), "runs are not exactly 0..15"
    for r in rs:
        for k in r["total"]:
            assert r["total"][k] == r["setup"][k] + r["online"][k] + r["verify"][k]
        assert r["gae_total"] == r["total"]["gadd"] and r["native_total"] == native(r["total"])
    S = [mean(r["gae_total"] for r in rs) + w * mean(r["native_total"] for r in rs) for w in WS]
    return S, sum(1 for r in rs if r["ok"] is True), mean(r["attempts"] for r in rs), rs


A = {}
for b, bx in DESIGN.items():
    S, ok, att, _ = fullic([f"fullic-a-b{b}.jsonl"], "a", b, bx)
    A[b] = {"S": S, "ok": ok, "att": att, "ratio": [S[i] / rho[b][i] for i in range(3)]}
A_ladder = {}
for bx in (3, 4, 5, 6, 7):
    S, ok, att, _ = fullic([f"fullic-a-b16-bx{bx}.jsonl"], "a", 16, bx)
    A_ladder[bx] = {"S": S, "ok": ok}

B_full = {}
for bx, files in ((3, ["fullic-b-b16-bx3.jsonl"]),
                  (4, ["fullic-b-b16-bx4-r00-06.jsonl", "fullic-b-b16-bx4-r06-11.jsonl",
                       "fullic-b-b16-bx4-r11-16.jsonl"])):
    S, ok, att, rs = fullic(files, "b", 16, bx)
    B_full[bx] = {"S": S, "ok": ok, "att": att, "ratio": [S[i] / rho[16][i] for i in range(3)],
                  "setup_build": sorted({r["setup"]["build"] for r in rs})}

# Arm (a) T_dec and its controls (all nine tdec-a cells).
Ta, a_controls = {}, []
for b, bx, f in [(16, 8, "tdec-a-b16.jsonl"), (20, 15, "tdec-a-b20.jsonl"), (24, 29, "tdec-a-b24.jsonl"),
                 (28, 63, "tdec-a-b28.jsonl")] + [(16, k, f"tdec-a-b16-bx{k}.jsonl") for k in (3, 4, 5, 6, 7)]:
    rs = [r for r in load(f) if r.get("kind") == "tdec"]
    rnd = [r for r in rs if r["tkind"] == "random"]
    pl = [r for r in rs if r["tkind"] == "planted"]
    mism = sum(1 for r in rnd + pl if r["hit"] != r["crosscheck_decomposable"])
    miss_cost = {r["counts"]["gadd"] for r in rnd if not r["hit"]}
    a_controls.append({"bits": b, "bx": bx, "random": len(rnd), "planted_found": sum(r["hit"] for r in pl),
                       "planted": len(pl), "crosscheck_mismatches": mism, "miss_gadd": sorted(miss_cost)})
    if DESIGN.get(b) == bx:
        Ta[b] = [mean(r["counts"]["gadd"] for r in rnd) + w * mean(r["native"] for r in rnd) for w in WS]
check("arm (a) planted 4-sums found and pair-table cross-check agrees at every cell",
      all(c["planted_found"] == c["planted"] == 4 and c["crosscheck_mismatches"] == 0 and c["random"] == 16
          and c["miss_gadd"] == [2 * c["bx"]] for c in a_controls),
      json.dumps(a_controls))

# Arm (b) T_dec: all tdec-b files.
tdec_b_files = ["tdec-b-b16-bx3.jsonl", "tdec-b-b16-bx4.jsonl", "tdec-b-b16-bx5.jsonl", "tdec-b-b16-bx6.jsonl",
                "tdec-b-b16-bx7-t00-09.jsonl", "tdec-b-b16-bx7-t09-18.jsonl",
                "tdec-b-b16-bx8-t00-05.jsonl", "tdec-b-b16-bx8-t05-10.jsonl", "tdec-b-b16-bx8-t10-15.jsonl",
                "tdec-b-b16-bx8-t15-18.jsonl",
                "tdec-b-b20-bx5.jsonl", "tdec-b-b20-bx6.jsonl", "tdec-b-b24-bx5.jsonl", "tdec-b-b24-bx6.jsonl",
                "tdec-b-b28-bx5.jsonl", "tdec-b-b28-bx6.jsonl",
                "tdec-b-b20-bx15.jsonl", "tdec-b-b24-bx29.jsonl", "tdec-b-b28-bx63.jsonl"]
Bc: dict[tuple, list] = {}
s5_build = set()
for f in tdec_b_files:
    for r in load(f):
        if r.get("kind") == "tdec" and r.get("arm") == "b":
            Bc.setdefault((r["bits"], r["bx"]), []).append(r)
        if r.get("kind") == "setup" and r.get("arm") == "b":
            s5_build.add(r["counts"]["build"])
b_controls, Bm = [], {}
for (b, bx), rs in sorted(Bc.items()):
    tg = sorted(r["target"] for r in rs)
    assert len(set(tg)) == len(tg), (b, bx, "duplicate target index")
    assert all(r["counts"]["gadd"] == 0 and r["native"] == native(r["counts"]) for r in rs)
    rnd = [r for r in rs if r["tkind"] == "random"]
    pl = [r for r in rs if r["tkind"] == "planted"]
    done = [r for r in rs if r["status"] == "done"]
    chk = [r for r in rs if isinstance(r.get("dim"), int) and isinstance(r.get("brute_roots"), int)]
    b_controls.append({"bits": b, "bx": bx, "targets": len(rs), "random": len(rnd), "planted": len(pl),
                       "done": len(done), "status": sorted({r["status"] for r in rs}),
                       "dim_eq_brute": f"{sum(r['dim'] == r['brute_roots'] for r in chk)}/{len(chk)}",
                       "planted_vanishes": f"{sum(r.get('planted_vanishes') is True for r in pl)}/{len(pl)}",
                       "random_dims": sorted({r.get('dim') for r in rnd}, key=str)})
    if len(done) == len(rs):
        Bm[(b, bx)] = {"all18": mean(r["counts"]["madd"] for r in rs),
                       "random16": mean(r["counts"]["madd"] for r in rnd),
                       "native_all18": mean(r["native"] for r in rs)}
check("arm (b) quotient dimension equals brute-force root count at every finished cell; planted vanishes",
      all(c["dim_eq_brute"] == f"{c['targets']}/{c['targets']}" and c["planted_vanishes"] == "2/2"
          and c["random_dims"] == [0] for c in b_controls if c["done"] == c["targets"]),
      json.dumps([c for c in b_controls if c["done"] == c["targets"]]))
check("arm (b) design cells 20/24/28 stopped by the count cap on all 16 targets (lower bounds only)",
      all(c["status"] == ["budget-nnz"] and c["targets"] == 16 for c in b_controls if c["bx"] in (15, 29, 63)),
      json.dumps([c for c in b_controls if c["bx"] in (15, 29, 63)]))

lad = [bx for bx in (3, 4, 5, 6, 7, 8) if (16, bx) in Bm]
assert lad == [3, 4, 5, 6, 7, 8]
lx = [math.log2(2 * bx) for bx in lad]
slope18 = ols(lx, [math.log2(Bm[(16, bx)]["all18"]) for bx in lad], df_expected=4)
slope16 = ols(lx, [math.log2(Bm[(16, bx)]["random16"]) for bx in lad], df_expected=4)

# Derived fits as the README states them: x = the cell label "log2 p" (16, 20, 24, 28), w = 0.
xl = [float(b) for b in DESIGN]
xp = [math.log2(P[b]) for b in DESIGN]
growth = {w: ols(xl, [math.log2(A[b]["ratio"][i]) for b in DESIGN], 2)[0] for i, w in enumerate(WKEY)}
growth_actual_p = {w: ols(xp, [math.log2(A[b]["ratio"][i]) for b in DESIGN], 2)[0] for i, w in enumerate(WKEY)}
S_exp = {w: ols(xl, [math.log2(A[b]["S"][i]) for b in DESIGN], 2)[0] for i, w in enumerate(WKEY)}
rho_exp = {w: ols(xl, [math.log2(rho[b][i]) for b in DESIGN], 2)[0] for i, w in enumerate(WKEY)}
S_exp_p = {w: ols(xp, [math.log2(A[b]["S"][i]) for b in DESIGN], 2)[0] for i, w in enumerate(WKEY)}
rho_exp_p = {w: ols(xp, [math.log2(rho[b][i]) for b in DESIGN], 2)[0] for i, w in enumerate(WKEY)}
Ta_exp = ols(xp, [math.log2(Ta[b][0]) for b in DESIGN], df_expected=2)  # summarizer convention: actual p

model_oracle = A[16]["att"] * Bm[(16, 8)]["all18"]  # MODEL lower bound on arm (b) oracle phase at bx = 8

# ---------------------------------------------------------------------------------------------
# 3. Headlines: claimed (README) vs recomputed, rel_tol from the rounding chain.
# ---------------------------------------------------------------------------------------------
headline = []


def H(name, claimed, recomputed, stages, note=""):
    tol = 0.0 if not stages else sum(0.5 * u for u in stages) / abs(claimed)
    e = {"name": name, "claimed": claimed, "recomputed": recomputed, "rel_tol": tol,
         "rounding_chain": stages}
    if note:
        e["note"] = note
    headline.append(e)


# Arm (b) growth exponent.
H("arm (b) |F|-exponent: OLS slope log2 madds on log2 |F|, 16-bit ladder bx=3..8, mean over the 18 targets "
  "(16 random + 2 planted) as the README states", 11.031, slope18[0], [0.001])
H("arm (b) |F|-exponent 95% t lower bound (4 df), README definition", 11.009, slope18[1], [0.001])
H("arm (b) |F|-exponent 95% t upper bound (4 df), README definition", 11.053, slope18[2], [0.001])
H("arm (b) |F|-exponent at headline precision (commit subject 'slope 11.03'), frozen PROTOCOL definition "
  "(mean over the 16 random targets only)", 11.03, slope16[0], [0.01],
  "at 3 decimals the protocol definition gives 11.026, not 11.031; see discrepancies")
H("arm (b) p-exponent = slope/4 (|F| = p^(1/4) assumed)", 2.76, slope18[0] / 4, [0.01])
for bx, claim in zip((3, 4, 5, 6, 7, 8), (5.48e5, 1.33e7, 1.56e8, 1.16e9, 6.32e9, 2.75e10)):
    H(f"arm (b) mean multiply-adds per attempt, 16 bits, bx={bx} (18 targets, README definition)",
      claim, Bm[(16, bx)]["all18"], [10 ** (math.floor(math.log10(claim)) - 4), 10 ** (math.floor(math.log10(claim)) - 2)])

# Arm (a) full IC table, every cell.
fmt_S1 = {16: 1, 20: 10, 24: 100, 28: 1000}
claims_a = {
    16: (155.7, 3651, 3997, 7108, 566.5, 602.6, 927.8, 6.45, 6.63, 7.66),
    20: (368.4, 16481, 18139, 33060, 1119, 1208, 2006, 14.73, 15.02, 16.48),
    24: (719.0, 76401, 84242, 154800, 3708, 4053, 7157, 20.61, 20.79, 21.63),
    28: (1816.6, 568573, 627636, 1159000, 18913, 20775, 37538, 30.06, 30.21, 30.88),
}
for b, c in claims_a.items():
    bx = DESIGN[b]
    rho_stage = [0.1] if b == 16 else [0.1, 1]
    r1_stage = [0.001, 0.01] if b == 16 else [0.01]
    H(f"arm (a) full IC {b} bits bx={bx}: mean attempts", c[0], A[b]["att"], [0.1])
    H(f"arm (a) full IC {b} bits: S(0)", c[1], A[b]["S"][0], [0.1, 1])
    H(f"arm (a) full IC {b} bits: S(0.1)", c[2], A[b]["S"][1], [0.1, 1])
    H(f"arm (a) full IC {b} bits: S(1)", c[3], A[b]["S"][2], [fmt_S1[b]])
    H(f"matched rho {b} bits: rho(0)", c[4], rho[b][0], rho_stage)
    H(f"matched rho {b} bits: rho(0.1)", c[5], rho[b][1], rho_stage)
    H(f"matched rho {b} bits: rho(1)", c[6], rho[b][2], rho_stage)
    H(f"arm (a) full IC {b} bits: S/rho at w=0", c[7], A[b]["ratio"][0], [0.01])
    H(f"arm (a) full IC {b} bits: S/rho at w=0.1" + (" (headline 'about 6.5x')" if b == 16 else
                                                      " (headline 'about 30x')" if b == 28 else ""),
      c[8], A[b]["ratio"][1], [0.01])
    H(f"arm (a) full IC {b} bits: S/rho at w=1", c[9], A[b]["ratio"][2], r1_stage)
H("fitted growth of log2(S/rho) on log2 p, arm (a), 4 design cells (x = cell label 16..28, w = 0)",
  0.179, growth["0"], [0.001], "reproduces only at w = 0 with x = cell label; see limitations")
H("arm (a) S scales like p^e (OLS, x = cell label, w = 0)", 0.60, S_exp["0"], [0.01])
H("matched rho scales like p^e (OLS, x = cell label, w = 0)", 0.42, rho_exp["0"], [0.01])
H("arm (a) T_dec exponent on log2 p (actual primes), design cells", 0.2359, Ta_exp[0], [0.0001])
H("arm (a) T_dec exponent 95% t lower bound (2 df)", 0.2223, Ta_exp[1], [0.0001])
H("arm (a) T_dec exponent 95% t upper bound (2 df)", 0.2496, Ta_exp[2], [0.0001])
H("arm (a) full IC 16-bit ladder bx=3: S(0)", 15489, A_ladder[3]["S"][0], [0.1, 1])

# Arm (b) full IC table.
claims_b = {3: (2431, 1.368e8, 1.368e9, 4.29, 2.27e5, 1.47e6), 4: (1353, 1.439e9, 1.439e10, 2.39, 2.39e6, 1.55e7)}
for bx, c in claims_b.items():
    H(f"arm (b) full IC 16 bits bx={bx}: S(0)", c[0], B_full[bx]["S"][0], [0.1, 1])
    H(f"arm (b) full IC 16 bits bx={bx}: S(0.1)", c[1], B_full[bx]["S"][1], [0.1, 10 ** (math.floor(math.log10(c[1])) - 3)])
    H(f"arm (b) full IC 16 bits bx={bx}: S(1)", c[2], B_full[bx]["S"][2], [10 ** (math.floor(math.log10(c[2])) - 3)])
    H(f"arm (b) full IC 16 bits bx={bx}: S/rho at w=0", c[3], B_full[bx]["ratio"][0], [0.01])
    H(f"arm (b) full IC 16 bits bx={bx}: S/rho at w=0.1", c[4], B_full[bx]["ratio"][1],
      [0.01, 10 ** (math.floor(math.log10(c[4])) - 2)])
    H(f"arm (b) full IC 16 bits bx={bx}: S/rho at w=1", c[5], B_full[bx]["ratio"][2],
      [10 ** (math.floor(math.log10(c[5])) - 3), 10 ** (math.floor(math.log10(c[5])) - 2)])

# MODEL rows (lower bounds, README "Arm (b) (own F4)").
H("MODEL arm (b) oracle phase at bx=8, w=0.1 (attempts x madds x 0.1)", 4.28e11, 0.1 * model_oracle, [1e7, 1e9])
H("MODEL arm (b) oracle phase at bx=8, w=1", 4.28e12, model_oracle, [1e8, 1e10])
H("MODEL arm (b) / rho at w=0.1", 7.1e8, 0.1 * model_oracle / rho[16][1], [1e7])
H("MODEL arm (b) / rho at w=1", 4.6e9, model_oracle / rho[16][2], [1e8])

# Solves: 16 of 16 per cell (exact).
for b in DESIGN:
    H(f"arm (a) full IC solves at {b} bits (of 16)", 16, A[b]["ok"], [])
    H(f"matched rho logarithms recovered at {b} bits (of 16)", 16, rho_ok[b], [])
for bx in (3, 4):
    H(f"arm (b) full IC solves at 16 bits bx={bx} (of 16)", 16, B_full[bx]["ok"], [])

failed = [h for h in headline
          if abs(h["claimed"] - h["recomputed"]) > h["rel_tol"] * max(abs(h["claimed"]), abs(h["recomputed"]), 1e-300)]
all_checks_ok = all(c["ok"] for c in checks)
status = "pass" if not failed and all_checks_ok else "fail"

# ---------------------------------------------------------------------------------------------
# 4. Write the report and the rho descriptor draft.
# ---------------------------------------------------------------------------------------------
for name in ("README.md", "PROTOCOL.md"):
    read_files[ITEM / name] = "claims" if name == "README.md" else "frozen definitions and cell table"
read_files[RAW / "summary.md"] = ("rounding-chain reference only (print precision of the values the README "
                                  "copied); no recomputed value is taken from it")

head = subprocess.run(["git", "-C", str(REPO), "rev-parse", "HEAD"], capture_output=True, text=True).stdout.strip()
# Every file read must be committed and unmodified at HEAD (the committed snapshot, not a working tree).
not_committed = []
for p in read_files:
    rp = str(p.resolve().relative_to(REPO))
    tracked = subprocess.run(["git", "-C", str(REPO), "ls-files", "--error-unmatch", "--", rp],
                             capture_output=True).returncode == 0
    clean = subprocess.run(["git", "-C", str(REPO), "diff", "--quiet", "HEAD", "--", rp],
                           capture_output=True).returncode == 0
    same_as_start = subprocess.run(["git", "-C", str(REPO), "diff", "--quiet", SNAPSHOT_START, "HEAD", "--", rp],
                                   capture_output=True).returncode == 0
    if not (tracked and clean and same_as_start):
        not_committed.append(rp)
check("every file read is tracked and unmodified at HEAD", not not_committed,
      f"{len(read_files)} files; not committed or modified: {not_committed}")
untracked_in_item = subprocess.run(["git", "-C", str(REPO), "status", "--porcelain", "--untracked-files=normal", "--",
                                    "research/blindspots_20261018/14-prime-m4-cells"],
                                   capture_output=True, text=True).stdout.split("\n")
untracked_in_item = sorted({ln[3:].split("/replay/")[0] + "/replay/" if "/replay/" in ln else ln[3:]
                            for ln in untracked_in_item if ln.strip()})
all_checks_ok = all(c["ok"] for c in checks)
status = "pass" if not failed and all_checks_ok else "fail"

discrepancies = [
    {"what": "arm (b) |F|-exponent definition",
     "detail": (f"README 11.031 [11.009, 11.053] averages each ladder cell over all 18 targets including the 2 "
                f"planted controls (recomputed {slope18[0]:.4f} [{slope18[1]:.4f}, {slope18[2]:.4f}]). PROTOCOL "
                f"'T_dec = mean over the cell's 16 random targets' gives {slope16[0]:.4f} "
                f"[{slope16[1]:.4f}, {slope16[2]:.4f}]. Same at 2 decimals (11.03); the bx=3 cell mean is "
                f"{Bm[(16, 3)]['random16']:.4e} (random) vs {Bm[(16, 3)]['all18']:.4e} (README 5.48e5). "
                "No verdict changes (threshold slope 1)."),
     "material": False},
    {"what": "S(0.1) at 20 bits is double-rounded",
     "detail": (f"exact {A[20]['S'][1]:.4f}; summary.md prints 18138.5 and the README rounds that to 18139; a "
                "single rounding gives 18138. Within the README's documented rounding chain (0.05 + 0.5)."),
     "material": False},
    {"what": "'about 6.5x' (commit subject '6.5x-30x', task paraphrase) at w = 0.1",
     "detail": (f"measured 16-bit S/rho is {A[16]['ratio'][0]:.4f} (w=0), {A[16]['ratio'][1]:.4f} (w=0.1), "
                f"{A[16]['ratio'][2]:.4f} (w=1). '6.5' is not a rounding of the w=0.1 value (6.6). The README "
                "table value 6.63 is used as the claim."),
     "material": False},
    {"what": "arm (b) full-IC S omits the S5 interpolation",
     "detail": (f"in `fullic --arm b` S5 is built once per process outside every run's counted phases "
                f"(setup.build = {B_full[3]['setup_build']} in all 32 arm-(b) full-IC records), while the "
                f"tdec-b setup records count {sorted(s5_build)} build operations for it, and PROTOCOL's S covers "
                "'table or S5' cold. Priced at w, this understates arm (b) S(0.1) and S(1) by "
                f"{sorted(s5_build)[0] / B_full[3]['S'][2] * 100:.2f}% at bx=3 and "
                f"{sorted(s5_build)[0] / B_full[4]['S'][2] * 100:.2f}% at bx=4 (resultant evaluations are "
                "uncounted by design). Arm (a) and rho are unaffected; no ranking changes. Not disclosed in the README."),
     "material": False},
    {"what": "planted-target dimensions",
     "detail": ("README: 'dimension 0 for random targets, 24 or 12 for planted'. At 16 bits bx=8, planted target 16 "
                "has dimension 48 = brute-force root count 48 (a second decomposition). The control (dimension equals "
                "the brute-force count, planted point vanishes) holds; only the README wording is incomplete."),
     "material": False},
    {"what": "no benchmarks/README.md row",
     "detail": (f"at HEAD {head} benchmarks/README.md has no row for 20261018-blindspots-prime-m4-cells; the "
                "assess commit bde5b137c7 added only REPORT.pdf, graph.svg and graph.png (byte-identical to the "
                "research copies). The 'benchmarks row' claims checked are the commit subject's (slope 11.03; "
                "6.5x-30x)."),
     "material": False},
]
limitations = [
    ("Derived fits use mixed conventions: the README's 0.179 growth, p^0.60 and p^0.42 reproduce with x = the cell "
     f"label (16, 20, 24, 28) at w = 0 only. log2(S/rho) growth on the cell label is {growth['0']:.4f} / "
     f"{growth['0.1']:.4f} / {growth['1']:.4f} at w = 0 / 0.1 / 1; on log2 of the actual primes it is "
     f"{growth_actual_p['0']:.4f} / {growth_actual_p['0.1']:.4f} / {growth_actual_p['1']:.4f}. S exponent on "
     f"actual primes {S_exp_p['0']:.3f}, rho {rho_exp_p['0']:.3f} at w = 0. The summarizer's arm-(a) T_dec fit "
     "uses actual primes. 'The gap grows with p' holds under every convention."),
    ("The matched rho is a plain r-adding walk over points. The x-keyed DP table catches +-matches only at "
     "distinguished points; it is not a negation-map walk. The strongest reference for these curves "
     "(j != 0, 1728) is negation (expected about 1/sqrt(2) of the walk steps, heuristic). A stronger rho could "
     "only lower rho's cost and widen the measured gap, so the order 'rho is cheaper' is unaffected; the ratios "
     "are understated."),
    ("DP parameter theta = floor(bits/4) uses the cell label (4, 5, 6, 7 bits), as PROTOCOL's 'log2 p' convention "
     "does; with log2 of the actual prime it would be one bit smaller at every cell."),
    ("The rho binary has no recorded sha256 (deleted by the card's remove-bin step). Binary identity rests on the "
     "receipt: one build of target/blindspots-14/card-01/b14 from the bound source at a clean commit, then every rho "
     "and arm-(a) run invokes that path. Arm (b) full IC (cards 09-12) used separate rustc builds of the same source "
     "and flags, not the same binary file as the rho."),
    ("One curve per size, 16 runs per cell; rho means have about 13% standard error at 16 runs. Full-IC targets are "
     "a correlated walk (Deviation 11). Toy scale (16-28 bits); arm (b)'s p-exponent assumes |F| = p^(1/4) and "
     "extrapolates beyond |F| = 6..16."),
    ("Validation, not replay: no command was re-executed. This report is the gate's validator report, not its "
     "replay certificate. A passed report means the receipts are admissible evidence; it supports no ECDLP or "
     "speedup claim."),
]

rho_tag = {b: [round(x, 6) for x in rho[b]] for b in DESIGN}
report = {
    "schema": "validation-gate/v1 validator-report",
    "id": "VAL-20261022-e873ba",
    "status": status,
    "independent": True,
    "independence": ("Validator subagent; did not produce item 14. Recomputed every value with its own stdlib "
                     "Python from the raw JSONL. The producer's instrument source was read only for field semantics "
                     "and print formats; the instrument was not run and no number was taken from summary.md."),
    "subject": {"item": "research/blindspots_20261018/14-prime-m4-cells",
                "evidence_directory": "benchmarks/20261018-blindspots-prime-m4-cells",
                "snapshot_validated_from": SNAPSHOT_START,
                "repository_head": head,
                "files_read_committed_and_clean_at_head": not not_committed,
                "files_read_identical_at_snapshot_and_head": not not_committed,
                "uncommitted_paths_in_item_tree_not_read": untracked_in_item,
                "uncommitted_paths_note": ("untracked or modified paths present in the item tree at validation time "
                                           "(another lane's replay output); none was read by this validation"),
                "instrument_sha256": INSTRUMENT_SHA,
                "card_commits": {f"card-{i:02d}": receipts[i]["commit"] for i in receipts}},
    "script": {"path": "validator_recompute.py", "run": "python3 -I benchmarks/20261018-blindspots-prime-m4-cells/validator_recompute.py"},
    "tolerance_policy": ("rel_tol = sum of half-units of each rounding stage between the exact value and the "
                         "README's printed digits, divided by |claimed|. Stages are the summarize print format of "
                         "that column (README: 'Numbers are those printed by summarize') followed by the README's "
                         "own precision. Integer counts use rel_tol 0. Gate comparison: "
                         "|claimed - recomputed| <= rel_tol * max(|claimed|, |recomputed|)."),
    "raw_data": [{"path": rel(p), "sha256": sha(p), "role": role} for p, role in sorted(read_files.items())],
    "headline": headline,
    "headline_failures": [h["name"] for h in failed],
    "artifact_and_control_checks": checks,
    "arm_b_controls": b_controls,
    "discrepancies": discrepancies,
    "limitations": limitations,
    "validation_report": {
        "id": "VAL-20261022-e873ba",
        "task_id": "validation-gate item 14 (blind-spot 20261018), no Conductor task card supplied",
        "run_ids": [f"CARD-BLINDSPOT-14-{i:02d}" for i in range(1, 13)],
        "verdict": "passed" if status == "pass" else "failed",
        "artifact_paths": ["validator-report.json", "rho-reference.json", "validator_recompute.py"],
    },
    "review_attestation": {
        "joints_owned": ["headline recompute from committed raw data (validator report)",
                         "matched rho reference descriptor (draft)"],
        "sources_read": sorted({rel(p) for p in read_files} | {
            "../../docs/VALIDATION_GATE.md", "../../scripts/validation_gate.py",
            "/home/user/crypto-autoresearcher/docs/agent-runtime-core.md",
            "/home/user/crypto-autoresearcher/agents/validator.md"}),
        "read_sibling_reports": False,
        "sibling_note": ("the replay lane's outputs under research/.../14-prime-m4-cells/replay/ appeared during this "
                         "validation and were not read"),
        "verdict": "holds" if status == "pass" else "breaks",
    },
}
(HERE / "validator-report.json").write_text(json.dumps(report, indent=2) + "\n")

L = 28
ratio = [A[L]["S"][i] / rho[L][i] for i in range(3)]
rho_ref = {
    "schema": "validation-gate/v1 rho_reference (DRAFT; not VALIDATION.json)",
    "draft": True,
    "scope": ("arm (a) full index calculus vs the matched rho, the four design cells (16, 20, 24, 28 bits); "
              "gae_band at the largest measured cell, 28 bits, bx = 63. Arm (b) full-IC rows are not covered: "
              "they ran in separate builds (cards 09-12), so same_binary_or_process would be false for them."),
    "kind": "no_symmetry_used",
    "kind_note": ("Deliberately outside the gate's enum, and the gate will refuse it. The measured reference is plain "
                  "Pollard rho: a 16-jump r-adding walk over points, partition by a hash of x, step P -> P + T_j. P and "
                  "-P diverge after one step, so the negation map is not used. The x-keyed distinguished-point table "
                  "only detects +-matches between stored DPs. 'negation' would misdescribe that walk. "
                  "'no_symmetry_available' would be false: a, b != 0 at every cell, so j != 0, 1728 and Aut(E) = {+-1}. "
                  "Frobenius is trivial on E(F_p)."),
    "strongest_available_kind": "negation",
    "protocol": ("PROTOCOL.md 'Unit, accounting, pricing', Matched rho: Pollard rho with a 16-jump r-adding walk "
                 "(jump scalars with four random bits set, built from the doubling chains of G and Q, counted), van "
                 "Oorschot-Wiener distinguished points theta = floor(log2 p / 4) with log2 p the cell label (4, 5, 6, "
                 "7 bits), one thread, 16 independent logarithms per cell, each logarithm verified by a counted scalar "
                 "multiplication, jump-table setup counted (included in gae) and reported apart (setup_gadd)."),
    "unit": ("GAE: one counted affine group addition (doubling and P + (-P) included) = 1. Plus w x native operations "
             "(lookup, insert, legendre, sqrt, rowop, madd, dptest, build), w in {0, 0.1, 1}. The same counters and "
             "the same native set are used for both arms (recomputed: rho native = lookup + insert + dptest; arm (a) "
             "native = lookup + insert + legendre + sqrt + rowop)."),
    "binary_or_process": ("target/blindspots-14/card-01/b14: one rustc build (--edition 2021 -C opt-level=3 -C "
                          "codegen-units=1 -C target-cpu=native) of examples/blindspots_14_prime_m4.rs, sha256 "
                          f"{INSTRUMENT_SHA}, at commit {receipts[1]['commit']} with dirty_before empty. Card "
                          "CARD-BLINDSPOT-14-01 (card sha256 " + receipts[1]["card_sha256"] + "; receipt "
                          "raw/receipt-card-01.json sha256 " + sha(RAW / "receipt-card-01.json") + "). Command "
                          "indices: build 2; rho-b16..b28 6-9; fullic-a-b16..b28 19-22; remove-bin 28. Same binary "
                          "file, separate processes; the binary itself has no recorded sha256."),
    "matches_protocol": True,
    "strongest_available": False,
    "same_unit": True,
    "same_binary_or_process": bool(same_binary),
    "evidence": {
        "matches_protocol": ("rho-b{16,20,24,28}.jsonl: 16 runs each (run 0..15), ok 16/16, gae == counts.gadd, which "
                             "includes setup_gadd (counts snapshotted before make_jumps); instrument rho_run: J = 16, "
                             "four distinct bits per alpha and beta, theta = (bits/4).max(2), verification by "
                             "cv.mul(cand, g) inside the counted window; cmd_rho draws a distinct d and walk seed per run."),
        "strongest_available": "false: negation is available (Aut(E) = {+-1}) and the walk does not use it; see kind_note.",
        "same_unit": "both arms are counted by the same instrument counters; native totals recompute from the same eight counters.",
        "same_binary_or_process": "receipt-card-01.json command list (see binary_or_process); holds for arm (a) only.",
    },
    "gae_band": {k: {"ic": A[L]["S"][i], "rho": rho[L][i]} for i, k in enumerate(WKEY)},
    "gae_band_cell": {"bits": L, "p": P[L], "bx": DESIGN[L], "runs_ic": 16, "runs_rho": 16,
                      "ic_over_rho": {k: ratio[i] for i, k in enumerate(WKEY)},
                      "definition": "ic = mean(gae_total) + w * mean(native_total) over 16 full-IC runs; "
                                    "rho = mean(gae) + w * mean(native) over 16 rho runs"},
    "order_holds_across_band": all(A[L]["S"][i] > rho[L][i] for i in range(3)) and all(
        A[b]["S"][i] > rho[b][i] for b in DESIGN for i in range(3)),
    "order_note": ("rho is cheaper than arm (a) full IC at w = 0, 0.1 and 1 at every design cell (ratios 6.45 to 30.88). "
                   "A negation-map rho could only lower rho's cost, so the order would still hold."),
}
(HERE / "rho-reference.json").write_text(json.dumps(rho_ref, indent=2) + "\n")

print(f"status {status}; headlines {len(headline)}, failures {len(failed)}; checks ok {all_checks_ok}")
for h in failed:
    print("FAIL", h["name"], h["claimed"], h["recomputed"], h["rel_tol"])
for c in checks:
    if not c["ok"]:
        print("CHECK FAIL", c["check"], c["detail"][:300])
