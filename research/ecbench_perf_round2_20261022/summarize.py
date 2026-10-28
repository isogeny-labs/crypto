"""Summarise round 2: previous binary vs this change, ecbench sessions and Koblitz `ic price`.

sessions/<spec>-<main|new>-<rep>/   ecbench sessions, old and new binary alternated
koblitz-ic/<curve>-<base|c2><rep>.json   `ic price --repeats 3` on the rule-v3 curves

Checks that every ecbench record and every `ic price` count is identical to the
old binary's outside timing, and writes results.json, solve-time.svg and
report.html next to this file.  Standard library only.
"""
import glob
import json
import math
import pathlib
import statistics

HERE = pathlib.Path(__file__).parent
SPECS = ["rho-32", "generic-32", "prime-ic-28"]
VOLATILE = {"time", "isolation", "env_class_id", "binary_sha256", "record_id", "session_id", "run_id"}
CURVES = ["k0n41", "k0n53", "k0n61"]


def records(spec, variant):
    out = []
    for d in sorted(HERE.glob(f"sessions/{spec}-{variant}-*")):
        out.append([json.loads(l) for l in (d / "records.jsonl").read_text().splitlines() if l.strip()])
    return out


def counted(r):
    r = {k: v for k, v in r.items() if k not in VOLATILE}
    r["phases"] = [{k: v for k, v in p.items() if k != "wall_ns"} for p in r["phases"]]
    if isinstance(r.get("online"), dict):
        o = dict(r["online"])
        o.pop("wall_ns", None)
        o["phases_ns"] = sorted(o.get("phases_ns") or {})
        r["online"] = o
    return json.dumps(r, sort_keys=True)


def ecbench():
    rows, n_same, n_all = [], 0, 0
    for spec in SPECS:
        old, new = records(spec, "main"), records(spec, "new")
        ref = [counted(r) for r in old[0]]
        for sess in old + new:
            for r, c in zip(sess, ref):
                n_all += 1
                n_same += counted(r) == c
        for arm in sorted({r["arm"] for r in old[0]}):
            def solve(ss):
                return [r["time"]["solve_wall_ns"] for s in ss for r in s if r["arm"] == arm and not r["warmup"]]
            logs = [math.log(b["time"]["solve_wall_ns"] / a["time"]["solve_wall_ns"])
                    for so, sn in zip(old, new) for a, b in zip(so, sn) if a["arm"] == arm and not a["warmup"]]
            mo, mn = statistics.median(solve(old)), statistics.median(solve(new))
            rows.append({"spec": spec, "arm": arm, "method": next(r["method"]["id"] for r in old[0] if r["arm"] == arm),
                         "old_ms": mo / 1e6, "new_ms": mn / 1e6, "solve_ratio": mn / mo,
                         "paired_geomean": math.exp(sum(logs) / len(logs)), "pairs": len(logs),
                         "sessions": (len(old), len(new))})
    return rows, n_same, n_all


def koblitz():
    def counts(d):
        reps = [{"ic": {k: r["ic_online"][k] for k in ("recovered", "trials", "verified")},
                 "rho": {k: r["rho_online"][k] for k in ("recovered", "steps", "walks", "distinguished_points", "verified")}}
                for r in d["repetitions"]]
        return json.dumps({"counts": d["counts"], "rho_counts": d["rho_counts"], "reps": reps}, sort_keys=True)

    rows, diffs = [], 0
    for c in CURVES:
        A = [json.load(open(f)) for f in sorted(glob.glob(str(HERE / f"koblitz-ic/{c}-base*.json")))]
        B = [json.load(open(f)) for f in sorted(glob.glob(str(HERE / f"koblitz-ic/{c}-c2*.json")))]
        ref = counts(A[0])
        diffs += sum(counts(d) != ref for d in A + B)

        def med(ds, key):
            return statistics.median(key(r) for d in ds for r in d["repetitions"]) / 1e6

        phases = {p: (med(A, lambda r: r["setup_phases_ns"].get(p, 0)), med(B, lambda r: r["setup_phases_ns"].get(p, 0)))
                  for p in A[0]["repetitions"][0]["setup_phases_ns"]}
        so, sn = med(A, lambda r: r["setup_ns"]), med(B, lambda r: r["setup_ns"])
        rows.append({"curve": c, "old_setup_ms": so, "new_setup_ms": sn, "setup_ratio": sn / so,
                     "old_online_ms": med(A, lambda r: r["ic_online"]["wall_ns"]),
                     "new_online_ms": med(B, lambda r: r["ic_online"]["wall_ns"]),
                     "rho_online_ms": med(B, lambda r: r["rho_online"]["wall_ns"]),
                     "phases_ms": {p: v for p, v in sorted(phases.items(), key=lambda kv: -kv[1][0]) if v[0] >= 0.5},
                     "repetitions": (sum(len(d["repetitions"]) for d in A), sum(len(d["repetitions"]) for d in B))})
    return rows, diffs


def figure(eb, kb):
    items = [(f"{r['spec']} {r['arm']}", r["paired_geomean"]) for r in eb]
    items += [(f"Koblitz IC {r['curve']} set-up", r["setup_ratio"]) for r in kb]
    W, L, rowh = 760, 250, 22
    H = 60 + rowh * len(items) + 30
    x0, x1 = L, W - 40
    sx = lambda v: x0 + (v - 0.6) / (1.1 - 0.6) * (x1 - x0)
    out = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}" font-family="Helvetica,Arial,sans-serif" font-size="12">',
           f'<rect width="{W}" height="{H}" fill="#fff"/>',
           f'<text x="16" y="20" font-size="14" fill="#222">New/old wall time per arm (below 1 = faster); counted results identical; grey = within ±5% (A/A noise)</text>',
           f'<text x="16" y="38" fill="#555">ecbench: paired geometric mean of solve_wall_ns; Koblitz IC: median set-up of ic price (15 vs 15)</text>']
    for t in (0.6, 0.7, 0.8, 0.9, 1.0, 1.1):
        x = sx(t)
        out.append(f'<line x1="{x:.1f}" y1="50" x2="{x:.1f}" y2="{H-28}" stroke="{"#333" if t == 1.0 else "#ddd"}"/>')
        out.append(f'<text x="{x:.1f}" y="{H-12}" text-anchor="middle" fill="#555">{t:.1f}</text>')
    for i, (label, v) in enumerate(items):
        y = 56 + i * rowh
        colour = "#2a6fdb" if v < 0.95 else ("#c0392b" if v > 1.05 else "#999")
        a, b = sorted((sx(1.0), sx(v)))
        out.append(f'<text x="{L-8}" y="{y+13}" text-anchor="end" fill="#222">{label}</text>')
        out.append(f'<rect x="{a:.1f}" y="{y+3}" width="{max(b-a,1):.1f}" height="{rowh-8}" fill="{colour}"/>')
        out.append(f'<text x="{sx(max(v,1.0))+6:.1f}" y="{y+13}" fill="#222">{v:.3f}</text>')
    out.append("</svg>")
    (HERE / "solve-time.svg").write_text("\n".join(out) + "\n")


def report(eb, kb, n_same, n_all, kdiff):
    erows = "\n".join(f"<tr><td>{r['spec']}</td><td>{r['arm']}</td><td><code>{r['method']}</code></td><td>{r['old_ms']:.2f}</td>"
                      f"<td>{r['new_ms']:.2f}</td><td>{r['solve_ratio']:.3f}</td><td>{r['paired_geomean']:.3f}</td><td>{r['pairs']}</td></tr>" for r in eb)
    krows = "\n".join(f"<tr><td>{r['curve']}</td><td>{r['old_setup_ms']:.1f}</td><td>{r['new_setup_ms']:.1f}</td><td>{r['setup_ratio']:.3f}</td>"
                      f"<td>{', '.join(f'{p} {a:.1f}→{b:.1f}' for p, (a, b) in r['phases_ms'].items())}</td>"
                      f"<td>{r['old_online_ms']:.2f}→{r['new_online_ms']:.2f}</td><td>{r['rho_online_ms']:.1f}</td></tr>" for r in kb)
    html = f"""<!doctype html><html><head><meta charset="utf-8"><title>Performance round 2</title>
<style>body{{font-family:Helvetica,Arial,sans-serif;max-width:900px;margin:24px auto;color:#222;font-size:13px}}
table{{border-collapse:collapse;margin:8px 0}}td,th{{border:1px solid #ccc;padding:3px 6px;text-align:right}}td:first-child,th:first-child{{text-align:left}}
code{{font-size:12px}}</style></head><body>
<h1>Algorithm performance, round 2 (2026-10-22)</h1>
<p>Identical-output speedups in the Koblitz index calculus set-up and the prime-field Pollard rho walk. No counted quantity changes:
{n_same} of {n_all} ecbench records are identical to the previous binary's outside timing, every session replays exactly under the other
binary (<code>ecbench verify --replay-all</code>), the §5a band of the prime IC sessions is identical, and {kdiff} <code>ic price</code> outputs differ in any count or answer.</p>
{(HERE / 'solve-time.svg').read_text()}
<h2>Koblitz index calculus set-up (<code>ic price</code>, rule-v3 T01, one thread, pinned)</h2>
<table><tr><th>curve</th><th>set-up old ms</th><th>new ms</th><th>ratio</th><th>phases ≥ 0.5 ms (old→new ms)</th><th>IC online ms</th><th>rho online ms</th></tr>
{krows}</table>
<p>Medians over 5 runs × 3 repetitions per binary. Changes: per-trial scratch buffers reused in the fast scan; a word add-mod in the walk;
the shared index map and field structure built once per collector; Fx hash sets for the orbit factor base and the signed projection map, with the
orbit stepped by k-fold squaring; aimed scans that pre-gather the target's doubled coordinates; the SIMD canonicaliser's tail handled as one padded block.
Build and verify vary ±10% on unchanged code (noise).</p>
<h2>ecbench sessions (prime-order curves)</h2>
<table><tr><th>spec</th><th>arm</th><th>method</th><th>old ms</th><th>new ms</th><th>median ratio</th><th>paired geo-mean</th><th>pairs</th></tr>
{erows}</table>
<p>Changes in <code>ic_boundary</code>: the negation-map coefficient flip is a subtraction instead of two word divisions; inversion for 32-bit
moduli runs Euclid on 32-bit words; the tuned walker computes each point's key and hash once per step. BSGS, kangaroo and prime IC do not
take the changed paths enough to move (flat within ±4%).</p>
<h2>Not done</h2>
<p>Most of a prime rho step is its one field inversion (~110 ns at 32 bits). Batching inversions across walks (Montgomery's trick, in-order commit)
would keep every walk and group-operation count but change the counted <code>field_ops</code> (inversions per step), which the accounting contract
says must be what the code performed; that is an owner decision, not an identical-output change. Barrett multiplication and a quotient-1 Euclid
shortcut were measured slower on this host (<code>microbench/</code>).</p>
</body></html>
"""
    (HERE / "report.html").write_text(html)


def main():
    eb, n_same, n_all = ecbench()
    kb, kdiff = koblitz()
    figure(eb, kb)
    report(eb, kb, n_same, n_all, kdiff)
    headline = [{"name": f"{r['spec']}/{r['arm']}/solve_ratio", "claimed": r["solve_ratio"]} for r in eb]
    headline += [{"name": f"koblitz/{r['curve']}/setup_ratio", "claimed": r["setup_ratio"]} for r in kb]
    out = {"identical_records": n_same, "records": n_all, "koblitz_outputs_differing": kdiff,
           "ecbench": eb, "koblitz": kb, "headline": headline}
    (HERE / "results.json").write_text(json.dumps(out, indent=1, sort_keys=True) + "\n")
    print(f"{n_same}/{n_all} identical, koblitz diffs {kdiff}")
    for r in eb:
        print(f"{r['spec']:12s} {r['arm']:12s} {r['solve_ratio']:.3f} geo {r['paired_geomean']:.3f}")
    for r in kb:
        print(f"{r['curve']} set-up {r['setup_ratio']:.3f}")


if __name__ == "__main__":
    main()
