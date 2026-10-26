"""Build a research-level, offline, editable diagram set for index calculus.

SVG labels remain text; outlined formulas ship as LaTeX and JSON sources. The
original introductory diagrams are preserved in the parent directory.
"""

from html import escape
import io
import json
import math
import os
from pathlib import Path
import re
import xml.etree.ElementTree as ET
from continuation import add_figures, controls, extra_js
from worked_examples import build_data

os.environ.setdefault("MPLCONFIGDIR", "/tmp/eccp131-research-mpl")
import matplotlib
matplotlib.use("Agg")
matplotlib.rcParams["svg.fonttype"] = "path"
from matplotlib.font_manager import FontProperties
from matplotlib.mathtext import math_to_image


OUT = Path(__file__).resolve().parent
WIDTH, HEIGHT = 1600, 1040
INK, MUTED, BG = "#192b3b", "#586879", "#f7f6f2"
BLUE, BLUE_LIGHT = "#2864ad", "#edf3fc"
GREEN, GREEN_LIGHT = "#087d68", "#eaf6f1"
PURPLE, PURPLE_LIGHT = "#7555a5", "#f1edf8"
AMBER, AMBER_LIGHT, LINE = "#a66a13", "#fff4dd", "#d9dfe5"
FORMULAS, FIGURES = [], []
TOTAL_FIGURES = 10
EXAMPLES = None


class Figure:
    def __init__(self, number, title, subtitle, name):
        self.number, self.title, self.name = number, title, name
        self.parts = [f'<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="{WIDTH}" height="{HEIGHT}" viewBox="0 0 {WIDTH} {HEIGHT}" role="img" aria-labelledby="title{number} desc{number}"><title id="title{number}">{escape(title)}</title><desc id="desc{number}">{escape(subtitle)}</desc><defs>']
        for name,color in [("ink",INK),("blue",BLUE),("green",GREEN),("purple",PURPLE)]:
            self.parts.append(f'<marker id="r{number}{name}" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0 0 L10 5 L0 10z" fill="{color}"/></marker>')
        self.parts.append('</defs><style>text{font-family:DejaVu Sans,Arial,sans-serif}</style>')
        self.rect(0,0,WIDTH,HEIGHT,BG,"none",0)
        self.text(56,45,"INDEX CALCULUS · RESEARCH DIAGRAMS",14,MUTED,650,spacing=1.5)
        self.text(1544,45,f"{number:02d} / {TOTAL_FIGURES:02d}",15,MUTED,anchor="end")
        self.text(56,99,title,36,INK,700)
        self.text(56,139,subtitle,20,MUTED)

    def rect(self,x,y,w,h,fill="white",stroke=LINE,radius=18,extra=""):
        self.parts.append(f'<rect x="{x}" y="{y}" width="{w}" height="{h}" rx="{radius}" fill="{fill}" stroke="{stroke}" stroke-width="1.5" {extra}/>')

    def text(self,x,y,value,size=20,color=INK,weight=400,anchor="start",spacing=None,extra=""):
        spacing=f'letter-spacing="{spacing}"' if spacing is not None else ""
        self.parts.append(f'<text x="{x}" y="{y}" font-size="{size}" font-weight="{weight}" text-anchor="{anchor}" fill="{color}" {spacing} {extra}>{escape(str(value))}</text>')

    def lines(self,x,y,values,size=20,color=INK,gap=30,weight=400):
        for i,value in enumerate(values):
            self.text(x,y+i*gap,value,size,color,weight)

    def tex(self,x,y,expression,size=28,color=INK,maxw=None,anchor="start",extra=""):
        FORMULAS.append({"figure":self.number,"expression":expression})
        stream=io.BytesIO()
        math_to_image("$"+expression+"$",stream,prop=FontProperties(size=size),format="svg",color=color)
        raw="\n".join(line.rstrip() for line in stream.getvalue().decode().splitlines())
        root=ET.fromstring(raw)
        view=root.attrib["viewBox"]
        _,_,w,h=map(float,view.split())
        scale=min(1,maxw/w) if maxw else 1
        width,height=w*scale,h*scale
        if anchor=="middle":x-=width/2
        elif anchor=="end":x-=width
        inner=re.search(r'<svg[^>]*>(.*)</svg>',raw,re.S).group(1)
        inner=re.sub(r'<g id="patch_1">.*?</g>',"",inner,flags=re.S)
        inner="\n".join(line.rstrip() for line in inner.splitlines())
        # Prefix definitions so the inline figures have no glyph-ID collisions.
        prefix=f'r{self.number}m{len(FORMULAS)}-'
        for identifier in re.findall(r'id="([^"]+)"',inner):
            inner=inner.replace('id="'+identifier+'"','id="'+prefix+identifier+'"').replace('#'+identifier+'"','#'+prefix+identifier+'"').replace('#'+identifier+')','#'+prefix+identifier+')')
        self.parts.append(f'<svg x="{x}" y="{y}" width="{width}" height="{height}" viewBox="{view}" aria-label="{escape(expression,quote=True)}" {extra}>{inner}</svg>')
        return height

    def line(self,x1,y1,x2,y2,color=LINE,width=1.5,dashed=False):
        dash='stroke-dasharray="7 5"' if dashed else ""
        self.parts.append(f'<path d="M{x1} {y1} L{x2} {y2}" fill="none" stroke="{color}" stroke-width="{width}" {dash}/>')

    def arrow(self,x1,y1,x2,y2,color="ink"):
        palette={"ink":INK,"blue":BLUE,"green":GREEN,"purple":PURPLE}
        self.parts.append(f'<path d="M{x1} {y1} L{x2} {y2}" fill="none" stroke="{palette[color]}" stroke-width="2.5" marker-end="url(#r{self.number}{color})"/>')

    def footer(self,text):
        self.line(56,988,1544,988)
        self.text(56,1018,text,15,MUTED)

    def save(self,notes):
        source="\n".join(self.parts)+"</svg>"
        (OUT/(self.name+".svg")).write_text(source)
        FIGURES.append({"number":self.number,"title":self.title,"name":self.name,"notes":notes})


def relations():
    d=Figure(1,"Relations as a module over the target prime field","Project to the cyclic order-n component before interpreting factor-base entries as logarithms.","01-relation-module")
    d.rect(56,171,1488,112)
    d.tex(82,192,r'G_n=\langle P\rangle\cong\mathbb{Z}/n\mathbb{Z},\qquad Q=[k]P,\quad n\ \mathrm{prime}',29,maxw=1100)
    d.text(82,258,"Assume a retraction ρ : G → Gₙ with ρ(P)=P; set Fⱼ=ρ(Bⱼ) for ambient factor-base elements Bⱼ.",20,MUTED)
    for x,color,light,label,expr,description in [
        (56,BLUE,BLUE_LIGHT,"ELLIPTIC ENGINE",r'R_i=[a_i]P+[b_i]Q=\sum_j e_{ij}F_j',["Constrained point decomposition", "Summation polynomials and a lift/sign check"]),
        (852,GREEN,GREEN_LIGHT,"JACOBIAN ENGINE",r'[D_i]=[a_i]P+[b_i]Q=\sum_j e_{ij}F_j',["Smoothness of reduced divisor classes", "Factorization and partial-relation recombination"]),
    ]:
        d.rect(x,330,692,229,light,color)
        d.text(x+26,369,label,15,color,700,spacing=1)
        d.tex(x+26,395,expr,27,maxw=638)
        d.lines(x+26,489,description,20,MUTED,gap=30)
    d.arrow(402,572,402,605,"blue")
    d.arrow(1198,572,1198,605,"green")
    d.rect(298,617,1004,115)
    d.text(325,653,"RELATION MATRIX",14,MUTED,650,spacing=1)
    d.tex(800,673,r'M\lambda=\mathbf{a}+k\mathbf{b}\quad (\mathrm{mod}\ n),\qquad M\in\mathbb{F}_n^{r\times s}',29,anchor="middle",maxw=930)
    d.rect(56,781,1488,180,"white",INK)
    d.text(82,817,"LEFT-KERNEL EXTRACTION",14,MUTED,650,spacing=1)
    d.tex(90,846,r'\mathbf{w}^{\top}M=0',29,maxw=360)
    d.arrow(405,865,465,865)
    d.tex(500,842,r'\mathbf{w}^{\top}\mathbf{a}+k\mathbf{w}^{\top}\mathbf{b}=0',28,maxw=510)
    d.arrow(1050,865,1110,865)
    d.tex(1140,829,r'k=-\frac{\mathbf{w}^{\top}\mathbf{a}}{\mathbf{w}^{\top}\mathbf{b}}',30,maxw=350)
    d.text(82,934,"Required: wᵀb ≠ 0 modulo n. Rank and denominator checks remain necessary after relation collection.",20,MUTED)
    d.footer("Example projector when #G=hn and gcd(h,n)=1: ρ=[h⁻¹ mod n]∘[h].  General primary decompositions require their own projector.")
    d.save("A retraction onto the cyclic order-n component is an explicit hypothesis; such a retraction does not exist for every subgroup of an arbitrary finite abelian group. If #G=hn with gcd(h,n)=1, [h] followed by the inverse of h modulo n supplies one. For the ECCp-131 cover, f* composed with [2 inverse modulo n] and f_* supplies a retraction onto the embedded elliptic group. The matrix columns are logarithms of projected factor-base elements. A usable left-kernel vector must have nonzero pairing with b; a raw count of relations does not certify this.")


def smoothness():
    d=Figure(2,"Divisor smoothness: factorization and support","An imaginary hyperelliptic model makes the infinity convention and the degree of a reduced divisor explicit.","02-divisor-smoothness")
    d.rect(56,171,1488,112)
    d.tex(82,192,r'C/\mathbb{F}_q:\ y^2=f(x),\quad \deg f=2g+1,\quad f\ \mathrm{squarefree},\quad\operatorname{char}\mathbb{F}_q\ne2',28,maxw=1420)
    d.text(82,258,"Use the unique point ∞ as base point; the divisor class group is J=Pic⁰(C).",20,MUTED)
    cards=[(56,480,"REDUCED MUMFORD DATA"),(610,420,"FACTOR u OVER 𝔽q"),(1104,440,"PRIME-DIVISOR CLASSES")]
    for x,width,label in cards:
        d.rect(x,331,width,283,GREEN_LIGHT,GREEN)
        d.text(x+24,371,label,14,GREEN,700,spacing=.6)
    d.tex(82,398,r'D=(u,v),\quad u\mid(f-v^2)',28,maxw=426)
    d.tex(82,456,r'\deg v<\deg u\leq g',27,maxw=426)
    d.text(82,554,"u monic; take squarefree support",19,MUTED)
    d.text(82,584,"for the displayed decomposition.",19,MUTED)
    d.arrow(548,469,596,469,"green")
    d.tex(638,405,r'u=\prod_j u_j',33,maxw=360)
    d.tex(638,480,r'd_j=\deg u_j',27,maxw=360)
    d.text(638,550,"Each irreducible factor plus",19,MUTED)
    d.text(638,581,"v mod uⱼ determines a place.",19,MUTED)
    d.arrow(1042,469,1090,469,"green")
    d.tex(1130,407,r'F_j=[\mathfrak{p}_j-d_j\infty]',29,maxw=388)
    d.tex(1130,479,r'\deg\mathfrak{p}_j=d_j',27,maxw=388)
    d.text(1130,551,"Degree-one places correspond",19,MUTED)
    d.text(1130,581,"to rational points of C.",19,MUTED)
    d.rect(56,651,1488,102)
    d.tex(800,679,r'[D]=\sum_j[\mathfrak{p}_j-d_j\infty]\ ;\qquad D\ \mathrm{is\ smooth\ when\ all}\ F_j\in\mathcal{F}',30,anchor="middle",maxw=1400)
    d.rect(56,794,888,169,"white",GREEN)
    d.text(82,831,"A COUNTING HEURISTIC",14,GREEN,650,spacing=.8)
    d.tex(82,855,r'\Pr[D\ \mathcal{F}\mathrm{-smooth}]\approx\frac{1}{g!}\left(\frac{B}{q}\right)^g',31,maxw=820)
    d.text(82,943,"B counts admissible signed degree-one classes; full B≈q gives ≈1/g!.",18,MUTED)
    d.rect(980,794,564,169,AMBER_LIGHT,"none")
    d.text(1006,831,"DOUBLE LARGE-PRIME VARIATION",14,AMBER,650)
    d.lines(1006,868,["Keep partial relations with ≤2 degree-one", "places outside the selected factor base.", "Eliminate those places by recombination."],18,MUTED,gap=29)
    d.footer("Heuristic: uniform ambient classes, fixed g and distinct support. Restricted elliptic subfamilies need separate relation statistics.")
    d.save("This page uses an odd-degree imaginary model with one rational point at infinity. The factorization formula is displayed for squarefree u. The probability is a counting heuristic, not a universal theorem for structured factor bases or an embedded elliptic subgroup. B counts the signed set of admissible degree-one classes; quotienting by the involution changes the bookkeeping. In the small-genus large-prime variation, the exceptional primes here are degree-one places outside the chosen factor base.")


def summation():
    d=Figure(3,"Summation polynomials and Weil restriction","The difficult task is finding admissible solutions; polynomial identities alone do not establish a fast solver.","03-summation-weil")
    d.rect(56,171,1488,112)
    d.tex(82,191,r'E/K:\ y^2=x^3+ax+b,\qquad K=\mathbb{F}_{q^d},\quad \mathcal{F}=\{\pm P:x(P)\in\mathbb{F}_q\}',28,maxw=1400)
    d.text(82,258,"Assume characteristic >3 for the displayed short model. More general factor bases impose their own coordinate constraints.",19,MUTED)
    d.rect(56,330,454,184,BLUE_LIGHT,BLUE)
    d.text(82,369,"POINT DECOMPOSITION",14,BLUE,700,spacing=.8)
    d.tex(82,386,r'R=\sum_{i=1}^{m}\epsilon_iP_i',32,maxw=390)
    d.text(82,501,"xᵢ ∈ 𝔽q; εᵢ ∈ {+1,−1}",19,MUTED)
    d.arrow(523,424,579,424,"blue")
    d.rect(594,330,950,184,"white",BLUE)
    d.tex(620,365,r'S_{m+1}(x_1,\ldots,x_m,x_R)=0',32,maxw=880)
    d.tex(620,425,r'\deg_{x_i}S_{m+1}=2^{m-1}\quad(m\geq2)',27,maxw=880)
    d.text(620,485,"Roots encode geometric lifts; verify field membership and the signed group relation.",18,MUTED)
    d.rect(56,562,1488,176,"white",INK)
    d.text(82,598,"CHOOSE A BASIS 1, ω, …, ωᵈ⁻¹ OF K OVER 𝔽q",14,MUTED,650,spacing=.6)
    d.tex(82,614,r'S_{m+1}=\sum_{\nu=0}^{d-1}s_\nu(x_1,\ldots,x_m)\omega^\nu',30,maxw=770)
    d.arrow(892,660,963,660)
    d.tex(995,631,r's_0=\cdots=s_{d-1}=0',30,maxw=500)
    d.text(82,719,"Dimension heuristic: m−d. At m=d the system is generically zero-dimensional only if these constraints are independent.",18,MUTED)
    d.rect(56,785,712,178,BLUE_LIGHT,BLUE)
    d.text(82,823,"GENERIC EXTENSION-FIELD REGIME",14,BLUE,650)
    d.tex(82,848,r'\widetilde{O}(q^{2-2/d}),\qquad d\geq3\ \mathrm{fixed}',30,maxw=650)
    d.lines(82,914,["Gaudry’s heuristic regime: a suitable nondegenerate", "factor base and controlled decomposition cost."],18,MUTED,gap=27)
    d.rect(812,785,732,178,AMBER_LIGHT,"none")
    d.text(838,823,"SUBFIELD TARGET SPECIALIZATION",14,AMBER,650)
    d.tex(838,848,r'a,b,x_R,x_i\in\mathbb{F}_q\ \Longrightarrow\ s_1=\cdots=s_{d-1}\equiv0',26,maxw=676)
    d.lines(838,914,["The canonical system collapses to one base-field equation.", "A nonrational x-line changes this system and must be analyzed."],18,MUTED,gap=27)
    d.footer("A summation-polynomial zero is not yet a verified relation in the intended group.  Solver and independence assumptions are substantive.")
    d.save("Semaev polynomials characterize geometric point-sum relations. Each variable of S_(m+1) has degree 2^(m−1). For x_i in the base field, coefficient expansion over a degree-d extension gives d equations; independence is an additional condition. The fixed-d Gaudry cost is a heuristic algorithmic regime, not an unconditional bound for every curve, target specialization, or factor base. For a,b,x_R and x_i all in the base field, the other coefficient equations vanish identically. This statement does not cover arbitrary nonrational affine x-lines.")


def covers():
    d=Figure(4,"Covers, norm maps and elliptic factors","A geometric cover gives a subgroup embedding on prime-to-degree torsion; it does not change the target order.","04-covers-prym")
    d.rect(56,171,692,164,PURPLE_LIGHT,PURPLE)
    d.text(82,209,"GENUS 2 · HYPERELLIPTIC",14,PURPLE,700,spacing=.7)
    d.tex(82,235,r'C_2:\ Y^2=X^6+aX^2+b',31,maxw=632)
    d.text(82,313,"f₂(X,Y)=(X²,Y); b≠0 and 4a³+27b²≠0",19,MUTED)
    d.rect(904,171,640,164,BLUE_LIGHT,BLUE)
    d.text(930,209,"GENUS 1 · ELLIPTIC",14,BLUE,700,spacing=.7)
    d.tex(930,235,r'E:\ y^2=x^3+ax+b',31,maxw=578)
    d.text(930,313,"All maps and equations are over the same field 𝔽p.",18,MUTED)
    d.arrow(766,255,888,255,"purple")
    d.text(826,227,"deg f₂=2",16,PURPLE,anchor="middle")
    d.rect(56,399,692,180,PURPLE_LIGHT,PURPLE)
    d.tex(82,428,r'J_2=\operatorname{Jac}(C_2)\sim E\times\widetilde{E}',31,maxw=632)
    d.tex(82,492,r'\widetilde{E}:\ v^2=bu^3+au^2+1',27,maxw=632)
    d.text(82,557,"The product decomposition is through a (2,2)-isogeny.",19,MUTED)
    d.rect(904,399,640,180,BLUE_LIGHT,BLUE)
    d.tex(930,427,r'\operatorname{Pic}^0(E)\cong E,\qquad Q=[k]P',30,maxw=578)
    d.tex(930,494,r'f_2^*Q=[k]f_2^*P',31,maxw=578)
    d.text(930,557,"If n is odd, the embedded subgroup still has order n.",19,MUTED)
    d.arrow(888,447,766,447,"blue")
    d.tex(826,410,r'f_2^*',28,BLUE,anchor="middle")
    d.arrow(766,530,888,530,"purple")
    d.tex(826,541,r'f_{2*}',26,PURPLE,anchor="middle")
    d.rect(56,620,1488,80,"white",INK)
    d.tex(800,642,r'f_{2*}\circ f_2^*=[2]\quad\Longrightarrow\quad\ker f_2^*\cap E[n]=\{O\}\ \ (n\ \mathrm{odd})',31,anchor="middle",maxw=1400)
    d.rect(56,746,750,216,"white",PURPLE)
    d.text(82,784,"EXPLICIT FIBER DIVISOR",14,PURPLE,650)
    d.tex(82,811,r'u(X)=X^2-x_P,\qquad v(X)=y_P',29,maxw=690)
    d.tex(82,865,r'f_2^*[P-O]=[f_2^{-1}(P)-(\infty_++\infty_-)]',26,maxw=690)
    d.text(82,940,"These are fiber data on the even-degree model, not an arbitrary reduced class.",17,MUTED)
    d.rect(850,746,694,216,"white",PURPLE)
    d.text(876,784,"GENUS 3 · NON-HYPERELLIPTIC ALTERNATIVE",14,PURPLE,650)
    d.tex(876,811,r'C_3:\ W^4=X^3+aX+b,\quad f_3(X,W)=(X,W^2)',27,maxw=638)
    d.tex(876,870,r'\operatorname{Jac}(C_3)\sim E\times\operatorname{Prym}(C_3/E)',26,maxw=638)
    d.text(876,940,"Smooth plane quartic; dim Prym = 2.  The degree-two norm identity also holds.",16,MUTED)
    d.footer("Characteristic >3.  ECCp-131 satisfies the smoothness conditions. The symbol ∼ denotes an isogeny, not equality of groups or polarizations.")
    d.save("For C2 the other elliptic quotient is u=X^(−2), v=Y/X^3, with equation v²=b u³+a u²+1. The two quotient maps give a (2,2)-split Jacobian. The sextic has two rational infinity points, so the fiber divisor uses infinity_+ + infinity_- as reference. A general even-degree reduced-divisor convention needs care. For C3, the projective equation W^4=X^3 Z+aXZ^3+bZ^4 is smooth when the elliptic cubic is squarefree and the characteristic exceeds 3; its degree-two cover has an abelian-surface Prym. The norm–pullback identity proves injectivity on odd-order torsion, without implying that the full Jacobian is cyclic.")


def extensions():
    d=Figure(5,"Frobenius decomposes the extension-field groups","For ECCp-131, the target prime factor is confined to the base-field summand in both extensions.","05-frobenius-components")
    d.rect(56,171,1488,100)
    d.tex(82,191,r'\pi=\operatorname{Frob}_p,\quad t=p+1-n,\quad |E(\mathbb{F}_p)|=n\ \mathrm{prime}',29,maxw=1400)
    d.text(82,247,"For the published curve: gcd(n,n′)=gcd(n,M₃)=1, n≠2,3.  These are checked arithmetic conditions.",20,MUTED)
    d.rect(56,314,712,428,BLUE_LIGHT,BLUE)
    d.rect(824,314,720,428,PURPLE_LIGHT,PURPLE)
    d.text(82,353,"QUADRATIC EXTENSION",15,BLUE,700,spacing=.6)
    d.tex(82,383,r'N_2=nn^{\prime},\qquad n^{\prime}=p+1+t',29,maxw=650)
    d.tex(82,441,r'e_+=\frac{1+\pi}{2},\qquad e_-=\frac{1-\pi}{2}',31,maxw=650)
    d.tex(82,513,r'E(\mathbb{F}_{p^2})\cong E(\mathbb{F}_p)\times E^{\prime}(\mathbb{F}_p)',26,maxw=650)
    d.text(82,578,"Here the group has odd order, so division by 2 is defined.",18,MUTED)
    d.tex(82,609,r'e_+E^{\prime}(\mathbb{F}_p)=\{O\}',28,maxw=650)
    d.text(82,684,"Set ρ₂=e₊. For R∈E(𝔽p), the twist component e₋R is zero.",19,MUTED)
    d.text(850,353,"CUBIC EXTENSION",15,PURPLE,700,spacing=.6)
    d.tex(850,383,r'N_3=nM_3,\quad M_3=p^2-p+1+(p+1)t+t^2',26,maxw=660)
    d.tex(850,441,r'T_3=1+\pi+\pi^2,\qquad \rho_3=[3^{-1}\ (\mathrm{mod}\ n)]T_3',27,maxw=660)
    d.tex(850,513,r'E(\mathbb{F}_{p^3})\cong E(\mathbb{F}_p)\times\ker T_3',26,maxw=660)
    d.tex(850,580,r'\{P\in E(\mathbb{F}_{p^3}):x(P)\in\mathbb{F}_p\}=E(\mathbb{F}_p)\setminus\{O\}',24,maxw=660)
    d.lines(850,650,["For a finite point with x∈𝔽p, the y-coordinate has", "degree dividing both 2 and 3, hence lies in 𝔽p."],18,MUTED,gap=29)
    d.rect(56,791,1488,172,AMBER_LIGHT,"none")
    d.text(82,829,"NONRATIONAL FACTOR BASES CAN ESCAPE THESE CANONICAL BASES",14,AMBER,650,spacing=.5)
    d.tex(82,853,r'R=\sum_j e_jB_j\quad\Longrightarrow\quad R=\sum_j e_j\rho_d(B_j),\quad R\in E(\mathbb{F}_p),\ d=2,3',28,maxw=1400)
    d.text(82,938,"Projection constrains any resulting relation; it does not rule out every algebraic improvement in how relations are generated.",20,MUTED)
    d.footer("The displayed direct products use the checked ECCp-131 hypotheses. Without them, intersections and primary-component issues require care.")
    d.save("For ECCp-131 the full quadratic group is odd and decomposes into the +1 and −1 eigenspaces, identified with the curve and its quadratic twist over Fp. For the cubic group, T3 is surjective onto E(Fp), since multiplication by 3 is invertible there; rho3 is its normalized retraction. The verified coprimality also separates the target prime from ker T3. Infinity is excluded from the literal x-coordinate factor base on the cubic side. Nonrational affine x-lines produce other factor bases; projecting their relations remains valid but does not prove a universal algorithmic lower bound.")


def costs():
    d=Figure(6,"Complexity regimes: ambient group or target subgroup?","Compare asymptotic work exponents with the prime subgroup order n stated explicitly.","06-complexity-regimes")
    d.rect(56,171,1488,89)
    d.text(82,204,"DEFAULT: ECCp-131 EMBEDDINGS",13,MUTED,650,spacing=.7,extra='id="modeTag"')
    d.text(82,241,"Same target: log₂ n ≈ log₂ q = 130.19",24,INK,650,extra='id="comparisonLabel"')
    d.rect(1118,198,20,14,BLUE,"none",3)
    d.text(1149,211,"Rho on the target",17,MUTED)
    d.rect(1335,198,20,14,GREEN,"none",3)
    d.text(1366,211,"IC benchmark",16,MUTED)
    left,scale=500,950/260
    for tick in range(0,251,50):
        x=left+tick*scale
        d.line(x,282,x,763,LINE,1)
        d.text(x,797,str(tick),16,MUTED,anchor="middle")
    for i,(label,g,ic,formula) in enumerate([
        ("Elliptic prime-field group",1,None,""),
        ("Genus-2 hyperelliptic",2,1,r'\widetilde O(q^{2-2/g})=\widetilde O(q)'),
        ("Genus-3 hyperelliptic",3,4/3,r'\widetilde O(q^{4/3})'),
        ("Genus-3 plane quartic",3,1,r'\widetilde O(q)'),
    ]):
        y=302+i*122
        d.text(80,y+16,label,22,INK,650)
        d.text(80,y+46,"n≈q; ambient order ≈ "+("q" if g==1 else "q"+("²" if g==2 else "³")),17,MUTED,extra=f'id="orderLabel{i}"')
        if formula:d.tex(80,y+62,formula,20,MUTED,maxw=390)
        rho=130.18749351230494/2
        d.rect(left,y,rho*scale,20,BLUE,"none",5,extra=f'id="rhoBar{i}"')
        d.text(left+rho*scale+12,y+17,f"2^{rho:.1f}",18,BLUE,extra=f'id="rhoText{i}"')
        if ic is None:
            d.text(left,y+58,"No established general prime-field IC improvement",18,MUTED)
        else:
            exponent=130.18749351230494*ic
            d.rect(left,y+37,exponent*scale,20,GREEN,"none",5,extra=f'id="icBar{i}" opacity="0.55"')
            d.text(left+exponent*scale+12,y+54,f"2^{exponent:.1f} †",18,GREEN,extra=f'id="icText{i}"')
    d.text(975,837,"Exponent e in asymptotic work ≈ 2ᵉ",20,MUTED,anchor="middle")
    d.rect(56,866,1488,96,AMBER_LIGHT,"none")
    d.text(82,902,"† Ambient IC benchmark: split-Jacobian hypotheses and restricted-subgroup relation statistics must be rechecked.",19,AMBER,extra='id="costCaveat"')
    d.text(82,937,"The target subgroup retains n≈q under the covers. Its rho exponent is therefore log₂(n)/2 in every row.",19,MUTED,extra='id="costExplanation"')
    d.footer("Fixed g; constants and logarithmic factors suppressed. Algorithmic hypotheses and operation costs differ; these are not runtime predictions.")
    d.save("The default chart holds the embedded target order n≈q fixed while retaining the original field; its rho cost is sqrt(n), even in a larger Jacobian. The displayed IC bars in that mode are ambient benchmarks, marked with a dagger: their model/class-group and relation-statistics assumptions do not automatically hold for split covers. The equal-field mode instead assumes a large generic subgroup of order about q^g. The equal-group mode compares independently parameterized generic groups of order N, using q≈N^(1/g); it does not describe the covers of ECCp-131. Hyperelliptic q^(2−2/g) statements require fixed genus and the hypotheses of the selected theorem/algorithm, including cyclic class-group assumptions in the cited rigorous result. The plane-quartic O-tilde(q) result uses its own geometric conditions. These are upper-bound regimes, not lower bounds or measured runtimes.")


def write_sources():
    (OUT/"formula-sources.json").write_text(json.dumps(FORMULAS,indent=2)+"\n")
    content=[r'\documentclass{article}',r'\usepackage{amsmath,amssymb}',r'\begin{document}']
    for figure in FIGURES:
        content.append(r'\section*{'+figure["title"]+"}")
        for item in FORMULAS:
            if item["figure"]==figure["number"]:
                content.extend([r'\[',item["expression"],r'\]'])
    content.append(r'\end{document}')
    (OUT/"research-formulas.tex").write_text("\n".join(content)+"\n")
    (OUT/"figures.json").write_text(json.dumps(FIGURES,indent=2)+"\n")


def viewer():
    navigation="".join(f'<a href="#figure{f["number"]}">{f["number"]} · {escape(f["title"])}</a>' for f in FIGURES)
    panels=[]
    for f in FIGURES:
        if controls(f["number"]):panels.append(controls(f["number"]))
        if f["number"]==6:
            panels.append('''<div class="controls"><label>Comparison<select id="mode"><option value="embedded">ECCp-131 embedded subgroup</option><option value="field">Equal field size</option><option value="group">Equal generic group size</option></select></label><label><span id="sliderName">Field-size bits</span><input id="bits" type="range" min="48" max="160" step="0.1" value="130.2"><output id="bitValue">130.2</output></label><button id="saveCost">Export current chart SVG</button><p id="modeDescription"></p></div>''')
        svg=(OUT/(f["name"]+".svg")).read_text()
        panels.append(f'<section class="paper" id="figure{f["number"]}">{svg}</section><details class="notes"><summary>Mathematical assumptions and notation · Figure {f["number"]}</summary><p>{escape(f["notes"])}</p><button data-svg="figure{f["number"]}" data-name="{f["name"]}.svg">Export figure SVG</button></details>')
    html='''<!doctype html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Index calculus — research diagrams</title><style>
*{box-sizing:border-box}body{margin:0;background:#e8ebee;color:#192b3b;font-family:system-ui,Arial,sans-serif}header,.sources{max-width:1600px;margin:auto;padding:28px 28px 22px}h1{margin:0;font-size:30px}header p,.sources p{line-height:1.6;color:#586879}nav{display:flex;flex-wrap:wrap;gap:8px;margin-top:20px}a,button{color:#2864ad}nav a,button{border:1px solid #cbd4dc;border-radius:8px;background:white;padding:9px 12px;font:inherit;text-decoration:none}button{cursor:pointer}.paper{max-width:1600px;margin:0 auto;background:#f7f6f2;scroll-margin-top:12px;box-shadow:0 10px 30px #192b3b12}.paper>svg{display:block;width:100%;height:auto}.notes,.controls{max-width:1600px;margin:0 auto 26px;background:white;border:1px solid #d9dfe5;border-radius:0 0 12px 12px;padding:20px 28px}.notes p{line-height:1.65;color:#586879}.notes summary{cursor:pointer;font-weight:600}.controls{border-radius:12px;display:flex;flex-wrap:wrap;align-items:center;gap:22px;margin-top:20px}.controls label{display:flex;gap:10px;align-items:center;font-weight:600}.controls select{font:inherit;padding:8px;border-radius:7px;border:1px solid #cbd4dc}.controls input{width:220px}.controls p{width:100%;margin:0;color:#586879;line-height:1.6}.sources{background:white;border-radius:12px;margin-bottom:30px}@page{size:16in 10.4in;margin:0}@media print{body{background:white}header,.notes,.controls,.sources{display:none!important}.paper{max-width:none;width:16in;height:10.4in;margin:0;box-shadow:none;break-after:page}.paper:last-of-type{break-after:auto}.paper>svg{width:16in;height:10.4in}}
</style></head><body><header><h1>Index calculus: research-level diagrams</h1><p>Six diagrams with explicit subgroup maps, relation equations, geometric hypotheses and complexity regimes. Labels remain SVG text; equations are vector outlines with LaTeX sources in the bundle. The original introductory set is preserved.</p><button onclick="window.print()">Print / save six figures as PDF</button><nav>NAVIGATION</nav></header>PANELS
<section class="sources"><strong>Primary references and computational context</strong><p><a href="https://eprint.iacr.org/2004/031">Semaev: summation polynomials (2004)</a> · <a href="https://doi.org/10.1016/j.jsc.2008.08.005">Gaudry: abelian varieties of small dimension (2009)</a> · <a href="https://eprint.iacr.org/2004/153">Gaudry–Thomé–Thériault–Diem: double large primes</a> · <a href="https://arxiv.org/abs/math/0606607">Diem: fixed-genus class groups with cyclic class group</a> · <a href="https://doi.org/10.5802/jtnb.815">Diem: plane curves (2012)</a> · <a href="https://arxiv.org/abs/math/0312413">Diem: degree-two genus-two covers</a> · <a href="https://www.certicom.com/en/curves-list">Published Certicom parameters</a></p><p>ECCp-131 is the prime-field challenge. The stored arithmetic checks validate its P,Q and subgroup order, extension-field identities, and cover smoothness conditions; the challenge logarithm was not computed. Heuristic statements and exact identities are separated in each figure’s notes.</p></section>
<script>
const mode=document.getElementById('mode'),bits=document.getElementById('bits'),rows=[{g:1,ic:null},{g:2,ic:1},{g:3,ic:4/3},{g:3,ic:1}];
function update(){const b=Number(bits.value),m=mode.value,scale=950/260,embedded=m==='embedded',group=m==='group';document.getElementById('bitValue').textContent=b.toFixed(1);document.getElementById('sliderName').textContent=group?'Generic group-size bits':'Field-size bits';document.getElementById('modeTag').textContent=embedded?'ECCp-131 COVER REGIME':group?'INDEPENDENT GENERIC GROUPS':'GENERIC AMBIENT GROUPS';document.getElementById('comparisonLabel').textContent=(embedded?'Same target: log₂ n ≈ log₂ q = ':group?'Same group: log₂ N = ':'Same field: log₂ q = ')+b.toFixed(1);document.getElementById('modeDescription').textContent=embedded?'Keep the target subgroup order n≈q inside each cover Jacobian. IC bars are ambient benchmarks whose hypotheses require rechecking for the split covers.':group?'Compare independently parameterized large generic subgroups of order N, with field size q≈N^(1/g). This does not model a cover of the original challenge.':'Fix the field q and assume large generic subgroups of order approximately q^g. The compared target groups therefore have different sizes.';document.getElementById('costCaveat').textContent=embedded?'† Ambient IC benchmark: split-Jacobian hypotheses and restricted-subgroup relation statistics must be rechecked.':'Fixed small genus. The cited algorithms have model, class-group and geometric hypotheses; these are not lower bounds.';document.getElementById('costExplanation').textContent=embedded?'The target subgroup retains n≈q under the covers. Its rho exponent is therefore log₂(n)/2 in every row.':group?'The field size changes with genus. These are independent generic DLP instances with the same subgroup order.':'Rho uses the displayed generic subgroup order n≈q^g; the genus-one and higher-genus targets differ in size.';rows.forEach((r,i)=>{const qb=group?b/r.g:b,rho=embedded?b/2:r.g*qb/2,ic=r.ic*qb;document.getElementById('rhoBar'+i).setAttribute('width',rho*scale);document.getElementById('rhoText'+i).setAttribute('x',500+rho*scale+12);document.getElementById('rhoText'+i).textContent='2^'+rho.toFixed(1);document.getElementById('orderLabel'+i).textContent=embedded?'n≈q; ambient order ≈ '+(r.g===1?'q':'q'+(r.g===2?'²':'³')):group?'log₂ q≈'+qb.toFixed(1)+'; n≈N':'n≈'+(r.g===1?'q':'q'+(r.g===2?'²':'³'));if(r.ic!==null){document.getElementById('icBar'+i).setAttribute('width',ic*scale);document.getElementById('icBar'+i).setAttribute('opacity',embedded?'0.55':'1');document.getElementById('icText'+i).setAttribute('x',500+ic*scale+12);document.getElementById('icText'+i).textContent='2^'+ic.toFixed(1)+(embedded?' †':'')}})}
function saveSVG(section,name){const svg=document.querySelector('#'+section+' svg'),blob=new Blob([new XMLSerializer().serializeToString(svg)],{type:'image/svg+xml'}),url=URL.createObjectURL(blob),a=document.createElement('a');a.href=url;a.download=name;a.click();setTimeout(()=>URL.revokeObjectURL(url),1000)}
mode.addEventListener('change',update);bits.addEventListener('input',update);document.getElementById('saveCost').addEventListener('click',()=>saveSVG('figure6','complexity-'+mode.value+'-'+bits.value+'bits.svg'));document.querySelectorAll('[data-svg]').forEach(b=>b.addEventListener('click',()=>saveSVG(b.dataset.svg,b.dataset.name)));update();
</script></body></html>'''
    html=html.replace("Six diagrams", "Ten diagrams").replace("save six figures", "save ten figures")
    html=html.replace('<a href="https://www.certicom.com/en/curves-list">', '<a href="https://doi.org/10.1109/TIT.1986.1057137">Wiedemann: sparse linear equations</a> · <a href="https://www.certicom.com/en/curves-list">')
    html=html.replace('</script>',extra_js(EXAMPLES)+'</script>')
    (OUT/"index.html").write_text(html.replace("NAVIGATION",navigation).replace("PANELS","\n".join(panels)))


if __name__=="__main__":
    EXAMPLES=build_data(OUT)
    relations();smoothness();summation();covers();extensions();costs()
    add_figures(globals(),EXAMPLES)
    write_sources();viewer()
    print("Built ten research diagrams, offline viewer, verified worked examples and LaTeX formula sources.")
