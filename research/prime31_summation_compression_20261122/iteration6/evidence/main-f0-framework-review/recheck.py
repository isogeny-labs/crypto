"""Revalidate the four-line restoration against pinned main; source/parser checks only."""
from pathlib import Path
import difflib
import hashlib
import json
import re
import subprocess

HERE = Path(__file__).resolve().parent
SOURCE = HERE.parent / "ci_ic_framework_audit"
PINNED_MAIN = "f0ca664eca0081a30adfcc79dcf62a32de40d2af"


def sha256(data):
    return hashlib.sha256(data).hexdigest()


def git_blob(data):
    return hashlib.sha1(b"blob " + str(len(data)).encode() + b"\0" + data).hexdigest()


current = (SOURCE / "mod.current.rs").read_bytes()
restored = (SOURCE / "mod.restored.rs").read_bytes()
assert git_blob(current) == "8c6fbde25df09ad504877f780f4bdbf6acbbd002"
assert git_blob(restored) == "f4f2cce4118d1510fd961cc6d6e0fc0354c257e1"

current_lines = current.decode().splitlines(keepends=True)
restored_lines = restored.decode().splitlines(keepends=True)
changes = []
for tag, i, j, k, l in difflib.SequenceMatcher(None, current_lines, restored_lines, autojunk=False).get_opcodes():
    if tag == "equal":
        continue
    assert tag == "insert"
    changes.append({"current_before_line": i + 1, "inserted_lines": restored_lines[k:l]})
assert sum(len(row["inserted_lines"]) for row in changes) == 4
assert [line for row in changes for line in row["inserted_lines"]] == [
    "        || fb\n",
    "            .values()\n",
    "            .any(|&index| index >= fb.points.len())\n",
    "    }\n",
]

candidate_text = restored.decode()
p1 = (SOURCE / "mod.culprit_parent1.rs").read_text()
p2 = (SOURCE / "mod.culprit_parent2.rs").read_text()


def control(text, start, end):
    left = text.index(start)
    right = text.index(end, left) + len(end)
    return text[left:right]


validation_start = "    if fb.points.len() != fb.col_of.len()"
validation_end = '        return Err("factor-base plug-in returned inconsistent point/column indexes".into());\n    }\n'
validation = control(candidate_text, validation_start, validation_end)
assert validation == control(p1, validation_start, validation_end)
lean_start = '    if fb.index_elided() && oracle.name() != "mitm-frobenius-layout-counted" {'
lean_end = "            oracle.name()\n        ));\n    }\n"
lean = control(candidate_text, lean_start, lean_end)
assert lean == control(p2, lean_start, lean_end)

tests_re = r"#\[test\](?:\s*#\[[^\]]*\])*\s*fn\s+([A-Za-z0-9_]+)"
tests_current = re.findall(tests_re, current.decode())
tests_candidate = re.findall(tests_re, candidate_text)
assert tests_current == tests_candidate
assert len(tests_candidate) == candidate_text.count("#[test]") == 35
test_start = current.index(b"#[cfg(test)]")
candidate_test_start = restored.index(b"#[cfg(test)]")
assert current[test_start:] == restored[candidate_test_start:]

parser_results = []
for name, expected in (("current", 1), ("restored", 0)):
    command = ["rustfmt", "--edition", "2021", "--emit", "stdout", "--config", "skip_children=true", str(SOURCE / f"mod.{name}.rs")]
    result = subprocess.run(command, capture_output=True)
    (HERE / f"{name}.rustfmt.rs").write_bytes(result.stdout)
    (HERE / f"{name}.parser.log").write_bytes(result.stderr)
    assert result.returncode == expected, (name, result.returncode)
    if name == "restored":
        assert not result.stderr
    parser_results.append({"input": name, "command": command, "exit_code": result.returncode, "stderr_sha256": sha256(result.stderr)})

receipt = {
    "pinned_main": PINNED_MAIN,
    "remote_path": "src/cryptanalysis/ic_framework/mod.rs",
    "remote_blob_verified_by_pinned_github_fetch": git_blob(current),
    "current_sha256": sha256(current),
    "candidate_blob": git_blob(restored),
    "candidate_sha256": sha256(restored),
    "diff": {"added_lines": 4, "deleted_lines": 0, "hunks": changes},
    "validation_guard_exact_parent1": True,
    "lean_layout_guard_exact_parent2": True,
    "test_module_entire_suffix_bytes_unchanged": True,
    "test_declarations": tests_candidate,
    "test_declaration_count": len(tests_candidate),
    "arithmetic_and_accounting_changes": "none; entire difference is the four restored guard lines",
    "parser_results": parser_results,
    "rustfmt_version": subprocess.run(["rustfmt", "--version"], capture_output=True, text=True, check=True).stdout.strip(),
    "build_or_crypto_execution": False,
    "remote_or_source_mutations": False,
}
(HERE / "receipt.json").write_text(json.dumps(receipt, indent=2) + "\n")
print(json.dumps({"candidate_blob": receipt["candidate_blob"], "added_lines": 4, "deleted_lines": 0, "tests_preserved": len(tests_candidate), "parser_exits": {x["input"]: x["exit_code"] for x in parser_results}, "receipt_sha256": sha256((HERE / "receipt.json").read_bytes())}, indent=2))
