#!/usr/bin/env python3
"""Validator recompute of the pq2_rank census headline counts (Task A).

Written from scratch by the validator. It does not import or copy
log_to_json.py or crosscheck/. It parses results/run.log itself (not the JSON),
recomputes each headline count of README "Results", checks every derived file
(results/pq2_rank.json, .csv; the degree-4 sweep JSON/CSV/log; the rerun
JSON/CSV/log) row by row against its source, pins every file it reads by
SHA-256, and writes validation/validator_report.json.

Run from anywhere:  python3 -I validation/recompute.py
"""
from __future__ import annotations

import csv
import datetime as dt
import hashlib
import json
import re
import statistics
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent          # .../pq2_rank/validation
PKG = HERE.parent                               # .../pq2_rank
REPO = PKG.parents[2]                           # repository root
VALIDATOR = "validator (claude-opus-5-5; Claude Code session 01AXUKfUpk9rEJg4Yyo1QcZQ)"
# The frozen commit validated. Every input is read from the git object store at
# this commit, never from the (moving) working tree; the working-tree copy is
# hashed too and any difference is reported.
COMMIT = "f96f4f4c019bc95084b24d9dcb80a10655200cb3"

READ: dict[str, str] = {}
WORKTREE_DIFFERS: list[str] = []


def rel(p: Path) -> str:
    return p.resolve().relative_to(REPO).as_posix()


def read_text(p: Path) -> str:
    r = rel(p)
    data = subprocess.run(["git", "-C", str(REPO), "show", f"{COMMIT}:{r}"], check=True,
                          capture_output=True).stdout
    h = hashlib.sha256(data).hexdigest()
    READ[r] = h
    if not p.exists() or hashlib.sha256(p.read_bytes()).hexdigest() != h:
        WORKTREE_DIFFERS.append(r)
    return data.decode()


# ---------------------------------------------------------------------------
# Instance-line parser (the validator's own)

LEFT_KEYS = ["n", "n'", "N", "inst", "|Z|", "affdim", "k", "rho", "low2", "low3", "low4", "d_lin"]
TIME_KEYS = ["Z", "deg2", "deg3", "deg4", "iter3", "iter4"]
LINE = re.compile(r"^(?P<left>n=.*?) \| (?P<right>iter3=.*) \((?P<times>[^()]*)\)$")
RIGHT = re.compile(r"^iter3=(\S+) \((\d+) rounds\) iter4=(\S+) \((\d+) rounds\) d_iter=(\S+)$")
PAIR = re.compile(r"(\S+?)=\s*(\S+)")
TIME = re.compile(r"^(\d+) ms (\S+)$")


def opt_int(s: str):
    return None if s == "-" else int(s)


def parse_line(line: str) -> dict | None:
    m = LINE.match(line)
    if not m:
        return None
    pairs = PAIR.findall(m["left"])
    if [k for k, _ in pairs] != LEFT_KEYS:
        raise ValueError(f"unexpected left keys in {line!r}")
    v = dict(pairs)
    r = RIGHT.match(m["right"])
    if not r:
        raise ValueError(f"unexpected right part in {line!r}")
    times = [TIME.match(t.strip()) for t in m["times"].split(",")]
    if any(t is None for t in times) or [t[2] for t in times] != TIME_KEYS:
        raise ValueError(f"unexpected timings in {line!r}")
    row = {
        "n": int(v["n"]), "nprime": int(v["n'"]), "N": int(v["N"]), "inst": int(v["inst"]),
        "Z": int(v["|Z|"]), "affdim": int(v["affdim"]), "k": int(v["k"]), "rho": int(v["rho"]),
        "low2": int(v["low2"]), "low3": int(v["low3"]), "low4": opt_int(v["low4"]), "d_lin": v["d_lin"],
        "iter3": opt_int(r[1]), "iter3_rounds": int(r[2]), "iter4": opt_int(r[3]),
        "iter4_rounds": int(r[4]), "d_iter": r[5],
    }
    row["ms"] = {t[2]: int(t[1]) for t in times}
    return row


VALUE_KEYS = ["n", "nprime", "N", "inst", "Z", "affdim", "k", "rho", "low2", "low3", "low4", "d_lin",
              "iter3", "iter3_rounds", "iter4", "iter4_rounds", "d_iter"]


def parse_log(p: Path):
    header, rows, other = [], [], []
    for raw in read_text(p).splitlines():
        if not raw.strip():
            continue
        r = parse_line(raw)
        if r is not None:
            rows.append(r)
        elif raw.startswith("pq2_rank census;") or raw.startswith("ns=") or raw.startswith("done in"):
            header.append(raw)
        else:
            other.append(raw)
    return header, rows, other


# ---------------------------------------------------------------------------

def main_by_key(rows):
    return {(r["n"], r["inst"]): r for r in rows}


def main() -> int:
    heads: list[dict] = []
    notes: list[str] = []
    detail: dict = {}

    def H(name, claimed, recomputed, **extra):
        item = {"name": name, "claimed": claimed, "recomputed": recomputed, "rel_tol": 0}
        item.update(extra)
        heads.append(item)

    read_text(PKG / "README.md")  # the claims source, pinned

    # ---- Claim 1: main sweep, from run.log -------------------------------
    hdr, L, other = parse_log(PKG / "results/run.log")
    detail["main_log_header"] = hdr
    detail["main_log_unparsed_lines"] = other
    keys = [(r["n"], r["inst"]) for r in L]
    H("main.instance_lines", 119, len(L))
    H("main.unparsed_nonblank_lines", 0, len(other))
    H("main.duplicate_instance_keys", 0, len(keys) - len(set(keys)))
    H("main.n_min", 8, min(r["n"] for r in L))
    H("main.n_max", 47, max(r["n"] for r in L))
    H("main.instances_n47", 2, sum(r["n"] == 47 for r in L))  # "stopped during n = 47"
    H("main.instances_n_ge_48", 0, sum(r["n"] >= 48 for r in L))
    H("main.N_eq_2ceil(n/2)", 119, sum(r["N"] == 2 * r["nprime"] == 2 * ((r["n"] + 1) // 2) for r in L))
    H("main.k_eq_N_minus_affdim", 119, sum(r["k"] == r["N"] - r["affdim"] for r in L))

    def rng(lo, hi):
        return [r for r in L if lo <= r["n"] <= hi]

    a = rng(8, 14)
    H("main.n8_14.instances", 21, len(a))
    H("main.n8_14.iter3_eq_k", 21, sum(r["iter3"] == r["k"] for r in a))
    H("main.n8_14.d_iter_3", 21, sum(r["d_iter"] == "3" for r in a))
    H("main.n8_14.d_lin_4", 20, sum(r["d_lin"] == "4" for r in a))
    H("main.n8_14.d_lin_gt4", 1, sum(r["d_lin"] == ">4" for r in a))
    a = rng(15, 15)
    H("main.n15.instances", 3, len(a))
    H("main.n15.iter3_eq_1_lt_k", 3, sum(r["iter3"] == 1 and r["k"] > 1 for r in a))
    H("main.n15.iter4_eq_k", 3, sum(r["iter4"] == r["k"] for r in a))
    H("main.n15.d_iter_4", 3, sum(r["d_iter"] == "4" for r in a))
    H("main.n15.d_lin_gt4", 3, sum(r["d_lin"] == ">4" for r in a))
    a = rng(16, 16)
    H("main.n16.instances", 3, len(a))
    H("main.n16.iter3_eq_k", 3, sum(r["iter3"] == r["k"] for r in a))
    H("main.n16.d_iter_3", 3, sum(r["d_iter"] == "3" for r in a))
    H("main.n16.d_lin_gt4", 3, sum(r["d_lin"] == ">4" for r in a))
    a = rng(17, 44)
    H("main.n17_44.instances", 84, len(a))
    H("main.n17_44.iter3_eq_1_lt_k", 84, sum(r["iter3"] == 1 and r["k"] > 1 for r in a))
    a = rng(17, 20)
    H("main.n17_20.instances", 12, len(a))
    H("main.n17_20.d_iter_gt3", 12, sum(r["d_iter"] == ">3" for r in a))
    H("main.n17_20.d_lin_gt4", 12, sum(r["d_lin"] == ">4" for r in a))
    a = rng(21, 44)
    H("main.n21_44.instances", 72, len(a))
    H("main.n21_44.d_iter_gt3", 72, sum(r["d_iter"] == ">3" for r in a))
    H("main.n21_44.d_lin_gt3", 72, sum(r["d_lin"] == ">3" for r in a))
    a = rng(45, 47)
    H("main.n45_47.instances", 8, len(a))
    H("main.n45_47.iter3_not_computed", 8, sum(r["iter3"] is None for r in a))
    H("main.n45_47.d_lin_gt3", 8, sum(r["d_lin"] == ">3" for r in a))
    H("main.iter3_eq_k_total(n<=14 and n=16)", 24, sum(r["iter3"] == r["k"] for r in L))
    H("main.low2_eq_1", 119, sum(r["low2"] == 1 for r in L))
    H("main.n_ge_9.instances", 116, sum(r["n"] >= 9 for r in L))
    H("main.n_ge_9.low3_eq_1", 116, sum(r["n"] >= 9 and r["low3"] == 1 for r in L))
    H("main.k_in_[N-7,N]", 119, sum(r["N"] - 7 <= r["k"] <= r["N"] for r in L))
    H("main.low2_lt_k", 119, sum(r["low2"] < r["k"] for r in L))
    H("main.iter4_computed_only_n_le_16", 27, sum(r["iter4"] is not None for r in L),
      note="iter4 present exactly on n <= 16: " + str(all((r["iter4"] is not None) == (r["n"] <= 16) for r in L)))
    detail["main_N_minus_k_range"] = [min(r["N"] - r["k"] for r in L), max(r["N"] - r["k"] for r in L)]

    # README Results, "Reading": "Literal Assumption 2 ... fails on all 119 instances: d_iter >= 3 everywhere."
    # README defines d_iter = least d with iter_d >= k (iter_d = degree-d closure). d_iter >= 3 therefore needs
    # iter2 < k. The producer never computed iter2 (its d_iter label tests degree 2 with the one-shot low2).
    # Established from the producer's artifacts: iter2 <= iter3 < k (closure monotonicity) where iter3 < k.
    by_mono = [r for r in L if r["iter3"] is not None and r["iter3"] < r["k"]]
    # The validator's own degree-2 closure on the replayed main-sweep instances (lines proven equal to run.log
    # in replay_certificate.json): closure_replay_n8_20.json and closure_replay_n21_24.json.
    own = {}
    for f in ("closure_replay_n8_20.json", "closure_replay_n21_24.json"):
        fp = HERE / f
        data = fp.read_bytes()
        READ[rel(fp)] = hashlib.sha256(data).hexdigest()
        for r in json.loads(data)["results"]:
            own[(r["n"], r["instance"])] = r["mine"]
    by_own = [r for r in L if (r["n"], r["inst"]) in own and own[(r["n"], r["inst"])]["iter2"] < r["k"]
              and own[(r["n"], r["inst"])]["k"] == r["k"]]
    est = {(r["n"], r["inst"]) for r in by_mono} | {(r["n"], r["inst"]) for r in by_own}
    open_keys = sorted(k for k in main_by_key(L) if k not in est)
    H("reading.d_iter_ge_3_established(README closure definition)", 119, len(est),
      established_by_iter3_lt_k_monotonicity=len(by_mono),
      established_by_validator_iter2=len({(r["n"], r["inst"]) for r in by_own} - {(r["n"], r["inst"]) for r in by_mono}),
      not_established=open_keys,
      note=("d_iter >= 3 needs the degree-2 closure iter2 < k; the producer computed only the one-shot low2 "
            "(low2 < k on all 119). On the open instances iter3 was not computed either and the producer's own "
            "log labels them d_iter=?; the README table also shows '-' there. The claim holds on 119 only under "
            "the code's convention of testing degree 2 with low2."))
    detail["main_iter3_present_iff_n_le_44"] = all((r["iter3"] is not None) == (r["n"] <= 44) for r in L)
    detail["main_low4_present_iff_n_le_20"] = all((r["low4"] is not None) == (r["n"] <= 20) for r in L)

    # results/pq2_rank.json and .csv against run.log, every field, in order
    J = json.loads(read_text(PKG / "results/pq2_rank.json"))
    jmap = {"n": "n", "n_prime": "nprime", "N": "N", "instance": "inst", "Z": "Z", "affdim": "affdim",
            "k": "k", "rho": "rho", "low2": "low2", "low3": "low3", "low4": "low4", "d_lin": "d_lin",
            "iter3": "iter3", "iter3_rounds": "iter3_rounds", "iter4": "iter4", "iter4_rounds": "iter4_rounds",
            "d_iter": "d_iter"}
    tmap = {"ms_Z": "Z", "ms_deg2": "deg2", "ms_deg3": "deg3", "ms_deg4": "deg4", "ms_iter3": "iter3",
            "ms_iter4": "iter4"}

    def row_eq_json(jr, lr):
        if set(jr) != set(jmap) | set(tmap):
            return False
        return all(jr[k] == lr[v] for k, v in jmap.items()) and all(jr[k] == lr["ms"][v] for k, v in tmap.items())

    H("main.json.rows", 119, len(J["rows"]))
    H("main.json.rows_equal_log_all_fields_in_order", 119,
      sum(row_eq_json(j, l) for j, l in zip(J["rows"], L)))
    H("main.json.log_header_equal", 1, int(J.get("log_header") == hdr))
    C = list(csv.DictReader(read_text(PKG / "results/pq2_rank.csv").splitlines()))

    def csv_val(s):
        if s == "":
            return None
        try:
            return int(s)
        except ValueError:
            return s

    def row_eq_csv(cr, lr):
        conv = {k: csv_val(v) for k, v in cr.items()}
        # d_lin / d_iter are strings in the log ("4", ">4"); compare as strings
        for k in ("d_lin", "d_iter"):
            conv[k] = cr[k]
        return all(conv[k] == lr[v] for k, v in jmap.items()) and all(conv[k] == lr["ms"][v] for k, v in tmap.items())

    H("main.csv.rows", 119, len(C))
    H("main.csv.rows_equal_log_all_fields_in_order", 119, sum(row_eq_csv(c, l) for c, l in zip(C, L)))

    # ---- binary-written JSON/CSV helpers ----------------------------------
    bmap = {"n": "n", "nprime": "nprime", "N": "N", "instance": "inst", "Z_size": "Z", "affdim": "affdim",
            "k": "k", "rho": "rho", "low2": "low2", "low3": "low3", "low4": "low4", "d_lin": "d_lin",
            "iter3": "iter3", "iter3_rounds": "iter3_rounds", "iter4": "iter4", "iter4_rounds": "iter4_rounds",
            "d_iter": "d_iter"}
    btmap = {"ms_z": "Z", "ms_low3": "deg3", "ms_low4": "deg4", "ms_iter3": "iter3", "ms_iter4": "iter4"}

    def bin_json_eq_log(jr, lr):
        return all(jr[k] == lr[v] for k, v in bmap.items()) and all(jr[k] == lr["ms"][v] for k, v in btmap.items())

    def bin_csv_eq_json(cr, jr):
        for k in ["n", "nprime", "N", "instance", "Z_size", "affdim", "k", "rho", "low2", "low3", "low4",
                  "iter3", "iter4", "ms_z", "ms_low3", "ms_low4", "ms_iter3", "ms_iter4", "F_size"]:
            if csv_val(cr[k]) != jr[k]:
                return False
        for k in ["d_lin", "d_iter", "a", "b", "x_R"]:
            if cr[k] != jr[k]:
                return False
        if jr["F_size"] is not None and cr["F_over_V"] != f"{jr['F_size'] / (1 << jr['nprime']):.4f}":
            return False
        return True

    # ---- Claim 2: degree-4 sweeps ------------------------------------------
    R = json.loads(read_text(PKG / "results/rerun_n8_40/pq2_rank.json"))["rows"]  # main-stream params
    main_params = {(r["a"], r["b"], r["x_R"], tuple(r["basis"])) for r in R}
    total, both = 0, 0
    for tag, lo, hi in (("iter4_n17_20", 17, 20), ("iter4_n21_24", 21, 24)):
        read_text(PKG / f"results/{tag}/COMMAND.txt")
        X = json.loads(read_text(PKG / f"results/{tag}/pq2_rank.json"))["rows"]
        XC = list(csv.DictReader(read_text(PKG / f"results/{tag}/pq2_rank.csv").splitlines()))
        good = sum(r["iter3"] is not None and r["iter4"] is not None and r["iter3"] < r["k"] and r["iter4"] == r["k"]
                   for r in X)
        H(f"{tag}.instances", 12, len(X))
        H(f"{tag}.n_range_ok", 12, sum(lo <= r["n"] <= hi for r in X))
        H(f"{tag}.iter3_lt_k_and_iter4_eq_k", 12, good)
        H(f"{tag}.iter3_eq_1", 12, sum(r["iter3"] == 1 for r in X))
        H(f"{tag}.d_iter_4", 12, sum(r["d_iter"] == "4" for r in X))
        H(f"{tag}.csv_rows_equal_json", 12, sum(bin_csv_eq_json(c, j) for c, j in zip(XC, X)) if len(XC) == len(X) else -1)
        H(f"{tag}.instances_shared_with_main_stream", 0,
          sum((r["a"], r["b"], r["x_R"], tuple(r["basis"])) in main_params for r in X))
        total += len(X)
        both += good
        if tag == "iter4_n21_24":
            h2, L2, o2 = parse_log(PKG / f"results/{tag}/run.log")
            H(f"{tag}.log_lines", 12, len(L2))
            H(f"{tag}.json_rows_equal_log_all_fields", 12,
              sum(bin_json_eq_log(j, l) for j, l in zip(X, L2)) if len(L2) == len(X) else -1)
    H("iter4_sweeps.instances", 24, total)
    H("iter4_sweeps.iter3_lt_k_and_iter4_eq_k", 24, both)

    # ---- Claim 3: same-seed rerun reproduces run.log -----------------------
    hr, LR, orr = parse_log(PKG / "results/rerun_n8_40/run.log")
    main_by = {(r["n"], r["inst"]): r for r in L}
    rer_by = {(r["n"], r["inst"]): r for r in LR}
    shared = sorted(set(main_by) & set(rer_by))
    mism = [k for k in shared if any(main_by[k][f] != rer_by[k][f] for f in VALUE_KEYS)]
    H("rerun.instance_lines", 99, len(LR))
    H("rerun.shared_instance_lines_compared", 99, len(shared))
    H("rerun.mismatching_lines_non_timing_fields", 0, len(mism))
    H("rerun.lines_only_in_rerun", 0, len(set(rer_by) - set(main_by)))
    H("rerun.main_lines_n_le_40_not_in_rerun", 0, sum(1 for k in main_by if k[0] <= 40 and k not in rer_by))
    # same arguments apart from ns and out prefix
    strip = lambda s: re.sub(r"ns=\[[^\]]*\] ", "", re.sub(r" out=\S+", "", s))
    H("rerun.header_seed_and_args_equal_main", 1,
      int(hr[0] == hdr[0] and strip(hr[1]) == strip(hdr[1])))
    detail["rerun_mismatch_keys"] = mism
    RC = list(csv.DictReader(read_text(PKG / "results/rerun_n8_40/pq2_rank.csv").splitlines()))
    H("rerun.json_rows_equal_log_all_fields", 99,
      sum(bin_json_eq_log(j, l) for j, l in zip(R, LR)) if len(R) == len(LR) else -1)
    H("rerun.csv_rows_equal_json", 99, sum(bin_csv_eq_json(c, j) for c, j in zip(RC, R)) if len(RC) == len(R) else -1)

    # ---- Claim 4: |F'| / |V| on the rerun JSON -------------------------------
    rat = [r["F_size"] / (1 << r["nprime"]) for r in R if r["F_size"] is not None]
    rat30 = [r["F_size"] / (1 << r["nprime"]) for r in R if r["F_size"] is not None and r["n"] >= 30]
    H("F_over_V.instances", 99, len(rat))
    H("F_over_V.min", 0.84, round(min(rat), 2), recomputed_exact=min(rat), compared_at="2 decimals")
    H("F_over_V.max", 1.31, round(max(rat), 2), recomputed_exact=max(rat), compared_at="2 decimals")
    H("F_over_V.mean", 1.021, round(statistics.fmean(rat), 3), recomputed_exact=statistics.fmean(rat),
      compared_at="3 decimals")
    H("F_over_V.n_ge_30.instances", 33, len(rat30))
    H("F_over_V.n_ge_30.min", 0.997, round(min(rat30), 3), recomputed_exact=min(rat30), compared_at="3 decimals")
    H("F_over_V.n_ge_30.max", 1.004, round(max(rat30), 3), recomputed_exact=max(rat30), compared_at="3 decimals")

    # ---- Claim 5: crosscheck agreement ---------------------------------------
    def parse_cc(p: Path):
        out = {}
        for line in read_text(p).splitlines():
            m = re.match(r"^(n\d+_i\d+)\.json \{(.*)\}$", line.strip())
            if not m:
                continue
            d = {}
            for kv in re.findall(r"'(\w+)': (\w+)", m[2]):
                d[kv[0]] = int(kv[1]) if kv[1].lstrip("-").isdigit() else kv[1]
            out[m[1]] = d
        return out

    cc_readme = read_text(PKG / "crosscheck/README.md")  # source of the d = 4 command (n15, n17, n18)
    detail["crosscheck_readme_d4_command"] = [l.strip() for l in cc_readme.splitlines() if "naive_closure.py 4" in l]
    d3 = parse_cc(PKG / "crosscheck/closure_d3.txt")
    d4 = parse_cc(PKG / "crosscheck/closure_d4.txt")
    rust_cc = {}
    for line in read_text(PKG / "crosscheck/rust_same_instances.txt").splitlines():
        r = parse_line(line.strip()) if line.startswith("n=") else None
        if r is None and line.startswith("n="):
            # the file carries the lines without the timing parentheses
            r = parse_line(line.strip() + " (0 ms Z, 0 ms deg2, 0 ms deg3, 0 ms deg4, 0 ms iter3, 0 ms iter4)")
        if r is not None:
            rust_cc[f"n{r['n']}_i{r['inst']}"] = r
    # k from each dumped Z with the validator's own rank (affine functions vanishing on Z)
    def k_from_dump(p: Path):
        dd = json.loads(read_text(p))
        piv = {}
        for z in dd["Z"]:
            v = 1 | (z << 1)
            while v:
                t = v.bit_length() - 1
                if t in piv:
                    v ^= piv[t]
                else:
                    piv[t] = v
                    break
        return dd["N"] + 1 - len(piv)

    names = sorted(rust_cc)
    kd = {nm: k_from_dump(PKG / f"crosscheck/{nm}.json") for nm in names}
    H("crosscheck.instances", 5, len(names))
    H("crosscheck.d3_python_low_eq_rust_iter3", 5, sum(nm in d3 and d3[nm]["low"] == rust_cc[nm]["iter3"] for nm in names))
    H("crosscheck.k_python_eq_rust_eq_dumpZ", 5,
      sum(nm in d3 and d3[nm]["k"] == rust_cc[nm]["k"] == kd[nm] for nm in names))
    H("crosscheck.d4_python_instances", 3, len(d4))
    rust_d4 = sum(1 for nm in names if rust_cc[nm]["iter4"] is not None)
    # README: "It agrees with Rust on five dumped instances at n = 14 to 18, at degree 3 and at degree 4."
    H("crosscheck.d4_python_eq_rust_iter4_on_same_instances", 5,
      sum(1 for nm in names if nm in d4 and rust_cc[nm]["iter4"] is not None and d4[nm]["low"] == rust_cc[nm]["iter4"]),
      note=("README Results claims agreement with Rust at degree 4 on the five dumped instances. In the committed "
            f"artifacts the Rust run on those instances has iter4 = '-' on all five ({rust_d4} Rust degree-4 values; "
            "its command ends '18 0', iter4_max_n = 0), and Python degree 4 was run on only 3 of the 5 "
            "(n15, n17, n18). The degree-4 agreement is therefore not supported by the committed artifacts. "
            "See replay_certificate.json for the validator's own Rust degree-4 values on these instances."))
    detail["crosscheck_rust_iter3"] = {nm: rust_cc[nm]["iter3"] for nm in names}
    detail["crosscheck_python_d3_low"] = {nm: d3[nm]["low"] for nm in d3}
    detail["crosscheck_python_d4_low"] = {nm: d4[nm]["low"] for nm in d4}
    detail["crosscheck_k_from_dump_Z"] = kd

    # ---------------------------------------------------------------------------
    failed = [h for h in heads if h["claimed"] != h["recomputed"]]
    status = "pass" if not failed else "fail"
    notes.append(f"{len(heads)} headline items recomputed; {len(failed)} do not match: "
                 + "; ".join(f"{h['name']} claimed {h['claimed']} recomputed {h['recomputed']}" for h in failed))
    notes.append("Counts parsed directly from results/run.log by the validator's own parser; derived JSON/CSV files "
                 "checked row by row against their source logs. Rounded claims (F_over_V) are compared at the "
                 "precision stated in the README; exact values are in recomputed_exact.")
    head = subprocess.run(["git", "-C", str(REPO), "rev-parse", "HEAD"], capture_output=True, text=True).stdout.strip()
    notes.append(f"All inputs read from commit {COMMIT} (git object store). Working-tree copies differing from it: "
                 f"{WORKTREE_DIFFERS or 'none'}. Repository HEAD at recompute time: {head}.")
    if WORKTREE_DIFFERS:
        status = "fail"
    report = {
        "schema": "pcc.validator-report/v1",
        "validated_commit": COMMIT,
        "head_at_recompute": head,
        "worktree_differs": WORKTREE_DIFFERS,
        "status": status,
        "validator": VALIDATOR,
        "produced_at": dt.datetime.now(dt.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "raw_data": [{"path": p, "sha256": h} for p, h in sorted(READ.items())],
        "headline": heads,
        "notes": " ".join(notes),
        "detail": detail,
    }
    (HERE / "validator_report.json").write_text(json.dumps(report, indent=1) + "\n")
    for h in heads:
        flag = "ok " if h["claimed"] == h["recomputed"] else "BAD"
        print(f"{flag} {h['name']}: claimed {h['claimed']} recomputed {h['recomputed']}")
    print("status:", status, "| files pinned:", len(READ))
    return 0 if status == "pass" else 1


if __name__ == "__main__":
    sys.exit(main())
