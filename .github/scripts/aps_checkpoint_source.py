#!/usr/bin/env python3
"""Custody checks for an approved source checkpoint; never runs an experiment.

This administrative helper selects source bytes. The unchanged standard
checkpoint script performs authentication, transfer and the mandatory gate.
"""
import argparse
import base64
import datetime as dt
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import re
import shutil
import subprocess
from urllib.parse import urlsplit

STUDY = "research/aps_cost_leads_20261106/"
SELECTION = STUDY + "CHECKPOINT_SELECTION.json"
ADMISSION = STUDY + "CHECKPOINT_ADMISSION.json"
CAPTURE = "aps-checkpoint-source"
DESTINATION = "s3://setec-astronomy-artifacts/checkpoints/crypto/"
REGION = r"[a-z]{2}-[a-z]+-[0-9]"
EXACT = {
    ".github/workflows/aps-source-checkpoint.yml",
    ".github/workflows/aps-export.yml",
    ".github/workflows/aps-single-relation.yml",
    ".github/scripts/aps_checkpoint_source.py",
    ".github/scripts/test_aps_checkpoint_source.py",
    "docs/ecbench/README.md",
    "problems/aps-single-relation-complete-cost.md",
    "src/cryptanalysis/aps_single_relation.rs",
    "src/cryptanalysis/aps_single_relation/leads.rs",
    "src/cryptanalysis/f4_fp.rs",
    "src/cryptanalysis/f4_fp/reuse.rs",
    "src/cryptanalysis/ic_measurement.rs",
    "src/cryptanalysis/ecbench/callgrind.rs",
    "src/cryptanalysis/ecbench/methods.rs",
    "src/cryptanalysis/ecbench/optimization.rs",
    "src/cryptanalysis/ecbench/registry_audit_tests.rs",
    "tests/aps_cost_leads.rs",
    "examples/aps_scalar_check.rs",
}


def require(condition, message):
    if not condition:
        raise ValueError(message)


def digest(body):
    return hashlib.sha256(body).hexdigest()


def git(root, *args):
    return subprocess.check_output(["git", "-C", str(root), *args], text=True).strip()


def allowed(name):
    path = PurePosixPath(name)
    return (bool(re.fullmatch(r"[A-Za-z0-9_./-]+", name))
            and not path.is_absolute() and ".." not in path.parts
            and "." not in name.split("/")
            and (name in EXACT or name.startswith(STUDY)
                 or name.startswith("tools/aps-export/")
                 or name.startswith("src/cryptanalysis/ecbench/aps/")))


def deadline(admission, now=None, margin=180):
    require(isinstance(admission, dict) and set(admission) == {
        "schema", "task_ref", "authorization_ref", "native_fence_sha256", "lease_expires_at"},
        "checkpoint admission contains unsupported metadata")
    require(admission.get("schema") == "aps.checkpoint-admission/v1"
            and admission.get("task_ref") == "T-10"
            and admission.get("authorization_ref") == "user-explicit-2026-10-30"
            and re.fullmatch(r"[0-9a-f]{64}", admission.get("native_fence_sha256", "")),
            "missing scoped checkpoint admission")
    expires = dt.datetime.fromisoformat(admission["lease_expires_at"].replace("Z", "+00:00"))
    require(expires.tzinfo is not None, "lease deadline must have a timezone")
    now = now or dt.datetime.now(dt.timezone.utc)
    require((expires - now).total_seconds() >= margin,
            "fenced upload window expired or too short; no transfer permitted")
    return expires


def token_destination():
    require(os.environ.get("CHECKPOINT_BUCKET", "setec-astronomy-artifacts")
            == "setec-astronomy-artifacts"
            and os.environ.get("CHECKPOINT_PREFIX", "checkpoints") == "checkpoints"
            and re.fullmatch(REGION, os.environ.get("CHECKPOINT_AWS_REGION", "us-east-2")),
            "checkpoint namespace differs from the approved standard gate")
    token = os.environ.get("CHECKPOINT_UPLOAD_TOKEN", "")
    if not token:
        return  # The standard gate must report missing authentication itself.
    try:
        value = json.loads(base64.b64decode(token, validate=True))
        url = urlsplit(value["url"])
        require(url.scheme == "https" and url.username is None and url.password is None
                and url.port in (None, 443) and not url.query and not url.fragment,
                "invalid token endpoint")
        virtual = (url.hostname == "setec-astronomy-artifacts.s3.amazonaws.com"
                   or re.fullmatch(r"setec-astronomy-artifacts\.s3\." + REGION
                                   + r"\.amazonaws\.com", url.hostname or "")) and url.path in ("", "/")
        path_style = (re.fullmatch(r"s3\." + REGION + r"\.amazonaws\.com", url.hostname or "")
                      and url.path.rstrip("/") == "/setec-astronomy-artifacts")
        require(virtual or path_style, "token destination differs from approved bucket")
        # The stock uploader passes these as curl -F arguments. Accept only
        # its native presigner's fields, with no key/file override or curl
        # file-reading / multipart syntax hidden in a value.
        fields = value["fields"]
        patterns = {
            "x-amz-algorithm": r"AWS4-HMAC-SHA256",
            "x-amz-credential": r"[A-Za-z0-9/+=._-]{1,1024}",
            "x-amz-date": r"[0-9]{8}T[0-9]{6}Z",
            "policy": r"[A-Za-z0-9/+=]{1,32768}",
            "x-amz-signature": r"[0-9a-f]{64}",
        }
        require(isinstance(fields, dict)
                and set(patterns) <= set(fields)
                and set(fields) <= set(patterns) | {"x-amz-security-token"},
                "token has unsupported form fields")
        patterns["x-amz-security-token"] = r"[A-Za-z0-9/+=]{1,32768}"
        require(all(isinstance(body, str) and re.fullmatch(patterns[name], body)
                    for name, body in fields.items()), "token has unsafe form values")
        require(value.get("prefix") == "checkpoints/", "token prefix differs from standard gate")
    except (ValueError, KeyError, TypeError):
        raise ValueError("checkpoint token binding has an invalid destination or format") from None


def selected(root, manifest):
    require(manifest.get("schema") == "aps.checkpoint-selection/v1", "wrong selection schema")
    rows = manifest.get("files", [])
    require(isinstance(rows, list) and 1 <= len(rows) <= 128, "invalid source selection size")
    names, total = set(), 0
    result = []
    for row in rows:
        require(isinstance(row, dict) and set(row) == {"path", "sha256"},
                "source selection row contains unsupported metadata")
        name = row["path"]
        require(allowed(name) and name not in names and name != SELECTION,
                "source selection includes a denied, duplicate or recursive path")
        names.add(name)
        source = root / name
        require(source.is_file() and not source.is_symlink()
                and source.resolve().is_relative_to(root.resolve()), "source escapes checkout")
        body = source.read_bytes()
        total += len(body)
        require(total <= 8 * 1024 * 1024, "source selection exceeds 8 MiB")
        require(digest(body) == row["sha256"], "selected source digest changed")
        result.append((name, source))
    require(ADMISSION in names and STUDY + "CHECKPOINT_RESUME.md" in names,
            "selection omits the admission or resume card")
    return result


def prepare(root, source):
    root = root.resolve()
    require(re.fullmatch(r"[0-9a-f]{40}", source), "source commit must be exact")
    require(git(root, "rev-parse", "HEAD") == source, "checkout differs from admitted source")
    require(git(root, "config", "--get", "remote.origin.url") in {
        "https://github.com/isogeny-labs/crypto", "https://github.com/isogeny-labs/crypto.git"},
        "origin is not the approved credential-free repository URL")
    require(not git(root, "status", "--porcelain", "--untracked-files=all"),
            "checkpoint worker must start clean")
    require(git(root, "rev-list", "--count", "HEAD", "--not", "--remotes") == "0",
            "unpushed history cannot enter this source-only capture")
    manifest_bytes = (root / SELECTION).read_bytes()
    manifest = json.loads(manifest_bytes)
    require(isinstance(manifest, dict) and set(manifest) == {
        "schema", "baseline_commit", "checkpoint_script_sha256", "files"},
        "source selection contains unsupported metadata")
    baseline = manifest["baseline_commit"]
    require(re.fullmatch(r"[0-9a-f]{40}", baseline), "baseline must be exact")
    subprocess.run(["git", "-C", str(root), "merge-base", "--is-ancestor", baseline, source], check=True)
    require(git(root, "rev-parse", baseline + ":pcc/ledger.jsonl")
            == git(root, "rev-parse", "HEAD:pcc/ledger.jsonl"),
            "native ledger differs from reviewed public baseline")
    require(digest((root / "scripts/checkpoint.sh").read_bytes()) == manifest["checkpoint_script_sha256"],
            "standard checkpoint script changed")
    admission = json.loads((root / ADMISSION).read_bytes())
    deadline(admission)
    token_destination()
    files = selected(root, manifest)
    changed = set(git(root, "diff", "--name-only", baseline, source).splitlines())
    captured = {name for name, _ in files} | {SELECTION}
    omitted = changed - captured
    require(all(re.fullmatch(r"pcc/claims/PCC-20261106-[0-9a-f]{6}\.json", name) for name in omitted)
            and captured <= changed, "selection differs from reviewed task-relative change")
    require(not (root / CAPTURE).exists() and not (root / ".resume").exists(),
            "refusing to overwrite an existing source capture or resume state")
    for name, src in files:
        dest = root / CAPTURE / name
        dest.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(src, dest)
    shutil.copyfile(root / SELECTION, root / CAPTURE / "CHECKPOINT_SELECTION.json")
    # A tracked source such as tools/aps-export/Cargo.lock can become ignored
    # when copied into the untracked snapshot. Keep only the reviewed paths
    # visible to the stock uploader, without changing repository ignore rules.
    capture_names = {name for name, _ in files} | {"CHECKPOINT_SELECTION.json"}
    ignore = ["# Exact reviewed source snapshot; payload closure is checked below.",
              "!/**/", "!/.gitignore"] + ["!/" + name for name in sorted(capture_names)]
    (root / CAPTURE / ".gitignore").write_text("\n".join(ignore) + "\n")
    resume = (root / STUDY / "CHECKPOINT_RESUME.md").read_text().replace("SOURCE_COMMIT", source)
    (root / ".resume").mkdir()
    (root / ".resume/RESUME.md").write_text(resume)
    actual = set(git(root, "ls-files", "--others", "--exclude-standard").splitlines())
    expected = {CAPTURE + "/" + name for name in capture_names} | {CAPTURE + "/.gitignore"}
    require(actual == expected, "untracked payload differs from reviewed source capture")
    return {"schema": "aps.source-checkpoint-preflight/v1", "source_commit": source,
            "selection_sha256": digest(manifest_bytes), "source_files": len(files),
            "pcc_subtree_captured": False, "unpushed_commits": 0,
            "benchmark_launched": False, "lease_expires_at": admission["lease_expires_at"]}


def finish(root, source, gate_exit, log):
    value = {"schema": "aps.source-checkpoint-receipt/v1", "source_commit": source,
             "gate_exit": gate_exit, "checkpoint_uploaded": False,
             "stored_objects_independently_read_back": False,
             "benchmark_launched": False, "execution_attested": False,
             "private_gate_log_sha256": digest(log.read_bytes()) if log.exists() else None}
    if gate_exit == 0:
        parts = (root / ".resume/.last-push").read_text().split()
        require(len(parts) == 3 and re.fullmatch(r"[0-9a-f]{64}", parts[0])
                and parts[2].startswith(DESTINATION), "standard gate lacks its upload receipt")
        value.update(checkpoint_uploaded=True, checkpoint_prefix=parts[2], state_sha256=parts[0])
    else:
        text = log.read_text(errors="replace") if log.exists() else ""
        value["failure_class"] = ("missing_auth" if "no way to upload" in text else
                                  "expired_auth" if "upload token expired" in text else
                                  "deadline" if gate_exit == 124 else "standard_gate_failure")
        value["partial_upload_possible"] = value["failure_class"] not in {"missing_auth", "expired_auth"}
    return value


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("mode", choices=["prepare", "deadline", "finish"])
    parser.add_argument("--source", default="")
    parser.add_argument("--out", type=Path)
    parser.add_argument("--gate-exit", type=int, default=1)
    parser.add_argument("--log", type=Path, default=Path("/nonexistent-private-log"))
    args = parser.parse_args()
    root = Path.cwd()
    if args.mode == "prepare":
        value = prepare(root, args.source)
    elif args.mode == "deadline":
        deadline(json.loads((root / ADMISSION).read_bytes()), margin=150)
        value = {"fenced_upload_window_current": True}
    else:
        value = finish(root, args.source, args.gate_exit, args.log)
    if args.out:
        with args.out.open("x") as output:
            json.dump(value, output, indent=2)
            output.write("\n")


if __name__ == "__main__":
    try:
        main()
    except (ValueError, KeyError, OSError, subprocess.CalledProcessError) as error:
        raise SystemExit(f"APS checkpoint custody refused: {error}") from None
