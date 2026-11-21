#!/usr/bin/env python3
"""VALIDATOR payload check for research/mt_timed_runpod_20261112.

1. Reads a payload candidate (the .tar.gz the producer's launcher wrote; accepted only if its sha256 equals the pin in
   launch/payload.json and launch/create.json), lists every member in memory (nothing extracted), and compares each
   member's bytes with the committed run.py, the validator's rebuilt binaries and the committed instances.
2. Rebuilds the payload from those committed / rebuilt inputs with the validator's own tar writer (GNU format, mode,
   uid/gid 0, mtime 0, members in the launcher's order) and gzip level 9.  The gzip header carries a time stamp the
   launcher does not fix (tarfile's default GzipFile mtime = time.time()), so the rebuild takes the header's 4-byte
   mtime from the candidate and, independently, searches the window [protocol commit - 2 h, pod creation] for it.
usage: payload_check.py <study dir> <candidate.tar.gz> <std bin> <fast bin> <out.json>
       payload_check.py <study dir> none <std bin> <fast bin> <out.json> a1 <run.py of the protocol commit>
       (attempt 1: no candidate file survives; the pin is searched with the protocol commit's run.py)"""
import gzip
import hashlib
import io
import json
import os
import struct
import sys
import tarfile
import calendar
import time

STUDY, CAND, STD, FAST, OUT = sys.argv[1:6]
A1 = len(sys.argv) > 6 and sys.argv[6] == "a1"
A1_RUNPY = sys.argv[7] if A1 else None
REC = os.path.join(STUDY, "attempts/a1" if A1 else "launch")
INST = os.path.join(STUDY, "../mt_fieldops_v5_20261112/instances")
sha = lambda b: hashlib.sha256(b).hexdigest()
pin = json.load(open(os.path.join(REC, "payload.json")))
create = json.load(open(os.path.join(REC, "create.json")))
res = dict(pin=pin["payload_sha256"], create_pin=create["payload_sha256"], pin_equals_create=pin["payload_sha256"] == create["payload_sha256"])
res["podjob_sha256_matches_create"] = sha(open(os.path.join(STUDY, "pod/podjob.py"), "rb").read()) == create["podjob_sha256"]
cand = open(CAND, "rb").read() if not A1 else b""
res["candidate_sha256"] = sha(cand)
res["candidate_bytes"] = len(cand)
res["candidate_is_pinned_payload"] = sha(cand) == pin["payload_sha256"] and len(cand) == pin["payload_bytes"]
runpy_bytes = open(os.path.join(STUDY, "run.py") if not A1 else A1_RUNPY, "rb").read()
inputs = {"run.py": runpy_bytes, "bin/mt_fieldops_std": open(STD, "rb").read(), "bin/mt_fieldops_fast": open(FAST, "rb").read()}
for pb in (24, 28, 32):
    for i in range(4):
        inputs[f"instances/W{pb}-{i}.json"] = open(os.path.join(INST, f"W{pb}-{i}.json"), "rb").read()
members = []
with (tarfile.open(fileobj=io.BytesIO(cand), mode="r:gz") if not A1 else tarfile.open(fileobj=io.BytesIO(b"\0" * 1024), mode="r")) as t:
    for m in t.getmembers():
        b = t.extractfile(m).read() if m.isfile() else None
        exp = inputs.get(m.name)
        members.append(dict(name=m.name, type=m.type.decode(), mode=oct(m.mode), uid=m.uid, gid=m.gid, mtime=m.mtime, size=m.size,
                            sha256=sha(b) if b is not None else None,
                            equals_validator_input=(None if exp is None else b == exp)))
res["members"] = members
res["member_names"] = [m["name"] for m in members]
res["unexpected_members"] = [m["name"] for m in members if m["name"] not in inputs]
res["missing_members"] = [k for k in inputs if k not in res["member_names"]]
res["all_members_equal_validator_inputs"] = all(m["equals_validator_input"] for m in members if m["name"] in inputs and inputs[m["name"]] is not None)
res["pinned_files_sha256_match"] = {k: (sha(inputs[k]) == v if inputs.get(k) is not None else None) for k, v in pin["files_sha256"].items()}
if A1:
    res["a1_run_py_sha256"] = sha(runpy_bytes)
    res["a1_run_py_matches_pin"] = sha(runpy_bytes) == pin["files_sha256"]["run.py"]


def build(mtime):
    tbuf = io.BytesIO()
    with tarfile.open(fileobj=tbuf, mode="w", format=tarfile.GNU_FORMAT) as t:
        order = ["run.py", "bin/mt_fieldops_std", "bin/mt_fieldops_fast"] + [f"instances/W{pb}-{i}.json" for pb in (24, 28, 32) for i in range(4)]
        for name in order:
            b = inputs[name]
            ti = tarfile.TarInfo(name)
            ti.size = len(b)
            ti.mode = 0o644 if name.startswith("instances/") else 0o755
            ti.mtime = 0
            ti.uid = ti.gid = 0
            t.addfile(ti, io.BytesIO(b))
    gbuf = io.BytesIO()
    with gzip.GzipFile(filename="", mode="wb", compresslevel=9, fileobj=gbuf, mtime=mtime) as g:
        g.write(tbuf.getvalue())
    return gbuf.getvalue()


rb = build(0)
if not A1:
    hdr_mtime = struct.unpack("<I", cand[4:8])[0]
    res["candidate_gzip_header"] = dict(mtime=hdr_mtime, mtime_utc=time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(hdr_mtime)),
                                        flags=cand[3], xfl=cand[8], os=cand[9])
    rh = build(hdr_mtime)
    res["rebuild_with_header_mtime"] = dict(sha256=sha(rh), equals_pin=sha(rh) == pin["payload_sha256"])
    window = ("2026-11-12T14:29:44Z", "2026-11-12T16:32:35Z")
else:
    hdr_mtime = None
    window = ("2026-11-12T14:29:44Z", "2026-11-12T16:30:48Z")
# independent search of the time window (no use of the candidate's header): the deflate stream does not depend on the
# header, so build once and splice the 4 mtime bytes
lo, hi = (calendar.timegm(time.strptime(w, "%Y-%m-%dT%H:%M:%SZ")) for w in window)
base = bytearray(rb)
found = None
for m in range(hi, lo - 1, -1):
    base[4:8] = struct.pack("<I", m)
    if sha(bytes(base)) == pin["payload_sha256"]:
        found = m
        break
res["rebuild_bytes"] = len(rb)
res["rebuild_bytes_equal_pin_bytes"] = len(rb) == pin["payload_bytes"]
res["window_search"] = dict(window=list(window), found_mtime=found,
                            found_utc=(time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(found)) if found else None))
res["payload_rebuilt_bit_identical"] = found is not None and (A1 or (res["rebuild_with_header_mtime"]["equals_pin"] and found == hdr_mtime))
json.dump(res, open(OUT, "w"), indent=1)
print(json.dumps({k: v for k, v in res.items() if k != "members"}, indent=1))
