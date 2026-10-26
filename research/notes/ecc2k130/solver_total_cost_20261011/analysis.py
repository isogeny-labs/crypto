"""Pure functions: receipts -> summary dict -> markdown tables / RESULTS.md.

Used by summarize.py (writes results/summary.json) and verify.py (recomputes and compares).
Nothing here runs a solver.  Medians are taken across repetitions of per-repetition
aggregates; min/max across repetitions are kept for spread.
"""
from __future__ import annotations

import base64
import gzip
import hashlib
import io
import json
import math
from pathlib import Path
import re
import statistics
import tarfile

import tcommon as tc

HERE = Path(__file__).resolve().parent
M83 = HERE.parent / "m83_solver_matrix_20260927"
R_SUBGROUP = 2417851639230796216685689          # m83 prime subgroup order (asserted via curve.order in the runner)
ENGINES = ("fes", "sat-cms-xor", "sat-z3", "f4-boolean", "f5b-boolean")
ENGINE_LABEL = {"fes": "FES", "sat-cms-xor": "CryptoMiniSat (native XOR)", "sat-z3": "Z3 (And/Xor)",
                "f4-boolean": "F4-style prototype", "f5b-boolean": "F5B prototype"}
CLOCKS = ("cpu_s", "wall_s")
ATTEMPT_STATUSES = ("complete", "timeout", "oom", "crashed", "wall_killed")
NUMERIC_SUFFIX = {"cpu_s": "cpu", "wall_s": "wall"}


# ------------------------------------------------------------------ loading
def load_receipts(path):
    manifest, children = None, []
    for line in Path(path).read_text().splitlines():
        if not line.strip():
            continue
        rec = json.loads(line)
        if rec["type"] == "manifest":
            manifest = rec
        else:
            children.append(rec)
    return manifest, children


def status_of(child):
    r = child["result"]
    if r:
        return r["status"]
    return "wall_killed" if child["timeout_killed"] else "crashed"


def m83_archive_files():
    data = base64.b64decode((M83 / "results" / "raw_receipts.tar.gz.base64").read_bytes())
    files = {}
    with tarfile.open(fileobj=io.BytesIO(gzip.decompress(data)), mode="r") as tar:
        for item in tar:
            files[item.name] = tar.extractfile(item).read()
    return files


def m83_receipt(files, name):
    wrapper = json.loads(files[name])
    lines = [json.loads(s) for s in wrapper["stdout"].splitlines() if s.strip()]
    return lines[-1]


def m83_published_medians():
    """Recompute the m83 'solver stage' medians (wall) from the archived receipts."""
    files = m83_archive_files()
    out = {}
    for engine in ENGINES:
        times = []
        for seed in (260938, 260939):
            for index in range(6):
                if engine == "sat-cms-xor":
                    name = f"extension_20260927/quotient_s2/s2-seed{seed}-i{index}-{engine}.json"
                else:
                    name = f"run_20260927/fixed-s{seed}-i{index}-{engine}.json"
                rec = m83_receipt(files, name)
                assert rec["status"] == "complete", name
                times.append(rec["timing"]["solver_including_setup"]["wall_s"])
        out[engine] = statistics.median(times)
    return out


# ------------------------------------------------------------------ metrics
def stage(res, name, clock):
    return res.get("stages", {}).get(name, {}).get(clock, 0.0)


def kendall_tau(a, b):
    n, conc, disc = len(a), 0, 0
    for i in range(n):
        for j in range(i + 1, n):
            s = (a[i] - a[j]) * (b[i] - b[j])
            conc += s > 0
            disc += s < 0
    return (conc - disc) / (n * (n - 1) / 2)


def cp_upper_zero(n, alpha=0.05):
    """One-sided (1-alpha) Clopper-Pearson upper bound on a rate with 0 successes in n trials."""
    return 1 - alpha ** (1.0 / n) if n > 0 else None


def cell_cost(child, clock, group_size):
    """Charged costs of one child for accountings A/B/C, solver stage, and per-stage values."""
    res = child["result"]
    if not res:   # wall-killed or crashed: only the parent-side wall time is known
        w = child["parent_wall_s"]
        z = {s: 0.0 for s in tc.STAGES}
        z["solve"] = w if clock == "wall_s" else child["child_cpu_s"]
        return {"A": z["solve"], "B": z["solve"], "C": z["solve"], "solver": z["solve"], "stages": z}
    st = {s: stage(res, s, clock) for s in tc.STAGES}
    a = sum(st[s] for s in tc.CORE_STAGES)
    solver = sum(st[s] for s in tc.SOLVER_STAGES)
    return {"A": a, "B": a - st["fb_setup"] + st["fb_setup"] / group_size,
            "C": a + st["engine_load"] + st["input_gen"], "solver": solver, "stages": st}


def group_rep_metrics(children_rep, group_sizes):
    """Metrics for one (group, engine, repetition) list of children."""
    out = {}
    attempts = [c for c in children_rep if status_of(c) in ATTEMPT_STATUSES]
    out["n.cells"] = len(children_rep)
    out["n.attempts"] = len(attempts)
    for s in ATTEMPT_STATUSES + ("setup_failed",):
        out[f"n.{s}"] = sum(status_of(c) == s for c in children_rep)
    setup_failed = [c for c in children_rep if status_of(c) == "setup_failed"]
    nat = [c for c in attempts if (c["result"] or {}).get("kind") == "natural"]
    pla = [c for c in attempts if (c["result"] or {}).get("kind") == "planted"]
    nat_done = [c for c in nat if status_of(c) == "complete"]
    out["n.natural_attempts"] = len(nat)
    out["n.natural_completed"] = len(nat_done)
    out["n.planted_attempts"] = len(pla)
    out["natural.verified_cells"] = sum(bool(c["result"]["relation"]["natural_verified"]) for c in nat_done)
    out["natural.new_independent"] = sum(c["result"]["relation"]["new_independent_natural"] for c in nat_done)
    out["natural.rank_final"] = max([c["result"]["relation"].get("rank_after", 0) for c in nat_done] or [0])
    out["planted.verified_cells"] = sum(bool(c["result"]["relation"]["planted_verified"])
                                        for c in pla if status_of(c) == "complete")
    out["algebraic_roots.total"] = sum(len(c["result"]["algebraic_roots"]) for c in attempts
                                       if status_of(c) == "complete")
    out["false_positives.total"] = sum(len(c["result"]["group_verification"]["false_positive"])
                                       for c in attempts if status_of(c) == "complete")
    lifts = [c["result"]["factor_base"]["usable_lifts"] for c in nat_done]
    out["expected_p_per_natural_attempt"] = (statistics.mean(x ** 3 for x in lifts) / (R_SUBGROUP - 1)
                                             if lifts else float("nan"))
    for clock in CLOCKS:
        k = clock
        costs = {id(c): cell_cost(c, clock, group_sizes[(c["workload"], c["policy"], c["payload_bits"],
                                                         c["seed"], c["engine"])]) for c in attempts}
        stage_sum = {s: sum(costs[id(c)]["stages"][s] for c in attempts) for s in tc.STAGES}
        for s in tc.STAGES:
            out[f"{k}.stage.{s}"] = stage_sum[s]
        for acc in ("A", "B", "C", "solver"):
            out[f"{k}.T_{acc}"] = sum(costs[id(c)][acc] for c in attempts)
        ta, ts = out[f"{k}.T_A"], out[f"{k}.T_solver"]
        out[f"{k}.f_s"] = ts / ta if ta else float("nan")
        out[f"{k}.amdahl_g"] = ta / (ta - ts) if ta > ts else float("inf")
        out[f"{k}.f_s_B"] = ts / out[f"{k}.T_B"] if out[f"{k}.T_B"] else float("nan")
        out[f"{k}.f_s_C"] = ts / out[f"{k}.T_C"] if out[f"{k}.T_C"] else float("nan")
        out[f"{k}.per_attempt.T_A"] = ta / len(attempts) if attempts else float("nan")
        out[f"{k}.per_attempt.T_solver"] = ts / len(attempts) if attempts else float("nan")
        for s in tc.STAGES:
            out[f"{k}.per_attempt.stage.{s}"] = stage_sum[s] / len(attempts) if attempts else float("nan")
        out[f"{k}.per_natural_attempt.T_A"] = (sum(costs[id(c)]["A"] for c in nat) / len(nat)
                                               if nat else float("nan"))
        # POST HOC (not preregistered): steady-state natural stream, no factor-base setup, library load or
        # input generation, natural attempts only
        ss = [costs[id(c)]["stages"] for c in nat]
        ss_core = sum(x["eq_build"] + x["solver_setup"] + x["solve"] + x["model_extraction"] + x["lifting"]
                      + x["verification"] + x["rank_update"] for x in ss)
        ss_solver = sum(x["solver_setup"] + x["solve"] + x["model_extraction"] for x in ss)
        out[f"{k}.ss.T"] = ss_core
        out[f"{k}.ss.T_solver"] = ss_solver
        out[f"{k}.ss.per_attempt"] = ss_core / len(nat) if nat else float("nan")
        out[f"{k}.ss.f_s"] = ss_solver / ss_core if ss_core else float("nan")
        out[f"{k}.ss.amdahl_g"] = ss_core / (ss_core - ss_solver) if ss_core > ss_solver else float("inf")
        pv = [c for c in pla if status_of(c) == "complete" and c["result"]["relation"]["planted_verified"]]
        out[f"{k}.per_planted_verified.T_A"] = (sum(costs[id(c)]["A"] for c in pla if c in pv) / len(pv)
                                                if pv else float("nan"))
        out[f"{k}.setup_failed_cost"] = sum(
            sum(stage(c["result"], s, clock) for s in ("fb_setup", "input_gen")) for c in setup_failed)
        done = [c for c in attempts if status_of(c) == "complete"]
        solver_m83 = [costs[id(c)]["solver"] + costs[id(c)]["stages"]["engine_load"] for c in done]
        out[f"{k}.median_solver_stage_m83_definition"] = statistics.median(solver_m83) if solver_m83 else float("nan")
        out[f"{k}.median_solver_stage_excl_load"] = (statistics.median(costs[id(c)]["solver"] for c in done)
                                                     if done else float("nan"))
    ncp = out["n.natural_completed"]
    out["natural.cp95_upper_rate"] = cp_upper_zero(ncp) if out["natural.new_independent"] == 0 else float("nan")
    cpa = out["cpu_s.per_natural_attempt.T_A"]
    out["natural.cost_per_relation_lower_bound_cpu_s"] = (
        cpa / out["natural.cp95_upper_rate"] if ncp and out["natural.new_independent"] == 0 else float("nan"))
    p = out["expected_p_per_natural_attempt"]
    out["natural.expected_cpu_s_per_relation_derived"] = cpa / p if p and p > 0 else float("inf")
    out["status.max_wall_s"] = max([c["parent_wall_s"] for c in children_rep] or [0.0])
    out["child.cpu_s_sum"] = sum(c["child_cpu_s"] for c in children_rep)
    out["child.wall_s_sum"] = sum(c["parent_wall_s"] for c in children_rep)
    out["child.loadavg_max"] = max([c["loadavg_1m_at_start"] for c in children_rep] or [0.0])
    return out


def over_reps(per_rep):
    keys = per_rep[0].keys()
    agg = {"median": {}, "min": {}, "max": {}}
    for k in keys:
        vals = [r[k] for r in per_rep]
        finite = [v for v in vals if isinstance(v, (int, float)) and not (isinstance(v, float) and math.isnan(v))]
        if len(finite) != len(vals) or not finite:
            agg["median"][k] = vals[0] if len(set(map(str, vals))) == 1 else None
            agg["min"][k] = agg["max"][k] = agg["median"][k]
            continue
        agg["median"][k], agg["min"][k], agg["max"][k] = statistics.median(finite), min(finite), max(finite)
    return agg


GROUPS = (("W1a", "W1a", None), ("W1b-s2", "W1b", 2), ("W1b-s3", "W1b", 3), ("W1b-s4", "W1b", 4))


def summarize_w1(children):
    reps = sorted({c["rep"] for c in children})
    group_sizes = {}
    for c in children:
        if c["rep"] == reps[0]:
            key = (c["workload"], c["policy"], c["payload_bits"], c["seed"], c["engine"])
            group_sizes[key] = group_sizes.get(key, 0) + 1
    out = {"reps": reps, "groups": {}}
    for gname, workload, s in GROUPS:
        gout = {"engines": {}}
        for engine in ENGINES:
            per_rep, flips = [], []
            for rep in reps:
                cs = [c for c in children if c["rep"] == rep and c["workload"] == workload
                      and c["engine"] == engine and (s is None or c["payload_bits"] == s)]
                per_rep.append(group_rep_metrics(cs, group_sizes))
                flips.append({f"s{c['payload_bits']}-{c['seed']}-i{c['index']}": status_of(c) for c in cs})
            agg = over_reps(per_rep)
            agg["cell_status_by_rep"] = flips
            unstable = sorted({k for k in flips[0] if len({f[k] for f in flips}) > 1})
            agg["status_unstable_cells"] = unstable
            gout["engines"][engine] = agg
        med = lambda e, k: gout["engines"][e]["median"][k]
        done = [e for e in ENGINES if med(e, "n.complete") == med(e, "n.attempts") and med(e, "n.attempts")]
        if len(done) == len(ENGINES):
            ts = [med(e, "cpu_s.T_solver") for e in ENGINES]
            ta = [med(e, "cpu_s.T_A") for e in ENGINES]
            tm = [med(e, "cpu_s.median_solver_stage_m83_definition") for e in ENGINES]
            tss = [med(e, "cpu_s.ss.T") for e in ENGINES]
            tb = [med(e, "cpu_s.T_B") for e in ENGINES]
            tcc = [med(e, "cpu_s.T_C") for e in ENGINES]
            gout["ordering"] = {
                "engines_all_complete": True,
                "order_by_T_solver": [e for _, e in sorted(zip(ts, ENGINES))],
                "order_by_T_core_A": [e for _, e in sorted(zip(ta, ENGINES))],
                "kendall_tau_solver_vs_core": kendall_tau(ts, ta),
                "spread_max_over_min_T_solver": max(ts) / min(ts),
                "spread_max_over_min_T_core_A": max(ta) / min(ta),
                "spread_max_over_min_m83_solver_stage": max(tm) / min(tm),
                "order_by_T_posthoc_steady_state": [e for _, e in sorted(zip(tss, ENGINES))],
                "spread_max_over_min_T_posthoc_steady_state": max(tss) / min(tss),
                "kendall_tau_solver_vs_posthoc": kendall_tau(ts, tss),
                "order_by_T_core_B": [e for _, e in sorted(zip(tb, ENGINES))],
                "order_by_T_core_C": [e for _, e in sorted(zip(tcc, ENGINES))],
                "kendall_tau_solver_vs_B": kendall_tau(ts, tb),
                "kendall_tau_solver_vs_C": kendall_tau(ts, tcc),
                "spread_max_over_min_T_core_B": max(tb) / min(tb),
                "spread_max_over_min_T_core_C": max(tcc) / min(tcc)}
            for i, e in enumerate(ENGINES):       # ratios to FES (ENGINES[0]) in each accounting
                gout["ordering"][f"ratio_{e}_solver"] = ts[i] / ts[0]
                gout["ordering"][f"ratio_{e}_core_A"] = ta[i] / ta[0]
                gout["ordering"][f"ratio_{e}_posthoc"] = tss[i] / tss[0]
            gout["ordering"]["ratio_z3_over_cms_solver"] = ts[2] / ts[1]
            gout["ordering"]["ratio_z3_over_cms_core_A"] = ta[2] / ta[1]
        else:
            gout["ordering"] = {"engines_all_complete": False}
        out["groups"][gname] = gout
    return out


def summarize_w2(children):
    out = {"engines": {}}
    for engine in ("sat-cms-xor", "sat-z3"):
        cs = sorted((c for c in children if c["engine"] == engine), key=lambda c: c["index"])
        rows, tot = [], {}
        rank_final = 0
        for c in cs:
            r = c["result"] or {}
            st = {k: {cl: r.get("stages", {}).get(k, {}).get(cl, 0.0) for cl in CLOCKS} for k in tc.STAGES}
            row = {"index": c["index"], "status": status_of(c), "result": r.get("result"),
                   "rejected_tuples": r.get("rejected_tuples"),
                   "new_independent": r.get("relation", {}).get("new_independent_natural", 0),
                   "stages": st, "model": r.get("model_size"), "peak_rss_mb": (r.get("peak_rss_bytes") or 0) / 1e6,
                   "parent_wall_s": c["parent_wall_s"], "child_cpu_s": c["child_cpu_s"],
                   "loadavg": c["loadavg_1m_at_start"]}
            for cl in CLOCKS:
                row[f"T_A.{cl}"] = sum(st[s][cl] for s in tc.CORE_STAGES)
                row[f"T_solver.{cl}"] = sum(st[s][cl] for s in tc.SOLVER_STAGES)
                row[f"T_C.{cl}"] = row[f"T_A.{cl}"] + st["engine_load"][cl] + st["input_gen"][cl]
            rank_final = max(rank_final, r.get("relation", {}).get("rank_after", 0))
            rows.append(row)
        agg = {"rows": rows, "n_targets": len(rows),
               "n_verified": sum(r["result"] == "verified" for r in rows),
               "n_timeout": sum(r["status"] == "timeout" for r in rows),
               "n_unsat": sum(r["result"] == "unsat" for r in rows),
               "new_independent": sum(r["new_independent"] for r in rows), "rank_final": rank_final}
        for cl in CLOCKS:
            k = NUMERIC_SUFFIX[cl]
            agg[f"{k}.T_A"] = sum(r[f"T_A.{cl}"] for r in rows)
            agg[f"{k}.T_C"] = sum(r[f"T_C.{cl}"] for r in rows)
            agg[f"{k}.T_solver"] = sum(r[f"T_solver.{cl}"] for r in rows)
            agg[f"{k}.f_s"] = agg[f"{k}.T_solver"] / agg[f"{k}.T_A"] if agg[f"{k}.T_A"] else float("nan")
            agg[f"{k}.nonsolver"] = agg[f"{k}.T_A"] - agg[f"{k}.T_solver"]
            agg[f"{k}.stage"] = {s: sum(r["stages"][s][cl] for r in rows) for s in tc.STAGES}
            agg[f"{k}.per_new_independent_A"] = (agg[f"{k}.T_A"] / agg["new_independent"]
                                                 if agg["new_independent"] else None)
            agg[f"{k}.per_attempt_A"] = agg[f"{k}.T_A"] / len(rows) if rows else None
        out["engines"][engine] = agg
    z, cm = out["engines"]["sat-z3"], out["engines"]["sat-cms-xor"]
    out["ratio_z3_over_cms_per_relation"] = (z["cpu.per_new_independent_A"] / cm["cpu.per_new_independent_A"]
                                             if z["cpu.per_new_independent_A"] and cm["cpu.per_new_independent_A"] else None)
    return out


def file_sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def build_summary(w1_path, w2_path=None):
    summary = {"schema": "solver_total_cost/summary/v1"}
    manifest, children = load_receipts(w1_path)
    summary["w1"] = summarize_w1(children)
    summary["w1_receipts"] = {"path": str(Path(w1_path).relative_to(HERE)), "sha256": file_sha(w1_path),
                              "children": len(children), "versions": manifest["versions"],
                              "git_head": manifest["git"]["head"],
                              "loadavg_at_start": manifest["loadavg_at_start"]}
    summary["m83_published_solver_stage_median_wall_s"] = m83_published_medians()
    if w2_path and Path(w2_path).exists():
        m2, c2 = load_receipts(w2_path)
        summary["w2"] = summarize_w2(c2)
        summary["w2_receipts"] = {"path": str(Path(w2_path).relative_to(HERE)), "sha256": file_sha(w2_path),
                                  "children": len(c2), "versions": m2["versions"],
                                  "targets_check": m2["targets_check"],
                                  "loadavg_at_start": m2["loadavg_at_start"]}
    return summary


# ------------------------------------------------------------------ formatting
def f_ms(x, nd=2):
    return "n/a" if x is None or (isinstance(x, float) and math.isnan(x)) else f"{x * 1000:.{nd}f}"


def f_s(x, nd=3):
    return "n/a" if x is None or (isinstance(x, float) and math.isnan(x)) else f"{x:.{nd}f}"


def f_pct(x, nd=1):
    return "n/a" if x is None or (isinstance(x, float) and math.isnan(x)) else f"{x * 100:.{nd}f}%"


def f_x(x, nd=2):
    if x is None or (isinstance(x, float) and math.isnan(x)):
        return "n/a"
    return "inf" if math.isinf(x) else f"{x:.{nd}f}x"


def f_sci(x, nd=2):
    if x is None or (isinstance(x, float) and math.isnan(x)):
        return "n/a"
    return "inf (p=0)" if math.isinf(x) else f"{x:.{nd}e}"


FORMATS = {"ms2": f_ms, "ms3": lambda x: f_ms(x, 3), "ms1": lambda x: f_ms(x, 1), "s3": f_s,
           "s2": lambda x: f_s(x, 2), "s1": lambda x: f_s(x, 1), "pct1": f_pct, "pct0": lambda x: f_pct(x, 0),
           "x1": lambda x: f_x(x, 1), "x2": f_x, "sci2": f_sci, "int": lambda x: str(int(x)),
           "g3": lambda x: f"{x:.3g}", "g2": lambda x: f"{x:.2g}", "f3": lambda x: f"{x:.3f}"}


def lookup(summary, path):
    """Dotted/slashed lookup with shorthands: a:<engine>/<key> (W1a median), g:<group>/<engine>/<key>,
    o:<key> (W1a ordering), w2:<engine>/<key>, m83:<engine> (published median)."""
    if path.startswith("a:"):
        engine, key = path[2:].split("/", 1)
        return summary["w1"]["groups"]["W1a"]["engines"][engine]["median"][key]
    if path.startswith("g:"):
        group, engine, key = path[2:].split("/", 2)
        return summary["w1"]["groups"][group]["engines"][engine]["median"][key]
    if path.startswith("o:"):
        return summary["w1"]["groups"]["W1a"]["ordering"][path[2:]]
    if path.startswith("w2:"):
        engine, rest = path[3:].split("/", 1)
        node = summary["w2"]["engines"][engine]
        for part in rest.split("/"):
            node = node[int(part)] if isinstance(node, list) else node[part]
        return node
    if path.startswith("m83:"):
        return summary["m83_published_solver_stage_median_wall_s"][path[4:]]
    if path.startswith("w2r:"):
        return summary["w2"][path[4:]]
    node = summary
    for part in path.split("/"):
        node = node[int(part)] if isinstance(node, list) else node[part]
    return node


def md_table(header, rows):
    out = ["| " + " | ".join(header) + " |", "|" + "|".join(":--" if i == 0 else "--:" for i in range(len(header))) + "|"]
    out += ["| " + " | ".join(str(c) for c in r) + " |" for r in rows]
    return "\n".join(out)


STAGE_SHORT = {"fb_setup": "fb setup", "input_gen": "input gen", "eq_build": "eq build",
               "engine_load": "engine load", "solver_setup": "solver setup", "solve": "solve",
               "model_extraction": "model extr.", "lifting": "lifting", "verification": "verif.",
               "rank_update": "rank upd."}


def tables(summary):
    t = {}
    w1 = summary["w1"]["groups"]
    a = w1["W1a"]["engines"]
    m = lambda g, e, k: w1[g]["engines"][e]["median"][k]
    pub = summary["m83_published_solver_stage_median_wall_s"]
    # T1 headline
    rows = []
    for e in ENGINES:
        rows.append([ENGINE_LABEL[e], f"{int(m('W1a', e, 'n.complete'))}/{int(m('W1a', e, 'n.attempts'))}",
                     f"{int(m('W1a', e, 'natural.new_independent'))}/{int(m('W1a', e, 'n.natural_completed'))}",
                     f"{int(m('W1a', e, 'planted.verified_cells'))}/{int(m('W1a', e, 'n.planted_attempts'))}",
                     f_ms(m("W1a", e, "cpu_s.per_attempt.T_solver")), f_ms(m("W1a", e, "cpu_s.per_attempt.T_A")),
                     f_pct(m("W1a", e, "cpu_s.f_s")), f_x(m("W1a", e, "cpu_s.amdahl_g"))])
    t["t1_headline"] = md_table(
        ["Engine (W1a, 12 inputs)", "complete", "natural relations / natural attempts", "planted verified",
         "solver stage ms/input (CPU)", "total T_core ms/input (CPU)", "solver share f_s", "Amdahl bound g"], rows)
    # T2 per-stage
    rows = []
    for e in ENGINES:
        rows.append([ENGINE_LABEL[e]] + [f_ms(m("W1a", e, f"cpu_s.per_attempt.stage.{s}"), 3) for s in tc.STAGES]
                    + [f_ms(m("W1a", e, "cpu_s.per_attempt.T_A"))])
    t["t2_stages"] = md_table(["Engine (mean CPU ms per input)"] + [STAGE_SHORT[s] for s in tc.STAGES] + ["T_core (A)"], rows)
    # T3 accountings
    rows = []
    for e in ENGINES:
        n = m("W1a", e, "n.attempts")
        rows.append([ENGINE_LABEL[e], f_ms(m("W1a", e, "cpu_s.T_A") / n), f_pct(m("W1a", e, "cpu_s.f_s")),
                     f_ms(m("W1a", e, "cpu_s.T_B") / n), f_pct(m("W1a", e, "cpu_s.f_s_B")),
                     f_ms(m("W1a", e, "cpu_s.T_C") / n), f_pct(m("W1a", e, "cpu_s.f_s_C")),
                     f_ms(m("W1a", e, "wall_s.T_A") / n), f_pct(m("W1a", e, "wall_s.f_s"))])
    t["t3_accountings"] = md_table(
        ["Engine (W1a)", "A: core, cold ms/input", "f_s (A)", "B: setup amortised ms/input", "f_s (B)",
         "C: fully loaded ms/input", "f_s (C)", "A in wall ms/input", "f_s (A, wall)"], rows)
    # T4 ordering
    o = w1["W1a"]["ordering"]
    rows = [["order by solver stage (T_solver, CPU)", " < ".join(ENGINE_LABEL[e] for e in o["order_by_T_solver"])],
            ["order by total cost (T_core, A, CPU)", " < ".join(ENGINE_LABEL[e] for e in o["order_by_T_core_A"])],
            ["Kendall tau between the two orderings", f"{o['kendall_tau_solver_vs_core']:.2f}"],
            ["order by T_core under B (setup amortised)", " < ".join(ENGINE_LABEL[e] for e in o["order_by_T_core_B"])],
            ["order by T_core under C (fully loaded)", " < ".join(ENGINE_LABEL[e] for e in o["order_by_T_core_C"])],
            ["Kendall tau, solver stage vs T_core under B / under C",
             f"{o['kendall_tau_solver_vs_B']:.2f} / {o['kendall_tau_solver_vs_C']:.2f}"],
            ["max/min over engines, solver stage", f_x(o["spread_max_over_min_T_solver"], 1)],
            ["max/min over engines, total T_core (A)", f_x(o["spread_max_over_min_T_core_A"], 1)],
            ["max/min over engines, m83-definition solver stage (incl. library load)",
             f_x(o["spread_max_over_min_m83_solver_stage"], 1)],
            ["post hoc steady state: max/min over engines, T_ss", f_x(o["spread_max_over_min_T_posthoc_steady_state"], 1)],
            ["post hoc steady state: Kendall tau, solver stage vs T_ss", f"{o['kendall_tau_solver_vs_posthoc']:.2f}"]]
    t["t4_ordering"] = md_table(["W1a ranking comparison", "value"], rows)
    # T5 yield and cost per relation
    rows = []
    for e in ENGINES:
        rows.append([ENGINE_LABEL[e], f"{int(m('W1a', e, 'natural.new_independent'))}/{int(m('W1a', e, 'n.natural_completed'))}",
                     f_ms(m("W1a", e, "cpu_s.per_natural_attempt.T_A")),
                     f_ms(m("W1a", e, "cpu_s.per_planted_verified.T_A")),
                     f"{m('W1a', e, 'natural.cp95_upper_rate'):.3f}",
                     f_s(m("W1a", e, "natural.cost_per_relation_lower_bound_cpu_s"), 3),
                     f_sci(m("W1a", e, "natural.expected_cpu_s_per_relation_derived"))])
    t["t5_cost_per_relation"] = md_table(
        ["Engine (W1a)", "new independent natural relations / attempts", "CPU ms per natural attempt (T_core)",
         "CPU ms per verified planted relation", "95% upper bound on yield/attempt",
         "CPU s per relation, measured 95% lower bound", "CPU s per relation, derived lower bound (uniform targets)"], rows)
    # T6 W1b
    rows = []
    for g in ("W1b-s2", "W1b-s3", "W1b-s4"):
        for e in ENGINES:
            mm = lambda k: m(g, e, k)
            n = mm("n.attempts")
            rows.append([g[-2:], ENGINE_LABEL[e],
                         f"{int(mm('n.complete'))}/{int(mm('n.timeout'))}/{int(mm('n.setup_failed'))}",
                         ", ".join(w1[g]["engines"][e]["status_unstable_cells"]) or "-",
                         f_ms(mm("cpu_s.per_attempt.T_solver")) if n else "n/a",
                         f_ms(mm("cpu_s.per_attempt.T_A")) if n else "n/a",
                         f_pct(mm("cpu_s.f_s")) if n else "n/a",
                         f"{int(mm('natural.new_independent'))}/{int(mm('n.natural_completed'))}"])
    t["t6_w1b"] = md_table(["s", "Engine", "complete / timeout / setup_failed (cells, median over reps)",
                            "cells whose status changed between repetitions", "solver ms/attempt (CPU)",
                            "T_core ms/attempt (CPU)", "f_s", "natural relations / completed natural"], rows)
    # T11 post hoc steady state (unregistered)
    rows = []
    for e in ENGINES:
        rows.append([ENGINE_LABEL[e], f_ms(m("W1a", e, "cpu_s.ss.per_attempt")),
                     f_ms(m("W1a", e, "cpu_s.ss.T_solver") / m("W1a", e, "n.natural_attempts")),
                     f_pct(m("W1a", e, "cpu_s.ss.f_s")), f_x(m("W1a", e, "cpu_s.ss.amdahl_g"))])
    t["t11_posthoc"] = md_table(
        ["Engine (W1a, 8 natural attempts; post hoc, not preregistered)", "T_ss CPU ms per attempt",
         "of which solver stage ms", "solver share f_s", "Amdahl bound g"], rows)
    # T2b W1b stage tables
    rows = []
    for g in ("W1b-s3", "W1b-s4"):
        for e in ENGINES:
            if not m(g, e, "n.attempts"):
                continue
            rows.append([g[-2:], ENGINE_LABEL[e]] + [f_ms(m(g, e, f"cpu_s.per_attempt.stage.{s}"), 2) for s in tc.STAGES]
                        + [f_ms(m(g, e, "cpu_s.per_attempt.T_A"), 1)])
    t["t2b_w1b_stages"] = md_table(["s", "Engine (mean CPU ms per attempt)"] + [STAGE_SHORT[s] for s in tc.STAGES]
                                   + ["T_core (A)"], rows)
    # T7 bridge to m83
    rows = []
    for e in ENGINES:
        rows.append([ENGINE_LABEL[e], f_s(pub[e], 6), f_s(m("W1a", e, "wall_s.median_solver_stage_m83_definition"), 6),
                     f_s(m("W1a", e, "wall_s.median_solver_stage_excl_load"), 6),
                     f_x(m("W1a", e, "wall_s.median_solver_stage_m83_definition") / pub[e], 2)])
    t["t7_m83_bridge"] = md_table(
        ["Engine", "m83 published median solver-stage wall s (recomputed from archive)",
         "this host, same definition (incl. library import), wall s", "this host, library load excluded, wall s",
         "this host / m83"], rows)
    # T8 repetitions
    rows = []
    for e in ENGINES:
        mn, mx, md_ = (w1["W1a"]["engines"][e][k]["cpu_s.T_A"] for k in ("min", "max", "median"))
        smn, smx = (w1["W1a"]["engines"][e][k]["cpu_s.T_solver"] for k in ("min", "max"))
        rows.append([ENGINE_LABEL[e], f_s(mn, 3), f_s(md_, 3), f_s(mx, 3), f_s(smn, 4), f_s(smx, 4),
                     f_s(w1["W1a"]["engines"][e]["max"]["child.loadavg_max"], 2)])
    t["t8_reps"] = md_table(["Engine (W1a, 12 inputs summed)", "T_core CPU s, min over reps", "median", "max",
                             "T_solver CPU s, min", "max", "max 1-min load average at a child start"], rows)
    # T9 W2
    if "w2" in summary:
        rows = []
        for e in ("sat-cms-xor", "sat-z3"):
            for r in summary["w2"]["engines"][e]["rows"]:
                rows.append([ENGINE_LABEL[e], r["index"], r["result"] or r["status"],
                             f_s(r["T_A.cpu_s"], 2), f_s(r["T_solver.cpu_s"], 2),
                             f_pct(r["T_solver.cpu_s"] / r["T_A.cpu_s"] if r["T_A.cpu_s"] else float("nan")),
                             f_s(r["stages"]["solve"]["wall_s"], 2), r["rejected_tuples"],
                             r["new_independent"], f"{r['peak_rss_mb']:.0f}"])
        t["t9_w2_targets"] = md_table(
            ["Engine", "target", "outcome", "T_core CPU s", "solver stage CPU s", "f_s", "solve wall s",
             "rejected tuples", "new independent natural relation", "peak RSS MB"], rows)
        rows = []
        for e in ("sat-cms-xor", "sat-z3"):
            g = summary["w2"]["engines"][e]
            per = g["cpu.per_new_independent_A"]
            rows.append([ENGINE_LABEL[e], f"{g['n_verified']}/{g['n_targets']}", g["n_timeout"], g["n_unsat"],
                         f"{g['new_independent']} (rank {g['rank_final']})", f_s(g["cpu.T_A"], 1),
                         f_pct(g["cpu.f_s"], 2),
                         "undefined (no relation)" if per is None else f_s(per, 1),
                         f_s(g["cpu.per_attempt_A"], 1)])
        t["t10_w2_total"] = md_table(
            ["Engine (W2, 4 natural targets)", "verified", "timeouts", "unsat", "new independent relations",
             "T_core CPU s (sum)", "f_s", "T_core CPU s per new independent relation", "T_core CPU s per attempt"], rows)
    return t


TOKEN = re.compile(r"\{\{\s*([^}|]+?)\s*(?:\|\s*([a-z0-9]+)\s*)?\}\}")


def render(template_text, summary):
    tabs = tables(summary)

    def sub(match):
        key, fmt = match.group(1), match.group(2)
        if key.startswith("table:"):
            return tabs[key[6:]]
        value = lookup(summary, key)
        return FORMATS[fmt](value) if fmt else str(value)

    return TOKEN.sub(sub, template_text)
