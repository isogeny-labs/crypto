from pathlib import Path
import typst,fitz
D=Path(__file__).resolve().parent
nodes=['Canonical field inputs','Packed convolutions and folds','Packed sum and inverse scaling','Decode n final coefficients']
svg='<svg xmlns="http://www.w3.org/2000/svg" width="760" height="240"><rect width="760" height="240" fill="white"/>'
for i,label in enumerate(nodes):
 y=10+58*i
 svg+=f'<rect x="90" y="{y}" width="580" height="40" rx="5" fill="#e7f0f4" stroke="#286e89"/><text x="380" y="{y+26}" text-anchor="middle" font-family="sans-serif" font-size="17">{label}</text>'
 if i<3:svg+=f'<path d="M380 {y+40} v18 m-4 -5 l4 5 l4 -5" fill="none" stroke="#286e89"/>'
svg+='</svg>'; (D/'pipeline.svg').write_text(svg)
t='''#set page(paper:"a4",margin:18mm)
#set text(size:11pt)
= Deferred reduction: construction and validation design
2026-11-01 · Derived algorithm; syntax-checked source; zero numerical trials

The candidate keeps positive polynomial coefficients packed through extraction,
wrapping, column accumulation and final inverse scaling. Only the final n
coefficients are decoded and reduced modulo the prime r. It accepts arbitrary
positive dimension n, generator width s and distinct canonical wraps a,b.

#image("pipeline.svg",width:100%)
*Carry argument.* With R=r−1, choose radix β=256^d strictly greater than
B=s n² R⁴ (1+a)(1+b). First-product coefficients are bounded by n R²;
extracted coefficients by n R²(1+u); second-product coefficients by
n² R³(1+u). The remaining fold, sum and canonical inverse scaling are bounded
by B. All intermediate coefficients are nonnegative, so this bound prevents
cross-digit carry. Buffer-size assertions alone do not prove that property.

*Derived accounting.* One field subtraction, one inverse and n final
coefficient reductions. Native multiplication count: 4s+8, including seven
bound-setup products. Input encodings: 3s+1; packing bytes: (3s+1)nd.
Intermediate serialization: snd bytes; final serialization: nd bytes.
Shifts, masks, additions and subtractions: 3s+2, 2s, 3s+2 and 3 respectively.
Bit-length volumes are recorded separately from arithmetic counts and bytes.
The larger radix and additional native arithmetic require validation; these
formulas are not runtime or memory measurements.

*Implementation delivered.* multiply.py has optional per-column P,T,Q,Y,
accumulator and final-result trace. reference.py independently forms integer
coefficient lists by naive convolution and evaluates them at β for comparison.
Both modules pass syntax checks; neither has been numerically executed.

*Full validation design.* Retain all 369 archived block contexts and 16164
products per arm, plus 480 carry contexts per arm covering four primes,
five dimension choices, three widths, four wrap pairs and two digit patterns.
Both directions and candidate/AA give 1698 context rows and 34248 products.
The baseline is archived dense-verified output/count evidence. Freeze the
complete seed schedule, source/input locks, checker, analyzer and cap before
trials. Include output/count/sign/trace/radix corruption controls. Undersized
radix rejection is a proof-guard test, distinct from an actual carry example.

*Continuation.* Finish independent bit-volume reconstruction and full-cohort
checker, then freeze and commit the protocol. Integrating this operator into
Schur construction requires a separate invocation. The original Schur run
is capped and immutable (terminal evidence commit d4449d1).

*Sources.* Local DERIVATION.md, multiply.py, reference.py, VALIDATION_PLAN.md;
archived companion-blocks-validation checks and original companion operator.
Diagram represents the derived flow, not measured performance.
'''
(D/'report.typ').write_text(t)
typst.compile(str(D/'report.typ'),output=str(D/'report.pdf'))
doc=fitz.open(D/'report.pdf')
for i,p in enumerate(doc):p.get_pixmap(matrix=fitz.Matrix(1.3,1.3)).save(str(D/f'page-{i+1}.png'))
print('pages',len(doc))
