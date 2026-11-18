# Local exact-algebra implementation qualification (goal turn2)

This is continuation of the active goal and draft PR4567. The public Magma
allowance of2 requests is exhausted; do not submit another request there.
Implement/qualify a local exact verifier, not another remote retry or a time
experiment. Existing m=2–7/depth8 acceptance and all old proof bytes stay fixed.

## Sources and decision before execution

KB remains unavailable: no exposed KB tools and scripts/research-kb exits127.
Read existing protocols/derivation and the current primary CoCoA manuals:

- https://cocoa.altervista.org/html/cmdNewZZmod.html documents arbitrary
  integer moduli; release notes explicitly include very large moduli:
  https://cocoa.altervista.org/download/RelNotes.shtml .
- https://cocoa.altervista.org/html/cmdreg.html distinguishes reg(ideal)
  from reg(quotient), showing4 versus3 for a degree2,3 CI. It names the
  Bermejo–Gimenez algorithm, not Hilbert-function RegularityIndex.
- Official download https://cocoa.altervista.org/download/ provides the
  stable5.4.0 Linux archive and a vendor SHA file. Use that exact release.

These documents justify a **capability check**, not an assumption that the
engine's polynomial/resolution code accepts a256-bit coefficient field.
Do not replace P256 by QQ, a small prime or a reduced parameter cell after
an unsupported-field error and label that a P256 result.

## Bounds and admission

Download/extract the official native tool in scratch only (archive<=100MiB),
record URL, archive/vendor/binary hashes and exact invocation. No paid service,
credential change or system-wide installation. One known-answer capability
program, at most10s wall/1GiB address space, must establish:

1. Characteristic exactly the full NIST P256 prime; coefficient ring a field.
2. Known CI degrees2,3: ideal regularity4, quotient regularity3.
3. A principal linear ideal: ideal regularity1, quotient regularity0.
4. A unique successful completion marker with no engine error.

Only after that passes, use the **unchanged frozen complete residual inputs**
at depths2/4,m2,dense/tower, same exact linear preprocessing, no source-degree
shortcut. Native exporter must verify coefficientwise H=1 reconstruction and
byte-identical original/residual JSON before writing the new syntax. Budget
four one-shot cells, each<=45s wall/2GiB, zero automatic retries; total native
algebra allowance<=240s. Stop at unsupported-field/algorithm errors. Record
all timeouts and unsuccessful arms; incomplete resolutions are not values.
No parameter search or benchmark/performance claim is authorized by this unit
qualification. Local builds and exact correctness unit checks remain development.

Report reg(J) and reg(S/J) separately, preserve Betti/resolution evidence and
normalization, and do not claim a minimal-resolution certificate from a final
basis degree or an unqualified bound. A CAS assertion remains unsigned until
independently checked. Exact group replay is required for every reported
relation; relation coverage/full original workload remain open if not checked.

If the capability check fails, retain the failure and continue independent
native proof work. In particular examine an exact retained-fibre selector in
the tower's squarefree monomial basis, so extension-field nonlifting points
cannot masquerade as equivalent retained-base algebra. This would fix a
semantic obligation, not automatically bound complete relation regularity.
