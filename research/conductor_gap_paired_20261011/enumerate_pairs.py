#!/usr/bin/env python3
"""Selection stage: enumerate F_{p^2} curve classes, group by (q, t, N, r), certify f_E,
select pairs with distinct certified f_E.  Writes curves.jsonl, isogeny_edges.jsonl,
selection_receipt.json (and selection.log via stdout).  See PROTOCOL.md sections 3-4
and Amendment 1.

Python prototype under a task-level exception to the no-Python rule (PROTOCOL.md s.1).
Run:  timeout 1500 python3 enumerate_pairs.py
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
import platform
import random
import subprocess
import sys
import time
from fractions import Fraction

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)

from cgp_field import Fq, factorint, is_prime  # noqa: E402
from cgp_curve import (  # noqa: E402
    Counter, Curve, class_number, disc_decomposition, enumerate_classes, j_invariant,
    kronecker_class_number, neighbours, twist_partner_index,
)
from cgp_certify import SUPPORTED_ELL, certify_curve  # noqa: E402

DEFAULT_PRIMES = (19, 23, 29, 31, 37, 41, 43)


def curve_id(p: int, j: int, w: int) -> str:
    return f"cgp-p{p}-j{j}-w{w}"


def largest_prime_factor(n: int) -> int:
    return max(factorint(n)) if n > 1 else 1


def factor_base_size(F: Fq, a: int, b: int) -> int:
    """Number of x in F_p with x^3 + a x + b a square in F_q (x-classes of the base)."""
    E = Curve(F, a, b)
    return sum(1 for x in range(F.p) if F.is_square(E.f(x)))


def find_path(F, a, b, ell, j_target, N, counter, max_depth=3):
    """BFS in the rational l-isogeny graph from (a, b) to j_target; the path as a list of
    {from_j, to_j, kernel_poly}.  None if not found within max_depth."""
    j0 = j_invariant(F, a, b)
    if j0 == j_target:
        return []
    models = {j0: (a, b)}
    parent = {j0: None}
    frontier = [j0]
    for _ in range(max_depth):
        nxt = []
        for jv in frontier:
            av, bv = models[jv]
            nb, _ = neighbours(F, av, bv, ell, random.Random(5), N, counter)
            for k in nb:
                if k["j"] in parent:
                    continue
                parent[k["j"]] = (jv, k["kernel"])
                models[k["j"]] = (k["a"], k["b"])
                if k["j"] == j_target:
                    path = []
                    cur = k["j"]
                    while parent[cur] is not None:
                        pj, ker = parent[cur]
                        path.append({"ell": ell, "from_j": pj, "to_j": cur, "kernel_poly": ker})
                        cur = pj
                    path.reverse()
                    return path
                nxt.append(k["j"])
        frontier = nxt
    return None


def git_info():
    def run(*a):
        try:
            return subprocess.check_output(a, cwd=HERE, text=True, stderr=subprocess.DEVNULL).strip()
        except Exception:
            return None
    return {"head": run("git", "rev-parse", "HEAD"),
            "dirty_files": (run("git", "status", "--porcelain", "--", ".") or "").splitlines()}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--primes", default=",".join(map(str, DEFAULT_PRIMES)))
    ap.add_argument("--cpu-budget-min", type=float, default=20.0)
    ap.add_argument("--curve-cap-s", type=float, default=90.0)
    ap.add_argument("--out", default=HERE)
    args = ap.parse_args()
    primes = sorted((int(x) for x in args.primes.split(",")), reverse=True)
    assert all(is_prime(p) and p >= 5 for p in primes)
    cpu0, wall0 = time.process_time(), time.perf_counter()
    budget_s = args.cpu_budget_min * 60

    curves_out = open(os.path.join(args.out, "curves.jsonl"), "w")
    edges_out = open(os.path.join(args.out, "isogeny_edges.jsonl"), "w")
    receipt = {
        "schema": "cgp.selection_receipt/v1",
        "protocol_sha256": hashlib.sha256(open(os.path.join(HERE, "PROTOCOL.md"), "rb").read()).hexdigest(),
        "command": "python3 " + " ".join(sys.argv),
        "primes_processed_order": primes, "cpu_budget_min": args.cpu_budget_min,
        "curve_cap_s": args.curve_cap_s, "supported_ell": list(SUPPORTED_ELL),
        "per_field": {}, "groups": [], "pairs": [], "trial_pairs": [],
        "git": git_info(), "python": sys.version, "platform": platform.platform(),
    }
    all_pairs = []
    edge_seen: set = set()
    edge_total_expanded = 0

    for p in primes:
        F = Fq(p)
        counter = Counter(F)
        q = F.q
        fld = {"p": p, "q": q, "theta_sq": F.n, "generator": F.gen, "classes": 2 * q + 6,
               "class_count_mismatches": [], "traces_checked": 0, "supersingular_classes": 0,
               "groups_f_pi_1_pos_t": 0, "classes_f_pi_1_pos_t": 0, "groups_pos_t_f_pi_gt1": 0,
               "ineligible": {}, "eligible_groups": 0, "certified_groups": 0, "not_reached_groups": 0,
               "curves_certified_ok": 0, "curves_cert_failed": 0, "cert_seconds_cpu": 0.0,
               "t_negative_mirror_classes": 0}
        classes = enumerate_classes(F)
        by_t: dict[int, list] = {}
        for (j, w, a, b) in classes:
            N = counter.count(a, b)
            by_t.setdefault(q + 1 - N, []).append((j, w, a, b, N))
        # class-count check for every ordinary trace, both signs
        for t, v in sorted(by_t.items()):
            if t % p == 0:
                fld["supersingular_classes"] += len(v)
                continue
            fld["traces_checked"] += 1
            delta, f, dk = disc_decomposition(t, q)
            H = sum(kronecker_class_number(f, dk).values())
            if H != len(v):
                fld["class_count_mismatches"].append({"t": t, "classes": len(v), "H": H})
            if t < 0:
                fld["t_negative_mirror_classes"] += len(v)
        print(f"[p={p}] {q=} classes {2*q+6}; ordinary traces checked {fld['traces_checked']}, "
              f"class-count mismatches {len(fld['class_count_mismatches'])}", flush=True)

        for t in sorted(k for k in by_t if k > 0 and k % p):
            v = by_t[t]
            delta, f, dk = disc_decomposition(t, q)
            N = q + 1 - t
            if f == 1:
                fld["groups_f_pi_1_pos_t"] += 1
                fld["classes_f_pi_1_pos_t"] += len(v)
                continue
            fld["groups_pos_t_f_pi_gt1"] += 1
            r = largest_prime_factor(N)
            reasons = []
            bad_l = [l for l in factorint(f) if l not in SUPPORTED_ELL]
            if bad_l:
                reasons.append(f"unsupported_ell:{bad_l}")
            if r < 4 * p:
                reasons.append("r<4p")
            if N % (r * r) == 0:
                reasons.append("r^2|N")
            gid = f"p{p}-t{t}"
            pred = kronecker_class_number(f, dk)
            grp = {"group": gid, "p": p, "q": q, "t": t, "N": N, "r": r, "cofactor": N // r,
                   "f_pi": f, "D_K": dk, "n_curves": len(v),
                   "class_number_prediction": {str(k): h for k, h in pred.items()},
                   "eligible": not reasons, "ineligible_reasons": reasons,
                   "fully_certified": False, "pair": None}
            if reasons:
                for rs in reasons:
                    key = rs.split(":")[0]
                    fld["ineligible"][key] = fld["ineligible"].get(key, 0) + 1
            if time.process_time() - cpu0 > budget_s and not reasons:
                grp["eligible"] = True
                grp["status"] = "not_reached"
                fld["not_reached_groups"] += 1
                reasons = ["not_reached"]
            if not reasons:
                fld["eligible_groups"] += 1
                grp["status"] = "certifying"
            else:
                grp.setdefault("status", "ineligible")

            recs = []
            for (j, w, a, b, Nn) in v:
                wp = twist_partner_index(j, w, F)
                rec = {"id": curve_id(p, j, w), "p": p, "q": q, "theta_sq": F.n, "a": a, "b": b,
                       "j": j, "j_in_Fp": F.in_fp(j), "twist_class": w,
                       "twist_partner_id": curve_id(p, j, wp), "twist_partner_trace": -t,
                       "N": Nn, "t": t, "ordinary": True, "r": r, "cofactor": N // r,
                       "f_pi": f, "D_K": dk, "group": gid, "eligible_group": not reasons,
                       "group_ineligible_reasons": reasons, "certificate": None, "fE": None,
                       "trial_eligible": not F.in_fp(j)}
                if not reasons:
                    c0 = time.process_time()
                    cert = certify_curve(F, counter, a, b, t, Nn, f, dk,
                                         deadline=time.perf_counter() + args.curve_cap_s)
                    fld["cert_seconds_cpu"] += time.process_time() - c0
                    edges = cert.pop("edges", [])
                    edge_total_expanded += len(edges)
                    cert["edges_expanded"] = len(edges)
                    rec["certificate"] = cert
                    rec["fE"] = cert["fE"] if cert["status"] == "ok" else None
                    rec["_edges"] = edges
                    if cert["status"] == "ok":
                        fld["curves_certified_ok"] += 1
                    else:
                        fld["curves_cert_failed"] += 1
                recs.append(rec)

            if not reasons:
                ok_all = all(r_["certificate"]["status"] == "ok" for r_ in recs)
                obs: dict[int, int] = {}
                for r_ in recs:
                    if r_["fE"] is not None:
                        obs[r_["fE"]] = obs.get(r_["fE"], 0) + 1
                grp["observed_distribution"] = {str(k): c for k, c in sorted(obs.items())}
                grp["class_check_ok"] = bool(ok_all and obs == pred)
                grp["fully_certified"] = grp["class_check_ok"]
                grp["status"] = "certified" if grp["fully_certified"] else "certification_incomplete_or_mismatch"
                if grp["fully_certified"]:
                    fld["certified_groups"] += 1
                    # ---- candidate pairs (distinct f_E, both trial-eligible)
                    te = [r_ for r_ in recs if r_["trial_eligible"]]
                    for r_ in te:
                        r_["fb_size"] = factor_base_size(F, r_["a"], r_["b"])
                    cands = []
                    for i in range(len(te)):
                        for k in range(i + 1, len(te)):
                            x_, y_ = te[i], te[k]
                            if x_["fE"] == y_["fE"]:
                                continue
                            lo, hi = (x_, y_) if (x_["fE"], x_["j"]) < (y_["fE"], y_["j"]) else (y_, x_)
                            fac1 = factorint(lo["fE"]) if lo["fE"] > 1 else {}
                            fac2 = factorint(hi["fE"]) if hi["fE"] > 1 else {}
                            diff_primes = sorted(l for l in set(fac1) | set(fac2)
                                                 if fac1.get(l, 0) != fac2.get(l, 0))
                            dec = []
                            for l in diff_primes:
                                b1 = lo["certificate"]["method_B"]["per_prime"][l]
                                b2 = hi["certificate"]["method_B"]["per_prime"][l]
                                if b1["decisive"] and b2["decisive"]:
                                    dec.append(l)
                            cands.append({"lo": lo, "hi": hi, "diff_primes": diff_primes,
                                          "B_decisive_primes": dec,
                                          "indep": bool(dec),
                                          "fbdiff": abs(lo["fb_size"] - hi["fb_size"])})
                    grp["n_candidate_pairs_distinct_fE"] = len(cands)
                    grp["n_candidate_pairs_independently_verified"] = sum(1 for c in cands if c["indep"])
                    verified = [c for c in cands if c["indep"]]
                    if verified:
                        verified.sort(key=lambda c: (c["fbdiff"], c["lo"]["j"], c["hi"]["j"]))
                        c = verified[0]
                        lo, hi = c["lo"], c["hi"]
                        ratio = Fraction(hi["fE"], lo["fE"])
                        path = None
                        for l in c["diff_primes"]:
                            pa = find_path(F, lo["a"], lo["b"], l, hi["j"], N, counter)
                            if pa is not None:
                                path = {"ell": l, "steps": pa}
                                break
                        same = [r_ for r_ in te if r_["fE"] == lo["fE"] and r_["j"] != lo["j"]]
                        same.sort(key=lambda r_: r_["j"])
                        pair = {
                            "group": gid, "p": p, "q": q, "t": t, "N": N, "r": r, "cofactor": N // r,
                            "f_pi": f, "D_K": dk, "lo_id": lo["id"], "hi_id": hi["id"],
                            "f_E_lo": lo["fE"], "f_E_hi": hi["fE"],
                            "conductor_ratio": f"{ratio.numerator}/{ratio.denominator}",
                            "index_prime_factors": {str(l): [factorint(lo["fE"]).get(l, 0) if lo["fE"] > 1 else 0,
                                                             factorint(hi["fE"]).get(l, 0) if hi["fE"] > 1 else 0]
                                                    for l in c["diff_primes"]},
                            "differing_primes": c["diff_primes"], "B_decisive_primes": c["B_decisive_primes"],
                            "independently_verified": True,
                            "j_lo": lo["j"], "j_hi": hi["j"], "twist_class_lo": lo["twist_class"],
                            "twist_class_hi": hi["twist_class"],
                            "twist_partner_lo": lo["twist_partner_id"], "twist_partner_hi": hi["twist_partner_id"],
                            "fb_size_lo": lo["fb_size"], "fb_size_hi": hi["fb_size"],
                            "M": min(lo["fb_size"], hi["fb_size"]),
                            "isogeny_path": path,
                            "same_conductor_control_id": same[0]["id"] if same else None,
                            "same_conductor_control_fb_size": same[0]["fb_size"] if same else None,
                        }
                        grp["pair"] = pair
                        all_pairs.append(pair)
                        lo["in_pair"] = hi["in_pair"] = True
            # ---- write curves and (filtered) edges
            pair_group = grp.get("pair") is not None
            for r_ in recs:
                edges = r_.pop("_edges", [])
                for e in edges:
                    if not (pair_group or e["label"] == "descending" or e["from_dist"] == 0):
                        continue
                    key = (e["ell"], e["from_j"], tuple(e["kernel_poly"]), gid)
                    if key in edge_seen:
                        continue
                    edge_seen.add(key)
                    edges_out.write(json.dumps({"group": gid, "p": p, "q": q, "t": t, **e}) + "\n")
                curves_out.write(json.dumps(r_) + "\n")
            receipt["groups"].append(grp)
        fld["cert_seconds_cpu"] = round(fld["cert_seconds_cpu"], 2)
        receipt["per_field"][str(p)] = fld
        print(f"[p={p}] groups(f_pi>1,t>0)={fld['groups_pos_t_f_pi_gt1']} eligible={fld['eligible_groups']} "
              f"certified={fld['certified_groups']} curves ok/failed={fld['curves_certified_ok']}/{fld['curves_cert_failed']} "
              f"ineligible={fld['ineligible']} cert_cpu={fld['cert_seconds_cpu']}s", flush=True)

    # ---- trial pairs: groups by p descending (then t ascending), first 4 with distinct (q, t)
    tp = sorted(all_pairs, key=lambda x: (-x["p"], x["t"]))
    seen = set()
    for pr in tp:
        if (pr["q"], pr["t"]) in seen:
            continue
        seen.add((pr["q"], pr["t"]))
        receipt["trial_pairs"].append(pr)
        if len(receipt["trial_pairs"]) == 4:
            break
    receipt["pairs"] = all_pairs
    receipt["n_pairs_independently_verified"] = len(all_pairs)
    receipt["g0"] = {
        "criterion": "2 independently verified pairs with different conductors, distinct (q,t) groups",
        "n_independently_verified_pairs_distinct_groups": len({(x["q"], x["t"]) for x in all_pairs}),
        "passed": len({(x["q"], x["t"]) for x in all_pairs}) >= 2,
    }
    receipt["negative_selection_receipt"] = not receipt["g0"]["passed"]
    receipt["edges_expanded_total"] = edge_total_expanded
    receipt["edges_written"] = len(edge_seen)
    receipt["time"] = {"cpu_seconds": round(time.process_time() - cpu0, 2),
                       "wall_seconds": round(time.perf_counter() - wall0, 2)}
    curves_out.close()
    edges_out.close()
    with open(os.path.join(args.out, "selection_receipt.json"), "w") as fh:
        json.dump(receipt, fh, indent=1)
    print(f"pairs independently verified: {len(all_pairs)} in "
          f"{receipt['g0']['n_independently_verified_pairs_distinct_groups']} groups; "
          f"trial pairs: {len(receipt['trial_pairs'])}; G0 passed: {receipt['g0']['passed']}; "
          f"cpu {receipt['time']['cpu_seconds']}s", flush=True)


if __name__ == "__main__":
    main()
