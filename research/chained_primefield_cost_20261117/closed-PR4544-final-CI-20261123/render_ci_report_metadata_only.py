"""Render recorded CI metadata; no scientific execution or arithmetic."""
from pathlib import Path
import hashlib
import json
import os
import subprocess
os.environ.setdefault("MPLCONFIGDIR", "/dev/shm/ct23-matplotlib")
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import typst

HERE = Path(__file__).resolve().parent
OUT = HERE.parent / "reports/closed-PR4544-final-CI-20261123"
OUT.mkdir(parents=True, exist_ok=True)
DATA = json.loads((HERE / "RECORDED_CHECK_COUNTS.json").read_text())
fig, ax = plt.subplots(figsize=(7, 1.8))
for key, left, color in [("success", [0, 0], "#43836c"), ("failure", [DATA["protocolC"]["success"], DATA["final"]["success"]], "#b86452"), ("skipped", [20, 20], "#a1a8ae")]:
    values = [DATA[x][key] for x in ["protocolC", "final"]]
    bars = ax.barh([0, 1], values, left=left, color=color, label=key)
    ax.bar_label(bars, label_type="center", color="white", fontsize=10)
ax.set(yticks=[0, 1], yticklabels=["Protocol dd6223fa", "Final 437a6303"], xlabel="Recorded checks (all pages, 23 November observation)", xlim=(0, 30))
ax.legend(frameon=False, loc="lower center", bbox_to_anchor=(.5, 1.02), ncol=3, fontsize=8)
ax.spines[["top", "right"]].set_visible(False)
fig.tight_layout(pad=.4)
fig.savefig(OUT / "checks.svg")
plt.close(fig)
p = OUT / "checks.svg"
p.write_text("\n".join(x.rstrip() for x in p.read_text().splitlines()).rstrip() + "\n")
paragraphs = [
    "All-event/all-page metadata for old PR4544 records 30 checks on each head. ProtocolC has 21 workflows, 13 successful checks, 7 failed and 10 skipped. Final has 19 workflows, 11 successful checks, 9 failed and 10 skipped. Each observed head is within the 30-check limit. These counts do not erase the separately retained historical 56-check excess on an earlier head. [pcc:unverified]",
    "The final validation gate and artifact gate failed; size passed. The validation log records the committed-tree positive fixture rejection. The artifact job passes its changed-file data check, then reports 35 existing workflows with stale or missing pull steps. Other failed steps include shared-code lock, data hydration, S3 credential setup, format/admission, recompute and lessons evidence. The records make no new baseline-rerun or defect-attribution claim. [pcc:unverified]",
    "Thirteen selected complete decoded job logs were retrieved through the existing connector: protocol gate/artifacts/size, every final failed job and final size. The invalid curve-covers workflow has no jobs. Full decoded-text hashes and complete provider URLs are recorded; the local cache is temporary and a durable local mirror is not asserted. Raw API byte hashes are separate from decoded-log UTF8 hashes. No raw log was truncated, split or renamed into Git. [pcc:unverified]",
    "PR4544 merged at 2026-11-17T15:16:40Z as 173a86cb. The final jobs complete later; merge-time CI success is not established by this observation. Corrected episode E-8a1dcd84022b supersedes the initial guessed start with the actual PR-created 09:07:31Z. The old record remains. Revert check: 2026-12-01T15:16:40Z. Exact human-message event timestamps and durable raw-log mirroring remain open. [pcc:unverified]",
    "This is closed-PR historical custody only. No old branch push, new workflow dispatch, scientific replay or currentPR4990 CI query occurred. Current repository gates still apply. Claim PCC-20261123-29c03d is unsigned. [pcc:unverified]",
]
md = "# Closed PR4544: final CI custody\n\n" + "\n\n".join(paragraphs) + "\n\n![All recorded checks](checks.svg)\n"
md += "\n[Complete metadata and raw provider references](../../closed-PR4544-final-CI-20261123/ALL_EVENT_CI_METADATA_AND_RAW_REFERENCES.json)\n"
(OUT / "REPORT.md").write_text(md)
src = ['#set page(paper: "a4", margin: 16mm)', '#set text(font: "DejaVu Sans", size: 10pt)', '#set par(leading: .6em)', '= Closed PR4544: final CI custody', '#text(size: 9pt)[23 November 2026 · immutable old heads only · producer assertions unsigned]']
for index, text in enumerate(paragraphs):
    src.append("#text(" + json.dumps(text) + ")\n")
    if index == 0:
        src.append('#image("checks.svg", width: 100%)\n')
src.append('#link("https://github.com/isogeny-labs/crypto/pull/4544")[Closed PR4544, exact final head and discussion]')
(OUT / "report.typ").write_text("\n".join(src).rstrip() + "\n")
typst.compile(str(OUT / "report.typ"), output=str(OUT / "REPORT.pdf"), root=str(OUT))
info = subprocess.check_output(["pdfinfo", str(OUT / "REPORT.pdf")]).decode()
assert "Pages:           1" in info, info
subprocess.run(["pdftoppm", "-scale-to", "1600", "-singlefile", "-png", str(OUT / "REPORT.pdf"), "/dev/shm/ct23-cost16-inspection/closed-PR4544-CI"], check=True)
print(json.dumps({"pdf_bytes": (OUT / "REPORT.pdf").stat().st_size, "pdf_sha256": hashlib.sha256((OUT / "REPORT.pdf").read_bytes()).hexdigest(), "pages": 1}))
