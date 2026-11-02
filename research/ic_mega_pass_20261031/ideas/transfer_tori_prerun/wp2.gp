PE(T)=T^2+T+2;
tau=(-1+sqrt(-7))/2;
{forprime(n=3,13,
  tn=round(real(tau^n+conj(tau)^n)); PEn=T^2-tn*T+2^n;
  PA=(subst(PEn,T,T^n))/PE(T);
  print("n=",n," #E(F_2^n)=",subst(PEn,T,1)," =",factor(subst(PEn,T,1)), "  P_A=",PA, "  irred/Q: ", polisirreducible(PA), " ord_n(2)=",znorder(Mod(2,n))));
}
print("ord_131(2)=",znorder(Mod(2,131)), " ord_263(2)=",znorder(Mod(2,263)), " kronecker(2,263)=", kronecker(2,263));
print("2^131-1 = ", factor(2^131-1));
print("Phi_130(2) mod 131 = ", polcyclo(130,2) % 131, "  2^65 mod 131 = ", lift(Mod(2,131)^65));
\\ P_A at n=131: degree and the factor structure argument: alpha*zeta generates Q(sqrt-7,zeta_131)
t131=round(real(tau^131+conj(tau)^131)); print("t_131=",t131); 
print("#E(F_2^131)=", 2^131+1-t131, " = ", factor(2^131+1-t131));
