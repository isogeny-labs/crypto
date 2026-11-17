\\ trial-weighted (not sqrt(n)-normalised) pooled b>=28 counted-cost ratios from out/opcounts_pooled.txt totals; arithmetic only, no sampling
v = Map(); 
mapput(v,"a0",[51576916,103153832,51576916,51576916]/272);
mapput(v,"kpure",[54570020,1173529977,654997124,54570020]/272);
mapput(v,"kmix",[41524481,892706352,498253778,41524481]/272);
c(a,r) = my(x=mapget(v,a)); x[2]+0.8*x[3]+r*x[4];
foreach([3,10,30,100], r, printf("I/M=%d: kpure/a0 %.3f  kmix/a0 %.3f  (per-trial mean cost a0 %.1f kpure %.1f kmix %.1f M-equiv)\n", r, c("kpure",r)/c("a0",r), c("kmix",r)/c("a0",r), c("a0",r)*1., c("kpure",r)*1., c("kmix",r)*1.));
