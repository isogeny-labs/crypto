"""Independent custody, shape, cap and original-field D18 symmetry replay."""

import argparse
import copy
import importlib.util
import json
from pathlib import Path

import run

HERE = Path(__file__).resolve().parent
PRIOR = HERE.parent / "local-extended-natural-20261025" / "audit.py"
SPEC = importlib.util.spec_from_file_location("n83_prior_group_replay", PRIOR)
prior = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(prior)


def read(path):
    return json.loads(path.read_text())


def check_arm(output, mode, identity, obj, group):
    path = output / mode
    positive = mode == "positive"
    supervisor = read(path / "supervisor.json")
    command = read(path / "command.json")
    if (supervisor["source_commit"] != identity["source_commit"]
            or supervisor["binary_sha256"] != identity["binary_sha256"]
            or supervisor["object_sha256"] != run.witness_run.OBJECT_SHA
            or supervisor["outer_cap_seconds"] != (310 if positive else 2_410)
            or supervisor["selected_best_total_runtime"] is not None
            or supervisor["isolation"] != "L0_shared_host_no_hard_cgroup"
            or command[:2] != ["/usr/bin/time", "-l"]
            or command[3:5] != ["primary-full-d18-m4-l0-ordered-first-pair", mode]):
        raise ValueError(f"{mode} supervisor or command identity differs")
    phases = [json.loads(line) for line in
              (path / "result.json.phases.jsonl").read_text().splitlines()]
    names = [item["phase"] for item in phases]
    if (names[:3] != ["started", "import_checked", "model_constructed"]
            or phases[0]["detail"]["source_commit"] != identity["source_commit"]
            or phases[0]["detail"]["binary_sha256"] != identity["binary_sha256"]):
        raise ValueError(f"{mode} phase provenance differs")
    shape = read(path / "result.json.shape.json")
    if (shape != phases[2]["detail"]
            or shape["variables"] != 329_808
            or shape["clauses"] != (2_807_999 if positive else 2_807_501)
            or shape["xor_rows"] != 3_901
            or shape["root_encoding"] != "shared_root_bits"
            or shape["choice_pin_policy"] != "none"
            or shape["ordered_first_pair"] is not True
            or shape["search_policy"] != "phase_zero_first_ordered_first_pair"):
        raise ValueError(f"{mode} SAT shape differs")
    result_path = path / "result.json"
    if not result_path.exists():
        cap_path = path / "result.json.cap.json"
        if (not positive and not supervisor["outer_cap_reached"]
                and supervisor["exit_code"] == 124 and cap_path.exists()):
            cap = read(cap_path)
            if (cap["schema"] != "n83.full-d18-m4-cap/v1"
                    or cap["status"] != "UNKNOWN_wall_cap"
                    or cap["mode"] != mode
                    or cap["budget_seconds"] != 2_400
                    or cap["root_encoding"] != "shared_root_bits"
                    or cap["root_choice_pinned"] is not False
                    or cap["choice_pin_policy"] != "none"
                    or cap["search_policy"] != "phase_zero_first_ordered_first_pair"
                    or cap["isolation"] != "L0_shared_host_no_hard_cgroup"
                    or cap["elapsed_process_wall_ms"] < 2_400_000
                    or cap["selected_best_total_runtime"] is not None
                    or names != ["started", "import_checked", "model_constructed"]):
                raise ValueError("internal wall-cap receipt differs")
            return {"status": "UNKNOWN_wall_cap", "phases": names,
                    "import_ms": shape["import_ms"], "model_ms": shape["model_ms"],
                    "search_ms": None,
                    "supervisor_wall_seconds": supervisor["elapsed_wall_seconds"],
                    "maximum_resident_set_bytes": supervisor["maximum_resident_set_bytes"],
                    "relation": None, "cap_receipt": cap}
        if positive or not supervisor["outer_cap_reached"] or supervisor["exit_code"] != -9:
            raise ValueError(f"{mode} missing result has no verified watchdog")
        return {"status": "OUTER_WATCHDOG_CENSORED",
                "phases": names, "import_ms": shape["import_ms"],
                "model_ms": shape["model_ms"], "search_ms": None,
                "supervisor_wall_seconds": supervisor["elapsed_wall_seconds"],
                "maximum_resident_set_bytes": supervisor["maximum_resident_set_bytes"],
                "relation": None}
    result = read(result_path)
    if (supervisor["exit_code"] != 0 or supervisor["outer_cap_reached"]
            or names != ["started", "import_checked", "model_constructed", "solver_returned"]
            or result["source_commit"] != identity["source_commit"]
            or result["binary_sha256"] != identity["binary_sha256"]
            or result["source_attestation_mode"] != "clean_checkout"
            or result["mode"] != mode
            or result["budget_seconds"] != (300 if positive else 2_400)
            or result["shape"] != shape
            or result["selected_best_total_runtime"] is not None):
        raise ValueError(f"{mode} worker or terminal receipt differs")
    relation = None
    if positive:
        if (result["status"] != "PASS_CONSTRUCTED_CONTROL"
                or result["solver_answer"] != "Sat"
                or result["relation"]["matches_planted_row"] is not True):
            raise ValueError("ordered constructed control failed")
        relation = prior.replay_relation(result, obj, group)
    elif result["status"] == "PENDING_INDEPENDENT_NATURAL_REPLAY":
        relation = prior.replay_relation(result, obj, group)
    elif result["status"] not in ("UNKNOWN_search_cap", "UNVERIFIED_UNSAT",
                                   "MODEL_GROUP_REJECTED", "INVALID_MODEL"):
        raise ValueError("unexpected natural worker status")
    return {"status": result["status"], "phases": names,
            "import_ms": shape["import_ms"], "model_ms": shape["model_ms"],
            "search_ms": result["search_ms"],
            "solver_conflicts": result["solver_conflicts"],
            "supervisor_wall_seconds": supervisor["elapsed_wall_seconds"],
            "maximum_resident_set_bytes": supervisor["maximum_resident_set_bytes"],
            "relation": relation}


def audit(source, binary, base, output):
    source, binary, base, output = map(Path, (source, binary, base, output))
    actual = run.witness_run.preflight(source, binary, base)
    actual.pop("expected_xcnf_sha256")
    actual["schema"] = "n83.d18-ordered-first-pair-identity/v1"
    actual["protocol"] = "positive-300-310_then_natural-2400-2410"
    identity = read(output / "identity.json")
    if identity != actual:
        raise ValueError("source, binary, S3 or host preflight differs")
    obj = base / "objects" / run.witness_run.OBJECT
    group = prior.group_module()
    positive = check_arm(output, "positive", identity, obj, group)
    result = read(output / "positive/result.json")
    negative = {}
    for name in ("changed_target", "changed_row_digest"):
        altered = copy.deepcopy(result)
        if name == "changed_target":
            altered["target"][0] = str(int(altered["target"][0]) ^ 1)
        else:
            altered["relation"]["row_decimal_blake3"] = "0" * 64
        try:
            prior.replay_relation(altered, obj, group)
        except AssertionError:
            negative[name] = "REJECTED"
        else:
            raise ValueError(f"{name} control was accepted")
    natural = (check_arm(output, "natural", identity, obj, group)
               if (output / "natural").exists() else None)
    summary = read(output / "summary.json")
    legacy_wall_cap_misclassified = (natural is not None
                                     and natural["status"] == "UNKNOWN_wall_cap"
                                     and summary["natural_status"] == "PRODUCER_FAILURE")
    if (summary["source_commit"] != identity["source_commit"]
            or summary["binary_sha256"] != identity["binary_sha256"]
            or summary["positive_status"] != "PASS_CONSTRUCTED_CONTROL"
            or summary["natural_search_executed"] != (natural is not None)
            or (summary["natural_status"] != (None if natural is None else natural["status"])
                and not legacy_wall_cap_misclassified)
            or summary["selected_best_total_runtime"] is not None):
        raise ValueError("run summary differs from independently checked arms")
    status = ("PASS_CONSTRUCTED_ONLY" if natural is None else
              "PASS_REPLAYED_NATURAL_RELATION" if natural["relation"] else
              "PASS_CENSORED_NATURAL" if natural["status"] in
              ("UNKNOWN_search_cap", "UNKNOWN_wall_cap", "OUTER_WATCHDOG_CENSORED") else
              "PASS_RECORDED_NONRELATION_TERMINAL")
    return {"schema": "n83.d18-ordered-first-pair-independent-audit/v1",
            "status": status, "source_commit": identity["source_commit"],
            "binary_sha256": identity["binary_sha256"],
            "object_sha256": run.witness_run.OBJECT_SHA,
            "positive": positive, "natural": natural,
            "reported_natural_status": summary["natural_status"],
            "legacy_wall_cap_misclassified": legacy_wall_cap_misclassified,
            "negative_controls": negative,
            "selected_best_total_runtime": None}


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    for name in ("source", "binary", "base", "output"):
        parser.add_argument("--" + name, required=True)
    args = parser.parse_args()
    print(json.dumps(audit(args.source, args.binary, args.base, args.output),
                     sort_keys=True, indent=2))
