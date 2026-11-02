#!/usr/bin/env python3
"""P1: the fixed, seeded target set, transported to every arm through the verified isogenies.

Target 0 is the planted DLP of neighbours.json (Q = k P); targets 1..T-1 are R_tau = k_tau P0
with k_tau = SHA256(seed | "target-k" | tau) mod n.  For every arm the abscissa of the image of
R_tau is recorded (phi_i for neighbours, the u-scaling for the isomorphic control, the identity
for E0 and its A/A arms).  Each phi_i is rebuilt here from its stored kernel polynomial and
re-verified against the stored j' and image of the generator.   Construction only.

    python3 make_targets.py --nb neighbours.json --ntargets 16 --out targets.json
"""
import argparse, hashlib, json, os
import cypari2

HERE = os.path.dirname(os.path.abspath(__file__))


def h256(*parts):
    return hashlib.sha256("|".join(str(x) for x in parts).encode()).hexdigest()


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--nb", default=os.path.join(HERE, "neighbours.json"))
    ap.add_argument("--ntargets", type=int, default=16)
    ap.add_argument("--out", default=os.path.join(HERE, "targets.json"))
    args = ap.parse_args()
    nb = json.load(open(args.nb))
    pari = cypari2.Pari(); pari.allocatemem(10**9)
    p, n, seed = int(nb["p"], 16), int(nb["n"], 16), nb["seed"]
    a, b = int(nb["E0"]["a"], 16), int(nb["E0"]["b"], 16)
    E0 = pari.ellinit([a, b], p)
    P0 = pari([int(x, 16) for x in nb["planted_dlp"]["P"]])
    Q0 = pari([int(x, 16) for x in nb["planted_dlp"]["Q"]])
    k0 = int(nb["planted_dlp"]["k"], 16)
    assert pari.ellmul(E0, P0, k0) == Q0
    X = pari("x")

    targets = []
    for tau in range(args.ntargets):
        if tau == 0:
            k, R = k0, Q0
        else:
            k = int(h256(seed, "target-k", tau), 16) % (n - 2) + 1
            R = pari.ellmul(E0, P0, k)
        targets.append({"tau": tau, "k": hex(k), "R": [hex(int(R[0].lift())), hex(int(R[1].lift()))],
                        "x_by_arm": {}, "_R": R})

    arms = [{"arm": "E0", "role": "reference", "a": nb["E0"]["a"], "b": nb["E0"]["b"], "slug": nb["E0"]["slug"]},
            {"arm": "AA_repeat", "role": "aa_repeat", "a": nb["E0"]["a"], "b": nb["E0"]["b"], "slug": nb["E0"]["slug"],
             "note": "same instance as E0, separate run slot (determinism and clock noise floor)"},
            {"arm": "AA_perm", "role": "aa_variable_order", "a": nb["E0"]["a"], "b": nb["E0"]["b"], "slug": nb["E0"]["slug"],
             "note": "same instance as E0 with a seeded permutation of the variable order (re-encoding noise floor)"}]
    for t in targets:
        for arm in ("E0", "AA_repeat", "AA_perm"):
            t["x_by_arm"][arm] = t["R"][0]
    c = nb["controls"][0]
    u = int(c["u"], 16)
    arms.append({"arm": "iso", "role": "iso_control", "a": c["a"], "b": c["b"], "slug": c["slug"], "u": c["u"]})
    for t in targets:
        t["x_by_arm"]["iso"] = hex(int(t["R"][0], 16) * pow(u, 2, p) % p)

    seen = {}
    for rec in nb["neighbours"]:
        l = rec["l"]
        idx = seen.get(l, 0); seen[l] = idx + 1
        arm = f"n{l}_{idx}"
        h = pari.Pol([pari.Mod(int(x, 16), p) for x in rec["kernel_poly_high_to_low"]])
        assert h == pari.Pol([int(x, 16) for x in rec["kernel_poly_high_to_low"]]) * pari.Mod(1, p)
        I = pari.ellisogeny(E0, h)
        Ep = pari.ellinit(I[0], p)
        assert int(Ep[12].lift()) == int(rec["j"], 16)
        assert int(I[0][3].lift()) == int(rec["a"], 16) and int(I[0][4].lift()) == int(rec["b"], 16)
        phi = I[1]
        fP = pari.ellisogenyapply(phi, P0)
        assert [hex(int(c_.lift())) for c_ in fP] == rec["phi_P"]
        arms.append({"arm": arm, "role": "neighbour", "l": l, "slot": idx, "a": rec["a"], "b": rec["b"],
                     "slug": rec["slug"], "kernel_poly_sha256": rec["kernel_poly_sha256"]})
        for t in targets:
            img = pari.ellisogenyapply(phi, t["_R"])
            assert pari.ellisoncurve(Ep, img)
            t["x_by_arm"][arm] = hex(int(img[0].lift()))
        print(arm, rec["slug"], flush=True)
    for t in targets:
        del t["_R"]
    out = {"schema": "p1.targets/v1", "seed": seed, "p": nb["p"], "n": nb["n"], "arms": arms, "targets": targets,
           "rule": "target 0 = planted Q; target tau = k_tau P0, k_tau = SHA256(seed|target-k|tau) mod (n-2) + 1"}
    json.dump(out, open(args.out, "w"), indent=1)
    print("wrote", args.out, len(arms), "arms", len(targets), "targets")


if __name__ == "__main__":
    main()
