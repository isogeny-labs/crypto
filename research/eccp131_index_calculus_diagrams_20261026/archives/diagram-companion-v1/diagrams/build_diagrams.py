"""Build editable SVG teaching diagrams and an offline interactive viewer."""

from html import escape
from pathlib import Path
import json
import math
import re


OUT = Path(__file__).resolve().parent
W, H = 1440, 940
INK = "#192b3b"
MUTED = "#586879"
BG = "#f7f6f2"
BLUE = "#2864ad"
BLUE_LIGHT = "#edf3fc"
GREEN = "#087d68"
GREEN_LIGHT = "#eaf6f1"
PURPLE = "#7555a5"
PURPLE_LIGHT = "#f1edf8"
AMBER = "#a66a13"
AMBER_LIGHT = "#fff4dd"
LINE = "#d9dfe5"


class Drawing:
    def __init__(self, number, title, subtitle):
        self.number = number
        self.parts = [f'''<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}" role="img" aria-labelledby="title{number} desc{number}">
<title id="title{number}">{escape(title)}</title><desc id="desc{number}">{escape(subtitle)}</desc>
<defs>''']
        for name, color in [("ink",INK),("blue",BLUE),("green",GREEN),("purple",PURPLE),("muted",MUTED)]:
            self.parts.append(f'<marker id="a{number}{name}" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="8" markerHeight="8" orient="auto-start-reverse"><path d="M0 0 L10 5 L0 10 z" fill="{color}"/></marker>')
        self.parts.append('''</defs><style>
text{font-family:DejaVu Sans,Arial,sans-serif;fill:#192b3b} .math{font-family:DejaVu Sans,Arial,sans-serif;font-weight:500}
</style>''')
        self.rect(0,0,W,H,BG,radius=0,stroke="none")
        self.text(56,48,"ELLIPTIC CURVES · JACOBIANS · INDEX CALCULUS",14,MUTED,weight=600,spacing=1.7)
        self.text(56,101,title,36,weight=700)
        self.text(56,140,subtitle,20,MUTED)
        self.text(1382,48,f"{number:02d} / 04",15,MUTED,anchor="end")

    def rect(self,x,y,w,h,fill="white",stroke=LINE,radius=18,extra=""):
        self.parts.append(f'<rect x="{x}" y="{y}" width="{w}" height="{h}" rx="{radius}" fill="{fill}" stroke="{stroke}" stroke-width="1.5" {extra}/>')

    def text(self,x,y,value,size=20,color=INK,weight=400,anchor="start",spacing=None,extra=""):
        style = f'letter-spacing="{spacing}"' if spacing is not None else ""
        self.parts.append(f'<text x="{x}" y="{y}" font-size="{size}" font-weight="{weight}" text-anchor="{anchor}" style="fill:{color}" {style} {extra}>{escape(str(value))}</text>')

    def lines(self,x,y,values,size=20,color=INK,gap=30,weight=400,anchor="start"):
        for i,line in enumerate(values):
            self.text(x,y+i*gap,line,size,color,weight,anchor)

    def arrow(self,x1,y1,x2,y2,color="ink",dashed=False):
        palette={"ink":INK,"blue":BLUE,"green":GREEN,"purple":PURPLE,"muted":MUTED}
        dash='stroke-dasharray="7 6"' if dashed else ""
        self.parts.append(f'<path d="M{x1} {y1} L{x2} {y2}" fill="none" stroke="{palette[color]}" stroke-width="2.5" marker-end="url(#a{self.number}{color})" {dash}/>')

    def path(self,data,color=INK,width=2,dashed=False):
        dash='stroke-dasharray="7 6"' if dashed else ""
        self.parts.append(f'<path d="{data}" fill="none" stroke="{color}" stroke-width="{width}" {dash}/>')

    def circle(self,x,y,r,fill,stroke="none"):
        self.parts.append(f'<circle cx="{x}" cy="{y}" r="{r}" fill="{fill}" stroke="{stroke}"/>')

    def footer(self,text):
        self.path("M56 886 L1384 886",LINE,1)
        self.text(56,918,text,15,MUTED)

    def save(self,name):
        data="\n".join(self.parts)+"\n</svg>"
        (OUT/name).write_text(data)
        return data


def workflow():
    d=Drawing(1,"One goal, two ways to find relations","Both methods collect equations, then recover the discrete log with sparse linear algebra.")
    d.rect(56,174,1328,94,"white")
    d.text(84,211,"THE TARGET",14,MUTED,600,spacing=1.4)
    d.text(84,247,"Q = kP",29,weight=600)
    d.text(337,211,"Unknown: k modulo the subgroup order n",22)
    d.text(337,246,"P and Q are points or divisor classes, depending on the group.",19,MUTED)

    headers=[(294,"1  CHOOSE A FACTOR BASE"),(710,"2  COLLECT RELATIONS"),(1151,"3  SAVE EQUATIONS")]
    for x,label in headers:
        d.text(x,309,label,14,MUTED,600,spacing=.7)
    for y,color,light,name,group,fb,engine,out in [
        (334,BLUE,BLUE_LIGHT,"ELLIPTIC","E(𝔽q)",["Selected points Fⱼ","with restricted coordinates"],["Solve for a point decomposition","using summation polynomials"],["R = Σ eⱼFⱼ","Check lifts and signs"]),
        (545,GREEN,GREEN_LIGHT,"HYPERELLIPTIC","Jac(C)(𝔽q)",["Selected prime-divisor classes","often from rational points"],["Factor a divisor’s polynomial","and test factor-base membership"],["D = Σ eⱼFⱼ","Merge partial rows"]),
    ]:
        d.rect(56,y,205,164,light,stroke=color)
        d.text(78,y+39,name,17,color,700)
        d.text(78,y+82,group,23,color,500)
        d.text(78,y+122,"group elements",15,MUTED)
        d.rect(286,y,350,164,"white")
        d.lines(310,y+54,fb,20,gap=35)
        d.rect(688,y,410,164,light,stroke=color)
        d.lines(712,y+54,engine,20,gap=35)
        d.rect(1148,y,236,164,"white")
        d.lines(1170,y+54,out,18,gap=35)
        d.arrow(645,y+82,677,y+82,"muted")
        d.arrow(1107,y+82,1137,y+82,"muted")
    d.path("M1267 506 L1267 528 L1399 528 L1399 753 L1230 753",BLUE,2)
    d.arrow(1230,753,1200,753,"blue")
    d.path("M1267 717 L1267 753 L1230 753",GREEN,2)
    d.rect(286,744,914,114,"white",stroke=INK)
    d.text(314,781,"SAME LINEAR-ALGEBRA STAGE",14,MUTED,600,spacing=1)
    d.text(314,826,"aᵢ + bᵢk = Σ eᵢⱼℓⱼ   (mod n)",26,weight=500)
    d.arrow(860,812,932,812)
    d.text(958,824,"recover k",26,weight=600)
    d.text(56,791,"Write R = aP + bQ.",17,MUTED)
    d.text(56,822,"ℓⱼ: log of Fⱼ to base P.",17,MUTED)
    d.footer("The factor base is chosen in, or projected to, the target prime-order component.  Σ means a group sum.")
    return d.save("01-workflow.svg")


def smoothness():
    d=Drawing(2,"Testing smoothness and solving decompositions","The relation-finding step is where the elliptic and hyperelliptic methods differ most.")
    d.rect(56,180,640,616,"white",stroke=GREEN)
    d.rect(744,180,640,616,"white",stroke=BLUE)
    d.text(84,222,"HYPERELLIPTIC",16,GREEN,700,spacing=1)
    d.text(84,259,"Test a candidate divisor",27,weight=650)
    d.text(772,222,"ELLIPTIC",16,BLUE,700,spacing=1)
    d.text(772,259,"Search for a decomposition",27,weight=650)
    d.rect(84,286,584,90,GREEN_LIGHT,stroke="none")
    d.text(108,323,"D is represented by (u(X), v(X))",23)
    d.text(108,355,"Typically deg u ≤ g; g is the genus.",18,MUTED)
    d.arrow(376,386,376,414,"green")
    d.text(376,447,"u(X) = (X − r₁)(X − r₂)",25,GREEN,500,anchor="middle")
    d.text(376,479,"Example: a divisor with two rational constituents",17,MUTED,anchor="middle")
    d.arrow(288,493,222,531,"green")
    d.arrow(464,493,530,531,"green")
    for x,label,point in [(108,"D₁","(r₁, v(r₁))"),(404,"D₂","(r₂, v(r₂))")]:
        d.rect(x,543,242,98,GREEN_LIGHT,stroke="none")
        d.text(x+121,577,label,25,GREEN,600,anchor="middle")
        d.text(x+121,611,point,21,anchor="middle")
    d.text(376,683,"[D] = D₁ + D₂",28,GREEN,500,anchor="middle")
    d.text(84,727,"SMOOTH WHEN",14,MUTED,600,spacing=.8)
    d.lines(84,758,["Every constituent class belongs to the chosen factor base."],17)

    d.rect(772,286,584,90,BLUE_LIGHT,stroke="none")
    d.text(796,323,"Seek R = ε₁P₁ + ··· + εₘPₘ",25)
    d.text(796,355,"Each x(Pᵢ) must lie in a selected set V.",18,MUTED)
    d.arrow(1064,386,1064,414,"blue")
    for x,label in [(851,"x₁ ?"),(1000,"x₂ ?"),(1250,"xₘ ?")]:
        d.rect(x-47,429,94,68,BLUE_LIGHT,stroke=BLUE,radius=12,extra='stroke-dasharray="5 4"')
        d.text(x,471,label,23,BLUE,500,anchor="middle")
    d.text(1124,470,"···",28,MUTED)
    d.arrow(1064,508,1064,536,"blue")
    d.text(1064,573,"Sₘ₊₁(x₁, …, xₘ, x(R)) = 0",25,BLUE,500,anchor="middle")
    d.text(1064,611,"Solve the polynomial system with xᵢ ∈ V.",19,MUTED,anchor="middle")
    d.arrow(1064,629,1064,656,"blue")
    d.lines(1064,690,["Lift x-coordinates to points,", "then verify the signed group sum."],20,gap=32,anchor="middle")
    d.text(772,766,"Writing S is easier than solving its constrained roots.",17,MUTED)

    d.rect(56,816,1328,51,AMBER_LIGHT,stroke="none",radius=12)
    d.text(720,849,"Fast smoothness testing still needs enough relations over a sufficiently small factor base.",20,AMBER,500,anchor="middle")
    d.footer("Divisor classes use an infinity/base-divisor convention.  The displayed split assumes valid points and factor-base membership.")
    return d.save("02-smoothness.svg")


def cover():
    d=Drawing(3,"A cover changes the ambient group","The curve map and its induced pullback point in opposite directions.")
    d.text(56,193,"CURVES",14,MUTED,600,spacing=1.4)
    d.rect(56,216,595,160,PURPLE_LIGHT,stroke=PURPLE)
    d.text(82,254,"Genus-2 hyperelliptic curve C",23,PURPLE,600)
    d.text(82,302,"Y² = X⁶ + aX² + b",30,weight=500)
    d.text(82,343,"Same field 𝔽p as the challenge curve",18,MUTED)
    d.rect(925,216,459,160,BLUE_LIGHT,stroke=BLUE)
    d.text(951,254,"Elliptic curve E",23,BLUE,600)
    d.text(951,302,"y² = x³ + ax + b",28,weight=500)
    d.text(951,343,"The published ECCp-131 curve",18,MUTED)
    d.arrow(669,284,905,284,"purple")
    d.text(787,252,"degree-2 cover f",19,PURPLE,500,anchor="middle")
    d.text(787,327,"(X,Y) ↦ (X²,Y)",20,PURPLE,500,anchor="middle")

    d.text(56,426,"GROUPS OF DIVISOR CLASSES",14,MUTED,600,spacing=1.4)
    d.rect(56,450,739,299,PURPLE_LIGHT,stroke=PURPLE)
    d.text(84,490,"Jac(C)(𝔽p)",26,PURPLE,600)
    d.text(84,525,"Ambient group size ≈ p²",20,MUTED)
    d.rect(116,552,608,155,BLUE_LIGHT,stroke=BLUE,radius=22)
    d.text(420,592,"Embedded elliptic subgroup",23,BLUE,600,anchor="middle")
    d.text(420,635,"f*(Q) = k f*(P)",29,weight=500,anchor="middle")
    d.text(420,674,"Subgroup order remains n ≈ p",21,BLUE,500,anchor="middle")
    d.rect(1000,493,384,214,BLUE_LIGHT,stroke=BLUE)
    d.text(1028,537,"E(𝔽p)",27,BLUE,600)
    d.text(1028,583,"Q = kP",30,weight=500)
    d.text(1028,628,"Challenge subgroup order n",19,MUTED)
    d.text(1028,666,"For ECCp-131: #E(𝔽p) = n",17,MUTED)
    d.arrow(982,615,743,615,"blue")
    d.text(866,571,"pullback f*",22,BLUE,600,anchor="middle")

    d.rect(56,775,1328,91,"white",stroke=INK)
    d.text(84,811,"Pushforward after pullback gives multiplication by 2.",23)
    d.text(84,846,"Since n is odd, pullback is injective on the target subgroup. The same unknown k survives.",20,MUTED)
    d.footer("Schematic group regions, not a plot of finite-field points.  A larger ambient group alone supplies no speedup.")
    return d.save("03-cover.svg")


def costs():
    parameters=json.loads((OUT.parent/"parameters.json").read_text())
    qbits=math.log2(int(parameters["p"],16))
    d=Drawing(4,"Compare field size and group size separately","Asymptotic work exponents for representative algorithms; constants and logarithmic factors are omitted.")
    d.rect(56,175,1328,76,"white")
    d.text(82,207,"COMPARISON",13,MUTED,600,spacing=1)
    d.text(82,237,"Same field: log₂ q = 130.19",23,weight=600,extra='id="comparisonLabel"')
    d.circle(962,215,7,BLUE)
    d.text(980,222,"Pollard rho",18,MUTED)
    d.circle(1162,215,7,GREEN)
    d.text(1180,222,"Index calculus",18,MUTED)
    left,right=440,1328
    maxexp=260
    scale=(right-left)/maxexp
    for tick in range(0,251,50):
        x=left+tick*scale
        d.path(f"M{x} 278 L{x} 741",LINE,1)
        d.text(x,775,str(tick),16,MUTED,anchor="middle")
    labels=[("Elliptic curve",1,None),("Genus-2 hyperelliptic",2,1),("Genus-3 hyperelliptic",3,4/3),("Genus-3 plane quartic",3,1)]
    for i,(label,g,exponent) in enumerate(labels):
        y=305+i*116
        rho=g*qbits/2
        d.text(80,y+16,label,22,weight=600)
        d.text(80,y+46,f"Generic subgroup order ≈ q{['','', '²','³'][g] if g>1 else ''}",17,MUTED,extra=f'id="orderLabel{i}"')
        d.rect(left,y,round(rho*scale,3),18,BLUE,stroke="none",radius=5,extra=f'id="rhoBar{i}"')
        d.text(left+rho*scale+12,y+16,f"2^{rho:.1f}",17,BLUE,extra=f'id="rhoText{i}"')
        if exponent is None:
            d.text(left,y+55,"No general prime-field IC speedup established",18,MUTED)
        else:
            ic=exponent*qbits
            d.rect(left,y+35,round(ic*scale,3),18,GREEN,stroke="none",radius=5,extra=f'id="icBar{i}"')
            d.text(left+ic*scale+12,y+51,f"2^{ic:.1f}",17,GREEN,extra=f'id="icText{i}"')
    d.text(885,814,"Work exponent e, where cost is approximately 2ᵉ",20,MUTED,anchor="middle")
    d.rect(56,834,1328,39,AMBER_LIGHT,stroke="none",radius=10)
    d.text(720,861,"These rows assume large generic subgroups. A cover of ECCp-131 retains its original order-n subgroup.",17,AMBER,500,anchor="middle")
    d.footer("Fixed small genus and standard algorithmic hypotheses. Field/group operations and constants differ; this is not a timing prediction.")
    return d.save("04-costs.svg")


def viewer():
    source=(OUT/"04-costs.svg").read_text()
    page='''<!doctype html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Index calculus — four diagrams</title>
<style>
:root{color-scheme:light;--ink:#192b3b;--muted:#586879;--paper:#f7f6f2;--blue:#2864ad}*{box-sizing:border-box}body{margin:0;background:#e8ebee;color:var(--ink);font-family:system-ui,Arial,sans-serif}header{max-width:1440px;margin:auto;padding:28px 24px 18px}h1{font-size:28px;margin:0 0 9px}header p{color:var(--muted);margin:8px 0;line-height:1.5}nav{display:flex;flex-wrap:wrap;gap:9px;margin:20px 0}a,button{color:var(--blue)}nav a,.download{padding:9px 14px;border:1px solid #c9d2db;border-radius:9px;text-decoration:none;background:white}.paper{max-width:1440px;margin:0 auto 28px;box-shadow:0 8px 36px #192b3b14;background:var(--paper);scroll-margin-top:14px}.paper img,.paper svg{display:block;width:100%;height:auto}.controls{max-width:1440px;margin:0 auto 12px;padding:20px 24px;background:white;border:1px solid #d9dfe5;border-radius:12px;display:flex;flex-wrap:wrap;gap:24px;align-items:center}.controls label{font-size:15px;font-weight:600;display:flex;gap:10px;align-items:center}.controls select{font:inherit;padding:8px;border-radius:7px;border:1px solid #c9d2db}.controls input{width:220px}.controls p{width:100%;margin:0;font-size:14px;color:var(--muted)}.sources{max-width:1440px;margin:0 auto 32px;padding:24px;background:white;border-radius:12px;line-height:1.65}.sources p{margin:4px 0}.downloads{max-width:1440px;margin:0 auto 30px;display:flex;flex-wrap:wrap;gap:12px}.download{font-size:14px}button{font:inherit;cursor:pointer}.no-print{display:block}@page{size:14.4in 9.4in;margin:0}@media print{body{background:white}header,.controls,.sources,.downloads{display:none!important}.paper{margin:0;padding:0;max-width:none;width:14.4in;height:9.4in;box-shadow:none;break-after:page}.paper:last-of-type{break-after:auto}.paper img,.paper svg{width:14.4in;height:9.4in}}
</style></head><body><header><h1>Index calculus: four editable diagrams</h1><p>A first teaching set for a technical audience new to index calculus. Each figure is also available as an editable SVG and a PNG.</p><nav><a href="#workflow">1 · Shared workflow</a><a href="#smoothness">2 · Relation finding</a><a href="#cover">3 · Covers and subgroups</a><a href="#costs">4 · Cost comparison</a></nav></header>
<section class="paper" id="workflow"><img src="01-workflow.svg" alt="Two index-calculus workflows converge on sparse linear algebra."></section>
<section class="paper" id="smoothness"><img src="02-smoothness.svg" alt="Factoring a divisor polynomial versus solving a constrained summation-polynomial system."></section>
<section class="paper" id="cover"><img src="03-cover.svg" alt="A degree-two curve cover induces an injective pullback into an order-n subgroup of the Jacobian."></section>
<div class="controls"><label>Compare<select id="mode"><option value="field">Same field size</option><option value="group">Same group size</option></select></label><label><span id="sliderName">Field-size bits</span><input id="bits" type="range" min="48" max="160" step="0.1" value="130.2"><output id="bitValue">130.2</output></label><button id="saveCost">Download current cost diagram (SVG)</button><p id="modeExplanation">Fix q. A genus-g Jacobian has order approximately qᵍ, so the compared groups have different sizes.</p></div>
<section class="paper" id="costs">COST_SVG</section>
<div class="downloads"><button class="download" onclick="window.print()">Print / save all four as PDF</button><button class="download" data-svg="workflow" data-name="01-workflow.svg">Workflow SVG</button><button class="download" data-svg="smoothness" data-name="02-smoothness.svg">Relation-finding SVG</button><button class="download" data-svg="cover" data-name="03-cover.svg">Cover SVG</button><button class="download" data-svg="costs" data-name="04-costs-current.svg">Cost SVG (current)</button></div>
<div class="sources"><strong>Scope and references</strong><p>All visual regions are schematic. Curves, groups, divisor classes, and subgroup orders are distinct objects. The cover example uses ECCp-131 over its original prime field, not the binary ECC2K-130 curve.</p><p>The hyperelliptic split illustrates valid rational constituent points and the appropriate infinity/base-divisor convention. A small factor base can still make usable relations scarce.</p><p>Cost exponents are representative fixed-small-genus asymptotic bounds under their standard hypotheses. They omit logarithmic factors and constants and do not predict execution time or prove lower bounds.</p><p><a href="https://eprint.iacr.org/2004/031">Semaev: summation polynomials</a> · <a href="https://arxiv.org/abs/math/0606607">Diem: small-genus index calculus</a> · <a href="https://jtnb.centre-mersenne.org/articles/10.5802/jtnb.815/">Diem: plane-quartic index calculus</a> · <a href="https://www.certicom.com/en/curves-list">Certicom parameters</a></p></div>
<script>
const mode=document.getElementById('mode'), bits=document.getElementById('bits');
function update(){const b=Number(bits.value),sameGroup=mode.value==='group',scale=888/260;document.getElementById('bitValue').textContent=b.toFixed(1);document.getElementById('sliderName').textContent=sameGroup?'Group-size bits':'Field-size bits';document.getElementById('modeExplanation').textContent=sameGroup?'Fix a generic subgroup order N. Its field size is approximately N^(1/g); these are independently parameterized groups, not covers of one challenge.':'Fix q. A genus-g Jacobian has order approximately qᵍ, so the compared groups have different sizes.';document.getElementById('comparisonLabel').textContent=sameGroup?'Same group: log₂ N = '+b.toFixed(1):'Same field: log₂ q = '+b.toFixed(1);const rows=[{g:1,ic:null},{g:2,ic:1},{g:3,ic:4/3},{g:3,ic:1}];rows.forEach((r,i)=>{const qbits=sameGroup?b/r.g:b,rho=r.g*qbits/2,ic=r.ic*qbits;document.getElementById('rhoBar'+i).setAttribute('width',rho*scale);document.getElementById('rhoText'+i).setAttribute('x',440+rho*scale+12);document.getElementById('rhoText'+i).textContent='2^'+rho.toFixed(1);document.getElementById('orderLabel'+i).textContent=sameGroup?'log₂ q ≈ '+qbits.toFixed(1)+'; group order ≈ N':'Generic subgroup order ≈ '+(r.g===1?'q':'q'+(r.g===2?'²':'³'));if(r.ic!==null){document.getElementById('icBar'+i).setAttribute('width',ic*scale);document.getElementById('icText'+i).setAttribute('x',440+ic*scale+12);document.getElementById('icText'+i).textContent='2^'+ic.toFixed(1)}})}
function saveSVG(section,name){const svg=document.querySelector('#'+section+' svg');const blob=new Blob([new XMLSerializer().serializeToString(svg)],{type:'image/svg+xml'});const url=URL.createObjectURL(blob),a=document.createElement('a');a.href=url;a.download=name;a.click();setTimeout(()=>URL.revokeObjectURL(url),1000)}
mode.addEventListener('change',update);bits.addEventListener('input',update);document.getElementById('saveCost').addEventListener('click',()=>saveSVG('costs','costs-'+mode.value+'-'+bits.value+'bits.svg'));document.querySelectorAll('[data-svg]').forEach(button=>button.addEventListener('click',()=>saveSVG(button.dataset.svg,button.dataset.name)));update();
</script></body></html>'''
    page=page.replace("COST_SVG",source)
    # Inline every figure: opening the HTML requires no neighboring files or CDN.
    page=re.sub(r'<img src="([^"]+)"[^>]*>',lambda match:(OUT/match.group(1)).read_text(),page)
    (OUT/"index.html").write_text(page)


if __name__=="__main__":
    workflow();smoothness();cover();costs();viewer()
    print("Wrote four SVGs and index.html in "+str(OUT))
