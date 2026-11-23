"""Metadata-only preservation check after the appended handoff."""
import collections
import hashlib
import json
import re
import subprocess
from pathlib import Path

ROOT = Path("research/single_relation_reduction_20261115/checks")
JOURNAL = "docs/session-logs/2026-11-15.md"
receipt = json.loads((ROOT / "journal-main001.json").read_text())


def blocks(data):
    return [s.rstrip() for s in re.split(r"(?m)(?=^## )", data) if s.startswith("## ")]


def read(ref):
    return subprocess.check_output(["git", "show", f"{ref}:{JOURNAL}"], text=True)


def subsequence(needle, haystack):
    cursor = iter(haystack)
    return all(any(item == candidate for candidate in cursor) for item in needle)


incoming = blocks(read(receipt["main"]))
own = blocks(read(receipt["head"]))
actual_data = Path(JOURNAL).read_text()
actual = blocks(actual_data)
handoff = blocks(Path("/workspace/scratch/single-relation-handoff001.md").read_text())
assert len(handoff) == 1
expected = (collections.Counter(incoming) | collections.Counter(own)) + collections.Counter(handoff)
assert collections.Counter(actual) == expected
assert subsequence(incoming, actual) and subsequence(own, actual)
protected = [
    "research/single_relation_reduction_20261115",
    "research/big_prime_halving_20261115",
    "research/critical_pair_norm_20261115",
]
allowed = {
    "research/single_relation_reduction_20261115/RESUME.md",
    "research/single_relation_reduction_20261115/PUBLICATION.md",
    "research/single_relation_reduction_20261115/CONTINUATION.md",
}
changed = subprocess.check_output(
    ["git", "diff", "--name-only", "--diff-filter=DMRTUXB", receipt["head"], "--", *protected],
    text=True,
).splitlines()
assert not (set(changed) - allowed), "an existing protected file changed"
result = {
    "head_before_merge": receipt["head"],
    "main": receipt["main"],
    "main_blocks": len(incoming),
    "own_blocks": len(own),
    "blocks_after_handoff": len(actual),
    "all_complete_block_multiplicities_preserved": True,
    "both_input_sequences_preserved": True,
    "existing_math_report_and_native_component_files_preserved": True,
    "allowed_publication_metadata": sorted(allowed),
    "handoff_header": handoff[0].splitlines()[0],
    "journal_sha256": hashlib.sha256(actual_data.encode()).hexdigest(),
    "checker_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
}
with (ROOT / "journal-post-handoff001.json").open("x") as stream:
    json.dump(result, stream, indent=2)
    stream.write("\n")
print(json.dumps(result, indent=2))
