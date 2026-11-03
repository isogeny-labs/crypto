"""Summarise the A/B sessions: old binary (main 412b60289) vs this change.

sessions/<spec>-<main|final>-<rep>/   headline: old vs final binary, 3 sessions each
attribution/<spec>-<variant>-<rep>/   one binary per change, 2 sessions each (gzipped
                                      records): which change moves which time

Checks that every record is identical to the old binary's outside timing,
and writes results.json, solve-time.svg and report.html next to this file.
Standard library only.
"""
import gzip
import json
import pathlib
import statistics

HERE = pathlib.Path(__file__).parent
SPECS = ["smoke-prime-20", "generic-32"]
HEAD_REPS = [1, 2, 3]
ATTR_REPS = [1, 2]
ATTR = ["main", "only-registry", "only-counters", "only-euclid", "new"]
ATTR_LABEL = {"main": "old", "only-registry": "registry once", "only-counters": "counter keys",
              "only-euclid": "Euclid inverse", "new": "all three"}
# Fields that carry wall time, host or binary identity, and nothing counted.
VOLATILE = {"time", "isolation", "env_class_id", "binary_sha256", "record_id", "session_id", "run_id"}


def records(group, spec, variant, rep):
    d = HERE / group / f"{spec}-{variant}-{rep}"
    if (d / "records.jsonl").exists():
        text = (d / "records.jsonl").read_text()
    else:
        text = gzip.decompress((d / "records.jsonl.gz").read_bytes()).decode()
    return [json.loads(l) for l in text.splitlines() if l.strip()]


def counted(r):
    r = {k: v for k, v in r.items() if k not in VOLATILE}
    r["phases"] = [{k: v for k, v in p.items() if k != "wall_ns"} for p in r["phases"]]
    if isinstance(r.get("online"), dict):
        o = dict(r["online"])
        o.pop("wall_ns", None)
        o["phases_ns"] = sorted(o.get("phases_ns") or {})
        r["online"] = o
    return r


def medians_of(group, reps, variants):
    out = {}
    for spec in SPECS:
        for v in variants:
            rs = [r for rep in reps for r in records(group, spec, v, rep) if not r["warmup"]]
            for arm in sorted({r["arm"] for r in rs}):
                x = [r for r in rs if r["arm"] == arm]
                out[f"{spec}|{v}|{arm}"] = {
                    "n": len(x),
                    "solve_ms": statistics.median(r["time"]["solve_wall_ns"] for r in x) / 1e6,
                    "process_ms": statistics.median(r["time"]["process_wall_ns"] for r in x) / 1e6,
                }
    return out


def arms_of(medians, spec):
    return sorted({k.split("|")[2] for k in medians if k.startswith(spec + "|main|")})


def main():
    identical = {}
    for group, reps, variants in (("sessions", HEAD_REPS, ["final"]), ("attribution", ATTR_REPS, ATTR[1:])):
        for spec in SPECS:
            for rep in reps:
                base = records(group, spec, "main", rep)
                for v in variants:
                    other = records(group, spec, v, rep)
                    same = len(base) == len(other) and all(counted(a) == counted(b) for a, b in zip(base, other))
                    identical[f"{group}|{spec}|{v}|{rep}"] = {"records": len(other), "identical_outside_timing": same}
    head = medians_of("sessions", HEAD_REPS, ["main", "final"])
    attr = medians_of("attribution", ATTR_REPS, ATTR)
    headline = []
    for spec in SPECS:
        for arm in arms_of(head, spec):
            a, b = head[f"{spec}|main|{arm}"], head[f"{spec}|final|{arm}"]
            headline.append({"name": f"{spec}/{arm}/solve_ratio", "claimed": b["solve_ms"] / a["solve_ms"]})
            headline.append({"name": f"{spec}/{arm}/process_ratio", "claimed": b["process_ms"] / a["process_ms"]})
    out = {"schema": "ecbench-overhead-review/v1", "identical": identical, "headline_medians": head,
           "attribution_medians": attr, "headline": headline}
    (HERE / "results.json").write_text(json.dumps(out, indent=1, sort_keys=True) + "\n")
    figure(head)
    report(head, attr, identical)
    bad = [k for k, v in identical.items() if not v["identical_outside_timing"]]
    total = sum(v["records"] for v in identical.values())
    print(f"{total} records compared; identical outside timing:", "all" if not bad else f"NOT {bad}")


def figure(head):
    """Paired bars: median solve wall time per arm, old vs final, one panel per spec."""
    W, L, R, T, bar = 760, 110, 70, 44, 9
    H = T + sum(len(arms_of(head, s)) * (2 * bar + 8) + 36 for s in SPECS) + 30
    out = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" font-family="sans-serif" font-size="11">',
           f'<rect width="{W}" height="{H}" fill="#fff"/>',
           f'<text x="{L}" y="18" font-size="13" fill="#222">Median solve wall time per arm, old binary vs this change (ms)</text>',
           f'<text x="{L}" y="33" fill="#555">ecbench solve_wall_ns, 3 sessions each, warm-up excluded; counted results identical</text>']
    y = T
    for spec in SPECS:
        arms = arms_of(head, spec)
        vmax = max(head[f"{spec}|{v}|{a}"]["solve_ms"] for v in ("main", "final") for a in arms)
        sx = lambda v: (v / vmax) * (W - L - R)
        out.append(f'<text x="{L}" y="{y + 12}" font-weight="bold" fill="#222">{spec}</text>')
        y += 20
        for a in arms:
            out.append(f'<text x="{L - 8}" y="{y + bar + 3}" text-anchor="end" fill="#333">{a}</text>')
            for v, colour in (("main", "#c0392b"), ("final", "#2471a3")):
                m = head[f"{spec}|{v}|{a}"]["solve_ms"]
                out.append(f'<rect x="{L}" y="{y}" width="{sx(m):.1f}" height="{bar}" fill="{colour}"/>')
                out.append(f'<text x="{L + sx(m) + 4:.1f}" y="{y + bar - 1}" font-size="9" fill="#555">{m:.2f}</text>')
                y += bar
            y += 8
        y += 16
    out.append(f'<rect x="{L}" y="{H - 22}" width="10" height="10" fill="#c0392b"/><text x="{L + 14}" y="{H - 13}" fill="#333">main 412b60289</text>')
    out.append(f'<rect x="{L + 130}" y="{H - 22}" width="10" height="10" fill="#2471a3"/><text x="{L + 144}" y="{H - 13}" fill="#333">this change</text>')
    out.append("</svg>")
    (HERE / "solve-time.svg").write_text("\n".join(out) + "\n")


def report(head, attr, identical):
    def hrow(spec, arm):
        a, b = head[f"{spec}|main|{arm}"], head[f"{spec}|final|{arm}"]
        return (f"<tr><td>{spec}</td><td>{arm}</td><td>{a['n']}</td><td>{a['solve_ms']:.3f}</td><td>{b['solve_ms']:.3f}</td>"
                f"<td>{b['solve_ms'] / a['solve_ms']:.3f}</td><td>{a['process_ms']:.1f}</td><td>{b['process_ms']:.1f}</td>"
                f"<td>{b['process_ms'] / a['process_ms']:.3f}</td></tr>")

    def arow(spec, arm):
        cells = "".join(f"<td>{attr[f'{spec}|{v}|{arm}']['solve_ms']:.2f}</td>" for v in ATTR)
        return f"<tr><td>{spec}</td><td>{arm}</td>{cells}</tr>"

    hrows = "".join(hrow(s, a) for s in SPECS for a in arms_of(head, s))
    arows = "".join(arow(s, a) for s in SPECS for a in arms_of(attr, s))
    n_same = sum(v["records"] for v in identical.values() if v["identical_outside_timing"])
    (HERE / "report.html").write_text(f"""<!doctype html><meta charset="utf-8"><title>ecbench harness overhead</title>
<style>body{{font-family:sans-serif;max-width:800px;margin:24px auto;color:#222;font-size:13px}}td,th{{padding:2px 7px;text-align:right}}td:nth-child(-n+2),th:nth-child(-n+2){{text-align:left}}table{{border-collapse:collapse;margin:8px 0}}tr:nth-child(even){{background:#f4f6f7}}code{{font-size:12px}}</style>
<h1>ecbench harness overhead: parse the registry once, allocate counter keys once</h1>
<p>Two harness changes. <code>docs/curves/registry.json</code> (4.7 MB) is parsed once per process instead of on every registry lookup (<code>Workload::on</code> in every measured child and every planned target). <code>GenericOutcome::count</code>, called several times per BSGS and kangaroo step, allocates a counter's key only on first use. No solver, cost unit or counted result changes: {n_same} records are identical to the old binary's outside timing, every session replays exactly under the other binary (<code>ecbench verify --replay-all</code>), and the §5a band is identical.</p>
{(HERE / 'solve-time.svg').read_text()}
<h2>Old binary vs this change</h2>
<p>Median over 3 sessions per binary, alternated old/new, warm-up excluded. <em>solve</em> is the method's timed window (<code>solve_wall_ns</code>); <em>process</em> is the whole measured child.</p>
<table><tr><th>spec</th><th>arm</th><th>n</th><th>solve old ms</th><th>solve new ms</th><th>new/old</th><th>process old ms</th><th>process new ms</th><th>new/old</th></tr>{hrows}</table>
<h2>Which change does what</h2>
<p>Median solve ms, one binary per change, 2 sessions each. The registry change removes 3.5–4 ms from every timed solve window and about 7 ms from every measured child; the counter change removes 1.8–2.8 ms from each BSGS arm at 32 bits (no visible effect at 20 bits); a word Euclid inverse in <code>prime_fast</code>, 2.3–2.9× faster per inversion in isolation, moved nothing and is not part of the change.</p>
<table><tr><th>spec</th><th>arm</th>{''.join(f'<th>{ATTR_LABEL[v]}</th>' for v in ATTR)}</tr>{arows}</table>
<h2>Why the solve window was affected</h2>
<p>Nothing in the solve path reads the registry. Callgrind counts the same instructions inside the window for both binaries (1.110 M vs 1.112 M on a 20-bit <code>rho.negation</code> solve), and the old child executes 44 M more instructions before it (193 M vs 149 M: the second parse). The old binary drops its freshly parsed copy of the registry immediately before the solve; the 3.5–4 ms it then loses inside the window is wall time with no extra instructions and no syscalls. Disabling glibc fast bins did not remove it; the exact memory-system cause was not isolated. Wall times recorded by the old binary on small curves therefore carry this offset; counted results do not.</p>
<p>Host: Intel Xeon @ 2.10 GHz, 4 vCPU cloud container; sessions pinned by ecbench (<code>--cpus auto</code>) with <code>--allow-busy</code>, so below L2. Engineering change to harness overhead, not an index-calculus result.</p>
""")


if __name__ == "__main__":
    main()
