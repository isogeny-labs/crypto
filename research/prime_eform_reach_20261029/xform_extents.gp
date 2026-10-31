\\ x-form extents of S3 and S4 over F_P (tooling): per-variable max and min exponent.
P = 2147483647; a = Mod(-3, P); b = Mod(1234567, P);
S3(x,y,z) = (x-y)^2*z^2 - 2*((x+y)*(x*y+a)+2*b)*z + ((x*y-a)^2 - 4*b*(x+y));
S4 = polresultant(S3(x1,x2,'X), S3(x3,x4,'X), 'X);
{
ext(Pl, vars) = vector(#vars, i, [valuation(Pl, vars[i]), poldegree(Pl, vars[i])]);
}
print("S3 (x1,x2,x3) [min,max] exponents: ", ext(S3(x1,x2,x3), [x1,x2,x3]));
print("S4 (x1,x2,x3,x4) [min,max] exponents: ", ext(S4, [x1,x2,x3,x4]));
