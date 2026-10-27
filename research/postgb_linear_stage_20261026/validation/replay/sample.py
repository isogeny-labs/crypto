#!/usr/bin/env python3
"""Nonce-driven sample of corpus systems for the replay certificate.

Independent of scripts/summary.py.  Reads the R2-L1..R2-L6 rows from the
committed snapshot copy given as argv[1] (a directory holding runs/), and the
nonce from argv[2] (64 hex chars, drawn from /dev/urandom after the snapshot
commit).  Within each family every system is keyed by
sha256(nonce ":" card ":" family ":" label); the K[family] smallest keys are
the sample.  Prints the sample and the replay invocations (one per
(family, p[, shape]) group, always with the card's --per 8, so the generator
calls are exactly the card's) as JSON.
"""
import hashlib
import json
import sys
from pathlib import Path

K = {"jv-nagao": 3, "gaudry-s4": 3, "gaudry-d4": 3, "gaudry-s5": 1, "dense": 4}
CARDS = ["R2-L1", "R2-L2", "R2-L3", "R2-L4", "R2-L5", "R2-L6"]


def main() -> None:
    root, nonce = Path(sys.argv[1]), sys.argv[2].strip()
    assert len(nonce) == 64 and int(nonce, 16) >= 0
    pool = []
    for card in CARDS:
        doc = json.loads((root / "runs" / f"{card}.json").read_text())
        for row in doc["rows"]:
            key = hashlib.sha256(f"{nonce}:{card}:{row['family']}:{row['label']}".encode()).hexdigest()
            pool.append({"card": card, "family": row["family"], "label": row["label"],
                         "p": row["p"], "n": row["n"], "delta": row["delta"], "key": key})
    sample = []
    for fam, k in K.items():
        members = sorted((s for s in pool if s["family"] == fam), key=lambda s: s["key"])
        sample += members[:k]
    groups = {}
    for s in sample:
        argv = ["--families", s["family"], "--p", str(s["p"]), "--per", "8", "--reps", "1"]
        if s["family"] == "dense":
            # label n{n}-d{d}-p{bits}-{k}
            n, d = s["label"].split("-")[0][1:], s["label"].split("-")[1][1:]
            argv[2:2] = []
            argv += ["--shapes", f"n{n}d{d}"]
        gid = "_".join(argv[1:4:2] + ([argv[-1]] if s["family"] == "dense" else []))
        groups.setdefault(gid, {"argv": argv, "sampled": []})["sampled"].append(
            {"card": s["card"], "label": s["label"]})
    print(json.dumps({"nonce": nonce, "pool": len(pool), "k_per_family": K,
                      "sample": [{k: v for k, v in s.items()} for s in sample],
                      "groups": groups}, indent=1))


if __name__ == "__main__":
    main()
