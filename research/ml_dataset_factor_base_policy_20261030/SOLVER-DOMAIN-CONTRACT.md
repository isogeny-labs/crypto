# Proposed native solver domain clarification

This is a proposed V2 activation clarification, not a new scientific claim.
Preserve all existing method/family/arity/modulus/work limits. Dispatch domain
handling by factor-base schema: explicit legacy policy in a V2 plan still uses
the historical V1 domain construction and bytes.

For signed factor-base/v2, independently check its public mathematical definition
and preserve the exact ordered point population. Do not append negations,
reorder, reproject, or reconstruct a legacy factor base. Derive distinct x values
in numeric order and a complete x-to-retained-point-index map. Existing lift()
already enumerates every retained point matching an x; no root becomes an
accepted PDP without group replay.

| Adapter | Actual domain | Signed V2 empty source | Existing scope remains |
|---|---|---|---|
| enumerate/pdp | Ordered signed population, repeated summands allowed | Censored before search; no negative PDP claim | All supported families, m2..15, transition cap |
| bsgs | Whole selected cyclic subgroup; factor base unused | **Admit** independent scalar search; retain empty construction separately | Existing subgroup/order/work caps; no PDP-domain claim |
| sat | Distinct retained x values, complete signed group lifts | Censored before circuit/search | Binary/Koblitz only; existing native chain bounds/pattern caps |
| weil, weil-descent, Boolean f4/f5 | Exact finite selector over distinct retained x; full signed lifts | Censored before encoding/search | Binary/Koblitz only, m2, <=64 distinct x |
| prime f4 | Exact retained x leaves; same unrestricted affine auxiliaries | Censored before algebra/search | Existing small-prime m2..15; wide supervised m2; no general negative affine-chain claim |
| f6 | Retained signed points and native geometric four-sum | Censored before pair table | Binary/Koblitz only, m4; not Groebner F6 |
| root | Retained x query/filter domain, complete signed lifts | Censored before root search | Existing prime/binary m2 limits and checked certificates |
| pkm | Separately reconstructed native Kummer population; g chosen from minimum nonzero source x | Typed unsupported: no source abscissa exists to define g | Prime only, m2/3, p<2^31; separate domain can contain points outside selected subgroup |
| double-large-prime | Exact retained small population plus explicitly disclosed residual population and native relation columns | Censored pending explicit empty-small-base admission contract | Existing Koblitz orbit backend/generic backend selection and limits |

The BSGS exception corrects the earlier blanket empty-source suppression; an
unneeded factor base must not suppress independent subgroup recovery. PKM's
alternate domain is not fabricated when its current constructor lacks a source
abscissa. No route silently falls back to legacy factor-base construction.

The new public math-only solver-domain/v2 artifact binds the source population
and its exact order, numeric distinct x and complete lift incidence, checked
negation/Frobenius actions/orbits, and original source-domain digest. A
source_domain_usage label distinguishes exact PDP population, unused BSGS input,
PKM source for an alternative population, and large-prime small population.
PKM's native artifact remains authoritative for its alternative columns/points;
large-prime artifacts remain authoritative for residual/column presentations.
Do not mislabel native relation compression as the source population's own
expanded action-map representation.

Outcome labels must distinguish actual domain scope, source population use,
constructor terminal, and solver completion. intended_domain_unsat_claim is
always false; native negative statements remain restricted to the actual
retained finite domain when their existing checks support them. Constructor
terminal comes from independently replayed restricted history in the worker;
solver adapters never read the private witness or history. Public domain and
scope replay rejects recomputed-hash substitutions independently of custody.

SAT retained-gate-map producer work is a separate dependency; this migration
does not overwrite or silently integrate that donor patch.

Outcome dependency metadata uses factor_base_required_by_solver, not a claim that an unsupported adapter actually ran. BSGS retains false here even for an empty source; empty source remains inadmissible as a PDP feature.
