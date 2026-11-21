#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
= Managed exporter: independent source review
Blocking findings, source-only. Current reviewed snapshot manifest:
#text(size:8pt)[b63943f0d46ebb5a5bcd806bd97028308240137e8b5900bc6dff3cd1d702ddec]
#image("flow.svg",width:100%)
== Findings
R1. Resource validation finds plans by the attempt session ID, while the plan inventory names primary sessions only. A distinct-session retry with resource evidence fails despite a valid primary parent. Pass the already validated parent slot's plan identity to resource validation.
R2. Structural custody closure and diagnostic-attachment completeness are stored but omitted from pair eligibility and its reasons. Raw finite arithmetic costs may be preserved, but comparison eligibility must separately enforce the required evidence contract. Policy changes during review do not resolve this discrepancy.
R3. Every session must already be complete with 3240 records; every retry session must cover all3240sequences. Killed/partial sessions abort export instead of retaining their observed evidence with missing slots and ineligibility. This requires a retention-path correction or a prospectively reviewed narrower contract.
R4. The assigned candidate manifest and patch changed during review, and the producer reported no saved original snapshot. The observed replacement is copied independently. Exact original recovery cannot be claimed; future changes require separately sealed successors.
== Checked progress and limits
Actual seven hydrated plans and their validation receipt matched registered hashes. Slot derivation, primary bijection, explicit retry parents and full triplet identity checks address the original aggregate-count weakness. Actual file hashes, native child/attachment/resource deserialization and existing nine-category price mapping are present. Null or incomplete mapper results remain null. Cargo manifests show no reverse dependency cycle.
Six source-written controls were inspected, not executed; they do not cover the retry resource join, missing-custody eligibility or partial-session path. Nine bound donor/overlay files match their pins, but donor runner source differs from the advertised Git-head bytes. Compilation and integrated runtime compatibility remain unchecked.
