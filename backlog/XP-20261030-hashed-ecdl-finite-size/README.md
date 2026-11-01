# Finite-size hashed-base relation search (draft, not runnable)

Question: do inverse-hash selector inputs and a structured witness backend
lower full useful-relation cost against the current native hashed-base IC
baseline? See the [source review](../../research/hashed_ecdl_finite_size_20261030/README.md).

Native selector-domain search/encoding, width-aware exact contraction,
witness propagation, partial collection and binary/orbit components are now
implemented; see the [implementation receipt](../../research/hashed_ecdl_finite_size_20261030/implementation/README.md).
Their development checks do not complete this measurement card. Preserve a separate
source-counting arm and independent curve/hash/group replay. A later frozen
study could start with public known-answer 16--40-bit cases, specified by
complete field/curve/subgroup/target identities, and expand only after
correctness and cost gates pass. This size interval is a proposal, not an
empirical feasibility claim or a frozen panel. Planted targets check
correctness; natural random relation queries are needed for yield/cost.

Before ready: implement the native measurement driver and independent checker; define
seeded cases and a development/holdout split; freeze successes, failures,
useful matrix rank, phase costs, limits and stopping rules; pin merged
code; name the strongest current IC/external solver and symmetry-aware rho
references; establish checkpoint/readback, authenticated image/execution and
measurement admission on the actual host. The existing frozen native A/B
panel is a distinct study and must not be replaced by this one.

No resource estimate is claimed: graph widths, decoder cost, invalid-code
frequency and solver work are not yet measured. The card's one-CPU/4-GB,
1800-second fields are an unapproved placeholder envelope, not an estimate
or launch authorization. Funding zero keeps this unready card below the
funded backlog; no cairn objective or job is posted. TODO command values
deliberately prevent accidental use as an existing executable.

Done for a future study means independently checked relations/scalars,
complete costs and failure records on matched cases, a qualified comparison
against current IC and the strongest references, and retained reports. A
negative comparison is still a result. Neither a stage win nor a lower-bit
panel closes ECCp-131, ECC2K-130 or the full audit.
