# Combined normalized monomial transport

For the fixed partition, put W(R)=c(R)^delta T_t(R)^alpha T_(2w)(R)^beta T_(s+e)(R)^gamma, omitting the tail if s=0. Here t=(r+1)/2 and the exponents are those derived in feature-native-multiseed-factor/DERIVATION.md. Write n_j for the selected indices, including n=1 for c, and e_j for the corresponding powers reduced modulo p-1. All displayed operands must be nonzero.

Multiplying the inherited normalized transport identities raised to e_j gives

    W(R+P) W(R-P) = D(R) W(P)^2 W(R)^2,
    D(R) = product_j (x([n_j]P)-x([n_j]R))^e_j.

The reduction of exponents is legitimate in F_p^*, with caller-certified prime p. For pairs W(R)=[A,B], W(R-P)=[C,D], the next pair is [D(R) W(P)^2 A^2 D, B^2 C]. Scaling either input pair scales both output coordinates equally. This directly specializes the already-used pair update; it does not require the individual columns after initialization. [pcc:unverified]

The consumer's ratio is its unchanged geometry-only residual multiplied by W. It never reconstructs the four input powers from a transported column row. Public setup computes W(P), W(2P), and the anchor W(-P)=(-1)^(sum e_j) W(P). In particular that anchor is not always negative: its parity is the singleton parity e from the partition. O remains the explicit [0,1] convention. P and 2P anchors reset the zero predecessor before a recurrence would divide by it. The first two nonanchor rows still use direct phase and division values. [pcc:unverified]

The code retains all original geometry lanes, grids, polynomial resultants, calibration and domain assumptions: p>3, odd prime subgroup r>=7, r!=p, gcd(r,p-1)=1, existing word-size bounds, and the conditional seed normalization law. Every coordinate difference is still checked nonzero even when its exponent is zero. This deliberately retains the reference domain rather than extending it without proof.

This change stores one pair per row instead of the selected input-column vector. It moves input exponentiation from residual consumption to the coordinate-difference product in transport, while direct initializations and anchor construction evaluate the monomial explicitly. No complete-cost or speed improvement follows from the identity alone. The immediate next integration is a consuming adaptive cursor that advances this pair even on skipped residuals, followed by a matched accounting comparison. Original4563-case cohort unchanged.

## Exact development checks

The frozen four-column source is the reference for represented ratios and character signs. Native tests check the three complete toy subgroups (499 rows) and eleven length16 windows (176 rows), including starts near and crossings of O. A separate test checks the even-parity -P anchor and one-pair representation. These tests share inherited geometry and residual code; they are not an independent mathematical verifier. Three tests pass and60 imported tests are filtered. The full command, binary digest and17 dependency hashes are in receipt.json/source-lock.json. [pcc:unverified]

## Sources checked

KB unavailable: no MCP tools; scripts/research-kb status reports no executable. Read the prior multiseed-factor and normalized-transport derivations and implementations, and transport-accounting report. Re-read NIST DLMF23.10.3, https://dlmf.nist.gov/23.10.E3 (2026-11-07), for the sigma-addition antecedent. This algebraic product construction inherits the seed-law assumption; it is not a characteristic-changing runtime map. No novelty-priority claim. No cloud job or comparative timing panel.
