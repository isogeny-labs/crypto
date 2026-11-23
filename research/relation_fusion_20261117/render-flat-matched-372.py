from pathlib import Path
s=Path('research/relation_fusion_20261117');v=s/'visual-372';v.mkdir(exist_ok=True)
labels=[['18 previous arms','36 matched controls'],['72 A/A +48 A/B arms','3600 planned calls'],['Two synthetic cohorts','7200 analytic proof replays'],['Real timing pending','15× goal still open']]
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="1100" height="120" viewBox="0 0 1100 120"><rect width="1100" height="120" fill="white"/>']
for i,lines in enumerate(labels):
 x=i*280;svg.append(f'<rect x="{x+3}" y="12" width="250" height="70" rx="7" fill="{ "#e9eff6" if i<3 else "#eeeeee"}"/>')
 for j,l in enumerate(lines):svg.append(f'<text x="{x+14}" y="{40+j*24}" font-family="DejaVu Sans" font-size="16" fill="#263b50">{l}</text>')
 if i<3:svg.append(f'<text x="{x+258}" y="55" font-family="DejaVu Sans" font-size="20">→</text>')
svg.append('<text x="4" y="110" font-family="DejaVu Sans" font-size="16" fill="#263b50">Invented clocks and monitor flags · complete saved proofs · no performance admission</text></svg>');(v/'draft-vs-evidence.svg').write_text(''.join(svg))
t='''#set page(paper: "a4", margin: 16mm)
#set text(size: 10pt, font: "DejaVu Sans")
#set par(leading: 0.4em)
#text(size: 18pt, weight: "bold")[Flat comparison draft: stronger controls]

*15× goal unachieved. Zero actual performance observations.*

All18previous generic/packed arms become36eligible controls, each with both launchers. Six flat variants add12candidate arms. Primaryflat-dag-487-native is fixed before performance data.72A/A plus48A/B arms,5rounds and six fixed prime/n cases give3600planned calls. Frozen253source/tools/evidence stay unchanged. [pcc:unverified]

#image("visual-372/draft-vs-evidence.svg", width:100%)
#table(columns:(30%,70%),inset:4pt,
[*Qualification*],[*Recorded result*],
[Unit365],[15pass:9analyzer+3collector+3fixture; includes previous-packed-primary strongest-control rejection],
[Synthetic neutral367],[3600whole-proof replays; threshold arithmetic false; admission false],
[Synthetic hypothetical20368],[3600whole-proof replays; invented20× clocks pass arithmetic; admission remains false],
[Clock369 /duplicate370],[Both exit1 at expected phase-binding/duplicate-cell gate; no analysis report],
[Isolation],[All21605neutral source file hashes unchanged; frozen253andnew366source/tool pins unchanged],
)

Every clock and monitor flag is invented fixture data, not a host observation or execution attestation. The fixture separately prechecks7200saved proofs before linking; these are distinct from7200analytic replays. Complete membership, containment, reduced monicity and strict-standard/checked-product obligations remain. Original Fp solving degree stays null. [pcc:unverified]

New files retain253's full application-call boundary, complete proof/basis/stat/hash/clock binding, strongest-control minimum, repeated-round/bootstrap/A/A rules and global gates. Only roster/include/schema/usage differ in runtime analysis. A copied unit expected24controls and initially failed364;365raises that assertion to36and adds the previous-primary test. No old gate, assertion or frozen file is weakened. [pcc:unverified]

Saved240generic/tree and281flat proofs supply the fixture. Proofs are checked before immutable-source hardlink reuse; canaries use atomic metadata replacements. A NEW synthetic pin record names actual flat274as current worker; original200pins remain unchanged. These are not prospective performance pins. Packet373 retains both full cohorts, canary byte deltas, initial364failure and source/tools. Replay374 describes fresh checks. [pcc:unverified]

Earlier actual-root compiler348,338recovered tests,16API/plugin tests,36native cases and two example fixtures remain published at4e6364822b. Full8216-test runtime suite remains unqualified. No independent-host, signed-execution or native-counter attestation; this draft adds no actual-root timing. [pcc:unverified]

*Canonical dependency:* #link("https://github.com/isogeny-labs/workspace/blob/main/.agents/skills/experiment-protocol/SKILL.md")[experiment-protocol SKILL.md] says “Read the canonical SKILL.md and follow it.” Encrypted current protocol/export/schema still lacks a supported Research Vault KEY binding (349). Reconcile this draft with canonical requirements before qualifying a separate prospectively pinned real collector/analyzer. Keep strongest controls, equivalent launcher improvements, full agreed boundary, repeats/A/A, all six15× thresholds, every failure and every proof. No fresh/reserved seeds or weaker baseline.

LocalCPU0/one thread/zero cloud and persistent checkpoint/S3 waiver remain. PR4989 · unsigned producer evidence [pcc:unverified]
''';(s/'flat-matched-report-372.typ').write_text(t)
import typst
typst.compile(str(s/'flat-matched-report-372.typ'),output=str(s/'flat-matched-report-372.pdf'),root=str(s.resolve()))
