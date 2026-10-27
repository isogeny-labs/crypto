"""Build report.pdf from RESULTS.md and the figures.

Renderer: Typst, via the `typst` Python package (pip install typst), with pandoc converting the
Markdown to Typst markup.  Falls back to matplotlib PdfPages (figures + plain-text pages) if the
Typst route fails, and says so on the first page.  The renderer actually used is written to
results/report_build.json.
"""
import json
from pathlib import Path
import re
import subprocess
import sys

HERE = Path(__file__).resolve().parent
PREAMBLE = r'''
#set page(paper: "a4", margin: (x: 1.4cm, y: 1.6cm), numbering: "1")
#set text(size: 8.6pt, lang: "en")
#set par(justify: false, leading: 0.55em)
#show heading.where(level: 1): set text(size: 15pt)
#show heading.where(level: 2): set text(size: 11.5pt)
#show raw: set text(size: 7.2pt)
#show table: set text(size: 6.2pt)
#set table(inset: 3pt, stroke: 0.4pt + luma(200))
#show figure: set block(breakable: true)
'''


def pandoc_to_typst(md_text):
    # pandoc's typst writer wants plain markdown; strip the HTML comments we use as table markers
    md_text = re.sub(r"<!--.*?-->", "", md_text, flags=re.S)
    proc = subprocess.run(["pandoc", "-f", "gfm", "-t", "typst", "--wrap=none"], input=md_text,
                          capture_output=True, text=True, check=True)
    return proc.stdout


def build_typst():
    import typst
    body = pandoc_to_typst((HERE / "RESULTS.md").read_text())
    (HERE / "report.typ").write_text(PREAMBLE + body)
    typst.compile(str(HERE / "report.typ"), output=str(HERE / "report.pdf"), root=str(HERE))
    return {"renderer": "typst", "typst_python_package": getattr(typst, "__version__", None),
            "pandoc": subprocess.run(["pandoc", "--version"], capture_output=True, text=True).stdout.splitlines()[0]}


def build_pdfpages(reason):
    import matplotlib
    matplotlib.use("Agg")
    import matplotlib.pyplot as plt
    from matplotlib.backends.backend_pdf import PdfPages
    import matplotlib.image as mpimg
    text = (HERE / "RESULTS.md").read_text().splitlines()
    with PdfPages(HERE / "report.pdf") as pdf:
        for start in range(0, len(text), 70):
            fig = plt.figure(figsize=(8.27, 11.69))
            fig.text(0.04, 0.97, f"[fallback renderer: matplotlib PdfPages; reason: {reason}]" if start == 0 else "",
                     fontsize=7, va="top")
            fig.text(0.04, 0.95, "\n".join(l[:150] for l in text[start:start + 70]), family="monospace",
                     fontsize=5.2, va="top")
            pdf.savefig(fig)
            plt.close(fig)
        for png in sorted((HERE / "figures").glob("*.png")):
            fig = plt.figure(figsize=(11.69, 8.27))
            plt.imshow(mpimg.imread(png))
            plt.axis("off")
            pdf.savefig(fig)
            plt.close(fig)
    return {"renderer": "matplotlib PdfPages (fallback)", "reason": reason}


if __name__ == "__main__":
    try:
        info = build_typst()
    except Exception as error:   # report the failure, then fall back
        info = build_pdfpages(f"{type(error).__name__}: {error}")
    (HERE / "results" / "report_build.json").write_text(json.dumps(info, indent=1) + "\n")
    print(json.dumps(info))
