\\ Kummer 5-covers of P^1 over F_2 (genus 4 when 4 branch points): w^5 = f(x). Point counts over F_{2^k}, k=1..4: for k not divisible by 4, w->w^5 is a bijection on F_{2^k}^* so #C = #P^1 = 2^k+1 (plus fibre over roots/poles). Verify numerically and compare Weil polynomial with P_A(n=5).
PA=T^8 - T^7 - T^6 + 3*T^5 - T^4 + 6*T^3 - 4*T^2 - 8*T + 16;
count(f,k)={ my(g=ffgen([2,k],'t), cnt=0, N=2^k-1, d=gcd(5,N));
  \\ affine x in F_{2^k}: number of w with w^5 = f(x)
  for(i=0,2^k-1, my(xx = if(i==0,0*g, g^i), v);
     v = subst(f, x, xx);
     if(v==0, cnt+=1, if(d==1, cnt+=1, if(v^(N/d)==1, cnt+=d))));
  \\ point(s) at infinity: deg f determines; treat generically as one place if 5 | deg f else ramified: count = number of w with w^5 = lc*inf... approximate: use the fact that f has deg divisible by 5 => 5 or 1 points; we choose f with deg f = 5 (unramified at infinity => d points if lc is a 5th power else 0)
  cnt + if(d==1,1, if(poldegree(f)%5==0, d, 1)) };
\\ example: branch points 0, 1, and the F_4 pair (roots of x^2+x+1), exponents 1,1 ,and 2,2 ... need sum e_i = 0 mod 5 up to adjusting: f = x * (x+1) * (x^2+x+1)^? ; take f = x*(x+1)^2*(x^2+x+1) (deg 5, exps 1,2,1,1 sum=5)
f = x*(x+1)^2*(x^2+x+1);
cs = vector(4,k,count(f,k)); print("counts F_{2^k}, k=1..4: ", cs, "  vs P^1: ", vector(4,k,2^k+1));
\\ Weil polynomial from counts via Newton identities: a_k = 2^k+1-#C
a = vector(4,k,2^k+1-cs[k]); print("traces a_k=",a);
\\ degree-8 Weil poly P(T)=T^8 + c1 T^7 + ... with c determined by power sums s_k = a_k (sum of roots^k)
s=a; e=vector(4); e[1]=s[1]; e[2]=(e[1]*s[1]-s[2])/2; e[3]=(e[2]*s[1]-e[1]*s[2]+s[3])/3; e[4]=(e[3]*s[1]-e[2]*s[2]+e[1]*s[3]-s[4])/4;
P = T^8 - e[1]*T^7 + e[2]*T^6 - e[3]*T^5 + e[4]*T^4 - 2*e[3]*T^3 + 4*e[2]*T^2 - 8*e[1]*T + 16;
print("P_C = ", P, "   equals P_A? ", P==PA);
