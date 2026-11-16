# Actual degree-word guard and original F4Run obstruction

[Report](reports/report.pdf), [protocol](PROTOCOL.md), [native protocol](protocol.json),
[export](export.json), [proof map](proof-map.json), [source plan](sources.json),
[verification](verification.json), [source review](evidence/source-review.json).

Main: `MsolveDegreeWordAudit.actual_degree_word_certificate`.

The [actual degree-word audit](README.md)
proves the missing UInt16 range guard and a counterexample to unrestricted
stored-LCM degrees. Actual Lean word storage equals the natural degree
modulo65536, and equals the natural degree iff it is below65536. Input
monomials(65535,0),(65534,1) have in-range degree65535; their LCM has
true degree65536 and stored degree0. Over every field, the actual family
x^(n+2)+1,x^(n+1)y excludes y from every original degree(n+2) F4Run,
while an actual degree(n+3) run produces y. This proves the degree needed
to produce y; no full generic Groebner-basis/solving-degree theorem is
asserted. [pcc:unverified]

The frozen native protocol and analyzer were committed before data.
One registered healthy control passes: exit0/exact basis y,x²+1/degree3.
The single boundary call records the predicted wrap/selecteddegree0/
true-degree65536 rows and writes y,x^65535+1, but returns timeout124.
Its normal-completion criterion FAILS; native execution remains
notvalidated. No retry/postdata analyzer repair. Lean decodes the actual
written set/captured prefix and proves its mathematical F4Run obstruction,
without asserting complete execution, formal parser/C/compiler refinement
or independent-host assurance. Unified export check --verify-files passes;
flow2attempted/1completed/1notcompleted. [pcc:unverified]

The actual historical f4-model-sources.json header is detected as a prefix
of the full613-character theorem, stopping inside(K:=...). That frozen
record stays unchanged. A new transparent bundle rebinds all seven
inherited clauses from the original theorem. New complete131-character
main/SHA285221ce90632adbf8398f66d7c011b1c8b9ad6cdc5be42b95e1616cd6011e64:
1complete/0prefix/0unresolved/empty suffix after completed gate; separate
legacy negative0complete/1prefix/0unresolved. [pcc:unverified]

Local **PCC-20261116-3e0526 PASS at 2026-11-16T16:19:26Z**, signed unverified. Wholegate68 declarations
including generated helpers/0failures/0trustednative. Six modules (five
concrete, one official bridge), new warnings0. Seven states retain four
failures01/02/03/05 and successes04/06/07;04 was generic mathematical
phase only. All6306 predecessor/dependency entries and51initial plans
match around builds/gate/binding/replay, plus46native instrument/data
pins. Producer6357records/1937unique committed paths match. Eleven
packages/no Lean copies/hydration/update.20formal59artifact79pins/
11actually imported custommodules(5credited+6new). Lossless282files/
141objects/196748XZbytes/4parts/roundtrip/actualrestore. Firstvalid
onepage47024B/fulltext/fullindividualvisual/inbounds. Standard foundations/
default limits, no holes/custom axioms/native proof evaluation/raised
limits/waivers/frozen edits/mathPython/spend. [pcc:unverified]

Primary pinned msolve5e72b9d code and official Lean/Mathlib interfaces are
recorded with bounded ranges. Faugere1999 DOI10.1016/s0022-4049(99)00005-5
checked as metadata only; original fullpaper503/403 and C-standard503
unread. Nine KB/two source Conductor calls127; publication scope
Conductor127. Available unmodified canonical protocol tool005a7d7 pinned;
fetched newer reviewed paths opaqueVLT1/unread. No novelty/absence claim.
All14 remain active, including whole-C/row-trace refinement, completed
normal boundary execution, physical-host independence and the existing
geometric/counting/walk obligations. [pcc:unverified]

## Publication

Published in ready [PR4389](https://github.com/isogeny-labs/crypto/pull/4389).
The proof-packet local commit `fbd77e5a5fde243500d981425abe9ab5918171fc` and remote commit
`1353313c8317f201be4d75f3bbd64ab056888e40` share tree
`127ac1b9f909d9e4fdd79df8eb9fd209c6e4ac5f`. Completed anonymous fetch and fresh exact
head/title/body/open/ready check are recorded in [publication.json](publication.json).
Current-head CI/review/signing remain pending; local replay PASS does not
establish signed standing. All14 targets remain active; boundary native
execution is timeout124/notvalidated. [pcc:unverified]
