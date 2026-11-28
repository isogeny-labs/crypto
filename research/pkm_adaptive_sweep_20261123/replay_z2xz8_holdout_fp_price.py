#!/usr/bin/env python3
"""Independently replay held-out PKM field-price receipts and freeze prices."""

import argparse
import hashlib
import json
import math
import os
import subprocess
from pathlib import Path

IC_SOURCE = "62908d5299c04c0b9bda686eba7f19e271b09786"
IC_SHA = "de461514bcb5f6538781ee25785f11ac33940711a93c0b6cb6a5dfbe0a587002"
MANIFEST_SHA = "99140281d9a94c288b8993b503cf91e4226879052e25c63dfdad0fccd20c8bc8"
CALIBRATION_SHA = "e70c562189a390b4d2a97a92d6d54484c3c99a340e49274efa5ce221878a3523"
BUILD_SHA = "986197cbf10c3f903cd8ae4f8e0b41be024fc923b1cc8c7c0cf9295e3e5a2b23"
REPS, ITERS = 7, 200_000


def sha(data):
    return hashlib.sha256(data).hexdigest()


def read(path):
    return json.loads(path.read_bytes())


def quiet(state):
    return (isinstance(state, dict) and state.get("competing_processes") == []
            and isinstance(state.get("logical_cpus"), int)
            and state["logical_cpus"] > 0
            and isinstance(state.get("loadavg"), list)
            and len(state["loadavg"]) == 3
            and isinstance(state["loadavg"][0], (int, float))
            and math.isfinite(state["loadavg"][0])
            and state["loadavg"][0] <= state["logical_cpus"])


def valid_measurement(data, p):
    if (not isinstance(data, dict) or data.get("p") != p
            or data.get("reps") != REPS or data.get("iters") != ITERS
            or data.get("field_mults_per_rep") != 8 * ITERS):
        return False
    samples, checks = data.get("ns_per_mul_per_rep"), data.get("result_checks")
    if (not isinstance(samples, list) or len(samples) != REPS
            or not isinstance(checks, list) or len(checks) != REPS
            or any(not isinstance(x, (int, float)) or isinstance(x, bool)
                   or not math.isfinite(x) or x <= 0 for x in samples)):
        return False
    if data.get("ns_per_mul_median") != sorted(samples)[REPS // 2]:
        return False
    for rep, check in enumerate(checks):
        a = 1 + ((rep + 1) % (p - 1))
        b = 1 + ((rep + 2) % (p - 1))
        if not isinstance(check, int) or isinstance(check, bool) or check != a * pow(b, 8 * ITERS, p) % p:
            return False
    return True


def receipts(path):
    found = {}
    ordered = []
    for line in path.read_text(encoding="utf-8").splitlines():
        row = json.loads(line)
        if not isinstance(row, dict) or not isinstance(row.get("id"), str) or row["id"] in found:
            raise ValueError("missing or duplicate receipt id")
        found[row["id"]] = row
        ordered.append(row["id"])
    return found, ordered


def validate_sequence(rec, ordered, labels):
    """Account for every append-only row, including rejected attempts."""
    cursor = 1
    for series, members in (("forward", labels), ("reverse", reversed(labels))):
        for label in members:
            case = f"{series}:{label}"
            attempt = 1
            qualified = False
            while cursor < len(ordered) and ordered[cursor] == f"pre:{case}:a{attempt}":
                if attempt > 20 or qualified:
                    raise ValueError(f"attempt after limit or qualification: {case}")
                pre_id = ordered[cursor]
                cursor += 1
                pre_quiet = quiet(rec[pre_id])
                if rec[pre_id].get("quiet") is not pre_quiet:
                    raise ValueError(f"pre-call quiet flag differs: {case}:a{attempt}")
                if pre_quiet:
                    expected = [f"launch:{case}:a{attempt}", f"result:{case}:a{attempt}"]
                    if ordered[cursor:cursor + 2] != expected:
                        raise ValueError(f"unresolved or reordered launch: {case}:a{attempt}")
                    qualified = rec[expected[1]].get("qualifying") is True
                    cursor += 2
                attempt += 1
            if attempt == 1:
                raise ValueError(f"missing calibration attempts: {case}")
    if cursor != len(ordered):
        raise ValueError(f"unrecognized or out-of-order receipt: {ordered[cursor]}")


def raw_matches(item):
    path = Path(item["path"])
    data = path.read_bytes()
    if sha(data) != item["sha256"] or len(data) != item["bytes"]:
        raise ValueError(f"raw hash or length differs: {path}")
    return data


def preserve(path, data):
    path.parent.mkdir(parents=True, exist_ok=True)
    encoded = (json.dumps(data, indent=2, sort_keys=True) + "\n").encode()
    if path.exists():
        if path.read_bytes() != encoded:
            raise ValueError(f"frozen price selection differs: {path}")
        return
    fd = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o644)
    with os.fdopen(fd, "wb") as stream:
        stream.write(encoded)
        stream.flush()
        os.fsync(stream.fileno())


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--manifest", type=Path, required=True)
    parser.add_argument("--checkout", type=Path, required=True)
    parser.add_argument("--ic-bin", type=Path, required=True)
    parser.add_argument("--bin", type=Path, required=True)
    parser.add_argument("--build-receipt", type=Path, required=True)
    parser.add_argument("--store", type=Path, required=True)
    parser.add_argument("--development-store", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    manifest = args.manifest.resolve(strict=True)
    checkout = args.checkout.resolve(strict=True)
    ic_bin = args.ic_bin.resolve(strict=True)
    binary = args.bin.resolve(strict=True)
    build_path = args.build_receipt.resolve(strict=True)
    store = args.store.resolve(strict=True)
    development_store = args.development_store.resolve(strict=True)
    here = Path(__file__).resolve().parent
    source = here / "fp_mul_explicit.rs"
    protocol = here / "PROTOCOL_Z2XZ8_HOLDOUT_FP_PRICE_20261128.md"
    runner = here / "run_z2xz8_holdout_fp_price.py"
    helper = checkout / "research/pkm_nonenum_decomp_20261125/run_cold_panel.py"
    if (subprocess.check_output(["git", "-C", str(checkout), "rev-parse", "HEAD"],
                                text=True).strip()
            != IC_SOURCE):
        raise ValueError("pinned source commit differs")
    if (sha(ic_bin.read_bytes()) != IC_SHA or sha(manifest.read_bytes()) != MANIFEST_SHA
            or sha(binary.read_bytes()) != CALIBRATION_SHA
            or sha(build_path.read_bytes()) != BUILD_SHA):
        raise ValueError("pinned CLI or manifest differs")
    build = read(build_path)
    if (build.get("schema") != "pkm-fp-mul-build/v1"
            or build.get("source_sha256") != sha(source.read_bytes())
            or build.get("binary_sha256") != CALIBRATION_SHA
            or build.get("compile_command", [])[:4] !=
                ["rustc", "-O", "-C", "target-cpu=native"]
            or build.get("compile_exit_code") != 0):
        raise ValueError("standalone build differs")
    entries = sorted(read(manifest)["entries"],
                     key=lambda e: (e["bits"], e["curve_index"]))
    if [(e["bits"], e["curve_index"], e["p"]) for e in entries] != [
            (11, 0, 2081), (11, 1, 2113), (11, 2, 2273),
            (14, 0, 16417), (14, 1, 16481), (14, 2, 16673)]:
        raise ValueError("six-curve roster differs")
    primes = {}
    for e in entries:
        if sha((manifest.parent / e["path"]).read_bytes()) != e["sha256"]:
            raise ValueError(f"workload differs: {e['path']}")
        primes[f"b{e['bits']}_c{e['curve_index']}"] = e["p"]
    rec, ordered = receipts(store)
    if not ordered or ordered[0] != "header":
        raise ValueError("header must be first receipt")
    validate_sequence(rec, ordered, list(primes))
    header = rec.get("header") or {}
    development_rows, development_order = receipts(development_store)
    development_header = development_rows.get("header") or {}
    if (not development_order or development_order[0] != "header"
            or development_header.get("schema") != "pkm-explicit-fp-price/v1"
            or development_header.get("host") != header.get("host")
            or header.get("development_store_sha256") != sha(development_store.read_bytes())):
        raise ValueError("development-price admission receipt differs")
    for series in ("forward", "reverse"):
        for t in (4, 5):
            for c in range(3):
                prefix = f"result:{series}:t{t}_c{c}:a"
                if not any(k.startswith(prefix) and row.get("qualifying") is True
                           for k, row in development_rows.items()):
                    raise ValueError("development-price series was incomplete")
    if (header.get("schema") != "pkm-z2xz8-heldout-fp-price/v1"
            or header.get("source_commit") != IC_SOURCE
            or header.get("manifest_sha256") != MANIFEST_SHA
            or header.get("ic_binary_sha256") != IC_SHA
            or header.get("primes") != primes
            or header.get("calibration_source_sha256") != sha(source.read_bytes())
            or header.get("calibration_binary_sha256") != sha(binary.read_bytes())
            or header.get("calibration_build_sha256") != sha(build_path.read_bytes())
            or header.get("protocol_sha256") != sha(protocol.read_bytes())
            or header.get("runner_sha256") != sha(runner.read_bytes())
            or header.get("helper_sha256") != sha(helper.read_bytes())
            or header.get("host") != build.get("host")
            or header.get("architecture") != build.get("architecture")
            or header.get("reps") != REPS or header.get("iters") != ITERS
            or header.get("wall_cap_s") != 30
            or header.get("rss_cap_kib") != 2 * 1024 * 1024):
        raise ValueError("store header differs from frozen inputs")
    selected = {}
    rejected = []
    qualified_launches = []
    for label, p in primes.items():
        medians = {}
        for series in ("forward", "reverse"):
            case = f"{series}:{label}"
            attempts = sorted((k for k in rec if k.startswith(f"pre:{case}:a")),
                              key=lambda k: int(k.rsplit("a", 1)[1]))
            if not attempts or [int(k.rsplit("a", 1)[1]) for k in attempts] != list(range(1, len(attempts) + 1)):
                raise ValueError(f"missing or nonconsecutive attempts: {case}")
            qualified = []
            for pre_id in attempts:
                suffix = pre_id.split(":a")[-1]
                launch_id, result_id = f"launch:{case}:a{suffix}", f"result:{case}:a{suffix}"
                pre = rec[pre_id]
                if not quiet(pre):
                    if launch_id in rec or result_id in rec:
                        raise ValueError(f"busy pre-call launched: {case}:a{suffix}")
                    rejected.append({"case": case, "attempt": int(suffix), "reason": "busy_pre"})
                    continue
                launch = rec.get(launch_id)
                result = rec.get(result_id)
                if not launch or not result:
                    raise ValueError(f"unresolved launch or pre-call: {case}:a{suffix}")
                command = [str(binary), str(p), str(REPS), str(ITERS)]
                if (launch.get("command") != command or result.get("command") != command
                        or launch.get("pre_id") != pre_id or launch.get("p") != p
                        or result.get("pre_id") != pre_id or result.get("p") != p
                        or launch.get("stdout_path") != result.get("stdout", {}).get("path")
                        or launch.get("stderr_path") != result.get("stderr", {}).get("path")):
                    raise ValueError(f"command or prime differs: {case}:a{suffix}")
                stdout = raw_matches(result["stdout"])
                raw_matches(result["stderr"])
                try:
                    parsed = json.loads(stdout)
                except ValueError:
                    parsed = None
                valid = (result.get("exit_code") == 0 and not result.get("cap_hit")
                         and not result.get("load_contamination")
                         and quiet(result.get("postcall_load"))
                         and result["postcall_load"].get("quiet") is True
                         and isinstance(result.get("wall_s"), (int, float))
                         and result["wall_s"] > 0
                         and isinstance(result.get("process_cpu_s"), (int, float))
                         and result["process_cpu_s"] >= 0
                         and isinstance(result.get("peak_rss_kib"), int)
                         and result["peak_rss_kib"] > 0
                         and parsed == result.get("parsed") and valid_measurement(parsed, p))
                if result.get("qualifying") is not valid:
                    raise ValueError(f"qualification differs: {case}:a{suffix}")
                if valid:
                    qualified.append((result_id, parsed["ns_per_mul_median"]))
                    qualified_launches.append(launch_id)
                else:
                    rejected.append({"case": case, "attempt": int(suffix), "reason": "unqualified_result"})
            if len(qualified) != 1:
                raise ValueError(f"need exactly one qualifying result: {case}")
            medians[series] = qualified[0]
        f, r = medians["forward"][1], medians["reverse"][1]
        spread = max(f, r) / min(f, r) - 1
        if spread > 0.10:
            raise ValueError(f"forward/reverse price spread exceeds 10%: {label}: {spread}")
        selected[label] = {"p": p, "ns_per_mul": (f + r) / 2,
                           "forward_ns_per_mul": f, "reverse_ns_per_mul": r,
                           "relative_spread": spread,
                           "receipt_ids": [medians["forward"][0], medians["reverse"][0]]}
    expected_order = ([f"launch:forward:{label}:a1" for label in primes]
                      + [f"launch:reverse:{label}:a1" for label in reversed(primes)])
    actual_order = [x for x in ordered if x in set(qualified_launches)]
    # Attempt numbers may rise after a refused or contaminated attempt.
    if ([x.rsplit(":a", 1)[0] for x in actual_order]
            != [x.rsplit(":a", 1)[0] for x in expected_order]):
        raise ValueError("qualifying series order differs")
    output = {"schema": "pkm-z2xz8-heldout-fp-price-selection/v1",
              "store": str(store), "store_sha256": sha(store.read_bytes()),
              "manifest_sha256": MANIFEST_SHA, "ic_binary_sha256": IC_SHA,
              "calibration_source_sha256": sha(source.read_bytes()),
              "calibration_binary_sha256": sha(binary.read_bytes()),
              "calibration_build_sha256": sha(build_path.read_bytes()),
              "host": header["host"], "selected": selected, "rejected": rejected}
    preserve(args.output.resolve(), output)
    print(json.dumps({"output": str(args.output.resolve()), "selected_primes": len(selected),
                      "rejected_attempts": len(rejected),
                      "store_sha256": output["store_sha256"]}, sort_keys=True))


if __name__ == "__main__":
    main()
