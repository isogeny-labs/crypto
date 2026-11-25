# Auxiliary factor bases reach native RR5 preparation

2026-11-25 · `auxiliary_adapter_v1` · Software qualification · PR #5142

The separate [Rust adapter](auxiliary-rr5/src/lib.rs) now passes auxiliary-curve
factor bases into the unchanged native monic-cofactor equation builder. It
preserves the sorted points and signed labels, and exposes both chained and
dense leaf constraints. **Six unit tests and the release library build pass.**
This reaches equation preparation; the chained leaf system is not yet a
candidate RR5 collector, and no relation solver ran. [pcc:unverified]

The unit chain is E': y²=x³+x over F31, G=(6,6), depth three and anchor G.
The primary target uses that same curve with its known order 32, for geometry
controls only. A second target, y²=x³+x+5, checks lifting filters; its unused
subgroup metadata is unset. Neither fixture is a prime-order recovery result.

| Control | Observed software behavior |
|:--|:--|
| Base order and labels | Eight primary points; anchor negation and kernel translation preserve the same sorted list |
| Leaf presentations | Both predicates match signed-base membership for every F31 affine pair on each of the two target curves |
| Poles and mutations | Isolated first/translated-step poles, changed intermediates, inverse-y and coefficient corruption are rejected |
| Actual target lifting | The filtering fixture retains fewer than eight points; its base polynomial is separate from the full degree-eight fiber |
| Native preparation | Adapter/direct calls produce the same equations, quartic and normalization multipliers; identity and target-in-base statuses propagate |
| Invalid inputs | Noncanonical values, malformed labels, wrong fields, singular targets and irregular fibers are rejected |

![The point list is connected to native preparation. The exported chained leaf system remains a separate input for later collector integration.](auxiliary-adapter-flow.pdf){width=96% height=3.0in}

\newpage

## Meaning of the matched leaf constraints

The full auxiliary fiber is N-chi D. The adapter separately forms
H(X), the monic product of X-x over the points that actually lift onto the
target curve with nonzero y. Positive labels use the smaller y; negative
labels use its negation. The unchanged native builder receives that exact
point order, so its relation indices retain their meaning.

The dense leaf system uses H(x)=0, the target equation and y*s-1=0.
The chained system uses the same target/nonzero-y equations and, for each
translated map, the existing rational identity

\[
 z(x_{i+1}-z-a_i)-b_i=0,\qquad z=x_i-r_i,
\]

with the last image fixed to chi. Since b_i is nonzero, this equation has no
solution at its pole. Each map equation is quadratic; the target equation is
cubic. The source and protocol supply the convention and explicit variable
order. This is an implementation of the established map construction, with
no new checked identity certificate or Lean formalization.

**The tested equivalence is on Fp assignments.** A target nonresidue at a
fiber x can acquire a y over the algebraic closure in the chained system,
while H excludes that x. Equality of ideals over the algebraic closure is
not asserted. A future solver comparison must handle that distinction and
charge any field-membership enforcement or extra components. Small map degree
alone supplies no solver-cost conclusion.

The native boundary validates curve geometry and chart inputs. It does not
certify the descriptor's subgroup order/generator; the existing recovery
consumer must do that separately. Four native preparation calls ran in the
unit test: two successful constructions and two unsupported-chart returns.
No F4/msolve, base-selection panel at F127/F8191, relation collection or
scalar recovery ran. There is no speed ratio in this component.

## Evidence and next integration step

```text
source: 1ab4844db046a65dabec9ce962c070ac18709e6a
claim:  PCC-20261125-52e108
```

The [claim](../../pcc/claims/PCC-20261125-52e108.json) binds the test/build logs
and [build identity](evidence/auxiliary_adapter_v1/build_identity.json), which
names 34 source/input files and the library/test artifacts. It remains unsigned
`not_covered`; local checks find no unexpected proof reasons. The 1,912-entry
PCC ledger chain passes locally. These are producer-local checks. [pcc:unverified]

The suite ran once, with six passes, zero failures and zero doc tests. Existing
dependency warnings are retained. Source-publication review caught an ignored
nested Cargo.lock; the exact used lockfile was explicitly committed before
the identity and claim. No code, dependency resolution or passing test was
changed or repeated. All 20 prior semantic-guard and seven auxiliary-library
source/input pins remain unchanged.

The [protocol](AUXILIARY_ADAPTER.md) and [reproduction guide](AUXILIARY_ADAPTER_REPRODUCE.md)
retain exact scope and commands. PKC 2016 Sections 3.2/3.4 and the actual
Isogeny Labs preparation/consumer APIs are the baseline; KB status and two
queries remain unavailable with exit 127 and a retained receipt. No novelty
or comparison against a newly timed external implementation is claimed.

Next, integrate the candidate leaf/quotient constraints into a bounded RR5
collector and admit a matched experiment. Charge generator search, chain
construction, coset enumeration, lifting, labels/indexing, both polynomial
presentations, serialization, relation attempts, verification, deduplication,
linear algebra and scalar replay. The earlier corrected-output execution
requirements remain open. PR #5142 stays draft pending the required eleven-file
artifact migration. The Cloudflare receiver is unconfigured; 713 records
remain local and no sender runs. Existing performance figures are unchanged.
