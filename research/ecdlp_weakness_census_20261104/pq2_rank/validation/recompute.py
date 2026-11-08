#!/usr/bin/env python3
"""Validator recompute of the pq2_rank census headline counts (Task A).

Written from scratch by the validator. It does not import or copy
log_to_json.py or crosscheck/. It parses results/run.log itself (not the JSON),
recomputes each headline count of README "Results", checks every derived file
(results/pq2_rank.json, .csv; the degree-4 sweep JSON/CSV/log; the rerun
JSON/CSV/log) row by row against its source, pins every file it reads by
SHA-256, and writes validation/validator_report.json.

Run from anywhere:  python3 -I validation/recompute.py
  --commit SHA        read every producer file from this commit (default: the validated f96f4f4c)
  --claims corrected  check the corrected README/corpus wording instead of the original
  --worktree          non-binding preview against the uncommitted working tree: every file is
                      hashed when read and re-hashed at the end; writes
                      validator_report.worktree_preview.json (status "refused"), never
                      validator_report.json
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


WORKTREE = False


def read_text(p: Path) -> str:
    r = rel(p)
    if WORKTREE:
        data = p.read_bytes()
    else:
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



def final2_items(H, readme, L, detail):
    """Items for the text at the final binding commit: the withdrawn bounded-degree reading, the
    Kousidis-Wiemers figures, the n = 36 / 37 step test, R5-002, R5-004 and the new pcc ids."""
    by = {(r["n"], r["inst"]): r for r in L}
    t = " ".join(readme.split())
    # ---- withdrawn phrases are absent everywhere --------------------------------
    docs = {"pq2_readme": PKG / "README.md", "census_readme": PKG.parent / "README.md",
            "report_typ": PKG.parent / "report.typ",
            "triage": REPO / "docs/ecdlp-hardness-corpus/WEAKNESS_TRIAGE.md",
            "lean-round5": REPO / "docs/ecdlp-hardness-corpus/lean-round5-20261107.json",
            "WEAKNESSES.md": REPO / "docs/ecdlp-hardness-corpus/WEAKNESSES.md",
            "weakness.json": REPO / "docs/ecdlp-hardness-corpus/weakness.json"}
    text = {k: " ".join(read_text(v).split()) for k, v in docs.items()}
    phrases = ["bounded-degree premise", "evidence for the bounded", "consistent with the bounded"]
    for k, v in text.items():
        H(f"withdrawn.{k}.bounded_degree_phrases", 0, sum(v.count(ph) for ph in phrases))
    # ---- Kousidis-Wiemers figures against the corpus record -------------------------
    lit = json.loads(read_text(REPO / "docs/ecdlp-hardness-corpus/lit-groebner.json"))
    kw = ""

    def walk(o):
        nonlocal kw
        if isinstance(o, dict):
            if o.get("id") == "MEAS-GB-KW-TABLE1":
                kw = o.get("statement", "")
            for v in o.values():
                walk(v)
        elif isinstance(o, list):
            for v in o:
                walk(v)
    walk(lit)
    m = re.search(r"m = 2, n = (\d+)\.\.(\d+): D_ff = 2 always, D_reg \(highest step degree\) = (\d+) for n <= (\d+) "
                  r"and (\d+) for n = (\d+)\.\.(\d+)", kw)
    kwv = tuple(int(x) for x in m.groups()) if m else (None,) * 7  # lo, hi, d_low, n_low_max, d_high, n_high_min, n_high_max
    detail["final2.kw_record"] = kwv
    H("pq2_readme.kw_magma_degree_4_up_to", text_num(t, r"Kousidis and Wiemers report Magma degree 4 for n ≤ (\d+) and 5 from n = \d+"), kwv[3])
    H("pq2_readme.kw_magma_degree_5_from", text_num(t, r"Kousidis and Wiemers report Magma degree 4 for n ≤ \d+ and 5 from n = (\d+)"), kwv[5])
    H("pq2_readme.direction_kw_5_from", text_num(t, r"They report 5 from n = (\d+) to \d+ \(`MEAS-GB-KW-TABLE1`\)"), kwv[5])
    H("pq2_readme.direction_kw_5_to", text_num(t, r"They report 5 from n = \d+ to (\d+) \(`MEAS-GB-KW-TABLE1`\)"), kwv[6])
    s36_ = json.loads(read_text(PKG / "f4/step_n36_37/msolve_n36_i0.json"))["rows"][0]
    main_ns = {r["n"] for r in json.loads(read_text(PKG / "f4/msolve_all.json"))["rows"]}
    msolve_ns = sorted(main_ns | ({s36_["n"]} if s36_.get("msolve_exit") == 0 else set()))
    detail["final2.msolve_measured_n"] = msolve_ns
    mo = re.search(r"agrees with their Magma value 4 where the ranges overlap \(n = (\d+) to (\d+)\)", text["census_readme"])
    if mo:
        stated = list(range(int(mo[1]), int(mo[2]) + 1))
        measured = [n for n in stated if n in msolve_ns and kwv[0] is not None and kwv[0] <= n <= kwv[3]]
        H("census_readme.overlap_n_values_with_msolve_and_magma_4", len(stated), len(measured),
          note=(f"stated overlap n = {stated}; msolve measured n in that span: {measured} (main sweep to n = 34, step test at "
                f"n = 36 and 37); Kousidis-Wiemers' degree 4 covers n = {kwv[0]}..{kwv[3]}. There is no msolve run at n = 35, "
                "so agreement 'where the ranges overlap (n = 34 to 36)' is stated for an n with no data. Supported wording: "
                "'where the ranges overlap (n = 34, and the fresh n = 36 control)', as in R5-004."))
        H("census_readme.kw_magma_5_from", text_num(text["census_readme"], r"they report 5 from n = (\d+)"), kwv[5])
    kw_docs = [] if mo else [
            ("census_readme", r"Magma value 4 for n ≤ (\d+); they report 5 from n = \d+", r"Magma value 4 for n ≤ \d+; they report 5 from n = (\d+)")]
    for k, pat_hi, pat_from in kw_docs + [
            ("report_typ", r"Magma value for \$n <= (\d+)\$; they report 5 from \$n = \d+\$", r"Magma value for \$n <= \d+\$; they report 5 from \$n = (\d+)\$"),
            ("triage", r"Magma value for n ≤ (\d+); they report 5 from n = \d+", r"Magma value for n ≤ \d+; they report 5 from n = (\d+)")]:
        H(f"{k}.kw_magma_4_up_to", text_num(text[k], pat_hi), kwv[3])
        H(f"{k}.kw_magma_5_from", text_num(text[k], pat_from), kwv[5])
    # ---- msolve rows (main sweep) and the Direction bullet -------------------------
    A = {(r["n"], r["instance"]): r for r in json.loads(read_text(PKG / "f4/msolve_all.json"))["rows"]}
    band = [r for k, r in A.items() if 17 <= k[0] <= 34]
    deg_by_n = {n: sorted({r["max_step_degree"] for k, r in A.items() if k[0] == n}) for n in range(8, 35)}
    rises = [n for n in range(9, 35) if len(deg_by_n[n]) == 1 and len(deg_by_n[n - 1]) == 1
             and deg_by_n[n][0] > deg_by_n[n - 1][0]]
    detail["final2.msolve_degree_by_n"] = deg_by_n
    def one(xs):
        xs = list(xs)
        return xs[0] if xs and len(set(xs)) == 1 else -1
    H("pq2_readme.direction_degree_4_n17_34", 4 if "msolve's degree is 4 on every instance from n = 17 to 34" in t else None,
      one(r["max_step_degree"] for r in band))
    H("pq2_readme.direction_one_above_kw_3", 1 if "one above the Kousidis–Wiemers value 3" in t else None,
      one(r["max_step_degree"] for r in band) - 3)
    s36 = json.loads(read_text(PKG / "f4/step_n36_37/msolve_n36_i0.json"))["rows"][0]
    overlap = [r["max_step_degree"] for k, r in A.items() if kwv[0] is not None and kwv[0] <= k[0] <= kwv[3]] + [s36["max_step_degree"]]
    H("pq2_readme.direction_matches_magma_4_where_ranges_overlap",
      4 if "matches their Magma value 4 for n ≤ 36" in t else None, one(overlap),
      note=f"msolve degrees where Kousidis-Wiemers report 4 (n = {kwv[0]}..{kwv[3]}): the main-sweep rows at n = 34 and the fresh n = 36 control")
    H("pq2_readme.direction_rises_inside_window_first", text_num(t, r"The step from 3 to 4 inside this window, at n = (\d+) and n = \d+"),
      rises[0] if rises else None, note=f"n where msolve's degree rises over n - 1 on n = 8..34: {rises}")
    H("pq2_readme.direction_rises_inside_window_second", text_num(t, r"The step from 3 to 4 inside this window, at n = \d+ and n = (\d+)"),
      rises[1] if len(rises) > 1 else None)
    H("pq2_readme.direction_published_data_show_rise", 1 if "the published data already show it rising" in t else None,
      int(kwv[4] is not None and kwv[4] > kwv[2]))
    # ---- the n = 36 / 37 step test -------------------------------------------------
    sp = json.loads(read_text(PKG / "f4/step_n36_37/pq2_rank.json"))["rows"]
    S = {(r["n"], r["instance"]): r for r in sp}
    H("step.instances_dumped", 4, len(sp))
    H("step.dumps_byte_identical_to_validator_regeneration", 4,
      sum(read_text(PKG / f"f4/step_n36_37/dumps/n{n}_i{i}.json") == read_own(HERE / f"step_n36_37/inst/n{n}_i{i}.json").decode()
          for (n, i) in S))
    cs = json.loads(read_own(HERE / "closure_step_n36_37.json"))["results"]
    H("step.dumps_equal_S3_descent", 4, sum(r["checks"].get("descent_anf_equals_dump") is True and not r["failed_checks"] for r in cs))
    H("pq2_readme.step_n36_degree", text_num(t, r"n = 36, instance 0 \(control\) \| maximal step degree (\d+)"), s36["max_step_degree"])
    H("pq2_readme.step_n36_linear", text_num(t, r"linear polynomials = k = (\d+), \d+ minutes"), s36["linear_in_gb"])
    H("pq2_readme.step_n36_k", text_num(t, r"linear polynomials = k = (\d+), \d+ minutes"), S[(36, 0)]["k"],
      note="census k of the regenerated n = 36 instance 0 (from its Z)")
    H("pq2_readme.step_n36_minutes", text_num(t, r"linear polynomials = k = \d+, (\d+) minutes"), round(s36["msolve_seconds"] / 60))
    own36 = HERE / "msolve_replay/msolve_step_n36.json"
    if own36.exists():
        rr = json.loads(read_own(own36))["results"]
        H("step.validator_msolve_n36_i0_agrees", 1, sum(not r["mismatches"] and not r["failed_checks"] for r in rr),
          note="validator's own msolve run and parser: steps, degree, basis size, linear count = producer row; quotient = |Z|; linear = k")
    att = [json.loads(read_text(PKG / f"f4/step_n36_37/{f}"))["rows"][0] for f in ("msolve_n37_i0.killed_t3.json", "msolve_n37_i0.json")]
    killed = [r for r in att if r["msolve_exit"] == -9 and r["gb_size"] == 0 and max(r["steps"]) == 4]
    oom = (HERE / "step_n36_37/kernel_oom.txt").read_text()
    READ[rel(HERE / "step_n36_37/kernel_oom.txt")] = hashlib.sha256(oom.encode()).hexdigest()
    n_oom = len(re.findall(r"Memory cgroup out of memory: Killed process \d+ \(msolve\)", oom))
    H("pq2_readme.step_n37_killed_twice_for_memory", 2 if "killed twice for memory during degree-4 steps" in t else None,
      min(len(killed), n_oom),
      note="attempts with exit -9, no basis and last step degree 4 (killed_t3 and single-thread), each matched by a kernel "
           "'Memory cgroup out of memory: Killed process (msolve)' record captured by the validator from dmesg in the same boot "
           "(uptime 12907 s ~ 23:12:45 and 16563 s ~ 00:13:41 UTC)")
    tlog = read_text(PKG / "f4/step_n36_37/msolve_n37_i0.timeout_3600s.log")
    H("step.n37_first_attempt_timed_out_3600s", 1, int("timed out after 3600 seconds" in tlog))
    i1_none = sum(read_text(PKG / f"f4/step_n36_37/msolve_n{n}_i1.log") == "" and
                  subprocess.run(["git", "-C", str(REPO), "cat-file", "-e", f"{COMMIT}:{rel(PKG / f'f4/step_n36_37/msolve_n{n}_i1.json')}"],
                                 capture_output=True).returncode != 0 for n in (36, 37))
    H("pq2_readme.step_i1_no_result", 2 if "stopped early to save memory, no result" in t else None, i1_none)
    mem = [(l.split()[0], float(l.split()[1])) for l in read_text(PKG / "f4/step_n36_37/msolve_n37_i0.mem.log").splitlines() if l.strip()]
    inc = [b[1] - a[1] for a, b in zip(mem, mem[1:])]
    last5 = sorted(inc[-5:])[2]
    detail["final2.mem_log"] = {"samples": len(mem), "max_GiB": max(v for _, v in mem), "median_rate_last5_GiB_per_min": last5,
                                "mean_rate_whole_run_GiB_per_min": (mem[-1][1] - mem[0][1]) / (len(mem) - 1)}
    H("pq2_readme.step_mem_rate_GiB_per_min", float(m2[1]) if (m2 := re.search(r"memory grew about ([\d.]+) GiB per minute", t)) else None,
      round(last5, 1), compared_at="1 decimal",
      note=f"median per-minute growth over the last five minutes of msolve_n37_i0.mem.log ({last5:.2f} GiB/min); over the whole "
           f"60-minute run the mean is {detail['final2.mem_log']['mean_rate_whole_run_GiB_per_min']:.2f} GiB/min with long plateaus, "
           "so the README's rate describes the final phase only")
    mavg = re.search(r"\(([\d.]+) GiB per minute averaged over the hour\)", t)
    if mavg or "over the last five minutes" in t:
        H("pq2_readme.step_mem_rate_is_last_five_minutes", 1, int("about 1.2 GiB per minute over the last five minutes" in t))
        H("pq2_readme.step_mem_mean_rate_GiB_per_min", float(mavg[1]) if mavg else None,
          round(detail["final2.mem_log"]["mean_rate_whole_run_GiB_per_min"], 2), compared_at="2 decimals")
    H("pq2_readme.step_mem_passed_GiB", text_num(t, r"and passed (\d+) GiB on a \d+ GiB machine"), int(max(v for _, v in mem)))
    memtotal = int(re.search(r"MemTotal:\s+(\d+) kB", Path("/proc/meminfo").read_text())[1]) / 1024 / 1024
    H("pq2_readme.step_machine_GiB", text_num(t, r"passed \d+ GiB on a (\d+) GiB machine"), int(memtotal),
      note="MemTotal of this VM, the same boot in which the n = 37 runs were killed (environment, not a repository artifact)")
    kept = [f"f4/step_n36_37/{f}" for f in ("msolve_n37_i0.timeout_3600s.log", "msolve_n37_i0.killed_t3.json", "msolve_n37_i0.killed_t3.log")]
    H("pq2_readme.step_earlier_attempts_kept", 3 if "*.timeout_3600s.log" in t and "*.killed_t3.*" in t else None,
      sum(subprocess.run(["git", "-C", str(REPO), "cat-file", "-e", f"{COMMIT}:{rel(PKG / f)}"]).returncode == 0 for f in kept))
    # ---- corpus fragment: MEAS-PQ2-MSOLVE-F4, R5-002 (3 copies), R5-004, THM-F4-CONTAINMENT ids ------
    def objs(path):
        flat = {}

        def w(o):
            if isinstance(o, dict):
                if isinstance(o.get("id"), str):
                    flat[o["id"]] = o
                for v in o.values():
                    w(v)
            elif isinstance(o, list):
                for v in o:
                    w(v)
        w(json.loads(read_text(path)))
        return flat
    r5 = objs(REPO / "docs/ecdlp-hardness-corpus/lean-round5-20261107.json")
    ms = " ".join(r5.get("MEAS-PQ2-MSOLVE-F4", {}).get("statement", "").split())
    H("corpus.MEAS-PQ2-MSOLVE-F4.n36_degree", 4 if "On one fresh instance at n = 36 msolve's degree is also 4" in ms else None, s36["max_step_degree"])
    H("corpus.MEAS-PQ2-MSOLVE-F4.n37_not_finished", 1 if "a fresh n = 37 instance did not finish within 15 GB of memory" in ms else None,
      int(len(killed) == 2 and n_oom >= 2))
    thm = r5.get("THM-F4-CONTAINMENT", {}).get("scope", "")
    ids = re.findall(r"PCC-\d{8}-[0-9a-f]{6}", thm)
    H("corpus.THM-F4-CONTAINMENT.pcc_ids_present", len(ids) if ids else None,
      sum(subprocess.run(["git", "-C", str(REPO), "cat-file", "-e", f"{COMMIT}:pcc/claims/{i}.json"]).returncode == 0 for i in ids))
    for doc in ("lean-round5", "WEAKNESSES.md", "weakness.json"):
        txt = " ".join(r5.get("R5-002", {}).get("statement", "").split()) if doc == "lean-round5" else text[doc]
        H(f"{doc}.R5-002.msolve_degree_4_n17_34", 4 if "Real F4 (msolve) degree is 4 on n = 17 to 34" in txt else None,
          one(r["max_step_degree"] for r in band))
        H(f"{doc}.R5-002.one_above_kw_3", 1 if "one above the Kousidis-Wiemers value 3" in txt else None,
          one(r["max_step_degree"] for r in band) - 3)
        H(f"{doc}.R5-002.kw_reports_5_from", text_num(txt, r"MEAS-GB-KW-TABLE1 reports 5 from n = (\d+)"), kwv[5])
        H(f"{doc}.R5-002.equal_magma_value_where_reported", 4 if "equal to their Magma value for n <= 36" in txt else None, one(overlap))
    r54 = " ".join(r5.get("R5-004", {}).get("statement", "").split())
    mm = re.search(r"Reproduces the Magma value 4 with a second implementation on n = (\d+) to (\d+), below the n = (\d+) step to 5", r54)
    claimed_ns = list(range(int(mm[1]), int(mm[2]) + 1)) if mm else []
    with_magma = [n for n in claimed_ns if kwv[0] is not None and kwv[0] <= n <= kwv[1]]
    H("corpus.R5-004.step_at", text_num(r54, r"below the n = (\d+) step to 5"), kwv[5])
    mn = re.search(r"Agrees with the Magma value 4 where the ranges overlap \(n = (\d+), and the fresh n = (\d+) control\); "
                   r"msolve gives (\d+) on n = (\d+) to (\d+)", r54)
    if mn:
        main_overlap = sorted({k[0] for k in A if kwv[0] is not None and kwv[0] <= k[0] <= kwv[3]})
        H("corpus.R5-004.overlap_main_sweep_n", int(mn[1]), main_overlap[0] if len(main_overlap) == 1 else -1,
          note=f"main-sweep msolve n inside Kousidis-Wiemers' degree-4 range n = {kwv[0]}..{kwv[3]}: {main_overlap}")
        H("corpus.R5-004.control_n", int(mn[2]),
          s36["n"] if s36["msolve_exit"] == 0 and kwv[0] <= s36["n"] <= kwv[3] else -1)
        H("corpus.R5-004.agrees_with_magma_4", 4,
          one([r["max_step_degree"] for k, r in A.items() if k[0] in main_overlap] + [s36["max_step_degree"]]))
        H("corpus.R5-004.msolve_gives_4_n17_34", int(mn[3]), one(r["max_step_degree"] for r in band))
        H("corpus.R5-004.range", int(mn[5]) - int(mn[4]) + 1, len({k[0] for k in A if 17 <= k[0] <= 34}))
    if mm:
      H("corpus.R5-004.n_values_reproduced", len(claimed_ns) if mm else None, len(with_magma),
      note=(f"R5-004 says msolve reproduces the Magma value 4 on n = 17..34 ({len(claimed_ns)} values of n). The corpus record "
            f"MEAS-GB-KW-TABLE1 has Magma data for m = 2 only on n = {kwv[0]}..{kwv[1]}, so the two overlap at n = {with_magma} "
            "(3 main-sweep instances; the fresh n = 36 control is outside the stated range). On n = 17..33 there is no Magma value "
            "to reproduce: msolve extends the value 4 there, it does not reproduce it. Supported wording: 'Agrees with the Magma "
            "value 4 where the ranges overlap (n = 34, and the fresh n = 36 control); msolve gives 4 on n = 17 to 34, below the "
            "n = 37 step to 5.'"))
    lean = " ".join(read_text(REPO / "research/ecdlp_weakness_lean_20261104/README.md").split())
    H("lean_readme.round5_new_ids_cited", 2 if "PCC-20261107-209a57" in lean and "PCC-20261107-91a01d" in lean else None,
      sum(subprocess.run(["git", "-C", str(REPO), "cat-file", "-e", f"{COMMIT}:pcc/claims/{i}.json"]).returncode == 0
          for i in ("PCC-20261107-209a57", "PCC-20261107-91a01d")))
    H("lean_readme.old_ids_gone", 0, sum(lean.count(i) + text["lean-round5"].count(i) for i in ("PCC-20261105-0d2470", "PCC-20261105-96e86b")))


def final_items(H, readme, L, est, detail):
    """Items for the text and artifacts at the final commit: the degree-4 sweeps to n = 27, the
    msolve F4 run (f4/), and every number in the changed documents."""
    by = {(r["n"], r["inst"]): r for r in L}
    n44 = {k for k in by if k[0] <= 44}
    est44 = est & n44
    # ---- degree-4 sweeps n = 17..27 ------------------------------------------------
    fresh, fresh_new = [], []
    main_params = set()
    for r in json.loads(read_own(HERE / "replay_n8_44/pq2_rank.json"))["rows"]:
        main_params.add((r["a"], r["b"], r["x_R"], tuple(r["basis"])))
    for tag in ("iter4_n17_20", "iter4_n21_24", "iter4_n25", "iter4_n26", "iter4_n27"):
        rows = json.loads(read_text(PKG / f"results/{tag}/pq2_rank.json"))["rows"]
        fresh += rows
        if tag in ("iter4_n25", "iter4_n26", "iter4_n27"):
            fresh_new += rows
            h, lr, other = parse_log(PKG / f"results/{tag}/run.log")
            csvr = list(csv.DictReader(read_text(PKG / f"results/{tag}/pq2_rank.csv").splitlines()))
            same_log = sum(all(j[k] == l[v] for k, v in (("n", "n"), ("instance", "inst"), ("k", "k"), ("iter3", "iter3"),
                                                          ("iter4", "iter4"), ("d_iter", "d_iter"), ("Z_size", "Z")))
                           for j, l in zip(rows, lr)) if len(lr) == len(rows) else -1
            same_csv = sum(c["k"] == str(j["k"]) and c["iter4"] == str(j["iter4"]) and c["d_iter"] == j["d_iter"]
                           for c, j in zip(csvr, rows)) if len(csvr) == len(rows) else -1
            H(f"{tag}.instances", 3, len(rows))
            H(f"{tag}.json_rows_equal_log", 3, same_log)
            H(f"{tag}.csv_rows_equal_json", 3, same_csv)
            H(f"{tag}.instances_shared_with_main_stream", 0,
              sum((r["a"], r["b"], r["x_R"], tuple(r["basis"])) in main_params for r in rows))
    same9 = sum(r["iter3"] == 1 and r["k"] > 1 and r["iter4"] == r["k"] for r in fresh_new)
    d4_33 = sum(r["d_iter"] == "4" and r["iter3"] < r["k"] and r["iter4"] == r["k"] for r in fresh)
    own_d4 = 0
    for n in (25, 26, 27):
        for r in json.loads(read_own(HERE / f"closure_iter4_n{n}.json"))["results"]:
            own_d4 += (not r["value_mismatches"] and not r["failed_checks"] and r["mine"]["iter3"] < r["mine"]["k"]
                       and r["mine"]["iter4"] == r["mine"]["k"])
    H("iter4_n25_27.validator_closure_confirms_iter3_lt_k_iter4_eq_k", len(fresh_new), own_d4,
      note="validator's own degree-4 closure (closure_check.py) on regenerated dumps, compared with the producer JSON")
    t = " ".join(readme.split())
    H("pq2_readme.fresh_n25_27_same_outcome", text_num(t, r"extend this to n = 27 with the same outcome on all (\d+) instances"), same9)
    H("pq2_readme.fresh_total_d_iter_4", text_num(t, r"In total d_iter = 4 on (\d+) of \d+ fresh instances from n = 17 to 27"), d4_33)
    H("pq2_readme.fresh_total_instances", text_num(t, r"In total d_iter = 4 on \d+ of (\d+) fresh instances from n = 17 to 27"), len(fresh))
    H("pq2_readme.reading_fresh_need_exactly_4", text_num(t, r"The (\d+) fresh instances at n = 17 to 27 need exactly 4"), d4_33)
    # ---- f4: dumps, dump log, msolve rows ---------------------------------------------
    ls = subprocess.run(["git", "-C", str(REPO), "ls-tree", "--name-only", COMMIT, rel(PKG / "f4/dumps") + "/"],
                        capture_output=True, text=True).stdout.split()
    dump_names = sorted(Path(x).name for x in ls if x.endswith(".json"))
    expect_names = sorted(f"n{n}_i{i}.json" for n in range(8, 35) for i in range(3))
    identical = sum(read_text(PKG / f"f4/dumps/{nm}") == read_own(HERE / f"main_n8_34_dump/inst/{nm}").decode()
                    for nm in dump_names if (HERE / f"main_n8_34_dump/inst/{nm}").exists())
    H("f4.dumps_are_the_81_main_sweep_keys", 81, sum(nm in expect_names for nm in dump_names) if len(dump_names) == 81 else -1)
    H("f4.dumps_byte_identical_to_validator_regeneration", 81, identical,
      note="validator regenerated the main stream n = 8..34 with the census arguments (validation/main_n8_34_dump); "
           "the n <= 24 dumps also passed its full S3 ANF check")
    desc_ok = 0
    for f in ("closure_replay_n8_20.json", "closure_replay_n21_24.json", "closure_main_n25_34.json"):
        for r in json.loads(read_own(HERE / f))["results"]:
            nm = f"n{r['n']}_i{r['instance']}.json"
            same = nm in dump_names and read_own(HERE / r["dump"].split("validation/", 1)[1]).decode() == read_text(PKG / f"f4/dumps/{nm}")
            desc_ok += bool(same and r["checks"].get("descent_anf_equals_dump") and not r["failed_checks"])
    H("f4.dumps_equations_equal_S3_descent", 81, desc_ok,
      note="validator's S3 descent: 2^N brute-force ANF and Z for n <= 24 (closure_replay_*), exact weight-<=2 "
           "interpolation plus 500 random points for n = 25..34 (closure_main_n25_34); each dump byte-identical to f4/dumps")
    dh, dl, do = parse_log(PKG / "f4/dump_run.log")
    eq_lines = sum(1 for r in dl if (r["n"], r["inst"]) in by and all(r[f] == by[(r["n"], r["inst"])][f] for f in VALUE_KEYS))
    main_hdr = read_text(PKG / "results/run.log").splitlines()[0]
    H("f4.dump_log_lines", 81, len(dl))
    H("f4.dump_log_seed_line_equal", 1, int(bool(dh) and dh[0] == main_hdr))
    H("f4.dump_log_args_ns_8_34_3_per_n_d4_20", 1,
      int(bool(re.search(r"ns=\[8, 9, .*, 33, 34\] instances_per_n=3 d4_max_n=20 ", dh[1] if len(dh) > 1 else ""))))
    pnum = lambda pat: text_num(t, pat)
    H("pq2_readme.f4_instances", pnum(r"ran on the main sweep's own instances for n = 8 to 34, (\d+) in all"), len(dump_names))
    H("pq2_readme.f4_dump_log_equals_run_log", pnum(r"the dump log equals `results/run.log` on all (\d+) lines"), eq_lines)
    # precedence merge: the full sweep first, then per-n and per-instance files
    files = ["f4/msolve_n8_34.json", "f4/msolve_n26_i0.json", "f4/msolve_n33.json", "f4/msolve_n33_i1.json",
             "f4/msolve_n33_i2.json", "f4/msolve_n34.json", "f4/msolve_n34_i2.json"]
    src = {}
    for f in files:
        for r in json.loads(read_text(PKG / f))["rows"]:
            src[(r["n"], r["instance"])] = (f, r)
    full = {(r["n"], r["instance"]): r for r in json.loads(read_text(PKG / "f4/msolve_n8_34.json"))["rows"]}
    allr = json.loads(read_text(PKG / "f4/msolve_all.json"))["rows"]
    A = {(r["n"], r["instance"]): r for r in allr}
    H("f4.msolve_all_rows", 81, len(allr))
    H("f4.msolve_all_equals_precedence_merge", 81, sum(A.get(k) == v[1] for k, v in src.items()) if len(src) == 81 else -1)
    H("f4.msolve_exit_0", 81, sum(r["msolve_exit"] == 0 for r in allr))
    H("f4.row_census_fields_equal_run_log", 81,
      sum(k in by and r["census_k"] == by[k]["k"] and r["census_d_iter"] == by[k]["d_iter"] for k, r in A.items()))
    H("f4.row_k_equals_dump_Z_k", 81,
      sum(r["k"] == by[k]["k"] for k, r in A.items() if k in by))
    lin_ok = sum(r["linear_in_gb"] == by[k]["k"] for k, r in A.items() if k in by)
    exact = [(k, r) for k, r in A.items() if by[k]["d_iter"].isdigit()]
    exact_ok = sum(r["max_step_degree"] == int(by[k]["d_iter"]) for k, r in exact)
    gt3_below = sum(1 for k, r in A.items() if by[k]["d_iter"].startswith(">") and r["max_step_degree"] < int(by[k]["d_iter"][1:]) + 1)
    below_lb = sum(1 for k, r in A.items()
                   if r["max_step_degree"] < (int(by[k]["d_iter"]) if by[k]["d_iter"].isdigit() else int(by[k]["d_iter"][1:]) + 1))
    band = [r for k, r in A.items() if 17 <= k[0] <= 34]
    deg4 = sum(r["max_step_degree"] == 4 for r in band)
    distinct_17_34 = len({r["max_step_degree"] for r in band})
    distinct_8_34 = sorted({r["max_step_degree"] for r in A.values()})
    le16 = [r for k, r in A.items() if k[0] <= 16]
    H("pq2_readme.f4_table_linear_eq_k", pnum(r"linear polynomials in msolve's reduced basis equal k \| (\d+) of \d+"), lin_ok)
    H("pq2_readme.f4_table_linear_eq_k_of", pnum(r"linear polynomials in msolve's reduced basis equal k \| \d+ of (\d+)"), len(A))
    H("pq2_readme.f4_table_degree_eq_exact_d_iter", pnum(r"census value is exact \(n ≤ 16\) \| (\d+) of \d+"), exact_ok)
    H("pq2_readme.f4_table_exact_rows", pnum(r"census value is exact \(n ≤ 16\) \| \d+ of (\d+)"), len(exact))
    H("pq2_readme.f4_table_gt3_msolve_below_4", pnum(r"census d_iter > 3 and msolve below 4 \| (\d+)"), gt3_below)
    H("pq2_readme.f4_table_n17_34_degree_value", pnum(r"msolve maximal step degree, n = 17 to 34 \| (\d+) on all \d+"),
      next(iter({r["max_step_degree"] for r in band})) if distinct_17_34 == 1 else -1)
    H("pq2_readme.f4_table_n17_34_count", pnum(r"msolve maximal step degree, n = 17 to 34 \| \d+ on all (\d+)"), deg4)
    bad = full.get((26, 0), {})
    H("pq2_readme.f4_bad_row_linear", pnum(r"One row first recorded (\d+) linear polynomials"), bad.get("linear_in_gb"))
    rr = json.loads(read_text(PKG / "f4/msolve_n26_i0.json"))["rows"][0]
    H("pq2_readme.f4_n26_i0_rerun_gives_k", 1 if "The rerun gives k" in t else None,
      int(rr["linear_in_gb"] == by[(26, 0)]["k"] and A[(26, 0)] == rr))
    H("pq2_readme.f4_reruns_after_restart", pnum(r"(\w+) instances were rerun after a machine restart"),
      len([f for f in files if re.search(r"_n\d+_i\d+\.json$", f) and not f.endswith("n26_i0.json")]),
      note="recomputed: per-instance rerun files other than n26_i0 (n33_i1, n33_i2, n34_i2); the restart itself is not verifiable")
    H("pq2_readme.reading_msolve_exactly_4", pnum(r"msolve's F4 reaches exactly 4 on all (\d+) main-sweep instances at n = 17 to 34"), deg4)
    # validator's own msolve replay (independent driver and parser)
    rep_ = json.loads(read_own(HERE / "msolve_replay/msolve_replay.json"))["results"]
    if (HERE / "msolve_replay/msolve_replay_extra.json").exists():
        rep_ += json.loads(read_own(HERE / "msolve_replay/msolve_replay_extra.json"))["results"]
    detail["final.validator_msolve_replay_keys"] = sorted([r["n"], r["instance"]] for r in rep_)
    rep_ok = sum(not r["mismatches"] and not r["failed_checks"] for r in rep_)
    H("f4.validator_msolve_replay_agrees_with_producer_rows", len(rep_), rep_ok,
      note="validator re-ran msolve with its own input writer and parser; agreement on steps, max step degree, basis "
           "size, linear count, k, census degree; plus quotient dimension = |Z| and basis vanishing on Z")
    # ---- Direction sentence: the bounded-degree reading ------------------------------
    H("pq2_readme.direction_one_above_prediction", 1 if "The degree is one above the predicted value" in t else None,
      (next(iter({r["max_step_degree"] for r in band})) - 3) if distinct_17_34 == 1 else -1,
      note="msolve degree on n = 17..34 minus the Kousidis-Wiemers value 3")
    H("pq2_readme.direction_no_growth_n17_34", 1 if "on n = 17 to 34 it does not grow" in t else None, distinct_17_34,
      note="number of distinct msolve maximal step degrees on n = 17..34")
    lit = json.loads(read_text(REPO / "docs/ecdlp-hardness-corpus/lit-groebner.json"))
    kw = ""

    def walk(o):
        nonlocal kw
        if isinstance(o, dict):
            if o.get("id") == "MEAS-GB-KW-TABLE1":
                kw = o.get("statement", "")
            for v in o.values():
                walk(v)
        elif isinstance(o, list):
            for v in o:
                walk(v)
    walk(lit)
    m = re.search(r"m = 2, n = (\d+)\.\.(\d+): D_ff = 2 always, D_reg \(highest step degree\) = (\d+) for n <= (\d+) and (\d+) for n = (\d+)\.\.(\d+)", kw)
    kw_rise = bool(m) and int(m[5]) > int(m[3])
    detail["final.kw_table1_m2"] = m.groups() if m else None
    detail["final.msolve_degrees_n8_34"] = distinct_8_34
    supported = int(not kw_rise and len(distinct_8_34) == 1)
    why = ("Encoded judgment, recomputed from repository data: a constant step degree on n = 17..34 is evidence for a "
           "bounded degree only if nothing in the repository shows this family's degree rising. Two records do: "
           f"(1) the msolve data themselves take the values {distinct_8_34} on n = 8..34 (3 at n <= 14 and n = 16, "
           "4 at n = 15 and from n = 17); (2) the corpus's own record MEAS-GB-KW-TABLE1 (docs/ecdlp-hardness-corpus/lit-groebner.json) "
           + (f"gives, for the same family (m = 2, n = {m[1]}..{m[2]}), Magma highest step degree {m[3]} for n <= {m[4]} "
              f"and {m[5]} for n = {m[6]}..{m[7]}" if m else "(not found)")
           + ", i.e. the next step lies just past the msolve window (n <= 34). A two-fold window in n cannot "
           "discriminate a bounded degree from a slowly growing one (cf. the LOG fit in "
           "research/fall_degree_bounds_20261008/RESULTS.md). Supported wording: 'the degree is 4 on every instance "
           "from n = 17 to 34, matching Kousidis-Wiemers' value 4 for n <= 36; they report 5 from n = 37, so these "
           "data neither support nor refute a bounded degree'.")
    H("pq2_readme.direction_evidence_for_bounded_degree_premise",
      1 if "This is evidence for the bounded-degree premise" in t else None, supported, note=why)
    # ---- census README, report.typ, triage, Lean README ------------------------------
    cen = " ".join(read_text(PKG.parent / "README.md").split())
    H("census_readme.d4_sweeps_reach_k_n17_27", text_num(cen, r"Fresh degree-4 sweeps at n = 17 to 27 reach k on all (\d+) instances"), d4_33)
    H("census_readme.msolve_exactly_4_n17_34", text_num(cen, r"reaches maximal step degree exactly 4 on all (\d+) instances from n = 17 to 34"), deg4)
    H("census_readme.msolve_matches_census_where_exact", len(exact) if "matches the census wherever the census value is exact" in cen else None, exact_ok)
    H("census_readme.no_growth_n17_34", 1 if "The degree does not grow over n = 17 to 34" in cen else None, distinct_17_34)
    typ = " ".join(read_text(PKG.parent / "report.typ").split())
    H("report_typ.iterated_exactly_4_n17_27", text_num(typ, r"exactly 4 on (\d+) fresh instances at n = 17--27"), d4_33)
    H("report_typ.d4_sweeps_reach_k_n17_27", text_num(typ, r"Fresh degree-4 sweeps at n = 17 to 27 reach k on all (\d+) instances"), d4_33)
    H("report_typ.msolve_exactly_4_n17_34", text_num(typ, r"reaches maximal step degree exactly 4 on all (\d+) main-sweep instances from n = 17 to 34"), deg4)
    H("report_typ.table_msolve_degree_4", 4 if "msolve F4 degree 4 at n = 17--34" in typ else None,
      next(iter({r["max_step_degree"] for r in band})) if distinct_17_34 == 1 else -1)
    H("report_typ.no_growth_n17_34", 1 if "The degree does not grow over n = 17 to 34" in typ else None, distinct_17_34)
    tri = " ".join(read_text(REPO / "docs/ecdlp-hardness-corpus/WEAKNESS_TRIAGE.md").split())
    H("triage.d4_reaches_k_n17_27", text_num(tri, r"Degree 4 reaches k on (\d+) of \d+ fresh instances at n = 17 to 27"), d4_33)
    H("triage.msolve_exactly_4_n17_34", text_num(tri, r"reaches maximal step degree exactly 4 on all (\d+) main-sweep instances at n = 17 to 34"), deg4)
    H("triage.degree_stays_4_n17_34", 4 if "The degree stays at 4 over n = 17 to 34" in tri else None,
      next(iter({r["max_step_degree"] for r in band})) if distinct_17_34 == 1 else -1)
    lean = " ".join(read_text(REPO / "research/ecdlp_weakness_lean_20261104/README.md").split())
    ids = re.findall(r"PCC-\d{8}-[0-9a-f]{6}", lean[lean.find("Round 5"):]) if "Round 5" in lean else []
    present = sum(subprocess.run(["git", "-C", str(REPO), "cat-file", "-e", f"{COMMIT}:pcc/claims/{i}.json"]).returncode == 0
                  for i in ids)
    H("lean_readme.round5_pcc_claim_records_present", len(ids) if ids else None, present)
    # ---- corpus: round 5 fragment, round 4 edge, generated copies ----------------------
    def corpus_objs(doc):
        flat = []

        def w(o):
            if isinstance(o, dict):
                flat.append(o)
                for v in o.values():
                    w(v)
            elif isinstance(o, list):
                for v in o:
                    w(v)
        w(json.loads(read_text(REPO / doc)))
        return {o.get("id"): o for o in flat if isinstance(o.get("id"), str)}
    r5 = corpus_objs("docs/ecdlp-hardness-corpus/lean-round5-20261107.json")
    ms = r5.get("MEAS-PQ2-MSOLVE-F4", {}).get("statement", "")
    H("corpus.MEAS-PQ2-MSOLVE-F4.instances", text_num(ms, r"on the (\d+) pq2_rank main-sweep instances with n = 8 to 34"), len(A))
    H("corpus.MEAS-PQ2-MSOLVE-F4.deg3_le16", text_num(ms, r"maximal step degree 3 on (\d+) and 4 on \d+ of the \d+ instances with n <= 16"),
      sum(r["max_step_degree"] == 3 for r in le16))
    H("corpus.MEAS-PQ2-MSOLVE-F4.deg4_le16", text_num(ms, r"maximal step degree 3 on \d+ and 4 on (\d+) of the \d+ instances with n <= 16"),
      sum(r["max_step_degree"] == 4 for r in le16))
    H("corpus.MEAS-PQ2-MSOLVE-F4.le16_rows", text_num(ms, r"of the (\d+) instances with n <= 16"), len(le16))
    H("corpus.MEAS-PQ2-MSOLVE-F4.eq_census_d_iter", text_num(ms, r"equal to the census d_iter on all (\d+)"), exact_ok)
    H("corpus.MEAS-PQ2-MSOLVE-F4.exactly_4_n17_34", text_num(ms, r"exactly 4 on all (\d+) instances with 17 <= n <= 34"), deg4)
    H("corpus.MEAS-PQ2-MSOLVE-F4.linear_eq_k", text_num(ms, r"exactly k linear polynomials on all (\d+)"), lin_ok)
    r53 = r5.get("R5-003", {}).get("statement", "")
    H("corpus.R5-003.never_below_census_bound", 0 if "is never below it" in r53 else None, below_lb)
    H("corpus.R5-003.matches_where_exact", len(exact) if "Matches the census lower bound on every instance where it is exact" in r53 else None, exact_ok)
    for doc, getter in (("docs/ecdlp-hardness-corpus/lean-round5-20261107.json", lambda: r5.get("R5-002", {}).get("statement", "")),
                        ("docs/ecdlp-hardness-corpus/WEAKNESSES.md", None),
                        ("docs/ecdlp-hardness-corpus/weakness.json", None)):
        txt = getter() if getter else " ".join(read_text(REPO / doc).split())
        tag = doc.rsplit("/", 1)[1]
        H(f"{tag}.R5-002.degree_one_above_KW", 1 if "Real F4 degree is 4, one above the Kousidis-Wiemers value 3" in txt else None,
          (next(iter({r["max_step_degree"] for r in band})) - 3) if distinct_17_34 == 1 else -1)
        H(f"{tag}.R5-002.constant_n17_34", 1 if "constant over n = 17 to 34" in txt else None, distinct_17_34)
        H(f"{tag}.R5-002.bounded_degree_premise_holds_on_range",
          1 if "the bounded-degree premise holds on this range" in txt else None, supported,
          note="a premise about all n cannot 'hold on a range'; " + why)
    for doc in ("docs/ecdlp-hardness-corpus/lean-round4-20261104.json", "docs/ecdlp-hardness-corpus/WEAKNESSES.md",
                "docs/ecdlp-hardness-corpus/weakness.json"):
        txt = " ".join(read_text(REPO / doc).split())
        tag = doc.rsplit("/", 1)[1]
        H(f"{tag}.R4-013.F4_degree_exceeds_2_n_le_44_under_containment",
          text_num(txt, r"Under the containment property \(THM-F4-CONTAINMENT\), F4's maximal step degree exceeds the first fall degree 2 on the (\d+) instances with n <= 44"),
          len(est44), note="lower bound via THM-F4-CONTAINMENT (Lean, not reviewed here)")


def r4013_exactly4(H, prefix, txt, d4exact, ge4):
    """R4-013's degree-4 clause in whichever wording the document carries."""
    old = text_num(txt, r"; exactly 4 on (\d+) fresh instances at n = 17 to 24")
    if old is not None:  # the solver degree stated as exactly 4 (second pass: unsupported)
        H(f"{prefix}.solver_degree_exactly_4_on_fresh", old, 0,
          note=("R4-013's subject is the solver degree. The census establishes the iterated closure degree d_iter = 4 "
                f"exactly on {d4exact} fresh instances, which gives solver degree >= 4 (under containment) and no upper "
                "bound. No artifact runs a solver on those 24 instances (f4/run_msolve.py was run on the main-sweep dumps "
                "f4/dumps, not on the iter4 sweeps; f4/ is in progress and was not validated). Instances where 'exactly 4' "
                "is established for the solver degree: 0. Supported wording: 'the iterated closure degree is exactly 4 "
                "(solver degree >= 4 under the containment property) on 24 fresh instances'."))
        return
    H(f"{prefix}.iterated_closure_exactly_4_on_fresh", text_num(txt, r"the iterated closure degree is exactly 4 \(solver degree >= 4 under the containment property\) on (\d+) fresh instances at n = 17 to 24"), d4exact,
      note="recomputed: fresh instances with iter3 < k and iter4 = k (d_iter = 4) in results/iter4_n17_20 and iter4_n21_24")
    H(f"{prefix}.solver_degree_ge_4_under_containment_on_fresh", text_num(txt, r"the iterated closure degree is exactly 4 \(solver degree >= 4 under the containment property\) on (\d+) fresh instances at n = 17 to 24"), ge4,
      note="recomputed: fresh instances with iter3 < k; a lower bound on F4's maximal step degree only under the "
           "containment property (separate Lean claim F4Containment, not validated here)")


def read_own(p: Path) -> bytes:
    """A validator-generated file (validation/): from the commit when it is tracked there and the
    run is bound to a commit, otherwise from disk; pinned either way."""
    r = rel(p)
    data = None
    if not WORKTREE:
        g = subprocess.run(["git", "-C", str(REPO), "show", f"{COMMIT}:{r}"], capture_output=True)
        if g.returncode == 0:
            data = g.stdout
            if hashlib.sha256(p.read_bytes()).hexdigest() != hashlib.sha256(data).hexdigest():
                WORKTREE_DIFFERS.append(r)
    if data is None:
        data = p.read_bytes()
    READ[r] = hashlib.sha256(data).hexdigest()
    return data


def corrected_items(H, readme, L, by_mono, by_own, est, open_keys, d3, d4, rust_cc, kd, names, sweep_good, detail,
                    version="corrected"):
    """Headline items for the corrected wording. Every 'claimed' value is parsed from the document
    text (None if the phrase is absent, which fails the item); every 'recomputed' value from data."""
    replaced = {"census_readme.d4_sweeps_reach_k", "report_typ.iterated_exactly_4", "report_typ.d4_sweeps_reach_k",
                "triage.d4_reaches_k", "WEAKNESSES.md.R4-013.solver_degree_exceeds_2_n_le_44",
                "weakness.json.R4-013.solver_degree_exceeds_2_n_le_44", "corpus.R4-013.solver_degree_exceeds_2_n_le_44"}
    if version == "final":
        H0 = H

        def H(name, claimed, recomputed, **extra):  # noqa: F811
            if name not in replaced:
                H0(name, claimed, recomputed, **extra)
    key = lambda r: (r["n"], r["inst"])
    n44 = {key(r) for r in L if r["n"] <= 44}
    est44 = est & n44
    own_only = {key(r) for r in by_own} - {key(r) for r in by_mono}
    iter2_vals = set()
    for f in ("closure_replay_n8_20.json", "closure_replay_n21_24.json"):
        for r in json.loads(read_own(HERE / f))["results"]:
            if (r["n"], r["instance"]) in own_only:
                iter2_vals.add(r["mine"]["iter2"])
    detail["corrected.iter2_values_on_the_24"] = sorted(iter2_vals)
    detail["corrected.all_n_le_44_established"] = est44 == n44
    t = readme
    # --- pq2_rank/README.md, cross-check paragraph
    H("pq2_readme.crosscheck.d3_agree", text_num(t, r"It agrees with Rust on (\w+) dumped instances at n = 14 to 18 at degree 3"),
      sum(nm in d3 and d3[nm]["low"] == rust_cc[nm]["iter3"] for nm in names))
    H("pq2_readme.crosscheck.d4_python_reaches_k", text_num(t, r"At degree 4 the Python closure reaches k on the (\w+) instances it was run on"),
      sum(d4[nm]["low"] == kd.get(nm) for nm in d4))
    first = read_text(PKG / "crosscheck/rust_same_instances.txt").splitlines()[0]
    disabled = sum(rust_cc[nm]["iter4"] is None for nm in names) if first.rstrip().endswith(" 18 0") else 0
    H("pq2_readme.crosscheck.rust_run_had_d4_disabled",
      5 if "the Rust run on those dumps had degree 4 disabled" in " ".join(t.split()) else None, disabled,
      note="recomputed: dumped instances whose Rust run has iter4 = '-', given the command ends '18 0' (iter4_max_n = 0)")
    cl = json.loads(read_own(HERE / "closure_dump_n14_18.json"))
    agree = 0
    for r in cl["results"]:
        nm = f"n{r['n']}_i{r['instance']}"
        same_dump = read_own(HERE / f"dump_n14_18/inst/{nm}.json").decode() == read_text(PKG / f"crosscheck/{nm}.json")
        if (same_dump and not r["failed_checks"] and r["rust"]["iter3"] == r["mine"]["iter3"] == rust_cc[nm]["iter3"]
                and r["rust"]["iter4"] is not None and r["rust"]["iter4"] == r["mine"]["iter4"]):
            agree += 1
    H("pq2_readme.crosscheck.validator_both_degrees_agree", text_num(t, r"The independent validator later ran both degrees on all (\w+) and found agreement"),
      agree, note="recomputed from validation/closure_dump_n14_18.json on dumps byte-identical to crosscheck/: "
                  "Rust iter3 = validator iter3 = committed Rust iter3, and validator-run Rust iter4 = validator iter4")
    # --- pq2_rank/README.md, Reading
    H("pq2_readme.reading.literal_fails_n_le_44", text_num(t, r"It fails on the (\d+) instances with n ≤ 44"), len(est44),
      note="d_iter >= 3 (iter2 < k) is established on every instance with n <= 44: " + str(est44 == n44))
    H("pq2_readme.reading.via_iter3_lt_k", text_num(t, r"On (\d+) of them iter3 < k already forces this"), len(by_mono))
    H("pq2_readme.reading.via_validator_iter2",
      text_num(t, r"on the other (\d+) the independent validator computed the degree-2 closure"), len(own_only),
      note="validator evidence: validation/closure_replay_n8_20.json (closure_check.py), pinned in raw_data")
    H("pq2_readme.reading.validator_iter2_value",
      text_num(t, r"computed the degree-2 closure \(iter2 = (\d+)\)"),
      next(iter(iter2_vals)) if len(iter2_vals) == 1 else -1)
    H("pq2_readme.reading.open_n45_47", text_num(t, r"so those (\d+) instances are open"),
      len([k for k in open_keys if 45 <= k[0] <= 47]) if all(45 <= k[0] <= 47 for k in open_keys) else -1)
    # --- crosscheck/README.md
    c = read_text(PKG / "crosscheck/README.md")
    H("crosscheck_readme.d3_agree_all", text_num(c, r"Rust and Python agree at degree 3 on all (\w+) instances"),
      sum(nm in d3 and d3[nm]["low"] == rust_cc[nm]["iter3"] for nm in names))
    rows = [[x.strip() for x in line.strip().strip("|").split("|")] for line in c.splitlines()
            if re.match(r"^\|\s*\d+\s*\|", line)]
    ok_rows, d4_cells, d4_match = 0, 0, 0
    rust_d4 = {f"n{r['n']}_i{r['instance']}": r["rust"]["iter4"] for r in cl["results"]}
    for cells in rows:
        nm = f"n{cells[0]}_i0"
        good = (nm in rust_cc and cells[1] == str(kd.get(nm)) and cells[2] == str(rust_cc[nm]["iter3"])
                and cells[3] == str(d3.get(nm, {}).get("low"))
                and cells[4] == (str(d4[nm]["low"]) if nm in d4 else ""))
        ok_rows += good
        if cells[4]:
            d4_cells += 1
            d4_match += cells[4] == str(rust_d4.get(nm))
    H("crosscheck_readme.table_rows_match_data", len(rows), ok_rows)
    H("crosscheck_readme.validator_rust_d4_matches_column",
      d4_cells if "re-ran Rust with degree 4 on the same dumps and it matches" in " ".join(c.split()) else None, d4_match)
    # --- census README, report.typ, triage, corpus
    sweeps = []
    for tag in ("iter4_n17_20", "iter4_n21_24"):
        sweeps += json.loads(read_text(PKG / f"results/{tag}/pq2_rank.json"))["rows"]
    d4exact = sum(r["d_iter"] == "4" and r["iter3"] < r["k"] and r["iter4"] == r["k"] for r in sweeps)
    ge4 = sum(r["iter3"] is not None and r["iter3"] < r["k"] for r in sweeps)
    lo1 = sum((r["n"] == 15 or 17 <= r["n"] <= 44) and r["iter3"] == 1 for r in L)
    lo1_17 = sum(17 <= r["n"] <= 44 and r["iter3"] == 1 for r in L)
    cen = read_text(PKG.parent / "README.md")
    H("census_readme.instances", text_num(cen, r"The sweep covers (\d+) instances, n = 8 to 47"), len(L))
    H("census_readme.iter3_eq_1_n15_and_17_44", text_num(cen, r"from 17 to 44 \((\d+) instances\) it reaches only 1"), lo1)
    H("census_readme.d4_sweeps_reach_k", text_num(cen, r"Fresh degree-4 sweeps at n = 17 to 24 reach k on all (\d+) instances"), sweep_good)
    H("census_readme.literal_fails_n_le_44", text_num(cen, r"Literal Assumption 2 \(degree 2\) fails on all (\d+) instances with n ≤ 44"), len(est44))
    typ = read_text(PKG.parent / "report.typ")
    H("report_typ.instances", text_num(typ, r"(\d+) instances, n = 8 to 47, three per n"), len(L))
    H("report_typ.literal_fails_n_le_44", text_num(typ, r"literal reading fails on the (\d+) instances with \$n <= 44\$"), len(est44))
    H("report_typ.iterated_exactly_4", text_num(typ, r"exactly 4 on (\d+) fresh instances at n = 17--24"), d4exact)
    H("report_typ.iter3_eq_1_n17_44", text_num(typ, r"reaches only 1 at n = 15 and on all (\d+) instances from n = 17 to 44"), lo1_17)
    H("report_typ.d4_sweeps_reach_k", text_num(typ, r"Fresh degree-4 sweeps at n = 17 to 24 reach k on all (\d+) instances"), sweep_good)
    H("report_typ.python_agrees_five", text_num(typ, r"agrees with the Rust closure on (\w+) instances"),
      sum(nm in d3 and d3[nm]["low"] == rust_cc[nm]["iter3"] for nm in names))
    H("report_typ.fall_degree_not_solver_degree_n_le_44",
      len(n44) if "is not the solver degree on any instance with $n <= 44$" in " ".join(typ.split()) else None, len(est44))
    tri = read_text(REPO / "docs/ecdlp-hardness-corpus/WEAKNESS_TRIAGE.md")
    H("triage.instances", text_num(tri, r"Assumption 2 \(`pq2_rank`\)\.\*\* (\d+) instances, n = 8 to 47"), len(L))
    H("triage.iter3_eq_1_n17_44", text_num(tri, r"only 1 of them at n = 15 and on all (\d+) instances from n = 17 to 44"), lo1_17)
    H("triage.d4_reaches_k", text_num(tri, r"Degree 4 reaches k on (\d+) of \d+ fresh instances at n = 17 to 24"), sweep_good)
    H("triage.literal_fails_n_le_44", text_num(tri, r"The literal reading \(degree 2\) fails on the (\d+) instances with n ≤ 44"), len(est44))
    corp = json.loads(read_text(REPO / "docs/ecdlp-hardness-corpus/lean-round4-20261104.json"))
    flat = []

    def walk(o):
        if isinstance(o, dict):
            flat.append(o)
            for v in o.values():
                walk(v)
        elif isinstance(o, list):
            for v in o:
                walk(v)
    walk(corp)
    e13 = next((o.get("statement", "") for o in flat if o.get("id") == "R4-013"), "")
    node = next((o.get("statement", "") for o in flat if o.get("id") == "MEAS-PQ2-DLIN-CENSUS"), "")
    H("corpus.MEAS-PQ2-DLIN-CENSUS.instances", text_num(node, r"(\d+) instances for n = 8 to 47"), len(L))
    H("corpus.MEAS-PQ2-DLIN-CENSUS.iter3_eq_1_n17_44", text_num(node, r"on all (\d+) instances with 17 <= n <= 44"), lo1_17)
    H("corpus.MEAS-PQ2-DLIN-CENSUS.d4_reaches_k", text_num(node, r"On (\d+) fresh instances at n = 17 to 24 the degree-4 closure reaches k"), sweep_good)
    H("corpus.MEAS-PQ2-DLIN-CENSUS.low2_eq_1_every_instance",
      len(L) if "reaches 1 linear polynomial at degree 2 on every instance" in node else None, sum(r["low2"] == 1 for r in L))
    for doc in ("docs/ecdlp-hardness-corpus/WEAKNESSES.md", "docs/ecdlp-hardness-corpus/weakness.json"):
        txt = read_text(REPO / doc)
        tag = doc.rsplit("/", 1)[1]
        H(f"{tag}.R4-013.solver_degree_exceeds_2_n_le_44",
          text_num(txt, r"The solver degree exceeds the first fall degree 2 on the (\d+) instances with n <= 44"), len(est44))
        r4013_exactly4(H, f"{tag}.R4-013", txt, d4exact, ge4)
    H("corpus.R4-013.solver_degree_exceeds_2_n_le_44",
      text_num(e13, r"The solver degree exceeds the first fall degree 2 on the (\d+) instances with n <= 44"), len(est44),
      note="a lower bound on F4's maximal step degree only under the containment property (separate Lean claim, not validated here)")
    r4013_exactly4(H, "corpus.R4-013", e13, d4exact, ge4)

# ---------------------------------------------------------------------------

def main_by_key(rows):
    return {(r["n"], r["inst"]): r for r in rows}


WORDS = {"one": 1, "two": 2, "three": 3, "four": 4, "five": 5, "six": 6, "seven": 7, "eight": 8}


def text_num(text: str, pattern: str, group: int = 1):
    """The number a document states, matched on whitespace-normalised text (None if absent)."""
    m = re.search(pattern, " ".join(text.split()))
    if not m:
        return None
    g = m[group]
    return int(g) if g.isdigit() else WORDS.get(g.lower())


def main() -> int:
    global COMMIT, WORKTREE
    import argparse
    ap = argparse.ArgumentParser()
    ap.add_argument("--commit", default=COMMIT)
    ap.add_argument("--claims", choices=["original", "corrected", "final", "final2"], default="original")
    ap.add_argument("--worktree", action="store_true")
    ap.add_argument("--out", default=str(HERE / "validator_report.json"))
    args = ap.parse_args()
    COMMIT, WORKTREE = args.commit, args.worktree
    heads: list[dict] = []
    notes: list[str] = []
    detail: dict = {}

    def H(name, claimed, recomputed, **extra):
        item = {"name": name, "claimed": claimed, "recomputed": recomputed, "rel_tol": 0}
        item.update(extra)
        heads.append(item)

    readme = read_text(PKG / "README.md")  # the claims source, pinned

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
        for r in json.loads(read_own(HERE / f))["results"]:
            own[(r["n"], r["instance"])] = r["mine"]
    by_own = [r for r in L if (r["n"], r["inst"]) in own and own[(r["n"], r["inst"])]["iter2"] < r["k"]
              and own[(r["n"], r["inst"])]["k"] == r["k"]]
    est = {(r["n"], r["inst"]) for r in by_mono} | {(r["n"], r["inst"]) for r in by_own}
    open_keys = sorted(k for k in main_by_key(L) if k not in est)
    detail["reading_established"] = {"by_iter3_lt_k": len(by_mono), "total": len(est), "open": open_keys}
    if args.claims == "original":
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
    if args.claims == "original":
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

    if args.claims in ("corrected", "final", "final2"):
        corrected_items(H, readme, L, by_mono, by_own, est, open_keys, d3, d4, rust_cc, kd, names, both, detail,
                        version="final" if args.claims == "final2" else args.claims)
    if args.claims == "final":
        final_items(H, readme, L, est, detail)
    if args.claims == "final2":
        # sentences withdrawn after the final pass are not re-claimed; their absence is checked in final2_items
        withdrawn = {"pq2_readme.direction_one_above_prediction", "pq2_readme.direction_no_growth_n17_34",
                     "pq2_readme.direction_evidence_for_bounded_degree_premise", "census_readme.no_growth_n17_34",
                     "report_typ.no_growth_n17_34", "triage.degree_stays_4_n17_34"}
        for doc in ("lean-round5-20261107.json", "WEAKNESSES.md", "weakness.json"):
            withdrawn |= {f"{doc}.R5-002.degree_one_above_KW", f"{doc}.R5-002.constant_n17_34",
                          f"{doc}.R5-002.bounded_degree_premise_holds_on_range"}

        def Hf(name, claimed, recomputed, **extra):
            if name not in withdrawn:
                H(name, claimed, recomputed, **extra)
        final_items(Hf, readme, L, est, detail)
        final2_items(H, readme, L, detail)

    # ---------------------------------------------------------------------------
    failed = [h for h in heads if h["claimed"] != h["recomputed"]]
    status = "pass" if not failed else "fail"
    notes.append(f"{len(heads)} headline items recomputed; {len(failed)} do not match: "
                 + "; ".join(f"{h['name']} claimed {h['claimed']} recomputed {h['recomputed']}" for h in failed))
    notes.append("Counts parsed directly from results/run.log by the validator's own parser; derived JSON/CSV files "
                 "checked row by row against their source logs. Rounded claims (F_over_V) are compared at the "
                 "precision stated in the README; exact values are in recomputed_exact.")
    head = subprocess.run(["git", "-C", str(REPO), "rev-parse", "HEAD"], capture_output=True, text=True).stdout.strip()
    if WORKTREE:
        moved = [r for r, h in READ.items() if hashlib.sha256((REPO / r).read_bytes()).hexdigest() != h]
        dirty = subprocess.run(["git", "-C", str(REPO), "status", "--porcelain", "--", *sorted(READ)],
                               capture_output=True, text=True).stdout.splitlines()
        preview = {
            "schema": "validator.worktree-preview/v1",
            "status": "refused",
            "refusal": ("Not a validation verdict. The validator validates only a frozen, committed artifact set; "
                        "these inputs are uncommitted working-tree files in a tree another session is committing "
                        "to. Commit them and re-run with --commit <sha> --claims corrected for a binding report."),
            "preview_all_headlines_match": not failed,
            "preview_mismatches": [{"name": h["name"], "claimed": h["claimed"], "recomputed": h["recomputed"]}
                                   for h in failed],
            "base_head": head,
            "uncommitted_inputs": dirty,
            "inputs_moved_during_run": moved,
            "validator": VALIDATOR,
            "produced_at": dt.datetime.now(dt.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
            "raw_data": [{"path": p, "sha256": h} for p, h in sorted(READ.items())],
            "headline": heads,
            "notes": " ".join(notes),
            "detail": detail,
        }
        (HERE / "validator_report.worktree_preview.json").write_text(json.dumps(preview, indent=1) + "\n")
        for h in heads:
            flag = "ok " if h["claimed"] == h["recomputed"] else "BAD"
            print(f"{flag} {h['name']}: claimed {h['claimed']} recomputed {h['recomputed']}")
        print("preview (refused as a verdict); all headlines match:", not failed, "| moved:", moved,
              "| uncommitted inputs:", len(dirty))
        return 2
    notes.append(f"All inputs read from commit {COMMIT} (git object store). Working-tree copies differing from it: "
                 f"{WORKTREE_DIFFERS or 'none'}. Repository HEAD at recompute time: {head}.")
    if WORKTREE_DIFFERS:
        status = "fail"
    report = {
        "schema": "pcc.validator-report/v1",
        "validated_commit": COMMIT,
        "claims": args.claims,
        "recompute_script_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
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
    Path(args.out).write_text(json.dumps(report, indent=1) + "\n")
    for h in heads:
        flag = "ok " if h["claimed"] == h["recomputed"] else "BAD"
        print(f"{flag} {h['name']}: claimed {h['claimed']} recomputed {h['recomputed']}")
    print("status:", status, "| files pinned:", len(READ))
    return 0 if status == "pass" else 1


if __name__ == "__main__":
    sys.exit(main())
