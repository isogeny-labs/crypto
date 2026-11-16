# Seed-monomial source factorization

The native paired source now has a separate candidate using the derived identity
R(c)=c^E R(1). It evaluates a seed-free residual and applies N^E,D^E once at the
end. E is computed once in setup from public p,r and the source partition. The
old pair consumer is test-only; raw scalar ratio and recovery code are unchanged.

All192 native methods pass, including2545 independent exponent checks,144 exact
ratio/rescaling cases and all8 complete recovery receipts. See DERIVATION.md,
control.json, source-lock.json, receipt.json and raw logs. This is a new candidate;
no comparative panel ran. Complete operation accounting is the next step.

The original4563-case cohort, prior candidate sources, binaries and protocols
remain unchanged. The report source, editable SVG and inspected PDF accompany
the derivation. Source source.rs lives under routed/, matching prior layout.
