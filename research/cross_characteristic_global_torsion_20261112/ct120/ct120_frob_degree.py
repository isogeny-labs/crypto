#!/usr/bin/env python3
"""CT120 independent matrix iteration and modular-order controls."""
import json

def check(c, why):
    if not c:
        raise RuntimeError(why)

def primes_to(limit):
    out = []
    for n in range(3, limit + 1, 2):
        if all(n % d for d in range(3, int(n ** 0.5) + 1, 2)):
            out.append(n)
    return out

def order(a, p):
    x = 1
    for k in range(1, p):
        x = (x * a) % p
        if x == 1:
            return k
    raise RuntimeError("nonunit")

def frob(v, p):
    x, y = v
    return ((-2 * y) % p, (x - 2 * y) % p)

def first_frob_return(v, p, up_to):
    w = v
    point = xonly = None
    for k in range(1, up_to + 1):
        w = frob(w, p)
        if xonly is None and (w == v or w == tuple(-t % p for t in v)):
            xonly = k
        if w == v:
            point = k
            break
    check(point is not None and xonly is not None, f"return not found {p}")
    return point, xonly

def main():
    check(first_frob_return((3, 1), 5, 20) == (1, 1), "split-eigenline counterexample")
    result = []
    for ell in primes_to(211):
        if ell % 4 != 3:
            continue
        a = (-4) % ell
        h = order(a, ell)
        predicted_point = 4 * h
        predicted_x = 4 * (h // 2 if h % 2 == 0 else h)
        for v in ((1, 0), (0, 1), (1, 1)):
            check(first_frob_return(v, ell, 4 * (ell - 1)) == (predicted_point, predicted_x),
                  f"matrix/order disagreement {ell} {v}")
        check(pow(-4, h, ell) == 1, "order check")
        result.append([ell, h, predicted_point, predicted_x])
    check(next(r for r in result if r[0] == 11) == [11, 10, 40, 20], "fixed CT115 control")
    check(next(r for r in result if r[0] == 3) == [3, 2, 8, 4], "small prime control")
    B = 10
    prod = 1
    for k in range(1, B + 1):
        prod *= (-4)**k - 1
    for ell, h, _, _ in result:
        check((h > B) or (prod % ell == 0), "exception-count bound failed")
        check((h <= B) or all(((-4)**k - 1) % ell for k in range(1, B + 1)),
              "exception converse failed")
    output = {
        "curve":"y^2+y=x^3+x^2 over F2; characteristic polynomial Z^2+2Z+2",
        "inert_primes_upto":211,
        "inert_count":len(result),
        "return_degrees_ell_order_point_x":result,
        "split_eigenline_ell5_point_x":[1,1],
        "sparsity_bound_B":B,
        "sparsity_bound_distinct_prime_count":B*(B+2),
        "sparsity_observed_ell_upto_211":sum(h<=B for _,h,_,_ in result),
    }
    print(json.dumps(output, sort_keys=True, separators=(',', ':')))

if __name__ == '__main__':
    main()
