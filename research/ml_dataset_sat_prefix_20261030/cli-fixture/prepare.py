#!/usr/bin/env python3
"""Write an official experiment.protocol/v1 DRAFT; never freeze or run a solver.

Missing binary/runtime evidence stays null. Supply the final binary SHA and an
actual preflight receipt before requesting the official freeze operation.
"""
import argparse
import datetime
import math
import re
from pathlib import Path
from common import LIMITS, config, file_bindings, require, sha, write, load, HERE

ARM_IDS = ("native-sat", "copied-source-controls")


def runtime_preflight(path, source, binary):
    empty = {"fixed_seconds": None, "per_sample_seconds": None, "threads": 1,
             "estimated_total_seconds": None, "measured_on": None}
    if path is None:
        return empty, None
    require(binary is not None, "runtime evidence requires final binary SHA256")
    receipt = load(path)
    require(receipt.get("schema") == "ml-dataset.sat-fixture-runtime-preflight/v1",
            "wrong runtime preflight schema")
    require(receipt.get("source_commit") == source and receipt.get("binary_sha256") == binary,
            "runtime preflight source/binary mismatch")
    require(receipt.get("threads") == 1 and receipt.get("excluded_from_candidate_selection") is True,
            "preflight must be single-threaded and excluded from candidate selection")
    require(isinstance(receipt.get("measured_on"), str) and receipt["measured_on"].strip(),
            "runtime preflight measurement provenance required")
    measurements = receipt.get("arm_measurements")
    require(isinstance(measurements, list) and len(measurements) == 2 and
            {m.get("arm") for m in measurements} == set(ARM_IDS),
            "one actual runtime observation for each arm required")
    for measurement in measurements:
        value = measurement.get("elapsed_seconds")
        require(type(value) in (int, float) and math.isfinite(value) and value >= 0,
                "finite measured arm runtime required")
        require(measurement.get("observed") is True and
                isinstance(measurement.get("command"), list) and
                all(isinstance(x, str) for x in measurement["command"]) and measurement["command"],
                "actual command receipt required")
        require(re.fullmatch(r"[0-9a-f]{64}", measurement.get("receipt_sha256", "")),
                "arm command receipt SHA256 required")
        receipt_name = measurement.get("receipt_path")
        require(isinstance(receipt_name, str) and receipt_name, "arm command receipt path required")
        evidence_path = Path(receipt_name)
        if not evidence_path.is_absolute():
            evidence_path = Path(path).parent / evidence_path
        command_evidence = load(evidence_path, maximum=1024 * 1024)
        require(sha(evidence_path) == measurement["receipt_sha256"], "arm command receipt digest mismatch")
        require(command_evidence.get("argv", command_evidence.get("command")) == measurement["command"] and
                command_evidence.get("elapsed_seconds_diagnostic", command_evidence.get("elapsed_seconds")) == value,
                "runtime observation differs from retained command receipt")
    for key in ("fixed_seconds", "per_sample_seconds", "estimated_total_seconds"):
        value = receipt.get(key)
        require(type(value) in (int, float) and math.isfinite(value) and value >= 0,
                "finite runtime preflight field required: " + key)
        empty[key] = value
    empty["measured_on"] = receipt["measured_on"]
    require(isinstance(receipt.get("estimate_method"), str) and receipt["estimate_method"].strip(),
            "estimate derivation required; a ceiling is not a measured runtime")
    return empty, {"path": str(path), "sha256": sha(path), "receipt": receipt}


def prepare(source, binary=None, preflight_path=None):
    require(re.fullmatch(r"[0-9a-f]{40}", source), "full source commit required")
    require(binary is None or re.fullmatch(r"[0-9a-f]{64}", binary), "binary SHA256 required")
    runtime, evidence = runtime_preflight(preflight_path, source, binary)
    noncomparative = "Exact software correctness only; no runtime advantage, ECDLP cost, population estimate or statistical inference."
    instrument = {"repo": "crypto", "commit": source,
                  "build": [["cargo", "build", "--release", "--bin", "crypto"]],
                  "binary_sha256": binary, "threads": 1,
                  "env": {"RAYON_NUM_THREADS": "1", "OMP_NUM_THREADS": "1"}, "flags": {}}
    factor_rows = [
        ("instrument", "fixed", "Source/binary/script hashes, flags and build receipts bound before execution"),
        ("threads", "fixed", "RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1; one worker at a time"),
        ("family-arity-key-twist", "crossed", "Exact 16-cell candidate table; both members before pair assessment"),
        ("seed-domain-generator-order", "fixed", "Seed20261030, curve/generator0, base8, fixed mixed-radix table"),
        ("host-layout-environment", "measured", "Record CPU/frequency/SMT/NUMA/kernel/hypervisor/co-tenants/layout and process environment; no timing inference"),
        ("warmup", "fixed", "Zero warmups; exact correctness assertions"),
        ("interruption", "measured", "Retain interrupted attempts; no automatic solver retry or restoration"),
        ("analyst-selection", "fixed", "Frozen mechanical verifier and first-qualified-pair rule; no discretionary search"),
        ("input-mutation", "crossed", "Original source and four preregistered independently copied corruption/censor controls"),
    ]
    fixture = {"schema": "ml-dataset.sat-cli-fixture-bindings/v1", "source_commit": source,
               "binary_sha256": binary, "driver_files": file_bindings(), "limits": LIMITS,
               "candidates": [{"cell_index": i, "config": config(i)} for i in range(16)],
               "runtime_preflight_evidence": evidence,
               "pending_before_freeze": (["final binary digest and build receipt"] if binary is None else []) +
                   (["actual runtime preflight observations for both arms; no placeholder timings"] if evidence is None else []) +
                   ["passing scoped detector controls and final driver hashes", "official freeze and Git commit before candidate data"],
               "arm_population": {"native-sat": "at most16 candidate cells, one attempt each, paired in fixed order",
                                  "copied-source-controls": "exactly4 copied-source export controls only after a pair qualifies; otherwise not run"},
               "control_recipes": ["in-range GateMap semantic substitution with consistent outer hashes",
                                   "in-range Assignment/reason substitution with consistent outer hashes",
                                   "truncate later Decision before matching Assignment; preserve prior contexts",
                                   "inject well-formed dynamic Clause after a matched Assignment; truncate later suffix"],
               "unrun_reasons": ["not_run_after_first_qualified_pair", "not_run_driver_deadline",
                                 "not_run_storage_cap", "not_run_prior_command_failure", "not_run_missing_qualified_pair"],
               "coverage_prerequisite": "Scoped positive unit/nonunit entailment and native mixed-stream detector controls must pass; no unqualified absence result becomes evidence",
               "scientific_samples_added": 0, "verified_cloud_dataset_cells": 0,
               "clock_valid_for_comparison": False}
    return {
        "schema": "experiment.protocol/v1", "protocol_id": "SAT-PDP-CLI-20261030", "version": 1,
        "created": datetime.date.today().isoformat(), "repository": "crypto",
        "study_dir": "research/ml_dataset_sat_prefix_20261030/cli-fixture",
        "authors": ["ML dataset software validation"],
        "question": "Does the bounded native PDP fixture admit a curve/twist pair with checked causal contexts before and after a learned/backjump transition, with correct custody and censor controls?",
        "claims": [
            {"id": "causal-context", "class": "exact_computation", "text": "Selected native PDP pattern admits decisions before and after a checked learned/backjump transition.", "decided_by": ["native-acceptance"]},
            {"id": "twist-custody", "class": "exact_computation", "text": "Selected curve/twist pair has nonzero verified decision rows, exact artifact joins and immutable export retry.", "decided_by": ["native-acceptance"]},
            {"id": "controls", "class": "exact_computation", "text": "Four bounded copied-input controls reject or censor at their intended semantic gates while preserving earlier contexts.", "decided_by": ["control-acceptance"]},
            {"id": "advantage", "class": "out_of_scope", "text": noncomparative, "decided_by": []},
            {"id": "global-proof", "class": "out_of_scope", "text": "No complete native execution, global UNSAT proof, whole-solve instruction count or ECDLP recovery claim.", "decided_by": []}],
        "arms": [
            {"id": "native-sat", "role": "candidate", "description": "Bounded native generated PDP candidate selection and immutable retry; natural public targets.", "instrument": instrument},
            {"id": "copied-source-controls", "role": "control_negative", "description": "Four copied-source corruption/censor exports; only after a qualifying pair, never new generated instances.", "instrument": instrument, "same_instrument_as": "native-sat"}],
        "controls": {
            "aa": {"present": False, "comparison": None, "not_applicable_reason": noncomparative + " Immutable identical-input export retry checks determinism, not timing noise."},
            "positive": {"present": False, "arm": None, "expected_detection": None, "not_applicable_reason": "No separate planted generation arm: positive detector unit/nonunit entailment and mixed native-stream checks are required pre-execution. Natural candidate absence is missing_fixture, never a scientific null."},
            "negative": {"present": True, "arm": "copied-source-controls", "not_applicable_reason": None,
                         "null_population": {"description": "Four deterministic source substitutions/truncations outside semantic acceptance, each on a separate selected-cell copy", "generator": "frozen driver.py mutation recipes", "n": 4, "why_n": "one control for each declared semantic gate; no significance claim", "matched_on": ["source binary", "original selected cell", "all unmutated artifacts", "consistent copied outer hashes", "replay/export limits"]}}},
        "variable_ledger": {"independent": [{"name": "input-mutation", "levels": ["original", "map", "assignment", "truncation", "dynamic-clause"]}],
            "factors": [{"name": n, "treatment": t, "how": h} for n, t, h in factor_rows],
            "comparisons": [
                {"id": "native-acceptance", "arms": ["native-sat"], "differs_in": [], "design": "absolute", "threshold": "Both selected cells have nonzero verified decisions and one has a checked native learned/backjump transition between admitted decisions; joins, hashes and deterministic re-export pass"},
                {"id": "control-acceptance", "arms": ["copied-source-controls"], "differs_in": [], "design": "absolute", "threshold": "All four declared semantic outcomes verified; custody-only rejection leaves semantic control missing"}],
            "crosses_map": False, "aa_not_applicable": noncomparative},
        "randomization": {"arm_order": "fixed", "arm_order_reason": "Deterministic finite software fixture search; controls require the selected cell; no effect estimation",
            "seed_derivation": {"method": "explicit_list", "seeds": [20261030], "note": "Dataset generator uses this fixed seed and the frozen16-cell table; controls use no new random seeds"},
            "instance_sampling": {"rule": "Frozen candidate table independent of outcomes; first qualified pair only. Controls derive mechanically from selected cell and are not independent population samples.", "independent_of_arm": True},
            "blinding": {"analysis_labels_masked": False, "how": "Mechanical exact predicates; no human comparative-effect assessment", "unmasked_at": None}},
        "sample_size": {"instances": 16, "replicates": 1, "warmups": 0,
            "power": {"method": "exact", "alpha": 0.05, "tests_in_family": 1, "correction": "none", "passes": True,
                      "note": "Schema requires alpha/tests fields; these are inert for exact software predicates. No hypothesis test or statistical power claim.16 is a candidate ceiling, not completed instances per arm; extension specifies4 conditional controls."},
            "preflight_runtime": runtime},
        "metrics": {"primary": [
            {"id": "verified-decisions", "name": "Verified decision pairs", "unit": "admitted Decision/Assignment pairs", "kind": "counted", "definition": "Count accepted restricted decision records after causal replay; excludes unpaired/censored decisions", "source": "export manifest sat_outcomes.verified_decision_occurrences", "aggregation": "per_row", "resolution": "1", "uncertainty": "none_exact"},
            {"id": "qualified-transition", "name": "Qualified learned transition", "unit": "Boolean", "kind": "exact", "definition": "Same-pattern before Decision, Learned, matching Backjump, after Decision with matching Assignments and certified later admission", "source": "driver transition_witness", "aggregation": "per_row", "resolution": "1", "uncertainty": "none_exact"}], "secondary": []},
        "measurement": {"isolation_required": "none", "warmups": 0, "order": "sequential", "clock": "none",
            "record": ["source_commit", "binary_sha256", "threads", "host CPU/kernel/SMT/NUMA/hypervisor", "commands and exit status", "all failed/capped/interrupted outcomes", "scoped telemetry and native/replay counters", "artifact hashes and bytes"], "timed_primary_justification": None},
        "execution": {"backend": "local", "queue": None, "measurement_class": "search", "timeout_seconds": LIMITS["driver_seconds"],
            "placement": {"single_worker": True, "parent_serialized_validation": True},
            "checkpoint": {"policy": "none", "interval_seconds": None, "grace_seconds": 2, "restore_validation": "none", "after_restore": {"wall_time": "invalid", "counted_units": "valid"}},
            "retries": {"infrastructure_only": True, "max_attempts": 1}},
        "stopping_rules": [
            {"kind": "preregistered_sequential", "preregistered": True, "rule": "Stop after first fully qualified pair in frozen order; run both pair members before assessment; at most16 candidate cells/24 native patterns"},
            {"kind": "budget", "preregistered": True, "rule": "Stop at any frozen extension limit:1800 driver seconds,3GiB retained attempt, command deadlines, four corruption exports; never extend implicitly"},
            {"kind": "invalidation", "preregistered": True, "rule": "Hard binding/replay or unexpected command failures stop selection; retain partial outputs and exact failure reason"}],
        "invalidation_rules": ["No zero-row acceptance", "No raw learned-event proof without later admitted context", "Custody-only rejection does not satisfy intended semantic control", "Interrupted/capped/failed rows are retained, not discarded"],
        "decision_rule": {"text": "Pass only the exact native pair, custody/retry and all four semantic-control predicates. Otherwise report the specific missing/failed gate and stop without scientific promotion.", "comparison_ids": ["native-acceptance", "control-acceptance"], "statistic": "Boolean conjunction of exact bounded acceptance predicates", "interval": "Not applicable: deterministic exact predicates", "multiple_comparison_correction": "none", "effect_size_measure": "Not applicable: no comparative effect", "verdict_vocabulary": ["passed", "missing_fixture", "controls_incomplete", "failed", "interrupted"]},
        "analysis_plan": {"script": {"path": "verify.py", "sha256": sha(HERE / "verify.py")}, "inputs": ["driver receipt", "copied/source inventories", "native events", "public/restricted export files"], "outputs": ["exact acceptance/control results and artifact bindings"], "blinded": False, "frozen_before_data": True},
        "reporting": {"export_schema": "experiment.export/v1", "review_schema": "experiment.review/v1", "graphs": [], "report_dir": "reports"},
        "inadmissible": ["change seeds/table after outcomes", "discard failed attempts", "claim model or solver advantage", "manufacture preflight timing", "call custody rejection semantic replay", "promote missing fixture to negative scientific evidence"],
        "literature": [
            {"ref": "https://arxiv.org/html/1612.02353v2", "provenance": "retrieved", "claim": "RUP entailment rather than RAT-only satisfiability preservation; sections2-3 reviewed during frozen SAT contract", "verified_by": "SAT contract f71728b66558"},
            {"ref": "../CONTRACT.md", "provenance": "internal", "claim": "Bounded native gate regeneration and independent semantic replay; no global proof claim"},
            {"ref": "isogeny-labs/crypto-autoresearcher@f755085ff951888dbe64682c210c9ef7d167b40c:docs/experiment-protocol.md", "provenance": "retrieved", "claim": "Official protocol schema, checker, draft/freeze distinctions and honest runtime preflight"}],
        "amendments": [], "freeze": {"status": "draft", "protocol_sha256": None, "commit": None, "frozen_at": None, "filed_before_affected_data": None},
        "extensions": {"ml_dataset_sat_fixture": fixture},
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source-commit", required=True)
    parser.add_argument("--binary-sha256", help="Omit only while final binary is unavailable; draft remains unready")
    parser.add_argument("--runtime-preflight", help="Actual source/binary-bound preflight receipt, never a timing placeholder")
    parser.add_argument("--out", required=True)
    args = parser.parse_args()
    write(args.out, prepare(args.source_commit, args.binary_sha256, args.runtime_preflight), exclusive=True)
    print("Official-schema draft written; run the real validate checker. This command never freezes or executes a solver. Missing runtime/binary evidence remains pending.")


if __name__ == "__main__":
    main()
