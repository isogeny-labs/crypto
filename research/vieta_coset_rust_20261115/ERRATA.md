# Retained protocol qualifications

These are post-call disclosures; no frozen protocol or raw record is rewritten.
All observations remain [pcc:unverified].

1. The original three-term P-224/m128 fixture context inherited earlier arm
   runtime and stopped after retained round 6. Twelve planned three-term arm
   calls are absent. `export-three-original.json` retains 564 of 576 planned
   three-term rows. Its canonical checker flags five findings (total retained
   shortfall plus each arm's 7 rather than 10 replicates). It is not a pass.
2. `protocol-three-repair.json` correctly freezes the repaired qualifier SHA-256,
   but its four instrument commit strings were copied from an older protocol.
   Actual source/pre-call publication is
   `f21cb5e7a51e85a2bf0715a23542516d131a439b`, independently fetchable in Git,
   recorded in `scale-20261115T2053Z/repair-publication.json`. The export reports
   that actual source. Its checker flags four commit mismatches. Executable
   custody is not substituted for flawless metadata adherence.
3. The canonical protocol tool refused to freeze an after-data metadata-only
   correction. `protocol-three-repair-postcall.json` is retained as a **draft**,
   not a preregistration or successful freeze. No inputs, qualifier bytes, seeds,
   arm order, operation limits or stage deadlines were changed by the attempted
   metadata correction. The original v3 freeze remains the governing document.

Checker receipts are `scale-20261115T2053Z/export-*-check.log`. The primary
two-term export passes with zero findings, including twelve timeout rows.
Missing and failed calls are never assigned zero cost or counted as wins.
