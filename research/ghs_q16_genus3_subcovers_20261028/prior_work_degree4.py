#!/usr/bin/env python3
"""Attach the published trace-zero coefficient gate to every exact degree-four hit."""

import hashlib
import json
from pathlib import Path

from prior_work import relative_trace


HERE = Path(__file__).resolve().parent
SCAN = HERE / "degree4_all_q16_scan_receipt.json"
REFERENCE = "https://eprint.iacr.org/2011/020.pdf"


def main():
    raw = SCAN.read_bytes()
    hits = json.loads(raw)["hits"]
    assert len(hits) == 1020
    rows = []
    for hit in hits:
        assert relative_trace(hit["b_mask"]) == hit["trace_b_mask"] == 0
        rows.append({"class": hit["class"], "model_index": hit["model_index"],
                     "u_mask": hit["u_mask"], "b_mask": hit["b_mask"],
                     "relative_trace_b_mask": 0,
                     "published_coefficient_locus": "Joux-Vitse characteristic-two trace-zero genus-three cover criterion",
                     "same_cover_formula_as_publication": "unverified",
                     "published_reference": REFERENCE,
                     "exact_equation_publication_status": "unverified",
                     "measured_attack_cost_status": "unmeasured"})
    receipt = {"schema": "ghs-q16-degree4-all1020-prior-family-gates-v1",
               "scan_sha256": hashlib.sha256(raw).hexdigest(),
               "rows": rows,
               "script_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    (HERE / "prior_work_degree4_receipt.json").write_text(json.dumps(receipt, indent=2)+"\n")
    print("1020 exact descriptions; all b match the published trace-zero coefficient criterion")


if __name__ == "__main__":
    main()
