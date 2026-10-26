# The `rho.quartic` / `rho.cubic` class set-up: what `verify` admits

Binaries built before `9ead78cf0` (2026-10-18) charged the class set-up of
`rho.quartic` and `rho.cubic` to the `setup` phase but not to `total_gae` or
`S`. (The class set-up is the scalar multiplication that fixes ι or ζ₃ on
the subgroup.) Their records fall short of the phase sum by exactly the
set-up. Since the fix, the total carries it and every record writes
`detail.class_setup`.

`ecbench verify` compares a replay with its record bit for bit. A pre-fix
record cannot match a current replay, so `verify` reconstructs the figure
the old binary wrote from the replay's own set-up. It does this only when
all of these hold (`audit::pre_class_setup_record`):

1. the method is `rho.quartic` or `rho.cubic`;
2. the record has no `detail.class_setup`, so it was not written by a
   fixed binary;
3. the record's `binary_sha256` is one of the producers below.

Every other mismatch fails the replay. That includes a record from the
current harness that is short by its class set-up
(`tests/ecbench.rs`, `a_current_record_undercharged_by_its_class_setup_fails_replay`).
The replay reports a reconstructed record as
`identical (pre-2026-10-18 phase accounting, reproduced from the replay)`.

## The producers

The list was found by scanning every ecbench session in this repository and
in `cryptanalysis`, `crypto-autoresearcher` and `ml-cryptanalysis`, in every
storage form. Each binary was built from a commit without
`charge_class_setup`.

| binary (sha256) | built from | sessions |
|:--|:--|:--|
| `0a8a48d7580df43d…eb1473b6` | `c9db37d01` | `research/prime_rho_automorphism_20261015/sessions/{quartic,cubic}` |
| `1a87842e67135807…c1967505fa` | `466035c18` | `research/drastic_prime_conductor_gap_round2_20261012/sessions/d3-*-cubic` |
| `d24c224771974018…f56ade6acf` | `7fa17f92e` | `research/drastic_prime_conductor_gap_round3_20261012/sessions/{d4-*-auto,d3-*-cubic}` |
| `ac4bc5b26f858a4d…360735fb35` | `d13f5259c` | `research/volcano_ecdlp_census_20261017/sessions/V3-*.tar.xz`, `research/volcano_ecdlp_census_followup_20261023/sessions/V5-*.tar.xz` |
| `7da27ec3f0dadbae…b4b1fa7a9f` | `3d191d806` | `research/isogeny-volcano-cost_20261017/sessions/{j0,j1728}/spec-crater-aut` |

The full digests are in `audit::PRE_CLASS_SETUP_BINARIES`. The list is
closed. A pre-fix record found later, from another binary, fails its
replay until that binary is added here and in the code with its evidence.

The ecbench CI job replays a sample of four of these sessions on every
pull request and requires the reconstruction to be what passed.

## Impact

The corrected headline ratios of every affected study, and what changes in
each, are in
[`research/class_setup_impact_20261024/`](../../research/class_setup_impact_20261024/README.md).
In short:

- The prime automorphism study's quartic control no longer matches its
  asymptotic prediction (0.819 [0.722, 0.929] against 0.707).
- The volcano census's j = 0 control sits 3–6% above 1/√3 rather than on
  it.
- The census's IC / `rho-cubic` gap on the V3 crater is about 81×, not
  85×.
- The 40-bit conductor-gap rows move by under 0.03%.

## Kangaroo re-placements

Before 2026-10-18 the same binaries also charged a kangaroo's re-placement
after a same-kind collision to `setup` instead of `search`. The total was
the same. No committed `kangaroo.vow` record has a same-kind collision
(`counters.same_kind_collisions`), so no reconstruction is admitted for it.
Every kangaroo record replays exactly. The pinned control,
`generic::tests::a_kangaroo_restart_is_charged_to_search`, runs on an 11-bit
curve where the wild kangaroo re-places on every seed.
