#!/usr/bin/env python3
"""Resume a frozen sweep over genuine PKM auxiliary-curve cosets."""
import argparse
import json
import math
import os
import random
import time
from pathlib import Path

import sweep as m

HERE = Path(__file__).resolve().parent
RNG_SEED = 20261124
PER_SOURCE = 16


def aux_add(u, v, a):
    if u is None:
        return v
    if v is None:
        return u
    x1, y1 = u
    x2, y2 = v
    if x1 == x2 and (y1 + y2) % m.P == 0:
        return None
    if x1 == x2:
        d, n = 2*y1 % m.P, (3*x1*x1+a) % m.P
    else:
        d, n = (x2-x1) % m.P, (y2-y1) % m.P
    if not d:
        return None
    slope = n*pow(d, -1, m.P) % m.P
    x3 = (slope*slope-x1-x2) % m.P
    return x3, (slope*(x1-x3)-y1) % m.P


def auxiliary_cosets(t, n):
    a, b = t["kind"]["aux"]["a"], t["kind"]["aux"]["b"]
    q = tuple(t["kind"]["Q"])
    assert (q[1]*q[1]-q[0]**3-a*q[0]-b) % m.P == 0
    subgroup = [None]
    u = None
    for _ in range(1, t["S"]):
        u = aux_add(u, q, a)
        assert u is not None
        subgroup.append(u)
    assert aux_add(u, q, a) is None
    gset = set(subgroup)
    rng = random.Random(RNG_SEED + t["S"])
    used = set()
    for _ in range(n):
        for _attempt in range(10000):
            x = rng.randrange(m.P)
            z = (x*x*x+a*x+b) % m.P
            y = pow(z, (m.P+1)//4, m.P)
            if y*y % m.P != z:
                continue
            r = (x, y if rng.randrange(2) else (-y) % m.P)
            if aux_add(r, r, a) in gset:
                continue
            v = []
            for h in subgroup:
                point = aux_add(r, h, a)
                assert point is not None
                v.append(point[0])
            key = min(v)
            if key in used:
                continue
            used.add(key)
            assert len(v) == len(set(v)) == t["S"]
            c = m.map_value(t["steps"], x)
            assert c is not None and all(m.map_value(t["steps"], z) == c for z in v)
            yield r, c, v, _attempt+1
            break
        else:
            raise RuntimeError("could not find a fresh valid coset")


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--out", type=Path, default=HERE / "coset_results.jsonl")
    args = ap.parse_args()
    existing = {}
    if args.out.exists():
        for line in args.out.read_text().splitlines():
            row = json.loads(line)
            existing[row["id"]] = row
    ts = m.targets()
    for name in ("cm", "isogeny"):
        t, digest = m.source(name)
        start, start_cpu = time.perf_counter(), time.process_time()
        cosets = list(auxiliary_cosets(t, PER_SOURCE))
        generation_total = time.perf_counter()-start
        generation_cpu = time.process_time()-start_cpu
        for i, (r, c, v, attempts) in enumerate(cosets):
            key = f"{name}:coset={i}"
            if key in existing:
                continue
            start_w, start_c = time.perf_counter(), time.process_time()
            base = sorted(x for x in v if m.lift(x) is not None)
            build_wall, build_cpu = time.perf_counter()-start_w, time.process_time()-start_c
            control_key = f"interval:size={len(base)}"
            if control_key not in existing:
                ctrl_start = time.process_time()
                ctrl = m.measure(control_key, m.interval_base(len(base)), ts,
                                 {"kind":"size_matched_control", "base_cpu_s":time.process_time()-ctrl_start})
                m.append(args.out, ctrl)
                existing[control_key] = ctrl
            control = existing[control_key]
            row = m.measure(key, base, ts,
                {"kind":"pkm-isogeny-coset", "source_sha256":digest,
                 "aux_R":list(r), "terminal_c":c, "high_degree":t["S"],
                 "low_degrees":[s["degree"] for s in t["steps"]],
                 "fiber_size":len(v), "fiber_verified":True, "lifting_count":len(base),
                 "coset_draw_attempts":attempts,"coset_generation_wall_s_amortized":generation_total/PER_SOURCE,
                 "coset_generation_cpu_s_amortized":generation_cpu/PER_SOURCE,
                 "base_wall_s":build_wall,"base_cpu_s":build_cpu,
                 "source_search_build_charged":False})
            pval, wins, losses = m.paired_tail(row["success_bits"][m.SCREEN:],
                                              control["success_bits"][m.SCREEN:])
            row.update({"control_id":control_key,"control_screen_hits":control["screen_hits"],
                        "control_confirm_hits":control["confirm_hits"],"paired_p":pval,
                        "paired_wins":wins,"paired_losses":losses})
            row["screen_advances"] = row["screen_hits"] > control["screen_hits"]
            row["confirmed_gain"] = (row["screen_advances"] and
                row["confirm_hits"] > control["confirm_hits"] and pval < .01/(2*PER_SOURCE) and
                row["query_cpu_s"]/max(row["screen_hits"]+row["confirm_hits"],1) <
                control["query_cpu_s"]/max(control["screen_hits"]+control["confirm_hits"],1))
            m.append(args.out, row)
            existing[key] = row
            print(json.dumps({k:row[k] for k in ("id","screen_hits","confirm_hits","paired_p","confirmed_gain")}),flush=True)
            if row["confirmed_gain"]:
                return


if __name__ == "__main__":
    main()
