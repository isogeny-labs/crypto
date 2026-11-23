# Twist generator order: failed full-order rule and maximal-order successor

The maximal-order rule fills **39/40** nonzero-shift bases versus **35/40** for the archived first-point rule. Native and PARI independently verify all120 complete constructions. One twist is **C7 x C119**; its first point has order7 and maximal points have order119. The source has order823 and the twist833. This explains why a whole-group generator cannot be selected there. Discovery requires416 order tests in the native exhaustive certificate.

The original full-order candidate fails closed and is preserved. The successor is separately frozen on every original fixture, with no pruning. The remaining p59,shift3,B16 case has14 available coordinates and stays underfilled. These are construction results; no IC speedup or scalar transfer is established.

- [Original failed run](full-order/RESULTS.md), [inspected PDF](full-order/RESULTS.pdf).
- [Maximal-order successor](maximal-order/RESULTS.md), [inspected PDF](maximal-order/RESULTS.pdf).
- [Reproduction](../../execution-v7/README.md).
- [Original failure and reporting gap](../../execution-v6/README.md).

All earlier full-campaign and external/full-IC/rho criteria remain open; final E28 labels32–63 remain unopened. Canonical validated pipeline and curve-map graphs are unchanged because this result supplies neither an admitted performance row nor a new scalar-preserving map.
