"""Serial, bounded Docker supervisor for the source-pinned Linux D18 gate."""

import argparse
import datetime
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import time
import traceback

import audit

MEMORY_BYTES = 8 * 1024 ** 3
OUTER_CAP_SECONDS = 1210
MODE = "primary-full-d18-m4-linux-shared-phase-first"
FIXTURE_REL = "research/koblitz_n83_factor_base_sweep_20261008/pilot-01"


def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def write_new(path, data):
    with Path(path).open("x") as stream:
        json.dump(data, stream, indent=2, sort_keys=True)
        stream.write("\n")


def command_output(args):
    return subprocess.check_output(args, text=True).strip()


def sha256(path):
    digest = hashlib.sha256()
    with Path(path).open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def supervise(mode, source, binary, base, image, root, source_commit, binary_sha):
    arm = root / mode
    arm.mkdir(exist_ok=False)
    cidfile = arm / "container-id.txt"
    argv = [
        "docker", "run", "--rm", "--pull", "never", "--platform", "linux/arm64",
        "--cidfile", str(cidfile), "--network", "none", "--cpus", "1",
        "--memory", "8192m", "--memory-swap", "8192m",
        "--pids-limit", "512", "--cap-drop", "ALL", "--security-opt", "no-new-privileges",
        "--read-only", "--tmpfs", "/tmp:rw,size=64m", "--user", f"{os.getuid()}:{os.getgid()}",
        "--mount", f"type=bind,src={source},dst=/src,readonly",
        "--mount", f"type=bind,src={binary},dst=/worker,readonly",
        "--mount", f"type=bind,src={base},dst=/base,readonly",
        "--mount", f"type=bind,src={arm},dst=/out",
        "--env", "GIT_OPTIONAL_LOCKS=0",
        "--workdir", "/src", image,
        "/worker", MODE, mode, "/base", f"/src/{FIXTURE_REL}", "/out/result.json",
    ]
    write_new(arm / "command.json", argv)
    started = now()
    t0 = time.monotonic()
    outer_cap_reached = False
    with (arm / "stdout.json").open("wb") as stdout, (arm / "stderr.txt").open("wb") as stderr:
        process = subprocess.Popen(argv, stdout=stdout, stderr=stderr)
        try:
            exit_code = process.wait(timeout=OUTER_CAP_SECONDS)
        except subprocess.TimeoutExpired:
            outer_cap_reached = True
            if cidfile.exists():
                subprocess.run(["docker", "kill", cidfile.read_text().strip()],
                               stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, check=False)
            process.kill()
            exit_code = process.wait()
    elapsed = time.monotonic() - t0
    ended = now()
    (arm / "start-utc.txt").write_text(started + "\n")
    (arm / "end-utc.txt").write_text(ended + "\n")
    (arm / "exit-code.txt").write_text(str(exit_code) + "\n")
    (arm / "outer-cap.txt").write_text("reached\n" if outer_cap_reached else "not_reached\n")
    supervisor = {
        "schema": "n83.d18-linux-phase-first-supervisor/v1", "mode": mode,
        "source_commit": source_commit, "binary_sha256": binary_sha,
        "started_utc": started, "ended_utc": ended, "elapsed_wall_seconds": elapsed,
        "exit_code": exit_code, "outer_cap_seconds": OUTER_CAP_SECONDS,
        "outer_cap_reached": outer_cap_reached, "memory_limit_bytes": MEMORY_BYTES,
        "cpu_limit": 1, "network": "none", "worker_root_read_only": True,
        "image_ref": image, "selected_best_total_runtime": None,
    }
    write_new(arm / "supervisor.json", supervisor)
    return supervisor


def inventory(root):
    entries = []
    for path in sorted(root.rglob("*")):
        if path.is_file() and path.name != "sha256-inventory.json":
            entries.append({"path": str(path.relative_to(root)), "bytes": path.stat().st_size,
                            "sha256": sha256(path)})
    write_new(root / "sha256-inventory.json",
              {"schema": "n83.d18-linux-phase-first-inventory/v1", "files": entries})


def run(args):
    source, base, binary, root = (Path(value).resolve() for value in
                                  (args.source, args.base, args.binary, args.output))
    root.mkdir(exist_ok=False)
    try:
        source_commit = command_output(["git", "-C", str(source), "rev-parse", "HEAD"])
        if command_output(["git", "-C", str(source), "status", "--porcelain"]):
            raise ValueError("source checkout is not clean")
        binary_sha = sha256(binary)
        identity = audit.verify_input(source, base, binary, source_commit)
        write_new(root / "identity.json", identity)
        image_id = command_output(["docker", "image", "inspect", args.image, "--format", "{{.Id}}"])
        write_new(root / "host.json", {"schema": "n83.d18-linux-phase-first-host/v1",
            "source_commit": source_commit, "binary_sha256": binary_sha,
            "image_ref": args.image, "image_id": image_id,
            "uname": command_output(["uname", "-a"]),
            "docker_server": command_output(["docker", "version", "--format", "{{.Server.Version}}"]),
            "selected_best_total_runtime": None})
        positive = supervise("positive", source, binary, base, args.image, root, source_commit, binary_sha)
        if positive["outer_cap_reached"]:
            outcome = {"status": "UNKNOWN_positive_outer_cap", "positive": None, "natural": None}
        else:
            positive_audit = audit.audit_arm(root / "positive", "positive", source_commit,
                                             binary_sha, base / audit.OBJECT_REL)
            write_new(root / "positive-audit.json", positive_audit)
            if positive_audit["status"] != "PASS_CONSTRUCTED_CONTROL":
                outcome = {"status": "UNKNOWN_positive_gate", "positive": positive_audit,
                           "natural": None}
            else:
                natural = supervise("natural", source, binary, base, args.image, root,
                                    source_commit, binary_sha)
                if natural["outer_cap_reached"]:
                    outcome = {"status": "UNKNOWN_natural_outer_cap", "positive": positive_audit,
                               "natural": None}
                else:
                    natural_audit = audit.audit_arm(root / "natural", "natural", source_commit,
                                                    binary_sha, base / audit.OBJECT_REL)
                    write_new(root / "natural-audit.json", natural_audit)
                    status = ("PASS_SINGLE_NATURAL_RELATION_REPLAY" if natural_audit["relation"]
                              else "UNKNOWN_natural_bounded")
                    outcome = {"status": status, "positive": positive_audit,
                               "natural": natural_audit}
        write_new(root / "outcome.json", {"schema": "n83.d18-linux-phase-first-outcome/v1",
                  "source_commit": source_commit, "binary_sha256": binary_sha,
                  "selected_best_total_runtime": None, **outcome})
        return 0
    except Exception as exc:
        write_new(root / "failure.json", {"schema": "n83.d18-linux-phase-first-failure/v1",
                  "status": "INVALID_RUN", "error_type": type(exc).__name__, "error": str(exc),
                  "traceback": traceback.format_exc(), "selected_best_total_runtime": None})
        return 2
    finally:
        inventory(root)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    for name in ("source", "base", "binary", "image", "output"):
        parser.add_argument("--" + name, required=True)
    raise SystemExit(run(parser.parse_args()))
