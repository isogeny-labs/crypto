# XP-20261029-packed-filter, card-launch-v2.md, MODE S3VERIFY (RULING-P28.md section (c), C2).
# Reproduces from S3 alone the checks that remote/run-request (lines 68-79) and remote/verify-build make
# after a job, and prints each as S3V_<NAME>=<value>. It reads no field of records.jsonl: that file is
# only downloaded and hashed (by `sha256sum -c`). It opens only these downloaded files:
# job-exit.json, SHA256SUMS, build-info.json, binary.sha256, input-spec.json and, for the printed
# outcome facts, audit.json, l2-gate.json, control-comparison.json (each only if the file exists).
#
# Values: true | false | absent. absent = the input of the fact does not exist, or the pod prefix is not
# unique so the fact was not evaluated. S3V_POD_UID_COUNT and S3V_EXIT_CODE print a number (S3V_EXIT_CODE
# prints the JSON text of the exit_code field, or absent).
#
# Usage (network mode, the card):
#   python3 -I s3verify.py --b B --tag TAG --job JOB --request REQUEST.json --s3-prefix s3://BUCKET/runs/JOB/
#       --card-sha256 HEX --out OUTDIR --download-dir DLDIR [--pod-status FILE --pod-status-sha256 HEX]
# Usage (local mode, dry run, no network): add --listing-file F (the output of `aws s3 ls --recursive
#   <s3-prefix>`) and --local-dir D (the already downloaded pod directory); --download-dir is then not used.
# OUTDIR and DLDIR must not exist. The tool writes OUTDIR/s3receipt.json and OUTDIR/check-output.txt.
import argparse, hashlib, json, os, re, subprocess, sys, datetime

UUID = re.compile(r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$")
HEX64 = re.compile(r"^[0-9a-f]{64}$")
# Expressions verbatim from remote/run-request lines 76-79 and remote/verify-build lines 8-11.
JQ_EXIT0 = ".exit_code==0"
JQ_L2GATE = ".accepted and .executions>0 and (.below_l2|length)==0 and (.invalid_outcomes|length)==0"
JQ_AUDIT = (".ok and .regraded and .records_bound and .verified_records==.records and "
            "(.replays|length)>0 and all(.replays[];.reproduced)")
JQ_CONTROL = '.wall.status=="admitted" and .wall.aa_pairs>=5 and .wall.aa_ci95!=null'
JQ_BUILD_INFO = ".source_revision==$request[0].job.build.source_revision and .ecbench_sha256==$request[0].job.build.ecbench_sha256"
JQ_IMAGE = ('[.pods[] | select(.uid==$pod) | .status.containerStatuses[] | '
            'select(.name=="task" and .state.terminated.exitCode==0) | '
            '.imageID | sub("^docker-pullable://";"")] == [$request[0].job.build.image_uri]')

OUT_LINES = []


def emit(line):
    OUT_LINES.append(line)
    print(line, flush=True)


def die(msg):
    print("STOP: " + msg, flush=True)
    sys.exit(1)


def sha256_file(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for blk in iter(lambda: f.read(1 << 20), b""):
            h.update(blk)
    return h.hexdigest()


def jq_bool(expr, path, extra=()):
    """absent if the file does not exist; true if `jq -e` exits 0; false otherwise."""
    if not os.path.isfile(path):
        return "absent"
    r = subprocess.run(["jq", "-e", *extra, expr, path], capture_output=True, text=True)
    return r.returncode == 0


def load_json(path):
    try:
        with open(path, "rb") as f:
            return json.loads(f.read().decode("utf-8")), True
    except (OSError, ValueError):
        return None, False


def parse_listing(text, prefix_key):
    """Lines of `aws s3 ls --recursive`: '<date> <time> <size> <key>'. Returns [(key_relative, size, date_time)]."""
    rows = []
    for ln in text.splitlines():
        if not ln.strip():
            continue
        parts = ln.split(None, 3)
        if len(parts) != 4 or not parts[2].isdigit():
            die("unparsable S3 listing line: " + ln)
        key = parts[3]
        if not key.startswith(prefix_key):
            die("listed key outside the job prefix: " + key)
        rows.append((key[len(prefix_key):], int(parts[2]), parts[0] + " " + parts[1]))
    return rows


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--b", required=True)
    ap.add_argument("--tag", required=True)
    ap.add_argument("--job", required=True)
    ap.add_argument("--request", required=True)
    ap.add_argument("--s3-prefix", required=True)
    ap.add_argument("--card-sha256", required=True)
    ap.add_argument("--out", required=True)
    ap.add_argument("--download-dir")
    ap.add_argument("--listing-file")
    ap.add_argument("--local-dir")
    ap.add_argument("--pod-status")
    ap.add_argument("--pod-status-sha256")
    a = ap.parse_args()

    if not HEX64.match(a.card_sha256):
        die("card sha256 is not 64 lowercase hex")
    local = a.local_dir is not None
    if local != (a.listing_file is not None):
        die("--local-dir and --listing-file are used together or not at all")
    if not local and not a.download_dir:
        die("--download-dir is required in network mode")
    if (a.pod_status is None) != (a.pod_status_sha256 is None):
        die("--pod-status and --pod-status-sha256 are used together or not at all")
    if os.path.exists(a.out):
        die("output directory exists: " + a.out)
    if not local and os.path.exists(a.download_dir):
        die("download directory exists: " + a.download_dir)
    if not a.s3_prefix.endswith("/") or not a.s3_prefix.endswith("/runs/%s/" % a.job):
        die("s3 prefix is not .../runs/<job>/")
    prefix_key = a.s3_prefix.split("://", 1)[1].split("/", 1)[1]  # runs/<job>/

    req, ok = load_json(a.request)
    if not ok or not isinstance(req, dict) or not isinstance(req.get("job"), dict):
        die("request is not a JSON object with a job")
    job = req["job"]
    if job.get("job_name") != a.job:
        die("request job_name %r differs from --job %r" % (job.get("job_name"), a.job))
    build = job.get("build")
    if not isinstance(build, dict) or not all(isinstance(build.get(k), str) and build.get(k)
                                              for k in ("source_revision", "ecbench_sha256", "image_uri")):
        die("request job.build lacks source_revision, ecbench_sha256 or image_uri")
    if "spec" not in job:
        die("request job.spec is absent")
    request_sha = sha256_file(a.request)

    # ---- S3 listing
    if local:
        listing_text = open(a.listing_file, encoding="utf-8").read()
    else:
        r = subprocess.run(["aws", "s3", "ls", "--recursive", a.s3_prefix], capture_output=True, text=True)
        if r.returncode == 0:
            listing_text = r.stdout
        elif r.returncode == 1 and not r.stdout.strip() and not r.stderr.strip():
            listing_text = ""  # `aws s3 ls` exits 1 with no output for an empty prefix
        else:
            die("aws s3 ls failed with status %d: %s" % (r.returncode, r.stderr.strip()[:300]))
    rows = parse_listing(listing_text, prefix_key)
    pods = sorted({k.split("/", 1)[0] for k, _, _ in rows if "/" in k and UUID.match(k.split("/", 1)[0])})
    emit("S3V_MODE=%s" % ("local" if local else "s3"))
    emit("S3V_JOB=%s" % a.job)
    emit("S3V_LISTED_OBJECTS=%d" % len(rows))
    emit("S3V_POD_UID_COUNT=%d" % len(pods))
    facts = {"S3V_POD_UID_COUNT": len(pods)}

    names = ["S3V_JOB_EXIT_PRESENT", "S3V_JOB_EXIT_IDENTITY", "S3V_EXIT_CODE", "S3V_SHA256SUMS_OK",
             "S3V_MANIFEST_COMPLETE", "S3V_BUILD_INFO", "S3V_BINARY_SHA", "S3V_INPUT_SPEC",
             "S3V_EXIT0", "S3V_L2GATE", "S3V_AUDIT", "S3V_CONTROL"]
    pod = None
    sha_sums_sha = None
    job_exit_obj = None
    downloaded = []
    if len(pods) == 1:
        pod = pods[0]
        emit("S3V_POD_UID=%s" % pod)
        d = a.local_dir if local else a.download_dir
        if not local:
            os.makedirs(os.path.dirname(a.download_dir.rstrip("/")), exist_ok=True)
            os.mkdir(a.download_dir)
            r = subprocess.run(["aws", "s3", "cp", a.s3_prefix + pod + "/", a.download_dir + "/", "--recursive",
                                "--only-show-errors"], capture_output=True, text=True)
            if r.returncode != 0:
                die("download failed with status %d: %s" % (r.returncode, r.stderr.strip()[:300]))
        for root, _dirs, files in os.walk(d):
            for fn in files:
                downloaded.append(os.path.relpath(os.path.join(root, fn), d))
        downloaded.sort()
        p = lambda n: os.path.join(d, n)

        # job-exit.json: present, identity, exit code
        present = os.path.isfile(p("job-exit.json"))
        facts["S3V_JOB_EXIT_PRESENT"] = present
        if present:
            job_exit_obj, ok = load_json(p("job-exit.json"))
            facts["S3V_JOB_EXIT_IDENTITY"] = bool(ok and isinstance(job_exit_obj, dict)
                                                  and job_exit_obj.get("job_name") == a.job
                                                  and job_exit_obj.get("pod_uid") == pod)
            facts["S3V_EXIT_CODE"] = (json.dumps(job_exit_obj.get("exit_code"))
                                      if ok and isinstance(job_exit_obj, dict) and "exit_code" in job_exit_obj
                                      else "absent")
        else:
            facts["S3V_JOB_EXIT_IDENTITY"] = "absent"
            facts["S3V_EXIT_CODE"] = "absent"

        # SHA256SUMS: sha256sum -c all OK; manifest complete
        sums_path = p("SHA256SUMS")
        if os.path.isfile(sums_path):
            sha_sums_sha = sha256_file(sums_path)
            text = open(sums_path, encoding="utf-8", errors="replace").read()
            lines = [ln for ln in text.splitlines() if ln.strip()]
            r = subprocess.run(["sha256sum", "-c", "SHA256SUMS"], cwd=d, capture_output=True, text=True)
            ok_lines = [ln for ln in r.stdout.splitlines() if ln.endswith(": OK")]
            facts["S3V_SHA256SUMS_OK"] = bool(r.returncode == 0 and lines and len(ok_lines) == len(lines)
                                              and len(r.stdout.splitlines()) == len(lines))
            listed = []
            parse_ok = True
            for ln in lines:
                m = re.match(r"^([0-9a-f]{64}) [ *](.+)$", ln)
                if not m:
                    parse_ok = False
                    break
                listed.append(m.group(2))
            s3_files = sorted(k[len(pod) + 1:] for k, _, _ in rows
                              if k.startswith(pod + "/") and not k.endswith("/"))
            dl_wo = sorted(f for f in downloaded if f != "SHA256SUMS")
            facts["S3V_MANIFEST_COMPLETE"] = bool(
                parse_ok and len(set(listed)) == len(listed)
                and sorted(listed) == dl_wo
                and sorted(downloaded) == s3_files)
        else:
            facts["S3V_SHA256SUMS_OK"] = "absent"
            facts["S3V_MANIFEST_COMPLETE"] = "absent"

        # build identity (verify-build lines 7-11, 17-19)
        facts["S3V_BUILD_INFO"] = jq_bool(JQ_BUILD_INFO, p("build-info.json"), ("--slurpfile", "request", a.request))
        if os.path.isfile(p("binary.sha256")):
            t = open(p("binary.sha256"), encoding="utf-8", errors="replace").read().rstrip("\n")
            facts["S3V_BINARY_SHA"] = bool("\n" not in t and t.split(" ")[0] == build["ecbench_sha256"])
        else:
            facts["S3V_BINARY_SHA"] = "absent"
        if os.path.isfile(p("input-spec.json")):
            spec_obj, ok = load_json(p("input-spec.json"))
            facts["S3V_INPUT_SPEC"] = bool(ok and spec_obj == job["spec"])
        else:
            facts["S3V_INPUT_SPEC"] = "absent"

        # outcome facts: printed, never gated
        facts["S3V_EXIT0"] = jq_bool(JQ_EXIT0, p("job-exit.json"))
        facts["S3V_L2GATE"] = jq_bool(JQ_L2GATE, p("l2-gate.json"))
        facts["S3V_AUDIT"] = jq_bool(JQ_AUDIT, p("audit.json"))
        facts["S3V_CONTROL"] = jq_bool(JQ_CONTROL, p("control-comparison.json"))
    else:
        emit("S3V_POD_UID=absent")
        for n in names:
            facts[n] = "absent"

    # image digest: only from a pinned pod-status input (verify-build lines 12-16)
    if a.pod_status is not None:
        if not HEX64.match(a.pod_status_sha256) or sha256_file(a.pod_status) != a.pod_status_sha256:
            die("pod-status file does not match its pinned sha256")
        if pod is None:
            facts["S3V_IMAGE_DIGEST"] = "absent"
        else:
            r = subprocess.run(["jq", "-e", "--arg", "pod", pod, "--slurpfile", "request", a.request,
                                JQ_IMAGE, a.pod_status], capture_output=True, text=True)
            facts["S3V_IMAGE_DIGEST"] = r.returncode == 0
    else:
        facts["S3V_IMAGE_DIGEST"] = "unverifiable_without_pod_status"

    order = ["S3V_POD_UID_COUNT", "S3V_JOB_EXIT_PRESENT", "S3V_JOB_EXIT_IDENTITY", "S3V_EXIT_CODE",
             "S3V_SHA256SUMS_OK", "S3V_MANIFEST_COMPLETE", "S3V_BUILD_INFO", "S3V_BINARY_SHA", "S3V_INPUT_SPEC",
             "S3V_IMAGE_DIGEST", "S3V_EXIT0", "S3V_L2GATE", "S3V_AUDIT", "S3V_CONTROL"]
    for n in order[1:]:
        v = facts[n]
        emit("%s=%s" % (n, str(v).lower() if isinstance(v, bool) else v))
    if job_exit_obj is not None:
        emit("S3V_JOB_EXIT_JSON=" + json.dumps(job_exit_obj, sort_keys=True, separators=(",", ":")))
    gate = bool(facts["S3V_POD_UID_COUNT"] == 1
                and facts["S3V_JOB_EXIT_PRESENT"] is True and facts["S3V_JOB_EXIT_IDENTITY"] is True
                and facts["S3V_SHA256SUMS_OK"] is True and facts["S3V_MANIFEST_COMPLETE"] is True
                and facts["S3V_BUILD_INFO"] is True and facts["S3V_BINARY_SHA"] is True
                and facts["S3V_INPUT_SPEC"] is True)
    emit("S3_TERMINAL=%s" % ("true" if gate else "false"))

    receipt = {
        "schema": "pf.s3verify/v1",
        "mode": "local" if local else "s3",
        "tag": a.tag, "b": int(a.b), "job": a.job, "pod_uid": pod,
        "s3_prefix": a.s3_prefix,
        "generated_utc": datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "facts": facts, "s3_terminal": gate,
        "jq_expressions": {"S3V_EXIT0": JQ_EXIT0, "S3V_L2GATE": JQ_L2GATE, "S3V_AUDIT": JQ_AUDIT,
                           "S3V_CONTROL": JQ_CONTROL, "S3V_BUILD_INFO": JQ_BUILD_INFO, "S3V_IMAGE_DIGEST": JQ_IMAGE},
        "job_exit_json": job_exit_obj,
        "s3_listing": [{"key": k, "size": s, "last_modified": t} for k, s, t in rows],
        "downloaded_files": downloaded,
        "sha256sums_sha256": sha_sums_sha,
        "request_sha256": request_sha,
        "card_sha256": a.card_sha256,
        "tool_sha256": sha256_file(os.path.abspath(__file__)),
        "pod_status_sha256": a.pod_status_sha256,
        "not_checked": ["pod image digest from Kubernetes status (verify-build lines 12-16) unless S3V_IMAGE_DIGEST is true or false",
                        "Kubernetes Job state and pod identity (run-request lines 56-58, 68)"],
    }
    os.makedirs(a.out)
    rp = os.path.join(a.out, "s3receipt.json")
    with open(rp, "w", encoding="utf-8") as f:
        json.dump(receipt, f, sort_keys=True, indent=1)
        f.write("\n")
    emit("S3V_RECEIPT_SHA256=%s" % sha256_file(rp))
    emit("S3V_OK")
    with open(os.path.join(a.out, "check-output.txt"), "w", encoding="utf-8") as f:
        f.write("\n".join(OUT_LINES) + "\n")


if __name__ == "__main__":
    main()
