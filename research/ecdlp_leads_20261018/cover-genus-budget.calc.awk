# Arithmetic of the formulas stated in README.md "Figures"; writes cover-genus-budget.csv.
# rho = 0.5*log2 r + 0.5*log2(pi/4) - 0.5*log2|Aut|;  g_max solves sqrt(2)*sqrt(G ln G) = rho*ln2, G = g ln2.
BEGIN{
 ln2=log(2); c=-0.5*log(3.141592653589793/4)/ln2;
 split("ECC2K-130 K-163 K-233 K-283 K-409 K-571 B-163 B-233 B-283 B-409 B-571",nm," ");
 split("131 163 233 283 409 571 163 233 283 409 571",N," ");
 split("129 162 231 281 407 569 162 232 282 408 570",R," ");
 split("K K K K K K B B B B B",T," ");
 split("130 162 29 94 204 114 162 29 94 204 114",D," ");
 wb[2]=33.97; wb[3]=197.99; wb[4]=1153.9; wb[5]=6725;   # Weil bound (1+sqrt2)^(2 g0) on #J_B(F_2)
 print "curve,family,n,log2_r,aut_order,log2_rho_steps,g_max_L12,g_max_over_n,ord_n_2,etale_g0,etale_genus,klein_ramified_genus";
 for(i=1;i<=11;i++){ n=N[i]; aut=(T[i]=="K")?2*n:2; rho=0.5*R[i]-c-0.5*log(aut)/ln2;
   y=(rho*ln2)^2/2; G=y/log(y); for(k=0;k<60;k++){ G=G-(G*log(G)-y)/(log(G)+1) } g=G/ln2;
   if(T[i]=="K"){ g0=2; while(wb[g0]<n) g0++; et=sprintf("%d,%d",g0,1+n*(g0-1)); kr=sprintf("%d",1+D[i]*(n-1)/2) } else { et="n/a,n/a"; kr="n/a" }
   printf "%s,%s,%d,%d,%d,%.2f,%.0f,%.2f,%d,%s,%s\n",nm[i],(T[i]=="K"?"Koblitz":"random-b binary"),n,R[i],aut,rho,g,g/n,D[i],et,kr }
}
