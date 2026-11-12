#!/usr/bin/env python3
"""Launch the wide-modulus sparse-LA grid on RunPod CPU pods, one pod per seed.

Governed by ../PROTOCOL.md and ../PROTOCOL_AMENDMENT_1.md. Needs a clean
checkout (the bundle must be a function of HEAD), RUNPOD_API_KEY, AWS creds
and boto3. Writes ../launch.json (pod ids, S3 prefix, hashes; no secrets, no
signed URLs). Pods pull a minimal source bundle and pod_run.sh from S3 via
presigned GETs and push their raw JSONL back via presigned PUTs (7 days).

  python3 launch.py --dry-run          # bundle + upload + print plan, no pods
  python3 launch.py                    # create the pods
"""
import argparse, datetime, hashlib, json, os, subprocess, sys, time, urllib.request
import boto3

HERE = os.path.dirname(os.path.abspath(__file__))
OUTFILES = {"GRID": "grid.jsonl", "AA": "aa.jsonl", "WIDE": "wide_coeffs.jsonl",
            "RUNS": "runs.jsonl", "HOST": "host.json", "LOG": "pod.log", "DONE": "done.json"}
ap = argparse.ArgumentParser()
ap.add_argument("--seeds", type=int, nargs="+", default=[1, 2, 3])
ap.add_argument("--flavor", default="cpu5g")          # General Purpose, 4 GiB per vCPU
ap.add_argument("--vcpu", type=int, default=4)
ap.add_argument("--image", default="rust:1.97.0-bookworm")
ap.add_argument("--deadline-hours", type=float, default=40.0)
ap.add_argument("--bucket", default="setec-astronomy-artifacts")
ap.add_argument("--dry-run", action="store_true")
a = ap.parse_args()

root = subprocess.check_output(["git", "rev-parse", "--show-toplevel"], text=True, cwd=HERE).strip()
# Records, checkpoints and logs change during a run; everything else must match HEAD.
if subprocess.run(["git", "diff", "--quiet", "HEAD", "--", ".", ":!benchmarks", ":!.resume", ":!docs/session-logs"],
                  cwd=root).returncode:
    sys.exit("refusing: tracked files outside benchmarks/.resume/session-logs differ from HEAD")
sha = subprocess.check_output(["git", "rev-parse", "HEAD"], text=True, cwd=root).strip()
stamp = datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%MZ")
prefix = f"lab-results/wide-sparse-la/{stamp}-{sha[:10]}"
tmp = os.path.join(os.environ.get("TMPDIR", "/tmp"), f"wsla-{stamp}")
os.makedirs(tmp, exist_ok=True)
bundle = os.path.join(tmp, "src.tar.gz")
print(subprocess.check_output([sys.executable, os.path.join(HERE, "make_bundle.py"), bundle], text=True, cwd=root))
h = lambda p: hashlib.sha256(open(p, "rb").read()).hexdigest()
src_sha, script = h(bundle), os.path.join(HERE, "pod_run.sh")
s3 = boto3.client("s3")
s3.upload_file(bundle, a.bucket, f"{prefix}/src.tar.gz")
s3.upload_file(script, a.bucket, f"{prefix}/pod_run.sh")
ttl = 7 * 24 * 3600
get = lambda k: s3.generate_presigned_url("get_object", Params={"Bucket": a.bucket, "Key": k}, ExpiresIn=ttl)
put = lambda k: s3.generate_presigned_url("put_object", Params={"Bucket": a.bucket, "Key": k}, ExpiresIn=ttl)
deadline = int(time.time() + a.deadline_hours * 3600)
manifest = {"schema": "wide-sparse-la.runpod-launch/v1", "commit": sha, "s3_prefix": f"s3://{a.bucket}/{prefix}/",
            "src_bundle_sha256": src_sha, "pod_run_sh_sha256": h(script), "image": a.image,
            "flavor": a.flavor, "vcpu": a.vcpu, "cloud": "SECURE", "interruptible": False,
            "deadline_utc": datetime.datetime.fromtimestamp(deadline, datetime.timezone.utc).isoformat(),
            "launched_utc": stamp, "pods": []}
for seed in a.seeds:
    env = {"SEED": str(seed), "SRC_URL": get(f"{prefix}/src.tar.gz"), "SRC_SHA256": src_sha,
           "DEADLINE_EPOCH": str(deadline), "SCRIPT_URL": get(f"{prefix}/pod_run.sh")}
    for k, f in OUTFILES.items():
        env[f"PUT_{k}"] = put(f"{prefix}/seed{seed}/{f}")
    body = {"name": f"wsla-seed{seed}-{sha[:8]}", "imageName": a.image, "computeType": "CPU",
            "cpuFlavorIds": [a.flavor], "vcpuCount": a.vcpu, "containerDiskInGb": 30,
            "cloudType": "SECURE", "interruptible": False, "ports": [], "env": env,
            "dockerStartCmd": ["bash", "-c", 'curl -fsSL --retry 5 "$SCRIPT_URL" -o /pod_run.sh && exec bash /pod_run.sh']}
    entry = {"seed": seed, "s3": f"s3://{a.bucket}/{prefix}/seed{seed}/"}
    if a.dry_run:
        entry["pod_id"] = None
    else:
        req = urllib.request.Request("https://rest.runpod.io/v1/pods", data=json.dumps(body).encode(), method="POST",
                                     headers={"Authorization": f"Bearer {os.environ['RUNPOD_API_KEY']}",
                                              "Content-Type": "application/json",
                                              "User-Agent": "wsla-launcher/1.0"})
        try:
            r = json.load(urllib.request.urlopen(req, timeout=120))
        except urllib.error.HTTPError as e:
            sys.exit(f"pod create failed for seed {seed}: {e.code} {e.read()[:500]!r}")
        entry.update({"pod_id": r.get("id"), "cost_per_hr": r.get("costPerHr"), "machine_id": r.get("machineId")})
    manifest["pods"].append(entry)
    print(json.dumps(entry))
out = os.path.join(HERE, "..", "launch.json" if not a.dry_run else "launch.dry-run.json")
json.dump(manifest, open(out, "w"), indent=1)
print("wrote", os.path.normpath(out))
