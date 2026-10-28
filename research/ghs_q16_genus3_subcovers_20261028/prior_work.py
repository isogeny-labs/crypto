#!/usr/bin/env python3
"""Attach per-equation prior-class checks to the 15 exact q16 sources."""

import hashlib
import json
from pathlib import Path


HERE = Path(__file__).resolve().parent
MAP = HERE / "map15_receipt.json"
ATTACK = HERE / "attack15_receipt.json"
CLASSES = {
    3970: ("3.16.a_a_aex", [1, 0, 0, -127, 0, 0, 4096]),
    4098: ("3.16.a_a_b", [1, 0, 0, 1, 0, 0, 4096]),
    4162: ("3.16.a_a_cn", [1, 0, 0, 65, 0, 0, 4096]),
}
CLASSIFICATION = "https://www.iwsec.org/scis/2022/_abst/1A3-2.pdf"


def main():
    mapped = json.loads(MAP.read_text())["rows"]
    attacked = {row["b_mask"]: row for row in json.loads(ATTACK.read_text())["rows"]}
    rows = []
    for row in mapped:
        order, b = row["source_order"], row["b_mask"]
        label, poly = CLASSES[order]
        assert poly[0] + poly[3] + poly[6] == order
        assert attacked[b]["source_order"] == order
        rows.append({
            "b_mask": b,
            "exact_source_equation": f"Y^2+X*Y=X^3+w^5*X^2+K.from_integer({b})",
            "source_order": order,
            "base_jacobian_weil_polynomial_coefficients": poly,
            "prior_lmfdb_isogeny_class": label,
            "prior_lmfdb_url": f"https://www.lmfdb.org/Variety/Abelian/Fq/3/16/{label[5:]}",
            "prior_even_characteristic_cover_classification": CLASSIFICATION,
            "exact_equation_publication_status": "unverified",
            "new_in_local_66_source_frontier": True,
            "verified_eligible_transfer": row["verified_nonzero_base_field_subgroup_map"],
            "measured_full_cost_below_rho": attacked[b]["full_transfer_over_mean_rho_wall_ratio"] < 1,
            "claim_label": "exact equation newly found by this local search; known isogeny class; cover-family novelty unestablished",
        })
    assert len(rows) == 15 and all(not row["measured_full_cost_below_rho"] for row in rows)
    receipt = {
        "schema": "ghs-q16-genus3-all15-prior-work-gate-v1",
        "map_receipt_sha256": hashlib.sha256(MAP.read_bytes()).hexdigest(),
        "attack_receipt_sha256": hashlib.sha256(ATTACK.read_bytes()).hexdigest(),
        "rows": rows,
        "search_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
    }
    (HERE/"prior_work_receipt.json").write_text(json.dumps(receipt,indent=2)+"\n")
    print("15 prior-class checks; zero measured speedups; exact-publication status unverified")


if __name__ == "__main__":
    main()
