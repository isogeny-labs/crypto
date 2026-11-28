#!/usr/bin/env python3
"""Run the six held-out PKM prime multiplication prices, resumably."""

import argparse
import datetime as dt
import fcntl
import hashlib
import importlib.util
import json
import math
import platform
import subprocess
from pathlib import Path

IC_SOURCE = "62908d5299c04c0b9bda686eba7f19e271b09786"
IC_SHA = "de461514bcb5f6538781ee25785f11ac33940711a93c0b6cb6a5dfbe0a587002"
MANIFEST_SHA = "99140281d9a94c288b8993b503cf91e4226879052e25c63dfdad0fccd20c8bc8"
CALIBRATION_SHA = "e70c562189a390b4d2a97a92d6d54484c3c99a340e49274efa5ce221878a3523"
BUILD_SHA = "986197cbf10c3f903cd8ae4f8e0b41be024fc923b1cc8c7c0cf9295e3e5a2b23"
REPS, ITERS = 7, 200_000
PROTOCOL = "PROTOCOL_Z2XZ8_HOLDOUT_FP_PRICE_20261128.md"
SOURCE = "fp_mul_explicit.rs"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def read(path):
    return json.loads(path.read_bytes())


def helper_from(checkout):
    path = checkout / "research/pkm_nonenum_decomp_20261125/run_cold_panel.py"
    spec = importlib.util.spec_from_file_location("pkm_cold_helpers", path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    module.WALL_CAP_S = 30
    return module, sha(path.read_bytes())


def quiet(helper):
    state = helper.quiet()
    state["quiet"] = (not state["competing_processes"]
                      and state["loadavg"][0] <= state["logical_cpus"])
    return state


def valid_output(out, p):
    if (out.get("p") != p or out.get("reps") != REPS or out.get("iters") != ITERS
            or out.get("field_mults_per_rep") != 8 * ITERS):
        return False
    samples = out.get("ns_per_mul_per_rep")
    checks = out.get("result_checks")
    if (not isinstance(samples, list) or len(samples) != REPS
            or not isinstance(checks, list) or len(checks) != REPS
            or any(not isinstance(x, (int, float)) or not math.isfinite(x) or x <= 0
                   for x in samples)):
        return False
    return (out.get("ns_per_mul_median") == sorted(samples)[REPS // 2]
            and all(isinstance(x, int) and 0 <= x < p for x in checks))


def development_done(path):
    if not path.is_file():
        return False
    rows = [json.loads(line) for line in path.read_text().splitlines()]
    if (not rows or rows[0].get("id") != "header"
            or rows[0].get("schema") != "pkm-explicit-fp-price/v1"
            or rows[0].get("host") != platform.node()
            or len({row.get("id") for row in rows}) != len(rows)):
        return False
    for series in ("forward", "reverse"):
        for t in (4, 5):
            for c in range(3):
                prefix = f"result:{series}:t{t}_c{c}:a"
                if not any(row.get("id", "").startswith(prefix) and row.get("qualifying")
                           for row in rows):
                    return False
    return True


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--checkout", type=Path, required=True)
    parser.add_argument("--manifest", type=Path, required=True)
    parser.add_argument("--ic-bin", type=Path, required=True)
    parser.add_argument("--bin", type=Path, required=True)
    parser.add_argument("--build-receipt", type=Path, required=True)
    parser.add_argument("--store", type=Path, required=True)
    parser.add_argument("--development-store", type=Path,
                        default=Path("/Volumes/SSD990-2/pkm-fp-price-20261128/receipts.jsonl"))
    parser.add_argument("--series", choices=("forward", "reverse"), required=True)
    parser.add_argument("--max-new", type=int, default=0)
    parser.add_argument("--dry-run", action="store_true")
    args = parser.parse_args()
    if args.max_new < 0:
        parser.error("--max-new must be nonnegative")
    here = Path(__file__).resolve().parent
    checkout = args.checkout.resolve(strict=True)
    manifest = args.manifest.resolve(strict=True)
    ic_bin = args.ic_bin.resolve(strict=True)
    binary = args.bin.resolve(strict=True)
    build_path = args.build_receipt.resolve(strict=True)
    store = args.store.resolve()
    if (subprocess.check_output(["git", "-C", str(checkout), "rev-parse", "HEAD"],
                                text=True).strip() != IC_SOURCE
            or subprocess.check_output(["git", "-C", str(checkout), "status",
                                        "--porcelain=v1", "--untracked-files=no"],
                                       text=True).strip()):
        parser.error("pinned source checkout differs or has tracked edits")
    if (sha(ic_bin.read_bytes()) != IC_SHA or sha(manifest.read_bytes()) != MANIFEST_SHA
            or sha(binary.read_bytes()) != CALIBRATION_SHA
            or sha(build_path.read_bytes()) != BUILD_SHA):
        parser.error("pinned CLI or manifest hash differs")
    build = read(build_path)
    if (build.get("schema") != "pkm-fp-mul-build/v1"
            or build.get("source_sha256") != sha((here / SOURCE).read_bytes())
            or build.get("binary_sha256") != CALIBRATION_SHA
            or build.get("compile_command", [])[:4] !=
                ["rustc", "-O", "-C", "target-cpu=native"]
            or build.get("host") != platform.node()
            or build.get("architecture") != platform.machine()
            or not isinstance(build.get("rustc_version"), str)
            or build.get("compile_exit_code") != 0):
        parser.error("standalone calibration build receipt differs")
    helper, helper_sha = helper_from(checkout)
    dev_complete = development_done(args.development_store)
    entries = sorted(read(manifest)["entries"],
                     key=lambda e: (e["bits"], e["curve_index"]))
    if [(e["bits"], e["curve_index"], e["p"]) for e in entries] != [
            (11, 0, 2081), (11, 1, 2113), (11, 2, 2273),
            (14, 0, 16417), (14, 1, 16481), (14, 2, 16673)]:
        parser.error("six-curve roster differs")
    for e in entries:
        if sha((manifest.parent / e["path"]).read_bytes()) != e["sha256"]:
            parser.error(f"workload hash differs: {e['path']}")
    header = {"id": "header", "schema": "pkm-z2xz8-heldout-fp-price/v1",
              "source_commit": IC_SOURCE, "ic_binary_sha256": IC_SHA,
              "development_store_sha256": (sha(args.development_store.read_bytes())
                                           if dev_complete else None),
              "manifest_sha256": MANIFEST_SHA,
              "primes": {f"b{e['bits']}_c{e['curve_index']}": e["p"] for e in entries},
              "calibration_binary_sha256": sha(binary.read_bytes()),
              "calibration_build_sha256": sha(build_path.read_bytes()),
              "calibration_source_sha256": sha((here / SOURCE).read_bytes()),
              "protocol_sha256": sha((here / PROTOCOL).read_bytes()),
              "runner_sha256": sha(Path(__file__).read_bytes()),
              "helper_sha256": helper_sha, "host": platform.node(),
              "architecture": platform.machine(), "reps": REPS, "iters": ITERS,
              "wall_cap_s": helper.WALL_CAP_S, "rss_cap_kib": helper.RSS_CAP_KIB,
              "created_utc": dt.datetime.now(dt.timezone.utc).isoformat()}
    if args.dry_run:
        print(json.dumps({"header": header, "series": args.series,
                          "development_complete": dev_complete},
                         sort_keys=True))
        return
    if not dev_complete:
        print(json.dumps({"waiting_for_development_price_series":
                          str(args.development_store)}, sort_keys=True))
        return
    store.parent.mkdir(parents=True, exist_ok=True)
    lock = (store.parent / "receipts.lock").open("a+")
    fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
    prior = helper.records(store)
    if not prior:
        helper.append(store, header)
        prior["header"] = header
    else:
        stable = lambda x: {k: v for k, v in x.items() if k != "created_utc"}
        if stable(prior["header"]) != stable(header):
            raise ValueError("frozen store header differs")
    if args.series == "reverse":
        for e in entries:
            case = f"forward:b{e['bits']}_c{e['curve_index']}"
            if not any(v.get("qualifying") for k, v in prior.items()
                       if k.startswith("result:" + case + ":")):
                raise ValueError(f"forward series incomplete: {case}")
        entries.reverse()
    launched = 0
    for e in entries:
        case = f"{args.series}:b{e['bits']}_c{e['curve_index']}"
        if any(v.get("qualifying") for k, v in prior.items()
               if k.startswith("result:" + case + ":")):
            continue
        attempt = 1 + sum(k.startswith("pre:" + case + ":a") for k in prior)
        if attempt > 20:
            raise RuntimeError(f"20 attempts exhausted: {case}")
        for n in range(1, attempt):
            key = f"launch:{case}:a{n}"
            if key in prior and f"result:{case}:a{n}" not in prior:
                raise RuntimeError(f"unresolved launch retained: {key}")
        if args.max_new and launched >= args.max_new:
            return
        pre = quiet(helper)
        pre_id = f"pre:{case}:a{attempt}"
        helper.append(store, {"id": pre_id, **pre})
        prior[pre_id] = pre
        if not pre["quiet"]:
            print(json.dumps({"waiting_for_quiet_host": case,
                              "pre_id": pre_id}, sort_keys=True))
            return
        command = [str(binary), str(e["p"]), str(REPS), str(ITERS)]
        raw = store.parent / "raw"
        raw.mkdir(parents=True, exist_ok=True)
        stem = case.replace(":", "_") + f"_a{attempt}"
        stdout, stderr = raw / (stem + ".stdout"), raw / (stem + ".stderr")
        launch_id = f"launch:{case}:a{attempt}"
        helper.append(store, {"id": launch_id, "command": command,
                              "pre_id": pre_id, "p": e["p"],
                              "stdout_path": str(stdout), "stderr_path": str(stderr)})
        prior[launch_id] = {"command": command}
        result = helper.run_one(command, stdout, stderr)
        post = quiet(helper)
        result["postcall_load"] = post
        result["id"] = f"result:{case}:a{attempt}"
        result["command"] = command
        result["p"] = e["p"]
        result["pre_id"] = pre_id
        try:
            parsed = read(stdout)
        except (OSError, ValueError):
            parsed = None
        result["parsed"] = parsed
        result["qualifying"] = bool(
            result["exit_code"] == 0 and not result["cap_hit"]
            and not result["load_contamination"] and post["quiet"]
            and isinstance(parsed, dict) and valid_output(parsed, e["p"]))
        helper.append(store, result)
        prior[result["id"]] = result
        launched += 1
        if not result["qualifying"]:
            raise RuntimeError(f"calibration attempt retained but unqualified: {case}")
    print(json.dumps({"series": args.series, "qualifying_cases": len(entries),
                      "new_launches": launched, "store": str(store)}, sort_keys=True))


if __name__ == "__main__":
    main()
