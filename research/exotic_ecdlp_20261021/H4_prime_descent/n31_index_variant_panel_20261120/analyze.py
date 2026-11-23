#!/usr/bin/env python3
"""Reproducible exploratory paired bootstrap for ten audited local target pairs."""
import hashlib, json, random, statistics
from pathlib import Path
HERE=Path(__file__).resolve().parent
V=json.loads((HERE/'validation.json').read_text())
O=json.loads((HERE/'result.json').read_text())
assert V['status']=='passed' and O['status']=='complete'
assert V['result_sha256']==hashlib.sha256((HERE/'result.json').read_bytes()).hexdigest()
assert V['calibration_passes']==10 and V['verified_triples']==10
ratios=V['point_over_packed_ratios']
assert all(abs(r - x['arms']['point_ic']['wall_ns']/x['arms']['packed_ic']['wall_ns'])<1e-12
           for r,x in zip(ratios,O['triples']))
rng=random.Random(31120)
medians=sorted(statistics.median(rng.choices(ratios,k=len(ratios))) for _ in range(100000))
result={'schema':'h4-n31-index-variant-paired-analysis-v1','status':'complete',
        'method':'post-hoc ordinary nonparametric paired-target bootstrap of median point/packed online ratio',
        'seed':31120,'resamples':100000,'n':len(ratios),
        'median_point_over_packed':statistics.median(ratios),
        'bootstrap_median_ratio_95pct':[medians[2500],medians[97499]],
        'min_max_paired_ratio':[min(ratios),max(ratios)],
        'packed_faster_count':sum(r>1 for r in ratios),
        'scope':'exploratory target resampling on one ordinary host; excludes host and run-order uncertainty'}
(HERE/'analysis.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
print(json.dumps(result,indent=2,sort_keys=True))
