# Exact direct Hess genus-four models and evaluated maps

27 October 2026. These are the normalized Magma outputs retained in the four `attack*_receipt.json` files. `a` is a root of `a^4+a+1` over GF(2); `w` is a root of `w^12+w^7+w^6+w^5+w^3+w+1`. All four source models are `Y²+XY=X³+w^5 X²+b` over `GF(4096)`, with `b` and source points in polynomial-basis integer encoding. The triples are reduced Mumford representatives in the corresponding genus-four Jacobians.

## Order 4006, `b=198`, `r=2003`

```text
G=(199,2783), Q=(738,3018)
C: y² + (a^8 x^4 + a^10 x² + a^11 x + a^4)y
   = a^5 x^9 + a^4 x^8 + a^9 x^5 + x^4 + a^11 x³
     + a² x² + a^14 x + a^4.
#Jac(C)(k)=88132=44·2003.
h(G)=(x^4+a³ x²+a^6 x+a^8,
      a^6 x²+a^12 x+a^13, 4)
h(Q)=(x^4+x³+a^10 x²+a^9 x+a^12,
      a^10 x³+a³ x²+a^4 x+a^11, 4)
```

The [exact Magma receipt](attack_4006_receipt.json) verifies `[r]h(G)=0`, a full-rank 22-row relation solve, `h(Q)=[7]h(G)`, and `Q=[7]G`.

## Order 4054, `b=32`, `r=2027`

```text
G=(33,349), Q=(2098,1182)
C: y² + (a^8 x^4 + a^13 x + a^9)y
   = a^10 x^9 + a^12 x^8 + a^10 x^4 + a^5 x³
     + a^7 x² + a^11 x + a^10.
#Jac(C)(k)=56756=28·2027.
h(G)=(x³+a² x²+a^4 x+a^8,
      x²+a^11 x+a, 3)
h(Q)=(x^4+a^10 x³+a³ x²+a² x+a^9,
      a^9 x³+a³ x²+a^5 x, 4)
```

The [exact Magma receipt](attack_4054_receipt.json) verifies `[r]h(G)=0`, a full-rank 14-row relation solve, `h(Q)=[7]h(G)`, and `Q=[7]G`.

## Order 4078, `b=44`, `r=2039`

```text
G=(45,1367), Q=(47,3257)
C: y² + (a^8 x^4 + a^14 x² + x + a^9)y
   = a^14 x^9 + x^8 + a^11 x^5 + a^11 x^4
     + a^13 x³ + a^8 x² + a^14 x + a^5.
#Jac(C)(k)=57092=28·2039.
h(G)=(x^4+a^6 x³+a^5 x²+x+a^10,
      a^5 x³+a x²+a^13 x+a^14, 4)
h(Q)=(x^4+a^12 x³+a^5 x²+a²,
      a^6 x³+x²+a^6 x+a^4, 4)
```

The [exact Magma receipt](attack_4078_receipt.json) verifies `[r]h(G)=0`, a full-rank 18-row relation solve, `h(Q)=[7]h(G)`, and `Q=[7]G`.

## Order 4126, `b=63`, `r=2063`

```text
G=(62,1499), Q=(2633,1798)
C: y² + (a^8 x^4 + a³ x² + x + a^4)y
   = a^11 x^9 + a^13 x^8 + a x^5 + a^10 x³
     + a x² + a^12 x + a^12.
#Jac(C)(k)=57764=28·2063.
h(G)=(x^4+a^6 x³+a^13 x²+x+a^8,
      a² x³+x²+a^6 x+a^9, 4)
h(Q)=(x^4+a^4 x²+a^5 x+a^10,
      a^12 x³+a^13 x²+a² x+a^12, 4)
```

The [exact Magma receipt](attack_receipt.json) verifies `[r]h(G)=0`, a full-rank 15-row relation solve, `h(Q)=[7]h(G)`, and `Q=[7]G`.

These maps are to the **known Hess genus-four curves above**. None is an evaluated map to the plane quartic witness in the corresponding isogeny class. The [benchmark summary](benchmark_summary.json) and [independent row audit](audit.py) retain that distinction and all costs.
