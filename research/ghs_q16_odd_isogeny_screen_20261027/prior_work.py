#!/usr/bin/env python3
"""Materialize per-class catalog and mechanism scope checks, with bounded labels."""
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent


def run():
    data = json.loads((HERE / "certificate.json").read_text())
    sources = [
        {"title": "Shimura-Momose-Chao, cubic even-characteristic weak coverings I and II",
         "url": "https://scis.jp/2011/program/",
         "scope": "conference titles establish prior cubic binary cover-family work; full class-by-class equations not checked"},
        {"title": "Murai-Shimura-Iijima-Chao, classification under the isogeny condition",
         "url": "https://www.iwsec.org/scis/2022/_abst/1A3-2.pdf",
         "scope": "classification scope is restricted to genus equal to extension degree times source genus"},
        {"title": "Tian, cover attacks for elliptic curves with prime order",
         "url": "https://arxiv.org/pdf/2012.07173",
         "scope": "published cubic rational odd-prime isogeny route; displayed 2-torsion reconstruction is not directly a characteristic-two implementation"},
    ]
    candidates = []
    for row in data["rows"]:
        label = row["catalog_label"]
        candidates.append({
            "source_order": row["order"], "source_model_count": row["source_models"],
            "catalog_label": label,
            "catalog_url": "https://www.lmfdb.org/Variety/Abelian/Fq/" + label.replace(".", "/", 2),
            "class_catalog_status": "existing genus-three Jacobian isogeny class in frozen LMFDB snapshot",
            "family_prior_work_status": "cubic binary cover classifications and rational cubic isogeny transfer predate this screen",
            "exact_source_equation_publication_status": "unresolved by this family-and-catalog check",
            "vulnerability_status": "unestablished: no explicit source-to-genus-three map or rho-beating attack receipt",
            "highlighted_exact_query": (
                f'\"{label}\" \"{row["order"]}\" GHS cover; '
                f'\"GF(4096)\" \"{row["order"]}\" GHS elliptic'
                if row["order"] in (4076, 4118) else None),
            "highlighted_exact_query_result": (
                "no directly matching paper in bounded web search; absence is not established"
                if row["order"] in (4076, 4118) else "not separately searched"),
        })
    out = {
        "schema": "ghs-q16-odd-isogeny-prior-work-v1",
        "checked_date_utc": "2026-10-27",
        "scope": "per-class frozen catalog label plus bounded family prior-work check; not a global publication absence claim",
        "published_mechanisms": sources, "candidates": candidates,
    }
    (HERE / "prior_work.json").write_text(json.dumps(out, indent=2) + "\n")
    assert len(candidates) == 44
    print("per-class prior-work ledger generated: 44 labels")


if __name__ == "__main__":
    run()
