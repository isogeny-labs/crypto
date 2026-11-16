#!/usr/bin/env python3
"""Render the checked all-trace receipts into a self-contained research report."""
from pathlib import Path
import json

here = Path(__file__).parent
a = json.loads((here / 'ANALYSIS.json').read_text())
v = json.loads((here / 'VERIFIED_UNMODELED.json').read_text())
abscheck = json.loads((here / 'VERIFIED_ABSOLUTE.json').read_text())
e = json.loads((here / 'ELLIPTIC_SOURCES.json').read_text())
assert a['classified'] == 1971 and a['genus5_carriers'] == 850
assert v['checked'] == e['sources'].__len__() == 19
assert abscheck['checked'] == 850
by_trace = {x['trace_t']: x for x in a['class_summary']}
replays = {x['trace_t']: x for x in v['records']}
sources = {x['trace_t']: x for x in e['sources']}
traces = a['eligible_unmodeled_traces']
assert set(traces) == set(replays) == set(sources)

lines = [
    '# All-trace census of five-node plane-sextic genus-five carriers over F3',
    '',
    '## Claim boundary',
    '',
    'This is an exhaustive census **inside one specified plane-sextic stratum and four-count filter**, '
    'not a classification of all genus-five curves or all curves vulnerable to GHS. '
    'It gives 19 elliptic-source-eligible Weil classes with explicit genus-five Jacobian '
    'representatives absent from the LMFDB model snapshot of 2026-11-16. '
    'An explicit evaluable subgroup transfer and complete GHS-versus-rho measurement remain open. '
    'Accordingly, these are candidate new class realizations, not established new vulnerabilities.',
    '',
    '## Exact finite search and bounds',
    '',
    'The two PGL3(F3) node-position orbits are the complete degree-five-orbit configuration '
    'with no four collinear points, as certified in the preceding quintic-node search. '
    'For each position, the sextic linear system has dimension 13 over F3, hence exactly '
    '(3^13-1)/(3-1)=797161 projective coefficient vectors. '
    'The earlier full enumeration retained 947 and 1024 equations satisfying '
    'N1=4, N2=10, N3=28, N4=82. This frozen classifier examined all 1971 rows: '
    '850 genus-five carriers, 1008 of different genus, 61 with a nonordinary intended node, '
    'and 52 outside the genus-five trace bound. This count is of searched equations, '
    'not isomorphism classes.',
    '',
    'For a genus-five curve C/F3 to have the Weil polynomial of Res_{F243/F3}(E), '
    'its first four counts must be 3^r+1 and its fifth count must satisfy '
    '`N5(C)=244-5t`, where `t=244-#E(F243)`. Hasse gives the exact integer bound '
    '`|t|<=31`. Waterhouse’s elliptic trace criterion specialized to F243 leaves '
    '`3∤t` or `t in {0,±27}`. The classifier found 50 trace classes; 35 pass '
    'this source-existence filter. Of these, 19 have zero stored curve models in the '
    'dated LMFDB query. The 19 classes contain 236 of the 850 searched equations. '
    'The other 378 source-eligible equations lie in 16 classes with recorded models.',
    '',
    'The direct replay independently evaluated all monomials over F3, F9, F27, F81, '
    'and F243 for one representative of each of the 19 classes, checked five ordinary '
    'nodes and the split correction at degree five, and recomputed the normalized '
    'counts. Sage/Singular checked geometric genus five. Factoring all 850 equations '
    'over F729 rules out the six-conjugate-line case omitted by the earlier F9/F27 '
    'factor checks. `TraceLedger.lean` checked the finite trace, order, and fifth-count '
    'arithmetic with Lean 4.35.0; it does not formalize genus, point counting, '
    'Waterhouse, or Tate.',
    '',
    '## Candidate classes and exact elliptic sources',
    '',
    'The field is `F243=F3[u]/(u^5+2u+1)`. In the table, ordinary source equations '
    'are `y^2=x^3+a2*x^2+a6`; for trace -27 the source is '
    '`y^2=x^3+2*x+u^4`. The full sextic equations appear below.',
    '',
    '| trace t | equations in stratum | #E(F243)=#J(F3) | LMFDB class | a2 | a6 | sextic case:index |',
    '|---:|---:|---:|---|---|---|---|',
]
for t in traces:
    s, src, rec = by_trace[t], sources[t], replays[t]
    assert rec['normalized_counts'] == [4, 10, 28, 82, 244-5*t]
    a2, a4, a6 = src['ainvs'][1], src['ainvs'][3], src['ainvs'][4]
    if t != -27:
        assert a4 == '0'
    lines.append(f"| {t} | {s['count']} | {src['order']} | {s['LMFDB_label']} | `{a2}` | `{a6}` | {rec['case']}:{rec['index']} |")

lines += [
    '',
    '## Exact plane sextics',
    '',
    'For each trace, `C` is the normalization of the displayed homogeneous plane sextic '
    'over F3. The row `case:index` identifies its full machine receipt in `ANALYSIS.json` '
    'and `VERIFIED_UNMODELED.json`. Coefficients `-1` mean `2` in F3.',
    '',
]
for t in traces:
    r = replays[t]
    lines += [f"- **t={t}, case {r['case']} index {r['index']}**: `{r['sextic']} = 0`; "
              f"normalized counts `{r['normalized_counts']}`."]

lines += [
    '',
    '## Prior work and novelty',
    '',
    'Kudo–Harashita (2023) introduced the five-node plane-sextic parameterization, '
    'including the two degree-five node configurations, but their reported search '
    'targeted at least 32 F9 points; these sparse classes all have N2=10. '
    'Their 2022 hyperelliptic/trigonal study also addressed high F9 point counts. '
    'Thus the geometric construction and broad family are known, while the exact '
    'low-point equations and the 19 catalog-absent Jacobian class realizations '
    'are plausible new results. The LMFDB `model_count=0` flag means that its '
    'snapshot has no stored curve equation; it does not establish publication priority '
    'or prove that the class has no Jacobian. A per-equation publication search '
    'and isomorphism comparison remain open. `ANALYSIS.json` includes a '
    '`candidate_prior_work_checks` row for every one of the 850 carriers and '
    'marks exact-equation priority unresolved.',
    '',
    'The class equality `L_C(T)=1-t*T^5+243*T^10=L_{Res E}(T)` implies an '
    'isogeny by Tate’s theorem, but it does not supply an evaluable map on '
    'the chosen rational subgroup. No vulnerability or speedup is claimed. '
    'The strongest prime-order candidate here is t=-19: '
    '`#E(F243)=#J(C)(F3)=263`, with an exact source point and 7P in '
    '`TRACE_MINUS19.json`. Its rho reference is on a group of order 263; '
    'the entire transfer, decomposition, linear algebra, descent, and verification '
    'cost must be measured before any attack comparison.',
    '',
    '## Primary sources',
    '',
    '- Kudo and Harashita, *Representation of non-special curves of genus 5 as plane sextic curves* (2023): https://arxiv.org/pdf/2102.07270',
    '- Kudo and Harashita, *Genus-five hyperelliptic or trigonal curves with many rational points* (2022): https://arxiv.org/pdf/2204.06805',
    '- Waterhouse criterion as restated in Agathocleous, Joux and Taufer, Theorem 2.1 (2026): https://pure.mpg.de/pubman/item/item_3596867_6/component/file_3655796/Agathocleous-Joux-Taufer_Elliptic%2Bcurves%2Bover%2BHasse%2Bpairs_2026.pdf',
    '- LMFDB finite-abelian-variety data queried 2026-11-16: exact SQL and rows in `LMFDB_SNAPSHOT.json`.',
    '',
    '## Reproduction',
    '',
    'The frozen full run is `sage -python classify.py FULL.jsonl`. The derived checks are '
    '`python3 analyze.py > ANALYSIS.json`, '
    '`sage -python verify_unmodeled.py > VERIFIED_UNMODELED.json`, '
    '`sage -python verify_absolute.py > VERIFIED_ABSOLUTE.json`, '
    '`sage -python build_elliptic_sources.py > ELLIPTIC_SOURCES.json`, and '
    '`lean TraceLedger.lean`. Hashes linking scripts and inputs appear in the receipts.',
]
(here / 'RESULT.md').write_text('\n'.join(lines) + '\n')
