#!/usr/bin/env python3
"""Turn verified decompositions into relations over orbit representatives,
verify each relation on the curve, and track the incremental rank over Z/r.

A decomposition  R = Q_1 + Q_2 + Q_3 + Q_4  with  Q_i = eps_i * pi^k_i(V_j_i)
(V_j = the lifted representative, pi = Frobenius) gives, on the odd subgroup
H of prime order r where pi acts as [lambda],

    log R = sum_i eps_i * lambda^k_i * log V_j_i   (mod r).

The relation row is the coefficient vector (c_0 .. c_4) over the 5
representative columns, c_j = sum of eps_i * lambda^k_i over the summands that
use representative j.  There is no column for the target (see PROTOCOL.md
addendum 3).  Rank saturates at the number of representatives.

Checks per relation (all must pass, fail closed):
  terms_on_curve   every Q_i equals eps_i * pi^k_i(V_j_i) recomputed on the curve
  curve_sum        the signed points add to R using the harness's own curve code
  coeffs_consistent the stored coefficients equal the recomputed lambda-powers
  lambda_log       sum_j c_j * log V_j == log R (mod r), logs by exhaustive table
"""
import argparse
import json
import sys

from paired_common import (Ctx, MANIFEST, frob_pt, hx, load_manifest, unhex)


class RelationError(ValueError):
    pass


class RankTracker:
    """Incremental Gaussian elimination over the prime field Z/r."""

    def __init__(self, r, ncols):
        self.r, self.ncols, self.rows = r, ncols, {}

    @property
    def rank(self):
        return len(self.rows)

    def add(self, row):
        r = self.r
        row = [v % r for v in row]
        for piv in sorted(self.rows):
            if row[piv]:
                f = row[piv]
                base = self.rows[piv]
                row = [(a - f * b) % r for a, b in zip(row, base)]
        for piv, v in enumerate(row):
            if v:
                inv = pow(v, -1, self.r)
                self.rows[piv] = [(a * inv) % r for a in row]
                return True
        return False


def relation_from_points(ctx, pts):
    """Signed points (x, y) -> terms and coefficient vector over representatives."""
    E = ctx.E
    terms = []
    coeffs = [0] * len(ctx.reps_x)
    for P in pts:
        if P[0] not in ctx.orbit:
            raise RelationError(f"x={hx(P[0])} is not in any representative's Frobenius orbit")
        j, k = ctx.orbit[P[0]]
        base = frob_pt(E, ctx.reps_pt[j], k)
        if P == base:
            sign = 1
        elif P == E.neg(base):
            sign = -1
        else:
            raise RelationError(f"point at x={hx(P[0])} is not +-pi^{k}(V_{j})")
        terms.append({"x": hx(P[0]), "rep": j, "phase": k, "sign": sign})
        coeffs[j] = (coeffs[j] + sign * ctx.lam_pow[k]) % ctx.r
    return {"terms": terms, "coeffs": coeffs}


def check_relation(ctx, rel, R, pts=None):
    """Independent verification.  Returns {check_name: bool}."""
    E, r = ctx.E, ctx.r
    out = {}
    recomputed = []
    ok = True
    for t in rel["terms"]:
        base = frob_pt(E, ctx.reps_pt[t["rep"]], t["phase"])
        Q = base if t["sign"] == 1 else E.neg(base)
        recomputed.append(Q)
        ok = ok and E.on(Q) and t["sign"] in (1, -1) and 0 <= t["phase"] < ctx.n
    if pts is not None:
        ok = ok and [tuple(p) for p in pts] == recomputed
    out["terms_on_curve"] = bool(ok)
    S = None
    for Q in recomputed:
        S = E.add(S, Q)
    out["curve_sum"] = S == tuple(R)
    co = [0] * len(ctx.reps_x)
    for t in rel["terms"]:
        co[t["rep"]] = (co[t["rep"]] + t["sign"] * ctx.lam_pow[t["phase"]]) % r
    out["coeffs_consistent"] = co == [c % r for c in rel["coeffs"]]
    lhs = sum(c * L for c, L in zip(rel["coeffs"], ctx.rep_logs)) % r
    out["lambda_log"] = lhs == ctx.log[tuple(R)]
    out["all"] = all(out.values())
    return out


def load_attempts(path):
    hdr, atts = None, []
    for ln in open(path):
        if not ln.strip():
            continue
        r = json.loads(ln)
        if r["type"] == "header":
            hdr = r
        elif r["type"] == "attempt":
            atts.append(r)
    return hdr, atts


def last_per_key(atts):
    """One record per (target, arm, form): the last non-skipped, else the last."""
    best = {}
    for a in atts:
        key = (a["target_id"], a["arm"], a["form"])
        if key not in best or best[key]["status"] == "skipped" or a["status"] != "skipped":
            best[key] = a
    return sorted(best.values(), key=lambda a: a["seq"])


def build(ctx, atts):
    """Relations and rank trajectories.  Returns (relations, summary)."""
    atts = last_per_key(atts)
    scopes = {}  # (arm, form or "pooled") -> tracker/cumulative state
    relations = []

    def scope(arm, form):
        key = (arm, form)
        if key not in scopes:
            scopes[key] = {"tracker": RankTracker(ctx.r, len(ctx.reps_x)),
                           "wall": 0.0, "cpu": 0.0, "attempts": 0, "verified": 0,
                           "unsat": 0, "timeout": 0, "skipped": 0, "rejected": 0,
                           "dependent_verified": 0, "relations_failed_check": 0,
                           "first_hit": {}}
        return scopes[key]

    for a in atts:
        pts = None
        rel = checks = None
        if a["status"] == "verified":
            pts = [(unhex(p[0]), unhex(p[1])) for p in a["found_points"]]
            R = (unhex(a["target"][0]), unhex(a["target"][1]))
            rel = relation_from_points(ctx, pts)
            checks = check_relation(ctx, rel, R, pts)
        for form in (a["form"], "pooled"):
            sc = scope(a["arm"], form)
            if a["status"] == "skipped":
                sc["skipped"] += 1
                continue
            sc["attempts"] += 1
            sc["wall"] += a["total_wall_s"]
            sc["cpu"] += a["total_cpu_s"]
            sc["rejected"] += a.get("rejected_tuples", 0)
            if a["status"] in ("unsat", "timeout"):
                sc[a["status"]] += 1
            if a["status"] == "verified":
                sc["verified"] += 1
                if not checks["all"]:
                    sc["relations_failed_check"] += 1
                    new = False
                else:
                    new = sc["tracker"].add(rel["coeffs"])
                if new:
                    rk = sc["tracker"].rank
                    sc["first_hit"][rk] = {"wall_s": round(sc["wall"], 4),
                                           "cpu_s": round(sc["cpu"], 4),
                                           "attempts": sc["attempts"], "seq": a["seq"]}
                else:
                    sc["dependent_verified"] += 1
        if a["status"] == "verified":
            relations.append({
                "seq": a["seq"], "arm": a["arm"], "form": a["form"],
                "target_id": a["target_id"], "target": a["target"],
                "terms": rel["terms"], "coeffs": rel["coeffs"], "checks": checks,
                "rank_after_form_scope": scopes[(a["arm"], a["form"])]["tracker"].rank,
                "rank_after_pooled_scope": scopes[(a["arm"], "pooled")]["tracker"].rank,
            })
    summary = {}
    for (arm, form), sc in sorted(scopes.items()):
        rank = sc["tracker"].rank
        d = {k: sc[k] for k in ("attempts", "verified", "unsat", "timeout", "skipped",
                               "rejected", "dependent_verified", "relations_failed_check")}
        d.update({
            "rank": rank, "columns": len(ctx.reps_x),
            "wall_s": round(sc["wall"], 3), "cpu_s": round(sc["cpu"], 3),
            "wall_s_per_independent": round(sc["wall"] / rank, 3) if rank else None,
            "cpu_s_per_independent": round(sc["cpu"] / rank, 3) if rank else None,
            "verified_per_attempt": round(sc["verified"] / sc["attempts"], 4) if sc["attempts"] else None,
            "wall_s_per_attempt": round(sc["wall"] / sc["attempts"], 3) if sc["attempts"] else None,
            "first_hit_rank": {str(k): v for k, v in sorted(sc["first_hit"].items())},
        })
        summary[f"{arm}|{form}"] = d
    return relations, summary


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--receipts", required=True)
    ap.add_argument("--relations-out", required=True)
    ap.add_argument("--summary-out", required=True)
    ap.add_argument("--manifest", default=MANIFEST)
    a = ap.parse_args(argv)
    ctx = Ctx(load_manifest(a.manifest))
    hdr, atts = load_attempts(a.receipts)
    relations, summary = build(ctx, atts)
    with open(a.relations_out, "w") as fh:
        for rel in relations:
            fh.write(json.dumps(rel, sort_keys=True) + "\n")
    out = {"lambda": ctx.lam, "r": ctx.r, "rep_logs": ctx.rep_logs,
           "generator_log_base": [hx(c) for c in ctx.g],
           "relations": len(relations),
           "relations_failing_checks": sum(1 for r in relations if not r["checks"]["all"]),
           "scopes": summary}
    with open(a.summary_out, "w") as fh:
        json.dump(out, fh, indent=1, sort_keys=True)
        fh.write("\n")
    print(json.dumps({k: v for k, v in out.items() if k != "scopes"}))
    for key, d in summary.items():
        print(key, {k: d[k] for k in ("attempts", "verified", "unsat", "timeout", "rank",
                                      "wall_s", "wall_s_per_independent")})
    return 1 if out["relations_failing_checks"] else 0


if __name__ == "__main__":
    sys.exit(main())
