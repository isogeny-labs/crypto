#!/usr/bin/env python3
"""Match index policy and public point while alternating K1111/K1115."""

import argparse
import os
from pathlib import Path
import shutil
import subprocess
import time

from research.n83_fp8_e2e_control_20261025 import run
from research.n83_nearby_grid_tier1_20261024 import protocol as fixture


ORDER = (1111, 1115, 1115, 1111)
BASE_HASHES = {
    1111: "f0c8213daff19553f3a3b450e89b5b8e85fb932d134c004ef89aeff89745b705",
    1115: "95e970dcb7cbc5c299498409cd062749c49c6320175e83e0db7e635d906a594f",
}
BASE_MANIFEST_SHA256 = {
    1111: "1e2721602fb8f7240b551d38137fa014cd39a9b8233d0fcd0c26ec387d36349c",
    1115: "1fb0f87d0977cc8a40105ffd74943c70fb551a038508d5c7b971bf00619aa932",
}


def base_manifest_path(k):
    candidate, expected = fixture.CANDIDATES[k]
    if expected != BASE_MANIFEST_SHA256[k]:
        raise ValueError("frozen candidate catalog differs")
    return run.ROOT / "research/n83_nearby_full_grid_20261024/identities" / f"candidate_{candidate}.json"


def experiment(binary, output):
    binary = Path(binary).resolve(strict=True)
    output = Path(output).resolve()
    if (output.exists() or output == run.ROOT or run.ROOT in output.parents or
            run.sha(run.SOURCE) != run.SOURCE_SHA256 or
            run.sha(binary) != "a768b71f8059f0cf9049213ebc5bd2ed7ff9eccc3ab4947cff822093b58ef706" or
            run.sha(fixture.TARGETS) != fixture.TARGETS_SHA256 or
            run.sha(fixture.ANSWERS) != fixture.ANSWERS_SHA256 or
            any(run.sha(base_manifest_path(k)) != BASE_MANIFEST_SHA256[k]
                for k in BASE_HASHES)):
        raise ValueError("frozen source, binary, point or fresh output gate failed")
    output.mkdir(parents=True)
    executed = output / "koblitz_orbit_dlp_fast"
    shutil.copy2(binary, executed)
    executed.chmod(0o755)
    receipt = {
        "schema": "n83.fp8-base-abba/v1", "scope": "shared_host_exploratory",
        "source_sha256": run.SOURCE_SHA256, "binary_sha256": run.sha(executed),
        "target_sha256": fixture.TARGETS_SHA256, "answer_sha256": fixture.ANSWERS_SHA256,
        "order": ORDER, "base_flags": run.FLAGS, "base_hashes": BASE_HASHES,
        "base_manifest_sha256": BASE_MANIFEST_SHA256,
        "cap_seconds_per_child": run.CAP_SECONDS, "rss_cap_bytes": run.RSS_CAP_BYTES,
        "started_utc": run.now(), "rows": [],
    }
    run.write(output / "summary.json", receipt)
    for index, k in enumerate(ORDER):
        folder = output / f"{index}-k{k}"
        folder.mkdir()
        flags = dict(run.FLAGS)
        flags["KIC_INDEXED4_FINGERPRINT8"] = "1"
        environment = {name: value for name, value in os.environ.items()
                       if not name.startswith("KIC_")}
        environment.update(flags)
        command = [str(executed), f"construct:83:1:{k}", str(fixture.TARGETS),
                   "29", str(folder / "target.jsonl")]
        run.write(folder / "launch.json",
                  {"command": command, "flags": flags, "started_utc": run.now()})
        started = time.monotonic()
        peak, reason = 0, None
        with (folder / "stdout.jsonl").open("wb") as stdout, \
             (folder / "stderr.log").open("wb") as stderr:
            child = subprocess.Popen(command, cwd=run.ROOT, env=environment,
                                     stdout=stdout, stderr=stderr,
                                     start_new_session=True)
            while child.poll() is None:
                peak = max(peak, run.sample_rss(child.pid))
                if peak > run.RSS_CAP_BYTES:
                    reason = "RSS_CAP"
                    run.stop_group(child.pid)
                    break
                if time.monotonic() - started > run.CAP_SECONDS:
                    reason = "TIME_CAP"
                    run.stop_group(child.pid)
                    break
                time.sleep(0.25)
            code = child.wait()
        row = {
            "index": index, "k": k,
            "status": reason or ("PROCESS_EXITED" if code == 0 else "PROCESS_FAILED"),
            "exit_code": code, "wall_seconds": time.monotonic() - started,
            "peak_sampled_rss_bytes": peak,
            "stdout_sha256": run.sha(folder / "stdout.jsonl"),
            "stderr_sha256": run.sha(folder / "stderr.log"),
            "target_output_sha256": run.sha(folder / "target.jsonl")
            if (folder / "target.jsonl").exists() else None,
            "completed_utc": run.now(),
        }
        receipt["rows"].append(row)
        run.write(output / "summary.json", receipt)
        print(index, k, row["status"], round(row["wall_seconds"], 3), flush=True)
        if row["status"] != "PROCESS_EXITED":
            break
    receipt["stopped_utc"] = run.now()
    run.write(output / "summary.json", receipt)
    return 0 if len(receipt["rows"]) == 4 and all(
        row["status"] == "PROCESS_EXITED" for row in receipt["rows"]) else 1


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()
    raise SystemExit(experiment(args.binary, args.output))
