"""Run the carry-less-multiply probe on a Blackwell card with a CUDA 13.3 ptxas.

Triton ships ptxas from CUDA 12.x, which does
not know `clmad`; the container's CUDA 13.3 ptxas does, so
TRITON_PTXAS_PATH points at it.

    modal run modal_clmul.py
"""
import json
import os
import pathlib
import subprocess
import tarfile
import time
import uuid

import modal

LOCAL = pathlib.Path(__file__).parent
REMOTE = "/work/kernelagent-harness"
DATA = "/data"
GPU = os.environ.get("ECC_GPU", "RTX-PRO-6000")

image = (
    modal.Image.from_registry("nvidia/cuda:13.3.1-devel-ubuntu24.04", add_python="3.12")
    .entrypoint([])
    .apt_install("git", "build-essential")
    .pip_install("torch==2.8.0", index_url="https://download.pytorch.org/whl/cu128")
    .pip_install("triton==3.5.1")
    .env({"TRITON_PTXAS_PATH": "/usr/local/cuda/bin/ptxas"})
    .add_local_dir(LOCAL, remote_path=REMOTE, copy=True, ignore=["__pycache__", "*.pyc"])
)
volume = modal.Volume.from_name("ecc2k130-jobs", create_if_missing=True)
app = modal.App("ecc2k130-clmul")


def sh(cmd, log, env, cwd):
    p = subprocess.Popen(cmd, cwd=cwd, env=env, stdout=subprocess.PIPE,
                         stderr=subprocess.STDOUT, text=True, bufsize=1)
    for line in p.stdout:
        print(line.rstrip(), flush=True)
        log.write(line)
    return p.wait()


@app.function(image=image, gpu=GPU, cpu=8, timeout=3600, volumes={DATA: volume})
def run(script: str, token: str) -> dict:
    results = "/results"
    os.makedirs(results, exist_ok=True)
    env = dict(os.environ, PYTHONPATH=REMOTE)
    started = time.time()
    with open(os.path.join(results, "job.log"), "w") as log:
        ptxas = subprocess.run([os.environ["TRITON_PTXAS_PATH"], "--version"],
                               capture_output=True, text=True)
        log.write("ptxas: %s\n" % (ptxas.stdout or ptxas.stderr).strip())
        print("ptxas:", (ptxas.stdout or "").strip())
        rc = sh(["python3", os.path.join(REMOTE, script)], log, env, cwd=results)
    meta = {"script": script, "exitCode": rc, "seconds": time.time() - started,
            "gpu": subprocess.run("nvidia-smi --query-gpu=name,driver_version --format=csv,noheader",
                                  shell=True, capture_output=True, text=True).stdout.strip(),
            "token": token}
    with open(os.path.join(results, "meta.json"), "w") as f:
        json.dump(meta, f, indent=2)
    out = os.path.join(DATA, token)
    os.makedirs(out, exist_ok=True)
    with tarfile.open(os.path.join(out, "results.tgz"), "w:gz") as archive:
        archive.add(results, arcname=".")
    volume.commit()
    return meta


@app.local_entrypoint()
def main(script: str = "clmul_test.py"):
    token = time.strftime("%Y%m%d-%H%M%S-") + uuid.uuid4().hex[:8]
    print("token", token)
    meta = run.remote(script, token)
    print(json.dumps(meta, indent=2))
    dest = pathlib.Path(os.environ.get("CLMUL_OUT", "clmul-runs"), "clmul-%s" % token)
    dest.mkdir(parents=True, exist_ok=True)
    tgz = dest / "results.tgz"
    with open(tgz, "wb") as f:
        for chunk in volume.read_file("%s/results.tgz" % token):
            f.write(chunk)
    with tarfile.open(tgz) as archive:
        try:
            archive.extractall(dest, filter="data")
        except TypeError:
            archive.extractall(dest)
    tgz.unlink()
    print("results in", dest)
