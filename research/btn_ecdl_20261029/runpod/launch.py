"""Run gpujob.sh for the BTN prototype on a RunPod GPU pod.

    export RUNPOD_API_KEY=...   # plus AWS credentials for the S3 result channel
    python3 launch.py --list-gpus
    python3 launch.py --out /tmp/btn-runpod-1 [--gpu-match "L40S"] [--env TOY_SEEDS=1]

Reuses the container contract of ecc2k130/runpod_job.py (CUDA image, source
tarball and results through presigned S3 URLs, pod terminated in `finally`).
Only research/btn_ecdl_20261029/ is shipped.
"""
import argparse, hashlib, json, pathlib, subprocess, sys, tarfile, time, uuid

HERE = pathlib.Path(__file__).resolve().parent
SRC = HERE.parent
REPO = SRC.parents[1]
sys.path.insert(0, str(REPO / "ecc2k130"))
import runpod_job  # noqa: E402  start_command, CUDA_IMAGE


def archive(path):
    rev = subprocess.run(["git", "rev-parse", "HEAD"], cwd=SRC, capture_output=True, text=True).stdout.strip()
    dirty = bool(subprocess.run(["git", "status", "--porcelain", "--", "."], cwd=SRC,
                                capture_output=True, text=True).stdout.strip())
    skip = lambda i: None if "__pycache__" in i.name or i.name.endswith((".pyc", ".log")) else i
    with tarfile.open(path, "w:gz") as tar:
        for p in sorted(SRC.glob("*.py")):
            tar.add(p, arcname=p.name, filter=skip)
        tar.add(HERE / "gpujob.sh", arcname="runpod/gpujob.sh")
        marker = path.parent / "SOURCE_REV"
        marker.write_text("%s%s\n" % (rev, " (uncommitted changes)" if dirty else ""))
        tar.add(marker, arcname="SOURCE_REV")
    return hashlib.sha256(path.read_bytes()).hexdigest(), rev, dirty


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--out", type=pathlib.Path)
    ap.add_argument("--gpu-type", default="")
    ap.add_argument("--gpu-match", default="L40S")
    ap.add_argument("--cloud-type", default="SECURE", choices=("SECURE", "COMMUNITY", "ALL"))
    ap.add_argument("--cuda-versions", default="13.0,13.1,13.2,13.3,13.4")
    ap.add_argument("--env", action="append", default=[], help="KEY=VALUE for gpujob.sh (TOY_SEEDS, MEM_GB, STEP_MINUTES)")
    ap.add_argument("--job-minutes", type=int, default=330)
    ap.add_argument("--wait-minutes", type=int, default=360)
    ap.add_argument("--s3-bucket", default="")
    ap.add_argument("--list-gpus", action="store_true")
    ap.add_argument("--dry-run", action="store_true", help="build the archive and start command, launch nothing")
    a = ap.parse_args()
    if a.dry_run:
        out = (a.out or pathlib.Path("/tmp/btn-runpod-dry")).resolve(); out.mkdir(parents=True, exist_ok=True)
        sha, rev, dirty = archive(out / "source.tgz")
        urls = {k: "https://example.invalid/" + k for k in ("source", "results", "done")}
        cmd = runpod_job.start_command(urls, sha, "runpod/gpujob.sh", [tuple(kv.split("=", 1)) for kv in a.env], a.job_minutes)
        (out / "start_command.txt").write_text(cmd + "\n")
        with tarfile.open(out / "source.tgz") as t: names = t.getnames()
        print(json.dumps(dict(sourceSha256=sha, gitRev=rev, gitDirty=dirty, files=names, image=runpod_job.CUDA_IMAGE), indent=1))
        return 0
    if not a.list_gpus and a.out is None:
        ap.error("--out is required")
    # Delegate the pod lifecycle to runpod_job.main with our archive builder and job path.
    runpod_job.ROOT = SRC
    runpod_job.source_archive = lambda path, job, extra: archive(path)
    argv = ["runpod_job.py", "--job", "runpod/gpujob.sh", "--out", str(a.out or "/tmp/unused"),
            "--gpu-match", a.gpu_match, "--cloud-type", a.cloud_type, "--cuda-versions", a.cuda_versions,
            "--job-minutes", str(a.job_minutes), "--wait-minutes", str(a.wait_minutes)]
    if a.gpu_type: argv += ["--gpu-type", a.gpu_type]
    if a.s3_bucket: argv += ["--s3-bucket", a.s3_bucket]
    if a.list_gpus: argv += ["--list-gpus"]
    for kv in a.env: argv += ["--env", kv]
    sys.argv = argv
    return runpod_job.main()


if __name__ == "__main__":
    raise SystemExit(main())
