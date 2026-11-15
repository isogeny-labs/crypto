# First build attempt and faithful correction

At0a5b90e2 C++ build0; Rust build1 with two E0277 diagnostics: default-edition array.into_iter returns references. No solver, audit or control process started. Original source is immutable in that commit and copied locally with exact logs. [pcc:unverified]

Changed only the two map literal constructors to iter().copied().collect(); no polynomial, group, resource, cap, certificate, coverage or acceptance rule changed. NewPINS.v2 and separately named run-v2 bind the correction before affected compilation. Existing LOCAL budget and the full153 configurations plus positive control remain unchanged. [pcc:unverified]

## First native run rejected by actual independent audit

At01c4ba62 both native builds0 and solver0; auditA101 on exact input-polynomial mismatch before receipt; auditB/controls unstarted. C++ used neg(8) on a FIELD code (1+i), while the quartic requires the INTEGER coefficient−8 (−1 in characteristic7). Rust independently reconstructed the correct coefficient and refused it. Every original source/case/log remains immutable/local. No degree observation from that attempt is accepted. [pcc:unverified]

Change exactly that literal to neg(mod(8)); leave Rust arithmetic, cap/order/resource/coverage/hypotheses/acceptance unchanged. PINS.v3/run-v3 freeze the faithful correction before a fresh complete replay. This is a real producer defect, not an auditor relaxation. [pcc:unverified]
