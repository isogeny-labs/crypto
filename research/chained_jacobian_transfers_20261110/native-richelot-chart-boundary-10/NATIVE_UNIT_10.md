# Fixed F101 Richelot model charts: first native unit

All 12 frozen input models and their constructed monic output models have 120 rational points each in this unit. The complete point sets, both inverse roundtrips, independent character counts and 102 homogeneous projective arguments per model pass. All four tests and eight recorded phases exit 0; current source/qualifier guards, strict no-deps all-target Clippy and format pass. Source3eaf2d5733a9/library06fe86f1 was publicly tagged before compilation/calls. This is a finite curve-model component for the existing Richelot work; full divisor/Jacobian maps, dual orientation and costs remain open. Claim1422f5 is unsigned. [pcc:unverified]

| Literal fixture | a | b | Input/output points | Monic output coefficients, ascending |
| --- | ---: | ---: | --- | --- |
| source_Cpi1 | 0 | 31 | 120/120 | [56, 0, 85, 0, 84, 0, 1] [pcc:unverified] |
| source_Cpi2 | 2 | 15 | 120/120 | [26, 9, 50, 59, 49, 7, 1] [pcc:unverified] |
| source_C10 | 1 | 7 | 120/120 | [94, 59, 58, 7, 3, 82, 1] [pcc:unverified] |
| source_C20 | 1 | 28 | 120/120 | [75, 46, 74, 23, 6, 55, 1] [pcc:unverified] |
| target_Cpi1_shift1 | 2 | 20 | 120/120 | [37, 71, 4, 32, 40, 13, 1] [pcc:unverified] |
| target_Cpi1_shift2 | 2 | 16 | 120/120 | [1, 33, 98, 43, 17, 79, 1] [pcc:unverified] |
| target_Cpi2_shift1 | 0 | 28 | 120/120 | [77, 32, 0, 33, 80, 85, 1] [pcc:unverified] |
| target_Cpi2_shift2 | 0 | 28 | 120/120 | [77, 69, 0, 68, 80, 16, 1] [pcc:unverified] |
| target_C10_shift1 | 0 | 7 | 120/120 | [27, 95, 79, 43, 78, 56, 1] [pcc:unverified] |
| target_C10_shift2 | 0 | 7 | 120/120 | [27, 6, 79, 58, 78, 45, 1] [pcc:unverified] |
| target_C20_shift1 | 0 | 5 | 120/120 | [100, 46, 95, 50, 27, 12, 1] [pcc:unverified] |
| target_C20_shift2 | 0 | 5 | 120/120 | [100, 55, 95, 51, 27, 89, 1] [pcc:unverified] |

## Exact map and infinity cases

For the selected a,b with b²=H(a)≠0, the source uses u=1/(x−a), v=y*u³/b and F(u)=u⁶H(a+1/u)/b². Its inverse uses x=a+1/u, y=b*v/u³. Binomial coefficient construction is checked against independent homogeneous monomial evaluation at all102 normalized projective F101 arguments. The program rejects singular/degree-drop/noncanonical inputs and invalid chart parameters. [pcc:unverified]

The input points(a,±b) map to the two output infinities with leading ratios±1. Rational input infinities with ratio t map to(0,t/b), with the inverse returning ratio b*v. When the input leading coefficient is nonsquare there is no rational input infinity and no output point at u=0. Every rational point in both directions is retained in the trace, so removing the output infinity cases is detected. These statements describe the checked finite models, with no generic Lean curve-isomorphism or compiler theorem claimed. [pcc:unverified]

## Controls, retained evidence and orientation

Wrong coefficient/reversal/normalization, invalid points and malformed/singular/degree/parameter/search-exhaustion controls reject. The empty/restricted-candidate amendmentfc1a07 was published before implementation; no nonexistent smooth-curve obstruction is invented. Both b signs give legitimate curve maps and opposite y-images on every frozen input. This confirms that equation/roundtrip checks cannot select Richelot dual orientation. [pcc:unverified]

The entire493468B archive retains110 full files: exact source, compiler cache, separately copied immutable qualifier, all raw outputs/exits, guards and caller. Every member's bytes/SHA/mode are checked. The native stdout is287462 bytes with100 base-inverse,1224 projective,1440 forward,1440 reverse and1440 sign trace records. The original unit is not rerun for reporting; independent trace auditing is next. [pcc:unverified]

The output vectors meet the monic coefficient condition needed by the balanced code; carrying these charts through its divisor classes, Richelot factors, exceptional fibres and dual[2] is untested. The forced rational-return result in CT50 remains. Generic field/curve/library formalization, executable full Jacobian transfer, original LP populations, runtime/admission and matched setup/transfer/solve/recovery costs remain open. Paid STOP and unsigned standing remain unchanged. [pcc:unverified]
