#!/usr/bin/env python3
"""Render lead-check.svg from the recorded metrics in graph-data.json."""
import json
from pathlib import Path
p=Path(__file__).parent
d=json.loads((p/'graph-data.json').read_text())
svg=f'''<svg xmlns="http://www.w3.org/2000/svg" width="900" height="390" viewBox="0 0 900 390" role="img" aria-labelledby="title desc">
<title id="title">N83 radix neural candidate check</title>
<desc id="desc">All {d['flagged_censored_below_42']} censored rows predicted below degree 42 are refuted by certified lower bounds; {d['bound_violations']} of {d['censored_rows']} lower bounds are violated.</desc>
<rect width="900" height="390" fill="#fff"/><text x="42" y="48" font-family="sans-serif" font-size="24" font-weight="700" fill="#182230">N83 radix neural probe: no validated lead</text>
<text x="42" y="78" font-family="sans-serif" font-size="15" fill="#526071">Fixed prime m=2 census · {d['rows']:,} rows · {d['exact_rows']} exact, {d['censored_rows']:,} censored lower bounds</text>
<text x="42" y="132" font-family="sans-serif" font-size="16" font-weight="700" fill="#182230">Censored predictions below degree 42</text><rect x="42" y="151" width="760" height="38" rx="5" fill="#e8edf2"/><rect x="42" y="151" width="760" height="38" rx="5" fill="#c2413b"/><text x="58" y="176" font-family="sans-serif" font-size="16" font-weight="700" fill="#fff">{d['flagged_censored_below_42']} flagged · {d['contradicted_flags']} refuted by certified bounds</text>
<text x="42" y="229" font-family="sans-serif" font-size="16" font-weight="700" fill="#182230">Censored lower-bound constraints violated by full-fit model</text><rect x="42" y="248" width="760" height="38" rx="5" fill="#e8edf2"/><rect x="42" y="248" width="{760*d['bound_violation_rate']:.1f}" height="38" rx="5" fill="#dc8b35"/><text x="58" y="273" font-family="sans-serif" font-size="16" font-weight="700" fill="#182230">{d['bound_violations']:,} / {d['censored_rows']:,} ({100*d['bound_violation_rate']:.2f}%)</text>
<text x="42" y="329" font-family="sans-serif" font-size="14" fill="#526071">Exact-row 5-fold MAE: MLP {d['mlp_cv_exact_mae']:.2f} · censored-aware constant {d['constant_cv_exact_mae']:,.2f}</text><text x="42" y="354" font-family="sans-serif" font-size="13" fill="#526071">The constant is dominated by an extreme certified bound; MAE is descriptive. No new factor-base points were generated.</text>
</svg>'''
(p/'lead-check.svg').write_text(svg)
