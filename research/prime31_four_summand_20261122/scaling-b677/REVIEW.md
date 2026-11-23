# Review scope and resolved findings

This is a same-team source-review record, not a signed verdict or formal proof.
The final executed bytes are identified by the immutable build/run inventories.

## Mathematical and construction review

The reviewers checked the four-leaf anti-Frobenius exclusion under full odd
rational-group order, split squarefree selector, distinct abscissae and finite
target outside the selector. The denominator and tangent-space arguments retain
their stated conditions. The argument does not transfer automatically to more
than four leaves. [pcc:unverified]

Every modular gate enforces all seven coefficients of AB-UQ-C. Monicity gives
unit pivots for its quotient and remainder variables, including degree drops.
The final gate has only three quotient auxiliaries and seven equations; it adds
exactly the four remainder constraints. The D4 recurrence, eight squarings,
four-product f^7 chain and four-product blocked G evaluation were reviewed.
The independent dense-selector and mutation controls then passed in the final
32-test native run. [pcc:unverified]

## Native guard review

The production file was independently source-reviewed at SHA256
`a6a3ec89d2278e40553ab68fa52bb81d69a78510ce66b1bb163ec1dcac444843`.
Review covered pair candidate admission, symbolic support/rows, matrix and
recording workspace, retained basis terms, final interreduction and resource
stop propagation. The default remains None; the original default algorithms
and omitted optional-field serialization remain selected without caps.

These are per-native-call admission counts. Caller inputs, preserved generators,
exponent widths, root buffers and allocator overhead are not a complete process
memory bound. The fixed external address-space limit remains required.
All eight new public-API guard controls passed in both recorded native suites.
[pcc:unverified]

## Findings resolved before the final run

- The extractor's trace guard originally could mark an already interrupted
  resource/deadline result as a refused univariate-free fallback. It now lets
  those native outcomes terminate with their own status. The zero-budget B677
  integration controls require explicit ResourceLimit with no branch, callback
  or fallback count.
- The first fixture source trusted the wrong historical eighth-coset spelling.
  Native admission rejected it. The independent scalar control established
  1852957275; the new fixture/test expectations were corrected while the wrong
  value remains a negative control. Exact failed source bytes and the 27/5
  outcome are preserved. [pcc:unverified]
- Dependency cfg(test) modules would not execute through the source closure.
  The qualifier now selects the complete anchored production guard-test module
  verbatim, with no assertion rewriting. The actual solver is still the full
  production source; unrelated legacy dependency tests remain outside scope.

## Remaining scope

The local full-root build did not produce a test executable. Its dependency
allocation error and outer timeout are retained independently of the scoped
passes. The registered B677 extraction requests have not run. No B677 relation,
rank, recovery, full-cost or speed conclusion follows from this review or the
constructor controls. [pcc:unverified]
