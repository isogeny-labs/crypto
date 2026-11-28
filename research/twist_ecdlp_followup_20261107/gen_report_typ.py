"""Build report.typ from README.md (same approach as ../twist_ecdlp/gen_report_typ.py), then compile with the typst python module."""
import re, sys
D = "research/twist_ecdlp_followup_20261107"
SPECIAL = '\\#$@*_<>~`[]'
def esc(s):
    return ''.join('\\' + c if c in SPECIAL else c for c in s)
def inline(s):
    out = []
    for part in re.split(r'(\*\*.+?\*\*|`.+?`)', s):
        if part.startswith('**') and part.endswith('**') and len(part) > 4:
            out.append('*' + esc(part[2:-2]) + '*')
        elif part.startswith('`') and part.endswith('`') and len(part) > 2:
            t = esc(part[1:-1]).replace('/', '/​')
            out.append('#text(font: "DejaVu Sans Mono", size: 7.5pt)[' + t + ']')
        else:
            out.append(esc(part))
    return ''.join(out)
src = open(D + "/README.md").read().split("\n")
out = ['#set document(title: "Twist-ECDLP follow-up study: L1, L2, L3 and re-replication")',
       '#set page(paper: "a4", margin: 1.6cm, numbering: "1")',
       '#set text(size: 9pt)', '#set par(justify: false, leading: 0.55em)',
       '#show heading.where(level: 2): set text(size: 11.5pt)']
table = []
def flush():
    global table
    if table:
        n = len(table[0])
        cols = '(auto, ' + ', '.join(['1fr'] * (n - 1)) + ')'
        out.append(f'#table(columns: {cols}, inset: 3.5pt, stroke: 0.4pt + gray,')
        out.append(' table.header(' + ', '.join('[*' + inline(c.strip()) + '*]' for c in table[0]) + '),')
        for r in table[1:]:
            out.append(' ' + ', '.join('[' + inline(c.strip()) + ']' for c in r) + ',')
        out.append(')'); out.append('')
        table = []
for ln in src:
    if ln.startswith('|'):
        cells = ln.strip().strip('|').split('|')
        if set(cells[0].strip()) <= set('-: '): continue
        table.append(cells); continue
    flush()
    if ln.startswith('# '):
        out.append('= ' + inline(ln[2:])); out.append('')
    elif ln.startswith('## '):
        out.append('== ' + inline(ln[3:])); out.append('')
    elif ln.startswith('!['):
        out.append('#figure(image("summary.svg", width: 100%), caption: [IC cost over matched-rho cost across the GAE band (w = 0, 0.1, 1), log scale, 1 = tie; recorded values only.])'); out.append('')
    elif ln.startswith('- '):
        out.append('- ' + inline(ln[2:]))
    elif re.match(r'^\d+\. ', ln):
        out.append('+ ' + inline(re.sub(r'^\d+\. ', '', ln)))
    elif ln.strip() == '':
        out.append('')
    else:
        out.append(inline(ln)); out.append('')
flush()
open(D + "/report.typ", "w").write('\n'.join(out))
import typst
typst.compile(D + "/report.typ", output=D + "/report.pdf", root=D)
print("ok")
