"""Lay out recorded Rust values and custody metadata; no scientific arithmetic."""
from pathlib import Path
import ast
import hashlib
import json
import os
import subprocess

os.environ.setdefault("MPLCONFIGDIR", "/dev/shm/ct23-matplotlib")
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import typst

STUDY = Path(__file__).resolve().parent
ROOT = STUDY.parent.parent
PREVIEW = Path("/dev/shm/ct23-cost16-inspection")
PREVIEW.mkdir(exist_ok=True)
DATA = json.loads((STUDY / "first-native-B-success-04/ACTUAL_FIRST16B_SUCCESS_AND_NATIVE_AGGREGATES.json").read_text())
CLAIM = (STUDY / "CURRENT_HISTORICAL_RECORD_CLAIM.txt").read_text().strip()
REPO = "https://github.com/isogeny-labs/crypto"
CHECKPOINT = "7f9d552c804a0f58e403b1671d07901a735b4499"
TAG = " [pcc:unverified]"

def fields(line):
    return dict(x.split("=", 1) for x in line.split("|") if "=" in x)

records = [fields(x) for x in DATA["Rust_emitted_aggregate_budget_and_comparison_lines_only"]]
maps = next(x for x in records if x.get("kind") == "maps" and "pairs" in x)
budgets = {x["arm"]: ast.literal_eval(x["ops"]) for x in records if x.get("kind") == "maps" and x.get("scope") == "inverse_only"}
out = STUDY / "reports/actual-cost16"
out.mkdir(parents=True, exist_ok=True)
fig, ax = plt.subplots(figsize=(7.0, 2.6))
pos = [0, 1, 2, 3]
for values, offset, color, label in [
    (ast.literal_eval(maps["inverse"]), -.18, "#346a87", "Uniform inversion-call atom"),
    (ast.literal_eval(maps["expanded_inverse"]), .18, "#b47a42", "Separate inversion-input coordinates"),
]:
    bars = ax.bar([x + offset for x in pos], values, width=.36, color=color, label=label)
    ax.bar_label(bars, padding=3, fontsize=9)
ax.set(xticks=pos, xticklabels=["Equal", "Candidate lower", "Baseline lower", "Incomparable"], ylabel="Paired map calls", ylim=(0, 39))
ax.legend(frameon=False, fontsize=8.3, loc="lower center", bbox_to_anchor=(.5, 1.02), ncol=2)
ax.tick_params(labelsize=9)
ax.spines[["top", "right"]].set_visible(False)
fig.tight_layout(pad=.6)
fig.savefig(out / "fixed34-map-cost-classifications.svg")
fig.savefig(out / "fixed34-map-cost-classifications.pdf")
_svg = out / "fixed34-map-cost-classifications.svg"
_svg.write_text("\n".join(line.rstrip() for line in _svg.read_text().splitlines()).rstrip() + "\n")
plt.close(fig)

def paragraph(text):
    return ("p", text + TAG)

def link(label, url):
    return ("link", label, url)

def table(headers, rows):
    return ("table", headers, rows)

def image(name, caption):
    return ("image", name, caption + TAG)

def dot(folder, filename, graph):
    folder.mkdir(parents=True, exist_ok=True)
    (folder / (filename + ".dot")).write_text(graph)
    subprocess.run(["dot", "-Tsvg", str(folder / (filename + ".dot")), "-o", str(folder / (filename + ".svg"))], check=True)

def render(folder, title, subtitle, pages):
    folder.mkdir(parents=True, exist_ok=True)
    md = ["# " + title, subtitle]
    src = ['#set page(paper: "a4", margin: 16mm, numbering: "1 / 1")',
           '#set text(font: "DejaVu Sans", size: 10pt)',
           '#set par(justify: false, leading: 0.58em)',
           '#set heading(numbering: none)',
           '#align(center)[#text(size: 17pt, weight: "bold")[' + title + ']#linebreak()#text(size: 9pt)[' + subtitle + ']]']
    for page_index, blocks in enumerate(pages):
        if page_index:
            src.append("#pagebreak()")
        for block in blocks:
            kind = block[0]
            if kind == "p":
                md.append(block[1]); src.append("#text(" + json.dumps(block[1]) + ")\n")
            elif kind == "heading":
                md.append("## " + block[1]); src.append("== " + block[1])
            elif kind == "link":
                md.append("[" + block[1] + "](" + block[2] + ")")
                src.append("#text(size: 8.5pt)[#link(" + json.dumps(block[2]) + ")[" + block[1] + "]]\n")
            elif kind == "image":
                md.append("![" + block[2] + "](" + block[1] + ")")
                src.append("#figure(image(" + json.dumps(block[1]) + ", width: 100%), caption: [" + block[2] + "])\n")
            elif kind == "table":
                heads, rows = block[1], block[2]
                md.append("| " + " | ".join(heads) + " |\n| " + " | ".join(["---"] * len(heads)) + " |\n" + "\n".join("| " + " | ".join(row) + " |" for row in rows))
                cells = heads + [cell for row in rows for cell in row]
                src.append("#text(size: 8.5pt)[#table(columns: " + str(len(heads)) + ", inset: 4pt, " + ", ".join("text(" + json.dumps(x) + ")" for x in cells) + ")]\n")
    (folder / "REPORT.md").write_text("\n\n".join(md) + "\n")
    (folder / "report.typ").write_text("\n".join(src).rstrip() + "\n")
    typst.compile(str(folder / "report.typ"), output=str(folder / "REPORT.pdf"), root=str(folder))
    info = subprocess.check_output(["pdfinfo", str(folder / "REPORT.pdf")]).decode()
    count = int(next(x.split(":", 1)[1] for x in info.splitlines() if x.startswith("Pages:")))
    if count != len(pages):
        raise RuntimeError("Unexpected page count for " + str(folder) + ": " + str(count))
    subprocess.run(["pdftoppm", "-scale-to", "1600", "-png", str(folder / "REPORT.pdf"), str(PREVIEW / folder.name)], check=True)
    pdf = (folder / "REPORT.pdf").read_bytes()
    return {"folder": str(folder.relative_to(ROOT)), "pages": count, "pdf_bytes": len(pdf), "pdf_sha256": hashlib.sha256(pdf).hexdigest(), "preview_prefix": str(PREVIEW / folder.name), "inspection": "pending"}

result_pages = [
    [paragraph("The historical first cost16B record compares the guarded cubic inverse with the original norm-and-Euclid inverse on 17 forward and 17 dual Richelot map calls. All 34 coarse operation vectors are candidate-lower; separating inversion inputs leaves 2 candidate-lower and 32 incomparable pairs. This is a fixed same-engine component count, not a measured runtime advantage. Current unsigned record: " + CLAIM + "."),
     image("fixed34-map-cost-classifications.svg", "Recorded classifications for 34 paired map calls, source e7efdbf1, 17 November 2026. These are complete fixed populations; no statistical interval was recorded."),
     paragraph("Exact totals below cover only work inside Field.inv. M, A, S and R count explicit BigUint multiplication, addition, subtraction and remainder outside modpow. I counts actual modpow calls atomically; R64 counts the CubicField.c u64 remainder. Internal Montgomery limbs, allocations and CPU prices are not expanded."),
     table(["Arm", "M", "A", "S", "R", "I", "R64"], [["Original"] + [f"{x:,}" for x in budgets["baseline"]], ["Guarded"] + [f"{x:,}" for x in budgets["candidate"]]]),
     paragraph("The aggregate I coordinate assigns one common atom to different inversion inputs. Expanded coordinates retain every normalized input and do not assign fitted prices. Across 64 cubic primitive pairs the coarse model gives 64 lower; the expanded model gives 1 lower and 63 incomparable. All 8 general inverse pairs are equal in both models."),
     paragraph("The simultaneous all-backend budgets cover the instrumented FpPoly/CubicField sites, but omit direct Fp2/u64 work elsewhere in the maps. Neither budget is a complete map-discovery, destination-solver or end-to-end ECDLP cost. The native test harness elapsed time is not a benchmark."),
     link("Immutable native-emitted aggregates, full-input histograms and exact totals", REPO + "/blob/" + CHECKPOINT + "/research/chained_primefield_cost_20261117/first-native-B-success-04/ACTUAL_FIRST16B_SUCCESS_AND_NATIVE_AGGREGATES.json")],
    [("heading", "Frozen checks and current evidence"),
     paragraph("The recorded B caller exited zero and all 15 phase expectations match, including the expected Clippy-negative exit. Strict compilation, Clippy, format, 12 hand-calculated calibration cases at all 17 sites, one corrupt-counter/restored-positive control, and all 106 prior scientific rows are retained in the checkpoint records. One new native test passed; 15 inherited tests were compiled and filtered. There are 64 cubic and 8 general primitive pairs, 2 zero, 8 sign, 3 constructor controls, and 17 forward plus 17 dual map calls."),
     paragraph("The execution record pins 85 original sources, 28 copied sources, 3 prebuilt dependency rlibs and 3 compiler tools, with distinct build/pre/post manifests and qualifier d9db82db. The official num-bigint 0.4.8 archive matches Cargo.lock and the three retained primary source files. Those historical input and guard records survive. No native compilation or arithmetic was repeated during this publication."),
     paragraph("The full 161,784-byte stdout has historical SHA-256 7d528378104dc4f2646f3f6d92bf9b2426c6b38b0af67a72bd86d09607a6d666. The 66-member, 1,773,712-byte whole archive has SHA-256 eefd5d1a321425be038136d45196decc3c9a51d3b1c66e5e5bc392edb47bedef. It contained source, three rlibs, both ELFs and complete recorded streams when sealed on 17 November. The full stdout was deliberately kept outside Git under the unchanged artifact policy."),
     paragraph("On 23 November the recorded tmpfs packet and raw-output paths are absent after restart. Current public custody consists of exact source, guards, aggregate native lines, hashes and inventory; the complete raw stdout and private archive are not presently available at the inspected paths. Historical seal assertions remain unchanged and are not assertions of current availability. Recovery or approved whole-packet storage remains open."),
     paragraph("SourceA 8f3d314c failed the exact-publication guard before copy, compilation or native arithmetic because ignored Cargo.lock was omitted. B e7efdbf1 explicitly published the same lock and changed metadata capture/helper selection only. The missing original A Git stderr is preserved as a separate gap; a later diagnostic never replaces it."),
     table(["Requirement", "Publication status"], [["Exact frozen source and native aggregate record", "Retained and hash-matched"], ["Complete original raw stdout/private packet", "Missing at inspected restart paths"], ["Full Field/Picard formal bridge, independent signing", "Open"], ["Admitted runtime and complete solver comparison", "Open; no new measurement"]]),
     link("Prospective protocol dd6223fa (before implementation and calls)", REPO + "/blob/dd6223faae3ff638eb2582e8f40ceca41c5a142c/research/chained_primefield_cost_20261117/PROTOCOL_16.md"),
     link("Resume custody record and continuing queue: PR #4990 / issue #3904", REPO + "/pull/4990")]
]
inventory = [render(out, "Prime-field work in guarded Richelot inverses", "Historical cost16 unit: 17 November 2026 · publication and custody update: 23 November 2026", result_pages)]

negative = STUDY / "reports/first-A-publication-negative"
dot(negative, "flow", '''digraph g { rankdir=LR; bgcolor="transparent"; node[shape=box,style="rounded,filled",fillcolor="#eef3f8",fontname="DejaVu Sans",fontsize=10]; a[label="Physical A source lock\ncontains Cargo.lock"]; b[label="Public A tree omits lock\nexact-source guard exits 1"]; c[label="No copy or compiler\nNo native arithmetic"]; a->b->c; }''')
inventory.append(render(negative, "First cost16 attempt: publication guard", "Executed setup outcome: 17 November 2026 · current custody noted below", [[
    paragraph("SourceA 8f3d314c failed during initialization. Cargo.lock was present in its physical 25-file execution lock but omitted from the published Git tree because it was ignored. The exact-public-source guard stopped the caller before source/dependency copying, compilation or native arithmetic. Historical unsigned claim PCC-20261117-30be8e and the original negative record are retained as data."),
    image("flow.svg", "Executed A path. This is a publication failure; no arithmetic measurement or mathematical rejection occurred."),
    paragraph("The two recorded metadata phase exits are 1: initial source validation and final guards for files never created. The 85 original source hashes were checked only afterward. The 36-member, 50,652-byte private packet historically retained every available stream/context and all 26 declared source/lock files; its SHA-256 is 57d41266ef81c7c22cc6b5c5a27b445149551dcd069475b9b785b7f742df1999. No dependency rlib or ELF was copied or built."),
    paragraph("Caller A retained the CalledProcessError representation but lost the metadata subprocess's original Git stdout/stderr. The later read-only diagnostic against the same immutable A commit is explicitly post-call. Its diagnostic survives as such and does not repair the missing first-call stream."),
    paragraph("B separately froze the unchanged Cargo.lock and only metadata helper/caller/lock selection. The revised helper captures failed Git argv, exit, stdout and stderr before raising. All 28 B public source blobs were checked before the native call. A's mathematical code, calibration oracles, prior snapshot, populations and limits were unchanged."),
    paragraph("At resume on 23 November the old private tmpfs storage is absent. The sourceA and complete available negative text/metadata remain in immutable Git; current availability of the former whole private archive is not asserted. The A publication failure, original stderr gap and restart custody loss remain three distinct observations."),
    link("Immutable first-A negative record and exact source", REPO + "/blob/" + CHECKPOINT + "/research/chained_primefield_cost_20261117/first-source-A-publication-negative-03/ACTUAL_FIRST16A_PUBLICATION_NEGATIVE.json")
]]))

restart = STUDY / "reports/restart-custody-20261123"
dot(restart, "custody", '''digraph g { rankdir=TB; bgcolor="transparent"; node[shape=box,style="rounded,filled",fontname="DejaVu Sans",fontsize=10]; a[label="17 Nov: packet sealed and compared\nraw stdout + source + dependencies + ELFs",fillcolor="#eef3f8"]; b[label="Public immutable checkpoint 7f9d552c\nsource / native aggregates / guards / hashes",fillcolor="#e0f0e8"]; c[label="Private tmpfs whole packet\nrecorded paths absent on 23 Nov",fillcolor="#f4e7df"]; d[label="Recovery or approved whole storage OPEN\nNo reconstructed old stream or silent rerun",fillcolor="#f4e7df"]; a->b; a->c; c->d; }''')
inventory.append(render(restart, "Restart custody gap", "Bounded owned-path observations: 23 November 2026 · PCC-20261123-5e3cc7, unsigned", [[
    paragraph("The completed native cost16B data checkpoint survives in GitHub, and all 128 study blobs matched it before new report/resume edits. The previously recorded full raw stdout and whole private archive are absent at their recorded owned tmpfs paths. This is a custody loss at the inspected locations, not a fresh execution or a global irrecoverability finding."),
    image("custody.svg", "Different custody states of the historical packet and its public records; the missing branch remains open."),
    paragraph("The original /dev/shm/ct812-first-cost16B tree was deliberately reclaimed after whole-packet comparison on 17 November. Its complete archive backing was /run/lock/chained-transfers-ct669/entire-native16B-all-source-deps-ELFs-and-raw-streams.tar.xz, linked from /dev/net/chained-transfers-owned-storage-20261117. Both recorded archive paths are now absent. The old owned Gitdir and index are also absent; the original worktree's .git remains a dangling link."),
    paragraph("The public record still pins the 161,784-byte stdout7d528378, the 66-member archiveeefd5d1a, exact public sourcee7efdbf1, all recorded guard manifests and 15 native-emitted aggregate lines. Those metadata do not replace the full raw bytes. No original timestamp, PID or elapsed time has been reconstructed, and no science call was repeated."),
    paragraph("New work uses an isolated partial clone and the untouched current-main PCC prefix. The persistent /workspace overlay reports zero available bytes; new state is therefore checkpointed promptly through GitHub. No protected original, shared Git objects/index, keyring, verifier, budget, provider identity or vault was changed."),
    link("Bounded path observations and exact hashes", REPO + "/blob/918120b5e21f6338c93345a6b49faf9a0faa93a1/research/chained_primefield_cost_20261117/restart-custody-20261123/OBSERVED_RESTART_CUSTODY_GAP.json")
]]))
(STUDY / "REPORT_BUILD_INVENTORY_16.json").write_text(json.dumps(inventory, indent=2) + "\n")
print(json.dumps(inventory, indent=2))
