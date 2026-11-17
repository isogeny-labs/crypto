#!/usr/bin/env sage -python
"""Complete factor-base DLP transfer and same-target rho for b=1384."""

import hashlib
import json
import random
from pathlib import Path
from time import perf_counter

from sage.all import EllipticCurve, GF, HyperellipticCurve, Matrix, PolynomialRing, vector


HERE = Path(__file__).resolve().parent
MAP = HERE / "map4162_receipt.json"


def main():
    map_receipt = json.loads(MAP.read_text())
    k = GF(16, "a", modulus="conway")
    K = GF(4096, "w", modulus="conway")
    embed = k.embeddings(K)[0]
    lookup = {embed(z): z for z in k}
    a, w = k.gen(), K.gen()
    R = PolynomialRing(k, "t")
    t = R.gen()
    h = a*a+a*t**3
    f = (a**3+a+1)+(a*a+a+1)*t**4+(a*a+a)*t**6+(a**3+a)*t**7
    C = HyperellipticCurve(f, h)
    J = C.jacobian()
    r = 2081
    Fr = GF(r)
    def jpoint(u_masks, v_masks):
        return J(R([lookup[K.from_integer(m)] for m in u_masks]),
                 R([lookup[K.from_integer(m)] for m in v_masks]))

    D = jpoint(map_receipt["image_G_u_masks"], map_receipt["image_G_v_masks"])
    T = jpoint(map_receipt["image_Q_u_masks"], map_receipt["image_Q_v_masks"])
    E = EllipticCurve(K, [1, w**5, 0, 0, K.from_integer(1384)])
    G = E(*[K.from_integer(m) for m in map_receipt["source_G_xy_masks"]])
    Q = E(*[K.from_integer(m) for m in map_receipt["source_Q_equals_7G_xy_masks"]])
    assert D != J(0) and r*D == J(0) and r*T == J(0)
    assert T == 7*D and Q == 7*G and r*G == E(0)
    assert C.count_points(3) == [17, 257, 4292]

    all_start = perf_counter()
    setup_start = perf_counter()
    xs, ys, fb = [], [], []
    Ry = PolynomialRing(k, "y")
    y = Ry.gen()
    for xx in k:
        for yy, multiplicity in (y*y+h(xx)*y+f(xx)).roots():
            assert multiplicity == 1
            xs.append(xx)
            ys.append(yy)
            fb.append(J(t-xx, R(yy)))
    assert len(fb) == 16
    index = {(xx, yy): i for i, (xx, yy) in enumerate(zip(xs, ys))}
    setup_seconds = perf_counter()-setup_start

    rng = random.Random(20261028)
    walk, aa, bb = J(0), 0, 0
    rows, rhs = [], []
    smooth = 0
    seen = set()
    recovered = None
    relation_start = perf_counter()
    steps_used = 0
    for step in range(1, 20001):
        z = rng.randrange(3)
        if z == 0:
            walk += D
            aa += 1
        elif z == 1:
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
            i = index.get((xx, yy))
            if i is None:
                valid = False
                break
            coeffs[i] += 1
            check += fb[i]
        if not valid:
            continue
        assert check == walk
        smooth += 1
        key = tuple(int(z.to_integer()) for z in u.list()+v.list())
        if key in seen:
            continue
        seen.add(key)
        coeffs[-1] = -Fr(bb)
        rows.append(coeffs)
        rhs.append(Fr(aa))
        if len(rows) < len(fb):
            continue
        M = Matrix(Fr, rows)
        rhs_vector = vector(Fr, rhs)
        try:
            solution = M.solve_right(rhs_vector)
        except ValueError:
            continue
        if all(basis[-1] == 0 for basis in M.right_kernel().basis()):
            recovered = int(solution[-1])
            steps_used = step
            break
    relation_seconds = perf_counter()-relation_start
    assert recovered is not None and recovered*G == Q and recovered*D == T
    full_seconds = perf_counter()-all_start+map_receipt["transfer_two_points_wall_seconds"]

    rho_rng = random.Random(20261029)
    rho_steps = []
    rho_attempts = 0
    rho_start = perf_counter()
    while len(rho_steps) < 1000 and rho_attempts < 5000:
        rho_attempts += 1
        aa0 = rho_rng.randrange(r)
        bb0 = rho_rng.randrange(r)
        salt = rho_rng.randrange(1000000)
        P = aa0*G+bb0*Q
        seen_rho = {}
        for iteration in range(1, 10001):
            key = None if P == E(0) else (int(P[0].to_integer()), int(P[1].to_integer()))
            if key in seen_rho:
                olda, oldb = seen_rho[key]
                den = (bb0-oldb) % r
                if den:
                    log = ((olda-aa0)*pow(den, -1, r)) % r
                    assert log*G == Q
                    rho_steps.append(iteration-1)
                break
            seen_rho[key] = (aa0, bb0)
            cls = ((key[0] if key else 0)+salt) % 3
            if cls == 0:
                P += G
                aa0 = (aa0+1) % r
            elif cls == 1:
                P += Q
                bb0 = (bb0+1) % r
            else:
                P += P
                aa0 = (2*aa0) % r
                bb0 = (2*bb0) % r
    rho_seconds = perf_counter()-rho_start
    assert len(rho_steps) == 1000
    sorted_steps = sorted(rho_steps)
    rho_median_steps = (sorted_steps[499]+sorted_steps[500])/2
    rho_mean_seconds = rho_seconds/len(rho_steps)
    receipt = {
        "schema": "ghs-q16-genus3-full-transfer-vs-rho-4162-v1",
        "map_receipt_sha256": hashlib.sha256(MAP.read_bytes()).hexdigest(),
        "source_b_mask": 1384,
        "source_order": 4162,
        "subgroup_prime": r,
        "source_target_log_recovered": recovered,
        "factor_base_size": len(fb),
        "relation_rows": len(rows),
        "relation_matrix_rows": [[int(z) for z in coeffs] for coeffs in rows],
        "relation_rhs": [int(z) for z in rhs],
        "relation_rank": int(Matrix(Fr, rows).rank()),
        "smooth_relations_seen": smooth,
        "walk_steps": steps_used,
        "transfer_two_points_wall_seconds": map_receipt["transfer_two_points_wall_seconds"],
        "factor_base_setup_wall_seconds": setup_seconds,
        "relation_and_linear_wall_seconds": relation_seconds,
        "full_transfer_wall_seconds": full_seconds,
        "rho_successful_trials": len(rho_steps),
        "rho_attempts": rho_attempts,
        "rho_total_wall_seconds": rho_seconds,
        "rho_mean_one_solve_wall_seconds": rho_mean_seconds,
        "rho_mean_steps": sum(rho_steps)/len(rho_steps),
        "rho_median_steps": rho_median_steps,
        "rho_steps_per_success": rho_steps,
        "full_transfer_over_mean_rho_wall_ratio": full_seconds/rho_mean_seconds,
        "verified_on_source_and_jacobian": True,
        "search_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
    }
    (HERE/"attack4162_receipt.json").write_text(json.dumps(receipt, indent=2)+"\n")
    print(json.dumps(receipt, indent=2))


if __name__ == "__main__":
    main()
