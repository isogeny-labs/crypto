#!/usr/bin/env python3
"""Counted S0 and d of the compact-orbit Koblitz producer, section 5a band, crossover G.

Reads runs/*.stdout + runs/*.jsonl (this follow-up's runs, n = 41..73) and the
recorded n = 83 files; writes results.json and prints the tables.

Units.  One GAE = one batched affine addition on the same field, priced as
5 field multiplications (3 for the shared inversion, 2 in the addition; the
squaring is linear in characteristic 2 and the 1/lanes inversion share is
dropped).  A root call (one S3 solve) is priced from its field-multiplication
count in the producer's code:
  u64 path (n <= 63), unbatched:  8.5 M + one Itoh-Tsujii inversion
  wide path (n >= 71), batched over 2n inputs: 10.5 M + inversion / (2n)
Probes (canonical rotation + hash lookup) and table inserts are unpriced native
work, priced at w in {0, 0.1, 1} GAE, as are rho's canonicalisations.
Variant A prices w on rho only (IC-favourable); variant B on both arms.
A relation check (lift) is charged 3 unbatched additions (I + 2 M each).
"""
import glob, json, math, os, random, re, statistics as st, sys

HERE = os.path.dirname(os.path.abspath(__file__))
N83 = sys.argv[1] if len(sys.argv) > 1 else "/Volumes/SSD990-2/crypto/research/20261016-n83-precomputed-rho-control/records/run-38855595549"
R = {41: 549756390943, 53: 21044858204113, 61: 162888033982417, 73: 86020738150056119, 83: 8569786107849059}
GAE_M = 5.0
WS = (0.0, 0.1, 1.0)


def it_mults(n):
    e = n - 1
    bits = e.bit_length()
    return (bits - 1) + sum((e >> i) & 1 for i in range(bits - 1))


def root_call_gae(n):
    if n <= 63:
        return (8.5 + it_mults(n)) / GAE_M
    return (10.5 + it_mults(n) / (2 * n)) / GAE_M


def lift_gae(n):
    return 3 * (it_mults(n) + 2) / GAE_M


def batch_k(k):
    # K(k) = 2k C(2k,k) / 4^k
    return 2 * k * math.exp(math.lgamma(2 * k + 1) - 2 * math.lgamma(k + 1) - 2 * k * math.log(2))


def best_g(s0, d, rho1):
    best = (float("inf"), None)
    k = 1.0
    while k < 1e12:
        kk = int(k)
        g = (s0 + kk * d) / (rho1 * batch_k(kk))
        if g < best[0]:
            best = (g, kk)
        k *= 1.05
    return best


def load_run(stdout_path):
    s = json.loads(open(stdout_path).read().strip().splitlines()[-1])
    recs = [json.loads(l) for l in open(stdout_path.replace(".stdout", ".jsonl"))]
    return s, recs


def costs(n, s, recs, cr, w):
    """IC counted S0 and per-target d list, in GAE, at probe/insert weight w and root-call price cr."""
    lg = lift_gae(n)
    build = s["index_pair_candidates"] if "index_pair_candidates" in s else s["regular_states"]
    inserts = s.get("root_table_entries", 0)
    rank_rc = s.get("rank_root_calls_total")
    rank_pr = s.get("rank_probes_total") or s["rank_probes_mean"] * s["rank_relations"]
    if rank_rc is None:  # K3300 record: probes = 2 root calls (both roots always exist)
        rank_rc = rank_pr / 2
    s0 = build * cr + inserts * w + rank_rc * cr + rank_pr * w + s["rank_relations"] * lg
    ds = []
    for r in recs:
        rc = r.get("root_calls", r["probes"] / 2)
        ds.append(rc * cr + r["probes"] * w + r.get("relation_checks", 1) * lg)
    parts = dict(build_root_calls=build, inserts=inserts, rank_root_calls=rank_rc, rank_probes=rank_pr,
                 target_root_calls_mean=st.mean(r.get("root_calls", r["probes"] / 2) for r in recs),
                 target_probes_mean=st.mean(r["probes"] for r in recs))
    return s0, ds, parts


def rho_counts(n):
    """Single-target rho: additions and canonicalisations per walk."""
    if n == 83:
        rho = [json.load(open(f))["outcome"]["charges_setup_and_solve"] for f in glob.glob(f"{N83}/k3300/rho_plain/w*.json")]
        return st.mean(r["group_additions"] for r in rho), st.mean(r["canonicalizations"] for r in rho), "measured, 16 walks"
    floor = math.sqrt(math.pi * (R[n] / (2 * n)) / 2)
    return floor, floor, "floor sqrt(pi N'/2), N' = r/2n (repo counted/floor 1.02-1.08)"


def main():
    rows = []
    paths = sorted(glob.glob(os.path.join(HERE, "runs", "n*_T*.stdout")))
    runs = []
    for p in paths:
        if os.path.getsize(p) == 0:
            continue
        m = re.search(r"n(\d+)_K(\d+)_N(\d+)_T(\d+)_s(\d)", p)
        n, K, N, T, sym = map(int, m.groups())
        s, recs = load_run(p)
        assert s["targets_solved"] == s["targets"] == len(recs), p
        runs.append((n, f"K{K} norm{N}" + (" sym" if sym else ""), s, recs))
    for K in ("k600", "k3300"):
        s = json.loads(open(f"{N83}/{K}/ic/stdout.log").read())
        recs = [json.loads(l) for l in open(f"{N83}/{K}/ic/records.jsonl")]
        runs.append((83, K.upper() + " (recorded, a=1)", s, recs))
    rng = random.Random(20261030)
    for n, label, s, recs in runs:
        cr = root_call_gae(n)
        ga, ca, rho_src = rho_counts(n)
        Np = R[n] / (2 * n)
        row = dict(n=n, recipe=label, root_call_gae=cr, rho_source=rho_src, targets=len(recs), bands={})
        for variant in ("A", "B"):
            for w in WS:
                s0, ds, parts = costs(n, s, recs, cr, w if variant == "B" else 0.0)
                rho1 = ga + w * ca
                d = st.mean(ds)
                g, kopt = best_g(s0, d, rho1)
                boots = [g, g]
                if (variant, w) in (("A", 0.0), ("B", 1.0)):
                    boots = sorted(best_g(s0, st.mean(rng.choice(ds) for _ in ds), rho1)[0] for _ in range(400))
                lo_i, hi_i = (10, 389) if len(boots) == 400 else (0, 1)
                row["bands"][f"{variant}{w}"] = dict(S0=s0, d=d, rho1=rho1, sigma=s0 / rho1, delta=d / rho1,
                                                     sigma_delta=s0 * d / rho1 ** 2, G=g, G_lo=boots[lo_i], G_hi=boots[hi_i],
                                                     k_opt=kopt, S0d_over_Nprime=s0 * d / Np)
                row["parts"] = parts
        # unit-price lower bound: root call = 1 GAE, w = 0
        s0, ds, _ = costs(n, s, recs, 1.0, 0.0)
        row["unit_lb"] = dict(S0=s0, d=st.mean(ds), G=best_g(s0, st.mean(ds), ga)[0], S0d_over_Nprime=s0 * st.mean(ds) / Np)
        # root-call price at which G = 1 (variant A, w = 1, i.e. the hardest rho)
        lo, hi = 0.01, 100.0
        for _ in range(60):
            mid = math.sqrt(lo * hi)
            s0, ds, _ = costs(n, s, recs, mid, 0.0)
            if best_g(s0, st.mean(ds), ga + ca)[0] < 1:
                lo = mid
            else:
                hi = mid
        row["root_call_gae_for_G1_at_A1"] = hi
        row["timing_ms"] = s.get("timing_ms")
        # d profile: counts per target, and the producer's exclusive per-target laps (practicality only)
        prof = dict(root_calls=st.mean(r.get("root_calls", r["probes"] / 2) for r in recs),
                    probes=st.mean(r["probes"] for r in recs),
                    probes_median=st.median(r["probes"] for r in recs),
                    probes_max=max(r["probes"] for r in recs),
                    relation_checks=st.mean(r.get("relation_checks", 1) for r in recs))
        for key in ("target_query_ms", "target_pdp_ms", "target_relation_check_ms", "target_descent_ms", "target_recovery_check_ms", "target_ms"):
            prof[key] = st.mean(r[key] for r in recs)
        row["d_profile"] = prof
        for tag in ("B0.0", "B1.0"):
            b = row["bands"][tag]
            b["cold_S"] = b["S0"] + b["d"]
        rows.append(row)
    json.dump(rows, open(os.path.join(HERE, "results.json"), "w"), indent=1)
    print("| n | recipe | build rc | rank rc | d rc (mean) | d probes | S0 B0 [GAE] | d B0 [GAE] | S0*d/N' (B0) | G A0 | G A1 | G B1 | rc price for G=1 at A1 |")
    for r in rows:
        p, b = r["parts"], r["bands"]
        print(f"| {r['n']} | {r['recipe']} | {p['build_root_calls']:.3g} | {p['rank_root_calls']:.3g} | {p['target_root_calls_mean']:.3g} | {p['target_probes_mean']:.3g} | "
              f"{b['B0.0']['S0']:.3g} | {b['B0.0']['d']:.3g} | {b['B0.0']['S0d_over_Nprime']:.2f} | "
              f"{b['A0.0']['G']:.2f} [{b['A0.0']['G_lo']:.2f},{b['A0.0']['G_hi']:.2f}] | {b['A1.0']['G']:.2f} | {b['B1.0']['G']:.2f} | {r['root_call_gae_for_G1_at_A1']:.2f} |")


if __name__ == "__main__":
    main()
