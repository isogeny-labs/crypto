\\ CM-discriminant screen per PREREG_DISCRIMINANT.md
default(parisize, 10^8);
B = 2^24;
cls(D) = {my(A=abs(D), f=1, F=factor(A, B), cof, e);
  for(k=1, #F~, e=F[k,2]; if(F[k,1] < B, f *= F[k,1]^(e\2)));
  cof = F[#F~,1]; if(cof >= B && issquare(cof), f = -f);  \\ negative flags a large square factor
  f};
\\ positive controls
r = nextprime(2^200)*nextprime(2^40);
print("control q=3 -> ", cls(-(9*r)), "  q=101 -> ", cls(-(101^2*r)), "  q=65537 -> ", cls(-(65537^2*r)), "  squarefree -> ", cls(-r));
read("cands.gp");
{for(k=1, #C, my(c=C[k], p=c[5], N=c[6], t=p+1-N, D=t^2-4*p, f=cls(D));
  printf("%s p%d cand %5d %s  D mod 8 = %d  conductor proxy f = %s\n", c[1], c[2], c[3], if(c[4],"PUBLISHED","skipped  "), D%8, f))}
