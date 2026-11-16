#!/usr/bin/env python3
"""Publish base image + one new layer to ECR without a Docker daemon.

Standard library only. Tokens are never printed. See --help.
"""
import argparse
import base64
import datetime
import gzip
import hashlib
import json
import os
import re
import ssl
import stat
import subprocess
import sys
import tarfile
import tempfile
import urllib.error
import urllib.parse
import urllib.request

CAFILE = os.environ.get("AWS_CA_BUNDLE") or "/root/.ccr/ca-bundle.crt"
CTX = ssl.create_default_context(cafile=CAFILE)
CHUNK = 16 * 1024 * 1024  # ECR minimum chunk is 5 MiB except the last

OCI_INDEX = "application/vnd.oci.image.index.v1+json"
DOCKER_LIST = "application/vnd.docker.distribution.manifest.list.v2+json"
OCI_MANIFEST = "application/vnd.oci.image.manifest.v1+json"
DOCKER_MANIFEST = "application/vnd.docker.distribution.manifest.v2+json"
ACCEPT = ", ".join([OCI_INDEX, DOCKER_LIST, OCI_MANIFEST, DOCKER_MANIFEST])


def die(msg):
    print("ERROR: " + msg, file=sys.stderr)
    sys.exit(1)


def redact(url):
    return re.sub(r"(token|X-Amz-Signature|X-Amz-Credential|X-Amz-Security-Token)=[^&]+",
                  r"\1=...", url)


class NoRedirect(urllib.request.HTTPRedirectHandler):
    def redirect_request(self, *a, **k):
        return None


OPENER = urllib.request.build_opener(urllib.request.HTTPSHandler(context=CTX), NoRedirect())
REDIRECTS = (301, 302, 303, 307, 308)


def sh(args):
    p = subprocess.run(args, capture_output=True, text=True)
    if p.returncode:
        die("%s failed: %s" % (" ".join(args[:3]), p.stderr.strip()[:300]))
    return p.stdout.strip()


def parse_ref(ref):
    host, _, rest = ref.partition("/")
    if "@" in rest:
        repo, _, tag = rest.partition("@")
    elif ":" in rest.rsplit("/", 1)[-1]:
        repo, _, tag = rest.rpartition(":")
    else:
        repo, tag = rest, "latest"
    return host, repo, tag


class Registry:
    def __init__(self, host, repo, password=None, push=False):
        self.host, self.repo, self.password, self.push = host, repo, password, push
        self.base = "https://" + host
        self.authz = None  # full Authorization header value

    def basic(self):
        return "Basic " + base64.b64encode(("AWS:" + self.password).encode()).decode()

    def _token(self, ch):
        scope = ch.get("scope") or "repository:%s:%s" % (
            self.repo, "pull,push" if self.push else "pull")
        url = ch["realm"] + "?" + urllib.parse.urlencode(
            {"service": ch.get("service", ""), "scope": scope})
        tries = [{"Authorization": self.basic()}] if self.password else []
        tries.append({})  # anonymous fallback
        for h in tries:
            try:
                with OPENER.open(urllib.request.Request(url, headers=h), timeout=60) as r:
                    d = json.load(r)
                tok = d.get("token") or d.get("access_token")
                if tok:
                    return "Bearer " + tok
            except urllib.error.HTTPError as e:
                print("token exchange (%s) HTTP %d" % ("basic" if h else "anonymous", e.code),
                      file=sys.stderr)
        die("could not obtain a bearer token from " + ch["realm"])

    def req(self, method, target, headers=None, data=None, ok=(200,)):
        url = target if target.startswith("http") else self.base + target
        h = dict(headers or {})
        for attempt in range(2):
            hh = dict(h)
            if self.authz:
                hh["Authorization"] = self.authz
            try:
                r = OPENER.open(urllib.request.Request(url, data=data, method=method,
                                                       headers=hh), timeout=900)
                return r.status, r.headers, r
            except urllib.error.HTTPError as e:
                if e.code == 401 and attempt == 0:
                    m = re.match(r"Bearer\s+(.*)", e.headers.get("WWW-Authenticate", ""), re.I)
                    e.close()
                    if m:
                        self.authz = self._token(dict(re.findall(r'(\w+)="([^"]*)"', m.group(1))))
                    elif self.password:
                        self.authz = self.basic()
                    else:
                        die("401 from %s without usable challenge" % self.host)
                    continue
                if e.code in REDIRECTS and method in ("GET", "HEAD"):
                    loc = urllib.parse.urljoin(url, e.headers["Location"])
                    e.close()
                    # presigned target: no registry credentials
                    try:
                        r = OPENER.open(urllib.request.Request(loc, method=method), timeout=900)
                        return r.status, r.headers, r
                    except urllib.error.HTTPError as e2:
                        die("HTTP %d on redirect target %s: %s" % (
                            e2.code, redact(loc), e2.read(300).decode("replace")))
                if e.code in ok:
                    return e.code, e.headers, e
                die("HTTP %d %s %s: %s" % (e.code, method, redact(url),
                                           e.read(400).decode("utf-8", "replace")))
            except urllib.error.URLError as e:
                die("%s %s failed: %s" % (method, redact(url), e.reason))

    def get_manifest(self, ref):
        _, h, r = self.req("GET", "/v2/%s/manifests/%s" % (self.repo, ref), {"Accept": ACCEPT})
        body = r.read()
        r.close()
        return body, h.get("Content-Type", "")

    def has_blob(self, digest):
        st, _, r = self.req("HEAD", "/v2/%s/blobs/%s" % (self.repo, digest), ok=(200, 404))
        r.close()
        return st == 200

    def open_blob(self, digest):
        return self.req("GET", "/v2/%s/blobs/%s" % (self.repo, digest))[2]

    def put_blob_file(self, digest, path):
        size = os.path.getsize(path)
        _, h, r = self.req("POST", "/v2/%s/blobs/uploads/" % self.repo,
                           {"Content-Length": "0"}, data=b"", ok=(202,))
        r.close()
        loc = urllib.parse.urljoin(self.base + "/", h["Location"])
        off = 0
        with open(path, "rb") as f:
            while size - off > CHUNK:
                buf = f.read(CHUNK)
                _, h, r = self.req("PATCH", loc, {
                    "Content-Type": "application/octet-stream",
                    "Content-Length": str(len(buf)),
                    "Content-Range": "%d-%d" % (off, off + len(buf) - 1)}, data=buf, ok=(202,))
                r.close()
                off += len(buf)
                loc = urllib.parse.urljoin(self.base + "/", h["Location"])
            buf = f.read()
        hdr = {"Content-Type": "application/octet-stream", "Content-Length": str(len(buf))}
        if off and buf:
            hdr["Content-Range"] = "%d-%d" % (off, off + len(buf) - 1)
        sep = "&" if "?" in loc else "?"
        _, _, r = self.req("PUT", loc + sep + "digest=" + digest, hdr, data=buf, ok=(201,))
        r.close()

    def put_manifest(self, tag, body, mtype):
        _, h, r = self.req("PUT", "/v2/%s/manifests/%s" % (self.repo, tag),
                           {"Content-Type": mtype, "Content-Length": str(len(body))},
                           data=body, ok=(200, 201))
        r.close()
        return h.get("Docker-Content-Digest")


def sha256_file(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for b in iter(lambda: f.read(1 << 20), b""):
            h.update(b)
    return "sha256:" + h.hexdigest()


def build_layer(layer_dir, out_gz, tmpdir):
    """Deterministic tar.gz. Returns (diff_id, digest, size, file_count)."""
    layer_dir = os.path.abspath(layer_dir)
    names = []
    for root, dirs, files in os.walk(layer_dir):
        for n in dirs + files:
            names.append(os.path.relpath(os.path.join(root, n), layer_dir))
    names.sort(key=lambda s: s.encode())
    rawpath = os.path.join(tmpdir, "layer.tar")
    seen, count = {}, 0
    with tarfile.open(rawpath, "w", format=tarfile.PAX_FORMAT) as tf:
        for name in names:
            p = os.path.join(layer_dir, name)
            st = os.lstat(p)
            ti = tarfile.TarInfo(name)
            ti.mtime = 0
            ti.uid, ti.gid = st.st_uid, st.st_gid
            ti.uname = ti.gname = ""
            ti.mode = stat.S_IMODE(st.st_mode)
            if stat.S_ISDIR(st.st_mode):
                ti.type = tarfile.DIRTYPE
                tf.addfile(ti)
            elif stat.S_ISLNK(st.st_mode):
                ti.type = tarfile.SYMTYPE
                ti.linkname = os.readlink(p)
                tf.addfile(ti)
            elif stat.S_ISREG(st.st_mode):
                key = (st.st_dev, st.st_ino)
                if st.st_nlink > 1 and key in seen:
                    ti.type = tarfile.LNKTYPE
                    ti.linkname = seen[key]
                    tf.addfile(ti)
                else:
                    if st.st_nlink > 1:
                        seen[key] = name
                    ti.size = st.st_size
                    with open(p, "rb") as f:
                        tf.addfile(ti, f)
            else:
                continue
            count += 1
    diff_id = sha256_file(rawpath)
    with open(rawpath, "rb") as f, open(out_gz, "wb") as o:
        gz = gzip.GzipFile(filename="", mode="wb", fileobj=o, mtime=0, compresslevel=9)
        for b in iter(lambda: f.read(1 << 20), b""):
            gz.write(b)
        gz.close()
    os.unlink(rawpath)
    return diff_id, sha256_file(out_gz), os.path.getsize(out_gz), count


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--base", required=True)
    ap.add_argument("--layer-dir", required=True)
    ap.add_argument("--target", required=True)
    ap.add_argument("--tag", required=True)
    ap.add_argument("--user")
    ap.add_argument("--workdir")
    ap.add_argument("--env", action="append", default=[])
    ap.add_argument("--label", action="append", default=[])
    ap.add_argument("--out", required=True)
    a = ap.parse_args()

    bhost, brepo, btag = parse_ref(a.base)
    thost, trepo, _ = parse_ref(a.target)
    tregion = thost.split(".")[3]
    now = datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")

    bpw = None
    if bhost == "public.ecr.aws":
        bpw = sh(["aws", "ecr-public", "get-login-password", "--region", "us-east-1"])
    tpw = sh(["aws", "ecr", "get-login-password", "--region", tregion])
    if bhost == thost:
        # Private base in the target registry (e.g. a study image under a runtime layer).
        bpw = tpw
    src = Registry(bhost, brepo, bpw)
    dst = Registry(thost, trepo, tpw, push=True)
    dst.authz = dst.basic()
    if bhost == thost:
        src.authz = src.basic()

    body, ctype = src.get_manifest(btag)
    m = json.loads(body)
    bdigest = "sha256:" + hashlib.sha256(body).hexdigest()
    if m.get("manifests") is not None:
        pick = next((e for e in m["manifests"]
                     if e.get("platform", {}).get("os") == "linux"
                     and e.get("platform", {}).get("architecture") == "amd64"), None)
        if not pick:
            die("no linux/amd64 entry in base index")
        bdigest = pick["digest"]
        body, ctype = src.get_manifest(bdigest)
        if "sha256:" + hashlib.sha256(body).hexdigest() != bdigest:
            die("base manifest digest mismatch")
        m = json.loads(body)
    mtype = m.get("mediaType") or ctype.split(";")[0]
    oci = "oci" in mtype
    print("base manifest", bdigest, mtype)
    r = src.open_blob(m["config"]["digest"])
    cfg = json.loads(r.read())
    r.close()

    tmp = tempfile.mkdtemp(prefix="ocipub-")
    for L in m["layers"]:
        dg = L["digest"]
        if dst.has_blob(dg):
            print("  exists", dg[:19])
            continue
        path = os.path.join(tmp, dg.replace(":", "_"))
        h = hashlib.sha256()
        r = src.open_blob(dg)
        with open(path, "wb") as f:
            for b in iter(lambda: r.read(1 << 20), b""):
                h.update(b)
                f.write(b)
        r.close()
        if "sha256:" + h.hexdigest() != dg:
            die("digest mismatch downloading " + dg)
        dst.put_blob_file(dg, path)
        os.unlink(path)
        print("  copied", dg[:19], L.get("size"))

    lpath = os.path.join(tmp, "layer.tar.gz")
    diff_id, ldigest, lsize, nfiles = build_layer(a.layer_dir, lpath, tmp)
    if not dst.has_blob(ldigest):
        dst.put_blob_file(ldigest, lpath)
    print("layer", ldigest, lsize, "files", nfiles)

    cfg["architecture"] = "amd64"
    cfg["os"] = "linux"
    cfg["created"] = now
    c = cfg.setdefault("config", {})
    cfg.setdefault("rootfs", {"type": "layers", "diff_ids": []})["diff_ids"].append(diff_id)
    cfg.setdefault("history", []).append(
        {"created": now, "created_by": "oci-layer-publish.py", "comment": "study layer"})
    if a.user:
        c["User"] = a.user
    if a.workdir:
        c["WorkingDir"] = a.workdir
    env = {}
    for e in (c.get("Env") or []) + a.env:
        env[e.split("=", 1)[0]] = e  # dict keeps first-insertion order, last value wins
    c["Env"] = list(env.values())
    c["Entrypoint"] = []
    c["Cmd"] = c.get("Cmd") or ["/bin/bash"]
    labels = dict(c.get("Labels") or {})
    for l in a.label:
        k, _, v = l.partition("=")
        labels[k] = v
    if labels:
        c["Labels"] = labels
    cbody = json.dumps(cfg, separators=(",", ":"), sort_keys=True).encode()
    cdigest = "sha256:" + hashlib.sha256(cbody).hexdigest()
    cpath = os.path.join(tmp, "config.json")
    with open(cpath, "wb") as f:
        f.write(cbody)
    dst.put_blob_file(cdigest, cpath)

    lmt = ("application/vnd.oci.image.layer.v1.tar+gzip" if oci
           else "application/vnd.docker.image.rootfs.diff.tar.gzip")
    nm = {"schemaVersion": 2,
          "mediaType": OCI_MANIFEST if oci else DOCKER_MANIFEST,
          "config": {"mediaType": m["config"]["mediaType"], "digest": cdigest,
                     "size": len(cbody)},
          "layers": [dict(x) for x in m["layers"]] +
                    [{"mediaType": lmt, "digest": ldigest, "size": lsize}]}
    nbody = json.dumps(nm, separators=(",", ":")).encode()
    hdr_digest = dst.put_manifest(a.tag, nbody, nm["mediaType"])
    local = "sha256:" + hashlib.sha256(nbody).hexdigest()
    if hdr_digest != local:
        die("Docker-Content-Digest %s != local %s" % (hdr_digest, local))
    got = sh(["aws", "ecr", "describe-images", "--region", tregion,
              "--repository-name", trepo, "--image-ids", "imageTag=" + a.tag,
              "--query", "imageDetails[0].imageDigest", "--output", "text"])
    if got != hdr_digest:
        die("describe-images digest %s != %s" % (got, hdr_digest))

    receipt = {
        "image_uri": "%s/%s@%s" % (thost, trepo, hdr_digest),
        "tag": a.tag,
        "base": {"reference": a.base, "digest": bdigest},
        "layer": {"diff_id": diff_id, "digest": ldigest, "size": lsize, "file_count": nfiles},
        "config_digest": cdigest,
        "published_at": now,
    }
    os.makedirs(os.path.dirname(os.path.abspath(a.out)), exist_ok=True)
    with open(a.out, "w") as f:
        json.dump(receipt, f, indent=2)
        f.write("\n")
    print(json.dumps(receipt, indent=2))


if __name__ == "__main__":
    main()
