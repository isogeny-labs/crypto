#!/usr/bin/env python3
"""Frozen single-target panel: plain vs negation-map vs automorphism-orbit rho.

Seed policy isogeny-labs/experiment-seeds/v1. Master seed and all stream
derivations are in evidence/seed_manifest.json (written before any run).
Stream key = SHA-256("<master hex>/<domain>/<curve>/<index>[/<arm>]"), first 8
bytes little-endian. Target logs are evaluator-side: the solver binary gets
only public coordinates; logs are used only by the verifier after the solve.
Arm order is rotated per trial index (interleaved). Single thread."""
import hashlib, json, os, subprocess, sys, time
sys.path.insert(0, os.path.dirname(__file__))
import structure as S

HERE = os.path.dirname(os.path.abspath(__file__)) + "/.."
BIN = HERE + "/native/target/release/m31-structure-rho"
MASTER = "4d33315f72686f5f"           # "M31_rho_"
T = int(os.environ.get("PANEL_T", "300"))
p = S.p

def key(*parts):
    return int.from_bytes(hashlib.sha256(("/".join([MASTER, *map(str, parts)])).encode()).digest()[:8], "little")

def j0_setup():
    b, n0 = 5, 2147444533
    r = 165188041; assert n0 == 13 * r and S.is_prime(r)
    rng = S.SM("panel/j0/G")
    while True:
        G = S.mul(13, S.rand_point(0, b, rng), 0)
        if G is not None and S.mul(r, G, 0) is None: break
    beta = S.pow(3, (p - 1) // 3, p) if False else None
    g = next(g for g in range(2, 50) if pow(g, (p - 1) // 3, p) != 1)
    beta = pow(g, (p - 1) // 3, p)
    # lambda: root of l^2 + l + 1 mod r matching (beta x, y) = lambda G
    h = next(h for h in range(2, 50) if pow(h, (r - 1) // 3, r) != 1)
    l1 = pow(h, (r - 1) // 3, r)
    lam = next(l for l in (l1, l1 * l1 % r) if S.mul(l, G, 0) == (beta * G[0] % p, G[1]))
    return dict(a=0, b=b, n=r, G=G, beta=beta, lam=lam, cofactor=13)

def curves():
    c1 = dict(a=847514846, b=2087444724, n=2147446831, G=(625037715, 2100627251))
    c2 = dict(a=p - 3, b=21, n=2147487637)
    rng = S.SM("panel/M31_minus3_21/G")
    c2["G"] = S.rand_point(c2["a"], c2["b"], rng)
    return {"B677": c1, "M31_minus3_21": c2, "J0_b5_r": j0_setup()}

DESIGN = [  # (curve, arms, W, dp, target policy)
    ("B677", ["plain", "neg", "neg_aa"], 64, 5, "frozen_Q"),
    ("M31_minus3_21", ["plain", "neg", "neg_aa"], 64, 5, "seeded"),
    ("J0_b5_r", ["plain", "neg", "neg_aa", "aut6"], 64, 5, "seeded"),
    ("J0_b5_r", ["neg", "aut6"], 16, 3, "seeded"),
]

def target(cname, c, i, policy):
    if policy == "frozen_Q":
        return 987654321, (976979610, 1228979761)
    k = key("target", cname, i) % (c["n"] - 1) + 1
    return k, S.mul(k, c["G"], c["a"])

def main(out):
    cs = curves()
    manifest = {"policy": "isogeny-labs/experiment-seeds/v1", "study": "m31_curve_structure_20261126",
                "master_seed_hex": MASTER, "prng_solver": "SplitMix64 (native/src/main.rs), state = stream key",
                "prng_structure": "SplitMix64 seeded by SHA-256 of '<0x4d33315f73747275 hex>/<stream>' (tools/structure.py)",
                "derivation": "first 8 bytes LE of SHA-256(master/domain/curve/index[/arm/W/dp])",
                "streams": {"target": "target/<curve>/<i> -> k = key mod (n-1) + 1 (evaluator custody)",
                            "walk": "walk/<curve>/<i>/<arm>/<W>/<dp> (arms independent; neg_aa is a second independent neg)"},
                "trials_per_cell": T, "design": DESIGN,
                "curves": {k: {kk: (list(vv) if isinstance(vv, tuple) else vv) for kk, vv in v.items()} for k, v in cs.items()},
                "binary_sha256": hashlib.sha256(open(BIN, "rb").read()).hexdigest()}
    mtxt = json.dumps(manifest, indent=1, sort_keys=True)
    open(HERE + "/evidence/seed_manifest.json", "w").write(mtxt + "\n")
    print("manifest sha256", hashlib.sha256(mtxt.encode()).hexdigest(), file=sys.stderr)
    seen = set()
    with open(out, "w") as fo:
        for cname, arms, W, dp, pol in DESIGN:
            c = cs[cname]
            for i in range(T):
                k, Q = target(cname, c, i, pol)
                rot = i % len(arms)
                for arm in arms[rot:] + arms[:rot]:
                    ws = key("walk", cname, i, arm, W, dp)
                    assert ws not in seen; seen.add(ws)
                    real = "neg" if arm == "neg_aa" else arm
                    cmd = [BIN, real, str(c["a"]), str(c["b"]), str(c["n"]), str(c["G"][0]), str(c["G"][1]),
                           str(Q[0]), str(Q[1]), hex(ws), str(W), str(dp)]
                    if real == "aut6": cmd += [str(c["beta"]), str(c["lam"])]
                    r = subprocess.run(cmd, capture_output=True, text=True)
                    rec = {"curve": cname, "trial": i, "arm_label": arm, "W": W, "dp": dp, "exit": r.returncode}
                    if r.returncode == 0:
                        rec.update(json.loads(r.stdout))
                        # independent verifier (Python affine arithmetic, evaluator log)
                        rec["verify_dG_eq_Q"] = S.mul(rec["recovered"], c["G"], c["a"]) == Q
                        rec["verify_eq_evaluator_log"] = rec["recovered"] == k
                    else:
                        rec["stderr"] = r.stderr[-400:]
                    fo.write(json.dumps(rec) + "\n")
            print(cname, W, dp, "done", file=sys.stderr)

if __name__ == "__main__":
    main(sys.argv[1])
