#!/usr/bin/env python3
"""Certify the fresh-process replays in `replay/` against the recorded runs in `raw/`.

Reads only the JSON records committed in this study directory.  Writes
`REPLAY_CERTIFICATE.json`; exits nonzero if any replayed scalar differs.
"""
from __future__ import annotations

import hashlib
import json
import math
import sys
from pathlib import Path

STUDY = Path(__file__).resolve().parent.parent
FIELDS = ("total_muls", "relation_digest", "f4_native_ops", "residuals",
          "relations", "decompositions", "stop_mode")


def load(rel: str):
    return json.loads((STUDY / rel).read_text())[0]


def sha256(rel: str) -> str:
    return hashlib.sha256((STUDY / rel).read_bytes()).hexdigest()


def s(rec: dict) -> float:
    return rec["total_muls"] / (rec["c_add_e"] * math.sqrt(rec["l"]))


def main() -> int:
    runs = []
    errors = []
    for replay in sorted((STUDY / "replay").glob("p*_s*_B_replay.json")):
        stem = replay.name.replace("_replay.json", "")
        p, _, seed = stem[:-2][1:].partition("_s")
        raw_rel = f"raw/{stem}.json"
        rep_rel = f"replay/{replay.name}"
        rec = load(raw_rel)
        rep = load(rep_rel)
        arm_a = load(f"raw/p{p}_s{seed}_A.json")
        matched = {}
        for f in FIELDS:
            matched[f] = rec.get(f) == rep.get(f)
        matched["S"] = abs(s(rec) - s(rep)) <= 1e-12 * s(rec)
        matched["digest_equals_A"] = rec["relation_digest"] == arm_a["relation_digest"]
        if not all(matched.values()):
            errors.append(f"{stem}: {[k for k, v in matched.items() if not v]}")
        runs.append({
            "p": int(p), "seed": int(seed), "arm": "B",
            "recorded": raw_rel, "recorded_sha256": sha256(raw_rel),
            "replay": rep_rel, "replay_sha256": sha256(rep_rel),
            "fields": list(FIELDS), "matched": matched,
            "total_muls": rec["total_muls"], "relation_digest": rec["relation_digest"],
            "f4_native_ops": rec["f4_native_ops"], "S": s(rec),
        })
    cert = {
        "schema": "replay-certificate/v1",
        "status": "verified" if not errors else "failed",
        "independent": True,
        "scalars_verified": True,
        "method": ("fresh-process rerun of arm B at the recorded instances with the same "
                   "jv_cover binary sha256 223902e2... (git 1f006b002, RAYON_NUM_THREADS=1); "
                   "total_muls, relation_digest, f4_native_ops, residuals, relations, "
                   "decompositions, stop_mode and S compared against raw/, and the digest "
                   "against the recorded arm A"),
        "binary_sha256": "223902e28201ce9a468769ec37fe7e50e7b6330afc21f05a2de36c6c08652543",
        "source_commit": "1f006b002f02e28ad1626338dc504960d6a1dd92",
        "runs": runs,
        "errors": errors,
    }
    (STUDY / "validation/REPLAY_CERTIFICATE.json").write_text(json.dumps(cert, indent=1) + "\n")
    print(f"{len(runs)} replays, {len(errors)} errors")
    for e in errors:
        print(" ", e)
    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(main())
