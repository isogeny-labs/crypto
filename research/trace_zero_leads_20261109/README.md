# Trace-zero leads beyond the initial full-point encoding

Two leads are implemented and locally checked: eliminate redundant full-point
witnesses on an admitted finite domain; and restrict the second trace bit to
the actual x subspace before eliminating an ordinate coordinate. Two further
lead cards specify complete quadratic relation chains and rank-aware presentation
selection. This advances the current implementation without claiming a new
ECDLP algorithm, global novelty or a measured solving gain [pcc:unverified].

For ECC2K-130's retained polynomial window V=span(z,...,z^16), the actual
quadratic bit q(x)=Tr(xH(x)) restricts to **zero**. Thus both full-kernel bits
are linear there: Tr(x)=0 by construction and Tr(y)=0. The matched compact
single-point model has146 variables,131 equations and3076 terms, all quadratic.
Its matched raw model has147 variables,132 equations and3108 terms, also
quadratic. These represent the same point set; no factor-base replacement or
yield gain is asserted [pcc:unverified].

| Requested work | Evidence delivered |
|---|---|
| Continue trace-zero integration | Opt-in certified finite-domain oracle and exact subspace-restricted models |
| Generate new leads | Four scoped cards in LEADS.json, mapped to reviewed atlas families |
| Check ECC2K-130 relevance | Actual degree131 models/circuits and public known-answer controls |
| Distinguish next steps | Complete chain builder, matched solving/performance and held-out selection remain open |

## Prior-art and source review

Sources checked (KB): no connector found and `scripts/research-kb status`
returns127, no executable. Before proposing leads, reviewed the current remote
[lead atlas](https://github.com/isogeny-labs/crypto-autoresearcher/tree/main/knowledge/leads/ecdlp)
and these four rows: decomposition-encoding-and-sat (review2026-11-07),
factor-base-subspace-choice (2026-10-30), cofactor-torsion-quotients (2026-11-01),
and prime-degree-algebraic-decomposition (2026-10-29).

The atlas already covers basis choice, trace filters, encoding changes and
their prior measured limits. The leads here are specific implementation
specializations and next checks in those existing families. A current lead
row is an evidence pointer, not a new independent verification of its cited
experiments. No sibling atlas disposition or research ledger is changed.

Primary sources checked: [Kosters–Yeo, Notes on Summation Polynomials](https://arxiv.org/pdf/1503.08001)
for the field-trace equation and the distinction between first fall and solving
degree; [Trimoska–Ionica–Dequen, AFRICACRYPT2020](https://pmc.ncbi.nlm.nih.gov/articles/PMC7334981/)
for SAT/XOR subspace decomposition; and
[Ri–Myong–Kim–Rim, trace and subtrace](https://arxiv.org/pdf/1304.0521),
definitions and Lemma3, for the second elementary symmetric trace form.
The production `is_in_4e` halving derivation and previous full-variety study
remain the membership foundation. A PMC follow-up encountered recaptcha;
the initial primary text was available. No new formal theorem is claimed.

## TZL-01: certify a finite domain once

Current explicit mode attaches an ordinate witness and the full point model
to each summand. But its finite x domain already comes from supplied points
individually admitted to T_n. For each allowed x, at least one such ordinate
exists. The other affine lift is its negation, also in T_n. The ordinate
witness variables occur only in that membership component; existentially
eliminating them preserves the S3 system projected onto all x wires.

`decompose_full_trace_zero_with_encoding(..., CertifiedDomain)` therefore:

- validates every actual supplied base point and the target;
- preserves precisely the same finite x domain and all identity patterns;
- omits only redundant full-point summand witness circuits;
- enumerates actual supplied base lifts by x, including sign restrictions,
  and replays the original group sum;
- retains Inconclusive on any resource cap and Refuted only on exhaustion.

This is a finite-domain certificate, not permission to omit membership from
an arbitrary algebraic subspace. The existing default remains ExplicitPoints.
The new API is opt-in; no CLI strategy changes. Admission runs once per
oracle call; a reusable cached certificate across calls is not implemented.

Actual matched circuit sizes with m3 and one allowed public G abscissa:

| n | Identity mask | Explicit variables | Certified variables | Explicit clauses | Certified clauses |
|---:|---:|---:|---:|---:|---:|
|83|0, all affine|73538|42413|215723|124334|
|83|1, identity intermediate|31540|415|91804|415|
|131|0, all affine|182090|104669|538547|309422|
|131|1, identity intermediate|78076|655|229780|655|

These are constructed circuits before extra full-point pins, not timings or
natural relation searches. The identity pattern is a special case and its
reduction is not used as a general performance ratio. At the same one-second/
2000-conflict cap, certified x-only identity controls are SAT at n83/131 and
replay G+(-G)+G=G. The old explicit n83 control remains UNKNOWN in the rerun.
Both modes also agree with exhaustive n5 group relations on bases {G},
{G,-G}, {G,2G} and {G,-G,2G}, including missing-sign rejection and multiple
distinct abscissae. The singleton controls establish
encoder behavior, not general relation yield [pcc:unverified].

**Next deciding check:** fixed non-singleton bases, unseen known-answer targets,
identical pattern order, seeds and cumulative budgets. Compare complete work
per successful replay, charging base admission and rejected models. Fewer
variables could also weaken propagation; that possibility remains open.

## TZL-02: restrict q before elimination

Define St(x)=sum_(i<j) x^(2^i)x^(2^j). For odd n,

~~~text
q(x)=Tr(xH(x))=Tr(x)+St(x),
B_q(x,z)=q(x+z)+q(x)+q(z)=Tr(xz)+Tr(x)Tr(z).
~~~

Derivation: the H term at exponent0 contributes Tr(x²)=Tr(x). Each remaining
even exponent2,4,...,n-1 chooses exactly one representative of a nonzero
difference modulo n up to sign, since n is odd. Taking traces therefore
counts every unordered Frobenius-conjugate pair once, giving St(x).
Polarizing the second symmetric polynomial gives the displayed B_q.
On V inside ker(Tr), this reduces to the trace pairing Tr(xz).
If its restriction is zero, q|V is linear, not necessarily zero.

The written expansion is source-backed algebra, with local checks rather
than a Lean proof. Integral certificate **IDC1h3f66989fcfd56337** records
S2(X+Z)-S2(X)-S2(Z)=(sum X)(sum Z)-sum(X_i Z_i) in16 formal coordinates,
degree2,32 issued plus32 replay points over2^61-1. It certifies that integral
expansion only, not the characteristic-two descent or every field-degree case.
Native tests independently compute conjugate-pair subtrace in small fields
and on public n83/131 fixture x values [pcc:unverified].

New production APIs `subspace_profile`, `affine_system_over_subspace` and
`affine_system_over_subspace_with_mode` preserve the exact ordered supplied
basis. They reject dependent, malformed or non-trace-zero inputs. The codec
rejects off-subspace points. If d=dim(V), the matched models use:

| Mode | Variables | Equations | Degree bound |
|---|---:|---:|---|
|Raw|d+n|n+1|2|
|Compact|d+n-1|n|3 generally;2 if q restricted is linear|
|Factored|d+n|n+1|2|

The x-trace row is implied by V in every row above. Compact eliminates y0
using Tr(y)=q(x); factored keeps a helper. Variables are Boolean, so these
are exact rational affine-point models plus a separate identity branch,
not positive-dimensional geometric defining ideals.

Constructed profile controls:

| n | d | q polar rank | q linear terms | q quadratic terms | dim(V·V) |
|---:|---:|---:|---:|---:|---:|
|83|16|0|0|0|31|
|83|32|0|1|0|63|
|83|64|48|4|56|83|
|131|16|0|0|0|31|
|131|32|0|0|0|63|
|131|64|0|1|0|127|

For d16 at n131, V is exactly the trace-zero intersection of the original
17-dimensional window span(1,z,...,z^16). The polynomial is
z^131+z^13+z²+z+1. These small windows are already isotropic; searching for a
different base is unnecessary for this example. At d64, q is the z^59
coordinate and is not zero. The degree83 polynomial is z^83+z^45+z²+z+1;
at d64 its nonzero polar rank preserves a counter-control to a general
quadratic-compaction claim. `models.json` includes every exact basis and
restricted ANF, plus d8 controls [pcc:unverified].

**Next deciding check:** actual matched relation systems, same allowed points
and targets, raw versus compact versus factored, with complete lifting and
identity coverage. Record solving degree and total successful work. Input
degree2 alone does not predict either quantity. The unrestricted260-variable
model describes another point set; it is not the baseline for146 here.

## TZL-03 and TZL-04: new follow-up constructions

**Complete quadratic full-point chains (proposed).** On a linear-q window,
the full summand model is quadratic. Introduce slope and partial-point
variables instead of eliminating ordinates into S3. For generic finite
addition in characteristic two, use

~~~text
(x1+x2)lambda=y1+y2,
x3=lambda²+lambda+x1+x2,
y3=lambda(x1+x3)+x3+y1.
~~~

All equations have Boolean coordinate degree at most2. A valid builder must
separately cover doubling, opposite points and identity inputs/outputs, and
enforce the generic nonzero denominator. Doubling satisfies
x1 lambda=x1²+y1. Dropping denominator guards or only retaining the generic
branch would be incomplete. Trace-zero intermediates stay in T_n, but their
x coordinates need not lie in the original V; do not impose V on them.
For m3,d16,n131 the summand point models alone use438 variables, already
more than the179-variable naive x-only chain layout before its trace-coordinate
reduction. The degree/variable tradeoff is the hypothesis. No branch-complete
builder or solver comparison is supplied in this round. S3 also admits
quadratic lifting, as the native AND/XOR circuit already demonstrates;
compare against that baseline too. Displayed input degree alone is not an
advantage. First acceptance is exhaustive n5/7 equivalence, then matched
algebraic solving.

**Rank-aware presentation selection (proposed).** Freeze V and use the actual
q profile and constructed model costs to select a presentation. At n83,d64,
compact has10646 AND auxiliaries and degree3; raw/factored have7328 and
degree2. At n131,d16 compact has2200 AND auxiliaries, versus2216 in the
matched raw/factored models. These are deterministic costs, not a cost theorem
for SAT/F4. A deciding protocol needs held-out targets/subspaces, every fixed
mode and a charged portfolio reference. The selector must pay construction
and failure costs. Altering V or searching normal bases is a separate test
with the prior atlas's size/null controls [pcc:unverified].

The addition formulas above are existing production point-addition algebra,
not a new group identity. The proposed lifted builder must preserve the
original verifier as its final acceptance check.

## Reproduction and validation

~~~sh
cargo test --locked --manifest-path research/trace_zero_leads_20261109/native/Cargo.toml -- --nocapture --test-threads=1
cargo run --locked --quiet --manifest-path research/trace_zero_leads_20261109/native/Cargo.toml
cargo run --locked --quiet --manifest-path research/trace_zero_leads_20261109/native/Cargo.toml --bin certify_subtrace_polarization
cargo test --locked --manifest-path research/full_trace_zero_variety_20261109/native/Cargo.toml -- --nocapture --test-threads=1
cargo check --lib --no-default-features
~~~

Seven new checks and ten original regression checks pass locally; the original
explicit n83 UNKNOWN is retained. The root production library type-check
passes. Both focused crates import the actual production modules through the
small dependency facade, without arithmetic replacements. Exhaustive model
checks compare every assignment with actual point enumeration at n3/5/7;
caps and invalid inputs do not become false refutations [pcc:unverified].

[Lead cards](LEADS.json), [constructed models](models.json),
[new checks](checks/new-tests.txt), [original regressions](checks/original-regressions.txt),
[root check](checks/root-check.txt), [commands/environment](checks/commands.txt),
[diagram](leads.svg), [PDF](report.pdf) and [source hashes](SOURCES.sha256)
carry the exact working scope. The previous study's hashes remain historical
at its recorded source revision; this study pins the changed production code.

Flow: prior-art review, both implementations, deterministic construction and
local checks completed; TZL-03/04 remain proposals; natural relation-finding
and matched performance are not attempted here. Experiment checkpoint/S3
credentials remain unavailable; no EKS or paid job was launched. The earlier
whole-root suite and unrelated benchrec limitations remain on PR3744.
No new signed independent proof or performance verdict is asserted.

The canonical index-calculus scoreboard, progress timeline and performance
graphs retain their prior values because no measured performance changed.
The old studies' diagrams remain historical; leads.svg shows only this new
construction and its scoped evidence. Atlas evidence additions are proposed
for the next cross-repository review in LEADS.json, mapped to existing families.
