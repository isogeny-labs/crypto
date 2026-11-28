# No extra-automorphism curve in the selected isogeny class

The `j=0` comparison has many easy three-term orbit rows, but its group
order differs from the selected `E:y²=x³−3x+40`. One could still ask
whether **another** `j=0` twist, reached through a chain of isogenies,
has the selected order and carries that automorphism. The selected
Frobenius trace rules this out without enumerating twists or isogenies.

For `p=2^61−1` and the certified selected order
`N=2305843009662956113`, the trace is

`t=p+1−N=−449262161`.

The [native integer check](native/examples/quotient_order.rs) computes

`(4p−t²)/3=3007178515849461961`,

whose integer square root is `1734121828` with nonzero remainder. If an
ordinary curve with `j=0` lay in the same `F_p`-isogeny class, its
Frobenius would lie in `Q(√−3)` and its characteristic discriminant
would have the form `t²−4p=−3v²` for some integer `v`. The nonsquare
above contradicts that necessary condition. Here `p≡1 (mod 3)`, so
`j=0` curves are ordinary in this characteristic.

For `j=1728`, `p≡3 (mod 4)`. Such curves over `F_p` are supersingular
and have trace zero for `p>3`, whereas the selected trace is nonzero.
In characteristic greater than three, elliptic curves with geometric
automorphisms beyond `±1` have `j=0` or `j=1728`. Consequently **no
curve in the selected `F_p`-isogeny class has an extra geometric
automorphism** of those types. This excludes transferring the CM orbit
relations to the selected hard subgroup by any `F_p`-isogeny chain. It
does not rule out a different algebraic correspondence or an
index-calculus relation locator that does not use such automorphisms.

Reproduce the trace and exact integer square check:

```sh
cargo run --release --offline --manifest-path research/m61_root_relation_20261126/native/Cargo.toml --example quotient_order
```
