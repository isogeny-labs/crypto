from pathlib import Path
import json,html,hashlib
import fitz
p=Path(__file__).resolve().parent;x=json.loads((p/'review.json').read_text())
s=['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="490"><rect width="1000" height="490" fill="white"/><g font-family="sans-serif" fill="#17304b"><text x="25" y="35" font-size="24">Recorded fixed-case qualification: 61 / 61 passed</text>']
for i,g in enumerate(x['groups']):
 y=70+i*50;s += [f'<text x="25" y="{y+20}" font-size="16">{html.escape(g["name"])}</text>',f'<rect x="390" y="{y}" width="{g["passed"]*24}" height="28" fill="#427c91"/>',f'<text x="{400+g["passed"]*24}" y="{y+20}" font-size="16">{g["passed"]}/{g["expected"]}</text>']
s+=['<text x="25" y="460" font-size="15">Bar length: passed fixed cases (count). Deterministic fixtures; no statistical uncertainty claim.</text></g></svg>'];(p/'case-coverage.svg').write_text('\n'.join(s))
d=fitz.open();page=d.new_page(width=595,height=842);page.insert_text((30,35),'Python 3.9 lease timestamp qualification',fontsize=18)
a=fitz.open(stream=(p/'case-coverage.svg').read_bytes(),filetype='svg');b=fitz.open('pdf',a.convert_to_pdf());page.show_pdf_page(fitz.Rect(20,55,575,340),b,0)
t=(p/'REPORT.md').read_text().replace('# Python 3.9 lease timestamp qualification\n\n','')+'\nSource SHA256:\n'+x['source_sha256']+'\nHarness SHA256:\n'+x['harness_sha256']+'\nPublished revision:\n'+x['publication']+'\n\nRaw receipts: result.json, terminal.json, stdout, stderr.\nPer-case outcomes: cases.csv. Bound source and method: review.json.\nThis report archives an existing run; no test was rerun.'
assert page.insert_textbox(fitz.Rect(30,365,565,800),t,fontsize=10,lineheight=1.25)>=0
d.save(p/'report.pdf',deflate=True);page.get_pixmap().save('/var/tmp/lease-time-qualification-report.png')
