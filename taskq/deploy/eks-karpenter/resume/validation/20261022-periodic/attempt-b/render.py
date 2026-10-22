"""Render recorded qualification counts; no benchmark computation."""
import json
from pathlib import Path
import typst
import pymupdf

root = Path(__file__).resolve().parent
data = json.loads((root / 'verification.json').read_text())
grades = {item['level']: item['count'] for item in data['fresh']['grades']}
assert grades == {'L1': 3, 'L2': 3603}
svg = '''<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="310" viewBox="0 0 1000 310">
<rect width="1000" height="310" fill="white"/>
<g font-family="sans-serif" fill="#18354e">
<text x="25" y="32" font-size="25">Fresh epoch rejected: 3 of 3,606 observations below L2</text>
<text x="25" y="62" font-size="17">ecbench-periodic-20261022-b · one epoch · zero retries</text>
<path d="M180 93 V234 H870" fill="none" stroke="#8797a4"/>
<text x="25" y="133" font-size="21">L2</text><rect x="180" y="105" width="650" height="42" fill="#327653"/>
<text x="842" y="133" font-size="21">3,603</text>
<text x="25" y="205" font-size="21">L1 (rejected)</text><rect x="180" y="177" width="1" height="42" fill="#ac3939"/>
<text x="194" y="205" font-size="21" fill="#ac3939">3</text>
<text x="180" y="260" font-size="16">0</text><text x="810" y="260" font-size="16">3,606</text>
<text x="420" y="284" font-size="17">Recorded observations (count)</text>
<text x="25" y="305" font-size="15">Native replay: 3,600 / 3,600 matched. A/A not evaluated after L2 rejection.</text>
</g></svg>'''
(root / 'grades.svg').write_text(svg)
source = '''#set page(paper: "a4", margin: 17mm)
#set text(size: 9pt)
#set par(leading: 0.5em)
= Periodic recovery qualification B
*Overall result: failed. Recovery passed; fresh L2 admission failed.*

#raw("ecbench-periodic-20261022-b")

#image("grades.svg", width: 100%)

== Frozen scope
Public 32-bit fixture, curve seed 59297, two targets (seed 20261002), rho.negation reference, rho.plain baseline and matching negation A/A control; 600 rounds plus one warmup. Checkpoint interval 30 seconds, original deadline 3,600 seconds, at most two restores, zero measurement retries. Account quota 96 vCPUs, lab cap 64. Runtime source: #raw("5499eeb9d72e6e777cb0d28b0ade41afbfff90b7"). Full immutable request and identities: protocol.json.

== Recovery observed
Two encrypted, completed snapshots committed with full versioned archive readback. After the original controller stopped and its dedicated worker terminated, generation 2 restored on a replacement c8a.2xlarge. The first 2,486 records / 11,113,922 bytes remained identical; computation completed 3,606 verified records and 3,600 matching native replays. All computational timing remains invalid.

Both restored processes were stopped with UID/GID 1000, empty supplementary groups, zero capabilities, no_new_privs and Seccomp 2. Both checkpoint captures succeeded initially: live zombie recapture was not forced, although native regression tests cover it. See restored-control and g1/g2 metadata.

== Fresh epoch rejected
The distinct fresh Job completed native verification, but sequences 676, 1406 and 1826 each recorded three excess busy ticks on CPU 4. All had zero steal and no SMT siblings. The retained counters do not identify a noise source or prove a kernel-accounting defect. No grade or threshold was changed. L2 exited 42 before A/A, which remains unevaluated. The logical run and restarted CodeBuild controller failed; no benchmark retry occurred.

All 18 emitted S3 payload hashes matched after download. The frozen acceptance required all 3,606 L2, admitted A/A and 19 payloads; those criteria were not met. Earlier positive F evidence does not turn this run into a pass.

== Evidence and remaining work
REPORT.md links the exact private S3 result prefix and logical receipts. public-evidence.tar.xz retains the full records, audits and identities; verification.json and verify-failure.sh document checks. Checkpoint memory stays in private encrypted S3, never Git. Both owned PVCs and all three temporary operator access associations were removed after preservation; committed snapshots remain retained.

The runtime pin now matches the deployed checkpoint repair with this limited qualification. The separately frozen main-only GitHub Action remains unverified and retains its original acceptance criteria. No identical B rerun is scheduled. This is infrastructure evidence, not a timing or algorithm speed claim; canonical performance graphs and leaderboards are unchanged.
'''
(root / 'report.typ').write_text(source)
typst.compile(str(root / 'report.typ'), output=str(root / 'report.pdf'))
pdf = pymupdf.open(root / 'report.pdf')
assert len(pdf) == 1
assert data['run'] in pdf[0].get_text()
pdf[0].get_pixmap(dpi=120).save(str(root / 'report-1.png'))
print('One-page PDF rendered; visual inspection required.')
