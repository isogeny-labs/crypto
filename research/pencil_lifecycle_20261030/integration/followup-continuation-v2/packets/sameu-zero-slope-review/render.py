from pathlib import Path
import json,hashlib
D=Path(__file__).parent;P=Path('/var/tmp/pencil-sameu-zero-slope-20261031-v1')
receipts={}
for n in ['attempt1','attempt2','controls1','controls2','controls3','controls4','controls5','readback1']:receipts[n]=json.loads((P/n/'receipt.json').read_text())['exit_code']
(D/'producer-receipt-readback.json').write_text(json.dumps(receipts,indent=2))
text='''# Independent same-u zero-slope review

Result: validated at the stated field-expression scope. All52 artifact/source bindings match. Fresh isolated main proof, producer controls, axiom readback and reviewer controls exited0. The three theorem readbacks list only propext, Classical.choice and Quot.sound. Producer files were not changed.

The reviewed claim excludes same_u_mixed_zero_slope_delta from the original validTwo conditions, four nonzero weights, literal equal crossed q/r guards, and preceding inverse/equal ordinate guards both false. The polynomial f is arbitrary and no smoothness or characteristic restriction is assumed. This is an ordered guard exclusion, not an additional payload formula.

Source comparison: retained projective_law_gate.rs903–933 forms z²,z³,z⁴,z⁵ and crossed q/r/s/t in the same multiplication association as crossQ/crossR/crossS/crossT. The source tests inverse first, equal second, then subtracts crossed slopes. The imported sourceK is that subtraction. Cross scaling transports these literal expressions through nonzero common factors to normalized support/ordinate coordinates; both directions used in the contradiction are justified.

Original validity supplies two shared-support congruences. factor_identity then gives (v1-v2)(v1+v2)=C((s1-s2)(s1+s2))*u. Equal slopes make the right side zero; the polynomial integral-domain property forces equal or inverse ordinates. Coefficient extraction yields the two exact source alternatives. Thus assuming sourceK=0 contradicts a preceding guard being false. No output-validity, extracted-point or nonzero-ordinate assumption is added.

Independent fixed controls strengthen the producer unit-weight example. Over Q, a=(0,0,24,0,2,3) and b=(0,0,108,0,3,2), with f=X^5, satisfy original validity and both guard exclusions; the full theorem applies and direct sourceK equals -1296. A separate full-input F2 fixture uses a=(0,0,0,0,1,1), b=(0,0,1,0,1,1), f=X^5, and proves sourceK nonzero through the theorem. The producer retains equal/inverse branches and an invalid-input negative control.

The proof concerns field expressions; UInt64 equality reflection, compiler execution, counters, class sums and a complete degree2+2 dispatcher are separate. No scientific trial or source/cache mutation occurred. Reviewer previously reviewed imported same-u extraction and authored the subsequent local/global nonsingularity bridge; those latter bridges are not required by this theorem. Existing dependency checks are reused, not represented as fresh independent reconstruction of all dependencies.

One reviewer preparation failure is retained: the manifest reader initially assumed a flat dictionary instead of schema/files, so no compiler started in that failed preparation. After correcting the reader, all four fresh checks passed on their first native invocation. Producer main/control failures remain retained and match the recorded statuses. Replay uses existing pinned Lean/dependencies,120seconds+5seconds kill grace per command, j1 and4096MiB.

Producer manifest491d47dbf0ed05a6327265475088c7162f9ad4a8f8dd453d21b16a55d414e0c1. Exact source hashc3d8d2933ac4b55a9aa5cf2ebbaaf33c3cbeae2fecd7b43d6f4953be3314c0bf. Private replay/check.py and each invocation receipt provide the commands and dependency paths.
'''
(D/'REVIEW.md').write_text(text)
(D/'review.json').write_text(json.dumps({'schema':'research.formal-component-review/v1','verdict':'validated_scoped','producer_manifest':'491d47dbf0ed05a6327265475088c7162f9ad4a8f8dd453d21b16a55d414e0c1','source_sha256':'c3d8d2933ac4b55a9aa5cf2ebbaaf33c3cbeae2fecd7b43d6f4953be3314c0bf','bindings_checked':52,'all_bindings_match':True,'fresh_checks':{'main1':0,'controls1':0,'readback1':0,'reviewer1':0},'field':'arbitrary','source_guard':'same_u_mixed_zero_slope_delta','original_premises_preserved':True,'reviewer_controls':['nonunit Q full original-validity application and K=-1296','F2 full original-validity application'],'preparation_failures':1,'scientific_trials':0,'whole_cost_ratio':None},indent=2))
t='#set page(paper:"a4",margin:16mm)\n#set text(size:10pt)\n'
for p in text.split('\n\n'):t+='#text('+json.dumps(p.removeprefix('# '),ensure_ascii=False)+')\n#parbreak()\n'
(D/'report.typ').write_text(t)
(D/'diagram.svg').write_text('<svg xmlns="http://www.w3.org/2000/svg" width="920" height="300"><rect width="920" height="300" fill="#f4f7fb"/><g font-family="sans-serif" fill="#203248"><text x="25" y="45" font-size="23">Same-u zero-slope exception: independent field review</text><text x="25" y="100" font-size="18">Original valid inputs + crossed same support + sourceK = 0</text><text x="25" y="145" font-size="18">→ equal normalized slopes → equal or inverse ordinates</text><text x="25" y="190" font-size="18">→ contradicts preceding guard exclusions → sourceK ≠ 0</text><text x="25" y="250" font-size="17">52 bindings · 4 fresh checks pass · Q nonunit and F2 full-input controls</text></g></svg>')
