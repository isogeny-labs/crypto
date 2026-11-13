"""Run job_ab.sh (fes-f2-gpu before/after A/B on a CUDA worker) on a Modal GPU.

    IC_BEFORE=/path/ic-before IC_AFTER=/path/ic-after \\
        modal run research/ic_gpu_call_overhead_20261113/modal_ab.py --out /tmp/ab-a100
    modal run research/ic_gpu_call_overhead_20261113/modal_ab.py --fetch-token <token> --out /tmp/ab-a100

Same Volume-collection pattern as gpu/modal/modal_ic_job.py.  The two ic binaries are not in Git
(release builds of the same source); their sha256 is recorded in the results.  Credentials:
MODAL_TOKEN_ID and MODAL_TOKEN_SECRET in the environment; behind a proxy the client needs
`pip install 'modal[api-proxy-support]'`.
"""
import json, os, pathlib, subprocess, tarfile, time, uuid

import modal

HERE = pathlib.Path(__file__).resolve().parent
REPO = HERE.parents[1]
GPU = os.environ.get("AB_GPU", "A100-80GB")
CUDA = os.environ.get("AB_CUDA", "12.8.1")
BEFORE, AFTER = os.environ.get("IC_BEFORE", ""), os.environ.get("IC_AFTER", "")
HOUR = 3600

image = (
    modal.Image.from_registry(f"nvidia/cuda:{CUDA}-devel-ubuntu24.04", add_python="3.12")
    .entrypoint([])
    .apt_install("build-essential", "make", "g++", "python3")
    .add_local_dir(REPO / "gpu" / "fes", "/work/ab/gpu/fes", copy=True,
                   ignore=["fes_solve", "fes_cuda", "test_cpu_main", "*.o", "system.h", "vectors.h"])
    .add_local_dir(HERE, "/work/ab/exp", copy=True, ignore=["raw", "__pycache__", "*.pdf", "*.svg", "results.json"])
)
if BEFORE and AFTER:
    image = image.add_local_file(BEFORE, "/work/ab/ic-before", copy=True).add_local_file(AFTER, "/work/ab/ic-after", copy=True)
volume = modal.Volume.from_name("ic-gpu-call-overhead", create_if_missing=True)
app = modal.App("ic-gpu-call-overhead")


@app.function(image=image, gpu=GPU, cpu=8, timeout=2 * HOUR, volumes={"/data": volume})
def run(env: dict, token: str) -> dict:
    results = "/results"
    os.makedirs(results, exist_ok=True)
    full = dict(os.environ, RESULTS=results, **env)
    started = time.time()
    subprocess.run(["chmod", "+x", "/work/ab/ic-before", "/work/ab/ic-after", "/work/ab/exp/job_ab.sh"], check=False)
    with open(os.path.join(results, "job.log"), "w") as log:
        p = subprocess.Popen(["bash", "/work/ab/exp/job_ab.sh"], cwd="/work/ab", env=full, stdout=subprocess.PIPE,
                             stderr=subprocess.STDOUT, text=True, bufsize=1)
        for line in p.stdout:
            print(line.rstrip(), flush=True)
            log.write(line)
        rc = p.wait()
    pathlib.Path(results, "exit-code").write_text("%d\n" % rc)
    gpu = subprocess.run("nvidia-smi --query-gpu=name,driver_version --format=csv,noheader", shell=True,
                         capture_output=True, text=True).stdout.strip()
    meta = {"exitCode": rc, "seconds": time.time() - started, "gpu": gpu, "token": token}
    out = os.path.join("/data", token)
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
    dest = (outdir / "results").resolve()
    dest.mkdir(parents=True, exist_ok=True)
    with tarfile.open(outdir / "results.tgz") as archive:
        members = archive.getmembers()
        for m in members:
            if m.issym() or m.islnk() or m.isdev() or os.path.commonpath((str(dest), str((dest / m.name).resolve()))) != str(dest):
                raise ValueError(f"unsafe archive member: {m.name}")
        archive.extractall(dest, members=members)
    return meta


@app.local_entrypoint()
def main(out: str = "/tmp/ab-a100", env: str = "", fetch_token: str = ""):
    outdir = pathlib.Path(out)
    if fetch_token:
        outdir.mkdir(parents=True, exist_ok=True)
        meta = fetch(fetch_token, outdir)
        if meta is None:
            raise SystemExit("no finished run under token %s yet" % fetch_token)
        print("job exit code", meta["exitCode"], "on", meta["gpu"], "- results in", outdir / "results")
        raise SystemExit(1 if meta["exitCode"] else 0)
    if not (BEFORE and AFTER):
        raise SystemExit("set IC_BEFORE and IC_AFTER to the two ic binaries")
    rev = subprocess.run(["git", "rev-parse", "HEAD"], cwd=REPO, capture_output=True, text=True).stdout.strip()
    environment = {"SOURCE_REV": rev}
    for item in filter(None, env.split(",")):
        k, _, v = item.partition("=")
        environment[k] = v
    outdir.mkdir(parents=True, exist_ok=False)
    token = uuid.uuid4().hex
    receipt = {"job": "research/ic_gpu_call_overhead_20261113/job_ab.sh", "gpu": GPU, "cudaImage": f"nvidia/cuda:{CUDA}-devel-ubuntu24.04",
               "gitRev": rev, "env": environment, "provider": "modal", "token": token, "volume": "ic-gpu-call-overhead",
               "startedAt": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())}
    (outdir / "launch.json").write_text(json.dumps(receipt, indent=2) + "\n")
    call = run.spawn(environment, token)
    receipt["functionCallId"] = call.object_id
    (outdir / "launch.json").write_text(json.dumps(receipt, indent=2) + "\n")
    print("spawned", call.object_id, "token", token, "- if this client dies: modal run", __file__, "--fetch-token", token, "--out", out, flush=True)
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
