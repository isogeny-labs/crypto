from pathlib import Path
s=Path('research/relation_fusion_20261117');v=s/'visual-358';v.mkdir(exist_ok=True)
labels=[['Recover real accounting','Keep current kernels'],['338 +16 tests pass','Original assertions'],['36 native cases pass','Complete proofs retained'],['Full-boundary timing','Canonical access pending']]
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="1100" height="120" viewBox="0 0 1100 120"><rect width="1100" height="120" fill="white"/>']
for i,lines in enumerate(labels):
 x=i*280;svg.append(f'<rect x="{x+3}" y="12" width="250" height="70" rx="7" fill="{ "#e9eff6" if i<3 else "#eeeeee"}"/>')
 for j,l in enumerate(lines):svg.append(f'<text x="{x+16}" y="{40+j*24}" font-family="DejaVu Sans" font-size="17" fill="#263b50">{l}</text>')
 if i<3:svg.append(f'<text x="{x+258}" y="55" font-family="DejaVu Sans" font-size="20">→</text>')
svg.append('<text x="4" y="110" font-family="DejaVu Sans" font-size="17" fill="#263b50">Correctness qualification · no new timing result or 15× admission</text></svg>');(v/'qualification-flow.svg').write_text(''.join(svg))
t='''#set page(paper: "a4", margin: 17mm)
#set text(size: 10.5pt, font: "DejaVu Sans")
#set par(leading: 0.45em)
#text(size: 19pt, weight: "bold")[Actual-root compiler and algorithm qualification]

*Original15× goal open. No new comparative timing result.*

The full default-feature root test target now compiles. Recovered-group350 passes338 original tests; API/plugin356 passes16 more. Current-source native352 passes all36 declared relation cases and reproduces every181 stored file from322 byte-for-byte. All results are unsigned correctness evidence. [pcc:unverified]

#image("visual-358/qualification-flow.svg", width: 100%)
#table(columns: (31%,69%), inset: 5pt,
[*Scope*], [*Recorded outcome*],
[Root compiler348], [Exit0; every unit body compiles; no runtime pass implied by no-run],
[Recovered algorithms350], [338passed;0failed;41existingignored;7837outside selected scope],
[Changed APIs/plugins356], [16passed;0failed; all declared filters resolve],
[Native352], [36cases;252mutationassertions;2refusals;181files byte-equal322],
[Original example357], [Documented16/120 and36/512 fixtures pass;7260 and131328 complete pairs exactly equal BigUint reference],
)

*Real accounting recovered without changing solver mathematics.* Trusted f66a02c945 supplies filter/core/reconstruction work accounting. Removing accounting alone gives byte-identical reconstruction/core math; current fast word conversion, safe gather, four-lane kernels and FlatColumn layout remain. The original paired-solve cost assertion passes unchanged. This partial legacy row-work unit includes a residual dense bound and excludes failed Krylov work without a returned report. No full primitive-work or speed inference. [pcc:unverified]

Other repairs supply real reciprocals in four test constructors, complete occupied-entry test views, explicit legacy auxiliary/trace/FL defaults and a split mutable borrow. Every original assertion macro roster remains. Qualification covers recovered trace/radical/F4/projection/quartic/sparse/tower algorithms, both quartic decompositions against scan, sampled/SGE paths, collision ledgers, both pair tables, word sweep and all three restored plugin tails. [pcc:unverified]

Native352 keeps both declared primes, n3/4/6, seed7LAST-star and three original kernels. Membership, containment, reduced monicity, strict-standard and checked-product obligations remain complete. Original Fp observed solving degree remains null. [pcc:unverified]

#pagebreak()
#text(size: 18pt, weight: "bold")[Provenance, failures and remaining acceptance]

#table(columns: (27%,73%), inset: 5pt,
[*Attempt*], [*Retained outcome / correction*],
[332], [18 compile diagnostics;0runtime tests. Kept as historical failure],
[344→345/347], [Real accounting/API repairs; four missing test-module enum imports cause compile failure, retained exactly],
[348], [Adds the real enum import; actual default-feature root unit target compiles],
[354→356], [Strict roster rejects wrong cfgtest module path before any numerical call; corrected path, same original tests],
[355], [Only two hashed superseded November17 compiler-cache files reclaimed; all frozen evidence/tools and current executable preserved],
)

Root348/350/356 use default features; native352/example357 use actual crypto default-features=false with the same locked dependency closure. All use opt0/no-LTO/codegen16, CPU0, one build/test/numerical thread. The tiny example harness compiles the original main against the actual352library, with no copied math. Source348 pins1452 root/selected-crate files; it does not claim a standalone copy of every build dependency. Rebuild at published parent95f6003d16 with its dependencies/submodules plus the packet's seven-file delta and locked manifests. [pcc:unverified]

Packet360 retains current native artifacts, full raw logs, exact failed/before source contexts, scripts and source deltas, indexed by hashes. Saved-proof replay uses the unchanged small auditor331 against frozen85 inputs. The557MiB local unit executable is hashed before calls but is not redistributed; full unit reproduction requires source rebuilding. Original report334/archive337 remain frozen historical records. No independent-host or signed-execution attestation, full8216-test runtime-suite qualification or native-counter attestation. [pcc:unverified]

*Performance acceptance remains unchanged.* Last repeated200/211 primary medians1.0027–1.1244 remain inside A/A spread. No real comparative flat/native-launcher timing has run. Before15×: prospectively publish final source/tools/inputs, strongest matched old/current controls with equivalent launcher improvements, entire agreed application-call boundary, repeats/A/A, all six thresholds, every failure and all complete proofs. No fresh/reserved seeds, hidden setup or weaker baseline. [pcc:unverified]

*Canonical access dependency:* #link("https://github.com/isogeny-labs/workspace/blob/main/.agents/skills/experiment-protocol/SKILL.md")[experiment-protocol SKILL.md] says “Read the canonical SKILL.md and follow it.” Recheck349 finds no supported Research Vault KEY binding for current encrypted protocol/export/schema. No proof/checker/measurement gate waived. LocalCPU0/one-thread/zero-cloud and persistent checkpoint/S3 waiver remain; no S3 credentials needed.

PR4989 · ROOT_UNIT_REPORT_358.md · unsigned producer evidence [pcc:unverified]
''';(s/'root-unit-report-358.typ').write_text(t)
import typst
typst.compile(str(s/'root-unit-report-358.typ'),output=str(s/'root-unit-report-358.pdf'),root=str(s.resolve()))
