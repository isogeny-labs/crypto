// Development API qualification, not a performance measurement.
SetSeed(1);
SetNthreads(1);
SetColumns(256);
p := 115792089210356248762697446949407573530086143415290314195533631308867097853951;
F := GF(p);
assert #F eq p;
R<x,y> := PolynomialRing(F,2,"grevlex");
I := ideal<R | x^2-1,y-x>;
G := GroebnerBasis(I);
V := Variety(I);
assert #V eq 2;
for v in V do
    assert v[1]^2 eq 1 and v[2] eq v[1];
end for;
printf "P256_GB_PROBE_OK %o\n", #V;
quit;
