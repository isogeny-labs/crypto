"""Replay exact provider attempt 1 and its unapplied, two-key access proposal."""

import hashlib
import json
from pathlib import Path
import zipfile

ROOT = Path(__file__).resolve().parent
RUN_ID = 40220890684
ARTIFACT_ID = 12608973073
SOURCE = "2b7b04e1d613debe9d71c6403afba09ac3d9b289"
ZIP_SHA = "2f417b24cf499cc9d6f30ddb28c05972932d867962fcdb32ac768f6fd7dd5b81"
PREFIX = "arn:aws:s3:::crypto-autoresearcher/factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/d18-full-direct-subgroup-v1/a0/"
RESOURCES = [PREFIX + "manifest.json",
             PREFIX + "objects/c295da8898cfad29a053621c768ca059eea4263f89f5ca78b4f98af43c383f3f.jsonl.gz"]


def read(name):
    return json.loads((ROOT / name).read_text())


def verify():
    run = read("provider-run.json")
    assert run["headSha"] == SOURCE and run["status"] == "completed" and run["conclusion"] == "failure"
    assert run["url"] == f"https://github.com/aburan28/crypto/actions/runs/{RUN_ID}"
    assert len(run["jobs"]) == 1
    steps = {step["name"]: step["conclusion"] for step in run["jobs"][0]["steps"]}
    assert steps["Check out the exact experiment source and protocol"] == "success"
    assert steps["Freeze source, host and image identity"] == "success"
    assert steps["Run aws-actions/configure-aws-credentials@v4"] == "success"
    assert steps["Fresh-fetch and hash the complete S3 base"] == "failure"
    for name in ("Install pinned audit dependency and fetch Rust crates",
                 "Build one source-bound Linux release binary",
                 "Verify the exact worker cgroup before either arm",
                 "Run checked positive gate then one public natural query"):
        assert steps[name] == "skipped"
    assert steps["Preserve setup, custody, raw stages and audited outcome"] == "success"
    artifact = read("provider-artifact.json")["artifacts"]
    assert len(artifact) == 1 and artifact[0]["id"] == ARTIFACT_ID
    assert artifact[0]["name"] == f"n83-d18-phase-first-linux-{RUN_ID}-1"
    assert not artifact[0]["expired"]
    provider_zip = ROOT / "provider.zip"
    assert provider_zip.stat().st_size == artifact[0]["size_in_bytes"] == 2191
    assert hashlib.sha256(provider_zip.read_bytes()).hexdigest() == ZIP_SHA
    expected = {
        "n83-d18-setup/docker-info.txt", "n83-d18-setup/docker-pull.stderr",
        "n83-d18-setup/docker-pull.stdout", "n83-d18-setup/docker-version.txt",
        "n83-d18-setup/source-commit.txt", "n83-d18-setup/image.txt",
        "n83-d18-setup/uname.txt", "n83-d18-base/manifest-get-metadata.json",
    }
    with zipfile.ZipFile(provider_zip) as stream:
        assert set(stream.namelist()) == expected and stream.testzip() is None
        assert stream.read("n83-d18-setup/source-commit.txt").decode().strip() == SOURCE
        assert stream.read("n83-d18-base/manifest-get-metadata.json") == b""
        image_record = stream.read("n83-d18-setup/image.txt").decode().strip()
        assert "arm64" in image_record and "linux" in image_record
    log = (ROOT / "provider-failed-step.log").read_text()
    assert "AccessDenied" in log and "s3:GetObject" in log
    assert "rho-gh-snapshot-readonly-20261024" in log and RESOURCES[0] in log
    assert "AWS_ACCESS_KEY_ID: ***" in log and "AWS_SECRET_ACCESS_KEY: ***" in log
    assert "sig=" not in log and "BEGIN PRIVATE KEY" not in log
    proposal = read("exact-read-policy-proposal.json")
    assert proposal["Version"] == "2012-10-17" and len(proposal["Statement"]) == 1
    statement = proposal["Statement"][0]
    assert statement["Effect"] == "Allow" and statement["Action"] == "s3:GetObject"
    assert statement["Resource"] == RESOURCES
    return {"schema": "n83.d18-linux-phase-first-attempt1-audit/v1",
            "status": "PASS_SOURCE_BOUND_S3_ACCESS_DENIED_BEFORE_WORKER",
            "run_id": RUN_ID, "artifact_id": ARTIFACT_ID, "source_commit": SOURCE,
            "provider_zip_sha256": ZIP_SHA, "provider_zip_bytes": provider_zip.stat().st_size,
            "failed_step": "Fresh-fetch and hash the complete S3 base",
            "failed_resource": RESOURCES[0], "worker_build_executed": False,
            "positive_search_executed": False, "natural_search_executed": False,
            "proposed_policy_status": "NOT_APPLIED", "proposed_policy_resources": RESOURCES,
            "selected_best_total_runtime": None}


if __name__ == "__main__":
    print(json.dumps(verify(), indent=2, sort_keys=True))
