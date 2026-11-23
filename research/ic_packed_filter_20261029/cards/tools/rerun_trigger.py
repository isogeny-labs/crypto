"""Evaluates the noise-rerun trigger of XP-20261029-packed-filter PROTOCOL.md section 4 for one wall session and writes P<b>-RERUN.md.
Reads only job-exit.json, l2-gate.json and SHA256SUMS of that session (the blinding of section 4: no records.jsonl, no compare, band or work, no wall field).
usage: rerun_trigger.py B JOB SESSION POD_PREFIX_COUNT POD_UID COPY_DIR TRIGGER_JQ OUT_MD
  B                 24, 28 or 32
  JOB               the wall job evaluated (ic-dgbf-p<b>-20261029 or ic-dgbf-p<b>-20261029-r2)
  SESSION           first or r2
  POD_PREFIX_COUNT  the number of pod-uid prefixes under the job's S3 prefix, counted by the card
  POD_UID           the pod-uid prefix name (no trailing slash)
  COPY_DIR          holds the downloaded job-exit.json, l2-gate.json and SHA256SUMS
  TRIGGER_JQ        the jq program (trigger.jq)
  OUT_MD            the file to create (exclusive creation; an existing file is a STOP)
It prints the file it writes (its line NOISE_TRIGGER=true or NOISE_TRIGGER=false is the verdict) and the lines RERUN_MD, RERUN_MD_SHA256 and TRIGGER_OK. It exits 0 for either verdict."""
import collections, hashlib, json, os, re, subprocess, sys

b, job, session, n_pods, pod_uid, copy_dir, trig, out_md = sys.argv[1:9]
n_pods = int(n_pods)


def stop(msg):
    print("STOP: " + msg)
    sys.exit(1)


def sha(p):
    h = hashlib.sha256()
    with open(p, "rb") as f:
        for c in iter(lambda: f.read(1 << 20), b""):
            h.update(c)
    return h.hexdigest()


if b not in ("24", "28", "32"):
    stop("b is not 24, 28 or 32")
want_job = {"first": "ic-dgbf-p%s-20261029" % b, "r2": "ic-dgbf-p%s-20261029-r2" % b}.get(session)
if want_job is None or job != want_job:
    stop("session %s and job %s do not match (wall job names: ic-dgbf-p%s-20261029 for first, ic-dgbf-p%s-20261029-r2 for r2)" % (session, job, b, b))
if os.path.exists(out_md):
    stop("exists: " + out_md)
files = {}
for name in ("job-exit.json", "l2-gate.json", "SHA256SUMS"):
    p = os.path.join(copy_dir, name)
    if not os.path.isfile(p):
        stop("absent: " + p)
    files[name] = p
sums = {}
for line in open(files["SHA256SUMS"]):
    m = re.match(r"^([0-9a-f]{64})  (.+)$", line.rstrip("\n"))
    if m:
        sums[m.group(2)] = m.group(1)
sum_check = {}
for name in ("job-exit.json", "l2-gate.json"):
    sum_check[name] = {"in_SHA256SUMS": name in sums, "sha256_of_downloaded_file": sha(files[name]), "sha256_recorded": sums.get(name), "equal": sums.get(name) == sha(files[name])}
sums_ok = all(v["equal"] for v in sum_check.values())
try:
    jexit = json.load(open(files["job-exit.json"]))
    gate = json.load(open(files["l2-gate.json"]))
except Exception as e:
    stop("cannot read a JSON file: " + type(e).__name__)
# condition 1
c1 = (jexit.get("exit_code") == 42) and (n_pods == 1) and (jexit.get("pod_uid") == pod_uid)
# conditions 2 to 4: the one jq -e over l2-gate.json
cmd = ["jq", "-e", "-f", trig, files["l2-gate.json"]]
r = subprocess.run(cmd, capture_output=True, text=True)
jq_status = r.returncode
jq_out = r.stdout.strip()
jq_err = r.stderr.strip()
if jq_status not in (0, 1):
    stop("jq -e exited with status %d: %s" % (jq_status, jq_err))
c234 = (jq_status == 0)
verdict = sums_ok and c1 and c234
below = gate.get("below_l2") or []
tmpl = collections.Counter()
levels = collections.Counter()
for e in below:
    iso = e.get("isolation") or {}
    levels[str(iso.get("level"))] += 1
    for x in iso.get("blockers") or []:
        tmpl[str(x)[:4] + re.sub(r"[0-9]+(\.[0-9]+)?", "#", str(x)[4:])] += 1
lines = []
lines.append("# P%s noise-rerun trigger evaluation (PROTOCOL.md section 4)" % b)
lines.append("")
lines.append("Written by the launch card of XP-20261029-packed-filter (procedure TRIGGER) from `job-exit.json`, `l2-gate.json` and `SHA256SUMS` of the session below, and nothing else.")
lines.append("No `records.jsonl`, no `compare`, `band` or `work`, and no wall field was read.")
lines.append("")
lines.append("- job evaluated: `%s` (session: %s)" % (job, session))
lines.append("- pod-uid prefixes under the job's S3 prefix: %d; the prefix used: `%s`" % (n_pods, pod_uid))
lines.append("- job-exit.json: `%s`" % json.dumps(jexit, sort_keys=True))
lines.append("- sha256 of job-exit.json: `%s`" % sum_check["job-exit.json"]["sha256_of_downloaded_file"])
lines.append("- sha256 of l2-gate.json: `%s`" % sum_check["l2-gate.json"]["sha256_of_downloaded_file"])
lines.append("- SHA256SUMS lists both files with these hashes: %s" % ("yes" if sums_ok else "NO"))
lines.append("- sha256 of SHA256SUMS: `%s`" % sha(files["SHA256SUMS"]))
lines.append("")
lines.append("## Condition 1 (exit code 42 from a single pod)")
lines.append("")
lines.append("- exit_code = %s; pod-uid prefixes = %d; job-exit.json pod_uid equals the prefix: %s" % (json.dumps(jexit.get("exit_code")), n_pods, "yes" if jexit.get("pod_uid") == pod_uid else "no"))
lines.append("- condition 1: %s" % ("holds" if c1 else "does not hold"))
lines.append("")
lines.append("## Conditions 2 to 4 (one `jq -e` over `l2-gate.json`)")
lines.append("")
lines.append("Command: `%s`" % " ".join(cmd))
lines.append("")
lines.append("- exit status: %d" % jq_status)
lines.append("- printed: `%s`" % jq_out)
lines.append("- conditions 2 to 4: %s" % ("hold" if c234 else "do not hold"))
lines.append("")
lines.append("Descriptive facts of l2-gate.json (no value of a measurement):")
lines.append("")
lines.append("- invalid_outcomes length: %s" % (len(gate["invalid_outcomes"]) if isinstance(gate.get("invalid_outcomes"), list) else "absent or not a list"))
lines.append("- below_l2 length: %d; levels: `%s`" % (len(below), json.dumps(dict(levels), sort_keys=True)))
lines.append("- blocker texts with every number replaced by #, with counts:")
for t, n in sorted(tmpl.items()):
    lines.append("  - %d x `%s`" % (n, t))
lines.append("")
lines.append("## Verdict")
lines.append("")
lines.append("NOISE_TRIGGER=%s" % ("true" if verdict else "false"))
text = "\n".join(lines) + "\n"
os.makedirs(os.path.dirname(out_md), exist_ok=True)
with open(out_md, "x") as f:
    f.write(text)
print(text, end="")
print("RERUN_MD " + out_md)
print("RERUN_MD_SHA256 " + sha(out_md))
print("TRIGGER_OK")
