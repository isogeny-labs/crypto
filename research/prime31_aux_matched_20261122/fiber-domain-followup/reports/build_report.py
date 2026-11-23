"""Render the six-page fiber-domain tutorial from supplied report metadata.

Document layout only: this script does not invoke a Rust executable, run a
cryptographic calculation, enumerate a field, or infer experimental results.
The parent supplies report-data.json. --allow-pending permits a visibly marked
layout draft when that file is absent. The supplied file is never overwritten.
"""
from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import sys

from make_figures import make_figures

HERE = Path(__file__).resolve().parent


def pending_data() -> dict:
    return {
        "schema": "prime31-fiber-tutorial-data/v1",
        "report_date_label": "23 NOV 2026 / UTC",
        "qualification_status": "Pending",
        "tests_passed": None,
        "tests_failed": None,
        "tests_ignored": None,
        "root_library_status": "Pending coordinated qualification",
        "specialized_baseline_status": "Pending coordinated qualification",
        "source_commit": "Pending source binding",
        "source_manifest_sha256": "Pending source binding",
        "qualification_receipt": "Pending parent-supplied receipt",
        "pr_url": "",
        "campaigns_attempted": None,
        "end_to_end_advantage_established": False,
    }


def prepare_data(source: Path, allow_pending: bool) -> dict:
    if not source.exists():
        if not allow_pending:
            raise ValueError("report-data.json is required; --allow-pending renders a layout draft only")
        data = pending_data()
        digest = "No supplied report-data.json; layout-only pending values"
    else:
        data = json.loads(source.read_text(encoding="utf-8"))
        digest = hashlib.sha256(source.read_bytes()).hexdigest()
    required = set(pending_data())
    if not required.issubset(data):
        raise ValueError(f"Missing explicit report metadata: {sorted(required - set(data))}")
    if data["schema"] != "prime31-fiber-tutorial-data/v1":
        raise ValueError("Unexpected report-data schema")
    if data["end_to_end_advantage_established"] is not False:
        raise ValueError("This tutorial reports no established end-to-end advantage; revise the prose before changing that claim")
    count_keys = ("tests_passed", "tests_failed", "tests_ignored")
    counts = [data[key] for key in count_keys]
    available = all(type(value) is int and value >= 0 for value in counts)
    if not available and not all(value is None for value in counts):
        raise ValueError("Native test counts must all be explicit nonnegative integers or all null")
    campaigns = data["campaigns_attempted"]
    if campaigns is not None and (type(campaigns) is not int or campaigns < 0):
        raise ValueError("campaigns_attempted must be an explicit nonnegative integer or null")
    if "draft" in data and type(data["draft"]) is not bool:
        raise ValueError("The optional draft flag must be boolean")
    for key in required - set(count_keys) - {"campaigns_attempted", "end_to_end_advantage_established"}:
        if not isinstance(data[key], str):
            raise ValueError(f"{key} must be a string")
    data = dict(data)
    data.setdefault("qualification_scope", "See the supplied qualification receipt for the instrument inventory.")
    data.update({
        "tests_available": available,
        "tests_label": f"{counts[0]} passed" if available else "Pending",
        "tests_description": (
            f"The supplied metadata records {counts[0]} passed, {counts[1]} failed and {counts[2]} ignored native development contracts. These counts are development observations, not measured complete-recovery performance."
            if available else
            "Native qualification is pending. No test pass count is inferred from implementation, source review or document rendering."
        ),
        "campaigns_label": str(campaigns) if campaigns is not None else "Not supplied",
        "report_data_sha256": digest,
        "draft": data.get("draft", False) or not available or (available and counts[1] != 0),
    })
    return data


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--data", type=Path, default=HERE / "report-data.json")
    parser.add_argument("--allow-pending", action="store_true")
    arguments = parser.parse_args()
    data = prepare_data(arguments.data.resolve(), arguments.allow_pending)
    make_figures(HERE / "figures")
    (HERE / "render-data.json").write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")
    try:
        import typst
    except ModuleNotFoundError:
        sys.path.insert(0, "/tmp/isogeny-report-renderer")
        import typst
    suffix = "_DRAFT" if data["draft"] else ""
    output = HERE / f"Prime31_Fiber_Domains_Tutorial{suffix}.pdf"
    try:
        typst.compile(str(HERE / "REPORT.typ"), output=str(output), root=str(HERE))
    except typst.TypstError as error:
        print(error.diagnostic, file=sys.stderr)
        raise
    print(json.dumps({
        "event": "tutorial_rendered",
        "layout_only": True,
        "output": str(output),
        "draft": data["draft"],
        "qualification_status": data["qualification_status"],
        "report_data_sha256": data["report_data_sha256"],
        "pdf_sha256": hashlib.sha256(output.read_bytes()).hexdigest(),
    }))


if __name__ == "__main__":
    main()
