#!/usr/bin/env python3
"""Independent custody/join/selection checks; does not implement SAT reasoning."""
import argparse
from pathlib import Path
from common import artifacts, canonical, config, digest_bytes, jsonl, load, require, sha, write

FALSE_FLAGS = ("native_execution_trace_verified", "native_instruction_count_verified",
               "unsat_proof_checked")
INPUT_FILES = ["features.jsonl", "prefix-features.jsonl", "algebraic-features.jsonl",
               "sat-features.jsonl"]

REPLAY_LIMITS = {"raw_events": 100000, "raw_bytes": 16 << 20, "decisions": 4096,
                 "derived_bytes": 64 << 20, "operations": 10000000,
                 "retained_bytes": 64 << 20, "variables": 100000, "source_bytes": 64 << 20}
SUMMARY_KEYS = {"verified_decisions", "unknown_suffix_records", "patterns_admitted", "work", "limits", *FALSE_FLAGS}
WORK_KEYS = {"literal_visits", "parity_word_operations", "raw_events", "raw_bytes",
             "derived_bytes", "retained_logical_peak_bytes", "source_payload_logical_bytes"}


def projected_static(public, base):
    def point(value):
        return None if value is None else {k: value[k] for k in ("x", "y")}
    field = {k: public["field"][k] for k in ("characteristic", "degree", "basis")}
    for key in ("modulus", "irreducible_low_terms"):
        if key in public["field"]:
            field[key] = public["field"][key]
    return {"pdp_domain": "cyclic-subgroup-factor-base",
            **{k: public[k] for k in ("family", "n", "m", "twist")}, "field": field,
            "curve": {k: public["curve"][k] for k in ("model", "a", "b", "group_order", "subgroup_order", "cofactor")},
            "generator": point(public["generator"]), "target": point(public["target"]),
            "symmetry": {k: public["symmetry"][k] for k in ("coefficient_field_degree", "frobenius_power", "frobenius_nontrivial_on_model", "negation")},
            "factor_base": {"rule": base["rule"], "points": [point(p) for p in base["points"]],
                **{k: base[k] for k in ("negation_action", "frobenius_action", "frobenius_power")},
                "orbits": [{"points": [point(p) for p in orbit["points"]], "closed": orbit["closed"],
                            "stabilizer_size": orbit["stabilizer_size"]} for orbit in base["orbits"]]}}


def gate_shape(gate):
    require(set(gate) == {"schema", "n", "m", "irreducible_low_terms", "b", "target_x", "identity_mask", "variables",
                         "coordinate_blocks", "and_gates", "xor_rows", "unit_literals", "impossible", "finite_domain",
                         "reduction_outputs", "square_inputs"}, "gate map contains nonallowlisted fields")
    require(gate["schema"] == "crypto.binary-semaev-chain-gate-map/v1" and gate["n"] == 7 and gate["m"] in (2, 3), "gate schema/dimensions")
    nvars = gate["variables"]
    wire = lambda value: type(value) is int and 0 < value <= nvars
    require(type(nvars) is int and 0 < nvars <= 100000 and type(gate["impossible"]) is bool, "gate scalar types")
    for block in gate["coordinate_blocks"]:
        require(len(block) == 7 and all(wire(v) for v in block), "coordinate wire block")
    for triple in gate["and_gates"]:
        require(len(triple) == 3 and all(wire(v) for v in triple), "AND gate shape")
    for row in gate["xor_rows"]:
        require(set(row) == {"variables", "rhs"} and type(row["rhs"]) is bool and
                all(wire(v) for v in row["variables"]), "raw XOR gate shape")
    require(all(type(v) is int and v != 0 and abs(v) <= nvars for v in gate["unit_literals"]), "gate unit shape")
    for name in ("reduction_outputs", "square_inputs"):
        require(all(isinstance(row, list) and all(type(v) is int and v >= 0 for v in row)
                    for row in gate[name]), "linear field map shape")
    require(all(type(v) is int and 0 <= v < 7 for v in gate["irreducible_low_terms"]), "field exponents")
    domain = gate["finite_domain"]
    require(isinstance(domain, dict) and set(domain) == {"codes", "raw_clause_count"} and
            type(domain["raw_clause_count"]) is int and domain["raw_clause_count"] >= 0, "domain shape")
    for value in [gate["b"], *domain["codes"], *([] if gate["target_x"] is None else [gate["target_x"]])]:
        require(type(value) is str and value.isascii() and value.isdigit() and
                str(int(value)) == value and 0 <= int(value) < 128, "field code shape")


def expected_split():
    component = "field-size-n7"
    bucket = int(digest_bytes(("ml-dataset-whole-field-size-split-v1\n" + component).encode())[:16], 16) % 100
    return component, "train" if bucket < 80 else "validation" if bucket < 90 else "test"


def canonical_clause(values):
    require(all(type(v) is int and v != 0 for v in values), "invalid clause literal")
    return sorted(values, key=lambda value: (abs(value), value))


def cell_directory(root, index):
    return Path(root) / "cells" / f"{index:012d}"


def read_events(cell):
    path = cell / "sat-native-events.jsonl"
    if not path.exists():
        return []
    events = jsonl(path, 16 * 1024**2)
    require(len(events) <= 100000, "native event count bound")
    for sequence, event in enumerate(events, 1):
        require(event["schema"] == "ml-dataset.sat-event/v1" and event["sequence"] == sequence,
                "native stream schema/sequence mismatch")
    return events


def by_event(events, sequence, pattern, kind):
    require(type(sequence) is int and 1 <= sequence <= len(events), "invalid event join")
    event = events[sequence - 1]
    require(event["pattern"] == pattern and event["native"]["kind"] == kind,
            "event join pattern/kind mismatch")
    return event["native"]


def transition(events, decisions, features, unknown, source_hashes):
    for event in events:
        learned = event["native"]
        if learned["kind"] != "learned":
            continue
        p, seq = event["pattern"], event["sequence"]
        before = [d for d in decisions if d["pattern"] == p and d["assignment_sequence"] < seq]
        jumps = [e for e in events[seq:] if e["pattern"] == p and
                 e["native"]["kind"] == "backjump"]
        if not before or not jumps:
            continue
        jump = jumps[0]
        if jump["native"]["to_level"] != learned["backjump_level"]:
            continue
        clause = canonical_clause(learned["literals"])
        after = [d for d in decisions if d["pattern"] == p and
                 d["decision_sequence"] > jump["sequence"] and
                 clause in features[d["feature_id"]]["inputs"]["active_state"]["clauses"]]
        if not after:
            continue
        pre, post = max(before, key=lambda d: d["decision_sequence"]), min(
            after, key=lambda d: d["decision_sequence"])
        if any(u.get("pattern") in (None, p) and
               (u.get("event_sequence") if u.get("event_sequence") is not None else u["work"]["raw_events"] + 1) <= post["decision_sequence"]
               for u in unknown):
            continue
        require(pre["source_target_id"] == post["source_target_id"], "transition crosses source")
        return {"pattern": p, "decision_before": pre["decision_sequence"],
                "assignment_before": pre["assignment_sequence"], "learned": seq,
                "backjump": jump["sequence"], "decision_after": post["decision_sequence"],
                "assignment_after": post["assignment_sequence"],
                "feature_before": pre["feature_id"], "feature_after": post["feature_id"],
                "learned_clause_id": learned["clause_id"], "learned_literals": learned["literals"],
                "backjump_level": learned["backjump_level"], "source_hashes": source_hashes,
                "source_target_id": pre["source_target_id"],
                "entailment_scope": "later native export admission necessarily passed bounded RUP/parity for this preceding Learned; no independent Python SAT engine or standalone LRAT proof"}
    return None


def inspect(root, destination, index):
    root, destination = Path(root), Path(destination)
    cell = cell_directory(root, index)
    source = load(cell / "manifest.json")
    require(source["schema"] == "crypto.ml-dataset.cell/v1" and source["cell_index"] == index,
            "wrong cell manifest")
    require(source["config"] == config(index), "cell configuration differs from fixed table")
    source_artifacts = artifacts(cell, source)
    run = load(root / "run.json")
    require(run["schema"] == "crypto.ml-dataset.run/v1" and run["shard_index"] == index and run["shard_count"] == 16 and
            run["seconds_per_case"] == 30 and run["memory_mib"] == 2048 and run["trace_mib"] == 64 and
            run["plan_sha256"] == source["plan_sha256"], "wrong run shard/limits/plan")
    require(load(cell / "run.json") == run, "cell/root run binding mismatch")
    target_id = digest_bytes(f"{sha(root / 'run.json')}\n{sha(cell / 'manifest.json')}\n{index}".encode())
    manifest = load(destination / "manifest.json")
    require(manifest["schema"] == "ml-dataset.training-export/v1", "export schema")
    require(manifest["feature_policy"]["version"] == "strict-structural-allowlist-v7" and
            manifest["feature_policy"]["model_input_files"] == INPUT_FILES, "export allowlist")
    require(manifest["source_run_sha256"] == sha(root / "run.json") and
            manifest["source_commit"] == run["source_commit"] and
            manifest["binary_sha256"] == run["binary_sha256"], "export source binding")
    export_artifacts = artifacts(destination, manifest)
    require(all(name in export_artifacts for name in INPUT_FILES + ["restricted/sat-records.jsonl"]),
            "missing model-input/restricted artifact")
    component, split = expected_split()
    all_features = {}
    for name in INPUT_FILES:
        rows = jsonl(destination / name)
        for row in rows:
            require(row["feature_id"] == digest_bytes(canonical(row["inputs"])), "feature identity")
            require(row["split_component"] == component and row["split"] == split, "independent split mismatch")
        all_features[name] = rows
    features = {row["feature_id"]: row for row in all_features["sat-features.jsonl"]}
    require(len(features) == len(all_features["sat-features.jsonl"]), "duplicate public feature ID")
    static = [row["inputs"] for row in all_features["features.jsonl"]]
    expected_static = projected_static(load(cell / "public.json"), load(cell / "factor-base.json"))
    require(static == [expected_static], "static structural projection differs from independently selected source fields")
    require(not all_features["prefix-features.jsonl"] and not all_features["algebraic-features.jsonl"] and
            not jsonl(destination / "restricted/prefix-records.jsonl") and
            not jsonl(destination / "restricted/algebraic-records.jsonl"), "SAT fixture leaked unrelated prefix channels")
    targets, indices = jsonl(destination / "restricted/targets.jsonl"), jsonl(destination / "restricted/index.jsonl")
    require(len(targets) == 1 and len(indices) == 1 and manifest["sealed_source_cells"] == 1 and
            manifest["restricted_target_rows"] == 1 and manifest["unique_feature_rows"] == 1 and
            manifest["feature_occurrences"] == 1, "static target/index counts")
    target, source_index = targets[0], indices[0]
    static_id = all_features["features.jsonl"][0]["feature_id"]
    require(target["schema"] == "ml-dataset.restricted-targets/v1" and target["target_id"] == target_id and
            target["feature_id"] == static_id and target["split_component"] == component and target["split"] == split and
            target["solver"] == "sat" and target["labels"] == source["outcome"], "target/static join")
    require(source_index["schema"] == "ml-dataset.export-index/v1" and source_index["record_id"] == target_id and
            source_index["target_id"] == target_id and source_index["feature_id"] == static_id and
            source_index["source_manifest_sha256"] == sha(cell / "manifest.json") and
            source_index["source_run_sha256"] == sha(root / "run.json") and
            source_index["source_artifacts"] == source["artifacts"] and
            source_index["status"] == "verified_sealed_cell", "restricted index custody join")
    for row in features.values():
        inputs = row["inputs"]
        gate_shape(inputs["gate_map"])
        require(set(inputs) == {"task", "initial", "gate_map", "active_state"} and
                inputs["task"] == "binary-sat-decision/v1", "SAT input projection")
        require(inputs["initial"] in static, "SAT initial projection differs from static input")
        require(set(inputs["active_state"]) == {"clauses", "xor_rows", "xor_matrix", "trail",
                                               "decision_boundaries"}, "SAT state projection")
        for group in ("xor_rows", "xor_matrix"):
            for parity in inputs["active_state"][group]:
                require(set(parity) == {"variables", "rhs"} and type(parity["rhs"]) is bool,
                        "parity must use variable lists, not wide JSON masks")
                require(all(type(v) is int and 1 <= v <= inputs["gate_map"]["variables"]
                            for v in parity["variables"]), "parity wire range")
        for entry in inputs["active_state"]["trail"]:
            require(set(entry) == {"literal", "level"}, "trail leakage")
    events = read_events(cell)
    maps = {}
    source_hashes = {name: item["sha256"] for name, item in source_artifacts.items()
                     if name.startswith("sat-")}
    for event in events:
        p = event["pattern"]
        if p in maps:
            continue
        map_name, formula_name = f"sat-pattern-{p}.gate-map.json", f"sat-pattern-{p}.xor.cnf"
        if map_name not in source_artifacts:
            continue  # Explicit unavailable channel is allowed to fail selection.
        require(source_artifacts[map_name]["role"] == "native-binary-s3-gate-map" and
                source_artifacts[map_name]["encoding"] == "json", "map role/encoding")
        require(formula_name in source_artifacts and
                source_artifacts[formula_name]["role"] == "complete-original-cnf-xor" and
                source_artifacts[formula_name]["encoding"] == "dimacs-xor", "formula role/encoding")
        maps[p] = load(cell / map_name, 16 * 1024**2)
    records = jsonl(destination / "restricted/sat-records.jsonl")
    decisions, unknown, summaries, used = [], [], [], set()
    record_ids, occurrence_ids, decision_events, assignment_events = set(), set(), set(), set()
    for outer in records:
        require(outer["schema"] == "ml-dataset.restricted-sat-record/v1" and
                outer["source_target_id"] == target_id, "restricted source join")
        require(outer["record_id"] not in record_ids, "duplicate restricted record ID")
        record_ids.add(outer["record_id"])
        require(outer["split_component"] == component and outer["split"] == split, "restricted split mismatch")
        record = outer["record"]
        require(all(record.get(flag) is False for flag in FALSE_FLAGS), "proof scope flag")
        status = record.get("admission_status")
        if status == "verified_decision":
            require(record["kind"] == "sat_decision", "decision record kind")
            feature = features.get(outer["feature_id"])
            require(feature is not None, "missing feature join")
            require(feature["split"] == outer["split"] and
                    feature["split_component"] == outer["split_component"], "split join")
            require(feature["inputs"]["gate_map"] == maps.get(record["pattern"]), "feature gate map join")
            require(outer["source_event_artifact"] == source_artifacts["sat-native-events.jsonl"],
                    "raw event artifact join")
            decision = by_event(events, record["event_sequence"], record["pattern"], "decision")
            assignment = by_event(events, record["assignment_event_sequence"], record["pattern"], "assignment")
            require(record["assignment_event_sequence"] == record["event_sequence"] + 1 and
                    assignment["literal"] == decision["literal"] == record["chosen_literal"] and
                    assignment["level"] == decision["level"] == record["level"] and
                    assignment["reason"] == {"kind": "decision"}, "decision/action provenance")
            require(record["activity_bits"] == str(decision["activity_bits"]) and
                    record["priority"] == decision["priority"], "restricted activity provenance")
            trail = feature["inputs"]["active_state"]["trail"]
            require(len(trail) == decision["trail_index"] and
                    all(abs(entry["literal"]) != abs(decision["literal"]) for entry in trail),
                    "context includes its own action")
            require(len(feature["inputs"]["active_state"]["decision_boundaries"]) + 1 == decision["level"],
                    "context includes the new decision boundary")
            decision_key = (record["pattern"], record["event_sequence"])
            assignment_key = (record["pattern"], record["assignment_event_sequence"])
            require(decision_key not in decision_events and assignment_key not in assignment_events, "duplicate admitted event join")
            decision_events.add(decision_key)
            assignment_events.add(assignment_key)
            occurrence_ids.add(outer["record_id"])
            used.add(outer["feature_id"])
            decisions.append({"pattern": record["pattern"], "decision_sequence": record["event_sequence"],
                              "assignment_sequence": record["assignment_event_sequence"],
                              "feature_id": outer["feature_id"], "source_target_id": target_id})
        elif status == "bounded_unknown_suffix":
            require(record["kind"] == "sat_unknown_suffix" and outer["feature_id"] is None,
                    "unknown record has public supervision")
            occurrence_ids.add(outer["record_id"])
            unknown.append(record)
        else:
            require(outer["feature_id"] is None and outer["record_id"] == digest_bytes((target_id + "\nsat-replay-summary").encode()),
                    "invalid summary identity")
            if status is None:
                require(set(record) == SUMMARY_KEYS and record["limits"] == REPLAY_LIMITS and set(record["work"]) == WORK_KEYS,
                        "unknown normal summary shape or inflated replay limits")
            else:
                require(status == "unavailable" and record.get("schema") == "ml-dataset.sat-prefix-replay/v1" and
                        record.get("reason") in ("no_retained_sat_events", "no_retained_sat_gate_maps", "sat_source_artifact_budget_exhausted") and
                        set(record) == {"schema", "admission_status", "reason", "verified_decisions", *FALSE_FLAGS} and
                        record["verified_decisions"] == 0, "unrecognized unavailable summary")
            summaries.append(record)
    require(len(summaries) == 1, "one-cell SAT fixture must have exactly one replay summary")
    require(occurrence_ids == {digest_bytes(f"{target_id}\nsat-record\n{i}".encode())
                               for i in range(1, len(decisions) + len(unknown) + 1)}, "restricted occurrence identity set")
    require(used == set(features), "orphan public SAT feature")
    require(manifest["sat_feature_occurrences"] == len(decisions) and
            manifest["unique_sat_feature_rows"] == len(features) and
            manifest["restricted_sat_record_rows"] == len(records), "SAT manifest counts")
    outcomes = manifest["sat_outcomes"]
    require(outcomes["verified_decision_occurrences"] == len(decisions) and
            outcomes["unknown_suffix_records"] == len(unknown) and
            outcomes["summary_rows"] == len(summaries), "SAT outcome counts")
    require(sum(s["verified_decisions"] for s in summaries) == len(decisions) and
            sum(s.get("unknown_suffix_records", 0) for s in summaries) == len(unknown), "replay summary counts")
    require(outcomes["cells_without_sat_channel"] == sum(
        s.get("admission_status") == "unavailable" for s in summaries), "unavailable count")
    for summary in summaries:
        if "work" in summary:
            work, limits = summary["work"], summary["limits"]
            require(work["literal_visits"] + work["parity_word_operations"] <= limits["operations"] and
                    work["raw_events"] <= limits["raw_events"] and work["raw_bytes"] <= limits["raw_bytes"] and
                    work["derived_bytes"] <= limits["derived_bytes"] and
                    work["retained_logical_peak_bytes"] <= limits["retained_bytes"], "replay budget totals")
    witness = transition(events, decisions, features, unknown, source_hashes)
    return {"schema": "ml-dataset.sat-cli-cell-audit/v1", "cell_index": index,
            "config": source["config"], "source_manifest_sha256": sha(cell / "manifest.json"),
            "export_manifest_sha256": sha(destination / "manifest.json"),
            "all_manifest_artifacts_hash_checked": True, "joins_and_feature_ids_checked": True,
            "verified_decisions": len(decisions), "unique_features": len(features),
            "unknown_suffix_records": len(unknown), "unknown_suffixes": unknown,
            "replay_summaries": summaries, "decision_index": decisions,
            "transition_witness": witness, "source_outcome": source["outcome"],
            "native_attempts": load(cell / "sat-attempts.json") if (cell / "sat-attempts.json").exists() else None,
            "acceptance_scope": "independent byte/join/causal-shape checks around native semantic replay; no second cryptographic or SAT proof engine",
            "scientific_samples_added": 0, "verified_cloud_dataset_cells": 0}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source", required=True)
    parser.add_argument("--export", required=True)
    parser.add_argument("--cell", type=int, required=True)
    parser.add_argument("--out", required=True)
    args = parser.parse_args()
    write(args.out, inspect(args.source, args.export, args.cell), exclusive=True)


if __name__ == "__main__":
    main()
