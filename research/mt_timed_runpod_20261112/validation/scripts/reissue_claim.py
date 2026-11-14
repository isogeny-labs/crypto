#!/usr/bin/env python3
"""VALIDATOR re-issue of claim-report.json for research/mt_timed_runpod_20261112 at commit dd80a7c4d7.

The producer adopted the proposed wording of issue 1 (claim-report.json at dd80a7c4d7, `proposed_wording`) as its
statement.  This script judges that text clause by clause against recompute.json (the validator's recompute, re-run at
dd80a7c4d7 and byte-identical to the committed file), checks that raw data, harness, instances and the validated
evidence are unchanged since 43fc3b0e61, records how the RESULTS.md amendments were applied, and writes the re-issued
claim-report.json with issue 1 in `history`.
usage: reissue_claim.py <repo> <out claim-report.json>"""
import hashlib
import json
import subprocess
import sys

REPO, OUT = sys.argv[1:3]
VALIDATED = "dd80a7c4d7deb4f6e64eeca849c6a4a877c8d86b"
DATA = "43fc3b0e61907dbc12b95ddd471678535311b011"
ST = "research/mt_timed_runpod_20261112"
V = ST + "/validation"
WHO = "independent validator subagent (claude-opus-5-5), 2026-11-12; validator code in validation/scripts/, not the producer's"


def git(*a, text=True):
    return subprocess.run(["git", "-C", REPO] + list(a), capture_output=True, text=text, check=True).stdout


def show(c, p):
    return subprocess.run(["git", "-C", REPO, "show", f"{c}:{p}"], capture_output=True, check=True).stdout


sha = lambda b: hashlib.sha256(b).hexdigest()
issue1_bytes = show(VALIDATED, V + "/claim-report.json")
issue1 = json.loads(issue1_bytes)
stmt = issue1["proposed_wording"]
stmt_sha = sha(stmt.encode())
R = json.loads(show(VALIDATED, V + "/recompute.json"))

# ---------------------------------------------------------------- unchanged since the data commit
paths = [ST + "/results", ST + "/run.py", ST + "/analyze.py", ST + "/mk_runs.py", ST + "/launch", ST + "/launch.py",
         ST + "/attempts", ST + "/pod", "examples", "Cargo.toml", "Cargo.lock", "src", "crates",
         "research/mt_fieldops_v5_20261112/instances", "research/mt_timed_20261112"]
changed = [p for p in paths if subprocess.run(["git", "-C", REPO, "diff", "--quiet", DATA, VALIDATED, "--", p]).returncode != 0]
study_changes = git("diff", "--name-only", DATA, VALIDATED, "--", ST).split()
protocol_diff = [l for l in git("diff", "-U0", DATA, VALIDATED, "--", ST + "/PROTOCOL.md").splitlines()
                 if l[:1] in "+-" and not l.startswith(("+++", "---"))]
evidence = {n: sha(show(VALIDATED, V + "/" + n)) for n in ("replay-certificate.json", "validator-report.json", "recompute.json",
                                                         "results_check.json", "ec_check.json", "replay_check.json")}

# ---------------------------------------------------------------- clause checks
cells = {b: {(c["bits"], c["T"]): c for c in R[b]["cells"]} for b in ("std", "fast")}
size = lambda b, pb: R[b]["size"][str(pb)]
fs = R["fast_over_std"]
icfs = [r["ic"]["median"] for r in fs]
rhofs = [r["rho"]["median"] for r in fs]
bs = [c["ic_bsgs_solve"]["median"] for b in ("std", "fast") for c in R[b]["cells"]]
single = [v for b in ("std", "fast") for pb in (24, 28) for v in size(b, pb)["config_range"]]
dec = lambda b, pb: [cells[b][(pb, T)]["ic_rho_solve"]["decision"] for T in (1024, 4096, 16384)]
ci_is_range = all(c[f]["protocol_ci"] == c[f]["range"] == (c[f].get("exact_ci") or c[f]["range"])
                  for b in ("std", "fast") for c in R[b]["cells"] for f in ("ic_rho_solve",))
H = []


def h(name, claimed, rec, nd=2):
    H.append(dict(name=name, claimed=claimed, recomputed=rec, rel_tol=0.5 * 10 ** (-nd) / abs(claimed) + 1e-12))


for b, pb, lo, hi in (("std", 24, 0.70, 0.88), ("fast", 24, 0.71, 0.89), ("std", 28, 0.79, 0.95), ("fast", 28, 0.74, 0.94),
                      ("std", 32, 1.05, 1.18), ("fast", 32, 1.01, 1.15)):
    h(f"{b} {pb}b cell-median IC/rho min", lo, size(b, pb)["cell_median_range"][0])
    h(f"{b} {pb}b cell-median IC/rho max", hi, size(b, pb)["cell_median_range"][1])
for b, pb, n in (("std", 24, 12), ("fast", 24, 12), ("std", 28, 12), ("fast", 28, 12)):
    H.append(dict(name=f"{b} {pb}b configurations below 1", claimed=n, recomputed=size(b, pb)["below_1"], rel_tol=1e-9))
for b, n in (("std", 12), ("fast", 11)):
    H.append(dict(name=f"{b} 32b configurations above 1", claimed=n, recomputed=size(b, 32)["above_1"], rel_tol=1e-9))
h("single configurations 2^24-2^28 min", 0.70, min(single)); h("single configurations 2^24-2^28 max", 0.99, max(single))
h("IC time cut by fast hasher, min % (cell median)", 4, 100 * (1 - max(icfs)), 0)
h("IC time cut by fast hasher, max % (cell median)", 11, 100 * (1 - min(icfs)), 0)
h("rho time cut by fast hasher, max % (cell median)", 15, 100 * (1 - min(rhofs)), 0)
h("rho 2^32 T=1024 fast slower by %", 0.3, 100 * (fs[6]["rho"]["median"] - 1), 1)
h("IC/BSGS cell-median min", 1.41, min(bs)); h("IC/BSGS cell-median max", 1.62, max(bs))
h("round-1 drift (median key time rel. to key median)", 1.03, R["pooled_drift"]["median_rel_by_round"][0])
mism = [x for x in H if abs(x["claimed"] - x["recomputed"]) > x["rel_tol"] * abs(x["claimed"]) + 1e-12]
rho_slower_cells = [f"{r['bits']}/{r['T']}" for r in fs if r["rho"]["median"] > 1]

clauses = [
    ["On one pinned CPU of a RunPod Secure Cloud cpu5c pod (AMD EPYC 9575F; a shared cloud host, so the seconds are diagnostic)", True,
     "launch/create.json cloudType SECURE, cpuFlavorId cpu5c; host.json 'AMD EPYC 9575F 64-Core Processor', pin 91 within cpuset 27,56,91,120"],
    ["in this harness's implementation, 36 configurations (the 12 prime-order Weierstrass curves of the v5 study, group orders about 2^24, 2^28 and 2^32, seed 43, T = 1024/4096/16384) x 17 run keys x 3 interleaved repeats for each of two builds", True,
     "3672 rows = 2 x 36 x 17 x 3 in the pre-registered order; seed 43; instances prime order, cofactor 1 (ec_check.json)"],
    ["(std HashMap with SipHash; a fast multiplicative hasher, with identical counted operations)", True,
     "source_diff.json (alias/feature only); binaries rebuilt bit-identical; 1224/1224 keys replay exactly; counters equal across builds"],
    ["every target verified", True, "ok = targets in all 3672 rows; 301,056 recovered logs re-verified with validator EC code, 0 failures"],
    ["per (size, T) cell, the median over its 4 configurations of the solve-phase wall time of the fastest of four multi-target index-calculus variants divided by that of the fastest of ten multi-target rho runs", True,
     "definition matches recompute.py: per configuration min over ic5/ic5ks/ic5w/ic5rw key times / min over 4 rho2xks + 6 rhodpu key times (key time = median of 3 repeats of ms_arm); median over the cell's 4 configurations"],
    ["is 0.70-0.88 (std) and 0.71-0.89 (fast) at 2^24 and 0.79-0.95 and 0.74-0.94 at 2^28", True,
     "cell medians std 24 %.4f-%.4f, fast 24 %.4f-%.4f, std 28 %.4f-%.4f, fast 28 %.4f-%.4f" % tuple(
         v for b, pb in (("std", 24), ("fast", 24), ("std", 28), ("fast", 28)) for v in size(b, pb)["cell_median_range"])],
    ["(faster in all 3 cells and in 12 of 12 configurations at each size and build; single configurations 0.70-0.99)",
     all(d == "faster" for b in ("std", "fast") for pb in (24, 28) for d in dec(b, pb)) and all(size(b, pb)["below_1"] == 12 for b in ("std", "fast") for pb in (24, 28)),
     "decisions faster in 6/6 cells; 12/12 below 1 at 2^24 and 2^28 in both builds; single configurations %.4f-%.4f" % (min(single), max(single))],
    ["and 1.05-1.18 at 2^32 in the std build (slower in all 3 cells; 12 of 12 configurations above 1)",
     dec("std", 32) == ["slower"] * 3 and size("std", 32)["above_1"] == 12,
     "cell medians %.4f-%.4f; decisions %s; above 1: %d/12" % (*size("std", 32)["cell_median_range"], dec("std", 32), size("std", 32)["above_1"])],
    ["and 1.01-1.15 in the fast build (slower in 2 cells, unranked at T = 1024; 11 of 12 configurations above 1)",
     dec("fast", 32) == ["unranked", "slower", "slower"] and size("fast", 32)["above_1"] == 11,
     "cell medians %.4f-%.4f; decisions (T = 1024, 4096, 16384) %s; above 1: %d/12" % (*size("fast", 32)["cell_median_range"], dec("fast", 32), size("fast", 32)["above_1"])],
    ["Each cell decision is a 4-of-4 sign rule (the bootstrap interval of a median of 4 equals their range)", ci_is_range,
     "protocol bootstrap interval == exact 4^4 bootstrap interval == range of the 4 configuration ratios in all 18 build-cells"],
    ["By cell median, the fast hasher cuts the solve time of the fastest IC by 4-11%", True,
     "IC fast/std cell medians %.4f-%.4f (%.1f-%.1f%% less time)" % (min(icfs), max(icfs), 100 * (1 - max(icfs)), 100 * (1 - min(icfs)))],
    ["and of the fastest rho by up to 15% (0.3% slower in the 2^32, T = 1024 cell)", rho_slower_cells == ["32/1024"],
     "rho fast/std cell medians %.4f-%.4f (up to %.1f%% less time); the only cell above 1 is %s (%.4f)" % (min(rhofs), max(rhofs), 100 * (1 - min(rhofs)), rho_slower_cells, fs[6]["rho"]["median"])],
    ["and the IC takes 1.41-1.62x the time of a paired BSGS", True, "IC/BSGS cell medians %.4f-%.4f over both builds (fastest of 3 bsgs2 keys per configuration)" % (min(bs), max(bs))],
    ["the A/A control drifted (the first of three interleaved rounds about 3% slower), disclosed", True,
     "median key time relative to its key median by round %s (both builds); RESULTS 'Controls and their limits'" % [round(x, 4) for x in R["pooled_drift"]["median_rel_by_round"]]],
]
ok = all(c[1] is True for c in clauses) and not mism and not changed

amend = [
    dict(id="R-F1", applied=True, note="RESULTS: 'These ranges are per configuration; the table gives the cell medians ... 4-of-4 sign rule'; the statement says 'per (size, T) cell, the median over its 4 configurations' and states the sign rule"),
    dict(id="R-F2", applied=True, note="statement: 'slower in 2 cells, unranked at T = 1024'; RESULTS already stated it"),
    dict(id="R-F3", applied=True, note="'load average 37.6-93.5' (twice)"),
    dict(id="R-F4", applied=True, note="'whether the pinned CPU's SMT sibling was idle was not recorded'"),
    dict(id="R-F5", applied=True, note="prediction 1: 'magnitudes within 10% in 5 of 9 cells (up to 21% off, at 28 b T = 4096)' (recomputes: 4 cells beyond 10%, 28/4096 0.790); prediction 2 now quotes PROTOCOL.md line 50 '(ic5rw about 0.6x, ic5ks about 0.8x)' and reports ic5rw about 0.85x; ic5ks (0.84, holds) and rho2xks (0.84, not predicted) are not reported (minor)"),
    dict(id="R-F6", applied=True, note="'1.03-1.19x'; rho2xks fastest in 1 of 4 configurations at 28 b T = 1024 (fast)"),
    dict(id="R-F7", applied="partly", note="0.020 without the self-term and the fast 2^32/1024 cell vs drift are added; '(0.020 without it; 95th percentile 0.11)': 0.11 is the percentile with the self-term (0.12 without); '2^24 (12-30%)' is the std build only (fast 11-29%) and 'most 2^32 cells (5-20%)' still mixes cell medians (5-18%) with a per-configuration maximum (19.5%)"),
    dict(id="R-F8", applied=True, note="PROTOCOL.md A1 heading 16:40Z -> 16:32Z; the only PROTOCOL.md change since 43fc3b0e61"),
    dict(id="R-F9", applied=False, note="not applied; the producer says 'the artifact-store bot moves the runs.jsonl files when the PR opens', but .github/workflows/artifacts.yml only fails a PR that adds such a file (it 'touches no remote'); it passes only under the owner/reviewer label large-file-ok. Still open"),
    dict(id="R-F10", applied=True, note="Limits now state that the payload is reproducible only with the gzip time; host.json commit field not added (future runs)"),
    dict(id="R-F11", applied=False, note="producer keeps consumerUserId (redaction would change receipt hashes the validation pins) and flags it to the owner; acceptable"),
    dict(id="RESULTS validation sentence", applied=None, note="RESULTS says 'its amendments are applied here'; R-F9 is not applied and R-F7 only partly (non-blocking wording)"),
]

out = dict(
    schema="validator.report/v1", independent=True, validator=WHO, validated_commit=VALIDATED,
    data_commit_note=("raw data, harness, launcher, receipts, instances and the build inputs are byte-identical to 43fc3b0e61907dbc12b95ddd471678535311b011, "
                      "where they were validated; dd80a7c4d7 changes RESULTS.md, PROTOCOL.md (A1 time only), REPORT.pdf, ratios.svg, the bench record and adds validation/"),
    scope="this one statement exactly as worded (the validator's proposed wording from issue 1, adopted byte for byte by the producer); nothing else in the study",
    statement_judged=stmt, statement_sha256=stmt_sha,
    status="pass" if ok else "fail", disposition="validated" if ok else "insufficient_evidence",
    verdict=("endorsed exactly as worded: every clause recomputes from runs.json and the validator's replays at dd80a7c4d7 (data unchanged since 43fc3b0e61)"
             if ok else "not endorsed: see clause_checks / headline_mismatches / checks.changed_since_data_commit"),
    raw_data=[dict(path="../results/holdout/runs.json", sha256=sha(show(VALIDATED, ST + "/results/holdout/runs.json"))),
              dict(path="../results/holdout/opcost.jsonl", sha256=sha(show(VALIDATED, ST + "/results/holdout/opcost.jsonl")))],
    supporting=dict(replay_certificate="replay-certificate.json (status verified; unchanged, sha256 %s)" % evidence["replay-certificate.json"],
                    validator_report="validator-report.json (status pass; unchanged, sha256 %s)" % evidence["validator-report.json"],
                    recompute="recompute.json (re-run at dd80a7c4d7: byte-identical)", results_check="results_check.json"),
    clause_checks=[dict(clause=c, ok=o, recomputed=r) for c, o, r in clauses],
    headline=H, headline_mismatches=mism,
    checks=dict(changed_since_data_commit=changed, study_files_changed=study_changes, protocol_md_diff=protocol_diff,
                validation_evidence_sha256_at_head=evidence,
                statement_equals_issue1_proposed_wording=True,
                sha256_correction=("issue 1's VALIDATOR_REPORT.md and the validator's hand-off cited a8be619270b5eee0c323cd8d25535e3f043e3e08b6fd66351df30cd192a23ef9 "
                                   "for the proposed wording; that is the hash of an earlier draft ('fastest rho by 0-15%'). The proposed_wording in claim-report.json "
                                   "(and quoted in VALIDATOR_REPORT.md) is the final text, sha256 " + stmt_sha + ", as its own proposed_wording_sha256 field says. "
                                   "This statement is that final text."),
                amendments_check=amend),
    history=[dict(issue=1, commit=VALIDATED + " (committed by the producer; written by the validator against 43fc3b0e61)", status=issue1["status"],
                  disposition=issue1["disposition"], statement_sha256=issue1["statement_sha256"], sha256=sha(issue1_bytes),
                  judged="the producer's original statement", reason="R-F1 (cell medians not labelled), R-F2 (fast 2^32 T = 1024 unranked omitted)")],
    notes=["Any change to the statement needs a new judgement.",
           "RESULTS.md's headline and table are validated in validator-report.json (unchanged); this report judges only the statement.",
           "Non-blocking open items in RESULTS/the PR: R-F7 partly, R-F9 not applied (see checks.amendments_check)."])
open(OUT, "w").write(json.dumps(out, indent=1) + "\n")
print("status", out["status"], "statement sha256", stmt_sha, "headline", len(H), "mismatches", len(mism), "changed", changed)
for c in clauses:
    if c[1] is not True:
        print("  clause fails:", c[0], "|", c[2])
