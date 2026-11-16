"""Worked diagrams appended to the research set; called by build_research.py."""

import json
import math


def matrix(d,x,y,entries,cellw=78,cellh=48,ids=None,size=27,color=None):
    color=color or INK
    rows,cols=len(entries),len(entries[0])
    width,height=cols*cellw,rows*cellh
    d.parts.append(f'<path d="M{x-8} {y-9} L{x-20} {y-9} L{x-20} {y+height+2} L{x-8} {y+height+2} M{x+width+8} {y-9} L{x+width+20} {y-9} L{x+width+20} {y+height+2} L{x+width+8} {y+height+2}" fill="none" stroke="{color}" stroke-width="2"/>')
    for i,row in enumerate(entries):
        for j,value in enumerate(row):
            extra=f'id="{ids[(i,j)]}"' if ids and (i,j) in ids else ''
            d.text(x+(j+.5)*cellw,y+32+i*cellh,value,size,color,anchor="middle",extra=extra)


def kernel_example(data):
    d=Figure(7,"Worked extraction: three rows can determine k—or fail","The relations below are constructed and checked in an actual toy elliptic group; they do not measure relation-search cost.","07-worked-kernel")
    d.rect(56,171,1488,104)
    d.tex(82,192,r'E/\mathbb{F}_{571}:y^2=x^3+508x+137,\quad |E|=557,\quad P=(517,168)',27,maxw=1400)
    Q=data['toy_curve']['Q']
    d.text(82,248,f"Input Q={tuple(Q)}. The factor base has two unknown logs. All three rows are valid group identities.",20,MUTED)
    d.rect(56,317,1488,298,"white",BLUE)
    d.tex(82,341,r'M\lambda=\mathbf{a}+k\mathbf{b}\quad(\mathrm{mod}\ 557)',30,maxw=950)
    for x,label in [(294,'M'),(644,'a'),(914,'b')]:d.text(x,421,label,26,BLUE,650,anchor='middle')
    matrix(d,216,446,data['kernel']['M'],78,44)
    matrix(d,605,446,[[494],[469],[332]],78,44,ids={(2,0):'rankA3'})
    matrix(d,875,446,[[2],[3],[7]],78,44,ids={(2,0):'rankB3'})
    d.text(1120,464,'rank M = 2',24,INK,650)
    d.text(1120,510,'rank [M | −b] = 3',23,INK,extra='id="rankAugmented"')
    d.text(1120,554,'3 rows in both modes',19,MUTED)
    d.rect(56,659,1488,139,BLUE_LIGHT,BLUE)
    d.tex(82,687,r'\mathbf{w}^{\top}=(-1,-1,1),\quad \mathbf{w}^{\top}M=0',29,maxw=920)
    d.text(82,762,'wᵀa = 483',25,BLUE,650,extra='id="rankWa"')
    d.text(480,762,'wᵀb = 2',25,BLUE,650,extra='id="rankWb"')
    d.tex(1020,686,r'k=-\frac{\mathbf{w}^{\top}\mathbf{a}}{\mathbf{w}^{\top}\mathbf{b}}',31,maxw=440)
    d.rect(56,838,1488,126,GREEN_LIGHT,'none',extra='id="rankOutcomeBox"')
    d.text(82,883,'k = −483 · 2⁻¹ = 37 (mod 557)',29,GREEN,650,extra='id="rankConclusion"')
    d.text(82,931,'A nonzero target-column pairing turns a factor-base dependency into the logarithm.',20,MUTED,extra='id="rankInterpretation"')
    d.footer("Toggle the target column in the viewer. Keeping three valid rows does not guarantee that their target coefficients carry information.")
    d.save("This is a constructed toy example, not harvested attack data. The curve over F571 is the coefficient reduction of the challenge curve; its order is 557. Q=[37]P and the chosen factor-base points are [11]P and [23]P, used only to construct and verify the rows. In the usable mode, a=(494,469,332), b=(2,3,7), w=(-1,-1,1), w^Ta=483 and w^Tb=2. Changing a3 to 406 and b3 to 5 keeps all rows valid but makes both pairings zero; the augmented rank falls to 2, so these rows alone leave k free.")


def triangle(data):
    d=Figure(8,"Partial relations: cancel the large-prime columns","An oriented graph is a bookkeeping device for signed divisor-class relations.","08-partial-relations")
    d.rect(56,171,1488,120)
    d.tex(82,184,r'R_i=[a_i]P+[b_i]Q=\sum_j e_{ij}F_j+\sum_\ell A_{i\ell}L_\ell',29,maxw=1400)
    d.text(82,273,'Fⱼ lie in the selected factor base. Lℓ are exceptional degree-one classes outside it; retain their signs.',18,MUTED)
    d.rect(56,314,650,437,PURPLE_LIGHT,PURPLE)
    d.rect(760,314,784,437,'white',PURPLE)
    d.text(82,351,'A SIGNED LARGE-PRIME TRIANGLE',14,PURPLE,700,spacing=.6)
    positions=[(380,410),(188,650),(574,650)]
    for i,j,label,lx,ly in [(0,1,'R₁',235,507),(1,2,'R₂',380,709),(2,0,'R₃',547,507)]:
        x,y=positions[i];u,v=positions[j]
        length=math.hypot(u-x,v-y);ux,uy=(u-x)/length,(v-y)/length
        d.arrow(x+49*ux,y+49*uy,u-49*ux,v-49*uy,'purple')
        d.text(lx,ly,label,25,PURPLE,650,anchor='middle')
    for i,(x,y) in enumerate(positions):
        d.parts.append(f'<circle cx="{x}" cy="{y}" r="43" fill="white" stroke="{PURPLE}" stroke-width="2"/>')
        d.tex(x,y-19,rf'L_{i+1}',30,PURPLE,anchor='middle')
    d.text(786,353,'THE EDGE EQUATIONS',14,PURPLE,700,spacing=.6)
    for i,expression in enumerate([r'R_1=F_1+L_1-L_2',r'R_2=F_2+L_2-L_3',r'R_3=F_3+L_3-L_1']):
        d.tex(786,382+i*62,expression,31,maxw=710)
    d.line(786,578,1518,578)
    d.tex(786,606,r'\mathbf{z}^{\top}A=0,\quad \mathbf{z}=(1,1,1)',29,maxw=710)
    d.text(786,694,'Add the rows; every exceptional class cancels.',21,MUTED)
    d.rect(56,793,1488,172,GREEN_LIGHT,GREEN)
    d.tex(800,824,r'R_1+R_2+R_3=F_1+F_2+F_3',33,GREEN,maxw=1390,anchor='middle')
    d.tex(800,886,r'[363]P+[7]Q=F_1+F_2+F_3\quad(\mathrm{toy\ order}\ 557)',27,maxw=1390,anchor='middle')
    d.footer("A cycle in an unsigned graph is not, by itself, a verified relation. Use the full signed coefficient matrix when eliminating exceptional classes.")
    d.save("The large-prime coefficient rows are (1,-1,0), (0,1,-1), and (-1,0,1), so their sum is zero. Signs matter: arbitrary relations with two exceptional terms do not automatically cancel around every unsigned cycle. The numerical check realizes the identities in the toy order-557 elliptic component with factor-base logs 11,23,31 and exceptional logs 5,9,14. It is an algebraic illustration of the recombination mechanism, not a claim that these data were collected in a hyperelliptic factor-base search. In the small-genus large-prime variation, the exceptional classes represented by the graph are degree-one places outside the selected factor base.")


def fiber_plot(d,rows):
    left,top,width,height=880,407,578,228
    xmin,xmax=2,math.log2(320)
    ymin,ymax=-5,0
    tx=lambda B:left+(math.log2(B)-xmin)/(xmax-xmin)*width
    ty=lambda probability:top+(ymax-math.log10(probability))/(ymax-ymin)*height
    for B in [4,16,64,256]:
        x=tx(B);d.line(x,top,x,top+height,LINE,1);d.text(x,660,str(B),15,MUTED,anchor='middle')
    for exponent in [-5,-4,-3,-2,-1,0]:
        y=top+(0-exponent)/5*height
        d.line(left,y,left+width,y,LINE,1)
        d.text(left-12,y+5,'1' if exponent==0 else f'10^{exponent}',14,MUTED,anchor='end')
    for key,color in [('E_probability',BLUE),('J_probability',GREEN)]:
        coords=[(tx(row['B']),ty(row[key])) for row in rows]
        d.parts.append('<polyline points="'+ ' '.join(f'{x},{y}' for x,y in coords)+f'" fill="none" stroke="{color}" stroke-width="2.5"/>')
        for x,y in coords:d.parts.append(f'<circle cx="{x}" cy="{y}" r="4.5" fill="{color}"/>')
    d.text(left+width/2,685,'B = number of signed factor-base points',16,MUTED,anchor='middle')


def fibers(data):
    d=Figure(9,"Fiber smoothness is a structured distribution","A rational divisor class can be supported on rational points or on a nonrational closed point.","09-fiber-census")
    d.rect(56,171,1488,107)
    d.tex(82,192,r'f:C_2\longrightarrow E,\quad (X,Y)\longmapsto(X^2,Y),\qquad u_P(X)=X^2-x_P',29,maxw=1400)
    d.text(82,251,'The published ECCp-131 points have χ(xP)=+1 and χ(xQ)=−1. Their pullbacks are both 𝔽p-rational classes.',19,MUTED)
    d.rect(56,316,678,438,'white',PURPLE)
    d.text(82,354,'ACTUAL ECCp-131 FIBERS',14,PURPLE,700,spacing=.6)
    d.tex(82,384,r'B_\pm=(\pm r,y_P),\qquad r^2=x_P',27,maxw=620)
    d.text(82,446,'f⁻¹(P)={B₊,B₋}: two rational points; uP splits over 𝔽p.',19,MUTED)
    d.line(82,479,708,479)
    d.tex(82,509,r'f^{-1}(Q)=\{A_+,A_-\},\quad \pi(A_+)=A_-',27,maxw=620)
    d.text(82,572,'One degree-two place: uQ is irreducible over 𝔽p.',20,MUTED)
    d.tex(82,610,r'[A_++A_- - (\infty_++\infty_-)]\in J(\mathbb{F}_p)',27,maxw=620)
    d.text(82,704,'Rationality of the class does not require rational support.',18,MUTED)
    d.rect(790,316,754,438,'white',GREEN)
    d.text(816,354,'EXACT TOY CENSUS: p=571, n=557',14,GREEN,700,spacing=.6)
    d.text(816,381,'|J(𝔽p)| = 557 · 609 = 339213; both axes logarithmic.',17,MUTED)
    fiber_plot(d,data['fiber_census']['rows'])
    d.tex(820,702,r'\Pr_E=\frac{B}{2(n-1)}',19,BLUE,maxw=325)
    d.tex(1134,702,r'\Pr_J=\frac{B^2/2-B}{|J(\mathbb{F}_p)|}',19,GREEN,maxw=380)
    d.rect(56,796,1488,167,AMBER_LIGHT,'none')
    d.tex(82,819,r'\rho_C=[2^{-1}\ (\mathrm{mod}\ n)]f_*,\qquad\rho_C f^*=\mathrm{id}',28,maxw=1400)
    d.tex(82,868,r'\rho_C[B_+-\infty_+]+\rho_C[B_- -\infty_-]=[2^{-1}]P+[2^{-1}]P=P',27,maxw=1400)
    d.text(82,943,'Fiber splitting gives correlated projected summands. It does not reveal their factor-base logarithms.',20,MUTED)
    d.footer("Census scope: F_B is closed under X↦−X and Y↦−Y, excludes branch/infinity points; S_B uses two distinct X-coordinates. See notes for counts.")
    d.save("For the published ECCp-131 curve, the quadratic character of x_P is +1 and that of x_Q is −1. The Q fiber is nevertheless an Fp-rational degree-two divisor because Frobenius swaps its geometric points. The plot uses a different, explicitly labeled toy curve E/F571: y²=x³+508x+137, #E=557. Its genus-two companion quotient has order 609, hence #J=339213. The signed factor-base point set F_B is stable under the deck and hyperelliptic involutions, and excludes X=0 and infinity. Let S_B consist of classes of A+A'−D_infinity supported on F_B with distinct X. There are B(B−1)/2 unordered point pairs, of which B/2 share an X; hence |S_B|=B²/2−B. They give distinct classes because nonspecial degree-two divisors on a genus-two curve are unique in their linear-equivalence class; distinct X excludes the hyperelliptic pencil. Exactly B/2 nonidentity elliptic targets have their full fiber in F_B. Thus the two displayed probabilities have different sampling spaces and are exact in this toy setup. The E targets and support-pair encodings are enumerated; the whole Jacobian is not. These counts are not an attack-cost estimate. The final projection identity is for a split fiber over P, with the specified infinity reference.")


def cost_model(data):
    d=Figure(10,"From more roots to less total work","A change in relation yield must be evaluated together with verification, recombination, rank and matrix costs.","10-cost-accounting")
    d.rect(56,171,1488,104)
    d.tex(82,196,r'T=T_{\mathrm{setup}}+T_{\mathrm{collect}}+T_{\mathrm{filter}}+T_{\mathrm{LA}}+T_{\mathrm{indiv}}',29,maxw=1400)
    d.text(82,250,'Use measured operation costs on the same target subgroup and factor-base setup before comparing methods.',20,MUTED)
    d.rect(56,319,678,247,BLUE_LIGHT,BLUE)
    d.text(82,357,'A PHASE-AVERAGE COLLECTION MODEL',14,BLUE,700)
    d.tex(82,386,r'c_{\mathrm{eff}}=\frac{c_{\mathrm{trial}}}{p_{\mathrm{valid}}\eta},\qquad T_{\mathrm{collect}}\approx R_{\mathrm{req}}c_{\mathrm{eff}}',29,maxw=620)
    d.lines(82,484,['pvalid = probability of a verified relation per trial.', 'η = effective information yield per verified relation.', 'Assume pvalid,η>0; rates may change during collection.'],18,MUTED,gap=28)
    d.rect(790,319,754,247,GREEN_LIGHT,GREEN)
    d.text(816,357,'FILTERED SPARSE LINEAR ALGEBRA',14,GREEN,700)
    d.tex(816,391,r'T_{\mathrm{LA}}\sim s\,C_{\mathrm{mv}},\qquad C_{\mathrm{mv}}\sim r\overline{w}\,c_n',28,maxw=690)
    d.tex(816,447,r'\mathrm{memory}\sim O(r\overline{w}+s)\ \mathrm{field\ entries}',26,maxw=690)
    d.lines(816,513,['Schematic Krylov scale after filtering to r≈s.', 'cₙ is modular-arithmetic cost; w̄ is average row weight.'],18,MUTED,gap=28)
    d.rect(56,611,1488,353,'white',INK)
    d.text(82,649,'ILLUSTRATIVE MODEL · NOT CHALLENGE ATTACK MEASUREMENTS',14,MUTED,700)
    d.text(82,689,'Collection cost per effective row, in normalized work units',20,MUTED)
    d.text(82,742,'Baseline A',23,BLUE,650)
    d.text(82,776,'c=1; pvalid=2%; η=50%',18,MUTED)
    d.rect(500,722,450,23,BLUE,'none',5,extra='id="modelBaselineBar"')
    d.text(965,741,'100',20,BLUE,extra='id="modelBaselineValue"')
    d.text(82,838,'Alternative B',23,GREEN,650)
    d.text(82,872,'c=4; pvalid=4%; η=50%',18,MUTED,extra='id="modelAlternativeDescription"')
    d.rect(500,818,900,23,GREEN,'none',5,extra='id="modelAlternativeBar"')
    d.text(1415,837,'200',20,GREEN,extra='id="modelAlternativeValue"')
    d.text(82,934,'Alternative B: 2.00× the baseline collection cost per effective row.',23,AMBER,650,extra='id="modelRatio"')
    d.footer("The collection formula is a diagnostic model, not a theorem. Matrix filtering can change dimensions and row weight; include every phase in total work.")
    d.save("The sliders are an illustrative phase-average model, not ECCp-131 measurements. Baseline A uses trial cost 1, verified-row probability 0.02, and effective information fraction 0.5, giving cost 100 per effective row. Alternative B initially has twice the verified-row yield but four times the trial cost, giving 200. Effective information is a modeling proxy encompassing rank/recombination; it is not automatically an independent Bernoulli success probability and may vary during collection. The sparse linear-algebra estimate describes a near-square filtered matrix and Wiedemann-style Krylov work, omitting algorithm-specific preconditioning, logarithmic factors, block strategies and dense-fill effects. Costs require a common calibrated work unit before comparison with rho or a different solver.")


def add_figures(context,data):
    global Figure,OUT,INK,MUTED,BLUE,BLUE_LIGHT,GREEN,GREEN_LIGHT,PURPLE,PURPLE_LIGHT,AMBER,AMBER_LIGHT,LINE
    for name in ['Figure','OUT','INK','MUTED','BLUE','BLUE_LIGHT','GREEN','GREEN_LIGHT','PURPLE','PURPLE_LIGHT','AMBER','AMBER_LIGHT','LINE']:
        globals()[name]=context[name]
    kernel_example(data);triangle(data);fibers(data);cost_model(data)


def controls(number):
    if number==7:
        return '<div class="controls"><label>Target-column scenario<select id="rankMode"><option value="usable">Usable left-kernel pairing</option><option value="dependent">Three valid rows, k undetermined</option></select></label><p>Both scenarios use the same curve, Q, factor base and matrix M. Only a₃,b₃ change; every relation is checked in the toy group.</p></div>'
    if number==10:
        return '<div class="controls"><label>Alternative trial cost<input id="trialCost" type="range" min="0.2" max="10" step="0.1" value="4"><output id="trialCostValue">4.0</output></label><label>Verified-row yield<input id="validYield" type="range" min="0.1" max="20" step="0.1" value="4"><output id="validYieldValue">4.0%</output></label><label>Information yield η<input id="infoYield" type="range" min="1" max="100" step="1" value="50"><output id="infoYieldValue">50%</output></label><p>Compare collection cost per effective row. Other phases remain outside this local illustration; the comparison bars share a scale that follows the larger value.</p></div>'
    return ''


def extra_js(data):
    return '''
const workedData=WORKED_DATA;
function updateRank(){const name=document.getElementById('rankMode').value,r=workedData.kernel.scenarios[name],good=name==='usable';document.getElementById('rankA3').textContent=r.a[2];document.getElementById('rankB3').textContent=r.b[2];document.getElementById('rankWa').textContent='wᵀa = '+r.wTa;document.getElementById('rankWb').textContent='wᵀb = '+r.wTb;document.getElementById('rankAugmented').textContent='rank [M | −b] = '+r.rank_augmented;document.getElementById('rankConclusion').textContent=good?'k = −483 · 2⁻¹ = 37 (mod 557)':'wᵀa = wᵀb = 0: these rows do not determine k';document.getElementById('rankConclusion').setAttribute('fill',good?'#087d68':'#a66a13');document.getElementById('rankOutcomeBox').setAttribute('fill',good?'#eaf6f1':'#fff4dd');document.getElementById('rankInterpretation').textContent=good?'A nonzero target-column pairing turns a factor-base dependency into the logarithm.':'The same kernel exists, but its target-column pairing vanishes. Relation count alone misses this.'}
document.getElementById('rankMode').addEventListener('change',updateRank);updateRank();
function updateCollection(){const c=Number(document.getElementById('trialCost').value),p=Number(document.getElementById('validYield').value)/100,eta=Number(document.getElementById('infoYield').value)/100,cost=c/(p*eta),baseline=100,scale=900/Math.max(cost,baseline),ratio=cost/baseline;document.getElementById('trialCostValue').textContent=c.toFixed(1);document.getElementById('validYieldValue').textContent=(p*100).toFixed(1)+'%';document.getElementById('infoYieldValue').textContent=Math.round(eta*100)+'%';document.getElementById('modelBaselineBar').setAttribute('width',baseline*scale);document.getElementById('modelBaselineValue').setAttribute('x',500+baseline*scale+15);document.getElementById('modelAlternativeBar').setAttribute('width',cost*scale);document.getElementById('modelAlternativeValue').setAttribute('x',500+cost*scale+15);document.getElementById('modelAlternativeValue').textContent=cost.toFixed(cost<1000?1:0);document.getElementById('modelAlternativeDescription').textContent='c='+c.toFixed(1)+'; pvalid='+(p*100).toFixed(1)+'%; η='+Math.round(eta*100)+'%';document.getElementById('modelRatio').textContent='Alternative B: '+ratio.toFixed(2)+'× the baseline collection cost per effective row.';document.getElementById('modelRatio').setAttribute('fill',ratio<=1?'#087d68':'#a66a13')}
['trialCost','validYield','infoYield'].forEach(id=>document.getElementById(id).addEventListener('input',updateCollection));updateCollection();
'''.replace('WORKED_DATA',json.dumps(data,separators=(',',':')))
