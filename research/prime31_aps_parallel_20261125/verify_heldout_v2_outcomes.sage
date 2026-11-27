#!/usr/bin/env sage -python
"""Independent arithmetic and work-ledger replay of a complete V2 raw run."""

import argparse
import base64
import hashlib
import json
from collections import Counter
from pathlib import Path

from sage.all import EllipticCurve, GF

ROOT = Path(__file__).resolve().parent
MANIFEST_SHA256 = "b9d5cb5dca1f46131616f5703284edfc769645ee48a5e187f929a1c3e187f64d"


def digest(raw):
    return hashlib.sha256(raw).hexdigest()


def load_registered():
    manifest_raw = (ROOT / "heldout_v2_seed_manifest.json").read_bytes()
    assert digest(manifest_raw) == MANIFEST_SHA256
    manifest = json.loads(manifest_raw)
    public_raw = (ROOT / "heldout_v2_public_targets.json").read_bytes()
    answers_raw = (ROOT / "heldout_v2_caller_answers.json").read_bytes()
    assert digest(public_raw) == manifest["public_targets_sha256"]
    assert digest(answers_raw) == manifest["caller_answers_sha256"]
    public = json.loads(public_raw)
    answers = json.loads(answers_raw)
    assert len(public) == len(answers["records"]) == 48
    return manifest, public, answers


def expected_calls(manifest, public):
    for stream in manifest["streams"]:
        case, replicate = stream["case"], stream["replicate"]
        for arm in stream["arm_order"]:
            yield case, replicate, arm, public[case], stream


def check_completed(record, q, stream, arm, ec, generator, scalar, n):
    request = {"public_Q": q}
    if arm == "queued_aps":
        request["aps_source_seed_hex"] = stream["aps_source_seed_hex"]
    else:
        request["rho_walk_seed_hex"] = stream["rho_walk_seed_hex"]
        request["rho_lane_restart_seed_hex"] = stream[
            "rho_lane_restart_seed_hex"]
    wire = (json.dumps(request, sort_keys=True, separators=(",", ":")) + "\n").encode()
    assert record["input"] == request
    assert record["input_sha256"] == digest(wire)
    for name in ("stdout", "stderr"):
        raw = base64.b64decode(record[name + "_base64"], validate=True)
        assert digest(raw) == record[name + "_sha256"]
        if name == "stdout" and record["native_report"] is not None:
            assert json.loads(raw) == record["native_report"]
    assert record["external_cold_wall_ns"] > 0
    assert record["child_cpu_ns"] >= 0
    assert record["peak_rss_bytes"] >= 0
    if record["exit_code"] != 0 or record["timed_out"]:
        return "timeout" if record["timed_out"] else "failed"
    report = record["native_report"]
    assert isinstance(report, dict)
    assert report["public_Q"] == q
    assert report["p"] == 2147483647 and report["n"] == n
    assert report["workers"] == 4
    assert report["known_answer_in_solver"] is False
    assert report["seed"] == (stream["aps_source_seed_hex"] if arm == "queued_aps"
                              else stream["rho_walk_seed_hex"])
    assert report["cold_wall_ns"] > 0 and report["process_cpu_ns"] >= 0
    if arm == "queued_aps":
        assert report["profile"] == [384, 384, 16384]
        queue = report["queue"]
        cells = queue["cells"]
        assert queue["capacity_cells"] == 8 and queue["chunk_rows"] == 16
        assert len(cells) == queue["generated_cells"] == queue["admitted_cells"] == 24
        assert queue["queued_cells"] == queue["dequeued_cells"] == 24
        assert queue["never_started_cells"] == 0
        assert [cell["id"] for cell in cells] == list(range(24))
        assert [cell["rows"] for cell in cells] == [[i * 16, (i + 1) * 16]
                                                   for i in range(24)]
        counts = Counter(cell["status"] for cell in cells)
        assert queue["cancelled_cells"] == sum(v for k, v in counts.items()
                                                 if k.startswith("cancelled"))
        assert queue["failed_cells"] == counts["error"]
        assert queue["completed_or_witness_cells"] == 24 - \
            queue["cancelled_cells"] - queue["failed_cells"]
        assert len(report["worker_reports"]) == 4
        assert sorted(w["worker"] for w in report["worker_reports"]) == list(range(4))
        assert sum(w["cells_received"] for w in report["worker_reports"]) == 24
        assert sum(w["s3_pairs"] for w in report["worker_reports"]) == \
            report["parallel_s3_pairs_including_speculation"]
        if queue["work_accounting_complete"]:
            assert not queue["errors"] and queue["failed_cells"] == 0
            assert sum(cell["s3_pairs"] for cell in cells) == \
                report["parallel_s3_pairs_including_speculation"]
        else:
            assert queue["errors"] and queue["failed_cells"] > 0
            assert report["complete_target_scalar_recovered"] is False
            return "native_error"
        solution = report["solution"]
        if report["complete_target_scalar_recovered"]:
            assert solution["status"] == "verified"
            assert report["parallel_s3_pairs_including_speculation"] > 0
            assert counts["witness"] >= 1
            found = solution["log"]
            indices = solution["partition_indices"]
            assert len(indices) == 3
            assert 0 <= indices[0] < 384 and 0 <= indices[1] < 384
            assert 0 <= indices[2] < 16384
            assert len(solution["signs"]) == 3
            assert all(sign in (-1, 1) for sign in solution["signs"])
            assert solution["beta"] % n != 0
            assert (solution["alpha"] + solution["beta"] * found) % n == 0
            assert solution["alpha"] * generator + solution["beta"] * ec(*q) == ec(0)
        else:
            assert solution["status"] == "no_relation_in_bounded_sample"
            return "exhausted"
    else:
        assert report["fold_negation"] is (arm == "folded_rho")
        assert report["shared_jump_count"] == 64
        assert report["total_initial_lanes"] == 64
        assert report["lanes_per_worker"] == 16
        assert report["dp_bits"] == 5 and report["step_cap"] == 2_000_000
        assert report["logical_lane_restart_seeds_explicit"] is True
        workers = report["worker_reports"]
        assert len(workers) == 4
        assert sorted(w["worker"] for w in workers) == list(range(4))
        assert sum(w["walk_steps"] for w in workers) == report["walk_steps"]
        assert all(w["stop_reason"] in ("verified_collision", "peer_solution",
                                        "step_cap") for w in workers)
        if report["complete_target_scalar_recovered"]:
            assert report["walk_steps"] > 0
            found = report["log"]
        else:
            assert report["log"] is None
            return "exhausted"
    assert found == scalar
    assert found * generator == ec(*q)
    return "verified_success"


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--records", type=Path, required=True)
    parser.add_argument("--admission", type=Path, required=True)
    args = parser.parse_args()
    manifest, public, answers = load_registered()
    admission = json.loads(args.admission.read_bytes())
    admission_hash = digest(args.admission.read_bytes())
    assert admission["manifest_sha256"] == MANIFEST_SHA256
    curve = answers["curve"]
    ec = EllipticCurve(GF(curve["p"]), [curve["a"], curve["b"]])
    generator = ec(*curve["g"])
    assert curve["n"] * generator == ec(0)
    calls = list(expected_calls(manifest, public))
    assert len(calls) == 720
    records = [json.loads(line) for line in args.records.open()]
    assert len(records) == 2 * len(calls)
    assert len({record["host"] for record in records}) == 1
    statuses = {arm: Counter() for arm in manifest["arms"]}
    for ordinal, (case, replicate, arm, q, stream) in enumerate(calls):
        start, outcome = records[2 * ordinal:2 * ordinal + 2]
        for event in (start, outcome):
            assert event["ordinal"] == ordinal
            assert event["case"] == case and event["replicate"] == replicate
            assert event["arm"] == arm and event["public_Q"] == q
            assert event["manifest_sha256"] == MANIFEST_SHA256
            assert event["admission_sha256"] == admission_hash
            assert event["binary_sha256"] == admission["binary_sha256"]
        assert start["event"] == "started"
        if outcome["event"] == "interrupted":
            status = "interrupted"
            assert outcome["native_report"] is None
        else:
            assert outcome["event"] == "completed"
            scalar = answers["records"][case]["caller_known_scalar_verifier_only"]
            status = check_completed(outcome, q, stream, arm, ec, generator,
                                     scalar, curve["n"])
        statuses[arm][status] += 1
    assert all(sum(counts.values()) == 240 for counts in statuses.values())
    print(json.dumps({"standing": "independent V2 raw outcome arithmetic replay",
                      "records_sha256": digest(args.records.read_bytes()),
                      "manifest_sha256": MANIFEST_SHA256,
                      "admission_sha256": admission_hash,
                      "calls": len(calls),
                      "status_counts": {arm: dict(counts)
                                        for arm, counts in statuses.items()}},
                     sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
