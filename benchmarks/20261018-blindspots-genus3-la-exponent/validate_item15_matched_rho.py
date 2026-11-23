#!/usr/bin/env python3
"""Independent validator recompute for blindspots item 15 (genus-3 LA exponent) and its matched rho.

Standard library only. Reads committed raw data; never edits it.

    python3 -I benchmarks/20261018-blindspots-genus3-la-exponent/validate_item15_matched_rho.py          # print
    python3 -I benchmarks/20261018-blindspots-genus3-la-exponent/validate_item15_matched_rho.py --write  # also write
        rho-reference.json and validator-report.json beside this script

What it checks (each check is recorded with its result in the written JSON):

1. Receipts of cards m00..m03: card sha256, exit codes, output digests, stdout/stderr digests, the
   instrument pin, the git order (protocol addendum M committed before the card commit and before
   every run commit), GATE OK in each gate file.
2. Continuity, independently of the binary's `gate`: every IC record in raw/matched/b20_c<c>_matched.jsonl
   equals the recorded raw/b20_c<c>_r2.jsonl on every common field except wall times and vmhwm_kb.
3. Rho: 16 correct walks per curve, k == planted k, target 0 == the IC k_true, no mod-ell work in the
   counted CNT, fruitless-cycle escapes counted, round 4's target-0 records reproduced (curves 0-2),
   canon/rho_neg equal round 4's text after dropping `nat(` lines, j-invariant recomputed from
   orders.csv (j not in F_q, not 0, not 1728), ell prime, c_add = 60 M_q + 1 I_q = 81.
4. Section 5a band: S(w) = counted_fq / c_add + w * native_ops for every IC arm and every rho walk,
   recomputed from the raw counters; the frozen addendum statistic (geometric mean over curves of
   S_IC / mean_16 S_rho, hierarchical bootstrap) and the pooled statistic (mean IC S over 4 curves /
   mean rho S over 64 walks); order verdict per arm.
5. The README's old c_ell view (against round 4's single rho walk, curves 0-2, and against the matched
   rho mean, curves 0-3).
6. The item's LA headline from raw/b{16..20}_c{0..3}_r2.jsonl against raw/analysis.json and the README.
"""
from __future__ import annotations

import hashlib
import json
import math
import random
import re
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parent.parent
ITEM_REL = "research/blindspots_20261018/15-genus3-la-exponent"
ITEM = REPO / ITEM_REL
RAW = ITEM / "raw"
MATCHED = RAW / "matched"
CARDS = ITEM / "matched-rho-cards"
INSTR_REL = "research/blindspots_20261018/instruments/blindspots_15b_genus3_la_rho.rs"
INSTR_SHA = "79a821504c5b7a4639470d8e04dfd35fc6ad1a91d34bacee466461f36811e991"
R4_SRC_REL = "examples/ic_improve_04_genus3_2lp.rs"
R4_RHO_REL = "research/ic_improve_20261014/04-genus3-2lp/results/run-20261014"
ADDENDUM_REL = f"{ITEM_REL}/PROTOCOL-ADDENDUM-matched-rho.md"
ADDENDUM_COMMIT = "7555a42a6c"
ARMS = [("a", "r2:lpsgebw2@8"), ("d", "r2:lpsgebw2*"), ("b", "r2:lpsgebw4x4@8"), ("c", "r2:lpsgebl4@8")]
GATE_TAG = {"r2:lpsgebw2@8": "w2at8", "r2:lpsgebw2*": "w2star", "r2:lpsgebw4x4@8": "w4x4at8",
            "r2:lpsgebl4@8": "l4at8"}
BAND = (0.0, 0.1, 1.0)
BAND_KEYS = ("0", "0.1", "1")
EXEMPT = {"la_wall", "wall_stream", "build_wall", "vmhwm_kb"}
CURVES = (0, 1, 2, 3)
BOOT = 2000
SEED = 20261022
T975 = {3: 3.182446305284263, 63: 1.998340522}  # Student t, two-sided 95 %
# Addendum M, continuity control 2: round 4's rho target 0 at b = 20 (cost, steps)
R4_TARGET0 = {0: (18866263020, 232712993), 1: (23539588479, 290359854), 2: (17781900829, 219336676)}
# Claimed numbers as printed in the README (4 decimals)
README_LA = {"d_la_ratio_b20": 0.9862, "d_whole_ratio_b20": 0.9949, "b_la_ratio_b20": 1.4758,
             "c_la_ratio_b20": 1.7833, "a_la_exponent": 1.3846, "d_la_exponent": 1.3787,
             "b_la_exponent": 1.3850, "c_la_exponent": 1.4019, "d_minus_a_exponent": -0.0059}
README_OLD_VIEW = {"d": (0.4573, 0.4835, 0.7197), "b": (0.4573, 0.4966, 0.8502), "c": (0.4573, 0.5048, 0.9321)}

READ: dict[str, str] = {}
CHECKS: list[dict] = []


def sha256_bytes(b: bytes) -> str:
    return hashlib.sha256(b).hexdigest()


def rd(path: Path) -> bytes:
    data = path.read_bytes()
    READ[str(path.relative_to(REPO))] = sha256_bytes(data)
    return data


def rd_json(path: Path):
    return json.loads(rd(path))


def rd_jsonl_one(path: Path):
    lines = [ln for ln in rd(path).decode().splitlines() if ln.strip()]
    assert len(lines) == 1, f"{path}: expected one record, got {len(lines)}"
    return json.loads(lines[0])


def check(name: str, ok: bool, detail="") -> bool:
    CHECKS.append({"check": name, "ok": bool(ok), "detail": detail})
    return ok


def git(*args: str) -> str:
    return subprocess.run(["git", "-C", str(REPO), *args], check=True, capture_output=True,
                          text=True).stdout


def git_ok(*args: str) -> bool:
    return subprocess.run(["git", "-C", str(REPO), *args], capture_output=True).returncode == 0


def bitlen(x: int) -> int:
    return x.bit_length()


def cost_units(c, q: int, cl: float) -> float:
    return c[0] + bitlen(q - 1) * c[1] + cl * c[2]


# ------------------------------------------------------------------ 1. receipts
def card_binds_and_argv(text: str):
    binds = dict(re.findall(r"\{path: ([^,]+), sha256: ([0-9a-f]{64})\}", text))
    argvs = {}
    cur = None
    for line in text.splitlines():
        m = re.match(r"\s*- id: (\S+)", line)
        if m:
            cur = m.group(1)
        m = re.match(r"\s*argv: (\[.*\])\s*$", line)
        if m and cur:
            argvs[cur] = json.loads(m.group(1))
    return binds, argvs


def check_receipts() -> dict:
    out = {}
    instr_bytes = rd(REPO / INSTR_REL)
    check("instrument sha256 equals the pin", sha256_bytes(instr_bytes) == INSTR_SHA, INSTR_SHA)
    hist = git("log", "--format=%H", "--", INSTR_REL).split()
    check("instrument has exactly one commit (7555a42a6c, unchanged since)",
          len(hist) == 1 and hist[0].startswith(ADDENDUM_COMMIT), hist)
    add_hist = git("log", "--format=%H", "--", ADDENDUM_REL).split()
    check("addendum M has exactly one commit (7555a42a6c, unchanged since)",
          len(add_hist) == 1 and add_hist[0].startswith(ADDENDUM_COMMIT), add_hist)
    rd(REPO / ADDENDUM_REL)
    run_commits = set()
    for c in CURVES:
        card_p = CARDS / f"card-m0{c}.yaml"
        rec_p = MATCHED / f"receipt-card-m0{c}.json"
        card_text = rd(card_p).decode()
        card_sha = READ[str(card_p.relative_to(REPO))]
        rec = rd_json(rec_p)
        binds, argvs = card_binds_and_argv(card_text)
        r = {"card": str(card_p.relative_to(REPO)), "card_sha256": card_sha,
             "receipt": str(rec_p.relative_to(REPO)), "receipt_sha256": READ[str(rec_p.relative_to(REPO))],
             "commit": rec.get("commit"), "status": rec.get("status")}
        check(f"m0{c}: receipt card_sha256 equals the card file", rec.get("card_sha256") == card_sha)
        check(f"m0{c}: receipt status completed", rec.get("status") == "completed")
        check(f"m0{c}: dirty_before empty", not rec.get("dirty_before"))
        check(f"m0{c}: card binds the instrument at the pinned sha256", binds.get(INSTR_REL) == INSTR_SHA)
        for path, sha in binds.items():
            data = rd(REPO / path)
            check(f"m0{c}: bound file {path} hashes to its pin", sha256_bytes(data) == sha)
        blob = subprocess.run(["git", "-C", str(REPO), "show", f"{rec['commit']}:{INSTR_REL}"],
                              capture_output=True, check=True).stdout
        check(f"m0{c}: instrument at the receipt commit equals the pin", sha256_bytes(blob) == INSTR_SHA)
        check(f"m0{c}: addendum commit is an ancestor of the receipt commit",
              git_ok("merge-base", "--is-ancestor", ADDENDUM_COMMIT, rec["commit"]))
        check(f"m0{c}: commands_total equals commands run",
              rec.get("commands_total") == len(rec.get("commands", [])) == 9)
        exits = {}
        for cmd in rec["commands"]:
            exits[cmd["id"]] = cmd["exit_code"]
            check(f"m0{c}: {cmd['id']} exit 0, succeeded, not timed out",
                  cmd["exit_code"] == 0 and cmd["succeeded"] and not cmd["timed_out"])
            check(f"m0{c}: {cmd['id']} argv equals the card", cmd["argv"] == argvs.get(cmd["id"]))
            for path, sha in (cmd.get("outputs") or {}).items():
                data = rd(REPO / path)
                check(f"m0{c}: output {Path(path).name} digest", sha256_bytes(data) == sha, sha)
        by_id = {cmd["id"]: cmd for cmd in rec["commands"]}
        run = by_id[f"run-b20-c{c}-matched"]
        r["run_seconds"] = run["seconds"]
        for stream in ("stdout", "stderr"):
            data = rd(MATCHED / f"run-c{c}.{stream}.log")
            check(f"m0{c}: run {stream} log digest", sha256_bytes(data) == run[f"{stream}_sha256"])
            data = rd(MATCHED / f"build-c{c}.{stream}.log")
            check(f"m0{c}: build {stream} log digest", sha256_bytes(data) == by_id["build"][f"{stream}_sha256"])
        for arm, tag in GATE_TAG.items():
            gp = MATCHED / f"gate-c{c}-{tag}.txt"
            text = rd(gp).decode()
            check(f"m0{c}: gate {arm} stdout digest", sha256_bytes(text.encode()) == by_id[f"gate-{tag}"]["stdout_sha256"])
            check(f"m0{c}: gate {arm} prints GATE OK",
                  any(ln.startswith("GATE OK") for ln in text.splitlines()) and "FAILED" not in text, text.strip())
        jsonl_rel = f"{ITEM_REL}/raw/matched/b20_c{c}_matched.jsonl"
        added = git("log", "--diff-filter=A", "--format=%H", "--", jsonl_rel).split()
        check(f"m0{c}: matched jsonl added in exactly one commit", len(added) == 1, added)
        if added:
            run_commits.add(added[0])
            check(f"m0{c}: addendum commit is an ancestor of the run commit {added[0][:10]}",
                  git_ok("merge-base", "--is-ancestor", ADDENDUM_COMMIT, added[0]))
            check(f"m0{c}: card commit {rec['commit'][:10]} is an ancestor of the run commit",
                  git_ok("merge-base", "--is-ancestor", rec["commit"], added[0]))
        r["exits"] = exits
        r["run_commit"] = added[0] if added else None
        out[f"m0{c}"] = r
    for path in [f"{ITEM_REL}/raw/matched"]:
        dirty = git("status", "--porcelain", "--", path).strip()
        check("raw/matched has no uncommitted change", dirty == "", dirty)
    return out


# ------------------------------------------------------------------ 2. continuity
def deep_eq(a, b) -> bool:
    if isinstance(a, float) or isinstance(b, float):
        return a == b
    return a == b


def check_continuity(matched: dict, recorded: dict) -> dict:
    out = {}
    for c in CURVES:
        m, r = matched[c], recorded[c]
        top_fields = [k for k in r if k != "results" and k not in EXEMPT]
        top_bad = [k for k in top_fields if not deep_eq(m.get(k), r[k])]
        check(f"c{c}: instance fields equal (q, seed, N, ell, h, a, b, ...)", not top_bad, top_bad)
        per_arm = {}
        for _, arm in ARMS:
            rm = [x for x in m["results"] if x.get("arm") == arm]
            rr = [x for x in r["results"] if x.get("arm") == arm]
            ok = len(rm) == 1 and len(rr) == 1
            bad, n = [], 0
            if ok:
                for k, v in rr[0].items():
                    if k in EXEMPT:
                        continue
                    n += 1
                    if not deep_eq(rm[0].get(k), v):
                        bad.append(k)
            check(f"c{c}: {arm} matched record equals recorded r2 on every non-wall field", ok and not bad,
                  {"fields_compared": n, "mismatched": bad})
            per_arm[arm] = {"fields_compared": n, "mismatched": bad}
        out[f"c{c}"] = per_arm
    return out


# ------------------------------------------------------------------ 3. rho, symmetry, unit
def is_prime(n: int) -> bool:
    if n < 2:
        return False
    for p in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        if n % p == 0:
            return n == p
    d, s = n - 1, 0
    while d % 2 == 0:
        d //= 2
        s += 1
    for a in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        x = pow(a, d, n)
        if x in (1, n - 1):
            continue
        for _ in range(s - 1):
            x = x * x % n
            if x == n - 1:
                break
        else:
            return False
    return True


class Fq3:
    """F_q[x]/(x^3 - c1 x - c0), the instrument's field (irreducible f = x^3 - c1 x - c0)."""

    def __init__(self, p, c1, c0):
        self.p, self.c1, self.c0 = p, c1, c0

    def mul(self, a, b):
        p = self.p
        d = [0] * 5
        for i in range(3):
            for j in range(3):
                d[i + j] += a[i] * b[j]
        for k in (4, 3):  # x^k = x^(k-3) (c1 x + c0)
            t = d[k] % p
            d[k] = 0
            d[k - 2] += t * self.c1
            d[k - 3] += t * self.c0
        return [d[0] % p, d[1] % p, d[2] % p]

    def add(self, a, b):
        return [(x + y) % self.p for x, y in zip(a, b)]

    def smul(self, a, c):
        return [x * c % self.p for x in a]

    def pow(self, a, e):
        r, b = [1, 0, 0], a
        while e:
            if e & 1:
                r = self.mul(r, b)
            b = self.mul(b, b)
            e >>= 1
        return r

    def inv(self, a):
        return self.pow(a, self.p ** 3 - 2)


def j_from_orders(q: int, kmod, n_order: int):
    c1, c0 = kmod
    rows = []
    for line in rd(ITEM / "orders.csv").decode().splitlines():
        f = [int(x) for x in line.split(",")]
        if f[0] == q and f[1] == c1 and f[2] == c0 and f[9] == n_order:
            rows.append(f)
    if len(rows) != 1:
        return None, len(rows)
    f = rows[0]
    k = Fq3(q, c1, c0)
    a, b = f[3:6], f[6:9]
    a3 = k.mul(k.mul(a, a), a)
    num = k.smul(a3, 4 * 1728 % q)
    den = k.add(k.smul(a3, 4), k.smul(k.mul(b, b), 27))
    if den == [0, 0, 0]:
        return None, 1
    j = k.mul(num, k.inv(den))
    jq = k.pow(j, q)
    return {"ea": a, "eb": b, "j": j, "j_frob": jq}, 1


def extract_fn(text: str, name: str) -> list[str]:
    out, on = [], False
    for line in text.splitlines():
        if line.startswith(f"fn {name}("):
            on = True
        if on:
            out.append(line)
            if line == "}":
                break
    return out


def check_rho(matched: dict, recorded: dict) -> dict:
    out = {}
    src15b = rd(REPO / INSTR_REL).decode()
    src_r4 = rd(REPO / R4_SRC_REL).decode()
    for fn in ("canon", "rho_neg"):
        a = [ln for ln in extract_fn(src15b, fn) if "nat(" not in ln]
        b = extract_fn(src_r4, fn)
        check(f"{fn}: 15b text minus `nat(` lines equals round 4's text", a == b and len(b) > 5,
              {"lines_15b_after_drop": len(a), "lines_round4": len(b)})
    for c in CURVES:
        m = matched[c]
        q = m["q"]
        ic_k = {x["arm"]: x["k_true"] for x in m["results"] if x.get("arm", "").startswith("r2:")}
        rho = sorted([x for x in m["results"] if x.get("arm") == "rho"], key=lambda x: x["target"])
        check(f"c{c}: 16 rho walks, targets 0..15", [x["target"] for x in rho] == list(range(16)))
        check(f"c{c}: every rho walk returns the planted k",
              all(x["correct"] is True and x["k"] == x["k_true"] for x in rho))
        check(f"c{c}: every IC arm returns the planted k",
              all(x["correct"] is True and x["k"] == x["k_true"] for x in m["results"] if x["arm"].startswith("r2:")))
        check(f"c{c}: rho target 0 k equals the IC k_true of every arm",
              len(set(ic_k.values())) == 1 and rho[0]["k_true"] == list(ic_k.values())[0],
              {"rho_t0_k": rho[0]["k_true"], "ic_k_true": sorted(set(ic_k.values()))})
        check(f"c{c}: rho counted CNT has no mod-ell products or inversions",
              all(x["counters"][2] == 0 and x["counters"][3] == 0 for x in rho))
        check(f"c{c}: rho native counters 0-5 positive, IC counters 6-12 zero",
              all(all(v > 0 for v in x["native"][:6]) and all(v == 0 for v in x["native"][6:]) for x in rho))
        check(f"c{c}: rho cost recomputes from counters (c_ell = 1, unit M_q + ceil(log2 q) I_q)",
              all(x["cost"] == cost_units(x["counters"], q, 1) for x in rho))
        check(f"c{c}: rho additions counted (CNT[4]) >= steps", all(x["counters"][4] >= x["steps"] for x in rho))
        check(f"c{c}: fruitless-cycle escapes occur and are counted", all(x["escapes"] > 0 for x in rho),
              sum(x["escapes"] for x in rho))
        if c in R4_TARGET0:
            r4 = rd_jsonl_one(REPO / R4_RHO_REL / f"b20_c{c}_rho.jsonl")
            r4t0 = [x for x in r4["results"] if x.get("arm") == "rho"][0]
            check(f"c{c}: rho target 0 reproduces round 4 (cost, steps, k)",
                  (rho[0]["cost"], rho[0]["steps"]) == R4_TARGET0[c] == (r4t0["cost"], r4t0["steps"])
                  and rho[0]["k"] == r4t0["k"],
                  {"matched": [rho[0]["cost"], rho[0]["steps"]], "round4": [r4t0["cost"], r4t0["steps"]]})
        ca = m["c_add"]
        check(f"c{c}: c_add = 60 M_q + 1 I_q = 81 at ceil(log2 q) = {bitlen(q - 1)}",
              ca["mean"] == ca["min"] == ca["max"] == 60 + bitlen(q - 1) and ca["additions"] == 64, ca)
        sym = m["symmetry"]
        jj, nrows = j_from_orders(q, m["Kmod"], m["N"])
        ok_j = jj is not None and jj["j"] == sym["j"]
        check(f"c{c}: j(E0) recomputed from orders.csv equals the recorded j", ok_j,
              {"orders_rows": nrows, "recomputed": jj and jj["j"], "recorded": sym["j"]})
        generic = (jj is not None and (jj["j"][1], jj["j"][2]) != (0, 0) and jj["j_frob"] != jj["j"]
                   and jj["j"] != [0, 0, 0] and jj["j"] != [1728 % q, 0, 0])
        check(f"c{c}: j not in F_q, not Frobenius-fixed, not 0, not 1728 (negation is the strongest symmetry)",
              generic and sym["strongest_rho_symmetry"] == "negation" and not sym["j_in_Fq"]
              and not sym["j_is_0"] and not sym["j_is_1728"] and not sym["subfield_twist"])
        check(f"c{c}: N = c * ell, ell prime", m["N"] == m["c"] * m["ell"] and is_prime(m["ell"]),
              {"N": m["N"], "c": m["c"], "ell": m["ell"]})
        out[f"c{c}"] = {"j": sym["j"], "ea": jj and jj["ea"], "eb": jj and jj["eb"], "c_add": ca["mean"],
                        "ell": m["ell"], "cofactor": m["c"],
                        "rho_escapes_total": sum(x["escapes"] for x in rho),
                        "rho_steps_total": sum(x["steps"] for x in rho),
                        "rho_steps_over_model_mean": sum(x["steps_over_model"] for x in rho) / 16,
                        "rho_native_per_step": sum(sum(x["native"]) for x in rho) / sum(x["steps"] for x in rho)}
    return out


# ------------------------------------------------------------------ 4. section 5a band
def ic_gae(x: dict, q: int, c_add: float):
    tot = x["total_counters"]
    counted = tot[0] + bitlen(q - 1) * tot[1]
    native = sum(x["rel_native"]) + sum(x["la_native"]) + x["m_ell"] + x["i_ell"]
    return counted, native, tot


def rho_gae(x: dict, q: int):
    counted = x["counters"][0] + bitlen(q - 1) * x["counters"][1]
    native = sum(x["native"])
    return counted, native


def pct(v, p):
    w = sorted(v)
    return w[int(round((len(w) - 1) * p))]


def band_analysis(matched: dict) -> dict:
    data = {}
    for c in CURVES:
        m = matched[c]
        q, c_add = m["q"], float(m["c_add"]["mean"])
        rho = [x for x in m["results"] if x.get("arm") == "rho" and x["correct"]]
        rho_rows = []
        for x in rho:
            cf, nat = rho_gae(x, q)
            g = x["gae"]
            check(f"c{c} rho t{x['target']}: binary gae fields recompute",
                  g["counted_fq"] == cf and g["native_ops"] == nat
                  and all(abs(g["S"][k] - (cf / c_add + w * nat)) <= 1e-9 * g["S"][k] for k, w in zip(BAND_KEYS, BAND)))
            rho_rows.append({"counted_fq": cf, "native": nat, "steps": x["steps"], "escapes": x["escapes"],
                             "S": [cf / c_add + w * nat for w in BAND], "cost_fq": cf,
                             "model_fq": x["rho_model_neg"]})
        ic_rows = {}
        for role, arm in ARMS:
            x = [y for y in m["results"] if y.get("arm") == arm][0]
            cf, nat, tot = ic_gae(x, q, c_add)
            check(f"c{c} {arm}: counted_fq = cost(c_ell=1) - m_ell = rel_cost + LA M_q/I_q",
                  cf == x["cost"] - tot[2] and x["m_ell"] == tot[2] and x["i_ell"] == tot[3])
            g = x["gae"]
            check(f"c{c} {arm}: binary gae fields recompute",
                  g["counted_fq"] == cf and g["native_ops"] == nat
                  and all(abs(g["S"][k] - (cf / c_add + w * nat)) <= 1e-9 * g["S"][k] for k, w in zip(BAND_KEYS, BAND)))
            natc = sum(x["rel_native"]) + sum(x["la_native"])
            ic_rows[role] = {"counted_fq": cf, "native": nat, "m_ell": x["m_ell"], "i_ell": x["i_ell"],
                             "nat_counters": natc,
                             "S": [cf / c_add + w * nat for w in BAND], "cost_cl1": x["cost"],
                             # sensitivity B (not the frozen rule): M_ell counted at c_ell = 1 F_q-mult-eq, the
                             # PROTOCOL's primary unit; I_ell and the NAT counters priced at w GAE
                             "SB": [(cf + x["m_ell"]) / c_add + w * (natc + x["i_ell"]) for w in BAND]}
        rho_mean = [sum(r["S"][i] for r in rho_rows) / len(rho_rows) for i in range(3)]
        mr = m["matched_rho"]
        check(f"c{c}: binary rho_S_gae_mean recomputes",
              all(abs(mr["rho_S_gae_mean"][k] - rho_mean[i]) <= 1e-9 * rho_mean[i] for i, k in enumerate(BAND_KEYS)))
        for role, arm in ARMS:
            ratio = [ic_rows[role]["S"][i] / rho_mean[i] for i in range(3)]
            check(f"c{c} {arm}: binary ratio_ic_over_rho recomputes",
                  all(abs(mr["arms"][arm]["ratio_ic_over_rho"][k] - ratio[i]) <= 1e-9 * ratio[i]
                      for i, k in enumerate(BAND_KEYS)))
        data[c] = {"q": q, "c_add": c_add, "rho": rho_rows, "ic": ic_rows, "rho_mean": rho_mean,
                   "binary_matched": mr, "rho_records": rho}
        check(f"c{c}: at least 12 correct rho walks (addendum usability rule)", len(rho_rows) >= 12, len(rho_rows))

    def stats(curves, walks, key="S"):
        """curves: list of curve ids (with repetition); walks: per slot, the list of walk indices."""
        res = {}
        for role, _ in ARMS:
            geo, pooled = [], []
            for i in range(3):
                lr, ic_sum, n_ic, rho_sum, n_rho = [], 0.0, 0, 0.0, 0
                for slot, c in enumerate(curves):
                    rows = data[c]["rho"]
                    idx = walks[slot]
                    rm = sum(rows[j]["S"][i] for j in idx) / len(idx)
                    s_ic = data[c]["ic"][role][key][i]
                    lr.append(math.log(s_ic / rm))
                    ic_sum += s_ic
                    n_ic += 1
                    rho_sum += sum(rows[j]["S"][i] for j in idx)
                    n_rho += len(idx)
                geo.append(math.exp(sum(lr) / len(lr)))
                pooled.append((ic_sum / n_ic) / (rho_sum / n_rho))
            res[role] = (geo, pooled)
        return res

    ident_curves = list(CURVES)
    ident_walks = [list(range(len(data[c]["rho"]))) for c in CURVES]
    point = stats(ident_curves, ident_walks)
    point_b = stats(ident_curves, ident_walks, "SB")
    rng = random.Random(SEED)
    boots, boots_b = [], []
    for _ in range(BOOT):
        cs = [rng.randrange(4) for _ in range(4)]
        ws = [[rng.randrange(len(data[c]["rho"])) for _ in range(len(data[c]["rho"]))] for c in cs]
        boots.append(stats(cs, ws))
        boots_b.append(stats(cs, ws, "SB"))

    arms_out = {}
    for role, arm in ARMS:
        ic_mean = [sum(data[c]["ic"][role]["S"][i] for c in CURVES) / 4 for i in range(3)]
        rho_all = [r for c in CURVES for r in data[c]["rho"]]
        rho_mean = [sum(r["S"][i] for r in rho_all) / len(rho_all) for i in range(3)]
        n = len(rho_all)
        entry = {"arm": arm, "role": role, "band": {}, "per_curve": {}}
        for i, k in enumerate(BAND_KEYS):
            geo_b = [b[role][0][i] for b in boots]
            pool_b = [b[role][1][i] for b in boots]
            lr = [math.log(data[c]["ic"][role]["S"][i] / data[c]["rho_mean"][i]) for c in CURVES]
            mlr = sum(lr) / 4
            sd = math.sqrt(sum((x - mlr) ** 2 for x in lr) / 3)
            tg = [math.exp(mlr - T975[3] * sd / 2), math.exp(mlr + T975[3] * sd / 2)]
            vals = [r["S"][i] for r in rho_all]
            sdr = math.sqrt(sum((x - rho_mean[i]) ** 2 for x in vals) / (n - 1))
            se = sdr / math.sqrt(n)
            tp = [ic_mean[i] / (rho_mean[i] + T975[63] * se), ic_mean[i] / (rho_mean[i] - T975[63] * se)]
            entry["band"][k] = {
                "ic_S_mean_gae": ic_mean[i], "rho_S_mean_gae": rho_mean[i],
                "pooled_ratio": point[role][1][i], "pooled_ratio_boot95": [pct(pool_b, 0.025), pct(pool_b, 0.975)],
                "pooled_ratio_t95_rho_mean_only": tp,
                "geomean_ratio_frozen": point[role][0][i], "geomean_ratio_boot95": [pct(geo_b, 0.025), pct(geo_b, 0.975)],
                "geomean_ratio_t95_over_curves_df3": tg,
                "rho_S_sd_gae": sdr, "rho_S_mean_se_gae": se,
            }
        for c in CURVES:
            entry["per_curve"][f"c{c}"] = {
                "ic_S_gae": dict(zip(BAND_KEYS, data[c]["ic"][role]["S"])),
                "rho_S_mean_gae": dict(zip(BAND_KEYS, data[c]["rho_mean"])),
                "ratio": dict(zip(BAND_KEYS, [data[c]["ic"][role]["S"][i] / data[c]["rho_mean"][i] for i in range(3)])),
                "ic_counted_fq": data[c]["ic"][role]["counted_fq"], "ic_native_ops": data[c]["ic"][role]["native"],
                "ic_m_ell": data[c]["ic"][role]["m_ell"], "ic_i_ell": data[c]["ic"][role]["i_ell"],
                "ic_nat_counters": data[c]["ic"][role]["nat_counters"],
            }
        b = entry["band"]
        below = all(b[k]["geomean_ratio_frozen"] < 1 and b[k]["geomean_ratio_boot95"][1] < 1 for k in BAND_KEYS)
        above = all(b[k]["geomean_ratio_frozen"] > 1 and b[k]["geomean_ratio_boot95"][0] > 1 for k in BAND_KEYS)
        below_p = all(b[k]["pooled_ratio"] < 1 and b[k]["pooled_ratio_boot95"][1] < 1 for k in BAND_KEYS)
        above_p = all(b[k]["pooled_ratio"] > 1 and b[k]["pooled_ratio_boot95"][0] > 1 for k in BAND_KEYS)
        g_ic = ic_mean[0]
        n_ic = sum(data[c]["ic"][role]["native"] for c in CURVES) / 4
        g_rho = rho_mean[0]
        n_rho = sum(r["native"] for r in rho_all) / n
        wstar = (g_rho - g_ic) / (n_ic - n_rho) if n_ic != n_rho else None
        entry["order"] = {
            "frozen_rule_statistic": "geometric mean over curves of S_IC / mean_16 S_rho, hierarchical bootstrap",
            "ic_below_rho_all_w": below, "ic_above_rho_all_w": above,
            "pooled_ic_below_rho_all_w": below_p, "pooled_ic_above_rho_all_w": above_p,
            "order_holds_across_band": below or above,
            "verdict": ("IC below rho at every w" if below else "IC above rho at every w" if above
                        else "unresolved: order changes inside the band"),
            "pooled_crossover_w_gae_per_native_op": wstar,
            "per_curve_crossover_w": {f"c{c}": (data[c]["rho_mean"][0] - data[c]["ic"][role]["S"][0]) /
                                      (data[c]["ic"][role]["native"] - sum(r["native"] for r in data[c]["rho"]) / len(data[c]["rho"]))
                                      for c in CURVES},
        }
        # Sensitivity: rho's coefficient bookkeeping (two mod-ell additions per step, plus a negation of
        # both on a canonical flip) is not in rho's native counters; charge 3 native ops per step to rho.
        sens = {}
        for i, k in enumerate(BAND_KEYS):
            rs = sum(r["S"][i] + BAND[i] * 3 * r["steps"] for r in rho_all) / n
            sens[k] = ic_mean[i] / rs
        entry["sensitivity_rho_plus_3_native_per_step_pooled_ratio"] = sens
        # Sensitivity B (not the frozen rule): M_ell counted at c_ell = 1 F_q-mult-eq (PROTOCOL primary unit).
        sb = {}
        for i, k in enumerate(BAND_KEYS):
            sb[k] = {"geomean_ratio": point_b[role][0][i],
                     "geomean_ratio_boot95": [pct([bb[role][0][i] for bb in boots_b], 0.025),
                                              pct([bb[role][0][i] for bb in boots_b], 0.975)],
                     "pooled_ratio": point_b[role][1][i]}
        entry["sensitivity_B_m_ell_counted_at_c_ell_1"] = sb
        # Break-even price of one mod-ell product (F_q-mult-eq) at which pooled IC = rho, NAT counters at w GAE.
        m_ell_mean = sum(data[c]["ic"][role]["m_ell"] for c in CURVES) / 4
        natc_ic = sum(data[c]["ic"][role]["nat_counters"] + data[c]["ic"][role]["i_ell"] for c in CURVES) / 4
        c_add_m = sum(data[c]["c_add"] for c in CURVES) / 4
        entry["break_even_fq_mult_per_m_ell"] = {
            k: c_add_m * (g_rho - g_ic + BAND[i] * (n_rho - natc_ic)) / m_ell_mean for i, k in enumerate(BAND_KEYS)}
        # Sensitivity: a rho that met the sqrt(pi ell / 4) model on average (S scaled by model / measured).
        arms_out[role] = entry
    # best IC arm: lowest mean S over the band (all arms tie at w = 0 by construction)
    best = min((r for r, _ in ARMS),
               key=lambda r: (arms_out[r]["band"]["1"]["ic_S_mean_gae"], arms_out[r]["band"]["0.1"]["ic_S_mean_gae"]))
    rho_model = sum(r["model_fq"] for c in CURVES for r in data[c]["rho"]) / sum(len(data[c]["rho"]) for c in CURVES)
    rho_meas = sum(r["cost_fq"] for c in CURVES for r in data[c]["rho"]) / sum(len(data[c]["rho"]) for c in CURVES)
    som = [x["steps_over_model"] for c in CURVES for x in band_matched_rho_rows(data, c)]
    mu = sum(som) / len(som)
    sd = math.sqrt(sum((v - mu) ** 2 for v in som) / (len(som) - 1))
    esc = sum(r["escapes"] for c in CURVES for r in data[c]["rho"])
    steps = sum(r["steps"] for c in CURVES for r in data[c]["rho"])
    for role, _ in ARMS:
        arms_out[role]["sensitivity_model_efficient_rho_pooled_ratio"] = {
            k: arms_out[role]["band"][k]["pooled_ratio"] * rho_meas / rho_model for k in BAND_KEYS}
    return {"arms": arms_out, "best_arm": best, "data": data,
            "rho_model_secondary": {"mean_model_fq": rho_model, "mean_measured_counted_fq": rho_meas,
                                    "measured_over_model": rho_meas / rho_model,
                                    "steps_over_model_mean": mu, "steps_over_model_t95": [mu - T975[63] * sd / 8, mu + T975[63] * sd / 8],
                                    "steps_per_fruitless_escape": steps / esc,
                                    "note": ("round 4's walk checks for fruitless cycles only every L = 64 steps, so a walk "
                                             "caught in a 2-cycle bounces until the next saved point; with an escape every "
                                             "~164 steps this plausibly accounts for most of the excess over the model")}}


def band_matched_rho_rows(data, c):
    return data[c]["rho_records"]


# ------------------------------------------------------------------ 5. the README's old c_ell view
def old_view(matched: dict, recorded: dict, analysis: dict) -> dict:
    out = {"vs_round4_single_rho_curves_0_2": {}, "vs_matched_rho_mean_curves_0_3": {}}
    r4 = {c: [x for x in rd_jsonl_one(REPO / R4_RHO_REL / f"b20_c{c}_rho.jsonl")["results"]
              if x.get("arm") == "rho" and x.get("correct")][0]["cost"] for c in (0, 1, 2)}
    roles = {r["role"]: r for r in analysis["roles"]}
    for role, arm in ARMS:
        vals = []
        for i, cl in enumerate(BAND):
            lr = []
            for c in (0, 1, 2):
                x = [y for y in recorded[c]["results"] if y["arm"] == arm][0]
                lr.append(math.log(cost_units(x["total_counters"], recorded[c]["q"], cl) / r4[c]))
            vals.append(math.exp(sum(lr) / 3))
        claimed = None
        if role in roles:
            claimed = [g["whole_over_measured_rho_geomean"] for g in roles[role]["rho_gap_top"]]
        out["vs_round4_single_rho_curves_0_2"][role] = {"recomputed": dict(zip(BAND_KEYS, vals)),
                                                        "analysis_json": claimed and dict(zip(BAND_KEYS, claimed)),
                                                        "readme": README_OLD_VIEW.get(role)}
        if claimed:
            check(f"old view {role}: analysis.json rho_gap_top recomputes",
                  all(abs(a - b) <= 1e-9 * b for a, b in zip(vals, claimed)))
        vals2, per_curve = [], {}
        for i, cl in enumerate(BAND):
            lr = []
            for c in CURVES:
                m = matched[c]
                x = [y for y in m["results"] if y["arm"] == arm][0]
                cf = x["total_counters"][0] + bitlen(m["q"] - 1) * x["total_counters"][1]
                rho = [y for y in m["results"] if y.get("arm") == "rho" and y["correct"]]
                rmean = sum(y["counters"][0] + bitlen(m["q"] - 1) * y["counters"][1] for y in rho) / len(rho)
                v = (cf + cl * x["m_ell"]) / rmean
                per_curve.setdefault(f"c{c}", {})[BAND_KEYS[i]] = v
                bv = m["matched_rho"]["arms"][arm]["descriptive_c_ell_ratio_fq"][BAND_KEYS[i]]
                check(f"old view vs matched rho c{c} {arm} c_ell={cl}: binary descriptive ratio recomputes",
                      abs(v - bv) <= 1e-9 * bv)
                lr.append(math.log(v))
            vals2.append(math.exp(sum(lr) / 4))
        out["vs_matched_rho_mean_curves_0_3"][role] = {"geomean": dict(zip(BAND_KEYS, vals2)), "per_curve": per_curve}
    out["round4_rho_cost_fq"] = r4
    return out


# ------------------------------------------------------------------ 6. LA headline
def la_headline(analysis: dict) -> dict:
    pts = {}
    qs = {}
    for b in range(16, 21):
        for c in CURVES:
            rec = rd_jsonl_one(RAW / f"b{b}_c{c}_r2.jsonl")
            qs[b] = rec["q"]
            for role, arm_base in [("a", "lpsgebw2@"), ("d", "lpsgebw2*"), ("b", "lpsgebw4x4@"), ("c", "lpsgebl4@")]:
                xs_ = [x for x in rec["results"] if x["arm"].startswith("r2:" + arm_base)
                       and "+" not in x["arm"] and (role != "a" or "!" not in x["arm"])]
                assert len(xs_) == 1, (b, c, role, [x["arm"] for x in xs_])
                x = xs_[0]
                pts[(role, b, c)] = {"la": cost_units(x["la_counters"], rec["q"], 1),
                                     "whole": cost_units(x["total_counters"], rec["q"], 1),
                                     "correct": x["correct"], "arm": x["arm"]}
    check("LA headline: every arm a, b, c, d correct on every (b, curve), b = 16..20",
          all(p["correct"] for p in pts.values()) and len(pts) == 80)

    def ols(x, y):
        mx, my = sum(x) / len(x), sum(y) / len(y)
        return sum((a - mx) * (b - my) for a, b in zip(x, y)) / sum((a - mx) ** 2 for a in x)

    def slope(role, metric, cells=None):
        x, y = [], []
        for b in range(16, 21):
            for c in (cells[b] if cells else CURVES):
                x.append(math.log(qs[b]))
                y.append(math.log(pts[(role, b, c)][metric]))
        return ols(x, y)

    def ratio(role, metric, b, cells=None):
        cs = cells if cells is not None else list(CURVES)
        return math.exp(sum(math.log(pts[(role, b, c)][metric] / pts[("a", b, c)][metric]) for c in cs) / len(cs))

    roles = {r["role"]: r for r in analysis["roles"]}
    out = {}
    rng = random.Random(SEED + 1)
    for role in ("a", "d", "b", "c"):
        e = {"la_exponent": slope(role, "la"), "whole_exponent": slope(role, "whole")}
        if role != "a":
            e["la_ratio_b20"] = ratio(role, "la", 20)
            e["whole_ratio_b20"] = ratio(role, "whole", 20)
            e["la_ratio_by_size"] = {b: ratio(role, "la", b) for b in range(16, 21)}
            e["delta_la_exponent"] = e["la_exponent"] - slope("a", "la")
            boot_r, boot_d = [], []
            for _ in range(BOOT):
                cs = [rng.randrange(4) for _ in range(4)]
                boot_r.append(ratio(role, "la", 20, cs))
                cells = {b: [rng.randrange(4) for _ in range(4)] for b in range(16, 21)}
                boot_d.append(slope(role, "la", cells) - slope("a", "la", cells))
            e["la_ratio_b20_boot95"] = [pct(boot_r, 0.025), pct(boot_r, 0.975)]
            e["delta_la_exponent_boot95"] = [pct(boot_d, 0.025), pct(boot_d, 0.975)]
            s = roles[role]["sections"]
            e["analysis_json"] = {"la_ratio_top": s["la"]["ratio_top"], "la_ratio_top_ci": s["la"]["ratio_top_ci"],
                                  "whole_ratio_top": s["whole"]["ratio_top"],
                                  "slope_role": s["la"]["slope_role"], "slope_a": s["la"]["slope_a"],
                                  "delta": s["la"]["delta"], "delta_ci": s["la"]["delta_ci"],
                                  "verdict": roles[role]["verdict"]}
        out[role] = e
    out["d"]["la_saving_b20_percent"] = 100 * (1 - out["d"]["la_ratio_b20"])
    return out


# ------------------------------------------------------------------ main
def main(argv) -> int:
    write = "--write" in argv
    receipts = check_receipts()
    matched = {c: rd_jsonl_one(MATCHED / f"b20_c{c}_matched.jsonl") for c in CURVES}
    recorded = {c: rd_jsonl_one(RAW / f"b20_c{c}_r2.jsonl") for c in CURVES}
    analysis = rd_json(RAW / "analysis.json")
    cont = check_continuity(matched, recorded)
    rho = check_rho(matched, recorded)
    band = band_analysis(matched)
    old = old_view(matched, recorded, analysis)
    la = la_headline(analysis)

    best = band["best_arm"]
    arms = band["arms"]
    print(f"checks: {sum(c['ok'] for c in CHECKS)} ok / {len(CHECKS)}")
    for c in CHECKS:
        if not c["ok"]:
            print("FAILED:", c["check"], json.dumps(c["detail"])[:300])
    print(f"best IC arm under the band: {best} ({arms[best]['arm']})")
    for role, arm in ARMS:
        e = arms[role]
        print(f"{role} {arm}: {e['order']['verdict']}; pooled crossover w* = {e['order']['pooled_crossover_w_gae_per_native_op']:.4g}")
        for k in BAND_KEYS:
            b = e["band"][k]
            print(f"   w={k:>3}: IC S {b['ic_S_mean_gae']:.6g}  rho S {b['rho_S_mean_gae']:.6g}  pooled "
                  f"{b['pooled_ratio']:.4f} boot {b['pooled_ratio_boot95'][0]:.4f}-{b['pooled_ratio_boot95'][1]:.4f}  "
                  f"geo {b['geomean_ratio_frozen']:.4f} boot {b['geomean_ratio_boot95'][0]:.4f}-{b['geomean_ratio_boot95'][1]:.4f} "
                  f"t {b['geomean_ratio_t95_over_curves_df3'][0]:.4f}-{b['geomean_ratio_t95_over_curves_df3'][1]:.4f}")
    print("old c_ell view vs round-4 single rho (curves 0-2):",
          {r: {k: round(v, 4) for k, v in x["recomputed"].items()} for r, x in old["vs_round4_single_rho_curves_0_2"].items()})
    print("old c_ell view vs matched rho mean (curves 0-3):",
          {r: {k: round(v, 4) for k, v in x["geomean"].items()} for r, x in old["vs_matched_rho_mean_curves_0_3"].items()})
    print("LA headline d:", {k: v for k, v in la["d"].items() if k != "analysis_json"})
    print("rho model (secondary):", band["rho_model_secondary"])
    for role, arm in ARMS:
        e = arms[role]
        print(f"{role} sensitivities: model-efficient rho pooled",
              {k: round(v, 4) for k, v in e["sensitivity_model_efficient_rho_pooled_ratio"].items()},
              "| rho +3 nat/step pooled", {k: round(v, 4) for k, v in e["sensitivity_rho_plus_3_native_per_step_pooled_ratio"].items()},
              "| B (M_ell at c_ell=1) geo", {k: (round(v["geomean_ratio"], 4), [round(x, 4) for x in v["geomean_ratio_boot95"]])
                                             for k, v in e["sensitivity_B_m_ell_counted_at_c_ell_1"].items()},
              "| break-even F_q-mult per M_ell", {k: round(v, 3) for k, v in e["break_even_fq_mult_per_m_ell"].items()})
    if write:
        write_outputs(matched, receipts, cont, rho, band, old, la)
    return 0 if all(c["ok"] for c in CHECKS) else 1


def rel_to_evidence(repo_rel: str) -> str:
    return str(Path("../..") / repo_rel)


def write_outputs(matched, receipts, cont, rho, band, old, la):
    best = band["best_arm"]
    arms = band["arms"]
    all_ok = all(c["ok"] for c in CHECKS)
    any_holds = {r: arms[r]["order"]["order_holds_across_band"] for r, _ in ARMS}
    script_rel = str(Path(__file__).resolve().relative_to(REPO))
    ref = {
        "schema": "rho-reference/v1",
        "item": "blindspots item 15, genus-3 IC-2LP linear-algebra arms at b = 20 against a matched negation-map Pollard rho",
        "kind": "negation",
        "protocol": ("PROTOCOL.md 'Distance remaining' (round 4's measured negation-map rho), as amended before the runs by "
                     "PROTOCOL-ADDENDUM-matched-rho.md (commit 7555a42a6c): round 4's rho_neg + canon on classes {P,-P} of "
                     "E0(F_{q^3}), r = 64 adding walk, distinguished points, fruitless-cycle escape by doubling, 16 walks per curve "
                     "(target 0 = the IC target k), b = 20, curves 0-3, decision rule 'Decision rule (frozen)' of the addendum"),
        "unit": ("GAE: one generic affine E0(F_{q^3}) addition = c_add = 81 F_q-mult-eq (60 M_q + 1 I_q at ceil(log2 q) = 21), "
                 "measured in each process; every arm's counted cost M_q + ceil(log2 q) I_q is divided by c_add and every native "
                 "op (13 NAT counters, plus m_ell and i_ell for IC per the frozen PROTOCOL classification) is priced at w GAE"),
        "binary_or_process": ("research/blindspots_20261018/instruments/blindspots_15b_genus3_la_rho.rs (sha256 " + INSTR_SHA +
                              "), built per card with rustc -C opt-level=3 -C codegen-units=1; one process per curve runs the four "
                              "IC arms then the 16 rho walks (cards matched-rho-cards/card-m00..m03.yaml, receipts "
                              "raw/matched/receipt-card-m00..m03.json)"),
        "matches_protocol": True,
        "strongest_available": True,
        "same_unit": True,
        "same_binary_or_process": True,
        "gae_band": {k: {"ic": arms[best]["band"][k]["ic_S_mean_gae"], "rho": arms[best]["band"][k]["rho_S_mean_gae"]}
                     for k in BAND_KEYS},
        "gae_band_note": (f"best IC arm = {best} ({arms[best]['arm']}): lowest mean S at w = 0.1 and 1 (all four arms tie at w = 0 "
                          "because the LA phase is entirely mod-ell products, which the frozen classification prices as native). "
                          "ic = mean over curves 0-3 of the arm's S(w) in GAE; rho = mean over the 64 correct walks of S(w) in GAE."),
        "order_holds_across_band": any_holds[best],
        "order_verdict": {r: arms[r]["order"]["verdict"] for r, _ in ARMS},
        "order_rule": ("addendum M: 'IC below rho' only if the aggregate ratio (geometric mean over curves of S_IC / mean_16 S_rho) "
                       "is below 1 at w = 0, 0.1 and 1 with each 95 % bootstrap upper end below 1; 'IC above rho' the mirror; "
                       "otherwise unresolved"),
        "per_arm": {r: {"arm": arms[r]["arm"], "band": arms[r]["band"], "order": arms[r]["order"],
                        "sensitivities_not_the_frozen_rule": {
                            k: arms[r][k] for k in ("sensitivity_rho_plus_3_native_per_step_pooled_ratio",
                                                    "sensitivity_model_efficient_rho_pooled_ratio",
                                                    "sensitivity_B_m_ell_counted_at_c_ell_1",
                                                    "break_even_fq_mult_per_m_ell")},
                        "per_curve": arms[r]["per_curve"]} for r, _ in ARMS},
        "intervals": (f"pooled_ratio = mean IC S over 4 curves / mean rho S over 64 walks; geomean_ratio_frozen = the addendum's "
                      f"statistic. *_boot95: {BOOT} hierarchical resamples (curves with replacement, then walks with replacement "
                      f"within each drawn curve), percentile, random.Random({SEED}); geomean t95: Student t on the 4 per-curve log "
                      "ratios (df 3); pooled t95: Student t on the 64-walk rho mean only (df 63), IC mean held fixed"),
        "rho_model_secondary": band["rho_model_secondary"],
        "evidence": {
            "booleans": {
                "matches_protocol": ("the walk is round 4's: canon and rho_neg equal round 4's text after dropping the nat( counter "
                                     "lines (checked here); dispatch from the stream copy after k; target 0 reproduces round 4's "
                                     "recorded cost and steps exactly on curves 0-2; cells, argv and rule frozen in addendum M, "
                                     "committed (7555a42a6c) before the card commit and every run commit"),
                "strongest_available": ("j(E0) recomputed here from orders.csv for all four curves equals the recorded j; j has "
                                        "non-zero x and x^2 coefficients (not in F_q, so E0 is not a twist of a subfield curve and "
                                        "no Frobenius map acts on E0), j^q != j, j != 0, 1728 (Aut = {+-1}); negation is the "
                                        "strongest available symmetry"),
                "same_unit": ("IC and rho are both counted by CNT[0] (M_q) and CNT[1] (I_q) through the same cost_units at "
                              "ceil(log2 q) = 21 and converted by the same per-process c_add; rho's CNT[2], CNT[3] are 0; every "
                              "native counter is emitted for every arm and priced at the same w"),
                "same_binary_or_process": "one build of the pinned 15b source per card; the IC arms and the rho walks run in the same process",
            },
            "symmetry": {f"c{c}": {"j": rho[f"c{c}"]["j"], "ea": rho[f"c{c}"]["ea"], "eb": rho[f"c{c}"]["eb"]} for c in CURVES},
            "rho_per_curve": {f"c{c}": {k: v for k, v in rho[f"c{c}"].items() if k not in ("j", "ea", "eb")} for c in CURVES},
        },
        "limitations": [
            "Toy scale: q = 2^20 + 7, group order about 2^60, b = 20 only (b = 16..19 not in addendum M).",
            "One IC solve per curve (no IC walk-to-walk variation); intervals are over 4 curves and 16 rho walks per curve.",
            "At w = 0 the LA phase is free (all of it is mod-ell products, priced as native by the frozen classification), so w = 0 is the most IC-favourable point of the band.",
            "Rho's coefficient bookkeeping (two mod-ell additions per step, a negation on a canonical flip) is not in its native counters, while IC's mod-ell additions are; charging 3 native ops per step to rho (sensitivity field) only raises rho and does not change any order.",
            "The matched rho is round 4's walk as frozen, not a tuned one: its mean steps are about 1.33x the sqrt(pi ell / 4) negation-map model over 64 walks (t95 excludes 1), consistent with fruitless 2-cycles being detected only every L = 64 steps (one escape per ~164 steps). This inflates rho and favours IC; a model-efficient rho raises IC/rho at every w (sensitivity field) and leaves the order verdict unchanged.",
            "The order flip is a statement about the price of a mod-ell product: m_ell is about 98 % of each IC arm's native ops. Under the frozen classification (M_ell native, priced at w GAE = 0, 8.1, 81 F_q-mult-eq) the order changes near w = 0.02-0.04. Under the PROTOCOL's primary unit (M_ell counted at c_ell = 1 F_q-mult-eq; I_ell and NAT counters at w GAE; sensitivity B) every arm's point ratio is below 1 at every w, with bootstrap upper ends below 1 for arms a and d at every w, while the w = 0 intervals of arms b and c cross 1. The break-even price of one mod-ell product is about 1.6-2.9 F_q-mult-eq at w = 0 (field break_even_fq_mult_per_m_ell). Which classification governs is an owner decision (addendum M, 'Pricing'); this report applies the frozen one.",
            "Clique removal is not instrumented; it is used only by arm e, which is not in this comparison.",
            "Single-threaded counted operations on a shared host; no wall-time claim.",
        ],
        "validator_script": {"path": script_rel, "sha256": sha256_bytes((REPO / script_rel).read_bytes())},
    }
    (HERE / "rho-reference.json").write_text(json.dumps(ref, indent=1) + "\n")

    heads = []

    def head(name, claimed, recomputed, rel_tol, source):
        heads.append({"name": name, "claimed": claimed, "recomputed": recomputed, "rel_tol": rel_tol, "source": source})

    d = la["d"]
    aj = d["analysis_json"]
    head("d_la_ratio_b20 (analysis.json)", aj["la_ratio_top"], d["la_ratio_b20"], 1e-9, "raw/analysis.json roles[d].sections.la.ratio_top")
    head("d_la_ratio_b20 (README, 4 d.p.)", README_LA["d_la_ratio_b20"], d["la_ratio_b20"], 1e-4, "README table")
    head("d_la_saving_b20_percent (README 'about 1.4 %')", 1.4, d["la_saving_b20_percent"], 0.05, "README 'Primitive: ... 1.4 % lower'")
    head("d_whole_ratio_b20 (analysis.json)", aj["whole_ratio_top"], d["whole_ratio_b20"], 1e-9, "raw/analysis.json roles[d].sections.whole.ratio_top")
    head("d_whole_ratio_b20 (README, 4 d.p.)", README_LA["d_whole_ratio_b20"], d["whole_ratio_b20"], 1e-4, "README table")
    head("d_la_exponent (analysis.json)", aj["slope_role"], d["la_exponent"], 1e-9, "raw/analysis.json roles[d].sections.la.slope_role")
    head("a_la_exponent (analysis.json)", aj["slope_a"], la["a"]["la_exponent"], 1e-9, "raw/analysis.json roles[d].sections.la.slope_a")
    head("d_minus_a_la_exponent (analysis.json)", aj["delta"], d["delta_la_exponent"], 1e-6, "raw/analysis.json roles[d].sections.la.delta")
    head("d_la_exponent (README, 4 d.p.)", README_LA["d_la_exponent"], d["la_exponent"], 1e-4, "README table")
    head("a_la_exponent (README, 4 d.p.)", README_LA["a_la_exponent"], la["a"]["la_exponent"], 1e-4, "README table")
    head("d_minus_a_la_exponent (README, 4 d.p.)", README_LA["d_minus_a_exponent"], d["delta_la_exponent"], 1e-2, "README table (-0.0059, rounding half-unit 0.00005)")
    for role in ("b", "c"):
        e = la[role]
        head(f"{role}_la_ratio_b20 (analysis.json)", e["analysis_json"]["la_ratio_top"], e["la_ratio_b20"], 1e-9, f"raw/analysis.json roles[{role}]")
        head(f"{role}_la_ratio_b20 (README, 4 d.p.)", README_LA[f"{role}_la_ratio_b20"], e["la_ratio_b20"], 1e-4, "README table")
        head(f"{role}_la_exponent (analysis.json)", e["analysis_json"]["slope_role"], e["la_exponent"], 1e-9, f"raw/analysis.json roles[{role}]")
        head(f"{role}_la_exponent (README, 4 d.p.)", README_LA[f"{role}_la_exponent"], e["la_exponent"], 1e-4, "README table")
    for role, x in old["vs_round4_single_rho_curves_0_2"].items():
        if x["analysis_json"]:
            for k in BAND_KEYS:
                head(f"old_c_ell_view_{role}_cell{k} (analysis.json rho_gap_top; lower-bound comparison)",
                     x["analysis_json"][k], x["recomputed"][k], 1e-9, f"raw/analysis.json roles[{role}].rho_gap_top")
        if x["readme"]:
            for i, k in enumerate(BAND_KEYS):
                head(f"old_c_ell_view_{role}_cell{k} (README, 4 d.p.; lower-bound comparison)",
                     x["readme"][i], x["recomputed"][k], 2e-4, "README 'Distance to the matched rho' table")
    for c in (0, 1, 2):
        t0 = [x for x in matched[c]["results"] if x.get("arm") == "rho" and x["target"] == 0][0]
        head(f"rho_target0_cost_c{c} (addendum control 2, round 4's record)", R4_TARGET0[c][0], t0["cost"], 1e-12,
             f"PROTOCOL-ADDENDUM-matched-rho.md control 2; raw/matched/b20_c{c}_matched.jsonl")
        head(f"rho_target0_steps_c{c} (addendum control 2, round 4's record)", R4_TARGET0[c][1], t0["steps"], 1e-12,
             f"PROTOCOL-ADDENDUM-matched-rho.md control 2; raw/matched/b20_c{c}_matched.jsonl")
    for c in CURVES:
        mr = band["data"][c]["binary_matched"]
        for i, k in enumerate(BAND_KEYS):
            head(f"matched_rho_c{c}_rho_S_gae_mean_w{k} (binary)", mr["rho_S_gae_mean"][k], band["data"][c]["rho_mean"][i], 1e-9,
                 f"raw/matched/b20_c{c}_matched.jsonl matched_rho.rho_S_gae_mean")
        for role, arm in ARMS:
            for i, k in enumerate(BAND_KEYS):
                head(f"matched_rho_c{c}_{role}_ratio_w{k} (binary)", mr["arms"][arm]["ratio_ic_over_rho"][k],
                     band["data"][c]["ic"][role]["S"][i] / band["data"][c]["rho_mean"][i], 1e-9,
                     f"raw/matched/b20_c{c}_matched.jsonl matched_rho.arms[{arm}].ratio_ic_over_rho")
    for c in CURVES:
        head(f"c_add_c{c} (addendum: 81 at b = 20; code: 60 M_q + 1 I_q)", 81, band["data"][c]["c_add"], 1e-12,
             f"raw/matched/b20_c{c}_matched.jsonl c_add.mean")

    status_ok = all_ok and all(abs(h["claimed"] - h["recomputed"]) <= h["rel_tol"] * max(abs(h["claimed"]), abs(h["recomputed"]), 1e-300) for h in heads)
    raw_data = [{"path": rel_to_evidence(p), "sha256": s} for p, s in sorted(READ.items())]
    rep = {
        "schema": "validator-report/v1",
        "status": "pass" if status_ok else "fail",
        "independent": True,
        "independence_basis": ("validator subagent that did not produce item 15, its matched-rho cards, runs or replay; every number "
                               "recomputed by this stdlib script from the committed raw records (the binary's own gate, analyze and "
                               "matched_rho outputs are compared against, never used as inputs to the recompute)"),
        "snapshot": {"head": git("rev-parse", "HEAD").strip(),
                     "evidence_paths_clean": git("status", "--porcelain", "--", ITEM_REL, INSTR_REL, R4_SRC_REL,
                                                 R4_RHO_REL).strip() == "",
                     "evidence_paths": [ITEM_REL, INSTR_REL, R4_SRC_REL, R4_RHO_REL]},
        "raw_data": raw_data,
        "headline": heads,
        "findings": {
            "order_vs_matched_rho": {r: arms[r]["order"]["verdict"] for r, _ in ARMS},
            "best_arm_under_band": f"{best} ({arms[best]['arm']})",
            "readme_ic_below_rho_at_every_price": (
                "does not survive section 5a. The README's statement used c_ell pricing (M_ell at 0, 0.1, 1 F_q-mult) against one "
                "unpriced round-4 rho walk per curve (curves 0-2); those numbers recompute exactly and remain a lower-bound "
                "comparison. In that old unit the statement also holds against the matched rho mean over 4 curves x 16 walks "
                "(arm a 0.407 / 0.431 / 0.645). In GAE with every arm's native work priced (rho included), the frozen statistic "
                "puts every IC arm below rho at w = 0 and above rho at w = 0.1 and w = 1; the hierarchical bootstrap intervals "
                "exclude 1 at each w (the df-3 t intervals over curves at w = 0.1 cover 1 for arms a and d). The order changes "
                "inside the band (pooled crossover w about 0.022-0.044 GAE per native op), so the arms are unresolved against "
                "rho and no order between IC and rho is stated."),
            "la_headline": ("arm d's LA cost at b = 20 is 0.98623 of arm a's (1.38 % lower) on the four recorded curves, the "
                            "analysis.json and README numbers recompute from raw/b{16..20}_c{0..3}_r2.jsonl; this is in the c_ell = 1 "
                            "unit of the frozen PROTOCOL. Under the section 5a GAE band arm d is not cheaper than arm a at w > 0 "
                            "(its 16 SGE candidates add native bookkeeping), which the LA verdict does not depend on."),
        },
        "validation_report": {
            "id": "VAL-20261022-15b7e3",
            "task_id": "unassigned (dispatched by the main session for the validation gate; no TASK card)",
            "run_ids": ["CARD-BLINDSPOT-15-M00", "CARD-BLINDSPOT-15-M01", "CARD-BLINDSPOT-15-M02", "CARD-BLINDSPOT-15-M03",
                        "item 15 cards 01-09 (raw/receipt-card-0*.json; LA headline records)"],
            "artifact_checks": [c for c in CHECKS if c["check"].startswith(("m0", "instrument", "addendum", "raw/matched"))],
            "metric_recomputations": [c for c in CHECKS if not c["check"].startswith(("m0", "instrument", "addendum", "raw/matched"))],
            "control_checks": {"continuity_ic_rows": cont,
                               "rho_target0_vs_round4": "curves 0-2 reproduce cost and steps exactly (see headline)",
                               "planted_k": "all 64 rho walks and all 16 IC arm solves return the planted k",
                               "symmetry": "negation on every cell (j recomputed independently)"},
            "heuristic_validation_checks": ["rho model sqrt(pi ell / 4) * cost_per_step shown as a secondary column only: "
                                            f"measured mean / model mean = {band['rho_model_secondary']['measured_over_model']:.4f} over 64 walks; not used to rank"],
            "cost_model_checks": ["unit declared (GAE = 81 F_q-mult-eq, measured per process)",
                                  "every arm's native work listed and priced at w in {0, 0.1, 1} GAE, rho included (section 5a)",
                                  "IC m_ell and i_ell classified as native per the frozen PROTOCOL (unchanged by addendum M)",
                                  "memory: vmhwm_kb about 0.6 GB per process (descriptive)",
                                  "each rho walk is run to success; no success-probability scaling is needed (every walk correct)"],
            "proof_architecture_checks": [],
            "verdict": "passed" if status_ok else "failed",
            "limitations": ref["limitations"] + [
                "LA headline intervals are paired bootstraps over 4 curves on identical matrices; they do not include walk-to-walk variation (the aa null spread is about 1 %, the size of arm d's gain).",
                "The README's rho distance table is a pre-section-5a lower-bound comparison and must be cited as such.",
            ],
            "artifact_paths": [rel_to_evidence(p) for p in sorted(READ)] + ["rho-reference.json", Path(script_rel).name],
        },
        "validator_script": {"path": Path(script_rel).name, "sha256": sha256_bytes((REPO / script_rel).read_bytes())},
    }
    (HERE / "validator-report.json").write_text(json.dumps(rep, indent=1) + "\n")
    print("wrote", HERE / "rho-reference.json", "and", HERE / "validator-report.json", "status", rep["status"])


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
