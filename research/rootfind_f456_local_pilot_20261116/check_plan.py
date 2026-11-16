#!/usr/bin/env python3
"""Check the frozen local plan against the first two L2 panel workloads."""
import hashlib
import json
import subprocess
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
EXE = Path('/Volumes/SSD990-2/crypto/wt/codex-rootfind-k13-local-f456-screen-20261116/target/release/ecbench')
SPEC = HERE / 'SPEC-K13-L0.json'
PANEL = ROOT / 'research/rootfind_f456_batch_combined_20261115/SPEC-K13.json'
PLAN = HERE / 'PLAN.json'

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

local = json.loads(PLAN.read_text())
original = json.loads(subprocess.check_output([str(EXE), 'plan', '--spec', str(PANEL), '--doc'], cwd=ROOT))
assert len(local['arms']) == 8
assert len(local['workloads']) == 2
assert len(local['executions']) == 48
assert local['workloads'] == original['workloads'][:2]
original_arms = {a['name']: a for a in original['arms']}
assert all(a == original_arms[a['name']] for a in local['arms'])
assert all(w['curve']['icv1'] == 'ICV1:f2m-13-752c6463:181:8012:0x1:-7:unk:r:515ee5696294' for w in local['workloads'])
receipt = {
    'schema': 'rootfind-local-f456-plan-check/v1',
    'status': 'preflight_plan_only_no_experiment',
    'protocol_version_1_commit': 'ef1648b167',
    'amendment': 'AMENDMENT-1.md',
    'spec_sha256': sha(SPEC),
    'plan_sha256': sha(PLAN),
    'preflight_binary_sha256': sha(EXE),
    'binary_source_binding': 'pending_clean_release_build',
    'registered_curve': local['workloads'][0]['curve']['icv1'],
    'workload_ids': [w['workload_id'] for w in local['workloads']],
    'target_points': [w['target'] for w in local['workloads']],
    'arms': [a['name'] for a in local['arms']],
    'planned_executions': 48,
    'planned_warmups': 16,
    'planned_measured': 32,
    'matches_first_two_frozen_panel_workloads': True,
    'matches_frozen_panel_arm_parameters': True,
}
(HERE / 'PLAN_CHECK.json').write_text(json.dumps(receipt, indent=2) + '\n')
print(json.dumps(receipt, sort_keys=True))
