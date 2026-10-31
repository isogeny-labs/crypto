#!/usr/bin/env python3
"""Replay-verify and analyse a run.py output directory (PROTOCOL.md).

Usage: analyze.py <results_dir> [--out analysis.json] [--points N]
Verification (any failure marks the run INVALID):
  scalar per point == sealed answer for every IC arm and every rho draw;
  one relation (point_indices, probes), rank attempts/relations/probe mean,
  and base dump identical across the three IC arms of each point;
  padd arm: rank_solve=padd, 0 fallback states, 0 irregular inputs,
  resolved_hits == rank_relations; cur arms: rank_solve s3/absent.
"""
import argparse, hashlib, json, math, random, statistics, sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
BOOT = 10000
SEED = 20261019
T975_DF19 = 2.093024054408263
AA_BAND = (0.97, 1.03)
KILL_RANK = 1.2
GAE_BAND = (0.0, 0.1, 1.0)


def jl(path):
    return [json.loads(l) for l in Path(path).read_text().splitlines() if l.strip()]


def one(path):
    return json.loads(Path(path).read_text())


def find(obj, key):
    if isinstance(obj, dict):
        if key in obj:
            return obj[key]
        for v in obj.values():
            r = find(v, key)
            if r is not None:
                return r
    return None


def ic_run(folder, answer):
    usage = one(folder / "usage.json")
    if usage["status"] != "PASS_PRODUCER":
        raise ValueError(f"{folder.name}: {usage['status']}")
    rows = jl(folder / "targets_result.jsonl")
    summary = next(r for r in jl(folder / "stdout.jsonl") if r.get("kind") == "compact_orbit_dlp_summary")
    target = next(r for r in rows if r.get("kind") == "compact_orbit_dlp_target")
    if int(target["recovered_scalar"]) != int(answer["scalar"]) or target["group_verified"] is not True:
        raise ValueError(f"{folder.name}: scalar differs from sealed answer")
    t = summary["timing_ms"]
    return {"cold_s": usage["wall_seconds"], "rank_s": t["rank_stage"] / 1000,
            "setup_s": t["setup_total"] / 1000, "process_s": t["process_total"] / 1000,
            "target_ms": float(target["target_ms"]), "rss": usage.get("peak_rss_bytes"),
            "relation": (target["point_indices"], target["probes"]),
            "rank_id": (summary["rank_attempts"], summary["rank_relations"],
                        summary["rank_failures"], summary["rank_rows_without_gain"],
                        summary["rank_probes_mean"], summary["rank"]),
            "base_sha": hashlib.sha256((folder / "base_dump.jsonl").read_bytes()).hexdigest(),
            "summary": summary, "target_probes": target["probes"]}


def rho_run(folder, point, answer, seed):
    usage = one(folder / "usage.json")
    if usage["status"] != "PASS_PRODUCER":
        raise ValueError(f"{folder.name}: {usage['status']}")
    r = one(folder / "receipt.json")
    if (r["status"] != "PASS_VERIFIED_PUBLIC_TARGET" or int(r["recovered_scalar"]) != int(answer["scalar"])
            or r["public_target_q"] != point or r["walk_seed"] != seed
            or r["generic_scalar_replay_verified"] is not True):
        raise ValueError(f"{folder.name}: rho replay failed")
    return {"online_ms": float(r["online_ms"]), "wall_s": usage["wall_seconds"],
            "charges": find(r, "charges_setup_and_solve")}


def boot_ratio(num, den, rng, n=BOOT):
    """num, den: per-point lists; resample points; ratio of means."""
    k = len(num)
    out = []
    for _ in range(n):
        idx = [rng.randrange(k) for _ in range(k)]
        out.append(sum(num[i] for i in idx) / sum(den[i] for i in idx))
    out.sort()
    return out[int(0.025 * n)], out[int(0.975 * n)]


def boot_mean_diff(d, rng, n=BOOT):
    k = len(d)
    out = sorted(sum(d[rng.randrange(k)] for _ in range(k)) / k for _ in range(n))
    return out[int(0.025 * n)], out[int(0.975 * n)]


def ic_over_rho(ic, rho_by_point, rng, n=BOOT):
    """ic: per-point cold s; rho_by_point: list of per-point lists of rho s."""
    k = len(ic)
    pts = []
    out = []
    for _ in range(n):
        idx = [rng.randrange(k) for _ in range(k)]
        num = sum(ic[i] for i in idx) / k
        draws = [rng.choice(rho_by_point[i]) for i in idx for _ in rho_by_point[i]]
        out.append(num / (sum(draws) / len(draws)))
    out.sort()
    return out[int(0.025 * n)], out[int(0.975 * n)]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("results", type=Path)
    ap.add_argument("--out", type=Path)
    ap.add_argument("--points", type=int, default=20)
    a = ap.parse_args()
    res = a.results
    source = one(res / "source.json")
    targets = [json.loads(l) for l in (HERE / "targets.jsonl").read_text().splitlines()][:a.points]
    answers = jl(HERE / "answers.jsonl")[:a.points]
    problems, pts = [], []
    for i in range(1, a.points + 1):
        try:
            arms = {arm: ic_run(res / "runs" / f"p{i}_{arm}", answers[i - 1])
                    for arm in ("cur_a", "cur_b", "padd")}
            rhos = [rho_run(res / "runs" / f"p{i}_rho{d}", targets[i - 1], answers[i - 1],
                            source["rho_seed_base"] + 10 * i + d) for d in range(source["rho_draws"])]
            ref = arms["cur_a"]
            for name, r in arms.items():
                if (r["relation"], r["rank_id"], r["base_sha"]) != (ref["relation"], ref["rank_id"], ref["base_sha"]):
                    raise ValueError(f"p{i}: {name} relation/rank/base differs from cur_a")
            p = arms["padd"]["summary"]
            if (p.get("rank_solve") != "padd" or p["rank_padd_fallback_states"] != 0
                    or p["rank_padd_irregular_inputs"] != 0
                    or p["rank_padd_resolved_hits"] != p["rank_relations"]):
                raise ValueError(f"p{i}: padd counters unexpected {p.get('rank_padd_resolved_hits')}")
            for name in ("cur_a", "cur_b"):
                if arms[name]["summary"].get("rank_solve", "s3") != "s3":
                    raise ValueError(f"p{i}: {name} not s3")
            pts.append({"i": i, "arms": arms, "rho": rhos})
        except Exception as e:  # noqa: BLE001
            problems.append(str(e))
    out = {"schema": "n83.padd-rank-analysis/v1", "points_requested": a.points,
           "points_valid": len(pts), "problems": problems}
    if problems or len(pts) != a.points:
        out["status"] = "INVALID"
    else:
        rng = random.Random(SEED)
        f = lambda key, arm: [p["arms"][arm][key] for p in pts]
        cur = lambda key: [(x + y) / 2 for x, y in zip(f(key, "cur_a"), f(key, "cur_b"))]
        res_ = {}
        for key in ("cold_s", "rank_s", "setup_s", "target_ms"):
            c, pa, ca, cb = cur(key), f(key, "padd"), f(key, "cur_a"), f(key, "cur_b")
            d = [x - y for x, y in zip(c, pa)]
            m, sd = statistics.mean(d), statistics.stdev(d)
            res_[key] = {
                "cur_mean": statistics.mean(c), "padd_mean": statistics.mean(pa),
                "paired_saving_mean": m,
                "paired_saving_t95": [m - T975_DF19 * sd / math.sqrt(len(d)), m + T975_DF19 * sd / math.sqrt(len(d))],
                "paired_saving_boot95": list(boot_mean_diff(d, rng)),
                "ratio_cur_over_padd": sum(c) / sum(pa),
                "ratio_boot95": list(boot_ratio(c, pa, rng)),
                "aa_ratio_a_over_b": sum(ca) / sum(cb),
                "aa_boot95": list(boot_ratio(ca, cb, rng)),
                "per_point_saving": d}
        aa_ok = all(AA_BAND[0] <= res_[k]["aa_ratio_a_over_b"] <= AA_BAND[1] for k in ("cold_s", "rank_s"))
        out["metrics"] = res_
        out["aa_control"] = {"band": AA_BAND, "pass": aa_ok}
        rank_x = res_["rank_s"]["ratio_cur_over_padd"]
        cold_lo = res_["cold_s"]["paired_saving_boot95"][0]
        out["kill_condition"] = {"rank_speedup_threshold": KILL_RANK, "rank_speedup": rank_x,
                                 "idea_does_not_transfer": rank_x < KILL_RANK}
        out["verdict"] = ("INVALID_AA" if not aa_ok else
                          "IMPROVEMENT" if cold_lo > 0 else
                          "REGRESSION" if res_["cold_s"]["paired_saving_boot95"][1] < 0 else "NOT_SEPARATED")
        rho_s = [[r["online_ms"] / 1000 for r in p["rho"]] for p in pts]
        rho_wall = [[r["wall_s"] for r in p["rho"]] for p in pts]
        mean_rho = statistics.mean(x for row in rho_s for x in row)
        sd_rho = statistics.stdev(x for row in rho_s for x in row)
        ratios = {}
        for label, vals in (("cur", cur("cold_s")), ("padd", f("cold_s", "padd"))):
            ratios[label] = {
                "ic_over_rho_online": statistics.mean(vals) / mean_rho,
                "boot95": list(ic_over_rho(vals, rho_s, rng)),
                "ic_over_rho_child_wall": statistics.mean(vals) / statistics.mean(x for r in rho_wall for x in r),
                "boot95_child_wall": list(ic_over_rho(vals, rho_wall, rng)),
                "sealed5_ic_over_rho_online": statistics.mean(vals[:5]) / statistics.mean(x for r in rho_s[:5] for x in r)}
        out["rho"] = {"draws": sum(len(r) for r in rho_s), "mean_online_s": mean_rho,
                      "sd_online_s": sd_rho, "cv": sd_rho / mean_rho, "ratios": ratios}
        # section 5a: unpriced native work, every arm, band 0 / 0.1 / 1 GAE per native operation
        def ic_work(arm):
            tot = {"rank_probes": 0.0, "target_probes": 0.0, "rank_relations": 0.0}
            for p in pts:
                s = p["arms"][arm]["summary"]
                tot["rank_probes"] += s["rank_probes_mean"] * s["rank_relations"]
                tot["target_probes"] += p["arms"][arm]["target_probes"]
                tot["rank_relations"] += s["rank_relations"]
            return {k: v / len(pts) for k, v in tot.items()}
        w = {"cur": ic_work("cur_a"), "padd": ic_work("padd")}
        rho_ch = {k: statistics.mean(r["charges"][k] for p in pts for r in p["rho"])
                  for k in pts[0]["rho"][0]["charges"]}
        band = {}
        for g in GAE_BAND:
            ic_native = lambda x: x["rank_probes"] + x["target_probes"]   # 1 canonicalisation + 1 table query per probe, root solve per 2 probes
            ic_cost = {a_: x["rank_relations"] * 0 + g * (2 * ic_native(x) + ic_native(x) / 2) for a_, x in w.items()}
            rho_native = sum(rho_ch[k] for k in rho_ch if k not in ("group_additions", "scalar_multiplications", "failed_collisions"))
            rho_cost = rho_ch["group_additions"] + g * rho_native
            band[str(g)] = {"rho_gae": rho_cost, "ic_unpriced_native_ops_gae": ic_cost,
                            "note": "IC group additions (lifts, target descent) are charged at 1 each elsewhere; here only the unpriced native work is priced"}
        out["section_5a"] = {"ic_native_per_run": w, "rho_charges_mean": rho_ch, "band_gae": band,
                             "unpriced_ic_counters": ["canonicalizations (1/probe)", "table_queries (1/probe)",
                                                      "root_solves (1 per 2 probes: S3 or point-addition)"],
                             "caveat": "probe, canonicalisation and table-query counts are identical in every IC arm; "
                                       "the padd change moves field work between S3 root solves and point-addition "
                                       "partner roots at equal native-operation counts, so a GAE-priced S is equal "
                                       "across the two IC arms: the gain is an implementation (wall-clock) gain."}
        out["status"] = "COMPLETE"
        out["rss_bytes_max"] = max(p["arms"][a_]["rss"] or 0 for p in pts for a_ in p["arms"])
    text = json.dumps(out, indent=1, sort_keys=True, default=list)
    if a.out:
        a.out.write_text(text + "\n")
    print(text)
    sys.exit(0 if out["status"] == "COMPLETE" else 1)


main()
