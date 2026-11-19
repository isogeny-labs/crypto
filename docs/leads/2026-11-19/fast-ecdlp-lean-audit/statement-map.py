#!/usr/bin/env python3
"""Emit statement-map.svg: the Lean statement-fidelity audit chain for the
Kaarelson v2 decomposition oracle (Theorem 2(ii)). Layout only. See REPORT.md."""
import html
W, H = 960, 540
COL = {
    "ok":   ("#e3f3e8", "#2a9d55"),  # statement found faithful / standard
    "check":("#fbf1d8", "#caa43a"),  # item to verify
    "grey": ("#ededed", "#8a8a8a"),  # unrefereed / not run here
    "blue": ("#e3eefa", "#4b8fd1"),  # mechanism
}
# id, x, y, w, h, kind, title, sub(lines), dashed
boxes = [
 ("chal", 30, 70, 250, 92, "ok", "Challenge.lean : subexponentialRate m",
  "exists M : TM2ComputableInTime\n enc encOut (lexLeast m), finite Gamma,\n time <= C_k * 2^(l/k) for all k>=1\n= 2^o(l) = p^o(1). sorry (challenge).", False),
 ("fn", 30, 196, 250, 104, "check", "lexLeast m / strings m (the function)",
  "curve membership type-enforced;\n p prime, Delta != 0 gated; no vacuity.\n (2) factor base x < ceil(p^1/m), incl 0\n   (paper uses [1, floor(p^1/m)]).\n (3) genericity vs Definition 3.1: verify.", False),
 ("sol", 350, 70, 230, 70, "ok", "Solution.lean : subexponentialRate_all",
  "ECDLP.L104R.subexponentialRate_all\n (the proof term)", False),
 ("ver", 350, 182, 230, 82, "blue", "Verify.lean",
  "example : forall m, type_of%(subexp m)\n   := subexponentialRate_all\n #print axioms over the full set", False),
 ("cmp", 350, 300, 230, 70, "check", "Comparator (comparator.json)",
  "(4) real Challenge<->Solution identity\n guarantee; 2nd kernel 'nanoda'.\n Not run here.", False),
 ("eng", 640, 70, 290, 76, "grey", "Vendored engine + Mathlib",
  "lean/vendor/rudich-factoring/ (Rudich-\n Rousseau, unrefereed) + Mathlib v4.35.0.\n (d) independent review open.", True),
 ("ax", 640, 182, 290, 66, "grey", "#print axioms result",
  "[propext, Classical.choice, Quot.sound]\n (5) needs ~12 min / ~10 GB build to\n confirm; not run here.", True),
 ("bld", 640, 286, 290, 72, "grey", "lake build / lake env lean Verify.lean",
  "one sorry (challenge only), clean axioms.\n Build NOT run in this runtime.", True),
]
edges = [("chal","sol"),("chal","fn"),("sol","ver"),("ver","cmp"),
         ("sol","eng"),("ver","ax"),("cmp","bld")]
bi={b[0]:b for b in boxes}
def anc(i,side):
    _,x,y,w,h,*_=bi[i]
    return (x+w,y+h/2) if side=="r" else (x,y+h/2)
def esc(s): return html.escape(s)
P=[f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}" font-family="Helvetica,Arial,sans-serif">',
   '<defs><marker id="a" markerWidth="9" markerHeight="9" refX="7" refY="3" orient="auto"><path d="M0,0 L7,3 L0,6 Z" fill="#555"/></marker></defs>',
   f'<rect width="{W}" height="{H}" fill="white"/>',
   f'<text x="{W/2}" y="34" text-anchor="middle" font-size="16" font-weight="bold">Kaarelson v2 Lean statement-fidelity audit: the decomposition oracle (Theorem 2(ii))</text>',
   f'<text x="{W/2}" y="53" text-anchor="middle" font-size="11" fill="#444">statement-level audit of kaarelkaarelson/fast-ecdlp@d1deb5ab; build not run</text>']
for f,t in edges:
    x1,y1=anc(f,"r"); x2,y2=anc(t,"l"); mx=(x1+x2)/2
    P.append(f'<path d="M{x1:.0f},{y1:.0f} C{mx:.0f},{y1:.0f} {mx:.0f},{y2:.0f} {x2-8:.0f},{y2:.0f}" fill="none" stroke="#555" stroke-width="1.4" marker-end="url(#a)"/>')
for i,x,y,w,h,k,title,sub,dash in boxes:
    fill,stroke=COL[k]; d=' stroke-dasharray="5,3"' if dash else ''
    P.append(f'<rect x="{x}" y="{y}" width="{w}" height="{h}" rx="7" fill="{fill}" stroke="{stroke}" stroke-width="2"{d}/>')
    P.append(f'<text x="{x+9}" y="{y+18}" font-size="11" font-weight="bold" fill="#222">{esc(title)}</text>')
    for j,ln in enumerate(sub.split("\n")):
        P.append(f'<text x="{x+9}" y="{y+33+j*12.5:.0f}" font-size="9" fill="#444">{esc(ln)}</text>')
# legend + verdict
P.append('<rect x="30" y="396" width="900" height="118" rx="6" fill="#fbfbfb" stroke="#ccc"/>')
leg=[("ok","statement found faithful / standard"),("check","item to verify"),
     ("blue","audit mechanism"),("grey","unrefereed, or not run in this runtime")]
for n,(k,lab) in enumerate(leg):
    fill,stroke=COL[k]; yy=410+n*16
    P.append(f'<rect x="44" y="{yy}" width="12" height="11" rx="2" fill="{fill}" stroke="{stroke}" stroke-width="1.5"/>')
    P.append(f'<text x="62" y="{yy+10}" font-size="9.5" fill="#333">{esc(lab)}</text>')
P.append('<text x="360" y="420" font-size="10.5" font-weight="bold" fill="#1a3a5a">Verdict (statement level):</text>')
for n,ln in enumerate([
  "The oracle statement is a faithful, non-vacuous formalization of the paper's search:",
  "M is bound to compute lexLeast m; curve membership type-enforced; time is 2^o(l)=p^o(1).",
  "Open to close the audit: (2) factor-base set, (3) genericity = Def 3.1, (4) Comparator",
  "anti-circularity, (5) build for the sorry/axiom result, (d) the vendored engine."]):
    P.append(f'<text x="360" y="{438+n*16}" font-size="9.5" fill="#333">{esc(ln)}</text>')
P.append('</svg>')
open("statement-map.svg","w").write("\n".join(P))
print("wrote statement-map.svg")
