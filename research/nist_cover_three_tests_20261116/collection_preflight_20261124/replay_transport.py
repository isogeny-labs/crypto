"""Metadata-only portable replay wrapper; no curve/group/field arithmetic."""
from pathlib import Path
import hashlib
import json
import shutil
import subprocess
import sys

phase=Path(__file__).resolve().parent
qualification=Path(sys.argv[1]).resolve()
out=Path(sys.argv[2]).resolve()
assert not out.exists(),"fresh replay output required"
assert (qualification/"bin/preflight").is_file(),"run qualify.sh in a new directory first"
out.mkdir()
study=out/"isolated mirror study"
component=study/phase.name
component.mkdir(parents=True)
for name in ["transport_fixture.py","TRANSPORT_PROTOCOL.md","run-admitted.sh","preflight.cpp","typed_json.hpp","pins.hpp","PROTOCOL.md"]:
    shutil.copy2(phase/name,component/name)
staged=component/"evidence/qualification-v1"
(staged/"bin").mkdir(parents=True)
shutil.copy2(qualification/"bin/preflight",staged/"bin/preflight")
shutil.copytree(qualification/"input",staged/"input")
old=study/"split_tangent_20261123"
old.mkdir()
for name in ["collector.cpp","checker.cpp","rank_roots.hpp","PROTOCOL.md","QUALIFICATION_NOTES.md"]:
    shutil.copy2(phase.parent/old.name/name,old/name)
frozen=study/"recorded_pool_20261123"
(frozen/"evidence/qualification-v2/input").mkdir(parents=True)
(frozen/"evidence/traces-v1").mkdir(parents=True)
shutil.copy2(phase.parent/frozen.name/"evidence/qualification-v2/input/baseline.json",frozen/"evidence/qualification-v2/input/baseline.json")
shutil.copy2(phase.parent/frozen.name/"evidence/traces-v1/result.json",frozen/"evidence/traces-v1/result.json")
shutil.copy2(phase.parent/frozen.name/"TANGENT_COLLECTION_PROTOCOL.md",frozen/"TANGENT_COLLECTION_PROTOCOL.md")
source={name:hashlib.sha256((phase/name).read_bytes()).hexdigest() for name in ["replay_transport.py","transport_fixture.py","run-admitted.sh","PORTABILITY_NOTES.md"]}
for name in ["transport_fixture.py","run-admitted.sh"]:
    assert (component/name).read_bytes()==(phase/name).read_bytes()
(out/"source-bindings.json").write_text(json.dumps({"sources":source,"native_binary_sha256":hashlib.sha256((qualification/"bin/preflight").read_bytes()).hexdigest(),"helper_and_driver_byte_equal":True},indent=2)+"\n")
run=subprocess.run([sys.executable,str(component/"transport_fixture.py"),str(out/"transport result")],capture_output=True,text=True)
(out/"stdout.txt").write_text(run.stdout)
(out/"stderr.txt").write_text(run.stderr)
(out/"EXIT_CODE.txt").write_text(str(run.returncode)+"\n")
assert run.returncode==0,"inspect retained fixture failure"
result=json.loads((out/"transport result/result.json").read_text())
assert result["actual_collection_executed"] is False and result["provider_launch"] is False
(out/"result.json").write_text(json.dumps({"standing":"producer-unverified","scope":"portable-isolated-driver-transport-only","qualification_input":str(qualification),"transport":result,"actual_collection_executed":False},indent=2)+"\n")
print("Portable replay stages a fresh native qualification and preserves both frozen transport outcomes.")
