#!/usr/bin/env python3
"""Tables and P1-P5 verdicts for the m = 5 prime-tower study (stdlib only).

Reads cells/*.jsonl (rows of examples/pkm_tower_pilot.rs) and cells/*.time
(/usr/bin/time -v stderr) under this directory; tolerant of missing files;
always exits 0. A row with timed_out or oversize is a cap stop (inconclusive_cap)
and never a degree. Wall seconds are diagnostic only.
"""
import glob, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
RECORDED = {("kummer", 4, 8): 6, ("kummer", 3, 15): 7, ("kummer", 2, 10): 5}


def load():
    rows = []
    for f in sorted(glob.glob(os.path.join(HERE, "cells", "*.jsonl"))):
        name = os.path.basename(f)[:-6]
        try:
            lines = open(f).read().splitlines()
        except OSError:
            continue
        for ln in lines:
            try:
                r = json.loads(ln)
            except ValueError:
                continue
            if isinstance(r, dict) and "N" in r and "m" in r:
                r["_cell"] = name
                rows.append(r)
    return rows


def rss_kib(cell):
    try:
        t = open(os.path.join(HERE, "cells", cell + ".time")).read()
    except OSError:
        return None
    m = re.search(r"Maximum resident set size \(kbytes\): (\d+)", t)
    return int(m.group(1)) if m else None


def capped(r):
    return bool(r.get("timed_out") or r.get("oversize"))


def degs(rows, kind, m, N):
    """Return (list of exact D, number capped, number of rows) for a cell."""
    sel = [r for r in rows if r["kind"] == kind and r["m"] == m and r["N"] == N]
    ds = [r["solving_degree_max"] for r in sel if not capped(r)]
    return ds, sum(capped(r) for r in sel), len(sel)


def verdict(ds, ncap, n, ok):
    if n == 0:
        return "not_run"
    if ncap and not ds:
        return "inconclusive_cap"
    if ncap:
        return "inconclusive_cap"  # mixed: at least one target capped
    return "supported" if ok(ds) else "refuted"


def main():
    rows = load()
    print("%-8s %2s %3s %2s %-6s %-9s %9s %10s %9s %6s" %
          ("kind", "m", "N", "tg", "D", "state", "wall_s", "rss_MiB", "max_cols", "steps"))
    for r in sorted(rows, key=lambda r: (r["m"], r["N"], r["kind"], r.get("target_index", 0))):
        rss = rss_kib(r["_cell"])
        print("%-8s %2d %3d %2s %-6s %-9s %9.2f %10s %9s %6s" % (
            r["kind"], r["m"], r["N"], r.get("target_index", "-"),
            "cap" if capped(r) else r.get("solving_degree_max"),
            "cap" if capped(r) else "done", r.get("wall_s", float("nan")),
            "-" if rss is None else "%.0f" % (rss / 1024), r.get("max_cols"), r.get("steps")))
    if not rows:
        print("(no rows found under cells/)")

    print("\nControls (recorded D; rows must also match the recorded rows apart from ms, wall_s):")
    for (kind, m, N), want in RECORDED.items():
        ds, nc, n = degs(rows, kind, m, N)
        st = "not_run" if n == 0 else ("cap" if nc else ("OK" if ds and all(d == want for d in ds) else "MISMATCH"))
        print("  m=%d N=%d %s: recorded D=%d, got %s -> %s" % (m, N, kind, want, ds, st))

    print("\nWithin-cell A/A (two targets must give the same D):")
    for kind in ("kummer", "isogeny"):
        for N in (10, 15, 20):
            ds, nc, n = degs(rows, kind, 5, N)
            if n:
                print("  %s N=%d: D=%s capped=%d -> %s" % (kind, N, ds, nc,
                      "n/a (capped)" if nc else ("same" if len(set(ds)) <= 1 else "DIFFERENT")))

    print("\nPredictions:")
    allD = {}
    for kind in ("kummer", "isogeny"):
        for N in (10, 15, 20):
            allD[(kind, N)] = degs(rows, kind, 5, N)

    def pool(N):
        ds, nc, n = [], 0, 0
        for kind in ("kummer", "isogeny"):
            d, c, k = allD[(kind, N)]
            ds += d; nc += c; n += k
        return ds, nc, n

    p1 = verdict(*pool(10), lambda d: all(x in (6, 7) for x in d))
    p2 = verdict(*pool(15), lambda d: all(x in (7, 8) for x in d))
    ds, nc, n = pool(20)
    if n == 0:
        p3 = "not_run"
    elif nc:
        p3 = "supported" if not any(x <= 7 for x in ds) else "refuted"  # cap or D>=8 both support
    else:
        p3 = "supported" if all(x >= 8 for x in ds) else "refuted"
    pairs, bad = 0, 0
    for N in (10, 15, 20):
        a, b = allD[("kummer", N)], allD[("isogeny", N)]
        if a[0] and b[0] and not a[1] and not b[1]:
            pairs += 1
            bad += set(a[0]) != set(b[0]) or len(set(a[0] + b[0])) > 1
    p4 = "not_run" if (pairs == 0 and not any(allD[k][2] for k in allD)) else (
        "inconclusive_cap" if pairs == 0 else ("refuted" if bad else "supported"))
    # P5: N=15 within 7200 s and 8 GiB on both towers
    r15 = [r for r in rows if r["m"] == 5 and r["N"] == 15]
    if not r15:
        p5 = "not_run"
    elif any(capped(r) for r in r15):
        p5 = "inconclusive_cap"
    else:
        slow = any(r.get("wall_s", 0) >= 7200 for r in r15)
        big = any((rss_kib(r["_cell"]) or 0) >= 8 * 1024 * 1024 for r in r15)
        p5 = "refuted" if (slow or big) else "supported"
    for k, v, txt in (("P1", p1, "D(m=5,N=10) in {6,7}"), ("P2", p2, "D(m=5,N=15) in {7,8}"),
                      ("P3", p3, "D(m=5,N=20) >= 8 or cap"), ("P4", p4, "Kummer D = isogeny D at equal N"),
                      ("P5", p5, "N=15 within 7200 s and 8 GiB")):
        print("  %s %-16s %s" % (k, v, txt))
    kill = any(x >= 8 for N in (10, 15) for x in pool(N)[0])
    print("\nKill criterion (D >= 8 at N <= 15): %s" % ("TRIGGERED, close m=5 line on prime towers" if kill else "not triggered"))
    return 0


if __name__ == "__main__":
    try:
        main()
    except Exception as e:  # tolerant by design
        print("analyze.py: error: %r" % (e,), file=sys.stderr)
    sys.exit(0)
