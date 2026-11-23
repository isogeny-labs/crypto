# Pre-PARI compatibility amendment, 2026-10-27

After the native all-shift count completed, but **before any fixed-subset
Frobenius-polynomial output was run or inspected**, the installed primary GP
help reported `polrem: unknown identifier`. The frozen script uses that name
for polynomial remainder. PARI's supported `%` operator has the intended
semantics.

Preserve the original census, verifier, AUDIT.json, generated verify.gp,
protocol and their hashes. Add exactly one explicitly recorded preload:

```gp
polrem(a,b)=a%b;
```

The native operational transport now optionally passes the preload file before
the unchanged main script and records both files and their hashes in the
actual argv/receipt. No curve, shift, polynomial, sample, threshold, positive
control, count, native verifier or stderr allowlist changes. Native integer
division still independently checks every factor flag. The preload is not a
new cryptographic algorithm or an outcome-selected analysis. It corrects an
API spelling assumption before the affected PARI data.

The old frozen transport binary hash in FROZEN.json remains historical;
GP_COMPATIBILITY.json pins the new transport/helper hashes separately. The
original two native panel executions still use the unchanged frozen worker
and protocol. Record this amendment in the final export and report; do not
claim that the compatibility preload was part of the original freeze.
