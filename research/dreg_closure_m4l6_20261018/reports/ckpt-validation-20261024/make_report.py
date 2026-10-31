"""README.md -> report.typ -> report.pdf (Typst via the `typst` Python wheel)."""
import re, typst, pathlib
here = pathlib.Path(__file__).parent
lines = (here / "README.md").read_text().split("\n")
out = ['#set page(margin: 1.8cm)', '#set text(size: 10pt)', '#set par(justify: true)',
       '// one body font for code spans too: fewer embedded fonts, a smaller PDF', '#show raw: set text(font: "Libertinus Serif", size: 9pt)', '']
def inline(t):
    t = re.sub(r'`([^`]*)`', lambda m: '#raw("' + m.group(1).replace('"', '\\"') + '")', t)
    t = re.sub(r'\*\*([^*]+)\*\*', r'*\1*', t)
    return t.replace('@', '\\@').replace('$', '\\$')
i = 0
while i < len(lines):
    l = lines[i]
    if l.startswith('# '): out.append('= ' + inline(l[2:]))
    elif l.startswith('## '): out.append('== ' + inline(l[3:]))
    elif l.startswith('!['):
        out.append('#figure(image("ckpt-validation.svg", width: 100%), caption: [Stop, kill, resume: wall seconds, and the cost of one save. Data in `data/`, source `plot.py`.])')
    elif l.startswith('| '):
        tbl = []
        while i < len(lines) and lines[i].startswith('|'):
            if not set(lines[i]) <= set('|- '):
                tbl.append([c.strip() for c in lines[i].strip().strip('|').split('|')])
            i += 1
        n = len(tbl[0])
        out.append('#table(columns: (2.6fr,' + ','.join(['1fr'] * (n - 1)) + '), inset: 4pt, stroke: 0.4pt,')
        out += [' ' + ', '.join('[' + inline(c) + ']' for c in r) + ',' for r in tbl]
        out.append(')')
        continue
    elif l.startswith('- '):
        j, buf = i + 1, [l[2:]]
        while j < len(lines) and lines[j].startswith('  '):
            buf.append(lines[j].strip()); j += 1
        out.append('- ' + inline(' '.join(buf))); i = j
        continue
    else: out.append(inline(l))
    i += 1
(here / "report.typ").write_text('\n'.join(out))
typst.compile(str(here / "report.typ"), output=str(here / "report.pdf"))
