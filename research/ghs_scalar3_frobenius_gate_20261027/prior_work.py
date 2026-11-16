#!/usr/bin/env python3
"""Cross-link each surviving class to the frozen prior-work ledger."""
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
OLD = HERE.parent / "ghs_q16_odd_isogeny_screen_20261027" / "prior_work.json"


def main():
    old_raw = OLD.read_bytes()
    old = json.loads(old_raw)
    cert = json.loads((HERE / "certificate.json").read_text())
    by_order = {r["source_order"]: r for r in old["candidates"]}
    rows = []
    for row in cert["residual_scalar_rows"]:
        prior = by_order[row["order"]]
        assert prior["catalog_label"] == row["catalog_label"]
        rows.append({
            "order": row["order"],
            "scalar_model_count": row["scalar_three_models"],
            "one_step_eligible_model_count": row["source_models"],
            "largest_prime": row["largest_prime"],
            "catalog_label": row["catalog_label"],
            "catalog_url": prior["catalog_url"],
            "catalog_status": prior["class_catalog_status"],
            "published_family_status": prior["family_prior_work_status"],
            "exact_equation_publication_status": prior["exact_source_equation_publication_status"],
            "rational_cubic_kernel_status": "necessary scalar-3 gate passed directly or after at most one elliptic 3-isogeny; degree-27 quotient/evaluation absent",
            "vulnerability_status": "unestablished; full target-dependent GHS cost unavailable",
        })
    assert len(rows) == 8 and sum(r["scalar_model_count"] for r in rows) == 177
    assert sum(r["one_step_eligible_model_count"] for r in rows) == 651
    output = {
        "schema": "ghs-scalar3-per-candidate-prior-work-v1",
        "checked_date_utc": "2026-10-27",
        "source_prior_work_sha256": hashlib.sha256(old_raw).hexdigest(),
        "search_scope": [
            "repository prime-characteristic cubic-transfer module classification and 3-isogeny preprocessing checked",
            "frozen LMFDB genus-3 isogeny class snapshot linked per class",
            "published characteristic-two cubic cover and odd-level transfer mechanisms reviewed",
            "bounded web search for eight exact class labels returned no indexed result; absence is not established",
        ],
        "published_mechanisms": old["published_mechanisms"],
        "prior_repository_module_screen": "research/cubic_extension_transfer_20261025/historical/extension-field-experiments/research_lead/README.md",
        "candidates": rows,
    }
    (HERE / "prior_work.json").write_text(json.dumps(output, indent=2) + "\n")
    print("prior-work rows", len(rows))


if __name__ == "__main__":
    main()
