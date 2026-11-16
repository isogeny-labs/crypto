#!/usr/bin/env python3
"""Second follow-up placement for VAL-20261029-33a286 (validator's own code).

Writes (no commit):
  research/ic_dg_blocked_20261029/VALIDATION.json          promotion_blockers cleared, promotion_note, followup_review_2
  benchmarks/20261029-ic-dg-blocked/VALIDATION.json        the same fields (pins unchanged)
  research/ic_dg_blocked_20261029/validation/followup/     followup.py, followup.json, place.py, followup2.py,
                                                           followup2.json, place2.py (byte copies)
  research/ic_dg_blocked_20261029/validation/SHA256SUMS    the six new lines, merged in path order
The pinned files (validator-report.json, replay-certificate.json and their benchmarks copies) are untouched.
"""
import hashlib
import json
import os
import shutil

WT = "/dev/shm/wt-blk"
STUDY = "research/ic_dg_blocked_20261029"
EV = f"{WT}/{STUDY}"
BD = f"{WT}/benchmarks/20261029-ic-dg-blocked"
F = "/dev/shm/dgb-validation/followup"
ARCH = f"{EV}/validation/followup"
FILES = ("followup.py", "followup.json", "place.py", "followup2.py", "followup2.json", "place2.py")


def sha(p):
    return hashlib.sha256(open(p, "rb").read()).hexdigest()


def dumps(obj):
    return json.dumps(obj, indent=1) + "\n"


f2 = json.load(open(f"{F}/followup2.json"))
assert f2["all_checks_hold"] and f2["head"] == f2["remote_tip"] == f2["base_commit"]

review2 = {
    "reviewed": "working-tree corrections on top of 313ec5d2af (uncommitted when reviewed)",
    "date": "2026-10-29",
    "by": "VAL-20261029-33a286 (same validator, own code)",
    "code": {"path": "validation/followup/followup2.py", "sha256": sha(f"{F}/followup2.py")},
    "output": {"path": "validation/followup/followup2.json", "sha256": sha(f"{F}/followup2.json")},
    "checks": [
        "README.md: the arm-position table equals the recomputed positions (dg and dgb-edge at 0 and 3, dg-aa and dgb at 1 and 2, "
        "144 each; 144 forward and 144 reversed blocks; dgb follows dg-aa in 144 and precedes it in 144), identical at 24, 28 and 32 "
        "bits, and cites validation/work/recompute.json",
        "README.md: the sentence 'which is why every ratio missed its prediction' is gone; its replacement states both causes and "
        "carries 0.594, 0.565, 0.451, 0.898, 0.867 and 0.851, each equal to the validator's phase arithmetic at three decimals",
        "README.md: provenance line reworded, '(see the next table)' now points to 'Counted units', PR #2844 marked as outside the "
        "evidence and unchecked; no number of the committed README was dropped except in the replaced lines",
        f"README.pdf ({f2['pdf']['readme_pdf_sha256'][:8]}...): all {f2['pdf']['readme_decimal_numbers']} decimal numbers of README.md, "
        "the position table and the new sentence are in its text, the old sentence is not, the figure is embedded; benchmarks "
        "REPORT.pdf is the same file",
        "PLACEMENT.md records dg_blocked_component outside tables A-C for the two 28-bit rows and no row for 24 and 32 bits; "
        "pair_table_component at 313ec5d2af holds set-up figures only, so this is the PROTOCOL section 10 branch",
        f"docs/ic/leaderboard.json dg_blocked_component: one 28-bit row, values equal to the recompute to "
        f"{f2['leaderboard']['max_rel_diff_vs_recompute']:.0e} (12-digit rounding), counted dgb/dg 1.0 at every weight; "
        "LEADERBOARD.md and ic-leaderboard.html carry 0.9300 [0.9266, 0.9333]; the section reads 'validated (28 bits)'",
        "benchmarks/README.md row: edge control now 'A/A-corrected 0.9302'; decision 0.9300 [0.9266, 0.9333]",
    ],
    "reviewed_files_sha256": f2["reviewed_files_sha256"],
    "clearance_condition": ("The three promotion blockers are cleared for the reviewed bytes above. They were working-tree files "
                            "when reviewed (PLACEMENT.md untracked): the clearance holds only if they are committed byte for byte "
                            "with this VALIDATION.json. A differing hash reopens the corresponding blocker."),
    "cleared_blockers": [
        "README position table (PROTOCOL section 10)",
        "README causal sentence on the prediction miss",
        "placement record (PLACEMENT.md: dg_blocked_component)",
    ],
}

NOTE = ("Evidence admissibility only for the two 28-bit rows; 24 and 32 bits are undecidable and get no row. No promotion blocker "
        "known to the validator remains, provided the files pinned in followup_review_2.reviewed_files_sha256 are committed byte "
        "for byte with this file. Promotion is the Coordinator's decision: this file records evidence admissibility, not authorization.")


def update(path, extra_note=None):
    d = json.loads(open(path).read())
    pins = (d["replay_certificate"], d["validator_report"])
    d["promotion_blockers"] = []
    d["promotion_note"] = NOTE
    if extra_note:
        d["placement_note"] = extra_note
    if "followup_review" in d:
        d["followup_review"]["archived_as"] = {"code": "validation/followup/followup.py", "output": "validation/followup/followup.json",
                                               "placement": "validation/followup/place.py"}
    d["followup_review_2"] = review2
    assert (d["replay_certificate"], d["validator_report"]) == pins
    open(path, "w").write(dumps(d))


update(f"{EV}/VALIDATION.json",
       "Placed at 313ec5d2af. The validator's draft is kept unchanged as validation/VALIDATION.draft.json; this file differs from it "
       "only in promotion_blockers, promotion_note, this note (was draft_note), followup_review and followup_review_2. "
       "validation/validator-report.json still lists the blockers as of bf899979a6; the two follow-up reviews record what changed.")
update(f"{BD}/VALIDATION.json")

# archive the follow-up scripts and outputs, then merge their lines into validation/SHA256SUMS
os.makedirs(ARCH, exist_ok=True)
for fn in FILES:
    dst = f"{ARCH}/{fn}"
    assert not os.path.exists(dst), f"refusing to overwrite {dst}"
    shutil.copyfile(f"{F}/{fn}", dst)
sums = f"{EV}/validation/SHA256SUMS"
lines = [ln for ln in open(sums).read().split("\n") if ln]
paths = [ln.split("  ", 1)[1] for ln in lines]
assert not any(p.startswith("./followup/") for p in paths)
new = [f"{sha(f'{ARCH}/{fn}')}  ./followup/{fn}" for fn in FILES]
merged = sorted(lines + new, key=lambda ln: ln.split("  ", 1)[1]) if paths == sorted(paths) else lines + new
open(sums, "w").write("\n".join(merged) + "\n")

for p in (f"{EV}/VALIDATION.json", f"{BD}/VALIDATION.json", sums, *[f"{ARCH}/{fn}" for fn in FILES]):
    print(sha(p), os.path.getsize(p), p.replace(WT + "/", ""))
