#!/usr/bin/env python3
"""Read-only fetch of one content-addressed object from the lab artifact store.

Validator V, 2026-11-14. The checkout has AWS credentials in the environment
but neither the aws CLI nor boto3, so `tools/artifacts/artifacts.py pull`
cannot run. This is a stdlib SigV4 GET of
s3://setec-astronomy-artifacts/store/sha256/<aa>/<sha256>, written to OUT
only when its SHA-256 equals the requested digest. No write, list or delete.

Usage: s3_get_sigv4.py SHA256 OUT [--region us-east-1]
"""
import datetime, hashlib, hmac, os, ssl, sys, urllib.request, urllib.error

sha, out = sys.argv[1], sys.argv[2]
region = sys.argv[sys.argv.index("--region") + 1] if "--region" in sys.argv else os.environ.get("AWS_DEFAULT_REGION", "us-east-1")
bucket, key = "setec-astronomy-artifacts", f"store/sha256/{sha[:2]}/{sha}"
ak, sk = os.environ["AWS_ACCESS_KEY_ID"], os.environ["AWS_SECRET_ACCESS_KEY"]
tok = os.environ.get("AWS_SESSION_TOKEN")
ctx = ssl.create_default_context(cafile=os.environ.get("AWS_CA_BUNDLE") or "/root/.ccr/ca-bundle.crt")


def sign(k, m): return hmac.new(k, m.encode(), hashlib.sha256).digest()


def get(region):
    host = f"{bucket}.s3.{region}.amazonaws.com"
    now = datetime.datetime.now(datetime.timezone.utc)
    amzdate, date = now.strftime("%Y%m%dT%H%M%SZ"), now.strftime("%Y%m%d")
    ph = hashlib.sha256(b"").hexdigest()
    hdrs = {"host": host, "x-amz-content-sha256": ph, "x-amz-date": amzdate}
    if tok: hdrs["x-amz-security-token"] = tok
    sh = ";".join(sorted(hdrs))
    canon = "\n".join(["GET", "/" + key, "", "".join(f"{k}:{hdrs[k]}\n" for k in sorted(hdrs)), sh, ph])
    scope = f"{date}/{region}/s3/aws4_request"
    sts = "\n".join(["AWS4-HMAC-SHA256", amzdate, scope, hashlib.sha256(canon.encode()).hexdigest()])
    kd = sign(sign(sign(sign(("AWS4" + sk).encode(), date), region), "s3"), "aws4_request")
    sig = hmac.new(kd, sts.encode(), hashlib.sha256).hexdigest()
    hdrs["Authorization"] = f"AWS4-HMAC-SHA256 Credential={ak}/{scope}, SignedHeaders={sh}, Signature={sig}"
    req = urllib.request.Request(f"https://{host}/{key}", headers={k: v for k, v in hdrs.items() if k != "host"})
    return urllib.request.urlopen(req, context=ctx, timeout=120).read()


try:
    data = get(region)
except urllib.error.HTTPError as e:
    reg = e.headers.get("x-amz-bucket-region")
    if reg and reg != region:
        data = get(reg)
    else:
        print("HTTP", e.code, e.read()[:300]); sys.exit(2)
h = hashlib.sha256(data).hexdigest()
if h != sha:
    print("digest mismatch", h); sys.exit(3)
open(out, "wb").write(data)
print("ok", len(data), h)
