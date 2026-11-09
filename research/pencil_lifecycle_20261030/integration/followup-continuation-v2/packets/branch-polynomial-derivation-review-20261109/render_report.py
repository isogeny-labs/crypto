from pathlib import Path
import fitz,json
P=Path(__file__).resolve().parent
r=json.loads((P/'review.json').read_text())
steps=['Exact source: P = N^2 - b^2 F','F(a)=P(a)=0 implies N(a)=0',"P'(a) = -b^2 F'(a) is nonzero",'P and each factor are nonzero','Simple root: factor multiplicities sum to 1']
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="900" height="410"><rect width="900" height="410" fill="#f6f8fc"/>']
for i,t in enumerate(steps):
 y=20+i*76;svg.append(f'<rect x="25" y="{y}" width="850" height="52" rx="7" fill="white" stroke="#446080"/><text x="45" y="{y+33}" font-size="22" font-family="sans-serif">{t}</text>')
svg.append('</svg>');(P/'derivation.svg').write_text('\n'.join(svg))
d=fitz.open();p=d.new_page(width=595,height=842);p.insert_text((35,38),'Independent branch polynomial review',fontsize=17)
s=fitz.open(stream=(P/'derivation.svg').read_bytes(),filetype='svg');q=fitz.open('pdf',s.convert_to_pdf());p.show_pdf_page(fitz.Rect(30,60,565,305),q,0)
t="Verdict: faithful_scoped (mathematical/source review).\n\nThe exact sign and scalar agree with source_norm_identity. Product nonzero is derived before multiplicity additivity. No division by2, b or the residual leading coefficient is used. Any field characteristic is allowed; only simple-root derivative nonvanishing is required.\n\nThe conclusion matches the three factor-nonzero conditions and multiplicity sum1 in source_support_base_multiplicity_sum. Source divisor congruences and determinant nonzero supply the identity. No separate geometric/DVR claim is inferred.\n\nRational fixture: coefficientData=(-4,0,0,-16,0), N=0 and U=-16X. Product=-16(X^5-X), derivative at0=16; multiplicities0,0,1. The residual leading coefficient is zero."
assert p.insert_textbox(fitz.Rect(35,330,560,785),t,fontsize=11,lineheight=1.3)>=0
p=d.new_page(width=595,height=842);p.insert_text((35,38),'Controls, source bindings and review scope',fontsize=17)
t="Premise-removal controls (all over Q at0):\n\n1. b=0: F=X, N=X, factors X,X,1. Product X^2 has order2.\n\n2. Nonsimple F: F=X^2, b=1, N=0, factors X,X,-1. Product -X^2 has order2.\n\n3. No support root: F=X, b=1, N=1, factors1-X,1,1. Product1-X has order0.\n\nEach retains the exact generic identity. These tuples are not asserted to be realized by source coefficientData.\n\nCharacteristic2 witness: F=X, N=0, b=1, factors X,1,1. Since -X=X, the identity holds and product derivative is1.\n\nAdditional branch-root control: overQ, N=X+1, F=2X+1, b=1, productX^2. F(0)=1 and product order2.\n\nSeven producer manifest entries and five source bindings match. Reviewer additionally binds coefficientData source and RingDivision.lean product-additivity API.\n\nReviewer authored ancestor replay plumbing/proof repair, not this direct derivation. Prospective review plan retained. No compiler, experiment or theorem replay by reviewer. The direct derivative route is source-level; current branch replay remains separate.\n\nMachine-readable files: review.json, source-bindings.json and manifest.json. Detailed algebra: REPORT.md."
assert p.insert_textbox(fitz.Rect(35,65,560,790),t,fontsize=11,lineheight=1.25)>=0
d.save(P/'report.pdf',garbage=4,deflate=True)
for i,page in enumerate(d):page.get_pixmap().save(f'/var/tmp/branch-polynomial-review-{i+1}.png')
