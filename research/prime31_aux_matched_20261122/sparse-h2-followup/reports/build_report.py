"""Lay out retained evidence and render the sparse-H2 follow-up report.

This script performs document preparation only. It reads native receipts and
certificate metadata; it does not run arithmetic, collectors, tests, or
experiments, and does not recompute a research result.

Run after ../DEVELOPMENT_RECEIPT_v2.json exists. --allow-pending permits an
explicitly named draft while the parent session finishes qualification.
"""
from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import sys

HERE = Path(__file__).resolve().parent
STUDY = HERE.parent


def read_json(path: Path):
    with path.open(encoding="utf-8") as source:
        return json.load(source)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--allow-pending", action="store_true")
    arguments = parser.parse_args()
    receipt_path = STUDY / "DEVELOPMENT_RECEIPT_v2.json"
    if not receipt_path.exists() and not arguments.allow_pending:
        parser.error("DEVELOPMENT_RECEIPT_v2.json is required; --allow-pending writes a draft only")

    receipt = read_json(receipt_path) if receipt_path.exists() else None
    data = {
        "report_date": "2026-11-22",
        "report_timezone": "America/Los_Angeles",
        "report_date_label": "22 NOV 2026",
        "draft": receipt is None,
        "tests_available": False,
        "tests_label": "Pending",
        "tests_description": "The coordinated native qualification is pending. No pass count is inferred from source review or compilation.",
        "receipt_status": "pending",
        "receipt_sha256": "pending",
        "source_manifest_sha256": "pending",
        "certificate_available": False,
        "certificate_id": "Issuance pending",
        "certificate_description": "The native issuer and mutation-refusal contracts are implemented; an issued certificate is not yet present.",
        "certificate_path": "pending",
        "pcc_label": "Unverified producer evidence",
        "claim_id": "PCC-20261123-f873fd",
    }
    if receipt is not None:
        tests = receipt.get("tests", {})
        passed = receipt.get("tests_passed", tests.get("tests_passed"))
        failed = receipt.get("tests_failed", tests.get("tests_failed"))
        ignored = receipt.get("tests_ignored", tests.get("tests_ignored"))
        if not all(isinstance(value, int) for value in (passed, failed, ignored)):
            raise ValueError("The native receipt must explicitly report passed, failed, and ignored test counts")
        data.update({
            "tests_available": True,
            "tests_label": f"{passed} passed",
            "tests_description": f"The retained native receipt reports {passed} passed, {failed} failed, and {ignored} ignored focused contracts. These are producer development observations; they are not an admitted Prime31 recovery campaign or a signed execution verdict.",
            "receipt_status": receipt.get("status", receipt.get("experiment_admission", {}).get("status", "status unavailable")),
            "receipt_sha256": hashlib.sha256(receipt_path.read_bytes()).hexdigest(),
            "source_manifest_sha256": receipt.get("source", {}).get("source_manifest_sha256", receipt.get("build", {}).get("source_manifest_sha256", "unavailable in receipt")),
        })
        cli = receipt.get("cli", {})
        for field, expected in (("screening_inputs", 77), ("recovery_candidate_inputs", 385), ("selected_for_execution", 0), ("natural_query_campaigns_attempted", 0), ("complete_recovery_campaigns_attempted", 0)):
            if cli.get(field) != expected:
                raise ValueError(f"Receipt {field} differs from the report's registered inventory/status; revise its text and diagrams")
    certificates = sorted(path for path in STUDY.rglob("certificate.json") if HERE not in path.parents)
    if len(certificates) > 1:
        raise ValueError("Multiple certificate.json files found; bind the report to an explicit artifact before rendering")
    if certificates:
        certificate = read_json(certificates[0])
        if certificate.get("schema") != "identity.certificate/v1":
            raise ValueError("Unexpected identity-certificate schema")
        audit_path = certificates[0].with_name("identity-audit.json")
        audit = read_json(audit_path) if audit_path.exists() else None
        replay = "Replay audit is unavailable."
        if audit is not None:
            replay = (
                f"Issuer replay: {audit.get('issuer_replay', {}).get('status', 'unavailable')}; "
                f"extra-point replay: {audit.get('extra_replay', {}).get('status', 'unavailable')}."
            )
        data.update({
            "certificate_available": True,
            "certificate_id": certificate["id"],
            "certificate_path": str(certificates[0].relative_to(STUDY)),
            "certificate_description": f"Native certificate: {certificate['points']} issuer points, seed {certificate['seed']}. {replay} The fixed extra stream uses 32 points and seed 20261123. No formal-proof pointer or signed verdict is asserted.",
        })

    (HERE / "report-data.json").write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")
    try:
        import typst
    except ModuleNotFoundError:
        sys.path.insert(0, "/tmp/isogeny-report-renderer")
        import typst
    suffix = "_DRAFT" if data["draft"] else ""
    output = HERE / f"Prime31_Sparse_H2_Followup{suffix}.pdf"
    try:
        typst.compile(str(HERE / "REPORT.typ"), output=str(output), root=str(STUDY.parent))
    except typst.TypstError as error:
        print(error.diagnostic, file=sys.stderr)
        raise
    print(json.dumps({"event": "report_rendered", "output": str(output), "draft": data["draft"], "receipt_status": data["receipt_status"]}))


if __name__ == "__main__":
    main()
