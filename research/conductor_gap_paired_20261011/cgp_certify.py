"""Certify the endomorphism conductor f_E of ordinary curves over F_{p^2}.

Method A (volcano): for each prime l | f_pi, count F_q-rational l-isogenies and walk to
the l-floor with Velu images; v_l(f_E) = v_l(f_pi) - height.
Method B (group structure): E(F_q) ~ End(E)/(pi - 1), so with pi = x + f_pi*omega_K the
l-part of n1 in E(F_q) ~ Z/n1 x Z/n2 is min(v_l(x-1), v_l(f_pi/f_E)); computed exactly
for the curve and its quadratic twist (x -> -x) and compared with the Method-A value.

Python prototype under a task-level exception (PROTOCOL.md section 1).
"""
from __future__ import annotations

import random
import time

from cgp_field import Fq, factorint
from cgp_curve import (
    Counter, Curve, ell_part_of_n1, frobenius_x, predicted_vn1, volcano_height, vp,
)

SUPPORTED_ELL = (2, 3, 5, 7)


def method_b(F: Fq, a: int, b: int, t: int, f_pi: int, d_k: int, fac: dict[int, int]):
    """Exact group-structure data for the curve and its twist, per prime l | f_pi."""
    E = Curve(F, a, b)
    Et = E.twist(F.gen)
    pts, ptst = E.points(), Et.points()
    x = frobenius_x(t, f_pi, d_k)
    out = {}
    for ell, depth in fac.items():
        obs = ell_part_of_n1(E, pts, ell, depth)
        obs_tw = ell_part_of_n1(Et, ptst, ell, depth)
        consistent = [v for v in range(depth + 1)
                      if predicted_vn1(x, f_pi, v, ell) == obs
                      and predicted_vn1(-x, f_pi, v, ell) == obs_tw]
        out[ell] = {"v_n1_curve": obs, "v_n1_twist": obs_tw, "consistent_v_fE": consistent,
                    "decisive": len(consistent) == 1}
    return {"x": x, "per_prime": out}


def certify_curve(F: Fq, counter: Counter, a: int, b: int, t: int, N: int, f_pi: int,
                  d_k: int, deadline: float | None = None, seed: int = 99):
    """Certificate record for one curve.  Never raises on a mathematical anomaly: it records
    status 'failed' with the reason, so the unsuccessful attempt is preserved."""
    t0 = time.perf_counter()
    rng = random.Random(seed)
    fac = factorint(f_pi)
    rec = {"f_pi": f_pi, "D_K": d_k, "status": "ok", "method_A": {}, "method_B": None,
           "fE": None, "v_fE": {}, "seconds": 0.0, "edges": []}
    unsupported = [l for l in fac if l not in SUPPORTED_ELL]
    if unsupported:
        rec["status"] = "unsupported_ell"
        rec["reason"] = f"primes {unsupported} of f_pi outside the supported set {SUPPORTED_ELL}"
        rec["seconds"] = time.perf_counter() - t0
        return rec
    vfE = {}
    for ell, depth in fac.items():
        res = volcano_height(F, a, b, ell, depth, N, counter, deadline=deadline, rng=rng)
        rec["edges"].extend(res.get("edges", []))
        if res["status"] != "ok":
            rec["status"] = "failed"
            rec["reason"] = f"l={ell}: volcano walk status {res['status']}"
            rec["seconds"] = time.perf_counter() - t0
            return rec
        h = res["height"]
        start = res["counts"][res["path_j"][0]]
        bad_images = [e for e in res["edges"] if not e["N_ok"]]
        if h > depth or bad_images or res.get("anomalies"):
            rec["status"] = "failed"
            rec["reason"] = (f"l={ell}: height {h} depth {depth} bad_images {len(bad_images)} "
                             f"anomalies {res.get('anomalies')}")
            rec["seconds"] = time.perf_counter() - t0
            return rec
        vfE[ell] = depth - h
        rec["method_A"][ell] = {
            "depth": depth, "height": h, "v_fE": depth - h,
            "kernel_count_at_curve": start["kernels"],
            "psi_factor_degrees_at_curve": start["psi_factor_degrees"],
            "floor_j": res["floor_j"], "path_j": res["path_j"],
            "vertices_expanded": res["vertices_expanded"],
        }
    fE = 1
    for ell, v in vfE.items():
        fE *= ell ** v
    rec["v_fE"] = vfE
    rec["fE"] = fE
    rec["method_B"] = method_b(F, a, b, t, f_pi, d_k, fac)
    contradicted = [l for l, d in rec["method_B"]["per_prime"].items()
                    if vfE[l] not in d["consistent_v_fE"]]
    if contradicted:
        rec["status"] = "failed"
        rec["reason"] = f"method B contradicts method A at primes {contradicted}"
        rec["fE"] = None
    rec["seconds"] = time.perf_counter() - t0
    return rec
