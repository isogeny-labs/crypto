#!/usr/bin/env python3
"""Build the documentary ECBench coverage ledger.

This script parses no measurements and makes no performance calculation.  It
joins the native method-registry snapshot to a frozen disposition map so that
missing methods cannot silently disappear from the campaign report.
"""

from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent
SOURCE_COMMIT = "48c8a4767d2d1eed6a7b379f74410bc729239380"
BINARY_SHA256 = "91f5895637501c6de257bbaf3afc9cbc2c0f5e964964d3dc4fc3bf78a427d96b"

LEGACY_CONTROL = {
    "ic.three_leads_genus2_v1",
    "ic.three_leads_decomposition_v1",
    "rho.frozen_reference",
    "rho.plain",
    "rho.negation",
    "rho.cubic",
    "rho.quartic",
    "rho.signed_frobenius",
    "rho.signed_frobenius_strong",
    "rho.signed_frobenius_strong_legacy",
    "bsgs.textbook",
    "bsgs.interleaved",
    "bsgs.negation",
    "kangaroo.vow",
    "transfer.quartic_cover",
    "ic.hyperelliptic",
    "claw.pair_table",
    "ic.cm_folded_native_v1",
    "ic.cm_folded_recurrence_v1",
    "ic.cm_folded_native_d43_v1",
    "ic.large_prime",
}

DIAGNOSTIC_ONLY = {
    "rho.plus_model_v1",
    "rho.best2_model_v1",
    "rho.multitarget_pooled",
    "rho.preprocessing",
    "bsgs.negation_table",
    "ic.pipeline_counted",
    "ic.tensor_contraction_v1",
    "ic.known_log_mitm2_v1",
    "ic.known_log_ksum_v1",
    "ic.shared_rank",
}

PRICING_SENSITIVITY = {
    "ic.pipeline_full",
    "rho.signed_frobenius_full",
    "rho.signed_frobenius_strong_full",
    "ic.pipeline_wordram",
    "rho.signed_frobenius_wordram",
    "rho.signed_frobenius_strong_wordram",
}

SPECIALIZED = {
    "ic.aps_v1",
    "ic.three_leads_genus2_pencil_symbolic_v2",
    "ic.three_leads_genus2_choice2_v1",
    "ic.three_leads_genus2_projective_chart_v1",
    "ic.three_leads_genus2_target_rank_v1",
    "rho.three_leads_plain_v1",
    "rho.three_leads_negation_v1",
    "rho.three_leads_jacobian_v1",
    "ic.three_leads_genus2_v2",
    "ic.three_leads_decomposition_v2",
    "rho.negation_strong",
    "ic.k4_gaudry",
    "rho.frobenius_orbit",
    "rho.gls",
    "transfer.isogeny",
    "transfer.quartic_cover_v2",
    "transfer.ghs_cover",
    "ic.hyperelliptic_v2",
    "ic.cm_folded_native_v2",
    "ic.cm_folded_recurrence_v2",
    "ic.cm_folded_native_d43_v2",
    "ic.hyperelliptic_thread",
    "ic.trace_line_2lp",
    "ic.koblitz_descent",
}

SOLVERS = [
    ("f4-f2", "word XORs for elimination; incomplete whole-engine count"),
    ("buchberger-f2", "polynomial reductions; backend-native unit"),
    ("matrix-f4", "partial word operations; matrix build excluded"),
    ("matrix-f5", "partial word operations; matrix build excluded"),
    ("inherited-f4", "partial word operations; child specialisation excluded"),
    ("crossbred-f2", "partial word operations"),
    ("xl-f2", "modelled monomial operations"),
    ("sat-cdcl", "conflicts; CNF or native-XOR encoding"),
    ("fes-f2", "Gray-code word XORs; quadratic systems"),
    ("fes-f2-wide", "AVX2/AVX-512 vector XORs; at most 32 equations"),
    ("exhaustive", "short-circuited monomial tests"),
]


def disposition(method_id: str) -> tuple[str, str]:
    if method_id in LEGACY_CONTROL:
        return "legacy_control", "retained for replay/accounting drift; not a current winner"
    if method_id in DIAGNOSTIC_ONLY:
        return "diagnostic_only", "different advice, target multiplicity, known-log, or model scope"
    if method_id in PRICING_SENSITIVITY:
        return "pricing_sensitivity", "rank only inside its full-price or word-RAM unit"
    if method_id in SPECIALIZED:
        return "specialized_domain", "requires its own curve/object and matched reference cell"
    return "current_primary", "eligible in an applicable same-object E2E cell"


def cell(method: dict) -> str:
    method_id = method["id"]
    if method_id.startswith("ic.three_leads") or method_id.startswith("rho.three_leads"):
        return "three-leads/transfer"
    if method_id.startswith("transfer.") or "hyperelliptic" in method_id or "k4_" in method_id:
        return "transfer/Jacobian"
    if "cm_folded" in method_id:
        return "CM-folded"
    applies = method["applies"]
    if applies == "prime_only":
        return "prime"
    if applies in {"binary_only", "koblitz_only"}:
        return "binary/Koblitz"
    return "prime + binary, split before ranking"


def main() -> None:
    methods = json.loads((ROOT / "method-registry.json").read_text())
    if len(methods) != 74 or len({m["id"] for m in methods}) != 74:
        raise SystemExit("expected exactly 74 unique live methods")

    rows = []
    for method in methods:
        state, note = disposition(method["id"])
        rows.append(
            {
                "id": method["id"],
                "family": method["family"],
                "applies": method["applies"],
                "entry": method["entry"],
                "disposition": state,
                "campaign_cell": cell(method),
                "fresh_run": (
                    "not_globally_rankable"
                    if state in {"diagnostic_only", "legacy_control", "pricing_sensitivity"}
                    else "blocked_no_authenticated_eks_route"
                ),
                "note": note,
            }
        )

    solver_rows = [
        {
            "id": name,
            "disposition": "nested_backend",
            "campaign_cell": "algebraic-solver matched systems",
            "fresh_run": "blocked_no_authenticated_eks_route",
            "native_unit": unit,
            "global_rankable": False,
        }
        for name, unit in SOLVERS
    ]

    out = {
        "schema": "ecbench.frontier-coverage/v1",
        "date": "2026-10-31",
        "source_commit": SOURCE_COMMIT,
        "binary_sha256": BINARY_SHA256,
        "method_count": len(rows),
        "solver_backend_count": len(solver_rows),
        "coverage_semantics": (
            "Every live method receives a disposition. Fresh missing evidence is blocked/unranked, "
            "never interpreted as zero cost or a loss."
        ),
        "methods": rows,
        "solver_backends": solver_rows,
    }
    (ROOT / "coverage.json").write_text(json.dumps(out, indent=2) + "\n")

    counts: dict[str, int] = {}
    for row in rows:
        counts[row["disposition"]] = counts.get(row["disposition"], 0) + 1
    md = [
        "# Live implementation coverage",
        "",
        f"Pinned commit: `{SOURCE_COMMIT}`. Registry: **{len(rows)} methods**; nested solver registry: **{len(solver_rows)} backends**.",
        "",
        "A disposition is not a performance verdict. `blocked` means no fresh EKS record was produced in this workspace.",
        "",
        "## Summary",
        "",
        "| disposition | methods |",
        "|:--|--:|",
    ]
    md.extend(f"| `{key}` | {counts[key]} |" for key in sorted(counts))
    md.extend(
        [
            "",
            "## ECBench methods",
            "",
            "| # | method | family | applies | disposition | campaign cell | fresh run |",
            "|--:|:--|:--|:--|:--|:--|:--|",
        ]
    )
    for index, row in enumerate(rows, 1):
        md.append(
            f"| {index} | `{row['id']}` | {row['family']} | {row['applies']} | "
            f"{row['disposition']} | {row['campaign_cell']} | {row['fresh_run']} |"
        )
    md.extend(
        [
            "",
            "## Nested algebraic solver backends",
            "",
            "These are implementations inside `ic.pipeline`, not independent ECBench method IDs. Their native counters have different meanings; only matched E2E wall or a complete common priced unit may rank them.",
            "",
            "| backend | native unit / accounting boundary | fresh run |",
            "|:--|:--|:--|",
        ]
    )
    md.extend(f"| `{row['id']}` | {row['native_unit']} | {row['fresh_run']} |" for row in solver_rows)
    md.append("")
    (ROOT / "COVERAGE.md").write_text("\n".join(md))


if __name__ == "__main__":
    main()
