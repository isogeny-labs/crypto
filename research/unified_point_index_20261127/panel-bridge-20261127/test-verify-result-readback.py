#!/usr/bin/env python3
"""Offline file-closure controls for a separately downloaded native result."""

import hashlib
import importlib.util
import json
from pathlib import Path
import shutil
import subprocess
import tempfile


HERE = Path(__file__).resolve().parent
VERIFY = HERE / "verify-result-readback.py"
loader = importlib.util.spec_from_file_location("point_index_result_replay", VERIFY)
replay_module = importlib.util.module_from_spec(loader)
loader.loader.exec_module(replay_module)


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def write_json(path, value):
    path.write_text(json.dumps(value, indent=2) + "\n")


def checksum(root, output):
    lines = [f"{sha(path)}  {path.relative_to(root).as_posix()}"
             for path in sorted(root.rglob("*")) if path.is_file() and path != output]
    output.write_text("\n".join(lines) + "\n")


def main():
    receipt = json.loads((HERE.parents[2] / "crates/hashed-ecdl-tensor/tests/data/panel-receipt-p5-development-s001-t0.json").read_text())
    canonical = json.dumps(receipt["certificate"], separators=(",", ":"), ensure_ascii=False).encode()
    assert hashlib.sha256(canonical).hexdigest() == "992ede1c21f04ab882e9da167041f75fa14b37c3075abcaff6aa839220d94a45"
    with tempfile.TemporaryDirectory(prefix="point-index-result-closure-") as temp:
        base = Path(temp)
        inputs = base / "inputs"
        shutil.copytree(HERE / "fresh-inputs-20261127/panel", inputs)
        baseline, candidate, driver = "b" * 40, "c" * 40, "d" * 40
        manifest = json.loads((inputs / "manifest.json").read_text())
        manifest["baseline"] = baseline
        manifest["paired_baseline"] = candidate
        write_json(inputs / "manifest.json", manifest)
        for case in manifest["cases"]:
            problem = json.loads((inputs / case["problem"]).read_text())
            replay_module.check_scalar(problem, int(case["expected_scalar"]))
        wrong_problem = json.loads((inputs / manifest["cases"][0]["problem"]).read_text())
        wrong_problem["target"] = ["0", "0"]
        try:
            replay_module.check_scalar(wrong_problem, int(manifest["cases"][0]["expected_scalar"]))
        except ValueError:
            pass
        else:
            raise AssertionError("independent toy oracle accepted wrong target")
        input_list = base / "inputs.sha256"
        checksum(inputs, input_list)
        raw = base / "download"
        raw.mkdir()
        build = {"schema": "point-index.build-pair/v2", "driver_revision": driver,
                 "baseline_revision": baseline, "candidate_revision": candidate,
                 "baseline_sha256": "1" * 64, "candidate_sha256": "2" * 64,
                 "panel_sha256": "3" * 64}
        write_json(raw / "build-info.json", build)
        shutil.copyfile(input_list, raw / "inputs.sha256")
        job = f"point-index-{driver[:8]}-s0-a0"
        pod = "pod-123"
        for filename, value in (("job-id.txt", job), ("pod-uid.txt", pod),
                                ("node-name.txt", "node-1"), ("schedule-index.txt", "0")):
            (raw / filename).write_text(value + "\n")
        checksum(raw, raw / "SHA256SUMS")
        image = {"schema": "point-index.image/v3", "driver_revision": driver,
                 "image_uri": "104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:" + "a" * 64,
                 "baseline_revision": baseline, "candidate_revision": candidate,
                 "inputs_sha256": sha(input_list),
                 "seed_manifest_sha256": sha(inputs / "seed-manifest.json"),
                 "build_info_sha256": sha(raw / "build-info.json"),
                 "baseline_sha256": "1" * 64, "candidate_sha256": "2" * 64,
                 "panel_sha256": "3" * 64, "experiment_submitted": False}
        checkpoint = {key: image[key] for key in ("driver_revision", "baseline_revision",
                       "candidate_revision", "inputs_sha256", "seed_manifest_sha256")}
        checkpoint.update(schema="point-index.source-checkpoint/v1",
                          checkpoint_uri="s3://setec-astronomy-artifacts/checkpoints/isolab-point-index/123-1/crypto/detached-" + driver[:12] + "/20261127T190000Z-" + driver[:12] + "/",
                          archive_sha256="e" * 64,
                          upload_readback_verified=True, experiment_submitted=False)
        write_json(base / "image.json", image)
        write_json(base / "checkpoint.json", checkpoint)
        uri = f"s3://isolab-results-104186811093-us-east-2/runs/{job}/{pod}/"
        command = [str(VERIFY), "--raw", str(raw), "--image", str(base / "image.json"),
                   "--checkpoint", str(base / "checkpoint.json"), "--inputs", str(inputs),
                   "--s3-uri", uri, "--structure-only"]

        def run(accepted):
            result = subprocess.run(command, capture_output=True, text=True, check=False)
            if (result.returncode == 0) != accepted:
                raise AssertionError(f"unexpected replay result: {result.returncode} {result.stdout} {result.stderr}")
            return result

        assert json.loads(run(True).stdout)["scope"] == "structure_only"
        full_command = [arg for arg in command if arg != "--structure-only"] + [
            "--panel-bin", "/bin/false", "--baseline-bin", "/bin/false",
            "--candidate-bin", "/bin/false"]
        incomplete = subprocess.run(full_command, capture_output=True, text=True, check=False)
        assert incomplete.returncode != 0 and "result replay rejected" in incomplete.stderr
        (raw / "job-id.txt").write_text("changed\n")
        run(False)
        (raw / "job-id.txt").write_text(job + "\n")
        (raw / "unlisted.txt").write_text("extra\n")
        run(False)
        (raw / "unlisted.txt").unlink()
        (inputs / "problems/p5-development-f00-t0.json").write_text("{}\n")
        run(False)
        shutil.copyfile(HERE / "fresh-inputs-20261127/panel/problems/p5-development-f00-t0.json",
                        inputs / "problems/p5-development-f00-t0.json")
        command[command.index(uri)] = "s3://setec-astronomy-artifacts/runs/wrong/"
        run(False)
        command[command.index("s3://setec-astronomy-artifacts/runs/wrong/")] = uri
        checkpoint["inputs_sha256"] = "0" * 64
        write_json(base / "checkpoint.json", checkpoint)
        run(False)
        print("Result readback closure: exact files and identities accepted; altered file, unlisted file, public input, S3 URI and checkpoint rejected. No solver, benchmark or S3 call ran.")


if __name__ == "__main__":
    main()
