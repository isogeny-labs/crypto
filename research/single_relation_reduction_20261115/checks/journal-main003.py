"""Journal metadata reconciliation only; no scientific computation.

Preserve complete normalized blocks and multiplicities. Incoming order is
unchanged; own additions go before the first later timed incoming block.
Untimed headings keep their source order, without an imputed timestamp.
"""
import collections
import hashlib
import json
import re
import subprocess
from pathlib import Path

JOURNAL = "docs/session-logs/2026-11-15.md"
PREVIEW = Path("/workspace/scratch/single-relation-journal-preview003.md")
RECEIPT = Path("/workspace/scratch/single-relation-journal-preview003.json")


def git(*args):
    return subprocess.check_output(["git", *args], text=True).strip()


def read(ref):
    data = subprocess.check_output(["git", "show", f"{ref}:{JOURNAL}"], text=True)
    parts = re.split(r"(?m)(?=^## )", data)
    return (
        "".join(s for s in parts if not s.startswith("## ")),
        [s for s in parts if s.startswith("## ")],
    )


def canon(block):
    return block.rstrip()


def stamp(block):
    match = re.search(
        r"(?:T|\s)(\d\d):(\d\d)(?::(\d\d))?(?:Z|\s|$)", block.splitlines()[0]
    )
    return tuple(int(s or 0) for s in match.groups()) if match else None


def subsequence(needle, haystack):
    cursor = iter(haystack)
    return all(any(item == candidate for candidate in cursor) for item in needle)


head, main = git("rev-parse", "HEAD"), git("rev-parse", "origin/main")
base = git("merge-base", head, main)
op, ob = read(head)
mp, mb = read(main)
assert op == mp, "journal prefixes differ"
om = collections.Counter(map(canon, ob))
mm = collections.Counter(map(canon, mb))
remaining = mm.copy()
extra = []
extra_indices = []
for index, block in enumerate(ob):
    key = canon(block)
    if remaining[key]:
        remaining[key] -= 1
    else:
        extra.append(block)
        extra_indices.append(index)
assert extra, "no own additions to reconcile"
anchor = next(canon(s) for s in reversed(ob[: extra_indices[0]]) if mm[canon(s)])
split = max(i for i, s in enumerate(mb) if canon(s) == anchor) + 1
tail = list(mb[split:])
insertions = []
for block in extra:
    moment = stamp(block)
    position = len(tail)
    if moment is not None:
        for i, candidate in enumerate(tail):
            later = stamp(candidate)
            if later is not None and later > moment:
                position = i
                break
    insertions.append({"header": block.splitlines()[0], "tail_position": position})
    tail.insert(position, block)
blocks = mb[:split] + tail
counts = collections.Counter(map(canon, blocks))
assert counts == (mm | om), "full-block multiset union differs"
assert not (mm - counts), "incoming complete block lost"
assert not (om - counts), "own complete block lost"
incoming_order = subsequence(list(map(canon, mb)), list(map(canon, blocks)))
own_order = subsequence(list(map(canon, ob)), list(map(canon, blocks)))
assert incoming_order and own_order, "a source block sequence was reordered"
merged = mp + "".join(canon(s) + "\n\n" for s in blocks)
receipt = {
    "head": head,
    "main": main,
    "merge_base": base,
    "main_blocks": len(mb),
    "own_blocks": len(ob),
    "own_extra": len(extra),
    "merged_blocks": len(blocks),
    "main_duplicate_headers": len(mb) - len(set(s.splitlines()[0] for s in mb)),
    "own_duplicate_headers": len(ob) - len(set(s.splitlines()[0] for s in ob)),
    "anchor": anchor.splitlines()[0],
    "insertions": insertions,
    "main_all_complete_blocks_and_multiplicities_preserved": True,
    "own_all_complete_blocks_and_multiplicities_preserved": True,
    "main_block_order_preserved": incoming_order,
    "own_block_order_preserved": own_order,
    "normalization": "trailing whitespace at block boundaries only",
    "untimed_blocks": "preserve incoming sequence; no timestamp imputed",
    "preview_sha256": hashlib.sha256(merged.encode()).hexdigest(),
    "checker_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
}
with PREVIEW.open("x") as output:
    output.write(merged)
with RECEIPT.open("x") as output:
    json.dump(receipt, output, indent=2)
    output.write("\n")
print(json.dumps(receipt, indent=2))
