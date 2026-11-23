from pathlib import Path
import json,hashlib
D=Path(__file__).parent
report='''# Global coprimality supplies the local nonsingularity premise

Five Lean lemmas compile over an arbitrary field. IsCoprime f f.derivative means there exist polynomial A,B with A*f+B*f.derivative=1. Evaluation excludes a common zero. For the retained source quintic f=X^5+a3*X^3+a2*X^2+a1*X+a0, its evaluation is the existing value function and its Polynomial.derivative evaluation is exactly 5*x^4+3*a3*x^2+2*a2*x+a1. No characteristic restriction is required for these identities.

The global condition therefore implies LocalSmooth.LocalNonsingular at every x. With the original two support divisibilities, nonzero slope difference and noninverse ordinates, global_extracted_ordinate_nonzero proves the extracted ordinate nonzero. The existing reviewed weighted extraction/doubling theorem can consume the resulting local premise; its characteristic-not-two and four nonzero input weights remain required. This packet does not restate or remove those premises.

Five fixed F5 controls compile: X^5-X has derivative -1, an explicit Bezout certificate (A=0,B=-1), and the local condition for every x; X^5 has derivative zero and fails global coprimality. Thus the proof does not divide by five. The global premise is a certificate contract, not a new algorithm that checks curve separability. No Squarefree-to-Separable implication over imperfect fields is asserted.

Final native Lean exits: main attempt4=0, controls3=0, readback1=0. All five theorem readbacks list only propext, Classical.choice and Quot.sound. Earlier evidence is retained: missing-module import; remote-backed cache invocation killed at the bounded timeout; wrong ordinate namespace; two numeral simplification failures. Final compilation uses a private extension of the existing pinned dependency union: six cached modules, 84 companion files, 5,163,724 bytes. No download or dependency build occurred. Linter warnings are retained.

Source scope is mathematical correspondence to the retained quintic and reviewed LocalSmooth theorem, not compiler, native branch, counter or class-sum validation. No scientific trials or timing measurements were run. Independent source-fidelity review is pending.

Replay: copy GlobalSmooth.lean, Controls.lean, Readback.lean and check.py to a new private directory; run check.py GlobalSmooth fresh-main, then Controls fresh-controls and Readback fresh-readback. Existing pinned dependency overlay remains a bound dependency. Each check uses 120 seconds plus five seconds kill grace, one Lean thread and 4096 MiB maximum.

Primary library reference: https://github.com/leanprover-community/mathlib4/blob/d13f23b723b8a846827a245b89c10fc7d3f11612/Mathlib/Algebra/Polynomial/Derivative.lean and RingTheory/Coprime/Basic.lean at the same commit. Their exact retained source/object hashes are in cache-custody.json. Local dependency sources and retained source file are bound in sources.json.
'''
(D/'REPORT.md').write_text(report)
status={'schema':'mathematical_component/v1','status':'compiled_pending_independent_source_review','field_scope':'arbitrary field','global_condition':'IsCoprime f f.derivative','claims':['coprime_excludes_common_root','curve_eval','curve_derivative_eval','global_implies_local','global_extracted_ordinate_nonzero'],'fixed_controls':5,'final_exits':{'attempt4':0,'controls3':0,'readback1':0},'failed_attempts':{'attempt1':'missing import module','attempt2':'bounded process terminated -9','attempt3':'wrong ordinate namespace','controls1':'F5 numeral simplification','controls2':'F5 numeral simplification'},'squarefree_implication_claimed':False,'whole_cost_ratio':None,'native_trials':0,'review':'pending','five_characteristic_supported':True}
(D/'status.json').write_text(json.dumps(status,indent=2))
sources=json.loads(Path('/var/tmp/pencil-sameu-local-smooth-20261031-v1/sources.json').read_text())
for s in ['/var/tmp/pencil-sameu-local-smooth-20261031-v1/LocalSmooth.lean','/var/tmp/pencil-sameu-local-smooth-20261031-v1/manifest.json','/var/tmp/pencil-sameu-nonsingularity-independent-review-20261031-v1/manifest.json','/home/ec2-user/crypto-pr2304-persist/research/projective_law_followup_20261022/formal/lake-manifest.json']:
 sources[s]=hashlib.sha256(Path(s).read_bytes()).hexdigest()
(D/'sources.json').write_text(json.dumps(sources,indent=2))
svg='''<svg xmlns="http://www.w3.org/2000/svg" width="920" height="370" viewBox="0 0 920 370"><rect width="920" height="370" fill="#f6f8fb"/><g font-family="sans-serif" fill="#17263b"><text x="30" y="38" font-size="23">Global certificate → local condition → extracted ordinate</text><rect x="30" y="70" width="260" height="100" fill="#dce9f5"/><text x="45" y="103" font-size="17">A f + B f′ = 1</text><text x="45" y="137" font-size="15">IsCoprime f f.derivative</text><text x="307" y="125" font-size="26">→</text><rect x="350" y="70" width="240" height="100" fill="#dce9f5"/><text x="365" y="103" font-size="17">No common zero f(x), f′(x)</text><text x="365" y="137" font-size="15">Explicit quintic identity</text><text x="609" y="125" font-size="26">→</text><rect x="650" y="70" width="240" height="100" fill="#dce9f5"/><text x="665" y="103" font-size="17">Extracted y ≠ 0</text><text x="665" y="137" font-size="15">Original congruences retained</text><text x="30" y="215" font-size="18">Characteristic 5 controls</text><text x="30" y="248" font-size="16">f = X⁵ − X: f′ = −1; certificate (0, −1); local condition holds.</text><text x="30" y="280" font-size="16">f = X⁵: f′ = 0; global condition fails.</text><text x="30" y="332" font-size="15">5 lemmas · 5 fixed controls · no squarefree/separable equivalence assumed</text></g></svg>'''
(D/'diagram.svg').write_text(svg)
paras=report.split('\n\n')
t='#set page(paper:"a4",margin:17mm)\n#set text(size:10pt)\n'
for p in paras:t+='#text('+json.dumps(p.removeprefix('# '),ensure_ascii=False)+')\n#parbreak()\n'
(D/'report.typ').write_text(t)
