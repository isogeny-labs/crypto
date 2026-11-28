#!/usr/bin/env python3
"""Render the 36-bit rung tables from the recorded artifacts.

Wall time is host-load dependent on this shared machine; CPU time
(user+sys from /usr/bin/time -p) is the comparable quantity.
"""
import json
import pathlib

D = pathlib.Path(__file__).resolve().parent


def parse_timing(path):
    """-> list of (label, real_s, cpu_s) for each '== label' block."""
    out, cur = [], None
    for line in path.read_text().splitlines():
        s = line.strip()
        if s.startswith("=="):
            if cur:
                out.append(cur)
            cur = [s.split()[1].split("=", 1)[-1], None, None]
        elif s.startswith("real ") and cur:
            cur[1] = float(s.split()[1])
        elif s.startswith(("user ", "sys ")) and cur:
            cur[2] = (cur[2] or 0.0) + float(s.split()[1])
    if cur:
        out.append(cur)
    return out


def rows(path):
    return [json.loads(l) for l in path.read_text().splitlines() if l.strip().startswith("{")]


def main():
    print("### B probe (seed 20261121, sparse LA, --no-rho)\n")
    print("| B | IC wall ms | IC cpu ms | table ms | relations ms | LA ms | trials | IC ok |")
    print("|--:|--:|--:|--:|--:|--:|--:|:--|")
    timing = {t[0]: t for t in parse_timing(D / "a3_36bit_sweep.log")}
    for r in rows(D / "a3_36bit_sweep.log"):
        s = r["ic_stages"]
        t = timing.get(str(r["fb_size"]))
        cpu = round(t[2] * 1e3) if t and t[2] else "-"
        print(f"| {r['fb_size']} | {s['total_ms']:.0f} | {cpu} | {s['pair_table_ms']:.0f} "
              f"| {s['relations_ms']:.0f} | {s['linear_algebra_ms']:.0f} | {s['trials_total']} "
              f"| {r['ic_ok']} |")

    print("\n### Four paired seeds at B=2048\n")
    print("| seed | IC wall ms | IC cpu ms | table ms | relations ms | LA ms | trials "
          "| tuned rho ms | rho ops | S=ops/√n | IC/tuned rho | scalars agree |")
    print("|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|:--|")
    st = {t[0]: t for t in parse_timing(D / "a3_36bit_seeds_timing.txt")}
    rho = {r["target_seed"]: r for r in rows(D / "a3_36bit_rho_paired.jsonl")}
    for r in rows(D / "a3_36bit_u64_seeds.jsonl"):
        s = r["ic_stages"]
        seed = r["target_seed"]
        t = st.get(str(seed))
        cpu = round(t[2] * 1e3) if t and t[2] else "-"
        rr = rho[seed]
        ratio = (s["total_ms"] / 1e3) / rr["seconds"]
        print(f"| {seed} | {s['total_ms']:.0f} | {cpu} | {s['pair_table_ms']:.0f} "
              f"| {s['relations_ms']:.0f} | {s['linear_algebra_ms']:.0f} | {s['trials_total']} "
              f"| {1000*rr['seconds']:.1f} | {rr['ops']} "
              f"| {rr['ops']/(68719022539**0.5):.3f} | {ratio:.0f}× "
              f"| {rr.get('rho_agrees')} |")


if __name__ == "__main__":
    main()
