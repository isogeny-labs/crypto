"""Run gpu/modal/ic_baseline_job.sh (semaev + rootfind hardware baseline) on a Modal GPU.

    modal run --detach gpu/modal/modal_ic_job.py --out /tmp/gpu-ic-baseline
    modal run gpu/modal/modal_ic_job.py --fetch-token <token> --out /tmp/gpu-ic-baseline

Same container contract and Volume-collection pattern as ecc2k130/modal_job.py,
but the mounted tree is gpu/ (at /work/gpu) so the job can build gpu/semaev and
gpu/rootfind.  The image is nvidia/cuda:13.3.1-devel (clmad needs CUDA 13.3+).
Modal gives no exclusive-host or clock-lock guarantee: the job records SM clocks
and power every 500 ms and the protocol treats a round below 90% of max SM clock
as invalid.  Credentials: MODAL_TOKEN_ID and MODAL_TOKEN_SECRET in the environment.
"""
import json
import os
import pathlib
import subprocess
import tarfile
import time
import uuid

import modal

GPU_DIR = pathlib.Path(__file__).resolve().parents[1]
REPO = GPU_DIR.parent
REMOTE = "/work/gpu"
DATA = "/data"
CUDA_VERSION = os.environ.get("IC_CUDA_VERSION", "13.3.1")
DEFAULT_GPU = os.environ.get("IC_GPU", "RTX-PRO-6000")
HOUR = 60 * 60

image = (
    modal.Image.from_registry(f"nvidia/cuda:{CUDA_VERSION}-devel-ubuntu24.04", add_python="3.12")
    .entrypoint([])
    .apt_install("build-essential", "make", "g++", "python3")
    .add_local_dir(GPU_DIR, remote_path=REMOTE, copy=True,
                   ignore=["**/__pycache__", "**/*.pyc", "**/target", "**/benchmarks", "**/*.o", "**/*.a"])
)
volume = modal.Volume.from_name("gpu-ic-baseline-jobs", create_if_missing=True)
app = modal.App(os.environ.get("IC_MODAL_APP", "gpu-ic-baseline"))


@app.function(image=image, gpu=DEFAULT_GPU, cpu=8, timeout=2 * HOUR, volumes={DATA: volume})
def run(env: dict, token: str) -> dict:
    results = "/results"
    os.makedirs(results, exist_ok=True)
    full = dict(os.environ, RESULTS=results, TOKEN=token, **env)
    started = time.time()
    with open(os.path.join(results, "job.log"), "w") as log:
        p = subprocess.Popen(["bash", f"{REMOTE}/modal/ic_baseline_job.sh"], cwd=REMOTE, env=full,
                             stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, bufsize=1)
        for line in p.stdout:
            print(line.rstrip(), flush=True)
            log.write(line)
        rc = p.wait()
    pathlib.Path(results, "exit-code").write_text("%d\n" % rc)
    gpu = subprocess.run("nvidia-smi --query-gpu=name,driver_version --format=csv,noheader",
                         shell=True, capture_output=True, text=True).stdout.strip()
    meta = {"exitCode": rc, "seconds": time.time() - started, "gpu": gpu, "token": token}
    out = os.path.join(DATA, token)
    os.makedirs(out, exist_ok=True)
    with tarfile.open(os.path.join(out, "results.tgz"), "w:gz") as archive:
        archive.add(results, arcname=".")
    pathlib.Path(out, "done.json").write_text(json.dumps(meta))
    volume.commit()
    return meta


def fetch(token: str, outdir: pathlib.Path):
    try:
        meta = json.loads(b"".join(volume.read_file(f"{token}/done.json")).decode())
    except Exception:
        return None
    (outdir / "results.tgz").write_bytes(b"".join(volume.read_file(f"{token}/results.tgz")))
    destination = (outdir / "results").resolve()
    destination.mkdir(parents=True, exist_ok=True)
    with tarfile.open(outdir / "results.tgz") as archive:
        members = archive.getmembers()
        for m in members:  # regular files and directories only, nothing escaping the destination
            if m.issym() or m.islnk() or m.isdev() or \
               os.path.commonpath((str(destination), str((destination / m.name).resolve()))) != str(destination):
                raise ValueError(f"unsafe result archive member: {m.name}")
        archive.extractall(destination, members=members)
    return meta


@app.local_entrypoint()
def main(out: str = "/tmp/gpu-ic-baseline", gpu: str = "", env: str = "", fetch_token: str = ""):
    """--env takes comma-separated KEY=VALUE pairs for the job (SEM_LS="12 14", REPS=5, ARCH=sm_120)."""
    outdir = pathlib.Path(out)
    if fetch_token:
        outdir.mkdir(parents=True, exist_ok=True)
        meta = fetch(fetch_token, outdir)
        if meta is None:
            raise SystemExit("no finished run under token %s yet" % fetch_token)
        print("job exit code", meta["exitCode"], "on", meta["gpu"], "- results in", outdir / "results")
        raise SystemExit(1 if meta["exitCode"] else 0)
    rev = subprocess.run(["git", "rev-parse", "HEAD"], cwd=REPO, capture_output=True, text=True).stdout.strip()
    dirty = bool(subprocess.run(["git", "status", "--porcelain", "--", "gpu"], cwd=REPO,
                                capture_output=True, text=True).stdout.strip())
    environment = {"SOURCE_REV": rev + (" (uncommitted changes)" if dirty else "")}
    for item in filter(None, env.split(",")):
        key, _, value = item.partition("=")
        environment[key] = value
    fn = run if not gpu or gpu == DEFAULT_GPU else run.with_options(gpu=gpu)
    outdir.mkdir(parents=True, exist_ok=False)
    token = uuid.uuid4().hex
    receipt = {"job": "gpu/modal/ic_baseline_job.sh", "gpu": gpu or DEFAULT_GPU,
               "cudaImage": f"nvidia/cuda:{CUDA_VERSION}-devel-ubuntu24.04", "gitRev": rev, "gitDirty": dirty,
               "env": environment, "provider": "modal", "token": token, "volume": "gpu-ic-baseline-jobs",
               "startedAt": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())}
    (outdir / "launch.json").write_text(json.dumps(receipt, indent=2) + "\n")
    call = fn.spawn(environment, token)
    receipt["functionCallId"] = call.object_id
    (outdir / "launch.json").write_text(json.dumps(receipt, indent=2) + "\n")
    print("spawned", call.object_id, "token", token, "- if this client dies: modal run gpu/modal/modal_ic_job.py "
          "--fetch-token", token, "--out", out, flush=True)
    while True:
        try:
            meta = call.get(timeout=60)
            break
        except TimeoutError:
            continue
        except Exception as exc:  # noqa: BLE001 - transient client link errors
            print(time.strftime("%H:%M:%S"), "client error, retrying:", type(exc).__name__, flush=True)
            time.sleep(30)
    meta = fetch(token, outdir) or meta
    receipt.update(exitCode=meta["exitCode"], seconds=meta["seconds"], deviceLine=meta["gpu"],
                   finishedAt=time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()))
    (outdir / "launch.json").write_text(json.dumps(receipt, indent=2) + "\n")
    print("job exit code", meta["exitCode"], "on", meta["gpu"], "- results in", outdir / "results")
    raise SystemExit(1 if meta["exitCode"] else 0)
