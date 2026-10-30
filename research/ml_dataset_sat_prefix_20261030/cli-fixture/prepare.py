#!/usr/bin/env python3
"""Prepare a substantive protocol DRAFT; never freezes it or invokes a solver.

The referenced authoritative schema/checker is unavailable in this workspace.
This document must be adapted/validated by that tooling, frozen, and committed
before run.sh can admit it. No schema compatibility is asserted here.
"""
import argparse
import datetime
import re
from common import LIMITS, config, file_bindings, require, sha, write, HERE


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source-commit", required=True)
    parser.add_argument("--binary-sha256", required=True)
    parser.add_argument("--out", required=True)
    args = parser.parse_args()
    require(re.fullmatch("[0-9a-f]{40}", args.source_commit), "full source commit required")
    require(re.fullmatch("[0-9a-f]{64}", args.binary_sha256), "binary SHA256 required")
    noncomparative = "Software correctness only; no runtime, solver advantage, ECDLP cost, statistical or population claim."
    body = {
        "schema": "experiment.protocol/v1", "version": 1,
        "purpose": "bounded native SAT PDP CLI software correctness",
        "prepared_at": datetime.datetime.now(datetime.timezone.utc).isoformat(),
        "draft_status": "requires authoritative schema/checker, freeze and commit before execution",
        "official_schema_validation": "unavailable_at_authoring_time; not asserted",
        "claims": [
            {"id": "causal-context", "class": "exact_computation",
             "statement": "One retained native PDP pattern admits decisions before and after a checked learned/backjump transition."},
            {"id": "twist-custody", "class": "exact_computation",
             "statement": "A selected curve/twist pair has nonzero checked decision rows, exact artifact joins and immutable re-export."},
            {"id": "advantage", "class": "out_of_scope", "statement": noncomparative},
            {"id": "global-proof", "class": "out_of_scope",
             "statement": "No full native execution, global UNSAT, complete instructions, or ECDLP recovery claim."}],
        "literature": [{"source": "https://arxiv.org/html/1612.02353v2",
                        "provenance": "Primary LRAT paper sections2-3 checked during frozen SAT contract design; RUP entailment required, RAT-only satisfiability preservation insufficient."},
                       {"source": "../CONTRACT.md", "provenance": "Frozen source contract f71728b66558"}],
        "arms": [{"id": "native-sat", "role": "candidate", "source_commit": args.source_commit,
                  "binary_sha256": args.binary_sha256, "threads": 1,
                  "purpose": "actual CLI correctness fixture using natural generated public targets"}],
        "controls": {
            "reference": {"not_applicable_reason": noncomparative + " No matched rho speed comparison is made."},
            "baseline": {"not_applicable_reason": "Pinned static-export compatibility remains a separate regression gate; no candidate performance contrast."},
            "a_a": {"not_applicable_reason": "No timed primary metric or statistical inference; immutable identical-input export retry is checked separately."},
            "positive": {"description": "Verified source/decision pairs and learned transition detector; scoped unit controls include known entailed unit/nonunit clauses. No zero-row absence result is promoted without detector controls passing."},
            "negative": {"description": "Four preregistered copied-source map, reason, truncation and dynamic-clause corruptions; exact observed rejection stage retained."}},
        "comparisons": [{"kind": "absolute", "claim_ids": ["causal-context", "twist-custody"],
                         "decision": "exact acceptance predicates, no statistical comparison"}],
        "variable_ledger": [
            {"factor": "instrument/source/binary/scripts/flags", "treatment": "fixed", "value": "bound by fixture hashes"},
            {"factor": "worker threads", "treatment": "fixed", "value": 1},
            {"factor": "curve family/arity/key/twist", "treatment": "crossed", "value": "fixed16-cell table"},
            {"factor": "order/seed/curve-seed/generator/domain", "treatment": "fixed", "value": "fixed table, seed20261030, curve/generator0, base8"},
            {"factor": "CPU/frequency/SMT/NUMA/kernel/hypervisor/co-tenants/layout/environment", "treatment": "measured", "value": "host and process receipts; no comparable timing claim"},
            {"factor": "warmup", "treatment": "fixed", "value": "none; not timing"},
            {"factor": "spot/preemption/interruption", "treatment": "measured", "value": "interrupted attempts retained; no implicit retry"},
            {"factor": "analyst/selection", "treatment": "fixed", "value": "frozen verifier and first-qualified pair; no discretionary seed search"}],
        "randomization": {"order": "fixed", "reason": "deterministic bounded regression-fixture selection; not estimation",
                          "seed_method": "explicit", "seeds": [20261030], "instance_selection_independent_of_arm": True},
        "blinding": {"used": False, "not_applicable_reason": "Machine-verifiable exact predicates; no human comparative effect judgment."},
        "sample_size": {"instances_maximum": 16, "replicates": 1, "warmups": 0,
                        "rationale": "Finite software coverage/fixture search; first qualifying pair, no population estimate.",
                        "power_not_applicable_reason": "No hypothesis test, p-value, resampling or effect-size inference.",
                        "runtime_preflight": "not executed; bounded outer deadlines, no fabricated timing estimate"},
        "metrics": [
            {"id": "verified_decisions", "kind": "counted", "unit": "admitted decision/assignment pairs",
             "source_field": "sat_outcomes.verified_decision_occurrences", "aggregation": "per cell and all attempted cells",
             "definition": "matching restricted records after checked native replay", "resolution": 1, "uncertainty": "exact bounded count"},
            {"id": "qualified_transition", "kind": "counted", "unit": "Boolean exact predicate",
             "source_field": "transition_witness", "aggregation": "first qualifying pair", "resolution": 1,
             "definition": "same-pattern before/after certified Learned and matching Backjump", "uncertainty": "no statistical inference"},
            {"id": "costs", "kind": "diagnostic", "unit": "named native/verifier counts; CPU/time/bytes with scopes",
             "source_field": "commands, source_outcome, native_attempts, replay_summaries, telemetry",
             "aggregation": "retain every attempt including failed/censored; RSS nonadditive",
             "resolution": "native counter units", "uncertainty": "PMU unavailable/multiplexed flags and interrupted scopes preserved"}],
        "execution": {"backend": "local serialized software validation", "measurement_class": "search",
                      "isolation_required": "no timing inference; exclusive build/fixture execution by parent",
                      "limits": LIMITS,
                      "checkpoint": {"policy": "command receipts and immutable outputs, no solver-state restore",
                                     "interval": "before/after every command", "grace_seconds": 2,
                                     "restore_validation": "fresh source/binary/protocol hashes; never resume partial solver state"},
                      "retry_policy": "no implicit generation retry; immutable export retry only; new attempt for infrastructure retry"},
        "stopping": {"rule": "first qualified curve/twist pair in fixed table; otherwise16 cells or explicit global cap/error",
                     "no_implicit_extension": True},
        "decision_rules": {"vocabulary": ["passed", "missing_fixture", "controls_incomplete", "failed", "interrupted"],
                           "statistics": "not applicable; exact predicates", "multiple_comparisons": "not applicable; no significance claim",
                           "inadmissible": ["discard failed attempts", "change table after data", "zero-row success", "raw learned events without later admission", "label custody rejection as semantic replay"]},
        "analysis": {"script": "verify.py", "sha256": sha(HERE / "verify.py"),
                     "timed_primary_metric": False, "judgment": "mechanical correctness acceptance; scientific promotion out of scope"},
        "fixture": {"schema": "ml-dataset.sat-cli-fixture-bindings/v1",
                    "source_commit": args.source_commit, "binary_sha256": args.binary_sha256,
                    "driver_files": file_bindings(), "limits": LIMITS,
                    "candidates": [{"cell_index": i, "config": config(i)} for i in range(16)]},
        "scientific_samples_added": 0, "verified_cloud_dataset_cells": 0,
        "clock_valid_for_comparison": False,
        "unified_export_gate": "authoritative experiment.export/v1 adapter/checker unavailable at authoring; emit retained rows/flow and explicit missing gate until located",
    }
    write(args.out, body, exclusive=True)
    print("Draft written. It is NOT frozen or schema-validated. Locate authoritative tooling, validate, freeze, commit, then retain a checker receipt before run.sh.")


if __name__ == "__main__":
    main()
