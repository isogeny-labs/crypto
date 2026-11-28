#!/usr/bin/env python3
"""PROTOCOL section 2 selection and G2, recomputed from the search rows with the validator's own code.

For every search chunk: the seed list is rebuilt from the file name exactly as search.sh builds it,
the cell's `tried` must equal its length, the validator's own irreducibility test (jac.is_irreducible)
must reproduce the cell's f_reducible_even_order count, every admitted seed must lie in the chunk and
have f = random_f(seed), and the rejects must add up.  Coverage must be exactly 1..N with no seed in two
chunks.  Selection = smallest admitted seeds.  Every admitted curve gets a group-order certificate:
f squarefree and irreducible, r prime (deterministic Miller-Rabin), r inside the Hasse-Weil interval
and wider than it (so r is the only multiple of r there), and [r]D = 0 for 3 random nonzero divisors D
drawn with the validator's RNG; hence #J = r.  Also: F_p-rational affine automorphisms of y^2 = f.
Writes validation/out/curves_check.json.
"""
import glob, json, os, random, re, sys, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import jac

V = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
S = os.path.dirname(V)
nonce = open(os.path.join(V, "out", "nonce.txt")).read().split()[0]
CELLS = {(4, 54293): 3, (5, 2357): 5, (5, 4099): 3}


def chunk_seeds(lo, hi, w, j):
    s = list(range(lo, hi + 1))
    mine = s[w::4]
    return mine[j:j + 28]


def scan(pattern, g, p):
    files = sorted(glob.glob(pattern))
    covered, admitted, chunks, problems = {}, {}, [], []
    for f in files:
        m = re.search(r"_r(\d+)-(\d+)_w(\d)_c(\d+)\.jsonl$", f)
        lo, hi, w, j = map(int, m.groups())
        seeds = chunk_seeds(lo, hi, w, j)
        rows = [json.loads(l) for l in open(f) if l.strip()]
        cell = [r for r in rows if r["kind"] == "cell"]
        inst = [r for r in rows if r["kind"] == "instance"]
        cfg = [r for r in rows if r["kind"] == "config"]
        if len(cell) != 1:
            problems.append(f"{os.path.basename(f)}: {len(cell)} cell rows")
            continue
        c = cell[0]
        if not (cfg and cfg[0]["max_h"] == 1 and cfg[0]["primes"] == [p] and cfg[0]["genera"] == [g]):
            problems.append(f"{os.path.basename(f)}: config row mismatch")
        if c["tried"] != len(seeds) or c["g"] != g or c["p"] != p:
            problems.append(f"{os.path.basename(f)}: tried {c['tried']} vs {len(seeds)} seeds")
        irr = [cs for cs in seeds if jac.is_irreducible(jac.random_f(p, g, cs), p)]
        red = len(seeds) - len(irr)
        rej = c["rejects"]
        if rej.get("f_reducible_even_order", 0) != red:
            problems.append(f"{os.path.basename(f)}: reducible {rej.get('f_reducible_even_order', 0)} vs validator {red}")
        if sum(rej.values()) + c["admitted"] != c["tried"]:
            problems.append(f"{os.path.basename(f)}: rejects + admitted != tried")
        if c["admitted"] != len(inst):
            problems.append(f"{os.path.basename(f)}: admitted {c['admitted']} vs {len(inst)} instance rows")
        for r in inst:
            cs = r["curve_seed"]
            if cs not in irr:
                problems.append(f"{os.path.basename(f)}: admitted {cs} not irreducible / not in chunk")
            if r["f"] != jac.random_f(p, g, cs) or r["h"] != 1 or r["order"] != r["r"] or r["g"] != g or r["p"] != p:
                problems.append(f"{os.path.basename(f)}: instance row {cs} fields")
            admitted[cs] = r
        for cs in seeds:
            covered.setdefault(cs, []).append(os.path.basename(f))
        chunks.append({"file": os.path.basename(f), "seeds": len(seeds), "irreducible": len(irr),
                       "rejects": rej, "admitted": [r["curve_seed"] for r in inst]})
    return files, covered, admitted, chunks, problems


def certify_curve(p, g, row, rnd):
    f, r = row["f"], row["r"]
    J = jac.Jac(p, g, f)
    lo, hi = jac.hasse_weil(p, g)
    divs = []
    for _ in range(3):
        D = J.ID
        while D == J.ID:
            placed, D = 0, J.ID
            while placed < g:
                x = rnd.randrange(p)
                y = jac.sqrt_mod_spec(jac.peval(f, x, p), p)
                if y is None:
                    continue
                if rnd.random() < 0.5:
                    y = (-y) % p
                D = J.add(D, J.point(x, y))
                placed += 1
        divs.append(D)
    rD = [J.mul(D, r) == J.ID for D in divs]
    t0 = time.time()
    aut = jac.affine_automorphisms(f, p, g)
    c = {"f_eq_random_f": f == jac.random_f(p, g, row["curve_seed"]), "monic_deg": len(f) - 1 == 2 * g + 1 and f[-1] == 1,
         "squarefree": jac.is_squarefree(f, p), "irreducible": jac.is_irreducible(f, p),
         "r_prime": jac.is_prime(r), "order_eq_r": row["order"] == r, "h": row["h"],
         "hasse_weil": [lo, hi], "r_in_hw": lo <= r <= hi, "r_gt_hw_width": r > hi - lo,
         "rD_zero_for_3_random_D": rD, "divisors_valid": all(J.is_valid(D) for D in divs),
         "affine_automorphisms": aut, "only_identity_automorphism": aut == [(1, 0, True)],
         "log2_r": r.bit_length(), "aut_scan_s": round(time.time() - t0, 1)}
    c["J_order_certified_eq_r"] = all([c["f_eq_random_f"], c["monic_deg"], c["squarefree"], c["r_prime"], c["order_eq_r"],
                                       c["r_in_hw"], c["r_gt_hw_width"], all(rD), c["divisors_valid"], c["h"] == 1])
    return c


out = {"cells": {}, "problems": []}
curves_json = json.load(open(os.path.join(S, "curves.json")))
for (g, p), need in CELLS.items():
    files, covered, admitted, chunks, problems = scan(os.path.join(S, f"search_g{g}_p{p}_r*.jsonl"), g, p)
    N = max(covered)
    dup = sorted(cs for cs, fs in covered.items() if len(fs) > 1)
    missing = sorted(set(range(1, N + 1)) - set(covered))
    seeds = sorted(admitted)
    sel = {"confirm": seeds[:3]}
    if (g, p) == (5, 2357):
        sel["tune"] = seeds[3:5]
    cj = curves_json[f"g{g}_p{p}"]
    rnd = random.Random(f"{nonce}|{g}|{p}")
    certs = {str(cs): certify_curve(p, g, admitted[cs], rnd) for cs in seeds}
    cell = {"files": len(files), "covered_max": N, "seeds_covered": len(covered), "duplicates": dup, "missing": missing,
            "complete_1_to_N": not dup and not missing, "admitted": seeds, "selection": sel,
            "irreducible_total": sum(c["irreducible"] for c in chunks),
            "reducible_total": sum(c["seeds"] - c["irreducible"] for c in chunks),
            "order_rejects_total": sum(c["rejects"].get("order_not_near_prime", 0) for c in chunks),
            "curves_json_agrees": cj["admitted"] == seeds and cj["confirm"] == sel["confirm"] and cj.get("tune") == sel.get("tune")
            and cj["tried"] == len(covered) and cj["complete"] is True,
            "enough": len(seeds) >= need, "problems": problems, "certs": certs}
    out["cells"][f"g{g}_p{p}"] = cell
    out["problems"] += problems
    print(f"g{g} p{p}: N={N} complete={cell['complete_1_to_N']} admitted={seeds} sel={sel} agrees={cell['curves_json_agrees']} "
          f"irr={cell['irreducible_total']} problems={len(problems)}")
    for cs, c in certs.items():
        print("  ", cs, "J=r certified:", c["J_order_certified_eq_r"], "irreducible:", c["irreducible"], "aut:", c["affine_automorphisms"])

# D1 evidence: the p = 60013 chunks; the 'other' rejects must be exactly the irreducible seeds
files, covered, admitted, chunks, problems = [], {}, {}, [], []
irr_tot, other_tot, red_ok = 0, 0, True
for f in sorted(glob.glob(os.path.join(S, "failed_p60013", "search_g4_p60013_r*.jsonl"))):
    m = re.search(r"_r(\d+)-(\d+)_w(\d)_c(\d+)\.jsonl$", f)
    lo, hi, w, j = map(int, m.groups())
    seeds = chunk_seeds(lo, hi, w, j)
    c = [json.loads(l) for l in open(f) if '"cell"' in l][0]
    irr = sum(jac.is_irreducible(jac.random_f(60013, 4, cs), 60013) for cs in seeds)
    irr_tot += irr
    other_tot += c["rejects"].get("other", 0)
    red_ok &= c["rejects"].get("f_reducible_even_order", 0) == len(seeds) - irr and c["admitted"] == 0
    for cs in seeds:
        covered.setdefault(cs, []).append(f)
hw = jac.hasse_weil(60013, 4)
out["p60013"] = {"seeds_covered": len(covered), "irreducible": irr_tot, "other_rejects": other_tot,
                 "other_eq_irreducible": irr_tot == other_tot, "reducible_counts_match": red_ok,
                 "guard_value_(sqrt_p+1)^8": (60013 ** 0.5 + 1) ** 8, "guard_9e18_refuses": (60013 ** 0.5 + 1) ** 8 >= 9.0e18,
                 "guard_value_p54293": (54293 ** 0.5 + 1) ** 8, "guard_admits_54293": (54293 ** 0.5 + 1) ** 8 < 9.0e18,
                 "largest_prime_admitted_by_guard": max(q for q in range(54000, 60014) if jac.is_prime(q) and (q ** 0.5 + 1) ** 8 < 9.0e18),
                 "hasse_weil_60013": hw}
print(json.dumps(out["p60013"], indent=1))

# instance rows in the run files agree with the search rows
runinst = {}
for f in glob.glob(os.path.join(S, "b4_*.jsonl")) + glob.glob(os.path.join(S, "b5_*.jsonl")) + glob.glob(os.path.join(S, "tune5_*.jsonl")) + glob.glob(os.path.join(S, "rho5_*.jsonl")):
    for l in open(f):
        r = json.loads(l)
        if r["kind"] == "instance":
            key = f"g{r['g']}_p{r['p']}"
            srch = None
            for k, cell in out["cells"].items():
                pass
            runinst.setdefault(os.path.basename(f), []).append(r["curve_seed"])
            # compare with the search instance row
            g, p = r["g"], r["p"]
            cert = out["cells"][key]["certs"].get(str(r["curve_seed"]))
            if cert is None:
                out["problems"].append(f"{f}: curve {r['curve_seed']} not an admitted search seed")
            elif r["f"] != jac.random_f(p, g, r["curve_seed"]) or not cert["J_order_certified_eq_r"]:
                out["problems"].append(f"{f}: curve {r['curve_seed']} instance differs")
out["run_file_curves"] = runinst
expect = {"b4": curves_json["g4_p54293"]["confirm"], "b5_p2357": curves_json["g5_p2357"]["confirm"],
          "b5_p4099": curves_json["g5_p4099"]["confirm"], "tune5": curves_json["g5_p2357"]["tune"],
          "rho5": curves_json["g5_p2357"]["confirm"][:1]}
out["run_files_use_selected_curves"] = {f: sorted(v) == sorted(next(e for k, e in expect.items() if f.startswith(k)))
                                        for f, v in runinst.items()}
json.dump(out, open(os.path.join(V, "out", "curves_check.json"), "w"), indent=1)
print("run files use selected curves:", out["run_files_use_selected_curves"])
print("problems:", out["problems"])
