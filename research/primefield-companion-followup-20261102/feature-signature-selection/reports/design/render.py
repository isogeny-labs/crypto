"""Render the derived selection mechanism; no arithmetic producer imports."""
from pathlib import Path
from html import escape
import typst,fitz
D=Path(__file__).resolve().parent
labels=['q shifted characters: at most 2^q + q public signatures',
        'If 2^q + q < r: exclude X by an exact cardinality argument',
        'Optional sparse certificate: offer public points to feature-row elimination',
        'Keep independent rows; discard consistent dependencies with cost charged',
        'Inconsistent dependency: normalize, then independently check all moments']
svg='<svg xmlns="http://www.w3.org/2000/svg" width="800" height="280"><rect width="800" height="280" fill="white"/>'
for i,label in enumerate(labels):
 y=6+55*i
 svg+=f'<rect x="10" y="{y}" width="780" height="38" rx="4" fill="#edf3f6" stroke="#286e89"/><text x="400" y="{y+24}" text-anchor="middle" font-family="sans-serif" font-size="13">{escape(label)}</text>'
 if i<4:svg+=f'<path d="M400 {y+38} v17 m-4 -5 l4 5 l4 -5" fill="none" stroke="#286e89"/>'
(D/'pipeline.svg').write_text(svg+'</svg>')
s="""#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Character-signature exclusion and sparse selection
2026-11-03 · Derived theorem / source construction · Numerical trials: zero

*Structural bound.* Outside q exceptional character-zero points, the q shifted
quadratic characters have binary signs. Thus K = min(r, 2^q + q) bounds the
number of signatures. Any function of the signature agrees with X at most
once per signature fiber, hence at most K times. If K < r, representation of
X is excluded at every polynomial degree, independently of Gram weights.

#image("pipeline.svg",width:100%)

*Public selection.* A bounded streaming routine evaluates the original cubic
features at supplied points. It retains independent rows and their original-row
combinations, applying the same elimination to target values. A zero feature
residual with nonzero target produces a normalized sparse dual. A separate
checker recomputes all moments and the pairing from public support.

*Progress proof.* At each nonterminal state, some feature combination fits
the retained independent rows. At least r−K points disagree with that fit.
Each such point either raises feature rank or produces a certificate. Under
independent uniform point sampling and K < r, conditional progress probability
is at least delta=(r−K)/r. Expected offers are at most (m+1)/delta, where
m=1+q+C(q,2)+C(q,3). No independence of character signs is assumed.

#table(columns:(2fr,2fr),inset:5pt,
 [Derived quantity],[Bound / schedule],
 [Witness support],[At most m+1 public points],
 [Elimination at rank t],[t(m+2)+t(t−1)/2 products and subtractions],
 [Stored field entries],[tm+t+t(t+1)/2],
 [Deterministic sufficient stream],[K+1 distinct points, only if K < r],
 [Sampler / stopping],[Caller-supplied stream; explicit finite cap])

Feature evaluation, normalization, inversions, native work and independent
checking are additional and accounted in DERIVATION.md. The theorem precheck
needs no field operations. Exhausted samples mean inconclusive. No automatic
field enumeration, random sampler or signature collision table is introduced.

*Applicability.* The saved F479 width-15 failure has K=r, so the strict bound
is inconclusive there. Its earlier dual certificate remains necessary. The
retained recovery panels start outside this guaranteed regime; no saved-panel
speed improvement is claimed. Positive represented programs remain separate.

*Evidence / next.* Syntax and preserved source bytes checked statically;
no numerical, corruption, heap or performance trials. Next exploit cubic degree
to sharpen agreement or certify an interpolant when K=r. A sampled fit alone
cannot establish a representation valid on the full field.

*Sources.* NIST DLMF 27.9 (Legendre alphabet including zero); preserved
feature-span-dual report and coefficient construction. Cardinality and progress
arguments are derived here, without a novelty claim for general row elimination.
"""
(D/'report.typ').write_text(s)
typst.compile(str(D/'report.typ'),output=str(D/'report.pdf'))
doc=fitz.open(D/'report.pdf')
for i,p in enumerate(doc):p.get_pixmap(matrix=fitz.Matrix(1.3,1.3)).save(D/f'page-{i+1}.png')
print('pages',len(doc))
