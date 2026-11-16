#!/usr/bin/env python3
"""Recompute the fixed-base N83 first-fall coverage index from saved records."""
from __future__ import annotations

import argparse
import hashlib
import json
from collections import Counter
from pathlib import Path


ROOT = Path(__file__).resolve().parents[5]
STUDY = ROOT / "research/n83_candidate_pdp_20261028"
EVIDENCE = STUDY / "evidence/n83-factorbase-variants"
OUT = Path(__file__).resolve().parent / "coverage.json"
PROTOCOL = STUDY / "protocol-pdp-factorbase-v2.json"
EXPECTED_PROTOCOL_ID = "N83-FACTOR-BASE-PDP-FFD-LFD-20261031-V2"
EXPECTED_PROTOCOL_SHA256 = "50cd6f8213789de2dfd54a3b33a067ed87cc0def761b7c98769be3e19aee47a9"
LEGACY_SUMMARIES = {
    "spike27": "r27-attempt-07/first-fall-20261031T1725Z/first-fall-summary.json",
    "spike28": "r28-attempt-01/first-fall-20261031T1715Z/first-fall-summary.json",
    "spike30": "r30-attempt-01/first-fall-20261031T1700Z/first-fall-summary.json",
    "spike31": "r31-attempt-01/first-fall-20261031T1610Z/first-fall-summary.json",
    "spike32": "r32-attempt-01/first-fall-20261031T1630Z/first-fall-summary.json",
    "spike33": "r33-attempt-01/first-fall-20261031T1640Z/first-fall-summary.json",
}
COMBINED = EVIDENCE / "candidate-fall-20261102/combined-144/combined-summary.json"
CUSTODY = EVIDENCE / "candidate-fall-20261102/combined-144/S3-CUSTODY-complete.json"
LFD_SUMMARY = STUDY / "evidence/degree4-propagation/summary.json"
R29_INPUT_DIR = EVIDENCE / "r29-attempt-02/pdp-inputs/attempt-02/candidate-structure"
R29_TOP_DIR = EVIDENCE / "r29-attempt-02/top-form/attempt-01"
R29_BINDING_DIR = EVIDENCE / "r29-attempt-02/source-binding-audit-20261103"
R29_FIRST_FALL_DIR = EVIDENCE / "r29-attempt-02/first-fall-attempt-03/output-01"
R29_FIRST_FALL_VALIDATION = R29_FIRST_FALL_DIR / "post-run-analysis/independent-validation.json"
R29_ARCHIVE_READBACK = R29_FIRST_FALL_DIR / "post-run-analysis/archive-readback.json"
R38_TOP_DIR = EVIDENCE / "r38-attempt-01/top-form-20261101T2015Z"
R39_TOP_DIR = EVIDENCE / "r39-attempt-01/top-form-20261101T2045Z"
EXPECTED_CANDIDATES = [f"spike{i}" for i in range(27, 40)]


def digest(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            h.update(block)
    return h.hexdigest()


def load(path: Path) -> dict:
    return json.loads(path.read_text())


def source_record(path: Path, rows: int) -> dict:
    return {"path": path.relative_to(ROOT).as_posix(), "bytes": path.stat().st_size,
            "sha256": digest(path), "rows": rows}


def build() -> dict:
    protocol = load(PROTOCOL)
    assert protocol["protocol_id"] == EXPECTED_PROTOCOL_ID
    assert protocol["freeze"]["protocol_sha256"] == EXPECTED_PROTOCOL_SHA256
    planned = protocol["sample_size"]["instances"]
    assert planned == 624

    sources = []
    rows_by_candidate: dict[str, list[dict]] = {}
    for candidate, relative in LEGACY_SUMMARIES.items():
        path = EVIDENCE / relative
        summary = load(path)
        assert summary["candidate_id"] == candidate
        assert summary["protocol_id"] == EXPECTED_PROTOCOL_ID
        assert summary["protocol_sha256"] == EXPECTED_PROTOCOL_SHA256
        assert summary["planned_cases"] == summary["attempted_cases"] == 48
        assert summary["completed_certificates"] == summary["replay_verified"] == 48
        assert len(summary["rows"]) == 48
        rows_by_candidate[candidate] = [dict(row, candidate_id=candidate) for row in summary["rows"]]
        sources.append(source_record(path, len(summary["rows"])))

    combined = load(COMBINED)
    assert combined["protocol_id"] == EXPECTED_PROTOCOL_ID
    assert combined["protocol_freeze_sha256"] == EXPECTED_PROTOCOL_SHA256
    assert combined["systems_planned"] == combined["systems_attempted"] == 144
    assert combined["certificates_produced"] == combined["fresh_replays_verified"] == 144
    assert len(combined["rows"]) == 144
    sources.append(source_record(COMBINED, len(combined["rows"])))
    for row in combined["rows"]:
        rows_by_candidate.setdefault(row["candidate_id"], []).append(row)

    r29_first_fall = load(R29_FIRST_FALL_DIR / "summary.json")
    r29_validation = load(R29_FIRST_FALL_VALIDATION)
    r29_archive_readback = load(R29_ARCHIVE_READBACK)
    r29_preflight = load(R29_FIRST_FALL_DIR / "preflight.json")
    r29_custody = load(R29_FIRST_FALL_DIR / "custody.json")
    assert r29_preflight["protocol"]["frozen_sha256"] == EXPECTED_PROTOCOL_SHA256
    assert r29_first_fall["systems_planned"] == r29_first_fall["systems_attempted"] == 48
    assert r29_first_fall["certificates"] == r29_first_fall["fresh_replays_verified"] == 48
    assert r29_first_fall["s3_readbacks_verified"] == 48
    assert r29_validation["status"] == "pass"
    assert r29_validation["systems_checked"] == r29_validation["certificates_checked"] == 48
    assert r29_validation["fresh_replays_verified"] == r29_validation["s3_certificate_readbacks_verified"] == 48
    assert r29_archive_readback["status"] == "verified"
    assert r29_archive_readback["sha256"] == r29_custody["archive"]["sha256"]
    assert r29_archive_readback["bytes"] == r29_custody["archive"]["bytes"]
    rows_by_candidate["spike29"] = [dict(row, candidate_id="spike29") for row in r29_first_fall["rows"]]
    sources.extend([
        source_record(R29_FIRST_FALL_DIR / "preflight.json", 48),
        source_record(R29_FIRST_FALL_DIR / "source-index.json", 48),
        source_record(R29_FIRST_FALL_DIR / "summary.json", 48),
        source_record(R29_FIRST_FALL_DIR / "custody.json", 0),
        source_record(R29_FIRST_FALL_VALIDATION, 48),
        source_record(R29_ARCHIVE_READBACK, 0),
    ])

    candidate_records = []
    all_rows = []
    for candidate in sorted(rows_by_candidate):
        rows = rows_by_candidate[candidate]
        keys = [(row["family"], row["m"], row["target_kind"]) for row in rows]
        assert len(rows) == 48 and len(set(keys)) == 48
        assert set(row["m"] for row in rows) == set(range(2, 10))
        assert set(row["family"] for row in rows) == {"koblitz", "binary", "prime"}
        assert set(row["target_kind"] for row in rows) == {"natural", "planted"}
        assert all(row.get("last_fall_degree") is None for row in rows)
        status_counts = Counter(row["status"] for row in rows)
        replay_count = sum(bool(row.get("replay_verified", row.get("replay", {}).get("verified"))) for row in rows)
        ffd_counts = Counter((row.get("family"), row.get("first_fall_degree"),
                              row.get("first_fall_lower_bound")) for row in rows)
        candidate_records.append({
            "candidate_id": candidate,
            "systems": len(rows),
            "replay_verified": replay_count,
            "status_counts": dict(sorted(status_counts.items())),
            "first_fall_by_family": {
                family: {
                    "rows": sum(count for (fam, _, _), count in ffd_counts.items() if fam == family),
                    "outcomes": [
                        {"degree": degree, "lower_bound": lower_bound, "count": count}
                        for (fam, degree, lower_bound), count in sorted(
                            ffd_counts.items(), key=lambda item: str(item[0])) if fam == family
                    ],
                }
                for family in ("koblitz", "binary", "prime")
            },
            "exact_lfd_rows": sum(row.get("last_fall_degree") is not None for row in rows),
        })
        all_rows.extend(rows)

    covered = len(all_rows)
    assert covered == 480
    assert set(rows_by_candidate) == {"spike27", "spike28", "spike30", "spike31", "spike32", "spike33",
                                     "spike29", "spike35", "spike36", "spike37"}
    assert len({(r.get("candidate_id"), r["id"]) for r in all_rows}) == covered
    total_ffd = Counter((r["family"], r.get("first_fall_degree"), r.get("first_fall_lower_bound")) for r in all_rows)
    assert total_ffd == Counter({("koblitz", 2, 2): 160, ("binary", 2, 2): 160, ("prime", None, 5): 160})

    custody = load(CUSTODY)
    archive = custody["canonical_archive"]
    assert archive["full_s3_readback_verified"] is True
    assert archive["sha256"] == archive["readback_sha256"]
    sources.append(source_record(CUSTODY, 0))
    lfd_summary = load(LFD_SUMMARY)
    assert lfd_summary["verified_cases"] == len(lfd_summary["rows"]) == 12
    assert all(row["verified"] and row["replay_exit_code"] == 0 for row in lfd_summary["rows"])
    assert all(row["last_fall_degree"] is None and row["last_fall_upper_bound"] is None
               and not row["contains_one"] and not row["exhaustive_affine_completion"]
               for row in lfd_summary["rows"])
    assert all(row["inherited_last_fall_lower_bound"] == 4 for row in lfd_summary["rows"])
    sources.append(source_record(LFD_SUMMARY, len(lfd_summary["rows"])))

    # Inventory extant sources and terminal records for the unlabelled groups.
    # This checks metadata and file presence only; it never interprets top-form
    # Dreg as affine Dreg or as an FFD/LFD result.
    r29_inputs_path = R29_INPUT_DIR / "inputs.json"
    r29_manifest_path = R29_INPUT_DIR / "manifest.sha256"
    r29_inputs = load(r29_inputs_path)
    assert len(r29_inputs) == 48
    assert Counter(row["family"] for row in r29_inputs) == Counter(
        {"koblitz": 16, "binary": 16, "prime": 16})
    r29_system_paths = [R29_INPUT_DIR / row["id"] / "system.sing" for row in r29_inputs]
    assert all(path.is_file() for path in r29_system_paths)
    r29_manifest_paths = {
        line.split(None, 1)[1].lstrip("*")
        for line in r29_manifest_path.read_text().splitlines() if line.strip()
    }
    assert all(f"{row['id']}/system.sing" in r29_manifest_paths for row in r29_inputs)
    r29_top_run = load(R29_TOP_DIR / "run.json")
    r29_top_replay = load(R29_TOP_DIR / "replay-summary.json")
    assert r29_top_run["rows"] == r29_top_run["replay_rows"] == 48
    assert r29_top_run["replay_all_verified"] is True
    assert r29_top_replay["all_verified"] is True and len(r29_top_replay["rows"]) == 48
    assert r29_preflight["candidate_id"] == "spike29"
    r29_binding = load(R29_BINDING_DIR / "audit.json")
    assert r29_binding["schema"] == "n83.r29-pdp-source-binding-audit/v1"
    assert r29_binding["protocol"]["id"] == EXPECTED_PROTOCOL_ID
    assert r29_binding["protocol"]["frozen_protocol_sha256"] == EXPECTED_PROTOCOL_SHA256
    assert r29_binding["factor_base"]["complete"] is True
    assert r29_binding["factor_base"]["same_base_for_m"] == list(range(2, 10))
    assert r29_binding["pdp_inputs"]["rows"] == 48
    assert r29_binding["top_form_replay_binding"]["rows"] == 48
    assert r29_binding["top_form_replay_binding"]["source_hashes_match"] is True
    sources.extend([
        source_record(r29_inputs_path, len(r29_inputs)),
        source_record(r29_manifest_path, len(r29_system_paths)),
        source_record(R29_TOP_DIR / "run.json", 48),
        source_record(R29_TOP_DIR / "replay-summary.json", 48),
        source_record(R29_BINDING_DIR / "audit.json", 48),
        source_record(R29_BINDING_DIR / "FILE_LAYOUT.json", 0),
    ])

    r38_preflight = load(R38_TOP_DIR / "preflight.json")
    r38_terminal = load(R38_TOP_DIR / "terminal.json")
    assert r38_preflight["candidate_id"] == "spike38" and r38_preflight["systems"] == 48
    assert r38_preflight["complete_base_index"] is True
    assert r38_terminal["candidate_id"] == "spike38" and r38_terminal["exit_code"] == 1
    assert not (R38_TOP_DIR / "certificates" / "summary.json").exists()
    r39_preflight = load(R39_TOP_DIR / "preflight.json")
    r39_terminal = load(R39_TOP_DIR / "terminal.json")
    r39_failure = load(R39_TOP_DIR / "failure.json")
    assert r39_preflight["candidate_id"] == "spike39" and r39_preflight["systems"] == 48
    assert r39_terminal["candidate_id"] == "spike39" and r39_terminal["exit_code"] == 1
    assert "aws s3 ls" in r39_failure["failing_command"]
    assert not (R39_TOP_DIR / "certificates" / "summary.json").exists()
    sources.extend([
        source_record(R38_TOP_DIR / "preflight.json", 48),
        source_record(R38_TOP_DIR / "terminal.json", 0),
        source_record(R39_TOP_DIR / "preflight.json", 48),
        source_record(R39_TOP_DIR / "terminal.json", 0),
        source_record(R39_TOP_DIR / "failure.json", 0),
    ])

    return {
        "schema": "n83.fixed-base-first-fall-coverage/v1",
        "scope": "Fixed factor-base coordinates spike27 through spike39; 48 systems per candidate, m2-9, three families, natural/planted targets.",
        "protocol": {
            "id": EXPECTED_PROTOCOL_ID,
            "freeze_sha256": EXPECTED_PROTOCOL_SHA256,
            "planned_systems": planned,
            "source_commit": protocol["freeze"]["commit"],
        },
        "coverage": {
            "systems_planned": planned,
            "systems_with_first_fall_records": covered,
            "candidate_groups_covered": sorted(rows_by_candidate),
            "candidate_groups_uncovered": [c for c in EXPECTED_CANDIDATES if c not in rows_by_candidate],
            "uncovered_systems": planned - covered,
            "known_complete_base_unlabeled_candidates": ["spike38", "spike39"],
            "candidate_missing_factor_base": "spike34",
            "source_availability": {
                "spike29": {
                    "pdp_system_files_locally_present": 48,
                    "pdp_input_manifest_sha256": digest(r29_manifest_path),
                    "prior_top_form_certificates_replayed": 48,
                    "frozen_v2_source_binding_audit_sha256": digest(R29_BINDING_DIR / "audit.json"),
                    "frozen_v2_source_hashes_bound_to_top_form_replay": True,
                    "v2_first_fall_rows": 48,
                    "v2_first_fall_certificates_replayed": 48,
                    "first_fall_summary_sha256": digest(R29_FIRST_FALL_DIR / "summary.json"),
                    "first_fall_independent_validation_sha256": digest(R29_FIRST_FALL_VALIDATION),
                    "affine_dreg_rows": 0,
                    "source_binding_note": "All 48 saved PDP source hashes are bound to the frozen V2 spike29 family arms; all 48 first-fall certificates have fresh-process replay and byte-matching S3 readback. No affine Dreg labels are produced.",
                },
                "spike38": {
                    "complete_factor_base_index": True,
                    "top_form_preflight_systems": 48,
                    "top_form_terminal_exit_code": 1,
                    "top_form_certificate_summary_present": False,
                    "v2_first_fall_rows": 0,
                    "affine_dreg_rows": 0,
                },
                "spike39": {
                    "complete_factor_base_index": True,
                    "top_form_preflight_systems": 48,
                    "top_form_terminal_exit_code": 1,
                    "top_form_failure": "aws s3 ls failed before data production",
                    "top_form_certificate_summary_present": False,
                    "v2_first_fall_rows": 0,
                    "affine_dreg_rows": 0,
                },
                "spike34": {
                    "complete_factor_base": False,
                    "pdp_system_files_locally_present": 0,
                    "v2_first_fall_rows": 0,
                    "affine_dreg_rows": 0,
                },
            },
        },
        "first_fall_outcomes": {
            "rows": covered,
            "replay_verified": sum(bool(r.get("replay_verified", r.get("replay", {}).get("verified"))) for r in all_rows),
            "by_family": {
                "koblitz": {"rows": 160, "exact_degree_2": 160},
                "binary": {"rows": 160, "exact_degree_2": 160},
                "prime": {"rows": 160, "exact_degree": 0, "lower_bound_5": 160},
            },
            "candidate_specific_outcome_variation_within_family": False,
            "interpretation": "The observed FFD labels do not separate the ten covered bases within a family; they cannot train an elite-base model for degree-of-regularity reduction.",
        },
        "last_fall_degree": {
            "exact_rows": 0,
            "unknown_rows": covered,
            "candidate_first_fall_panel": "The 480 candidate first-fall certificates do not establish LFD.",
            "related_restricted_propagation_panel": {
                "verified_m3_cases": 12,
                "inherited_strict_lower_bound_4_cases": 12,
                "global_upper_certificates": 0,
                "exact_lfd_certificates": 0,
                "scope": "Restricted V4 consequence propagation; no unrestricted V4 closure or higher-fall inference.",
                "summary": LFD_SUMMARY.relative_to(ROOT).as_posix(),
                "report": "research/n83_candidate_pdp_20261028/reports/run-20261029T1907Z/report.md",
            },
            "related_panel_report": "research/n83_candidate_pdp_20261028/reports/run-20261029T1907Z/report.md",
        },
        "degree_of_regularity": {
            "measured_in_this_panel": False,
            "top_form_proxies_are_separate": True,
            "model_lead_from_top_form_proxies": False,
        },
        "s3_archive_for_spike35_to_spike37_panel": {
            "key": archive["key"],
            "bytes": archive["bytes"],
            "sha256": archive["sha256"],
            "full_readback_verified": archive["full_s3_readback_verified"],
            "scope": custody["scope"],
            "source_system_payloads_in_archive": archive["excludes_source_system_payloads"] is False,
            "custody_record": CUSTODY.relative_to(ROOT).as_posix(),
        },
        "s3_archive_for_spike29_panel": {
            "key": r29_custody["archive"]["key"],
            "bytes": r29_custody["archive"]["bytes"],
            "sha256": r29_custody["archive"]["sha256"],
            "full_readback_verified": r29_archive_readback["status"] == "verified",
            "custody_record": (R29_FIRST_FALL_DIR / "custody.json").relative_to(ROOT).as_posix(),
            "archive_readback_record": R29_ARCHIVE_READBACK.relative_to(ROOT).as_posix(),
        },
        "candidate_groups": candidate_records,
        "source_files": sources,
        "file_structure": {
            "per_candidate_summaries": "Six 48-row first-fall summaries are linked by source path and hash.",
            "spike35_spike37_panel": "One 144-row combined summary is linked by source path and hash; attempt-01 and attempt-02 records remain preserved separately.",
            "spike35_spike37_raw_case_archive": "The canonical 144-case S3 archive and custody record are referenced without copying raw case files into this coverage index; source system payloads are stored separately.",
            "spike29_raw_case_archive": "The canonical 48-case S3 archive and custody record are hash-bound; certificate and attempt records are separately available under the same S3 prefix.",
            "other_candidate_archives": "The six earlier candidate summaries retain their own evidence directories and S3 persistence references; this index records each summary's hash instead of flattening those attempts.",
            "unlabelled_source_availability": "source_availability distinguishes the now first-fall-labelled but affine-Dreg-unlabelled spike29 systems, spike38/spike39 bases with failed top-form attempts, and the missing spike34 base; linked metadata hashes are in source_files. This inventory does not represent failed or absent runs as measured labels.",
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true", help="compare the recomputed index with coverage.json")
    args = parser.parse_args()
    result = build()
    rendered = json.dumps(result, indent=2, sort_keys=True) + "\n"
    if args.check:
        if not OUT.is_file() or OUT.read_text() != rendered:
            print("coverage.json differs from recomputed source summaries")
            return 1
        print(f"PASS: {len(result['source_files'])} source objects; {result['coverage']['systems_with_first_fall_records']}/624 rows; 0 exact LFD")
        return 0
    OUT.write_text(rendered)
    print(f"wrote {OUT.relative_to(ROOT)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
