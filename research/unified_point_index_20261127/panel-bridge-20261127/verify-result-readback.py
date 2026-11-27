#!/usr/bin/env python3
"""Verify a separately downloaded native-panel result tree. Never start a solve."""

import argparse
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import re
import subprocess
import sys


def fail(message):
    raise ValueError(message)


def sha(path):
    h = hashlib.sha256()
    with open(path, "rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def unique_pairs(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            fail(f"duplicate JSON key: {key}")
        result[key] = value
    return result


def read_json(path):
    if path.is_symlink() or not path.is_file() or path.stat().st_size > 32 * 1024 * 1024:
        fail(f"missing, symlinked or oversized JSON: {path}")
    return json.loads(path.read_text(), object_pairs_hook=unique_pairs)


def check_result_tree(raw):
    if raw.is_symlink() or not raw.is_dir():
        fail("missing or symlinked result tree")
    files = {}
    total = 0
    for directory, dirs, names in os.walk(raw, followlinks=False):
        for name in dirs + names:
            path = Path(directory, name)
            if path.is_symlink():
                fail(f"symlink in result tree: {path}")
        for name in names:
            path = Path(directory, name)
            if not path.is_file():
                fail(f"non-file result entry: {path}")
            relative = path.relative_to(raw).as_posix()
            files[relative] = path
            total += path.stat().st_size
            if len(files) > 10000 or total > 1024 * 1024 * 1024:
                fail("result tree exceeds size or file-count bound")
    checksum = files.get("SHA256SUMS")
    if checksum is None:
        fail("missing result checksum list")
    listed = {}
    for line in checksum.read_text().splitlines():
        match = re.fullmatch(r"([a-f0-9]{64})  ([^\n\r]+)", line)
        if match is None:
            fail("malformed result checksum line")
        digest, relative = match.groups()
        parsed = PurePosixPath(relative)
        if parsed.is_absolute() or ".." in parsed.parts or relative.startswith("./"):
            fail("unsafe result checksum path")
        if relative in listed:
            fail("duplicate result checksum path")
        listed[relative] = digest
    if set(listed) != set(files) - {"SHA256SUMS"}:
        fail("result checksum list omits or adds a file")
    for relative, digest in listed.items():
        if sha(files[relative]) != digest:
            fail(f"changed result file: {relative}")
    return files


def check_public_inputs(inputs, hash_list):
    if inputs.is_symlink() or not inputs.is_dir():
        fail("missing or symlinked public input tree")
    listed = {}
    for line in hash_list.read_text().splitlines():
        match = re.fullmatch(r"([a-f0-9]{64})  (manifest[.]json|seed-manifest[.]json|problems/[a-zA-Z0-9-]+[.]json)", line)
        if match is None:
            fail("invalid public input hash entry")
        digest, relative = match.groups()
        if relative in listed:
            fail("duplicate public input hash entry")
        listed[relative] = digest
    if len(listed) != 86 or "manifest.json" not in listed or "seed-manifest.json" not in listed:
        fail("incomplete public input hash list")
    actual = {}
    for directory, dirs, names in os.walk(inputs, followlinks=False):
        for name in dirs + names:
            path = Path(directory, name)
            if path.is_symlink():
                fail("symlink in public input tree")
        for name in names:
            path = Path(directory, name)
            actual[path.relative_to(inputs).as_posix()] = path
    if set(actual) != set(listed):
        fail("public input tree differs from hash list")
    for relative, digest in listed.items():
        if sha(actual[relative]) != digest:
            fail(f"changed public input: {relative}")


def point(pair):
    return None if pair is None else (int(pair[0]), int(pair[1]))


def add(left, right, p, a):
    if left is None:
        return right
    if right is None:
        return left
    x, y = left
    u, v = right
    if x == u and (y + v) % p == 0:
        return None
    if x == u:
        numerator, denominator = (3 * x * x + a) % p, (2 * y) % p
    else:
        numerator, denominator = (v - y) % p, (u - x) % p
    slope = numerator * pow(denominator, -1, p) % p
    rx = (slope * slope - x - u) % p
    return rx, (slope * (x - rx) - y) % p


def check_scalar(problem, expected):
    p, a, b, r = (int(problem[name]) for name in ("p", "a", "b", "r"))
    if p not in (5, 7, 17) or not 0 < expected < r or int(problem["cofactor"]) != 1:
        fail("scalar or toy curve outside frozen panel")
    if (4 * a**3 + 27 * b**2) % p == 0:
        fail("singular public toy curve")
    curve_points = 1 + sum((y * y - x**3 - a * x - b) % p == 0
                           for x in range(p) for y in range(p))
    if curve_points != r:
        fail("independent toy curve cardinality differs")
    result = None
    orbit = {None}
    generator = point(problem["generator"])
    for _ in range(1, r):
        result = add(result, generator, p, a)
        orbit.add(result)
    if len(orbit) != r or add(result, generator, p, a) is not None:
        fail("independent generator orbit differs")
    result = None
    for _ in range(expected):
        result = add(result, generator, p, a)
    if result != point(problem["target"]):
        fail("independent toy group replay disagrees with target")


def verify(args):
    raw, inputs = Path(args.raw), Path(args.inputs)
    files = check_result_tree(raw)
    image = read_json(Path(args.image))
    checkpoint = read_json(Path(args.checkpoint))
    if image.get("schema") != "point-index.image/v3" or checkpoint.get("schema") != "point-index.source-checkpoint/v1":
        fail("wrong image or checkpoint schema")
    if not re.fullmatch(r"104186811093[.]dkr[.]ecr[.]us-east-2[.]amazonaws[.]com/isolab/tasks@sha256:[a-f0-9]{64}", image.get("image_uri", "")):
        fail("image receipt lacks an immutable native ECR digest")
    if not re.fullmatch(r"s3://setec-astronomy-artifacts/checkpoints/isolab-point-index/[0-9]+-[0-9]+/crypto/(main|detached-[a-f0-9]{7,40})/[0-9]{8}T[0-9]{6}Z-[a-f0-9]{12}/", checkpoint.get("checkpoint_uri", "")):
        fail("source checkpoint receipt has an unexpected S3 prefix")
    if not re.fullmatch(r"[a-f0-9]{64}", checkpoint.get("archive_sha256", "")):
        fail("source checkpoint receipt lacks an archive digest")
    if checkpoint.get("upload_readback_verified") is not True or checkpoint.get("experiment_submitted") is not False:
        fail("source checkpoint readback is not recorded")
    if image.get("experiment_submitted") is not False:
        fail("image receipt has a submitted experiment marker")
    for key in ("driver_revision", "baseline_revision", "candidate_revision", "inputs_sha256", "seed_manifest_sha256"):
        if image.get(key) != checkpoint.get(key):
            fail(f"image and checkpoint disagree on {key}")
    if sha(files["build-info.json"]) != image.get("build_info_sha256"):
        fail("result build metadata differs from image receipt")
    build = read_json(files["build-info.json"])
    if build.get("schema") != "point-index.build-pair/v2" or build.get("driver_revision") != image.get("driver_revision"):
        fail("result build metadata has the wrong source identity")
    for label in ("baseline", "candidate", "panel"):
        if build.get(f"{label}_sha256") != image.get(f"{label}_sha256"):
            fail(f"result build and image disagree on {label}")
    if sha(files["inputs.sha256"]) != image.get("inputs_sha256"):
        fail("result input hash list differs from image receipt")
    check_public_inputs(inputs, files["inputs.sha256"])
    manifest = read_json(inputs / "manifest.json")
    seeds = read_json(inputs / "seed-manifest.json")
    if (manifest.get("schema") != "hashed-ecdl-panel/v2" or manifest.get("status") != "prepared"
            or manifest.get("execution") != "not_run" or manifest.get("measurements") is not None
            or len(manifest.get("cases", [])) != 84):
        fail("public panel manifest is incomplete or already marked executed")
    if sha(inputs / "seed-manifest.json") != image.get("seed_manifest_sha256"):
        fail("public schedule seed manifest differs from image receipt")
    if manifest.get("baseline") != image.get("baseline_revision") or manifest.get("paired_baseline") != image.get("candidate_revision"):
        fail("public panel does not bind both image arms")
    job = files["job-id.txt"].read_text().strip()
    pod = files["pod-uid.txt"].read_text().strip()
    node = files["node-name.txt"].read_text().strip()
    slot = files["schedule-index.txt"].read_text().strip()
    if not re.fullmatch(r"point-index-[a-f0-9]{8}-s[012]-a0", job) or not pod or not node or slot not in ("0", "1", "2"):
        fail("missing job, pod, node or schedule identity")
    if job != f"point-index-{image['driver_revision'][:8]}-s{slot}-a0":
        fail("job name does not bind driver and schedule")
    if args.s3_uri != f"s3://isolab-results-104186811093-us-east-2/runs/{job}/{pod}/":
        fail("downloaded result prefix differs from pod identity")
    if args.structure_only:
        return {"schema": "point-index.result-replay/v1", "scope": "structure_only", "job_name": job,
                "file_count": len(files) - 1, "scientific_result": None}
    if files["panel.exit"].read_text().strip() != "0":
        fail("panel process did not exit successfully")
    for label, binary in (("baseline", args.baseline_bin), ("candidate", args.candidate_bin), ("panel", args.panel_bin)):
        if sha(binary) != image.get(f"{label}_sha256"):
            fail(f"independent {label} binary differs from image")
    analysis = read_json(files["panel-run/analysis.json"])
    if analysis.get("protocol_complete") is not True or analysis.get("evidence_class") != "protocol_schedule_requires_external_L2_admission":
        fail("panel did not complete the frozen protocol")
    if (analysis.get("inputs", {}).get("manifest_sha256") != sha(inputs / "manifest.json")
            or analysis.get("inputs", {}).get("inputs_sha256_file_sha256") != image["inputs_sha256"]
            or analysis.get("inputs", {}).get("selected_cases") != 84):
        fail("analysis input hashes or case count differ")
    if analysis.get("arms") != {"baseline_sha256": image["baseline_sha256"],
                                "candidate_sha256": image["candidate_sha256"],
                                "optimized_options_sha256": image["optimized_options_sha256"],
                                "unchanged_after_run": True}:
        fail("analysis arm identities changed")
    schedule = analysis.get("schedule", {})
    if (schedule.get("seed") != str(seeds["schedule_seeds"][int(slot)]) or
            schedule.get("aa_rounds") != 5 or schedule.get("ab_rounds") != 5 or
            schedule.get("deadline_ms") != 1800000 or schedule.get("executions") != 1682):
        fail("analysis schedule differs from frozen plan")
    cases = {case["id"]: case for case in manifest["cases"]}
    if files["panel-run/rows.jsonl"].stat().st_size > 128 * 1024 * 1024:
        fail("panel rows exceed readback bound")
    rows = [json.loads(line, object_pairs_hook=unique_pairs) for line in files["panel-run/rows.jsonl"].read_text().splitlines()]
    if len(rows) != 1682 or len(cases) != 84:
        fail("incomplete panel rows or public cases")
    for index, row in enumerate(rows):
        if row.get("row") != index or row.get("status") != "validated" or row.get("replay_status") != "verified":
            fail(f"row {index} did not validate and replay")
        checks = row.get("checks")
        if (row.get("native_metrics_check") != "validated" or not isinstance(checks, dict)
                or len(checks) != 10 or not all(value is True for value in checks.values())):
            fail(f"row {index} lacks native metrics or independent checks")
        arm = row.get("arm")
        if arm not in ("baseline", "candidate") or row.get("binary_sha256") != image[f"{arm}_sha256"]:
            fail(f"row {index} uses an unexpected arm")
        if row.get("optimized_options_sha256") != image["optimized_options_sha256"]:
            fail(f"row {index} uses an unexpected optimized policy")
        case = cases.get(row.get("case_id"))
        if case is None or row.get("command") != "solve-hashed-optimized":
            fail(f"row {index} uses an unexpected case or command")
        if (not re.fullmatch(r"[a-zA-Z0-9-]+", row["case_id"])
                or not re.fullmatch(r"problems/[a-zA-Z0-9-]+[.]json", case["problem"])):
            fail(f"row {index} has an unsafe public case path")
        stem = f"{index:05}-{row['kind']}-r{row['round']}-p{row['position']:02}-s{row['slot']}-{row['case_id']}"
        receipt = files.get(f"panel-run/receipts/{stem}.json")
        if receipt is None:
            fail(f"row {index} has no retained solver receipt")
        record = read_json(receipt)
        problem = read_json(inputs / case["problem"])
        expected = int(case["expected_scalar"])
        if record.get("problem") != problem or record.get("certificate", {}).get("scalar") != [expected]:
            fail(f"row {index} receipt problem or scalar differs")
        certificate = json.dumps(record["certificate"], separators=(",", ":"), ensure_ascii=False).encode()
        if hashlib.sha256(certificate).hexdigest() != row.get("certificate_sha256"):
            fail(f"row {index} certificate digest differs")
        if row.get("scalar") != str(expected) or record.get("diagnostics", {}).get("scalar_decimal") != str(expected):
            fail(f"row {index} scalar copies differ")
        native = row.get("native_metrics", {})
        if not isinstance(native, dict):
            fail(f"row {index} lacks native phases")
        phases = native.get("online_phases_ns", {})
        if (native.get("schema") != "hashed-ecdl-panel.native-metrics/v1"
                or not isinstance(phases, dict) or len(phases) != 5
                or not all(type(n) is int and n >= 0 for n in phases.values())
                or sum(phases.values()) != native.get("online_wall_ns")
                or native.get("calibrated_total_operations") is not None):
            fail(f"row {index} has inconsistent unpriced native phases")
        check_scalar(problem, expected)
        replay = subprocess.run([args.baseline_bin if arm == "baseline" else args.candidate_bin,
                                 "verify", str(receipt)], capture_output=True, timeout=30, check=False)
        if replay.returncode != 0:
            fail(f"row {index} certificate replay failed")
    rerun = subprocess.run([args.panel_bin, "analyze", str(files["panel-run/rows.jsonl"])],
                           capture_output=True, timeout=30, check=False)
    if rerun.returncode != 0:
        fail("fresh panel reanalysis failed")
    fresh = json.loads(rerun.stdout, object_pairs_hook=unique_pairs)
    if any(fresh.get(key) != analysis.get(key) for key in ("aa", "ab", "choices")):
        fail("fresh paired analysis disagrees with retained analysis")
    if any(analysis[kind].get("complete_pairs") != 420 or analysis[kind].get("excluded_pairs") != 0
           or analysis[kind].get("cases_in_interval") != 84
           for kind in ("aa", "ab")):
        fail("paired panel contains missing or excluded cases")
    return {"schema": "point-index.result-replay/v1", "scope": "complete_replay",
            "job_name": job, "file_count": len(files) - 1, "verified_rows": len(rows),
            "scientific_result": None}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--raw", required=True)
    parser.add_argument("--image", required=True)
    parser.add_argument("--checkpoint", required=True)
    parser.add_argument("--inputs", required=True)
    parser.add_argument("--s3-uri", required=True)
    parser.add_argument("--panel-bin")
    parser.add_argument("--baseline-bin")
    parser.add_argument("--candidate-bin")
    parser.add_argument("--structure-only", action="store_true")
    args = parser.parse_args()
    if not args.structure_only and not all((args.panel_bin, args.baseline_bin, args.candidate_bin)):
        parser.error("complete replay requires all three exact binaries")
    try:
        print(json.dumps(verify(args), sort_keys=True))
    except (ValueError, KeyError, TypeError, AttributeError, IndexError, OSError,
            subprocess.TimeoutExpired) as error:
        print(f"result replay rejected: {error}", file=sys.stderr)
        return 2
    return 0


if __name__ == "__main__":
    sys.exit(main())
