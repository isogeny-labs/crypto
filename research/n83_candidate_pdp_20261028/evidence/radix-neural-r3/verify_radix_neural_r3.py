#!/usr/bin/env python3
"""Independent consistency checks for the frozen N83 radix neural run."""
import hashlib
import json
import math
from pathlib import Path

HERE = Path(__file__).resolve().parent
INPUT = Path('/tmp/n83-radix-neural-input')
EXPECTED_RAW_MANIFEST = '5546a85f4a7e4467289bc3d167845f9054dbbfb546d75949f0a9fceec6e70e78'
EXPECTED_PROTOCOL = '66c2eb12cefef0dda537736cd5bc2ae0710003341c27a85108d08863ae4027c4'

def sha(data):
    return hashlib.sha256(data).hexdigest()

def features(parts):
    return [len(parts), sum((1 << p) - 1 for p in parts), max(parts), min(parts),
            sum(p == 1 for p in parts), sum(p >= 5 for p in parts)]

def model_prediction(model, parts):
    p = model['parameters']; h = 4; x = features(parts)
    w1 = [p[i*h:(i+1)*h] for i in range(6)]
    b1 = p[24:28]; w2 = p[28:32]; b2 = p[32]
    y = b2
    for j in range(h):
        z = b1[j] + sum(((x[i]-model['feature_mean'][i])/model['feature_scale'][i])*w1[i][j] for i in range(6))
        y += w2[j] * math.tanh(z)
    return y * model['target_scale'] + model['target_mean']

raw_manifest = (INPUT/'manifest.json').read_bytes()
assert sha(raw_manifest) == EXPECTED_RAW_MANIFEST
items = json.loads(raw_manifest)
case_items = [item for item in items if item['path'].startswith('cases/')]
assert len(items) == 3014 and len(case_items) == 3010
for item in case_items:
    data = (INPUT/item['path']).read_bytes()
    assert len(data) == item['bytes'] and sha(data) == item['sha256'], item['path']
cases = [json.loads((INPUT/'cases'/f'{i:04}.json').read_bytes()) for i in range(3010)]
preds = json.loads((HERE/'predictions.json').read_text())
model = json.loads((HERE/'model.json').read_text())
cv = json.loads((HERE/'cv.json').read_text())
metrics = json.loads((HERE/'metrics.json').read_text())
assert len(preds) == len(cases) == 3010
max_abs_error = 0.0
flagged_exact = flagged_censored = contradictions = violations = 0
fold_counts = [0]*5
for i, (case, row, pred) in enumerate(zip(cases, cases, preds)):
    assert pred['case'] == i and pred['parts'] == row['parts']
    exact = row.get('first_vanishing_degree')
    censored = exact is None
    assert pred['status'] == ('lower_bound_only' if censored else 'exact')
    assert pred['exact_degree'] == exact
    assert pred['lower_bound'] == row['lower_bound']
    p = model_prediction(model, row['parts'])
    max_abs_error = max(max_abs_error, abs(p-pred['prediction']))
    below = p < 42.0
    flagged_exact += int(below and not censored)
    flagged_censored += int(below and censored)
    contradictions += int(below and censored and row['lower_bound'] >= 43)
    violations += int(censored and p + 1e-9 < row['lower_bound'])
    tag = b'n83-radix-neural-fold-v1' + json.dumps(row['parts'], separators=(',', ':')).encode()
    fold_counts[int.from_bytes(hashlib.sha256(tag).digest()[:8], 'big') % 5] += int(not censored)
assert max_abs_error < 1e-10
held = cv['predictions']
mlp_mae = sum(abs(r['mlp_prediction']-r['exact_degree']) for r in held)/len(held)
constant_mae = sum(abs(r['censored_aware_constant']-r['exact_degree']) for r in held)/len(held)
assert len(held) == 35 and abs(mlp_mae-metrics['cv']['mlp_mae']) < 1e-10
assert abs(constant_mae-metrics['cv']['constant_mae']) < 1e-8
assert (flagged_exact, flagged_censored, contradictions, violations) == (30,188,188,2834)
run_manifest = json.loads((HERE/'run-manifest.json').read_text())
assert run_manifest['protocol_sha256'] == EXPECTED_PROTOCOL
receipt = {
  'schema':'n83.radix-neural-independent-verification/v1', 'raw_manifest_sha256':sha(raw_manifest),
  'protocol_sha256_recorded_by_run':EXPECTED_PROTOCOL,
  'protocol_file_sha256_at_verification':sha((HERE/'../../protocol-radix-neural-v5.json').read_bytes()),
  'case_hashes_verified_against_manifest':len(case_items),
  'prediction_reconstruction_max_abs_error':max_abs_error, 'fold_exact_counts':fold_counts,
  'held_out_exact_rows':len(held), 'recomputed_mlp_exact_mae':mlp_mae,
  'recomputed_constant_exact_mae':constant_mae, 'full_predictions_below_42_exact':flagged_exact,
  'full_predictions_below_42_censored':flagged_censored, 'censored_flags_refuted_by_bound':contradictions,
  'censored_lower_bound_violations':violations, 'result':'PASS'
}
(HERE/'independent-verification.json').write_text(json.dumps(receipt, indent=2)+'\n')
print(json.dumps(receipt, indent=2))
