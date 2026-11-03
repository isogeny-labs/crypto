#!/usr/bin/env python3
"""Per-hit prior-family gates for the 60 exact degree-three q16 covers."""

import hashlib
import json
from pathlib import Path

from prior_work import gf_pow, relative_trace


HERE = Path(__file__).resolve().parent
SCAN = HERE / "degree3_all_q16_scan_receipt.json"
JV = "https://eprint.iacr.org/2011/020.pdf"
MAGMA = "https://www.math.uzh.ch/sepp/magma-2.25.2-ds/HandbookVolume10.pdf"
GENERAL = "https://globals.ieice.org/en_transactions/fundamentals/10.1093/ietfec/e88-a.1.97/_p"


def main():
    raw = SCAN.read_bytes()
    scan = json.loads(raw)
    assert scan["counts"] == {"models": 10664, "exact_hits": 60}
    records = []
    for hit in scan["hits"]:
        b, u = hit["b_mask"], hit["u_mask"]
        assert relative_trace(b) == hit["trace_b_mask"]
        if b == 1:
            label = "base-field coefficient; existing descent setting"
        elif hit["trace_b_mask"] == 0:
            label = "published Joux-Vitse trace-zero cover locus"
        else:
            label = "outside Joux-Vitse trace-zero locus; exact-cover publication unverified"
        records.append({
            "class": hit["class"], "model_index": hit["model_index"],
            "u_mask": u, "b_mask": b, "relative_trace_b_mask": hit["trace_b_mask"],
            "frobenius_orbit_u": [gf_pow(u, 16**i) for i in range(3)],
            "frobenius_orbit_b": [gf_pow(b, 16**i) for i in range(3)],
            "prior_family_label": label,
            "joux_vitse_reference": JV, "general_ghs_reference": MAGMA,
            "general_extension_reference": GENERAL,
            "exact_equation_publication_status": "unverified",
            "new_attack_method_claim": False,
            "measured_speedup_claim": False,
        })
    assert len(records) == 60
    assert sum(r["relative_trace_b_mask"] != 0 for r in records) == 36
    assert sum(r["relative_trace_b_mask"] == 0 for r in records) == 24
    assert len({(r["u_mask"], r["b_mask"]) for r in records}) == 30
    receipt = {
        "schema": "ghs-q16-degree3-all60-prior-family-gates-v1",
        "scan_sha256": hashlib.sha256(raw).hexdigest(),
        "literature_scope": "General GHS and cover/decomposition methods are published; Joux-Vitse Appendix A.2 covers the trace-zero genus-three locus. Search did not establish absence of any exact equation from all publications.",
        "rows": records,
        "script_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
    }
    (HERE / "prior_work_degree3_receipt.json").write_text(json.dumps(receipt, indent=2)+"\n")
    print("60 per-hit gates; 36 nonzero-trace hits; 30 distinct (u,b) source equations")


if __name__ == "__main__":
    main()
