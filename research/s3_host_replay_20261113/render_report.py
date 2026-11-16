"""Render recorded hosted values and provenance; no native research execution."""
from pathlib import Path
import json,fitz,html
from reportlab.platypus import SimpleDocTemplate,Paragraph,Spacer
from reportlab.lib.styles import ParagraphStyle
from reportlab.lib import colors
from reportlab.graphics.shapes import Drawing,Rect,String
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
r=Path(__file__).resolve().parent;o=r/'reports/run-20261113-host-continuation';o.mkdir(parents=True,exist_ok=True)
e=json.loads((r/'export.json').read_text());h=json.loads((r/'evidence/hosted/rust/host.json').read_text())
rows={a:[json.loads(x)for x in(r/f'evidence/hosted/{a}/actual.jsonl').read_text().splitlines()]for a in['cpp','rust']}
pdfmetrics.registerFont(TTFont('DV','/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf'));pdfmetrics.registerFont(TTFont('DVB','/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf'));pdfmetrics.registerFontFamily('DV',normal='DV',bold='DVB')
b=ParagraphStyle('b',fontName='DV',fontSize=8.5,leading=11.4,spaceAfter=7);t=ParagraphStyle('t',fontName='DVB',fontSize=16,leading=19,spaceAfter=10)
def p(s):return Paragraph(s,b)
d=Drawing(500,174);svg=['<svg xmlns="http://www.w3.org/2000/svg" width="500" height="174" viewBox="0 0 500 174">']
def text(x,y,s,size=8):
 d.add(String(x,y,s,fontName='DV',fontSize=size));svg.append(f'<text x="{x}" y="{174-y}" font-family="DejaVu Sans,sans-serif" font-size="{size}">{html.escape(s)}</text>')
text(5,160,'Ordered triples per field; both separately compiled arms',9)
for j,arm in enumerate(['cpp','rust']):
 color=['#245b82','#d18b37'][j]
 for i,row in enumerate(rows[arm]):
  y=135-i*21-j*7;w=310*row['tuples']/2345778
  d.add(Rect(64,y,w,6,fillColor=colors.HexColor(color),strokeColor=None));svg.append(f'<rect x="64" y="{174-y-6}" width="{w}" height="6" fill="{color}"/>')
  if j==0:text(5,y-2,'q='+str(row['q']));text(386,y-2,format(row['tuples'],','))
text(64,3,'C++ (blue) / Rust (gold); linear axis: 0 → 2,345,778 triples',7.4)
svg.append('</svg>');(o/'triples.svg').write_text('\n'.join(svg)+'\n')
flow=[Paragraph('Separate-host S3 replay: both languages match',t),p('<b>Observed result.</b> First main-push run43501438026, job132147937956, attempt1 completed both frozen native calls. Each arm checks6prime fields,942nonsingular models and4,188,162ordered triples:244,236zero tuples,0false zeros and0missing zeros. Per-field aggregates exactly match each other and the previously seen known answer. [pcc:unverified]'),d,Spacer(1,8),p('<b>Figure.</b> Complete output records for q=5,7,11,13,17,19 from the public hosted job. Two fresh deterministic exhaustive calls; no sampling, RNG, confidence interval or timing claim. Source: evidence/hosted/{cpp,rust}/actual.jsonl. [pcc:unverified]'),p('<b>Flow and budget.</b> Planned2, attempted2, completed2, not_completed0, interrupted0, resumed0, excluded0. Standard public GitHub-hosted Ubuntu24.04runner, paid spend0, no agent artifact/cache upload, no outcome-based retries. The original manualHTTP404 remains a separate failed route; no historical execution rows imported. [pcc:unverified]'),p('<b>Frozen identity.</b> Protocol3bd9bdec28e8… froze2026-11-13T19:27:20Z before the20:25:34Zrun. PR4072merged at43875b707caedfb7b8175de0961e219a1152bbb3 after required checks; both arithmetic checkouts are pinnede3658b4d6ffc6839e32bad1f5f732f796a2c8e4d. C++ uses the original byte-identical arithmetic prefix plus a fixed S3-only entry, independently implemented Rust remains unchanged. All21source/component pins match. [pcc:unverified]'),p('<b>Actual host and binaries.</b> Azure hosted runner, AMD EPYC9V45, Linux7.0.0-1017-azure. C++g++13.3.0, binary612903b2b424…; Rust1.99.1, binary597621e63d47…. Complete hashes and compiler/build/run/comparison stdout/stderr/exits are retained. Both compile/run/compare exits0. Scientific clocks are invalid; native-only times, memory and isolated-core evidence are missing. [pcc:unverified]'),p('<b>Evidence checks.</b> The33784-byte timestamped public log is retained exactly as retrieved, losslessly gzip archived. All24marker parts are reconstructed with explicit timestamp/separator handling; their hashes identify reconstructed files, not remote-byte attestations. Canonical experiment.export/v1 validates with verify-files and0findings against the unchanged frozen protocol. Failed extraction/export preflights are retained; no native population was rerun. [pcc:unverified]'),p('<b>Formal and assurance boundary.</b> The existing11-declaration Lean quadratic-field/pair-oracle foundation and actual S3 equivalence remain linked separately. This result supplies actual separate-machine finite replication for item13. It does not prove the compilers/runtime/physical host, independence from the same author, binary characteristic, S4/S5, symbolic degrees or signed execution. Native evidence is an unsigned attestation; signed standing unverified. [pcc:unverified]'),p('<b>Sources and records.</b> Pinned original Semaev/repository finite-check scope, GitHub Actions event/runner documentation and public-runner billing review are in source-review.json. Canonical export.json, protocol.json, extractor and complete raw records accompany this report. No leaderboard or performance result changes. Other fourteen-item obligations remain in the audit map. [pcc:unverified]')]
SimpleDocTemplate(str(o/'report.pdf'),pagesize=(595.276,841.89),leftMargin=47,rightMargin=47,topMargin=37,bottomMargin=37).build(flow)
doc=fitz.open(o/'report.pdf');layout={'pages':[],'visual_inspection':'pending'}
for i,page in enumerate(doc):
 bad=[]
 for block in page.get_text('dict')['blocks']:
  for line in block.get('lines',[]):
   for span in line['spans']:
    x0,y0,x1,y1=span['bbox']
    if x0<0 or y0<0 or x1>page.rect.width or y1>page.rect.height:bad.append(span['text'])
 layout['pages'].append({'page':i+1,'text_within_page':not bad,'out_of_bounds':bad});page.get_pixmap(matrix=fitz.Matrix(1.25,1.25)).save('/workspace/scratch/s3-host-report-'+str(i+1)+'.png')
(o/'layout.json').write_text(json.dumps(layout,indent=2)+'\n');print(json.dumps({'pages':len(doc),'bytes':(o/'report.pdf').stat().st_size,'all_text_in_bounds':all(x['text_within_page']for x in layout['pages'])}))
