#!/usr/bin/env sage -python
"""End-to-end relation solve and matched rho on a nonzero-trace degree-three cover."""

import hashlib
import json
import random
import sys
from pathlib import Path
from time import perf_counter

from sage.all import EllipticCurve, GF, HyperellipticCurve, Matrix, PolynomialRing, vector


HERE = Path(__file__).resolve().parent
def attack_one(row, k, K, embed, rho_trials):
    R = PolynomialRing(k, "t")
    t = R.gen()
    lookup = {embed(z): z for z in k}
    h = R([lookup[K.from_integer(m)] for m in row["base_curve_odd_model_h_coeff_masks"]])
    f = R([lookup[K.from_integer(m)] for m in row["base_curve_odd_model_f_coeff_masks"]])
    J = HyperellipticCurve(f, h).jacobian()
    def jpoint(u_masks, v_masks):
        return J(R([lookup[K.from_integer(m)] for m in u_masks]),
                 R([lookup[K.from_integer(m)] for m in v_masks]))
    D = jpoint(row["image_G_u_masks"], row["image_G_v_masks"])
    T = jpoint(row["image_Q_u_masks"], row["image_Q_v_masks"])
    b, r = row["b_mask"], row["subgroup_prime"]
    Fr = GF(r)
    E = EllipticCurve(K, [1, K.from_integer(row["u_mask"]), 0, 0, K.from_integer(b)])
    G = E(*[K.from_integer(m) for m in row["source_G_xy_masks"]])
    Q = E(*[K.from_integer(m) for m in row["source_Q_equals_7G_xy_masks"]])
    assert D != J(0) and r*D == J(0) and T == 7*D
    assert G != E(0) and r*G == E(0) and Q == 7*G

    start = perf_counter()
    Ry = PolynomialRing(k, "y")
    y = Ry.gen()
    points = []
    for xx in k:
        for yy, multiplicity in (y*y+h(xx)*y+f(xx)).roots():
            assert multiplicity == 1
            points.append((xx, yy))
    assert len(points) == 16
    fb = [J(t-xx, R(yy)) for xx, yy in points]
    fb_index = {pair: i for i, pair in enumerate(points)}
    setup_seconds = perf_counter()-start

    rng = random.Random(20261028+b)
    walk, aa, bb = J(0), 0, 0
    relations, rhs = [], []
    smooth = 0
    recovered = None
    relation_start = perf_counter()
    steps_used = None
    for step in range(1, 20001):
        choice = rng.randrange(3)
        if choice == 0:
            walk += D
            aa += 1
        elif choice == 1:
            walk += T
            bb += 1
        else:
            walk += D+T
            aa += 1
            bb += 1
        aa %= r
        bb %= r
        u, v = walk[0], walk[1]
        if u.degree() < 1:
            continue
        factors = u.factor()
        if any(poly.degree() != 1 or multiplicity != 1
               for poly, multiplicity in factors):
            continue
        coeffs = [Fr(0)]*(len(fb)+1)
        check = J(0)
        valid = True
        for poly, _ in factors:
            xx = -poly[0]/poly[1]
            yy = v(xx)
            i = fb_index.get((xx, yy))
            if i is None:
                valid = False
                break
            coeffs[i] += 1
            check += fb[i]
        if not valid:
            continue
        assert check == walk
        smooth += 1
        coeffs[-1] = -Fr(bb)
        relations.append(coeffs)
        rhs.append(Fr(aa))
        M = Matrix(Fr, relations)
        try:
            solution = M.solve_right(vector(Fr, rhs))
        except ValueError:
            continue
        if all(basis[-1] == 0 for basis in M.right_kernel().basis()):
            recovered = int(solution[-1])
            steps_used = step
            break
    relation_seconds = perf_counter()-relation_start
    if recovered is not None:
        assert recovered*G == Q and recovered*D == T

    rho_rng = random.Random(20261029+b)
    rho_steps = []
    attempts = 0
    rho_start = perf_counter()
    while len(rho_steps) < rho_trials and attempts < 5*rho_trials:
        attempts += 1
        a0, b0 = rho_rng.randrange(r), rho_rng.randrange(r)
        salt = rho_rng.randrange(1000000)
        P = a0*G+b0*Q
        seen = {}
        for iteration in range(1, 10001):
            key = None if P == E(0) else (int(P[0].to_integer()), int(P[1].to_integer()))
            if key in seen:
                olda, oldb = seen[key]
                den = (b0-oldb) % r
                if den:
                    log = ((olda-a0)*pow(den,-1,r)) % r
                    assert log*G == Q
                    rho_steps.append(iteration-1)
                break
            seen[key] = (a0,b0)
            cls = ((key[0] if key else 0)+salt) % 3
            if cls == 0:
                P += G
                a0 = (a0+1) % r
            elif cls == 1:
                P += Q
                b0 = (b0+1) % r
            else:
                P += P
                a0 = (2*a0) % r
                b0 = (2*b0) % r
    rho_seconds = perf_counter()-rho_start
    assert len(rho_steps) == rho_trials
    sorted_steps = sorted(rho_steps)
    median_steps = (sorted_steps[(rho_trials-1)//2]+sorted_steps[rho_trials//2])/2
    full_seconds = row["construction_wall_seconds"]+setup_seconds+relation_seconds
    return {
        "b_mask": b, "source_order": row["source_order"],
        "subgroup_prime": r, "status": "SOLVED" if recovered is not None else "CENSORED",
        "recovered_log": recovered, "relation_limit": 20000,
        "factor_base_size": len(fb), "relation_rows": len(relations),
        "relation_matrix_rows": [[int(z) for z in coeffs] for coeffs in relations],
        "relation_rhs": [int(z) for z in rhs],
        "relation_rank": int(Matrix(Fr,relations).rank()) if relations else 0,
        "smooth_relations_seen": smooth, "walk_steps": steps_used,
        "construction_and_map_wall_seconds": row["construction_wall_seconds"],
        "factor_base_setup_wall_seconds": setup_seconds,
        "relation_and_linear_wall_seconds": relation_seconds,
        "full_transfer_wall_seconds": full_seconds,
        "rho_successful_trials": len(rho_steps), "rho_attempts": attempts,
        "rho_total_wall_seconds": rho_seconds,
        "rho_mean_one_solve_wall_seconds": rho_seconds/rho_trials,
        "rho_mean_steps": sum(rho_steps)/rho_trials,
        "rho_median_steps": median_steps,
        "rho_steps_per_success": rho_steps,
        "full_transfer_over_mean_rho_wall_ratio": full_seconds/(rho_seconds/rho_trials),
        "verified_on_source_and_jacobian": recovered is not None,
    }


def main():
    model_index = int(sys.argv[1]) if len(sys.argv) > 1 else 5
    assert model_index in (5, 12)
    MAP = HERE / ("degree3_nonzero_trace_receipt.json" if model_index == 5
                  else f"degree3_nonzero_trace_receipt_{model_index}.json")
    data = json.loads(MAP.read_text())
    k = GF(16, "a", modulus="conway")
    K = GF(4096, "w", modulus="conway")
    embed = k.embeddings(K)[0]
    rows = []
    for cover in [data]:
        row = {
            **cover,
            "base_curve_odd_model_h_coeff_masks": cover["odd_model_h_masks"],
            "base_curve_odd_model_f_coeff_masks": cover["odd_model_f_masks"],
            "source_G_xy_masks": cover["G_xy_masks"],
            "source_Q_equals_7G_xy_masks": cover["Q_equals_7G_xy_masks"],
            "construction_wall_seconds": cover["construction_wall_seconds"],
        }
        result = attack_one(row,k,K,embed,rho_trials=1000)
        rows.append(result)
        print("b",result["b_mask"],result["status"],
              "steps",result["walk_steps"],
              "cost_ratio",round(result["full_transfer_over_mean_rho_wall_ratio"],1),
              flush=True)
    receipt = {"schema": "ghs-q16-degree3-nonzero-trace-full-vs-rho-v1",
               "map_receipt_sha256": hashlib.sha256(MAP.read_bytes()).hexdigest(),
               "rows": rows,
               "search_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    filename = "attack_degree3_receipt.json" if model_index == 5 else f"attack_degree3_receipt_{model_index}.json"
    (HERE/filename).write_text(json.dumps(receipt,indent=2)+"\n")
    assert len(rows) == 1 and all(row["status"] == "SOLVED" for row in rows)


if __name__ == "__main__":
    main()
