#!/usr/bin/env python3
"""Render recorded exact counts and claim boundaries; no research computation."""
from pathlib import Path
import json,html
base=Path(__file__).resolve().parent.parent

def text(x,y,s,size=16,color='#172b4d',weight='normal'):
 return f'<text x="{x}" y="{y}" font-size="{size}" fill="{color}" font-weight="{weight}">{html.escape(str(s))}</text>'
def frame(title):
 return ['<svg xmlns="http://www.w3.org/2000/svg" width="960" height="480" viewBox="0 0 960 480"><rect width="960" height="480" fill="#f7f9fc"/><g font-family="DejaVu Sans,sans-serif">',text(28,36,title,23,weight='bold')]
def finish(parts,path):path.write_text('\n'.join(parts+['</g></svg>'])+'\n')
r=json.loads((base/'27/results.json').read_text());s=frame('R27: complete section spaces and projective equations')
s+=[text(30,75,'Complete sections (unit: sections)',17),text(495,75,'Complete degree-two kernels (unit: equations)',17)]
for y,label,n in [(120,'Source C61',68),(180,'Quotient H31',34)]:
 s += [text(30,y,label),f'<rect x="175" y="{y-23}" width="{n*3.6}" height="30" rx="4" fill="#2476b5"/>',text(185+n*3.6,y,str(n),17,weight='bold')]
for y,label,n in [(120,'Source',2150),(180,'Quotient',497)]:
 s += [text(495,y,label),f'<rect x="610" y="{y-23}" width="{n*.12}" height="30" rx="4" fill="#218c74"/>',text(620+n*.12,y,str(n),17,weight='bold')]
s += [text(30,233,'All 20 charts reconstructed; every product kernel and pivot recombined.',16),text(30,265,'Exact RR membership proves an actual class of order 11, with T = 6.',16),text(30,303,'Written geometry: source P67 / quotient P33 are closed embeddings;',16),text(30,329,'their complete quadrics generate prime saturated ideals.',16),text(30,376,'Separate: old coupled saturation, determinant-19 theta equations,',16,'#a33b3b'),text(30,402,'colored reverse transfer and the actual F2 modular-factor map remain open.',16,'#a33b3b'),text(30,451,'Recorded deterministic counts: RR-export01, ACTION01, QUADRICS03. No cost/rank comparison.',13)]
finish(s,base/'27/round27.svg')
s=frame('R28: exact adjugate ratios select three ordinary images')
for y,d,m,outcome in [(110,436,1,'Selected; second degree 436'),(185,12,2,'Both first derivatives zero'),(260,284,1,'Selected; second degree 284'),(335,83,1,'Selected; second degree 83')]:
 c='#d49a28'if d==12 else'#2476b5';s += [text(30,y,f'{d} (multiplicity {m})'),f'<rect x="225" y="{y-25}" width="{max(14,d*.83)}" height="32" rx="4" fill="{c}"/>',text(620,y,outcome,16)]
s += [text(30,399,'Unit: closed first-image degree. Four exact derivative functions; all 1,024 coefficients checked.',15),text(30,430,'R29 resolves degree 12 by second-order norms. Colored C5 lifts remain separate.',15),text(30,459,'Run IDs: dual28-02 and eval28-01. These bars are support sizes, not runtimes or IC ranks.',13)]
finish(s,base/'28/round28.svg')
s=frame('R29: completed image multiplicities and base Frobenius cancellations')
for y,d,m,off,note in [(110,436,1,'218','Base closed divisor cancels'),(175,12,2,'6','One second point, multiplicity 2'),(240,284,1,'none','No tested base-deck match'),(305,83,1,'none','No tested base-deck match')]:
 c='#d49a28'if off!='none'else'#2476b5';s += [text(30,y,f'Degree {d} × {m}',17),text(260,y,'Frobenius offset: '+off,17,c,weight='bold'),text(620,y,note,16)]
s += [text(30,364,'Ordinary 460 base difference cancels; this does not determine the colored C5 deck offset.',15),text(30,396,'Separate exact check: irreducible degree-3 fiber over F2^14 gives order-77 data over F2^42.',15),text(30,428,'Global independent-model identities, full source lifts and modular-factor columns remain open.',15,'#a33b3b'),text(30,461,'Run IDs: jet29-01, eval29-01, dependencies29-01, cubic29-01. No IC independence claim.',13)]
finish(s,base/'29/round29.svg')
