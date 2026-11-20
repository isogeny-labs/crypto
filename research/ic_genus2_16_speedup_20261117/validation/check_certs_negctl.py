#!/usr/bin/env python3
"""Negative controls for check_certs.py: perturbed certificates must fail the
validator's own tests ([t]G = T, [r]G = 0, G != 0, Mumford validity).
One record per cert file (the first certified run), five perturbations each."""
import glob
import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from hyperell import HCurve, trim  # noqa: E402

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def verdict(C, G, T, t, r):
    if not (C.is_valid(G) and C.is_valid(T)):
        return False
    return (not C.is_zero(G)) and C.is_zero(C.mul(r, G)) and C.mul(t, G) == T and 0 <= t < r


def main():
    tried = caught = base_pass = 0
    for fn in sorted(glob.glob(os.path.join(ROOT, "P4_followup/certs/*_cert.jsonl"))):
        rec = None
        with open(fn) as fh:
            for line in fh:
                d = json.loads(line)
                if d["kind"] == "run" and d["recovered_t"] is not None:
                    rec = d
                    break
        if rec is None:
            continue
        p, r, t, c = rec["p"], rec["r"], rec["recovered_t"], rec["cert"]
        C = HCurve(c["f"], p)
        G = (trim(list(c["G_u"])), trim(list(c["G_v"])))
        T = (trim(list(c["T_u"])), trim(list(c["T_v"])))
        base_pass += verdict(C, G, T, t, r)
        f2 = list(c["f"])
        f2[0] = (f2[0] + 1) % p
        perturb = {
            "t+1": (C, G, T, (t + 1) % r, r),
            "r+2": (C, G, T, t, r + 2),
            "T:=G": (C, G, G, t, r),
            "G_v[0]+1": (C, (G[0], trim([(G[1][0] + 1) % p] + G[1][1:])) if G[1] else (G[0], [1]), T, t, r),
            "f[0]+1": (HCurve(f2, p), G, T, t, r),
        }
        for name, args in perturb.items():
            tried += 1
            if not verdict(*args):
                caught += 1
            else:
                print("NOT CAUGHT", os.path.basename(fn), name)
    print(f"unperturbed records passing: {base_pass}; perturbations: {tried}; rejected: {caught}")
    print("NEGCTL", "PASS" if tried and caught == tried else "FAIL")


if __name__ == "__main__":
    main()
