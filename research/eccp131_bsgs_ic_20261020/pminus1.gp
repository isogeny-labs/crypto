\\ Exact field modulus from Certicom ECCp-131, recorded for the direct
\\ smooth-divisor factor-base admission check.
p = 0x048E1D43F293469E33194C43186B3ABC0B;
print("p=", p);
print("p_minus_one=", p-1);
print("factor_p_minus_one=", factor(p-1));
print("large_cofactor_isprime=", isprime((p-1)/10));
