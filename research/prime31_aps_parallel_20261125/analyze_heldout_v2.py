#!/usr/bin/env python3
"""Frozen V2 checker and paired analysis; reads caller answers only after run."""

import argparse
import base64
import hashlib
import json
import math
import random
import statistics
from collections import defaultdict
from pathlib import Path

from run_heldout_v2 import ARMS, MANIFEST_SHA256, ROOT, planned_calls, sha256

BOOTSTRAP_DRAWS = 2000


def percentile(values, fraction):
    ordered = sorted(values)
    position = fraction * (len(ordered) - 1)
    lower = int(position)
    return ordered[lower] + (ordered[min(lower + 1, len(ordered) - 1)] -
                             ordered[lower]) * (position - lower)


def load_checked(path, manifest, public, answers, admission):
    calls = list(planned_calls(manifest, public))
    by_call = {}
    starts = {}
    admission_digest = sha256(admission)
    for line_number, line in enumerate(path.open(), 1):
        event = json.loads(line)
        ordinal = event.get("ordinal")
        if not isinstance(ordinal, int) or not 0 <= ordinal < len(calls):
            raise ValueError(f"line {line_number}: invalid ordinal")
        case, replicate, arm, q, stream = calls[ordinal]
        expected = {"case": case, "replicate": replicate, "arm": arm,
                    "public_Q": q, "manifest_sha256": MANIFEST_SHA256,
                    "admission_sha256": admission_digest,
                    "binary_sha256": json.loads(admission.read_bytes())["binary_sha256"]}
        if any(event.get(k) != v for k, v in expected.items()):
            raise ValueError(f"line {line_number}: frozen call identity mismatch")
        if event["event"] == "started":
            if ordinal in starts or ordinal in by_call or ordinal != len(starts):
                raise ValueError(f"line {line_number}: non-prefix or repeated start")
            starts[ordinal] = event
        elif event["event"] in ("completed", "interrupted"):
            if ordinal not in starts or ordinal in by_call:
                raise ValueError(f"line {line_number}: missing or repeated outcome")
            by_call[ordinal] = event
        else:
            raise ValueError(f"line {line_number}: unknown event")
    if len(starts) != len(calls) or len(by_call) != len(calls):
        raise ValueError(f"incomplete campaign: {len(starts)} starts, "
                         f"{len(by_call)} outcomes, {len(calls)} required")
    if any(answers["records"][i]["public_Q"] != public[i] or
           answers["records"][i]["case"] != i for i in range(48)):
        raise ValueError("caller answers and public Q disagree")
    return calls, by_call


def classify(calls, by_call, answers):
    classified = defaultdict(list)
    for ordinal, (case, replicate, arm, q, stream) in enumerate(calls):
        event = by_call[ordinal]
        report = event.get("native_report")
        scalar = answers["records"][case]["caller_known_scalar_verifier_only"]
        status = "interrupted"
        if event["event"] == "completed":
            request = {"public_Q": q}
            if arm == "queued_aps":
                request["aps_source_seed_hex"] = stream["aps_source_seed_hex"]
            else:
                request["rho_walk_seed_hex"] = stream["rho_walk_seed_hex"]
                request["rho_lane_restart_seed_hex"] = stream[
                    "rho_lane_restart_seed_hex"]
            payload = (json.dumps(request, sort_keys=True,
                                  separators=(",", ":")) + "\n").encode()
            if event.get("input") != request or event.get("input_sha256") != \
                    hashlib.sha256(payload).hexdigest():
                raise ValueError(f"call {ordinal}: native input stream mismatch")
            for name in ("stdout", "stderr"):
                try:
                    raw = base64.b64decode(event[name + "_base64"],
                                           validate=True)
                except (KeyError, ValueError) as error:
                    raise ValueError(f"call {ordinal}: invalid {name} bytes") from error
                if event.get(name + "_sha256") != hashlib.sha256(raw).hexdigest():
                    raise ValueError(f"call {ordinal}: {name} digest mismatch")
                if name == "stdout" and report is not None:
                    if json.loads(raw) != report:
                        raise ValueError(f"call {ordinal}: native report differs from stdout")
            status = "timeout" if event["timed_out"] else "failed"
            if event["exit_code"] == 0 and isinstance(report, dict):
                if report.get("public_Q") != q or report.get("workers") != 4 or \
                   report.get("known_answer_in_solver") is not False:
                    raise ValueError(f"call {ordinal}: native identity mismatch")
                expected_seed = stream["aps_source_seed_hex"] if arm == "queued_aps" \
                    else stream["rho_walk_seed_hex"]
                if report.get("seed") != expected_seed:
                    raise ValueError(f"call {ordinal}: seed mismatch")
                if arm == "queued_aps":
                    solution = report.get("solution") or {}
                    log = solution.get("log")
                    queue = report.get("queue") or {}
                    cells = queue.get("cells") or []
                    if queue.get("generated_cells") != 24 or \
                       queue.get("dequeued_cells") != 24 or len(cells) != 24 or \
                       sorted(cell.get("id") for cell in cells) != list(range(24)):
                        raise ValueError(f"call {ordinal}: incomplete queue accounting")
                    native_errors = queue.get("errors", [])
                    if native_errors:
                        status = "native_error"
                else:
                    log = report.get("log")
                    if report.get("logical_lane_restart_seeds_explicit") is not True or \
                       report.get("fold_negation") != (arm == "folded_rho"):
                        raise ValueError(f"call {ordinal}: rho configuration mismatch")
                recovered = report.get("complete_target_scalar_recovered")
                if recovered is True and log == scalar:
                    status = "verified_success"
                elif recovered is True or log is not None:
                    status = "wrong_answer"
                elif status != "native_error":
                    status = "exhausted"
        classified[arm].append({"case": case, "replicate": replicate,
            "ordinal": ordinal, "status": status,
            "wall_ns": event.get("external_cold_wall_ns"),
            "cpu_ns": event.get("child_cpu_ns"),
            "rss_bytes": event.get("peak_rss_bytes")})
    return classified


def paired_metric(classified, comparator, key, seed):
    aps = {(r["case"], r["replicate"]): r for r in classified["queued_aps"]}
    rho = {(r["case"], r["replicate"]): r for r in classified[comparator]}
    if any(r["status"] != "verified_success" or not r[key] for r in
           list(aps.values()) + list(rho.values())):
        return {"available": False, "reason": "success parity or complete measurements missing"}
    by_case = defaultdict(list)
    for pair, a in aps.items():
        r = rho[pair]
        by_case[pair[0]].append(math.log(r[key] / a[key]))
    observed = statistics.fmean(value for case in sorted(by_case)
                                for value in by_case[case])
    rng = random.Random(seed)
    cases = sorted(by_case)
    draws = []
    for _ in range(BOOTSTRAP_DRAWS):
        sample = [cases[rng.randrange(len(cases))] for _ in cases]
        draws.append(math.exp(statistics.fmean(value for case in sample
                                               for value in by_case[case])))
    return {"available": True, "rho_over_aps_geometric_ratio": math.exp(observed),
            "case_cluster_bootstrap_95pct": [percentile(draws, .025),
                                              percentile(draws, .975)],
            "pairs": len(aps), "case_clusters": len(cases),
            "bootstrap_draws": BOOTSTRAP_DRAWS}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--records", type=Path, required=True)
    parser.add_argument("--admission", type=Path, required=True)
    args = parser.parse_args()
    manifest_path = ROOT / "heldout_v2_seed_manifest.json"
    if sha256(manifest_path) != MANIFEST_SHA256:
        raise ValueError("manifest digest mismatch")
    manifest = json.loads(manifest_path.read_bytes())
    public_path = ROOT / "heldout_v2_public_targets.json"
    answers_path = ROOT / "heldout_v2_caller_answers.json"
    if sha256(public_path) != manifest["public_targets_sha256"] or \
       sha256(answers_path) != manifest["caller_answers_sha256"]:
        raise ValueError("input digest mismatch")
    public = json.loads(public_path.read_bytes())
    answers = json.loads(answers_path.read_bytes())
    calls, by_call = load_checked(args.records, manifest, public, answers,
                                  args.admission)
    classified = classify(calls, by_call, answers)
    seed = int(manifest["analysis_bootstrap_seed_hex"], 16)
    summary = {arm: {"status_counts": {status: sum(r["status"] == status
                   for r in rows) for status in sorted({r["status"] for r in rows})},
               "total_wall_ns": sum(r["wall_ns"] or 0 for r in rows),
               "total_cpu_ns": sum(r["cpu_ns"] or 0 for r in rows),
               "peak_rss_bytes": max((r["rss_bytes"] or 0 for r in rows),
                                     default=0)} for arm, rows in classified.items()}
    comparisons = {arm: {metric: paired_metric(classified, arm, metric,
                   seed ^ (1 if arm == "folded_rho" else 2) ^
                   (4 if metric == "cpu_ns" else 0))
                   for metric in ("wall_ns", "cpu_ns")}
                   for arm in ("folded_rho", "unfolded_rho")}
    print(json.dumps({"standing": "[pcc:unverified]", "records_sha256":
        sha256(args.records), "manifest_sha256": MANIFEST_SHA256,
        "admission_sha256": sha256(args.admission), "calls": len(calls),
        "summary": summary, "paired": comparisons}, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
