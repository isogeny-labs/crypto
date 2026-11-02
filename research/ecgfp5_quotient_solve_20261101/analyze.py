#!/usr/bin/env python3
"""Analysis only: tabulate the m = 3 validation and the m = 5 quotient cells of PROTOCOL.md and
score each m = 5 cell against G1's frozen criteria (PROTOCOL.md §5, §7, amendment A1 §10).

Reads results/ (and dev/ for the labelled development run); writes analysis.json and analysis.md.
Units: c_cyc = (user + sys) CPU seconds x 2.8 GHz; c_ops band [c_cyc/4, 4 c_cyc].
    python3 analyze.py
"""
import json
import math
import re
from pathlib import Path

HERE = Path(__file__).resolve().parent
RES = HERE / "results"
CLOCK = 2.8e9
PRIMES = [4111, 262231, 33554371]
ARMS = ["raw_x", "sym_x", "raw_u", "sym_u", "quot"]
PRED_M3 = {"raw_x": 384, "sym_x": 64, "raw_u": 384, "sym_u": 64, "quot": 16}
F4_LINE = re.compile(r"^\s*(\d+)\s+(\d+)\s+(\d+)\s+(\d+) x (\d+)\s+([\d.]+)%\s+(\d+) new\s+(\d+) zero\s+([\d.]+) \|\s+([\d.]+)")


def load(p):
    """JSON, tolerating the integer-keyed histograms qsolve prints ({24: 1})."""
    try:
        txt = Path(p).read_text()
    except OSError:
        return None
    try:
        return json.loads(txt)
    except ValueError:
        try:
            return json.loads(re.sub(r"([{,]\s*)(\d+):", r'\1"\2":', txt))
        except ValueError:
            return None


def f4_rounds(log):
    rows = []
    for line in Path(log).read_text(errors="replace").splitlines():
        m = F4_LINE.match(line)
        if m:
            g = m.groups()
            rows.append({"deg": int(g[0]), "rows": int(g[3]), "cols": int(g[4]), "density_pct": float(g[5]),
                         "new": int(g[6]), "zero": int(g[7]), "real_s": float(g[8]), "cpu_s": float(g[9])})
    return rows


def last_started_degree(log):
    """Degree of the last F4 round that started (its line may be cut off by the stop)."""
    deg = None
    for line in Path(log).read_text(errors="replace").splitlines():
        m = re.match(r"^\s*(\d+)\s+(\d+)\s+(\d+)\s+(\d+) x (\d+)", line)
        if m:
            deg = int(m.group(1))
    return deg


def dim_quotient(log):
    m = re.findall(r"Dimension of quotient:\s*(\d+)", Path(log).read_text(errors="replace"))
    return int(m[-1]) if m else None


def lg(x):
    return round(math.log2(x), 2) if x and x > 0 else None


def m3_table():
    out = []
    for p in PRIMES + [65551]:
        for tg in ("planted", "random"):
            d = RES / f"m3_p{p}_{tg}"
            if not d.is_dir():
                continue
            row = {"p": p, "target": tg, "arms": {}}
            for a in ARMS:
                chk = load(d / f"{a}.check.json") or {}
                run = load(d / f"{a}.run.json") or {}
                row["arms"][a] = {
                    "D_log": dim_quotient(d / f"{a}.log") if (d / f"{a}.log").exists() else None,
                    "param_degree": chk.get("param_degree"),
                    "predicted": PRED_M3[a],
                    "planted_recovered": chk.get("planted_coordinates_recovered"),
                    "signal": run.get("signal"),
                }
            orb = load(d / "orbits.json")
            row["identity_control"] = orb
            out.append(row)
    return out


def score(c_cyc, complete, D):
    lo, hi = c_cyc / 4, 4 * c_cyc
    if complete and D is not None and D <= 2 ** 17 and hi <= 2 ** 40:
        return "PROMOTE"
    if (complete and D is not None and D >= 2 ** 19) or (complete and lo >= 2 ** 44):
        return "CLOSE"
    return "not_measured"


def m5_cell(d, label):
    run = load(d / "quot.run.json") or {}
    chk = load(d / "quot.check.json") or {}
    meta = load(d / "meta.json") or {}
    log = d / "quot.log"
    rounds = f4_rounds(log) if log.exists() else []
    cpu = (run.get("user_seconds") or 0) + (run.get("sys_seconds") or 0)
    c_cyc = cpu * CLOCK
    complete = bool(chk.get("planted_coordinates_recovered")) and run.get("exit_code") == 0
    D = dim_quotient(log) if log.exists() else None
    verdict = score(c_cyc, complete, D)
    lower_bound_close = (not complete) and c_cyc / 4 >= 2 ** 44
    stop = ("complete" if complete else "rss_cap" if run.get("killed_by_rss_cap") else
            "wall_cap" if run.get("killed_by_wall_cap") else f"signal {run.get('signal')}" if run.get("signal") else
            f"exit {run.get('exit_code')}")
    return {
        "cell": label, "p": meta.get("p"), "complete": complete, "stop": stop, "D": D,
        "param_degree": chk.get("param_degree"), "planted_recovered": chk.get("planted_coordinates_recovered"),
        "wall_s": run.get("wall_seconds"), "cpu_s": round(cpu, 1), "maxrss_gib": round((run.get("maxrss_kib") or 0) / 2 ** 20, 2),
        "rss_at_stop_gib": round((run.get("last_rss_sampled_kib") or 0) / 2 ** 20, 2),
        "log2_c_cyc": lg(c_cyc), "log2_c_ops_band": [lg(c_cyc / 4), lg(4 * c_cyc)],
        "last_f4_degree_started": last_started_degree(log) if log.exists() else None,
        "last_f4_degree_finished": rounds[-1]["deg"] if rounds else None,
        "verdict_G1": verdict,
        "close_on_lower_bound": lower_bound_close,
        "generation_wall_s": meta.get("generation_wall_seconds"),
        "rounds": rounds,
        "rss_samples": run.get("rss_samples_s_kib", []),
    }


W0_F4_CPU_RE = re.compile(r"^overall\(cpu\)\s+([\d.]+)", re.M)
W0_F4_WALL_RE = re.compile(r"^overall\(elapsed\)\s+([\d.]+)", re.M)
PARAM_RE = re.compile(r"Time for rational param:\s+([\d.]+) \(elapsed\) sec / ([\d.]+) sec \(cpu\)")
WTS = {"W1": "(2,2,2,2,2,1)", "W3": "(1,1,1,1,1,2)", "W2": "(1,2,3,4,5,5)"}
SBA = {"F5a": "sba(i, 0, 1)", "F5b": "sba(i, 1, 0)"}
IMPROVE = 1.5   # PROTOCOL.md §11, §12: CPU below W0's F4 CPU by a factor >= 1.5


def f4_split(log):
    """W0 phase split from a finished msolve -v 2 log: F4 (TIMINGS) and the whole solve."""
    t = Path(log).read_text(errors="replace")
    f4c, f4w, par = W0_F4_CPU_RE.search(t), W0_F4_WALL_RE.search(t), PARAM_RE.search(t)
    return {"f4_cpu_s": float(f4c.group(1)) if f4c else None, "f4_wall_s": float(f4w.group(1)) if f4w else None,
            "solve_wall_s": float(par.group(1)) if par else None, "solve_cpu_s": float(par.group(2)) if par else None}


def degree_falls(rounds):
    falls, top = 0, None
    for r in rounds:
        if top is not None and r["deg"] < top:
            falls += 1
        top = r["deg"] if top is None else max(top, r["deg"])
    return falls


def stop_of(run, done):
    if done:
        return "complete"
    if run.get("killed_by_rss_cap"):
        return "rss_cap"
    if run.get("killed_by_wall_cap"):
        return "wall_cap"
    return f"signal {run.get('signal')}" if run.get("signal") else f"exit {run.get('exit_code')}"


def probe_verdict(stop, cpu, base_cpu, ok=True):
    if stop == "complete" and ok:
        return "IMPROVEMENT" if cpu * IMPROVE <= base_cpu else "no improvement"
    if stop == "wall_cap":
        return "no improvement (cap: > 1.25 x W0)"
    if stop == "rss_cap":
        return "not_measured (RSS cap)"
    if stop == "owner_stop":
        return "no improvement (stopped by owner past the 1/1.5 bar)" if cpu * IMPROVE > base_cpu else "not_measured (stopped by owner)"
    return "not_measured"


def probes(base_cpu, base_wall, base_D):
    out = {"weighted": [], "sba": []}
    W = RES / "m5_p4111_weighted"
    for a in ("W1", "W3", "W2"):
        d = W / a
        if not d.is_dir():
            continue
        run = load(d / "run.json") or {}
        log = d / "quot_w.log"
        rounds = f4_rounds(log) if log.exists() else []
        sp = f4_split(log) if log.exists() else {}
        done = run.get("exit_code") == 0 and sp.get("f4_cpu_s") is not None
        cpu = sp["f4_cpu_s"] if done else (run.get("user_seconds") or 0) + (run.get("sys_seconds") or 0)
        st = stop_of(run, done)
        out["weighted"].append({
            "arm": a, "weights": WTS[a], "stop": st, "f4_cpu_s": round(cpu, 1),
            "f4_wall_s": sp.get("f4_wall_s") if done else run.get("wall_seconds"),
            "maxrss_gib": round((run.get("maxrss_kib") or 0) / 2 ** 20, 2),
            "D": dim_quotient(log) if log.exists() else None,
            "highest_degree": max((r["deg"] for r in rounds), default=None), "degree_falls": degree_falls(rounds),
            "cpu_ratio_vs_W0": round(base_cpu / cpu, 3) if cpu else None,
            "verdict": probe_verdict(st, cpu, base_cpu), "rounds": rounds})
    S = RES / "m5_p4111_sba"
    for a in ("F5a", "F5b"):
        d = S / a
        if not d.is_dir():
            continue
        run = load(d / "run.json") or {}
        txt = (d / "sing.log").read_text(errors="replace") if (d / "sing.log").exists() else ""
        m = re.search(r"RESULT vdim=(\d+)", txt)
        done = run.get("exit_code") == 0 and m is not None
        st = "owner_stop" if (d / "STOPPED_BY_OWNER.md").exists() else stop_of(run, done)
        cpu = (run.get("user_seconds") or 0) + (run.get("sys_seconds") or 0)
        vdim = int(m.group(1)) if m else None
        out["sba"].append({
            "arm": a, "call": SBA[a], "stop": st, "cpu_s": round(cpu, 1), "wall_s": run.get("wall_seconds"),
            "maxrss_gib": round((run.get("maxrss_kib") or 0) / 2 ** 20, 2), "vdim": vdim,
            "cpu_ratio_vs_W0": round(base_cpu / cpu, 3) if cpu else None,
            "verdict": probe_verdict(st, cpu, base_cpu, ok=(vdim == base_D)),
            "rss_samples": run.get("rss_samples_s_kib", [])})
    return out


def main():
    m3 = m3_table()
    m5 = [m5_cell(RES / f"m5_p{p}_planted", f"m5_p{p}") for p in PRIMES if (RES / f"m5_p{p}_planted").is_dir()]
    dev = HERE / "dev" / "m5p331"
    devcell = m5_cell(dev, "dev_m5_p331 (development run, not data)") if dev.is_dir() else None
    if devcell:
        # the dev run used run.json / quot.log names
        run = load(dev / "run.json") or {}
        cpu = (run.get("user_seconds") or 0) + (run.get("sys_seconds") or 0)
        devcell.update({"wall_s": run.get("wall_seconds"), "cpu_s": round(cpu, 1), "log2_c_cyc": lg(cpu * CLOCK),
                        "log2_c_ops_band": [lg(cpu * CLOCK / 4), lg(4 * cpu * CLOCK)],
                        "maxrss_gib": round((run.get("maxrss_kib") or 0) / 2 ** 20, 2), "stop": "stopped by hand",
                        "rss_samples": run.get("rss_samples_s_kib", []), "verdict_G1": "n/a (dev)"})
    base = RES / "m5_p4111_planted"
    split = {f"m5_p{p}": f4_split(RES / f"m5_p{p}_planted" / "quot.log") for p in PRIMES if (RES / f"m5_p{p}_planted" / "quot.log").exists()}
    w0 = split.get("m5_p4111", {})
    prb = probes(w0.get("f4_cpu_s") or 0, w0.get("f4_wall_s"), 65536)
    out = {"phase_split": split, "probes": prb, "W0_f4_cpu_s": w0.get("f4_cpu_s"), "improvement_factor": IMPROVE, "unit": "c_cyc = CPU seconds x 2.8 GHz; c_ops band [c_cyc/4, 4 c_cyc]",
           "thresholds_log2_c_cyc": {"PROMOTE_max": 38, "CLOSE_min": 46},
           "m3": m3, "m5": m5, "dev": devcell}
    (HERE / "analysis.json").write_text(json.dumps(out, indent=1) + "\n")
    L = ["# Analysis (generated by analyze.py)", "", "## m = 3 validation (D from msolve; predicted in brackets)", "",
         "| p | target | raw_x [384] | sym_x [64] | raw_u [384] | sym_u [64] | quot [16] | planted recovered (raw_u, sym_u, quot) | identity control |",
         "|--:|---|--:|--:|--:|--:|--:|---|---|"]
    for r in m3:
        a = r["arms"]
        cell = lambda x: str(a[x]["D_log"]) if a[x]["D_log"] is not None else f"SIGSEGV" if a[x]["signal"] == 11 else "–"
        pr = ", ".join(str(a[x]["planted_recovered"]) for x in ("raw_u", "sym_u", "quot")) if r["target"] == "planted" else "n/a"
        ic = r["identity_control"]
        ics = (f"pass={ic.get('identity_control_pass')}; raw {ic.get('raw_rational')} -> quot {ic.get('quot_rational')} "
               f"orbits {ic.get('orbit_size_histogram_quot')}" if isinstance(ic, dict) else "not run")
        L.append(f"| {r['p']} | {r['target']} | {cell('raw_x')} | {cell('sym_x')} | {cell('raw_u')} | {cell('sym_u')} | {cell('quot')} | {pr} | {ics} |")
    L += ["", "## m = 5 quotient cells", "",
          "| cell | stop | D | F4 degree reached | wall s | CPU s | log2 c_cyc | log2 c_ops band | max RSS GiB | G1 verdict | CLOSE on lower bound |",
          "|---|---|--:|--:|--:|--:|--:|---|--:|---|---|"]
    for c in m5 + ([devcell] if devcell else []):
        L.append(f"| {c['cell']} | {c['stop']} | {c['D']} | {c['last_f4_degree_started']} | {c['wall_s']} | {c['cpu_s']} | {c['log2_c_cyc']} | "
                 f"{c['log2_c_ops_band']} | {c['maxrss_gib']} | {c['verdict_G1']} | {c['close_on_lower_bound']} |")
    L += ["", "## m = 5 phase split (msolve -v 2 timings)", "",
          "| cell | F4 CPU s | log2 c_cyc F4 | parametrisation CPU s | whole solve CPU s | log2 c_cyc whole | F4 share |",
          "|---|--:|--:|--:|--:|--:|--:|"]
    for k, v in split.items():
        if v.get("f4_cpu_s") and v.get("solve_cpu_s"):
            L.append(f"| {k} | {v['f4_cpu_s']} | {lg(v['f4_cpu_s'] * CLOCK)} | {round(v['solve_cpu_s'] - v['f4_cpu_s'], 1)} | "
                     f"{v['solve_cpu_s']} | {lg(v['solve_cpu_s'] * CLOCK)} | {v['f4_cpu_s'] / v['solve_cpu_s']:.2f} |")
    L += ["", f"## Probe arms at p = 4111 against W0's F4 ({w0.get('f4_cpu_s')} CPU s; improvement needs CPU <= W0/{IMPROVE})", "",
          "| arm | ordering / call | stop | CPU s | W0/arm | max RSS GiB | D or vdim | highest degree | degree falls | verdict |",
          "|---|---|---|--:|--:|--:|--:|--:|--:|---|"]
    for r in prb["weighted"]:
        L.append(f"| {r['arm']} | W = {r['weights']} | {r['stop']} | {r['f4_cpu_s']} | {r['cpu_ratio_vs_W0']} | {r['maxrss_gib']} | {r['D']} | "
                 f"{r['highest_degree']} | {r['degree_falls']} | {r['verdict']} |")
    for r in prb["sba"]:
        L.append(f"| {r['arm']} | Singular {r['call']} | {r['stop']} | {r['cpu_s']} | {r['cpu_ratio_vs_W0']} | {r['maxrss_gib']} | {r['vdim']} | – | – | {r['verdict']} |")
    (HERE / "analysis.md").write_text("\n".join(L) + "\n")
    print("\n".join(L))


if __name__ == "__main__":
    main()
