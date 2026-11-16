#set page(paper:"a4",margin:18mm)
#set text(size:11pt)
= Joint source and kernel admission
Genus 3 · 15 November 2026
#image("flow.svg",width:100%)

The fixed cohort uses six primes 499,631,883,1039,2011,4003 and constants 35..50 in the actual source family. It produces 20 certified large-prime subgroups; 76 cases remain bounded unknown under the fixed four-place rule. All 20 admitted sources pass exact Weil recovery and independent polynomial coefficient checks.

#table(columns:2,inset:6pt,[Admission stage],[Cases],
 [Fixed source cohort],[96],
 [Certified large-prime subgroups],[20],
 [Bounded unknown],[76],
 [Independent Weil recovery],[20 / 20],
 [Necessary rational 3-kernel test],[7 pass / 13 fail],
 [Coprime order-13 primary pair],[1],
 [Primary pair and extension-cardinality fit],[1])

== Selected construction input
F631, constant49: N1=597, N=237185924 and ell=59296481. The recovered L coefficients are [1,−35,−229,26730,−144499,−13935635,251239591]. Its F3 primaries are T³+T²+2 and T³+2T+2; they are irreducible, coprime, reciprocal and satisfy T¹³=1 in each quotient. This matches the current builder's primary assumptions. No kernel is constructed by this admission stage.

Cardinality p¹³ fits u128 at the smaller three primes, not at1039 or above. This does not certify every arithmetic intermediate. Necessary-only passes do not establish kernel existence or builder support. All thirteen necessary failures and all other restrictions remain visible.

== Exact frozen execution
Complete source/binary freeze4acd0f46f2f0, checkpoint011953Z, preceded source execution. Source data committed599554daec20/checkpoint012054Z preceded Weil recovery; all lift data committed5bdc312b6490/checkpoint012127Z preceded kernel admission. Each stage ran once, statuses0 and stderr empty. A missing-delimiter controller build error and incomplete hash manifest are preserved as producer failures; correction and complete hashes preceded scientific execution.

The unique compatible source is passed to a separate frozen F631 actual-kernel gate. Its outcome is documented in that study. No admission count is a quotient map, DLP solve or speedup.

== Scope
Known eligibility checks, not novelty. Fixed family and selected fields are not generic random-curve evidence. Full quotient geometry, maps, charged comparisons and genus two/four remain. The mathematical tenfold objective is open; the earlier failed cold-cost comparison is unchanged. Raw certificates, exclusions, polynomials, protocols and hashes accompany RESULTS.md.
