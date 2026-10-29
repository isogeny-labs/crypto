import hashlib, json, os, sys
D, b, J, base, s3prefix, uid, frozen = sys.argv[1:8]

def sha(p):
    h = hashlib.sha256()
    with open(p, "rb") as f:
        for c in iter(lambda: f.read(1 << 20), b""):
            h.update(c)
    return h.hexdigest()

an = os.path.join(D, "analysis")
files = []
for d, _, fs in os.walk(an):
    for f in sorted(fs):
        p = os.path.join(d, f)
        if os.path.relpath(p, an) == "receipt.json":
            continue
        files.append({"path": os.path.relpath(p, D), "sha256": sha(p), "bytes": os.path.getsize(p)})
files.sort(key=lambda e: e["path"])
out = {
    "size_bits": int(b),
    "job_name": J,
    "baseline_arm": base,
    "s3_prefix": s3prefix,
    "pod_uid": uid.rstrip("/"),
    "frozen_spec_path": frozen,
    "frozen_spec_sha256": sha(frozen),
    "eks_results_SHA256SUMS_sha256": sha(os.path.join(D, "eks-results", "SHA256SUMS")),
    "recorded": json.load(open(os.path.join(an, "facts.json"))),
    "files": files,
}
json.dump(out, open(os.path.join(an, "receipt.json"), "w"), indent=2, sort_keys=True)
print("RECEIPT_OK files=" + str(len(files)))
