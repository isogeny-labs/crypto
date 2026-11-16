from pathlib import Path
import html,textwrap,subprocess
p=Path(__file__).resolve().parent;s=['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="1200"><rect width="1000" height="1200" fill="white"/><g font-family="sans-serif" fill="#173047">']
def txt(x,y,t,z=18):s.append(f'<text x="{x}" y="{y}" font-size="{z}">{html.escape(t)}</text>')
def para(y,t):
 for l in textwrap.wrap(t,99):txt(45,y,l,17);y+=25
 return y+16
txt(45,50,'Weighted law: global class bridge reconciliation',28)
txt(45,85,'T-1535 | retained source/evidence inspection | no new proof execution',17)
y=para(126,'The weighted-Proj charts, properness, global smoothness and actual infinity DVR already exist. Later sources also close the two-chart isomorphism and specialize branch zero orders to the actual quintic. These components should be reused.')
for yy,title,body in [(y,'Existing actual geometry','Charts / properness / smoothness / infinity DVR'),(y+95,'Existing local orders','Branch actual-quintic order; rational nonbranch and derivative DVRs'),(y+190,'Next concrete composition','Actual-quintic nonbranch norm identity and zero order in one stalk'),(y+285,'Later global construction','Common function field -> all places -> principal divisor -> class sum')]:
 s.append(f'<rect x="45" y="{yy}" width="905" height="75" rx="7" fill="#edf3f7" stroke="#6b8da5"/>');txt(65,yy+29,title,21);txt(65,yy+56,body,17)
y+=401
y=para(y,'Proposed theorem: for the actual quintic and a rational nonbranch point with n(x0)=b*y0, derive the actual point DVR, scalar units, numerator nonunit and mapped source norm identity. Then apply point_norm_order to obtain the sum of support valuations. Do not assume those local conclusions as free interface arguments.')
y=para(y,'The selection condition matters: norm zero can select the conjugate factor. Derive n(x0)=b*y0 from interpolation for input-support corollaries. Use one affine-to-localization-to-projective-stalk map for every polynomial and coordinate.')
y=para(y,'Global work still needs a single function field, finite zero multiplicities at all closed points, compatible infinity pole6 versus5, concrete Mumford divisors and the principal-class quotient. Rational points alone do not cover closed points over a finite field. Source exceptional branches and sequential additions retain their own obligations.')
y=para(y,'Evidence distinction: saved direct axiom logs were inspected for later cross-chart and local-order developments. Properness/smoothness/infinityDVR sources match later saved build narratives. No distinct-author geometry fidelity review was located. No fresh compilation or cohort was performed.')
y=para(y,'NEXT.md gives exact real types/maps and a proposed, unapproved120-second source validation envelope after dependency custody. CSV and JSON separate existing evidence from proposed and global obligations. Historical1,260-target cohorts and matched-rho records remain unchanged.')
s.append('</g></svg>');assert y<1180,y;(p/'report.svg').write_text(''.join(s));subprocess.run(['rsvg-convert','-f','pdf','-o',str(p/'report.pdf'),str(p/'report.svg')],check=True)
