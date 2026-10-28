# Round25 failed attempts and controls

Lean01 fails four elaboration goals because the untyped simp rewrite does
not rewrite the ring numeral2. The explicit typed equality (2:R)=0 replaces
that invocation. The failed source and output remain retained; sorryAx in
that rejected compile is not an accepted proof. No native or Lean failure
is interpreted as nonexistence. R24's sealed source/70 raw files are unchanged.

Deck25-positive-01 is rejected: the intermediate names n1,l1,n2,l2 collide
with Singular's short monomial exponent notation. Its purported involution
residuals are n^2+n,l^2+l, not composed images. Source residual/unit/kernel
invariance, the generic nu equation and both quotient coordinate certificates
all give zero, but this run is not accepted. Correct to distinct image/twice
names before the fresh positive attempt. Preserve the failed script/log.

The first6.3KiB checkpoint transport snapshot was accidentally staged with
source. Preserve its exact committed bytes and history. Subsequent mutable
transport uses checkpoint-live.tar.xz, untracked and explicitly unignored for
the checkpoint helper; never overwrite the committed initial snapshot.
