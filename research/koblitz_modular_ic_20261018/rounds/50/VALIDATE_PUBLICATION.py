"""Check historical Git pins, raw hashes and actual receipts; no scientific replay."""
from pathlib import Path
import hashlib
import json
import subprocess

h = Path(__file__).resolve().parent
repo = h.parents[3]
revision = (h / "source-revision.txt").read_text().strip()
pins = [line.split(maxsplit=1) for line in (h / "source.sha256").read_text().splitlines()]
names = [str((h / name).resolve().relative_to(repo)) for _, name in pins]
requests = "".join(revision + ":" + name + "\n" for name in names)
data = subprocess.check_output(["git", "cat-file", "--batch"], cwd=repo, input=requests.encode())
offset = 0
for (expected, name) in pins:
    end = data.index(b"\n", offset)
    header = data[offset:end].split()
    assert len(header) == 3 and header[1] == b"blob", name
    size = int(header[2])
    blob = data[end + 1:end + 1 + size]
    assert hashlib.sha256(blob).hexdigest() == expected, name
    assert hashlib.sha256((h / name).read_bytes()).hexdigest() == expected, name
    offset = end + 2 + size
assert offset == len(data)
rows = json.loads((h / "results.json").read_text())["runs"]
for row in rows:
    p = h / "runs" / row["attempt"]
    if row["kind"] == "Lean":
        assert int((p / "compile.exit").read_text()) == row["compile_exit"]
        if row["audit_exit"] is not None:
            assert int((p / "audit.exit").read_text()) == row["audit_exit"]
        else:
            assert not (p / "audit.exit").exists()
    else:
        assert int((p / "exit.txt").read_text()) == row["native_exit"]
    for line in (p / "source.sha256").read_text().splitlines():
        expected, name = line.split(maxsplit=1)
        assert hashlib.sha256((p / Path(name).name).read_bytes()).hexdigest() == expected
artifact = json.loads((h / "artifact-verification.json").read_text())
assert artifact["all_member_hashes_verified"] and artifact["deterministic_repack_verified"]
out = {"schema": "koblitz.publication-validation/v1", "round": 50,
       "historical_Git_source_pins_checked": len(pins), "terminal_attempts_checked": len(rows),
       "raw_member_and_repack_check": True, "scientific_replay": False, "semantic_review": False}
(h / "publication-validation.json").write_text(json.dumps(out, indent=2) + "\n")
print(json.dumps(out))
