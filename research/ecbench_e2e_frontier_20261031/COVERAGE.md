# Live implementation coverage

Pinned commit: `48c8a4767d2d1eed6a7b379f74410bc729239380`. Registry: **74 methods**; nested solver registry: **11 backends**.

A disposition is not a performance verdict. `blocked` means no fresh EKS record was produced in this workspace.

## Summary

| disposition | methods |
|:--|--:|
| `current_primary` | 13 |
| `diagnostic_only` | 10 |
| `legacy_control` | 21 |
| `pricing_sensitivity` | 6 |
| `specialized_domain` | 24 |

## ECBench methods

| # | method | family | applies | disposition | campaign cell | fresh run |
|--:|:--|:--|:--|:--|:--|:--|
| 1 | `ic.aps_v1` | ic | prime_only | specialized_domain | prime | blocked_no_authenticated_eks_route |
| 2 | `ic.three_leads_genus2_pencil_symbolic_v2` | ic | any | specialized_domain | three-leads/transfer | blocked_no_authenticated_eks_route |
| 3 | `ic.three_leads_genus2_choice2_v1` | ic | any | specialized_domain | three-leads/transfer | blocked_no_authenticated_eks_route |
| 4 | `ic.three_leads_genus2_projective_chart_v1` | ic | any | specialized_domain | three-leads/transfer | blocked_no_authenticated_eks_route |
| 5 | `ic.three_leads_genus2_target_rank_v1` | ic | any | specialized_domain | three-leads/transfer | blocked_no_authenticated_eks_route |
| 6 | `rho.three_leads_plain_v1` | rho | any | specialized_domain | three-leads/transfer | blocked_no_authenticated_eks_route |
| 7 | `rho.three_leads_negation_v1` | rho | any | specialized_domain | three-leads/transfer | blocked_no_authenticated_eks_route |
| 8 | `rho.three_leads_jacobian_v1` | rho | any | specialized_domain | three-leads/transfer | blocked_no_authenticated_eks_route |
| 9 | `ic.three_leads_genus2_v2` | ic | any | specialized_domain | three-leads/transfer | blocked_no_authenticated_eks_route |
| 10 | `ic.three_leads_genus2_v1` | ic | any | legacy_control | three-leads/transfer | not_globally_rankable |
| 11 | `ic.three_leads_decomposition_v2` | ic | any | specialized_domain | three-leads/transfer | blocked_no_authenticated_eks_route |
| 12 | `ic.three_leads_decomposition_v1` | ic | any | legacy_control | three-leads/transfer | not_globally_rankable |
| 13 | `rho.plus_model_v1` | rho | prime_only | diagnostic_only | prime | not_globally_rankable |
| 14 | `rho.best2_model_v1` | rho | prime_only | diagnostic_only | prime | not_globally_rankable |
| 15 | `rho.frozen_reference` | rho | any | legacy_control | prime + binary, split before ranking | not_globally_rankable |
| 16 | `rho.plain` | rho | any | legacy_control | prime + binary, split before ranking | not_globally_rankable |
| 17 | `rho.negation` | rho | any | legacy_control | prime + binary, split before ranking | not_globally_rankable |
| 18 | `rho.negation_strong` | rho | any | specialized_domain | prime + binary, split before ranking | blocked_no_authenticated_eks_route |
| 19 | `rho.multitarget_pooled` | rho | prime_only | diagnostic_only | prime | not_globally_rankable |
| 20 | `rho.preprocessing` | rho | prime_only | diagnostic_only | prime | not_globally_rankable |
| 21 | `ic.k4_gaudry` | ic | any | specialized_domain | transfer/Jacobian | blocked_no_authenticated_eks_route |
| 22 | `rho.cubic` | rho | any | legacy_control | prime + binary, split before ranking | not_globally_rankable |
| 23 | `rho.quartic` | rho | any | legacy_control | prime + binary, split before ranking | not_globally_rankable |
| 24 | `rho.frobenius_orbit` | rho | any | specialized_domain | prime + binary, split before ranking | blocked_no_authenticated_eks_route |
| 25 | `rho.gls` | rho | any | specialized_domain | prime + binary, split before ranking | blocked_no_authenticated_eks_route |
| 26 | `rho.signed_frobenius` | rho | koblitz_only | legacy_control | binary/Koblitz | not_globally_rankable |
| 27 | `rho.signed_frobenius_strong` | rho | koblitz_only | legacy_control | binary/Koblitz | not_globally_rankable |
| 28 | `rho.signed_frobenius_strong_legacy` | rho | koblitz_only | legacy_control | binary/Koblitz | not_globally_rankable |
| 29 | `rho.signed_frobenius_strong_escape` | rho | koblitz_only | current_primary | binary/Koblitz | blocked_no_authenticated_eks_route |
| 30 | `bsgs.textbook` | bsgs | any | legacy_control | prime + binary, split before ranking | not_globally_rankable |
| 31 | `bsgs.interleaved` | bsgs | any | legacy_control | prime + binary, split before ranking | not_globally_rankable |
| 32 | `bsgs.negation` | bsgs | any | legacy_control | prime + binary, split before ranking | not_globally_rankable |
| 33 | `bsgs.negation_table` | bsgs | any | diagnostic_only | prime + binary, split before ranking | not_globally_rankable |
| 34 | `kangaroo.vow` | kangaroo | any | legacy_control | prime + binary, split before ranking | not_globally_rankable |
| 35 | `transfer.isogeny` | rho | any | specialized_domain | transfer/Jacobian | blocked_no_authenticated_eks_route |
| 36 | `transfer.quartic_cover` | rho | any | legacy_control | transfer/Jacobian | not_globally_rankable |
| 37 | `transfer.quartic_cover_v2` | rho | any | specialized_domain | transfer/Jacobian | blocked_no_authenticated_eks_route |
| 38 | `transfer.ghs_cover` | rho | any | specialized_domain | transfer/Jacobian | blocked_no_authenticated_eks_route |
| 39 | `ic.hyperelliptic` | ic | any | legacy_control | transfer/Jacobian | not_globally_rankable |
| 40 | `ic.hyperelliptic_v2` | ic | any | specialized_domain | transfer/Jacobian | blocked_no_authenticated_eks_route |
| 41 | `claw.pair_table` | claw | koblitz_only | legacy_control | binary/Koblitz | not_globally_rankable |
| 42 | `ic.cm_folded_native_v1` | ic | any | legacy_control | CM-folded | not_globally_rankable |
| 43 | `ic.cm_folded_recurrence_v1` | ic | any | legacy_control | CM-folded | not_globally_rankable |
| 44 | `ic.cm_folded_native_d43_v1` | ic | any | legacy_control | CM-folded | not_globally_rankable |
| 45 | `ic.cm_folded_native_v2` | ic | any | specialized_domain | CM-folded | blocked_no_authenticated_eks_route |
| 46 | `ic.cm_folded_recurrence_v2` | ic | any | specialized_domain | CM-folded | blocked_no_authenticated_eks_route |
| 47 | `ic.cm_folded_native_d43_v2` | ic | any | specialized_domain | CM-folded | blocked_no_authenticated_eks_route |
| 48 | `ic.hyperelliptic_thread` | ic | any | specialized_domain | transfer/Jacobian | blocked_no_authenticated_eks_route |
| 49 | `ic.pipeline` | ic | any | current_primary | prime + binary, split before ranking | blocked_no_authenticated_eks_route |
| 50 | `ic.pipeline_full` | ic | binary_only | pricing_sensitivity | binary/Koblitz | not_globally_rankable |
| 51 | `rho.signed_frobenius_full` | rho | koblitz_only | pricing_sensitivity | binary/Koblitz | not_globally_rankable |
| 52 | `rho.signed_frobenius_strong_full` | rho | koblitz_only | pricing_sensitivity | binary/Koblitz | not_globally_rankable |
| 53 | `ic.pipeline_wordram` | ic | binary_only | pricing_sensitivity | binary/Koblitz | not_globally_rankable |
| 54 | `rho.signed_frobenius_wordram` | rho | koblitz_only | pricing_sensitivity | binary/Koblitz | not_globally_rankable |
| 55 | `rho.signed_frobenius_strong_wordram` | rho | koblitz_only | pricing_sensitivity | binary/Koblitz | not_globally_rankable |
| 56 | `ic.pipeline_counted` | ic | any | diagnostic_only | prime + binary, split before ranking | not_globally_rankable |
| 57 | `ic.tensor_contraction_v1` | ic | prime_only | diagnostic_only | prime | not_globally_rankable |
| 58 | `ic.known_log_mitm2_v1` | ic | any | diagnostic_only | prime + binary, split before ranking | not_globally_rankable |
| 59 | `ic.known_log_ksum_v1` | ic | prime_only | diagnostic_only | prime | not_globally_rankable |
| 60 | `ic.large_prime` | ic | binary_only | legacy_control | binary/Koblitz | not_globally_rankable |
| 61 | `ic.trace_line_2lp` | ic | binary_only | specialized_domain | binary/Koblitz | blocked_no_authenticated_eks_route |
| 62 | `ic.shared_rank` | ic | koblitz_only | diagnostic_only | binary/Koblitz | not_globally_rankable |
| 63 | `ic.koblitz_descent` | ic | koblitz_only | specialized_domain | binary/Koblitz | blocked_no_authenticated_eks_route |
| 64 | `rho.negation_v2` | rho | any | current_primary | prime + binary, split before ranking | blocked_no_authenticated_eks_route |
| 65 | `rho.cubic_v2` | rho | any | current_primary | prime + binary, split before ranking | blocked_no_authenticated_eks_route |
| 66 | `rho.quartic_v2` | rho | any | current_primary | prime + binary, split before ranking | blocked_no_authenticated_eks_route |
| 67 | `rho.signed_frobenius_v2` | rho | koblitz_only | current_primary | binary/Koblitz | blocked_no_authenticated_eks_route |
| 68 | `rho.signed_frobenius_strong_v2` | rho | koblitz_only | current_primary | binary/Koblitz | blocked_no_authenticated_eks_route |
| 69 | `bsgs.textbook_v2` | bsgs | any | current_primary | prime + binary, split before ranking | blocked_no_authenticated_eks_route |
| 70 | `bsgs.interleaved_v2` | bsgs | any | current_primary | prime + binary, split before ranking | blocked_no_authenticated_eks_route |
| 71 | `bsgs.negation_v2` | bsgs | any | current_primary | prime + binary, split before ranking | blocked_no_authenticated_eks_route |
| 72 | `kangaroo.vow_v2` | kangaroo | any | current_primary | prime + binary, split before ranking | blocked_no_authenticated_eks_route |
| 73 | `claw.pair_table_v2` | claw | koblitz_only | current_primary | binary/Koblitz | blocked_no_authenticated_eks_route |
| 74 | `ic.large_prime_v2` | ic | binary_only | current_primary | binary/Koblitz | blocked_no_authenticated_eks_route |

## Nested algebraic solver backends

These are implementations inside `ic.pipeline`, not independent ECBench method IDs. Their native counters have different meanings; only matched E2E wall or a complete common priced unit may rank them.

| backend | native unit / accounting boundary | fresh run |
|:--|:--|:--|
| `f4-f2` | word XORs for elimination; incomplete whole-engine count | blocked_no_authenticated_eks_route |
| `buchberger-f2` | polynomial reductions; backend-native unit | blocked_no_authenticated_eks_route |
| `matrix-f4` | partial word operations; matrix build excluded | blocked_no_authenticated_eks_route |
| `matrix-f5` | partial word operations; matrix build excluded | blocked_no_authenticated_eks_route |
| `inherited-f4` | partial word operations; child specialisation excluded | blocked_no_authenticated_eks_route |
| `crossbred-f2` | partial word operations | blocked_no_authenticated_eks_route |
| `xl-f2` | modelled monomial operations | blocked_no_authenticated_eks_route |
| `sat-cdcl` | conflicts; CNF or native-XOR encoding | blocked_no_authenticated_eks_route |
| `fes-f2` | Gray-code word XORs; quadratic systems | blocked_no_authenticated_eks_route |
| `fes-f2-wide` | AVX2/AVX-512 vector XORs; at most 32 equations | blocked_no_authenticated_eks_route |
| `exhaustive` | short-circuited monomial tests | blocked_no_authenticated_eks_route |
