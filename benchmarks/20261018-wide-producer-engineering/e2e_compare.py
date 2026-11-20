#!/usr/bin/env python3
"""End-to-end equality check: same instance, control vs every switch.

Runs the full producer (rank stage until all K columns are fixed, then the
targets) at a small wide n where relations are plentiful, once per arm, and
compares everything that is not a timing: relation counts, probes, the
published relations and the recovered scalars.  Exit 1 on any difference.
"""
import json, os, subprocess, sys, tempfile

binary, spec = sys.argv[1], sys.argv[2]
scalars = [12345, 6789, 31337, 2718, 31415, 99991, 4242, 777]
ARMS = [
    ("control", {"KIC_LAZY_CANON": "0", "KIC_PREFETCH": "0"}),
    ("defaults", {}),
    ("lazy_only", {"KIC_LAZY_CANON": "1", "KIC_PREFETCH": "0"}),
    ("prefetch_only", {"KIC_LAZY_CANON": "0", "KIC_PREFETCH": "1"}),
    ("blocked_only", {"KIC_LAZY_CANON": "0", "KIC_PREFETCH": "0", "KIC_BLOCKED_BLOOM": "1"}),
    ("all_switches", {"KIC_LAZY_CANON": "1", "KIC_PREFETCH": "1", "KIC_BLOCKED_BLOOM": "1",
                      "KIC_INDEX_THREADS": "4", "KIC_INDEX_BATCH_INV": "1", "KIC_VERIFY_INDEX": "1"}),
]
SUMMARY_KEYS = ["rank_attempts", "rank_failures", "rank_relations", "rank_rows_without_gain",
                "rank_probes_mean", "targets_solved", "factor_base_points", "base_hash"]
TARGET_KEYS = ["fixture_index", "point_indices", "probes", "x_codes", "pinned_intermediates",
               "recovered_scalar", "recovered_matches_published", "group_verified", "target"]
results = {}
with tempfile.TemporaryDirectory() as wd:
    targets = os.path.join(wd, "targets.txt")
    open(targets, "w").write("\n".join(map(str, scalars)) + "\n")
    for name, env in ARMS:
        out = os.path.join(wd, name + ".jsonl")
        e = dict(os.environ); e.update(env)
        p = subprocess.run([binary, spec, targets, "1", out], env=e, capture_output=True, text=True)
        if p.returncode != 0:
            sys.exit(f"{name} failed: {p.stderr[-800:]}")
        summary = json.loads(p.stdout.splitlines()[0])
        rows = [json.loads(l) for l in open(out)]
        results[name] = ({k: summary.get(k) for k in SUMMARY_KEYS},
                         [{k: r.get(k) for k in TARGET_KEYS} for r in rows],
                         {k: summary.get(k) for k in ("lazy_canonical_enabled", "probe_prefetch_enabled", "root_bloom_blocked")},
                         [l for l in p.stderr.splitlines() if "VERIFY_INDEX" in l])
ref = results["control"]
ok = True
for name, (s, t, flags, verify) in results.items():
    same = (s == ref[0]) and (t == ref[1])
    ok &= same
    print(f"{name:14s} identical_to_control={same} flags={flags} rank_relations={s['rank_relations']} "
          f"rank_probes_mean={s['rank_probes_mean']} targets_solved={s['targets_solved']} "
          f"probes_per_target={[r['probes'] for r in t]} {verify}")
sys.exit(0 if ok else 1)
