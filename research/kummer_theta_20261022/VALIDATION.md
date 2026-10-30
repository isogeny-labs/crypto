# Validation and interpretation boundary

The producer searches all 20 ordered finite-root pairs, retains the 10 pairs
with square leading coefficient, and tries all six orders of the other
three roots. For every order it solves two quadratic equations in Fp, tries
both signs and checks the five defining fast-Kummer equations from
[Chung, Costello, and Smith, section 7.1](https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/recovery.pdf).
It also rejects zero projective coordinates and the quartic denominators
`ad-bc`, `ac-bd`, `ab-cd`.

The separate `validate_theta` binary constructs a complete square-root
lookup table for each Fp, independently re-enumerates the bounded family,
checks every candidate against the original equations by cross
multiplication, and compares all four per-model counts to the recorded raw
rows. It checks the selected tuple, the stored `A,B,C,D`, root images, and
square scale. Its recorded output is `validated_models=21
validated_theta_tuples=1680 lookup_replay=pass
selected_original_equations=pass`.

The 21/21 finding applies only to the fixed affine Rosenhain family and
base-field constants. It does not assert that these tuples are ready-to-use
fast Kummer coordinates: projection, theta-model isomorphism, xADD/xDBL,
signed recovery, exceptional-case arithmetic, and label-carrying walk
integration have not been implemented or priced. No candidate/rho ratio
is assigned. The prior reference gate's 1,260 target checks are a separate
group-identity invariant; they are not a fast-Kummer DLP solve cohort.
