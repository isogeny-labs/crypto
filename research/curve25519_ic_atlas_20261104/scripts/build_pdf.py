import pathlib, subprocess, typst
D = pathlib.Path(__file__).resolve().parents[1]
md = (D / "REPORT.md").read_text()
def fig(name, cap):
    return f'\n```{{=typst}}\n#figure(image("figures/{name}.svg", width: 100%), caption: [{cap}])\n```\n\n'
f1 = fig("ledger_k20", "Mean S at w = 0, 0.1, 1 for the section 6 ledger arms, T12 session (log scale). Source: band_pooled.json.") + \
     fig("algebraic_progress", "Best algebraic arm against rho-neg at w = 0.1 over time; target sets differ between points. Source: section 6.")
f3 = fig("leads_map", "Lead status grid from section 9.")
md = md.replace("\n## 7. The grid limit", f1 + "\n## 7. The grid limit", 1)
md = md.replace("Ideas considered and not pursued", f3 + "Ideas considered and not pursued", 1)
tmp = D / "scripts" / "_report_figs.md"; tmp.write_text(md)
body = subprocess.run(["pandoc", "-f", "markdown", "-t", "typst", str(tmp)], capture_output=True, text=True, check=True).stdout
tmp.unlink()
pre = '''#set page(paper: "a4", margin: 1.8cm, numbering: "1", footer: context [#text(7pt)[Source of truth: research/curve25519_ic_atlas_20261104/REPORT.md; figures in figures/ (scripts/make_figures.py).] #h(1fr) #counter(page).display()])
#set text(size: 9.5pt)
#show raw: set text(size: 8pt)
#set table(inset: 4pt, stroke: 0.4pt + gray)
#show table: set text(size: 8pt)
#show heading.where(level: 1): set text(size: 15pt)
#set figure(placement: none)
'''
(D / "REPORT.typ").write_text(pre + body)
typst.compile(str(D / "REPORT.typ"), output=str(D / "REPORT.pdf"))
