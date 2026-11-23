#!/usr/bin/env python3
"""Administrative checkpoint controller; cryptography stays in the native binary."""
import argparse
import fcntl
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import time
import uuid

HERE = Path(__file__).resolve().parent
ROOT = Path("/mnt/spot-codex-state/tmp/ml-dataset-binary-root-bsgs-20261031-v2")
BINARY = Path("/home/ec2-user/ml-dataset-work-20261028/clean-build-02/crypto")
RUNNER = Path("/home/ec2-user/ml-dataset-work-20261028/runtime-preflight-runner-test-01/runner")
STORE = Path("/home/ec2-user/ml-dataset-work-20261028/graph-export-crypto")
SOURCE = "c41bec150e16e4c4f9a00634002b633b821de262"
HASHES = {
    BINARY: "b8e0881ec5e46f60c23e6bbb49ad7c3677b9168199f5787b68d89169bfaaa77e",
    RUNNER: "10258a68826f3ac123aa3190a4604377f652625f287d99990ab8d833879bd26b",
    STORE: "0d965332c529967b066ddc99819f47cdd57baeccde59a37cbdee94455cbc17b3",
    HERE / "protocol-v2.json": "90d677e590a21f2fbdaeb203d444073e931ffc91915249486aa8d809dadfdb62",
    HERE / "plan-v2.json": "1224a4060c29382923f048c7b72cb5d6685fbf7136efd1b68b6493a2b91e2b19",
}
BUCKET = "setec-astronomy-artifacts"
AWS_ENV = {
    "AWS_CONFIG_FILE": "/tmp/ml-dataset-aws-config-20261028",
    "AWS_PROFILE": "ml-dataset-checkpoint",
    "AWS_DEFAULT_REGION": "us-east-2",
    "CRYPTO_STORE_REMOTE": "s3://setec-astronomy-artifacts/store",
    "CRYPTO_STORE_INDEX": str(ROOT / "store-index.json"),
}


def digest(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def write_json(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_name(path.name + ".tmp-" + uuid.uuid4().hex)
    with open(tmp, "x", encoding="utf-8") as f:
        json.dump(value, f, sort_keys=True, indent=2)
        f.write("\n")
        f.flush()
        os.fsync(f.fileno())
    os.replace(tmp, path)
    fd = os.open(path.parent, os.O_RDONLY)
    try:
        os.fsync(fd)
    finally:
        os.close(fd)


def command(argv, timeout, stdout=None, env=None):
    started = time.time_ns()
    try:
        p = subprocess.run(
            list(map(str, argv)), stdin=subprocess.DEVNULL, stdout=subprocess.PIPE,
            stderr=subprocess.PIPE, timeout=timeout, start_new_session=True,
            env=env, check=False,
        )
        status = {"exit_code": p.returncode, "timed_out": False}
        out, err = p.stdout, p.stderr
    except subprocess.TimeoutExpired as e:
        # subprocess.run kills the direct process. A supervised native runner
        # normally exits itself after 90+5 seconds; any orphan is detected
        # before the next shard and blocks advancement.
        status = {"exit_code": None, "timed_out": True}
        out, err = e.stdout or b"", e.stderr or b""
    status["started_unix_ns"] = str(started)
    status["finished_unix_ns"] = str(time.time_ns())
    status["argv"] = list(map(str, argv))
    status["stdout_sha256"] = hashlib.sha256(out).hexdigest()
    status["stderr_sha256"] = hashlib.sha256(err).hexdigest()
    if stdout:
        stdout.parent.mkdir(parents=True, exist_ok=True)
        stdout.write_bytes(out)
        stdout.with_suffix(".stderr").write_bytes(err)
    return status


def stage(shard, name, argv, timeout, env=None):
    receipt = shard / (name + ".json")
    log = shard / (name + ".stdout")
    if receipt.exists():
        saved = json.loads(receipt.read_text())
        if saved.get("argv") != list(map(str, argv)) or not log.is_file() or not log.with_suffix(".stderr").is_file():
            raise RuntimeError(f"retained {name} receipt does not bind expected command/logs")
        if digest(log) != saved.get("stdout_sha256") or digest(log.with_suffix(".stderr")) != saved.get("stderr_sha256"):
            raise RuntimeError(f"retained {name} log digest mismatch")
        return saved
    if log.exists() or log.with_suffix(".stderr").exists():
        raise RuntimeError(f"unreceipted {name} output at {shard}; preserve for manual reconciliation")
    result = command(argv, timeout, log, env)
    write_json(receipt, result)
    return result


def active_worker(shard):
    needle = os.fsencode(str(shard / "runner"))
    for proc in Path("/proc").glob("[0-9]*"):
        if int(proc.name) == os.getpid():
            continue
        try:
            argv = (proc / "cmdline").read_bytes()
        except (OSError, PermissionError):
            continue
        if needle in argv and (b"crypto" in argv or b"runner" in argv):
            return proc.name
    return None


def admission():
    for path, expected in HASHES.items():
        if not path.is_file() or digest(path) != expected:
            raise RuntimeError(f"pinned input mismatch: {path}")
    build = json.loads(Path("/home/ec2-user/ml-dataset-work-20261028/clean-build-02/receipt.json").read_text())
    if SOURCE not in json.dumps(build):
        raise RuntimeError("clean-build receipt lacks source commit")
    plan = json.loads((HERE / "plan-v2.json").read_text())
    if plan.get("protocol_sha256") != HASHES[HERE / "protocol-v2.json"]:
        raise RuntimeError("plan is not bound to frozen protocol file")
    if plan.get("field_sizes") != [23] or plan.get("arities") != [2] or plan.get("families") != ["binary"] or plan.get("solvers") != ["root", "bsgs"] or not plan.get("twists"):
        raise RuntimeError("unexpected native plan axes")
    if plan.get("curve_seeds") != 1 or plan.get("generators") != 2 or plan.get("keys") != 2 or plan.get("seed") != 20261031 or plan.get("factor_base_size") != 8 or plan.get("degree_bound") != 3 or plan.get("work_limit") != 20000:
        raise RuntimeError("unexpected native population size")
    native_pre = subprocess.run([str(BINARY), "dataset", "preflight", "--plan", str(HERE / "plan-v2.json"), "--protocol", str(HERE / "protocol-v2.json")], capture_output=True, text=True, timeout=30, check=True)
    preflight = json.loads(native_pre.stdout)
    if preflight.get("source_commit") != SOURCE or preflight.get("source_dirty") is not False or preflight.get("binary_sha256") != HASHES[BINARY] or preflight.get("plan_file_sha256") != HASHES[HERE / "plan-v2.json"] or preflight.get("protocol_file_sha256") != HASHES[HERE / "protocol-v2.json"] or preflight.get("planned_cells") != 16 or preflight.get("generated_cells") != 0:
        raise RuntimeError("native preflight identity or scope mismatch")
    fs = command(["findmnt", "-n", "-o", "FSTYPE", "--target", "/mnt/spot-codex-state/tmp"], 10)
    out = subprocess.check_output(["findmnt", "-n", "-o", "FSTYPE", "--target", "/mnt/spot-codex-state/tmp"], timeout=10).decode().strip()
    if fs["exit_code"] != 0 or out != "xfs":
        raise RuntimeError("local output is not XFS")
    if not Path(AWS_ENV["AWS_CONFIG_FILE"]).is_file():
        raise RuntimeError("checkpoint AWS config unavailable")
    if not (HERE / "STORAGE-STOP-01.json").is_file() or any(not (HERE / "checkpoints" / f"shard-{i:02d}.json").is_file() for i in range(11)):
        raise RuntimeError("v1 stop record or completed checkpoint missing")
    if Path("/var/tmp/ml-dataset-binary-root-bsgs-20261031/shard-11/attempt-start.json").exists():
        raise RuntimeError("v1 index 11 already attempted; reconcile before v2 start")
    return {"schema": "ml-dataset.binary-root-bsgs-campaign-admission/v1", "planned_cells": 16, "plan_sha256": HASHES[HERE / "plan-v2.json"], "protocol_file_sha256": HASHES[HERE / "protocol-v2.json"], "source_commit": SOURCE, "binary_sha256": HASHES[BINARY], "runner_sha256": HASHES[RUNNER], "store_sha256": HASHES[STORE], "filesystem": out, "free_bytes": shutil.disk_usage("/mnt/spot-codex-state/tmp").free, "scientific_computation_performed": False}


def attempt(shard, index):
    marker = shard / "attempt-start.json"
    runner_dir = shard / "runner"
    if not marker.exists():
        if runner_dir.exists():
            raise RuntimeError(f"unmarked runner directory: {runner_dir}")
        if shutil.disk_usage("/mnt/spot-codex-state/tmp").free < 4 * 1024 ** 3:
            raise RuntimeError("less than 4 GiB XFS free; stop before new attempt")
        write_json(marker, {"schema": "ml-dataset.binary-root-bsgs-campaign-attempt/v1", "index": index, "protocol_file_sha256": HASHES[HERE / "protocol-v2.json"], "plan_file_sha256": HASHES[HERE / "plan-v2.json"], "created_unix_ns": str(time.time_ns()), "one_attempt_only": True})
        argv = [RUNNER, BINARY, SOURCE, f"binary23-root-bsgs-{index:02d}", "90", runner_dir, "--", "generate", "--plan", HERE / "plan-v2.json", "--protocol", HERE / "protocol-v2.json", "--out", runner_dir / "attempt", "--max-cases", "1", "--shard-index", str(index), "--shard-count", "16", "--seconds-per-case", "30", "--memory-mib", "2048", "--trace-mib", "64"]
        result = command(argv, 115, shard / "launcher.stdout")
        write_json(shard / "launcher.json", result)
    elif not (shard / "launcher.json").exists():
        if active_worker(shard):
            raise RuntimeError(f"active worker for shard {index}")
        write_json(shard / "launcher.json", {"schema": "ml-dataset.binary-root-bsgs-campaign-interrupted/v1", "index": index, "interrupted_before_launcher_receipt": True, "exit_code": None})
    return json.loads((shard / "launcher.json").read_text())


def cell_spec(index):
    # Native DatasetPlan.cell mixed-radix order: solver, twist, m, key,
    # generator, curve seed, n, family. The other axes are singletons here.
    return {"solver": ["root", "bsgs"][index % 2],
            "twist": bool((index // 2) % 2),
            "key_index": (index // 4) % 2,
            "generator_index": (index // 8) % 2}


def finalize(shard, index):
    if active_worker(shard):
        raise RuntimeError(f"active worker for shard {index}")
    native = shard / "runner" / "attempt"
    status = {"index": index, "cell_spec": cell_spec(index)}
    if native.is_dir():
        for name, args in [
            ("verify", ["verify", "--out", native]),
            ("report", ["report", "--out", native, "--destination", shard / "report"]),
            ("export", ["export", "--out", native, "--destination", shard / "export", "--cyclic-subgroup-ecdlp"]),
        ]:
            try:
                receipt = stage(shard, name, [BINARY, "dataset"] + args, 60)
                status[name] = receipt["exit_code"]
                status[name + "_timed_out"] = receipt["timed_out"]
            except RuntimeError:
                status[name] = "interrupted_unreceipted"
                status[name + "_timed_out"] = None
    else:
        for name in ("verify", "report", "export"):
            status[name] = "missing_native_attempt"
            status[name + "_timed_out"] = None
    status["runner_receipt_present"] = (shard / "runner" / "receipt.json").is_file()
    status["launcher"] = json.loads((shard / "launcher.json").read_text())
    status["artifact_hashes_verified"] = False
    status["sealed_cell_verified"] = False
    status["independently_solved_ecdlp_cells"] = None
    status["root_certificate_replayed"] = False
    status["root_queries_completed"] = None
    status["root_censored"] = None
    status["root_found_pdp"] = None
    status["scalar_recovery_label"] = None
    if status["verify"] == 0:
        try:
            verified = json.loads((shard / "verify.stdout").read_text())
            status["artifact_hashes_verified"] = verified.get("artifact_hashes_verified") is True
            native_cells = native / "cells" / f"{index:012d}"
            status["sealed_cell_verified"] = (status["artifact_hashes_verified"] and verified.get("completed_shard_receipts") == 1 and (native_cells / "manifest.json").is_file())
            manifest = json.loads((native_cells / "manifest.json").read_text())
            if manifest.get("config", {}).get("solver") != status["cell_spec"]["solver"] or manifest.get("config", {}).get("twist") != status["cell_spec"]["twist"]:
                raise RuntimeError("native manifest solver/twist differs from frozen plan")
            if status["cell_spec"]["solver"] == "root":
                # Native verify replays the registered root certificate. A
                # censored call may still have checked completed queries, but
                # never promotes a full-domain negative or ECDLP scalar label.
                roots = [a for a in manifest.get("artifacts", []) if a.get("role") == "binary-root-certificates/v1"]
                if len(roots) == 1 and status["artifact_hashes_verified"]:
                    cert = json.loads((native_cells / roots[0]["path"]).read_text())
                    status["root_certificate_replayed"] = cert.get("distinct_root_certificates_verified") is True
                    status["root_queries_completed"] = cert.get("root_queries_completed")
                    status["root_censored"] = cert.get("censored")
                    status["root_found_pdp"] = cert.get("found_pdp")
                status["independently_solved_ecdlp_cells"] = 0
            else:
                count = verified.get("independently_solved_ecdlp_cells", 0)
                status["independently_solved_ecdlp_cells"] = count
                status["scalar_recovery_label"] = bool(count == 1 and status["artifact_hashes_verified"])
        except (ValueError, OSError, KeyError, TypeError, RuntimeError) as exc:
            status["classification_error"] = str(exc)
    status["verified_and_exported"] = (all(status.get(k) == 0 for k in ("verify", "report", "export")) and status["sealed_cell_verified"])
    status["root_certificate_promoted"] = (status["sealed_cell_verified"] and status["cell_spec"]["solver"] == "root" and status["root_certificate_replayed"] and status["root_queries_completed"] is True and status["root_censored"] is False)
    # A checked positive PDP witness may stop before every root query is
    # visited. Keep that narrower positive label separate from full completion.
    status["root_positive_pdp_promoted"] = (status["sealed_cell_verified"] and status["cell_spec"]["solver"] == "root" and status["root_certificate_replayed"] and status["root_found_pdp"] is True)
    status["ecdlp_solve_promoted"] = (status["sealed_cell_verified"] and status["cell_spec"]["solver"] == "bsgs" and status["scalar_recovery_label"] is True)
    # Custody is attempted regardless of these labels or any failed stage.
    write_json(shard / "local-status.json", status)
    return status


def custody(shard, index, status):
    archive_dir = ROOT / "archives"
    archive_dir.mkdir(exist_ok=True)
    archive = archive_dir / f"shard-{index:02d}.tar.gz"
    if not archive.exists():
        temp = archive.with_name(archive.name + ".tmp-" + uuid.uuid4().hex)
        packed = command(["tar", "-C", ROOT, "-czf", temp, shard.name], 120)
        if packed["exit_code"] != 0 or not temp.is_file():
            raise RuntimeError(f"archive creation failed for shard {index}; partial temp retained")
        os.replace(temp, archive)
    sha = digest(archive)
    size = archive.stat().st_size
    env = os.environ.copy()
    env.update(AWS_ENV)
    logical = f"restricted/ml-dataset-binary-root-bsgs-mnt-v2-20261031/shard-{index:02d}.tar.gz"
    store_receipt = shard.parent / "custody" / f"shard-{index:02d}-store.json"
    store_receipt.parent.mkdir(exist_ok=True)
    if not store_receipt.exists():
        store_cmd = [STORE, "store", "--json", "push", archive, "--name", logical, "--description", "Restricted native local-XFS ordinary binary root or BSGS cell including witnesses and traces", "--provenance", f"frozen protocol {HASHES[HERE / 'protocol-v2.json']} shard {index} source {SOURCE}"]
        pushed = command(store_cmd, 120, shard.parent / "custody" / f"shard-{index:02d}-store.stdout", env)
        if pushed["exit_code"] != 0:
            raise RuntimeError(f"S3 store failed for shard {index}; local archive retained")
        write_json(store_receipt, pushed)
    key = f"store/sha256/{sha[:2]}/{sha}"
    download = shard.parent / "custody" / f"shard-{index:02d}-fresh-{uuid.uuid4().hex}.tar.gz"
    got = command(["aws", "s3api", "get-object", "--bucket", BUCKET, "--key", key, download], 120, shard.parent / "custody" / f"shard-{index:02d}-get.stdout", env)
    if got["exit_code"] != 0 or not download.is_file() or digest(download) != sha or download.stat().st_size != size:
        raise RuntimeError(f"fresh S3 readback mismatch for shard {index}")
    download.unlink()
    receipt = {"schema": "ml-dataset.binary-root-bsgs-campaign-cell-custody/v1", "index": index, "archive_sha256": sha, "archive_bytes": size, "s3_uri": f"s3://{BUCKET}/{key}", "logical_name": logical, "fresh_get_byte_comparison": True, "native_status": status, "protocol_file_sha256": HASHES[HERE / "protocol-v2.json"], "plan_file_sha256": HASHES[HERE / "plan-v2.json"]}
    local = shard.parent / "custody" / f"shard-{index:02d}.json"
    if local.exists() and json.loads(local.read_text()) != receipt:
        raise RuntimeError("immutable local cell receipt differs")
    if not local.exists():
        write_json(local, receipt)
    # Store the small discovery receipt via the same approved content-addressed
    # prefix. The logical-name index is checkpointed to Git by the campaign owner.
    index_name = f"evidence/ml-dataset-binary-root-bsgs-mnt-v2-20261031/shard-{index:02d}-custody.json"
    pushed_index = command([STORE, "store", "--json", "push", local, "--name", index_name, "--description", "Local dataset cell custody pointer", "--provenance", f"protocol {HASHES[HERE / 'protocol-v2.json']} shard {index}"], 120, shard.parent / "custody" / f"shard-{index:02d}-index-put.stdout", env)
    if pushed_index["exit_code"] != 0:
        raise RuntimeError(f"custody pointer store failed for shard {index}")
    pointer_sha = digest(local)
    key2 = f"store/sha256/{pointer_sha[:2]}/{pointer_sha}"
    copied = shard.parent / "custody" / f"index-get-{index:02d}-{uuid.uuid4().hex}.json"
    get = command(["aws", "s3api", "get-object", "--bucket", BUCKET, "--key", key2, copied], 60, shard.parent / "custody" / f"shard-{index:02d}-index-get.stdout", env)
    if get["exit_code"] != 0 or not copied.is_file() or copied.read_bytes() != local.read_bytes():
        raise RuntimeError(f"fixed cell index S3 readback mismatch for shard {index}")
    copied.unlink()
    write_json(shard.parent / "custody" / f"shard-{index:02d}-published.json", {"schema": "ml-dataset.binary-root-bsgs-campaign-published/v1", "archive_sha256": sha, "pointer_sha256": pointer_sha, "archive_fresh_get": True, "pointer_fresh_get": True})
    return receipt


def git_ready_checkpoint(index):
    pointer = ROOT / "custody" / f"shard-{index:02d}.json"
    published = ROOT / "custody" / f"shard-{index:02d}-published.json"
    if not pointer.is_file() or not published.is_file():
        raise RuntimeError(f"missing local custody checkpoint for shard {index}")
    saved = HERE / "checkpoints" / f"shard-{index:02d}.json"
    content = {"schema": "ml-dataset.binary-root-bsgs-campaign-git-checkpoint/v1", "index": index, "custody": json.loads(pointer.read_text()), "published": json.loads(published.read_text()), "store_index_sha256": digest(ROOT / "store-index.json")}
    if saved.exists():
        if json.loads(saved.read_text()) != content:
            # The mutable logical-name index gains later entries. The original
            # per-cell snapshot is immutable and remains the Git checkpoint.
            old = json.loads(saved.read_text())
            content["store_index_sha256"] = old.get("store_index_sha256")
            if old != content:
                raise RuntimeError(f"Git checkpoint differs for shard {index}")
    else:
        write_json(saved, content)


def run(max_new_cells):
    if max_new_cells is None or not 1 <= max_new_cells <= 5:
        raise RuntimeError("run requires --max-new-cells in 1..5")
    admission_receipt = admission()
    ROOT.mkdir(mode=0o700, parents=True, exist_ok=True)
    os.chmod(ROOT, 0o700)
    lock = open(ROOT / "campaign.lock", "a+")
    try:
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
    except BlockingIOError:
        raise RuntimeError("another campaign controller owns the lock")
    if not (ROOT / "admission.json").exists():
        admission_receipt["campaign_started_unix_ns"] = str(time.time_ns())
        write_json(ROOT / "admission.json", admission_receipt)
    admitted = json.loads((ROOT / "admission.json").read_text())
    if admitted.get("protocol_file_sha256") != HASHES[HERE / "protocol-v2.json"] or admitted.get("plan_sha256") != HASHES[HERE / "plan-v2.json"]:
        raise RuntimeError("existing campaign admission binds different frozen inputs")
    deadline_ns = int(admitted["campaign_started_unix_ns"]) + 12 * 3600 * 10 ** 9
    processed = 0
    for index in range(11, 16):
        shard = ROOT / f"shard-{index:02d}"
        cell_receipt = ROOT / "custody" / f"shard-{index:02d}-published.json"
        if cell_receipt.exists():
            published = json.loads(cell_receipt.read_text())
            pointer = ROOT / "custody" / f"shard-{index:02d}.json"
            archive = ROOT / "archives" / f"shard-{index:02d}.tar.gz"
            if not pointer.is_file() or not archive.is_file() or digest(pointer) != published.get("pointer_sha256") or digest(archive) != published.get("archive_sha256"):
                raise RuntimeError(f"published shard {index} has a local custody mismatch")
            git_ready_checkpoint(index)
            continue
        if processed >= max_new_cells:
            print(json.dumps({"planned": 16, "new_or_resumed_cells_processed": processed,
                              "complete": False, "stop_reason": "between_cell_wave_limit"}), flush=True)
            return
        if time.time_ns() > deadline_ns:
            raise RuntimeError("12-hour campaign admission ceiling reached")
        shard.mkdir(exist_ok=True)
        attempt(shard, index)
        status = finalize(shard, index)
        receipt = custody(shard, index, status)
        git_ready_checkpoint(index)
        processed += 1
        print(json.dumps({"index": index, "verified_and_exported": status["verified_and_exported"], "root_certificate_promoted": status["root_certificate_promoted"], "root_positive_pdp_promoted": status["root_positive_pdp_promoted"], "ecdlp_solve_promoted": status["ecdlp_solve_promoted"], "archive_sha256": receipt["archive_sha256"], "fresh_get": True}), flush=True)
    print(json.dumps({"planned": 16, "custody_receipts": 5, "complete": True}), flush=True)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("mode", choices=["check", "run"])
    parser.add_argument("--max-new-cells", type=int,
                        help="Required for run: at most this many unpublished cells, including resumed finalizations; stop only after each cell custody checkpoint")
    args = parser.parse_args()
    try:
        if args.mode == "check":
            print(json.dumps(admission(), sort_keys=True))
        else:
            run(args.max_new_cells)
    except Exception as exc:
        print(f"campaign stopped: {exc}", file=sys.stderr)
        sys.exit(1)
