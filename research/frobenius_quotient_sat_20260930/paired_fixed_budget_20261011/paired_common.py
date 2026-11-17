"""Shared helpers for the paired fixed-budget phase comparison.

Imports the parent harness (``../encode.py``, ``../run.py``, ``../brute_check.py``)
rather than copying it.  Field and curve arithmetic is the harness's own
(``frobenius_quotient_followup_20260926/verify_claims.py`` via ``encode``).
"""
import hashlib
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
PARENT = os.path.dirname(HERE)
if PARENT not in sys.path:
    sys.path.insert(0, PARENT)

from encode import Curve  # noqa: E402  (parent harness)

N = 13
S = 4
FACTOR_SEED = 20260930      # same factor space as ../README.md tables
TARGET_SEED = 20261011      # fresh targets for this comparison
PER_SET = 4
SET_NAMES = {
    "a": "planted_random_phase",
    "b": "planted_phase0",
    "c": "uniform_random",
}
MANIFEST = os.path.join(HERE, "manifest.json")
SEALED = os.path.join(HERE, "witness_sealed.json")
MANIFEST_SHA = os.path.join(HERE, "manifest.sha256")
ARM_PHASES = {"A": False, "B": True}


def sha256_file(path):
    h = hashlib.sha256()
    with open(path, "rb") as fh:
        for chunk in iter(lambda: fh.read(1 << 16), b""):
            h.update(chunk)
    return h.hexdigest()


def sha256_bytes(b):
    return hashlib.sha256(b).hexdigest()


def dumps(obj):
    """Canonical JSON used for the manifest: sorted keys, fixed indent."""
    return (json.dumps(obj, indent=1, sort_keys=True) + "\n").encode()


def hx(v):
    return hex(v)


def unhex(s):
    return int(s, 16)


def subgroup_generator(E):
    """Deterministic non-identity point of the odd subgroup H = 4E.

    The order r of H is prime for n = 13, so every non-identity element
    generates it."""
    x = 2
    while True:
        P = E.lift(x)
        if P is not None:
            G = E.mul(4, P)
            if G is not None:
                return G
        x += 1


def frob_pt(E, P, k):
    for _ in range(k):
        P = E.frob(P)
    return P


class Ctx:
    """Everything rank/verify need, rebuilt from the frozen manifest alone."""

    def __init__(self, manifest):
        self.manifest = manifest
        self.n = manifest["curve"]["n"]
        self.E = Curve(self.n)
        self.r = manifest["curve"]["odd_subgroup_order"]
        assert self.E.h == self.r
        self.reps_x = [unhex(r["x"]) for r in manifest["factor_space"]["representatives"]]
        self.reps_pt = [E_pt(self.E, r) for r in manifest["factor_space"]["representatives"]]
        # x -> (representative index, Frobenius phase k) with x = v_j^(2^k)
        self.orbit = {}
        for j, v in enumerate(self.reps_x):
            x = v
            for k in range(self.n):
                assert x not in self.orbit, "representatives share a Frobenius orbit"
                self.orbit[x] = (j, k)
                x = self.E.f.sq(x)
        self.g = tuple(unhex(c) for c in manifest["curve"]["generator"])
        assert self.E.on(self.g) and self.E.mul(self.r, self.g) is None
        # brute-force discrete logs in H (r = 2003 points)
        self.log = {None: 0}
        P = None
        for k in range(self.r):
            self.log[P] = k
            P = self.E.add(P, self.g)
        assert P is None and len(self.log) == self.r
        self.lam = self.log[self.E.frob(self.g)]
        t = manifest["curve"]["trace_t"]
        assert (self.lam * self.lam - t * self.lam + 2) % self.r == 0
        self.lam_pow = [pow(self.lam, k, self.r) for k in range(self.n)]
        self.rep_logs = [self.log[P] for P in self.reps_pt]


def E_pt(E, rep):
    P = (unhex(rep["point"][0]), unhex(rep["point"][1]))
    assert E.on(P)
    return P


def load_manifest(path=MANIFEST):
    with open(path) as fh:
        return json.load(fh)


def target_list(manifest):
    """Targets in the fixed manifest order: set a, then b, then c."""
    out = []
    for key in ("a", "b", "c"):
        for t in manifest["target_sets"][key]["targets"]:
            out.append({"id": t["id"], "set": key,
                        "R": (unhex(t["R"][0]), unhex(t["R"][1]))})
    return out


def abba_schedule(targets, arms=("A", "B")):
    """Pair i runs (A, B) when i is even and (B, A) when odd."""
    seq = []
    for i, t in enumerate(targets):
        order = arms if i % 2 == 0 else tuple(reversed(arms))
        for pos, arm in enumerate(order):
            seq.append({"pair": i, "position": pos, "arm": arm, "target": t})
    return seq
