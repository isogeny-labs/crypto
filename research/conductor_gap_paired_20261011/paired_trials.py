#!/usr/bin/env python3
"""Paired relation-cost trials over F_{p^2} for the pairs selected by enumerate_pairs.py.

  python3 paired_trials.py --mode main      # runs.jsonl, paired_results.csv, analysis.json, environment.json
  python3 paired_trials.py --mode replay    # runs_replay.jsonl, replay_report.json (G1 reproducibility)

Design, endpoints and decision rules are frozen in PROTOCOL.md (sections 5-7, Amendment 2).
Python prototype under a task-level exception to the no-Python rule (PROTOCOL.md s.1).
"""
from __future__ import annotations

import argparse
import csv
import gc
import json
import math
import os
import platform
import random
import resource
import subprocess
import sys
import time

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)

from cgp_field import Fq  # noqa: E402
from cgp_curve import Curve  # noqa: E402
from cgp_engine import (  # noqa: E402
    Echelon, Instance, decompose_exhaustive, decompose_weil_gb, fingerprint, relation_row,
)
from cgp_stats import (  # noqa: E402
    bootstrap_ci, holm, pooled_stratified_bootstrap, sign_flip_pvalue,
)

ENGINES = ("exhaustive", "weil_gb")
SETUP_PHASES = ("setup", "verification_setup")
ATTEMPT_PHASES = ("target_generation", "decompose", "verify", "linear_algebra")
UNIT_SHUFFLE_SEED = 20261026
REPLAY_SHUFFLE_SEED = 20261028


class Timer:
    def __init__(self, phases):
        self.cpu = {p: 0.0 for p in phases}
        self.wall = {p: 0.0 for p in phases}
        self._c = time.process_time()
        self._w = time.perf_counter()

    def tick(self, phase):
        c, w = time.process_time(), time.perf_counter()
        self.cpu[phase] += c - self._c
        self.wall[phase] += w - self._w
        self._c, self._w = c, w


def sample_P0(E: Curve, h: int, seed):
    """Deterministic P0 and G = [h]P0 != O (rejection sampling), stream Random('<seed>:G')."""
    F = E.F
    rng = random.Random(f"{seed}:G")
    while True:
        x = rng.randrange(F.q)
        y = F.sqrt(E.f(x))
        if y is None:
            continue
        if rng.random() < 0.5:
            y = F.neg(y)
        P0 = (x, y)
        if E.mul(h, P0) is not None:
            return P0


def run_one(F: Fq, spec: dict, comp_id: str, seed: int, engine: str, T: int, M: int,
            iso_ctx: dict | None, order_pos: int, unit_index: int, emit):
    """One run = one curve, one seed, one engine, T attempts.  Calls emit(record) for the
    setup line, every attempt line and the summary line.  Returns the summary."""
    gc.collect()
    gc.disable()
    c_start, w_start = time.process_time(), time.perf_counter()
    tm = Timer(SETUP_PHASES)
    a, b = spec["a"], spec["b"]
    r, h = spec["r"], spec["cofactor"]
    inst = Instance(F, a, b, M)
    if engine == "exhaustive":
        inst.pair_constants()
    E = inst.E
    if iso_ctx is not None:  # model run: image of the original curve's P0 under (x, y) -> (u^2 x, u^3 y)
        P0 = (F.mul(iso_ctx["u2"], iso_ctx["P0"][0]), F.mul(iso_ctx["u3"], iso_ctx["P0"][1]))
    else:
        P0 = sample_P0(E, h, seed)
    G = E.mul(h, P0)
    assert G is not None and E.on_curve(G)
    rng_s = random.Random(f"{seed}:S")
    d = rng_s.randrange(1, r)
    H = E.mul(d, G)
    # columns: canonical class of [h]P (up to sign) for every base point
    colinfo: dict[int, tuple[int | None, int]] = {}
    colkeys: dict[tuple, int] = {}
    colpts: list = []
    for (x, y) in inst.base:
        Q = E.mul(h, (x, y))
        if Q is None:
            colinfo[x] = (None, 0)
            continue
        ny = F.neg(Q[1])
        if Q[1] <= ny:
            canon, sign = Q, 1
        else:
            canon, sign = (Q[0], ny), -1
        key = (canon[0], canon[1])
        if key not in colkeys:
            colkeys[key] = len(colpts)
            colpts.append(canon)
        colinfo[x] = (colkeys[key], sign)
    ncols = len(colpts) + 2
    colG, colH = len(colpts), len(colpts) + 1
    tm.tick("setup")
    # discrete-log table of <G>, for exact verification of every relation
    table = {None: 0}
    cur = None
    for k in range(1, r):
        cur = E.add(cur, G)
        table[cur] = k
    logC = [table[Q] for Q in colpts]
    tm.tick("verification_setup")
    setup_cpu, setup_wall = dict(tm.cpu), dict(tm.wall)
    emit({"record": "run_setup", "comparison": comp_id, "curve": spec["id"], "seed": seed,
          "engine": engine, "unit_index": unit_index, "order_position": order_pos, "T": T, "M": M,
          "full_fb_size": inst.full_size, "r": r, "cofactor": h, "ncols": ncols,
          "n_distinct_columns": len(colpts), "d_secret": d, "G": list(G), "H": list(H),
          "P0": list(P0), "iso_model": iso_ctx is not None,
          "setup_cpu_s": setup_cpu, "setup_wall_s": setup_wall})

    ech = Echelon(r, ncols)
    tot = {"relations": 0, "invalid": 0, "verification_failed": 0, "outside_base": 0,
           "not_rational": 0, "duplicate": 0, "dependent": 0, "rank_inc": 0, "failed": 0,
           "identity": 0, "ops": 0}
    phase_cpu = {p: 0.0 for p in ATTEMPT_PHASES}
    phase_wall = {p: 0.0 for p in ATTEMPT_PHASES}
    degrees, qdims = [], []
    target_sets, rel_items = [], []
    for j in range(T):
        at = Timer(ATTEMPT_PHASES)
        aj = rng_s.randrange(1, r)
        bj = rng_s.randrange(1, r)
        R = E.add(E.mul(aj, G), E.mul(bj, H))
        at.tick("target_generation")
        line = {"record": "attempt", "comparison": comp_id, "curve": spec["id"], "seed": seed,
                "engine": engine, "j": j, "a_j": aj, "b_j": bj, "R": None if R is None else list(R)}
        if R is None:
            tot["identity"] += 1
            tot["failed"] += 1
            line.update({"outcome": "identity_target", "relations_verified": 0, "rank_increments": 0,
                         "invalid": 0, "system": None})
            target_sets.append((j, ()))
            for p_ in ATTEMPT_PHASES:
                phase_cpu[p_] += at.cpu[p_]
                phase_wall[p_] += at.wall[p_]
            line["cpu_s"], line["wall_s"] = at.cpu, at.wall
            emit(line)
            continue
        if engine == "exhaustive":
            cls, met = decompose_exhaustive(inst, R[0])
            system = {"unknowns": 2, "pairs_evaluated": met["pair_evals"]}
            tot["ops"] += met["pair_evals"]
        else:
            cls, met = decompose_weil_gb(inst, R[0])
            system = {k: met[k] for k in ("variables", "equations", "input_degrees", "quotient_dim",
                                          "solving_degree", "grevlex_max_spoly_deg",
                                          "grevlex_max_elem_deg", "grevlex_spairs",
                                          "lex_max_spoly_deg", "non_zero_dim", "ops")}
            tot["ops"] += met["ops"]
            degrees.append(met["solving_degree"])
            if met["quotient_dim"] is not None:
                qdims.append(met["quotient_dim"])
        at.tick("decompose")
        tot["outside_base"] += len(cls["outside_base"])
        tot["not_rational"] += len(cls["not_rational"])
        tot["invalid"] += len(cls["not_rational"])
        n_ok = n_inv = n_ind = n_dup = n_dep = 0
        rows = []
        for (x1, x2) in cls["in_base"]:
            P1, P2 = (x1, inst.base_y[x1]), (x2, inst.base_y[x2])
            good, e1, e2, row = relation_row(E, R, P1, P2, x1, x2, colinfo, logC, ncols, colG,
                                             colH, h, aj, bj, d, r)
            if good:
                rows.append((x1, x2, e1, e2, row))
                n_ok += 1
            else:
                n_inv += 1
                if e1 is not None:  # group law held but the discrete-log identity failed
                    tot["verification_failed"] += 1
        at.tick("verify")
        for (x1, x2, e1, e2, row) in rows:
            st = ech.add(row)
            if st == "independent":
                n_ind += 1
            elif st == "duplicate":
                n_dup += 1
            else:
                n_dep += 1
            rel_items.append((j, x1, x2, e1, e2, st))
        at.tick("linear_algebra")
        tot["relations"] += n_ok
        tot["invalid"] += n_inv
        tot["duplicate"] += n_dup
        tot["dependent"] += n_dep
        tot["rank_inc"] += n_ind
        if not cls["in_base"]:
            tot["failed"] += 1
        target_sets.append((j, tuple(cls["in_base"])))
        for p_ in ATTEMPT_PHASES:
            phase_cpu[p_] += at.cpu[p_]
            phase_wall[p_] += at.wall[p_]
        line.update({"outcome": "relation" if n_ok else "no_decomposition",
                     "in_base_solutions": [list(s) for s in cls["in_base"]],
                     "outside_base": len(cls["outside_base"]), "not_rational": len(cls["not_rational"]),
                     "relations_verified": n_ok, "invalid": n_inv + len(cls["not_rational"]),
                     "rank_increments": n_ind, "duplicates": n_dup, "dependent": n_dep,
                     "system": system, "cpu_s": at.cpu, "wall_s": at.wall})
        emit(line)
    total_cpu = time.process_time() - c_start
    total_wall = time.perf_counter() - w_start
    gc.enable()
    rank = ech.rank
    y_cpu = math.log(total_cpu / rank) if rank > 0 else None
    y_wall = math.log(total_wall / rank) if rank > 0 else None
    summary = {
        "record": "run_summary", "comparison": comp_id, "curve": spec["id"], "seed": seed,
        "engine": engine, "unit_index": unit_index, "order_position": order_pos, "T": T, "M": M,
        "full_fb_size": inst.full_size, "r": r, "cofactor": h, "ncols": ncols,
        "total_cpu_s": total_cpu, "total_wall_s": total_wall,
        "phase_cpu_s": {**setup_cpu, **phase_cpu}, "phase_wall_s": {**setup_wall, **phase_wall},
        "relations_verified": tot["relations"], "rank_increments": rank,
        "dependent": tot["dependent"], "duplicate": tot["duplicate"],
        "invalid_solutions": tot["invalid"], "verification_failed": tot["verification_failed"],
        "not_rational": tot["not_rational"], "outside_base": tot["outside_base"],
        "failed_attempts": tot["failed"], "identity_targets": tot["identity"],
        "censored": rank == 0,
        "max_solving_degree": max(degrees) if degrees else None,
        "mean_solving_degree": float(np.mean(degrees)) if degrees else None,
        "mean_quotient_dim": float(np.mean(qdims)) if qdims else None,
        "ops_total": tot["ops"], "y_cpu": y_cpu, "y_wall": y_wall,
        "targets_fingerprint": fingerprint(target_sets),
        "relations_fingerprint": fingerprint(rel_items),
        "maxrss_kb_process_cumulative": resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,
    }
    emit(summary)
    return summary


def load_inputs(out_dir):
    receipt = json.load(open(os.path.join(out_dir, "selection_receipt.json")))
    wanted = set()
    for pr in receipt["trial_pairs"]:
        wanted.add(pr["lo_id"])
        wanted.add(pr["hi_id"])
        if pr["same_conductor_control_id"]:
            wanted.add(pr["same_conductor_control_id"])
    curves = {}
    with open(os.path.join(out_dir, "curves.jsonl")) as fh:
        for line in fh:
            r = json.loads(line)
            if r["id"] in wanted:
                curves[r["id"]] = r
    return receipt, curves


def spec_of(rec):
    return {"id": rec["id"], "a": rec["a"], "b": rec["b"], "p": rec["p"], "r": rec["r"],
            "cofactor": rec["cofactor"], "N": rec["N"], "t": rec["t"], "j": rec["j"],
            "fE": rec["fE"], "fb_size": rec.get("fb_size")}


def build_comparisons(receipt, curves, fields):
    comps = []
    for pi, pr in enumerate(receipt["trial_pairs"]):
        F = fields[pr["p"]]
        lo, hi = spec_of(curves[pr["lo_id"]]), spec_of(curves[pr["hi_id"]])
        lo["fb_size"], hi["fb_size"] = pr["fb_size_lo"], pr["fb_size_hi"]
        comps.append({"id": f"P{pi}-pair", "kind": "pair", "pair_index": pi, "group": pr["group"],
                      "A": lo, "B": hi, "M": min(lo["fb_size"], hi["fb_size"]), "iso": None})
        # isomorphic model of lo: (u^4 a, u^6 b), u = theta (the integer p), u^2 = n in F_p
        u = F.p
        u2 = F.mul(u, u)
        assert F.in_fp(u2)
        u3 = F.mul(u2, u)
        u4 = F.mul(u2, u2)
        u6 = F.mul(u4, u2)
        iso = dict(lo)
        iso["id"] = lo["id"] + "-isotheta"
        iso["a"], iso["b"] = F.mul(u4, lo["a"]), F.mul(u6, lo["b"])
        comps.append({"id": f"P{pi}-iso", "kind": "iso", "pair_index": pi, "group": pr["group"],
                      "A": lo, "B": iso, "M": lo["fb_size"], "iso": {"u": u, "u2": u2, "u3": u3}})
        cid = pr["same_conductor_control_id"]
        if cid:
            ctrl = spec_of(curves[cid])
            ctrl["fb_size"] = pr["same_conductor_control_fb_size"]
            A, B = (lo, ctrl) if lo["j"] < ctrl["j"] else (ctrl, lo)
            comps.append({"id": f"P{pi}-same", "kind": "same", "pair_index": pi, "group": pr["group"],
                          "A": A, "B": B, "M": min(lo["fb_size"], ctrl["fb_size"]), "iso": None})
    return comps


def environment(out_dir, command):
    def run(*a):
        try:
            return subprocess.check_output(a, cwd=HERE, text=True, stderr=subprocess.DEVNULL).strip()
        except Exception:
            return None
    cpu = None
    try:
        for line in open("/proc/cpuinfo"):
            if line.startswith("model name"):
                cpu = line.split(":", 1)[1].strip()
                break
    except OSError:
        pass
    return {"command": command, "python": sys.version, "numpy": np.__version__,
            "platform": platform.platform(), "cpu_model": cpu, "logical_cpus": os.cpu_count(),
            "loadavg_start": os.getloadavg(), "git_head": run("git", "rev-parse", "HEAD"),
            "git_dirty": (run("git", "status", "--porcelain", "--", ".") or "").splitlines(),
            "solver": "pure-Python Buchberger (grevlex + lex) and exhaustive S_3 enumeration; "
                      "no F4/F5/SAT, no external CAS",
            "excluded_costs": ["F_{p^2} log/exp table construction (shared by all curves)",
                               "selection and certification (selection_receipt.json)",
                               "regeneration of the original P0 for the model control"],
            "timers": "time.process_time (CPU, primary) and time.perf_counter (wall)"}


def execute(args, mode):
    out_dir = args.out
    receipt, curves = load_inputs(out_dir)
    primes = sorted({pr["p"] for pr in receipt["trial_pairs"]})
    fields = {p: Fq(p) for p in primes}
    comps = build_comparisons(receipt, curves, fields)
    seeds = list(range(1, (3 if mode == "replay" else args.seeds) + 1))
    units = [(ci, s, e) for ci in range(len(comps)) for s in seeds for e in ENGINES]
    random.Random(REPLAY_SHUFFLE_SEED if mode == "replay" else UNIT_SHUFFLE_SEED).shuffle(units)
    fname = "runs_replay.jsonl" if mode == "replay" else "runs.jsonl"
    path = os.path.join(out_dir, fname)
    if os.path.exists(path):
        raise SystemExit(f"{path} exists; raw outputs are never overwritten (move it or use a new --out)")
    env = environment(out_dir, "python3 " + " ".join(sys.argv))
    env["mode"] = mode
    env["n_units"] = len(units)
    summaries = []
    t0 = time.perf_counter()
    with open(path, "w") as fh:
        def emit(rec):
            fh.write(json.dumps(rec) + "\n")
        emit({"record": "environment", **env})
        for ui, (ci, s, e) in enumerate(units):
            comp = comps[ci]
            flip = random.Random(s ^ 0x5eed ^ (ci * 7919) ^ (0 if e == "exhaustive" else 1)).random() < 0.5
            order = [comp["A"], comp["B"]] if flip else [comp["B"], comp["A"]]
            iso_orig_P0 = None
            if comp["kind"] == "iso":
                F = fields[comp["A"]["p"]]
                Eo = Curve(F, comp["A"]["a"], comp["A"]["b"])
                iso_orig_P0 = sample_P0(Eo, comp["A"]["cofactor"], s)
            for pos, spec in enumerate(order):
                is_model = comp["kind"] == "iso" and spec is comp["B"]
                iso_ctx = None
                if is_model:
                    iso_ctx = {**comp["iso"], "P0": iso_orig_P0}
                F = fields[spec["p"]]
                summ = run_one(F, spec, comp["id"], s, e, args.attempts, comp["M"], iso_ctx, pos, ui, emit)
                summ["role"] = "A" if spec is comp["A"] else "B"
                summaries.append(summ)
            fh.flush()
    env["loadavg_end"] = os.getloadavg()
    env["elapsed_wall_s"] = time.perf_counter() - t0
    env["cpu_seconds_process"] = time.process_time()
    with open(os.path.join(out_dir, "environment.json" if mode == "main" else "environment_replay.json"), "w") as fh:
        json.dump(env, fh, indent=1)
    return receipt, comps, summaries


def cross_engine_check(summaries):
    by = {}
    for s in summaries:
        by.setdefault((s["curve"], s["seed"], s["M"]), {})[s["engine"]] = s["targets_fingerprint"]
    bad = [k for k, v in by.items() if len(v) == 2 and v["exhaustive"] != v["weil_gb"]]
    return {"keys_checked": sum(1 for v in by.values() if len(v) == 2), "mismatches": [list(k) for k in bad]}


def iso_image_check(path, comps, fields):
    """For every iso comparison: the model's in-base solution set at each attempt must be the
    image {(n x1, n x2)} of the original's (u = theta, u^2 = n in F_p).  Returns counts."""
    iso = {c["id"]: c for c in comps if c["kind"] == "iso"}
    got: dict = {}
    with open(path) as fh:
        for line in fh:
            r = json.loads(line)
            if r["record"] == "attempt" and r["comparison"] in iso and "in_base_solutions" in r:
                got[(r["comparison"], r["seed"], r["engine"], r["curve"], r["j"])] = r["in_base_solutions"]
    checked = bad = 0
    for cid, c in iso.items():
        F = fields[c["A"]["p"]]
        n = F.n
        for (cc, seed, eng, curve, j), sols in list(got.items()):
            if cc != cid or curve != c["A"]["id"]:
                continue
            other = got.get((cid, seed, eng, c["B"]["id"], j))
            img = sorted(sorted(((n * x1) % F.p, (n * x2) % F.p)) for x1, x2 in sols)
            if other is None:
                continue
            checked += 1
            if img != sorted(sorted(s_) for s_ in other):
                bad += 1
    return {"attempts_checked": checked, "mismatches": bad}


def write_csv(path, comps, summaries):
    idx = {(s["comparison"], s["seed"], s["engine"], s["role"]): s for s in summaries}
    rows = []
    for comp in comps:
        for e in ENGINES:
            seeds = sorted({s["seed"] for s in summaries if s["comparison"] == comp["id"]})
            for sd in seeds:
                a, b = idx.get((comp["id"], sd, e, "A")), idx.get((comp["id"], sd, e, "B"))
                if not a or not b:
                    continue
                dcpu = (b["y_cpu"] - a["y_cpu"]) if (a["y_cpu"] is not None and b["y_cpu"] is not None) else ""
                dwall = (b["y_wall"] - a["y_wall"]) if (a["y_wall"] is not None and b["y_wall"] is not None) else ""
                rows.append({
                    "comparison": comp["id"], "kind": comp["kind"], "group": comp["group"], "engine": e,
                    "seed": sd, "curve_A": a["curve"], "curve_B": b["curve"],
                    "fE_A": comp["A"]["fE"], "fE_B": comp["B"]["fE"] if comp["kind"] != "iso" else comp["A"]["fE"],
                    "M": comp["M"], "order_first": "A" if a["order_position"] == 0 else "B",
                    "cpu_A": a["total_cpu_s"], "cpu_B": b["total_cpu_s"],
                    "wall_A": a["total_wall_s"], "wall_B": b["total_wall_s"],
                    "rank_A": a["rank_increments"], "rank_B": b["rank_increments"],
                    "relations_A": a["relations_verified"], "relations_B": b["relations_verified"],
                    "failed_A": a["failed_attempts"], "failed_B": b["failed_attempts"],
                    "maxdeg_A": a["max_solving_degree"], "maxdeg_B": b["max_solving_degree"],
                    "ops_A": a["ops_total"], "ops_B": b["ops_total"],
                    "y_cpu_A": a["y_cpu"], "y_cpu_B": b["y_cpu"], "diff_y_cpu": dcpu,
                    "y_wall_A": a["y_wall"], "y_wall_B": b["y_wall"], "diff_y_wall": dwall,
                    "censored_A": a["censored"], "censored_B": b["censored"],
                    "counts_equal_AB": (a["relations_verified"], a["rank_increments"], a["failed_attempts"])
                                       == (b["relations_verified"], b["rank_increments"], b["failed_attempts"]),
                })
    with open(path, "w", newline="") as fh:
        w = csv.DictWriter(fh, fieldnames=list(rows[0]))
        w.writeheader()
        w.writerows(rows)
    return rows


def analyse(rows, comps):
    out = {"schema": "cgp.analysis/v1", "endpoint": "y = ln(total CPU seconds / rank increments); "
           "diff = y(B) - y(A); pair: B has the larger f_E; iso: B is the isomorphic model; "
           "same: B has the larger integer j", "engines": {}}
    for e in ENGINES:
        res = {}
        for comp in comps:
            sel = [r for r in rows if r["comparison"] == comp["id"] and r["engine"] == e]
            ok = [r for r in sel if r["diff_y_cpu"] != ""]
            d = [r["diff_y_cpu"] for r in ok]
            dw = [r["diff_y_wall"] for r in sel if r["diff_y_wall"] != ""]
            ci = bootstrap_ci(d)
            perm = sign_flip_pvalue(d) if d else {"p": None}
            ciw = bootstrap_ci(dw)
            permw = sign_flip_pvalue(dw) if dw else {"p": None}
            npos = int(sum(1 for x in d if x > 0))
            res[comp["id"]] = {
                "kind": comp["kind"], "pair_index": comp["pair_index"], "group": comp["group"],
                "curve_A": comp["A"]["id"], "curve_B": comp["B"]["id"], "M": comp["M"],
                "fE_A": comp["A"]["fE"], "fE_B": comp["B"]["fE"] if comp["kind"] != "iso" else comp["A"]["fE"],
                "n_seeds": len(sel), "n_censored_runs": int(sum(1 for r in sel if r["censored_A"] or r["censored_B"])),
                "primary_cpu": {"ci95": ci, "permutation": perm, "n_positive": npos, "n_negative": len(d) - npos,
                                "ratio_of_total_cost_geo_mean": math.exp(ci["mean"]) if ci["mean"] is not None else None},
                "sensitivity_wall": {"ci95": ciw, "permutation": permw},
                "secondary": {
                    "mean_relations_A": float(np.mean([r["relations_A"] for r in sel])) if sel else None,
                    "mean_relations_B": float(np.mean([r["relations_B"] for r in sel])) if sel else None,
                    "mean_rank_A": float(np.mean([r["rank_A"] for r in sel])) if sel else None,
                    "mean_rank_B": float(np.mean([r["rank_B"] for r in sel])) if sel else None,
                    "mean_failed_A": float(np.mean([r["failed_A"] for r in sel])) if sel else None,
                    "mean_failed_B": float(np.mean([r["failed_B"] for r in sel])) if sel else None,
                    "max_solving_degree_A": max((r["maxdeg_A"] for r in sel if r["maxdeg_A"] not in ("", None)), default=None),
                    "max_solving_degree_B": max((r["maxdeg_B"] for r in sel if r["maxdeg_B"] not in ("", None)), default=None),
                    "mean_ops_A": float(np.mean([r["ops_A"] for r in sel])) if sel else None,
                    "mean_ops_B": float(np.mean([r["ops_B"] for r in sel])) if sel else None,
                    "seeds_with_equal_relation_rank_failed_counts_AB": int(sum(1 for r in sel if r["counts_equal_AB"])),
                },
            }
        pair_ids = [c["id"] for c in comps if c["kind"] == "pair"]
        raw_p = [res[c]["primary_cpu"]["permutation"]["p"] for c in pair_ids]
        adj = holm([1.0 if x is None else x for x in raw_p])
        for c, a_ in zip(pair_ids, adj):
            res[c]["primary_cpu"]["permutation"]["p_holm_over_pairs"] = a_
        # decision rule (frozen, PROTOCOL section 6)
        for c in pair_ids:
            r_ = res[c]
            iso = res.get(f"P{r_['pair_index']}-iso")
            same = res.get(f"P{r_['pair_index']}-same")
            ci = r_["primary_cpu"]["ci95"]
            excl0 = ci["lo"] is not None and (ci["lo"] > 0 or ci["hi"] < 0)
            hp = r_["primary_cpu"]["permutation"]["p_holm_over_pairs"] < 0.05
            n = r_["n_seeds"]
            same_sign = max(r_["primary_cpu"]["n_positive"], r_["primary_cpu"]["n_negative"]) >= (n - 1)
            iso_hw = (iso["primary_cpu"]["ci95"]["hi"] - iso["primary_cpu"]["ci95"]["lo"]) / 2
            beats_iso = abs(ci["mean"]) > iso_hw
            beats_same = True if same is None else abs(ci["mean"]) > abs(same["primary_cpu"]["ci95"]["mean"])
            r_["decision"] = {
                "ci_excludes_zero": excl0, "holm_p_below_0.05": hp, "sign_consistent_ge_n_minus_1": same_sign,
                "effect_exceeds_iso_halfwidth": beats_iso, "effect_exceeds_same_control": beats_same,
                "same_control_present": same is not None,
                "verdict": "reproducible_difference" if (excl0 and hp and same_sign and beats_iso and beats_same)
                           else "no_reproducible_difference_detected"}
        strata = [[r["diff_y_cpu"] for r in rows if r["comparison"] == c and r["engine"] == e
                   and r["diff_y_cpu"] != ""] for c in pair_ids]
        out["engines"][e] = {"comparisons": res,
                             "pooled_pairs_stratified_bootstrap": pooled_stratified_bootstrap(strata)}
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--mode", choices=("main", "replay"), default="main")
    ap.add_argument("--seeds", type=int, default=12)
    ap.add_argument("--attempts", type=int, default=24)
    ap.add_argument("--out", default=HERE)
    args = ap.parse_args()
    receipt, comps, summaries = execute(args, args.mode)
    xe = cross_engine_check(summaries)
    if args.mode == "main":
        rows = write_csv(os.path.join(args.out, "paired_results.csv"), comps, summaries)
        an = analyse(rows, comps)
        an["cross_engine_check"] = xe
        an["iso_exact_image_check"] = iso_image_check(os.path.join(args.out, "runs.jsonl"), comps, {p: Fq(p) for p in {c["A"]["p"] for c in comps}})
        an["totals"] = {"runs": len(summaries),
                        "verification_failed": sum(s["verification_failed"] for s in summaries),
                        "censored_runs": sum(1 for s in summaries if s["censored"]),
                        "total_cpu_seconds_in_runs": sum(s["total_cpu_s"] for s in summaries)}
        with open(os.path.join(args.out, "analysis.json"), "w") as fh:
            json.dump(an, fh, indent=1)
        print(json.dumps({"runs": len(summaries), "cross_engine": xe, "totals": an["totals"]}))
    else:
        orig = {}
        with open(os.path.join(args.out, "runs.jsonl")) as fh:
            for line in fh:
                r = json.loads(line)
                if r["record"] == "run_summary":
                    orig[(r["comparison"], r["curve"], r["seed"], r["engine"])] = r
        rep = []
        for s in summaries:
            o = orig[(s["comparison"], s["curve"], s["seed"], s["engine"])]
            rep.append({"comparison": s["comparison"], "curve": s["curve"], "seed": s["seed"],
                        "engine": s["engine"],
                        "targets_fingerprint_equal": o["targets_fingerprint"] == s["targets_fingerprint"],
                        "relations_fingerprint_equal": o["relations_fingerprint"] == s["relations_fingerprint"],
                        "rank_equal": o["rank_increments"] == s["rank_increments"],
                        "y_cpu_original": o["y_cpu"], "y_cpu_replay": s["y_cpu"],
                        "cpu_ratio_replay_over_original": s["total_cpu_s"] / o["total_cpu_s"]})
        ratios = [x["cpu_ratio_replay_over_original"] for x in rep]
        report = {"n_replayed_runs": len(rep),
                  "all_targets_fingerprints_equal": all(x["targets_fingerprint_equal"] for x in rep),
                  "all_relation_fingerprints_equal": all(x["relations_fingerprint_equal"] for x in rep),
                  "all_ranks_equal": all(x["rank_equal"] for x in rep),
                  "cross_engine_check_replay": xe,
                  "cpu_ratio_replay_over_original": {"min": min(ratios), "median": float(np.median(ratios)),
                                                     "max": max(ratios)},
                  "runs": rep}
        with open(os.path.join(args.out, "replay_report.json"), "w") as fh:
            json.dump(report, fh, indent=1)
        print(json.dumps({k: v for k, v in report.items() if k != "runs"}))


if __name__ == "__main__":
    main()
