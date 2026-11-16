#set page(margin: 1.8cm)
#set text(size: 10pt)
= R6.3 — the trace pair as a label of $E slash 4E$

Curve $y^2 + x y = x^3 + 1$ over $FF_(2^n)$, $n$ odd. Toy and sampled fields; no security claim.

*Verdict: confirmed.* The pair $(op("Tr")(x), op("Tr")(y + x H(x)))$ is constant on every coset of $4E$ for all odd $n$ in 5..25 (44,740,508 points, exhaustive) and on 10,000 sampled points at each of $n in {131, 233, 239, 283, 409, 571}$. Labels: $k=0 -> 00$, $k=2 -> 01$, $k=1 -> (1, (n+1)/2 mod 2)$, $k=3 -> (1, (n-1)/2 mod 2)$ — periodic in $n mod 4$, which explains the round-4 swap ($131 equiv 3$, $233 equiv 1$).

#figure(image("labels.svg", width: 100%), caption: [Measured label per coset vs $n$ (mod 8 in parentheses).])

*Lemma R6.3.* The key step: for odd $P eq.not plus.minus P_4$, $b(P) = b(P + P_4) + (n-1)/2$, from $x_Q = m^2 + m$, $m = l + H(x)$, the certified curve identity $x(l^2+l)+l = g^2+g+1$, and $op("Tr")(H(x)) = (n+1)/2$ when $op("Tr")(x)=1$. Full proof in RESULTS.md. $L$ is a set bijection $E slash 4E -> FF_2^2$, not a homomorphism: $ZZ slash 4$ has no injective additive map into an exponent-2 group.

*Certificates.* The Boolean 4E identity does not fit the polynomial `identity.certificate/v1` schema; its two polynomial cores were issued and accepted by the library checker.

#figure(image("cost.svg", width: 100%), caption: [Constant-time vs table 4E test (members of 4E). Wall is contended.])

*Constant-time cost.* 2.5–5.0× wall (contended), 2.0–3.7× counted word ops vs the table version: *does not meet the < 2× bar*.
