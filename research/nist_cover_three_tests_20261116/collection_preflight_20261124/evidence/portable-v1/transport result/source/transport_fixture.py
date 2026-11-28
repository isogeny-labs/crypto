"""Metadata-only isolated driver fixture. No curve/group/field arithmetic.

The real driver and native input guard run, but all scientific dependencies
are shell sentinels. The simulated envelope is explicitly not admission.
"""
from pathlib import Path
import hashlib
import json
import shutil
import subprocess
import sys

phase=Path(__file__).resolve().parent
out=Path(sys.argv[1]).resolve()
assert not out.exists(),"fresh fixture output required"
out.mkdir()
(out/"source").mkdir()
for name in ["transport_fixture.py","TRANSPORT_PROTOCOL.md","run-admitted.sh"]:
    shutil.copy2(phase/name,out/"source"/name)
reference=phase.parent/"recorded_pool_20261123/evidence/qualification-v2/input/baseline.json"
actual=phase/"evidence/qualification-v1/input/actual.json"
construction=phase/"evidence/qualification-v1/input/construction"
binary=phase/"evidence/qualification-v1/bin/preflight"
old=phase.parent/"split_tangent_20261123"

old_qualifier=r'''#!/bin/bash
set -euo pipefail
test "$#" -eq 1
origin=$(cd "$(dirname "$0")" && pwd)
mkdir "$1"
mkdir "$1/bin" "$1/source"
cp "$origin/../fixtures/collector-stub.sh" "$1/bin/collector"
cp "$origin/../fixtures/checker-stub.sh" "$1/bin/checker"
cp "$origin/collector.cpp" "$1/source/collector.cpp"
cp "$origin/checker.cpp" "$1/source/checker.cpp"
chmod 0555 "$1/bin/collector" "$1/bin/checker"
'''
preflight_qualifier=r'''#!/bin/bash
set -euo pipefail
test "$#" -eq 1
origin=$(cd "$(dirname "$0")" && pwd)
mkdir "$1"
mkdir "$1/bin" "$1/source"
cp "$origin/../fixtures/preflight" "$1/bin/preflight"
cp "$origin/preflight.cpp" "$origin/typed_json.hpp" "$origin/pins.hpp" "$1/source/"
chmod 0555 "$1/bin/preflight"
'''
baseline_stub=r'''#!/bin/bash
set -euo pipefail
test "$#" -eq 1
origin=$(cd "$(dirname "$0")" && pwd)
mkdir "$1"
mkdir "$1/relations"
cp "$origin/fixtures/actual.json" "$1/relations/result.json"
cp -r "$origin/fixtures/construction" "$1/relations/input"
printf '%s\n' 'baseline fixture only; no mathematical replay' > "$1/BASELINE_STUB.txt"
'''
collector_stub=r'''#!/bin/bash
set -euo pipefail
test "$#" -eq 5
test "$1" = --collect
printf '%s\n' '{"scope":"collector-sentinel-only","production_collection_executed":false}' > "$5"
printf '%s\n' 'sentinel entered; no scientific code' > "$5.entered"
'''
checker_stub=r'''#!/bin/bash
set -euo pipefail
test "$#" -eq 7
test "$1" = --check
test -f "$6"
printf '%s\n' '{"scope":"checker-sentinel-only","cryptographic_verification_executed":false}' > "$7"
printf '%s\n' 'sentinel entered; no scientific code' > "$7.entered"
'''

def script(path,text):
    path.write_text(text)
    path.chmod(0o555)

cases=[]
for label,corrupt in [("recorded pass",False),("recorded drift",True)]:
    case=out/label
    study=case/"fixture study"
    component=study/"collection_preflight_20261124"
    component.mkdir(parents=True)
    for name in ["run-admitted.sh","preflight.cpp","typed_json.hpp","pins.hpp","PROTOCOL.md"]:
        shutil.copy2(phase/name,component/name)
    script(component/"qualify.sh",preflight_qualifier)
    old_component=study/"split_tangent_20261123"
    old_component.mkdir()
    for name in ["collector.cpp","checker.cpp","rank_roots.hpp","PROTOCOL.md","QUALIFICATION_NOTES.md"]:
        shutil.copy2(old/name,old_component/name)
    script(old_component/"qualify.sh",old_qualifier)
    fixture=study/"fixtures"
    fixture.mkdir()
    shutil.copytree(construction,fixture/"construction")
    shutil.copy2(binary,fixture/"preflight")
    if corrupt:
        payload=json.loads(actual.read_text())
        payload["rank_mod_r"]=0  # literal metadata corruption; no rank computation
        (fixture/"actual.json").write_text(json.dumps(payload,separators=(",",":"))+"\n")
    else:
        shutil.copy2(actual,fixture/"actual.json")
    script(fixture/"collector-stub.sh",collector_stub)
    script(fixture/"checker-stub.sh",checker_stub)
    script(study/"check-relation-e2e.sh",baseline_stub)
    frozen=study/"recorded_pool_20261123"
    (frozen/"evidence/qualification-v2/input").mkdir(parents=True)
    (frozen/"evidence/traces-v1").mkdir(parents=True)
    shutil.copy2(reference,frozen/"evidence/qualification-v2/input/baseline.json")
    shutil.copy2(phase.parent/"recorded_pool_20261123/evidence/traces-v1/result.json",frozen/"evidence/traces-v1/result.json")
    shutil.copy2(phase.parent/"recorded_pool_20261123/TANGENT_COLLECTION_PROTOCOL.md",frozen/"TANGENT_COLLECTION_PROTOCOL.md")
    envelope=case/"simulated envelope.json"
    envelope.write_text('{"scope":"transport-fixture-only","admission":false,"do_not_use_for_launch":true}\n')
    cases.append((case,component,envelope,corrupt))

bindings=[]
for path in sorted(out.rglob("*")):
    if path.is_file():
        bindings.append({"path":str(path.relative_to(out)),"sha256":hashlib.sha256(path.read_bytes()).hexdigest()})
(out/"pre-call-manifest.json").write_text(json.dumps(bindings,indent=2)+"\n")
results=[]
for case,component,envelope,corrupt in cases:
    destination=case/"driver output"
    run=subprocess.run(["bash",str(component/"run-admitted.sh"),"--admitted",str(envelope),str(destination)],cwd=component,capture_output=True,text=True)
    (case/"driver.stdout").write_text(run.stdout)
    (case/"driver.stderr").write_text(run.stderr)
    collector=(destination/"result.json.entered").exists()
    checker=(destination/"checker-result.json.entered").exists()
    preflight=int((destination/"PREFLIGHT_EXIT_CODE.txt").read_text())
    pipeline=int((destination/"PIPELINE_EXIT_CODE.txt").read_text())
    expected=1 if corrupt else 0
    assert run.returncode==expected and preflight==expected and pipeline==expected
    assert collector==(not corrupt) and checker==(not corrupt)
    if corrupt:
        assert not (destination/"result.json").exists() and not (destination/"preflight-result.json").exists()
    else:
        record=json.loads((destination/"preflight-result.json").read_text())
        assert record["mode"]=="check" and record["admission_validated"] is False
    results.append({"case":case.name,"driver_exit":run.returncode,"preflight_exit":preflight,"pipeline_exit":pipeline,
                    "collector_sentinel_entered":collector,"checker_sentinel_entered":checker,
                    "real_guard_executed":True,"baseline_collector_checker_are_stubs":True})
(out/"result.json").write_text(json.dumps({"standing":"producer-unverified","scope":"isolated-driver-transport-only",
    "cases":results,"actual_collection_executed":False,"curve_or_Jacobian_arithmetic_executed":False,
    "provider_launch":False,"admission_validated":False},indent=2)+"\n")
print("Two isolated driver transport cases matched their frozen outcomes.")
