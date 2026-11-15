#!/usr/bin/env python3
"""Build the draft umbrella protocol for the ECBench frontier refresh."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent
COMMIT = "ff7ad61cfed11908fb79de9dd485b696d0ada779"
BINARY = "924d090c1d83a20ce954848c04b116cee4d5a7cb04187bb618dff60958bfd64d"


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def instrument() -> dict:
    return {
        "repo": "crypto",
        "commit": COMMIT,
        "build": [["cargo", "build", "--release", "--bin", "ecbench"]],
        "binary_sha256": BINARY,
        "flags": {},
        "env": {"RAYON_NUM_THREADS": "1", "OMP_NUM_THREADS": "1"},
        "threads": 1,
    }


def protocol() -> dict:
    arms = [
        {
            "id": "prime-rho-v2",
            "role": "reference",
            "description": "rho.negation_v2 in every prime spec",
            "instrument": instrument(),
            "parameters": {"method": "rho.negation_v2", "specs": "prime-p20,p24,p28,p32"},
        },
        {
            "id": "prime-candidates",
            "role": "candidate",
            "description": "current factor-base, root-finding, BSGS and kangaroo prime arms",
            "instrument": instrument(),
            "parameters": {"manifest": "execution/shard-manifest.json", "cell": "prime"},
            "same_instrument_as": "prime-rho-v2",
        },
        {
            "id": "koblitz-rho-v2",
            "role": "reference",
            "description": "rho.signed_frobenius_strong_v2 plus a preregistered strong-escape comparison in every Koblitz frontier cell",
            "instrument": instrument(),
            "parameters": {
                "method": "rho.signed_frobenius_strong_v2",
                "additional_reference": "rho.signed_frobenius_strong_escape",
                "reference_rule": "paired minimum common-accounting S; union of both A/A envelopes",
                "specs": "specs/shards/koblitz-n13,n19,n23,n31 groups",
            },
            "same_instrument_as": "prime-rho-v2",
        },
        {
            "id": "koblitz-candidates",
            "role": "candidate",
            "description": "root finding, inherited F4, F6-IC and pair-claw Koblitz arms",
            "instrument": instrument(),
            "parameters": {"manifest": "execution/shard-manifest.json", "cell": "koblitz"},
            "same_instrument_as": "prime-rho-v2",
        },
        {
            "id": "solver-candidates",
            "role": "candidate",
            "description": "all 11 live nested algebraic solver backends on one matched E2E cell",
            "instrument": instrument(),
            "parameters": {"manifest": "execution/shard-manifest.json", "cell": "solver-backends"},
            "same_instrument_as": "prime-rho-v2",
        },
        {
            "id": "random-rho-v2",
            "role": "reference",
            "description": "rho.negation_v2 on the four random-binary sizes",
            "instrument": instrument(),
            "parameters": {"method": "rho.negation_v2", "cell": "random-binary"},
            "same_instrument_as": "prime-rho-v2",
        },
        {
            "id": "random-candidates",
            "role": "candidate",
            "description": "matrix F4/F5 and inherited F4/F6 on matched random-binary systems",
            "instrument": instrument(),
            "parameters": {"manifest": "execution/shard-manifest.json", "cell": "random-binary"},
            "same_instrument_as": "prime-rho-v2",
        },
        {
            "id": "j0-rho-v2",
            "role": "reference",
            "description": "rho.cubic_v2 on the certified j=0 subgroup",
            "instrument": instrument(),
            "parameters": {"method": "rho.cubic_v2", "cell": "symmetry-j0"},
            "same_instrument_as": "prime-rho-v2",
        },
        {
            "id": "j0-candidates",
            "role": "candidate",
            "description": "legacy cubic rho and current generic algorithms on the j=0 fixture",
            "instrument": instrument(),
            "parameters": {"manifest": "execution/shard-manifest.json", "cell": "symmetry-j0"},
            "same_instrument_as": "prime-rho-v2",
        },
        {
            "id": "j1728-rho-v2",
            "role": "reference",
            "description": "rho.quartic_v2 on the certified j=1728 subgroup",
            "instrument": instrument(),
            "parameters": {"method": "rho.quartic_v2", "cell": "symmetry-j1728"},
            "same_instrument_as": "prime-rho-v2",
        },
        {
            "id": "j1728-candidates",
            "role": "candidate",
            "description": "legacy quartic rho and current generic algorithms on the j=1728 fixture",
            "instrument": instrument(),
            "parameters": {"manifest": "execution/shard-manifest.json", "cell": "symmetry-j1728"},
            "same_instrument_as": "prime-rho-v2",
        },
        {
            "id": "generator-rho-full",
            "role": "reference",
            "description": "both full-price Frobenius rho references on the n=29 engineering panel",
            "instrument": instrument(),
            "parameters": {"cell": "generator-orbit", "unit": "full-price"},
            "same_instrument_as": "prime-rho-v2",
        },
        {
            "id": "generator-candidates",
            "role": "candidate",
            "description": "lazy, layout and generator-orbit one-column full-price pipelines",
            "instrument": instrument(),
            "parameters": {"manifest": "execution/shard-manifest.json", "cell": "generator-orbit"},
            "same_instrument_as": "prime-rho-v2",
        },
        {
            "id": "f2m4-rho",
            "role": "reference",
            "description": "counted strong negation rho on one-word fixtures and the implemented wide rho at m=16",
            "instrument": instrument(),
            "parameters": {"cells": "f2m4-specialized,f2m4-wide", "target_kind": "planted"},
            "same_instrument_as": "prime-rho-v2",
        },
        {
            "id": "f2m4-candidates",
            "role": "candidate",
            "description": "dense/Wiedemann Gaudry-D4 binary pipelines; planted-target implementation panel only",
            "instrument": instrument(),
            "parameters": {"manifest": "execution/shard-manifest.json", "cells": "f2m4-specialized,f2m4-wide"},
            "same_instrument_as": "prime-rho-v2",
        },
        {
            "id": "large-prime-rho",
            "role": "reference",
            "description": "both current Koblitz strong-rho references on the explicit n=11 fixture",
            "instrument": instrument(),
            "parameters": {"cell": "binary-large-prime"},
            "same_instrument_as": "prime-rho-v2",
        },
        {
            "id": "large-prime-candidate",
            "role": "candidate",
            "description": "exact n-1 double-large-prime v2 candidate",
            "instrument": instrument(),
            "parameters": {"manifest": "execution/shard-manifest.json", "cell": "binary-large-prime"},
            "same_instrument_as": "prime-rho-v2",
        },
        {
            "id": "rho-aa",
            "role": "control_aa",
            "description": "identical applicable rho method under a second arm name in every spec",
            "instrument": instrument(),
            "parameters": {"method": "same as cell reference", "role_in_spec": "control"},
            "same_instrument_as": "prime-rho-v2",
        },
    ]
    return {
        "schema": "experiment.protocol/v1",
        "protocol_id": "ECBENCH-E2E-FRONTIER-20261031",
        "version": 1,
        "created": "2026-10-31",
        "repository": "crypto",
        "study_dir": "research/ecbench_e2e_frontier_20261031",
        "authors": ["Codex frontier review"],
        "links": {
            "manifest": "execution/manifest.json",
            "shard_manifest": "execution/shard-manifest.json",
            "shard_preflight": "execution/shard-preflight-plans.json",
            "sharding": "SHARDS.md",
            "coverage": "coverage.json",
            "implementations": "IMPLEMENTATIONS.md",
            "literature": "LITERATURE.md",
            "blocker": "execution/BLOCKER.md",
            "publication": "execution/publication.json",
        },
        "question": (
            "What is the cold one-target E2E performance frontier of every current applicable ECBench candidate, "
            "relative to the strongest accounting-correct rho in the same domain, and which costs or evidence gaps prevent promotion?"
        ),
        "claims": [
            {
                "id": "C1",
                "text": "Every admitted row recovers and independently verifies the discrete logarithm.",
                "class": "exact_computation",
                "decided_by": [],
            },
            {
                "id": "C2",
                "text": "A candidate moves a domain frontier only if its paired cold E2E cost is below the matched rho outside the A/A spread across the full pricing band; planted extension-field panels cannot decide a public-target claim.",
                "class": "comparative",
                "decided_by": [
                    "C-prime",
                    "C-koblitz",
                    "C-random",
                    "C-j0",
                    "C-j1728",
                    "C-generator",
                    "C-f2m4",
                    "C-large-prime",
                ],
                "if_supported": "promote only the exact domain, sizes, target law, unit and evidence level measured",
                "if_not_supported": "retain the row as dominated, tied, censored or unranked; do not extrapolate a win",
            },
            {
                "id": "C3",
                "text": "Solver backends may be cross-ranked only by verified cold E2E L2 wall or a complete shared priced unit, never by unlike native counters.",
                "class": "comparative",
                "decided_by": ["C-solver"],
                "if_supported": "report the winning backend for this matched system cell",
                "if_not_supported": "report backend-local counts without a global rank",
            },
        ],
        "hypotheses": [
            {
                "id": "H1",
                "statement": "The current cold one-target IC candidates remain above matched rho on the four prime and the medium/scaling Koblitz cells; the one-column generator-orbit fixed-setup panel is a separately classified engineering crossover.",
                "mechanism": "relation probability, factor-base/setup work, and algebraic closure dominate the constant-factor backend gains",
                "predictions": [
                    {
                        "id": "P1",
                        "metric": "s_ratio_to_rho",
                        "direction": "higher",
                        "minimum_effect": "lower 95% bound above 1 at every pricing weight",
                        "provenance": {
                            "written_at": "2026-10-31",
                            "data_seen": [
                                {"path": "docs/ic/LEADERBOARD.md", "measured": "historical whole-pipeline ratios"},
                                {"path": "research/f4_f5_f6_cross_family_20261011/RESULT.md", "measured": "historical solver cells"},
                            ],
                            "sample_status": "in_sample",
                        },
                        "role": "calibration",
                    }
                ],
            }
        ],
        "arms": arms,
        "controls": {
            "aa": {"present": True, "comparison": "C-aa", "not_applicable_reason": None},
            "positive": {
                "present": False,
                "arm": None,
                "expected_detection": None,
                "not_applicable_reason": "every workload has an independently checked discrete-log recovery path; planted scalars are retained only by the runner and public targets are checked by [d]G=Q",
            },
            "negative": {
                "present": False,
                "arm": None,
                "null_population": None,
                "not_applicable_reason": "the study compares exact solver costs and makes no classifier/null-population claim",
            },
        },
        "variable_ledger": {
            "independent": [
                {
                    "name": "algorithm",
                    "levels": [
                        "matched rho",
                        "prime candidates",
                        "Koblitz candidates",
                        "random-binary candidates",
                        "symmetry candidates",
                        "generator-orbit candidates",
                        "extension-field candidates",
                        "large-prime candidate",
                        "solver backends",
                    ],
                }
            ],
            "factors": [
                {"name": "domain", "treatment": "fixed", "value": "the panel named in shard-manifest.json", "how": "never pooled across domains"},
                {"name": "size", "treatment": "fixed", "value": "four preregistered sizes in each scaling domain", "how": "two EKS target blocks per comparison group and size"},
                {"name": "target", "treatment": "randomized", "how": "two frozen four-target blocks, eight targets per comparison cell"},
                {"name": "target_law", "treatment": "fixed", "value": "public except planted f2m4 implementation panels", "how": "never pool planted and public rows; planted rows cannot decide a public-target claim"},
                {"name": "pricing_weight", "treatment": "fixed", "value": "0, 0.1, 1 GAE/native op", "how": "all three reported"},
                {"name": "arm_order", "treatment": "randomized", "how": "six alternate measured rounds after one warmup"},
                {"name": "host_load", "treatment": "measured", "how": "native isolation capsule and per-row L2 grade"},
            ],
            "comparisons": [
                {"id": "C-prime", "arms": ["prime-rho-v2", "prime-candidates"], "differs_in": ["algorithm"], "design": "one_factor"},
                {"id": "C-koblitz", "arms": ["koblitz-rho-v2", "koblitz-candidates"], "differs_in": ["algorithm"], "design": "one_factor"},
                {"id": "C-random", "arms": ["random-rho-v2", "random-candidates"], "differs_in": ["algorithm"], "design": "one_factor"},
                {"id": "C-j0", "arms": ["j0-rho-v2", "j0-candidates"], "differs_in": ["algorithm"], "design": "one_factor"},
                {"id": "C-j1728", "arms": ["j1728-rho-v2", "j1728-candidates"], "differs_in": ["algorithm"], "design": "one_factor"},
                {"id": "C-generator", "arms": ["generator-rho-full", "generator-candidates"], "differs_in": ["algorithm"], "design": "one_factor"},
                {"id": "C-f2m4", "arms": ["f2m4-rho", "f2m4-candidates"], "differs_in": ["algorithm"], "design": "one_factor"},
                {"id": "C-large-prime", "arms": ["large-prime-rho", "large-prime-candidate"], "differs_in": ["algorithm"], "design": "one_factor"},
                {"id": "C-solver", "arms": ["koblitz-rho-v2", "solver-candidates"], "differs_in": ["algorithm"], "design": "one_factor"},
                {"id": "C-aa", "arms": ["prime-rho-v2", "rho-aa"], "differs_in": [], "design": "aa"},
            ],
            "aa_not_applicable": None,
            "crosses_map": False,
        },
        "randomization": {
            "arm_order": "interleaved",
            "arm_order_reason": None,
            "seed_derivation": {
                "method": "harness_drawn",
                "seeds": [],
                "nonce": None,
                "note": "ecbench derives every algorithm seed from the frozen spec, workload and round",
            },
            "instance_sampling": {"rule": "two frozen four-target blocks per comparison cell; public SHA-256 targets except the disclosed planted f2m4 panels", "independent_of_arm": True},
            "blinding": {"analysis_labels_masked": False, "how": "method identities are required for accounting audits", "unmasked_at": None},
        },
        "sample_size": {
            "instances": 8,
            "replicates": 6,
            "warmups": 1,
            "power": {
                "method": "paired_bootstrap",
                "alpha": 0.05,
                "tests_in_family": 9,
                "correction": "holm",
                "rank_test_n": None,
                "min_attainable_two_sided_p": None,
                "passes": True,
                "note": "eight targets and six rounds per cell; inference clusters by workload and never pools domains",
            },
            "preflight_runtime": {
                "fixed_seconds": 0,
                "per_sample_seconds": 0,
                "threads": 1,
                "estimated_total_seconds": 0,
                "measured_on": "plan-only expansion complete; exact-image per-arm runtime preflight remains a freeze blocker and no local wall timing is substituted",
            },
        },
        "metrics": {
            "primary": [
                {
                    "id": "s_ratio_to_rho",
                    "name": "cold E2E S ratio to matched rho",
                    "unit": "dimensionless",
                    "kind": "counted",
                    "definition": "sum of completely charged candidate S divided by matched-reference S on paired workloads",
                    "source": "ecbench compare ops at weights 0, 0.1 and 1",
                    "aggregation": "paired_ratio",
                    "resolution": "native counter resolution",
                    "uncertainty": "paired_bootstrap_ci95",
                }
            ],
            "secondary": [
                {
                    "id": "wall_ratio_to_rho",
                    "name": "cold E2E wall ratio to matched rho",
                    "unit": "dimensionless",
                    "kind": "timed",
                    "definition": "solve wall ratio on paired rows that all earned L2 or L3",
                    "source": "records.jsonl time.solve_wall_ns",
                    "aggregation": "paired_ratio",
                    "resolution": "1 us",
                    "uncertainty": "paired_bootstrap_ci95",
                },
                {
                    "id": "peak_rss",
                    "name": "peak resident memory",
                    "unit": "bytes",
                    "kind": "counted",
                    "definition": "maximum child RSS retained per row",
                    "source": "records.jsonl resources",
                    "aggregation": "distribution",
                    "resolution": "page",
                    "uncertainty": "range",
                },
            ],
        },
        "measurement": {
            "isolation_required": "L2",
            "warmups": 1,
            "order": "alternate",
            "clock": "counted",
            "counters": ["total_gae", "native operations", "field operations", "solver-native work", "failures", "timeouts"],
            "record": ["cpu_model", "kernel_release", "binary_sha256", "env_class_id", "isolation", "steal", "PSI", "run_cpu", "threads"],
            "timed_primary_justification": None,
        },
        "execution": {
            "backend": "isolab-k8s",
            "queue": "cpu-l2-spot",
            "placement": {"instance_type": "c8a.2xlarge", "cpus": 4, "spot": True},
            "measurement_class": "measurement",
            "timeout_seconds": 3600,
            "checkpoint": {
                "policy": "none",
                "interval_seconds": None,
                "grace_seconds": None,
                "restore_validation": "none",
                "after_restore": {"wall_time": "invalid", "counted_units": "valid"},
            },
            "retries": {"infrastructure_only": True, "max_attempts": 1},
        },
        "stopping_rules": [
            {"rule": "92 plan-valid candidate/reference/A-A shards, two four-target blocks, one warmup and six measured rounds; the exact IDs remain draft until runtime preflight and analysis hashing", "kind": "fixed_n", "preregistered": True},
            {"rule": "a per-execution timeout is a censored row, never a zero-cost row and never silently retried", "kind": "budget", "preregistered": True},
            {"rule": "stop without rank promotion if no authenticated EKS route or trusted image digest is available", "kind": "invalidation", "preregistered": True},
        ],
        "invalidation_rules": [
            "wrong answers are retained and exclude the affected arm from decisive ranking",
            "wall time below L2 is descriptive only",
            "a native solver counter with incomplete scope is backend-local and cannot decide a global rank",
            "a planted f2m4 row cannot decide a public-target frontier claim",
            "a missing result is blocked/unmeasured, not a loss",
        ],
        "decision_rule": {
            "text": "Promote only when the candidate/reference paired ratio has a Holm-adjusted 95% interval below 1, exceeds the applicable A/A spread, remains below 1 at weights 0, 0.1 and 1, verifies every decisive answer, has no hidden/unpriced work outside the declared band, and matches the claim's target law.",
            "comparison_ids": [
                "C-prime",
                "C-koblitz",
                "C-random",
                "C-j0",
                "C-j1728",
                "C-generator",
                "C-f2m4",
                "C-large-prime",
                "C-solver",
            ],
            "threshold": "upper interval below 1 across the pricing band and outside A/A",
            "statistic": "ratio of summed paired S; L2 wall reported separately",
            "interval": "cluster bootstrap by workload, 95%, 4000 resamples",
            "multiple_comparison_correction": "holm",
            "effect_size_measure": "candidate/reference ratio and distance to theoretical rho floor",
            "verdict_vocabulary": ["frontier_promoted", "dominated", "tied_within_control", "censored", "unranked_incomplete_accounting", "blocked_infrastructure"],
        },
        "analysis_plan": {
            "script": {
                "path": "research/ecbench_e2e_frontier_20261031/analyze_shards.py",
                "sha256": sha256(ROOT / "analyze_shards.py"),
            },
            "inputs": ["session/records.jsonl", "session/session.json", "audit.json", "l2-gate.json"],
            "outputs": ["comparison JSON", "pricing-band table", "Pareto frontier", "REPORT.pdf"],
            "blinded": False,
            "frozen_before_data": True,
        },
        "reporting": {
            "export_schema": "experiment.export/v1",
            "review_schema": "experiment.review/v1",
            "graphs": ["candidate/rho ratio by size and pricing weight", "phase bottleneck shares", "work-memory Pareto frontier"],
            "report_dir": "reports",
        },
        "inadmissible": [
            "changing seeds, targets, accounting, budgets or arm membership after observing results",
            "using local shared-container wall time as the EKS result",
            "comparing SAT conflicts, FES vector XORs, F4 partial word XORs or GAE as one common unit",
            "calling post-freeze timeouts or absent rows wins",
            "extrapolating toy curves to cryptographic sizes without a separately qualified scaling model",
        ],
        "literature": [
            {"ref": "Shoup 1997, DOI 10.1007/3-540-69053-0_18", "provenance": "retrieved", "claim": "generic square-root lower bound", "verified_by": "LITERATURE.md, 2026-10-31"},
            {"ref": "Faugere F4 1999, DOI 10.1016/S0022-4049(99)00005-5", "provenance": "retrieved", "claim": "F4 algorithm and accounting scope", "verified_by": "LITERATURE.md, 2026-10-31"},
            {"ref": "Faugere F5 2002, DOI 10.1145/780506.780516", "provenance": "retrieved", "claim": "signature criteria", "verified_by": "LITERATURE.md, 2026-10-31"},
            {"ref": "crypto:docs/ecbench/README.md", "provenance": "internal", "claim": "native measurement, isolation and comparison contract", "verified_by": "frontier review, 2026-10-31"},
        ],
        "amendments": [],
        "freeze": {"status": "draft", "protocol_sha256": None, "commit": None, "frozen_at": None, "filed_before_affected_data": None},
    }


if __name__ == "__main__":
    (ROOT / "protocol.json").write_text(json.dumps(protocol(), indent=2) + "\n")
