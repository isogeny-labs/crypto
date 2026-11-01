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
    out = {"unit": "c_cyc = CPU seconds x 2.8 GHz; c_ops band [c_cyc/4, 4 c_cyc]",
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
    (HERE / "analysis.md").write_text("\n".join(L) + "\n")
    print("\n".join(L))


if __name__ == "__main__":
    main()
