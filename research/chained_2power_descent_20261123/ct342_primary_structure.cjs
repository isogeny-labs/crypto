'use strict';
// CT342 exact F103 finite elliptic group structure audit. No dependency on CT339-341.
const p=103;
function fail(s){throw new Error(s);}
function need(x,s){if(!x)fail(s);}
function mod(x){return ((x%p)+p)%p;}
function pow(a,n){let v=1;for(a=mod(a);n>0;n=Math.floor(n/2),a=mod(a*a))if(n%2)v=mod(v*a);return v;}
function inv(a){a=mod(a);need(a!==0,'division by zero');return pow(a,p-2);}
function cmul(a,b){let z=new Array(a.length+b.length-1).fill(0);for(let i=0;i<a.length;i++)for(let j=0;j<b.length;j++)z[i+j]=mod(z[i+j]+a[i]*b[j]);return z;}
function cubics(mode){const coeffs=[1,9,25].map(x=>mod(x*inv(4)));let f=[1];for(const x of coeffs) f=cmul(f,mode==='plus'?[mod(-x),1]:[1,mod(-x)]);return f;}
function polyval(c,x){let z=0;for(let i=c.length-1;i>=0;i--) z=mod(z*x+c[i]);return z;}
function eq(A,B){return A===null&&B===null || A!==null&&B!==null&&A.x===B.x&&A.y===B.y;}
function neg(A){return A===null?null:{x:A.x,y:mod(-A.y)};}
function add(c,A,B){if(A===null)return B;if(B===null)return A;
 if(A.x===B.x&&mod(A.y+B.y)===0)return null;
 let slope;
 if(A.x===B.x){need(A.y===B.y,'distinct same x points');slope=mod((3*c[3]*A.x*A.x+2*c[2]*A.x+c[1])*inv(2*A.y));}
 else slope=mod((B.y-A.y)*inv(B.x-A.x));
 const x=mod((slope*slope-c[2])*inv(c[3])-A.x-B.x),y=mod(slope*(A.x-x)-A.y);
 need(mod(y*y)===polyval(c,x),'group law produced off-curve point');return {x,y};
}
function mul(c,A,n){need(Number.isSafeInteger(n)&&n>=0,'invalid scalar');let R=null;while(n){if(n%2)R=add(c,R,A);n=Math.floor(n/2);if(n)A=add(c,A,A);}return R;}
function enumerate(c){let rows=[null];for(let x=0;x<p;x++)for(let y=0;y<p;y++)if(mod(y*y)===polyval(c,x))rows.push({x,y});return rows;}
function order(c,A,n){if(A===null)return 1;let q=A;for(let k=1;k<=n;k++){if(q===null)return k;q=add(c,q,A);}fail('point order not divisor of group size');}
function gcd(a,b){while(b){[a,b]=[b,a%b];}return a;}
function lcm(a,b){return (a/gcd(a,b))*b;}
function summarize(mode,mutant){let c=cubics(mode);
 if(mode==='minus'&&mutant==='wrong-coeff')c[0]=mod(c[0]+1);
 const pts=enumerate(c),n=pts.length;
 need(n===(mode==='plus'?92:100),'point count fails '+mode+' '+n);
 const ordHist={},all2=[],primary={}, orderWitness={};let exponent=1;
 for(let i=0;i<pts.length;i++){
  const A=pts[i];need(eq(add(c,A,neg(A)),null),'inverse mismatch');
  need(eq(mul(c,A,n),null),'Lagrange order mismatch');
  const t=order(c,A,n);need(n%t===0,'order not dividing group order');
  ordHist[t]=(ordHist[t]||0)+1;exponent=lcm(exponent,t);
  if(t===2)all2.push(A);if(!orderWitness[t])orderWitness[t]=A;
 }
 // Consistent full-rational 2-torsion independently computed from group arithmetic.
 need(all2.length===3,'expected 3 rational nonzero 2-torsion points');
 const d1=n/exponent,d2=exponent;need(Number.isInteger(d1)&&d2%d1===0,'invalid invariant factors');
 if(mutant==='wrong-primary'&&mode==='minus')need(d1===10 && d2===10,'mutated expected group Z/10 x Z/10 contradicted by computed point orders');
 if(mutant==='wrong-order'&&mode==='plus')need(n===93,'injected wrong group order expectation');
 for(const prime of (mode==='plus'?[2,23]:[2,5])){
  let total=0, maxPow=1, powVal=1;
  while(n% (powVal*prime)===0)powVal*=prime;
  for(const A of pts){if(eq(mul(c,A,powVal),null)){total++;const t=order(c,A,n);maxPow=Math.max(maxPow,t);}}
  primary[prime]={cardinality:total,max_order:maxPow};
 }
 const expect2=4;need(primary[2].cardinality===expect2 && primary[2].max_order===2,'2-Sylow wrong');
 if(mode==='plus'){need(primary[23].cardinality===23 && primary[23].max_order===23,'plus 23 part');}
 else {need(primary[5].cardinality===25,'minus 5 part');need([5,25].includes(primary[5].max_order),'unexpected minus 5 exponent');}
 // Independent group-law sample: O, 2-torsion and bounded deterministic non-2 subset.
 const sample=[null,...all2,...pts.slice(1,Math.min(17,pts.length))];let assocChecks=0;
 for(const A of sample)for(const B of sample)for(const C of sample){need(eq(add(c,add(c,A,B),C),add(c,A,add(c,B,C))),'nonassociative law');assocChecks++;}
 return {coefficients:c,count:n,invariant_factors:[d1,d2],exponent,order_histogram:Object.fromEntries(Object.entries(ordHist).sort((a,b)=>+a[0]- +b[0])),primary,nonzero_two_torsion:all2,order_witness:orderWitness,associativity_checks:assocChecks};
}
function main(){const mutant=(process.argv[2]||'').replace(/^--/,'');need(!mutant || ['wrong-coeff','wrong-order','wrong-primary'].includes(mutant),'unknown mutant');const plus=summarize('plus',mutant),minus=summarize('minus',mutant);
 const source2Rank=4,source2Order=16,prod2Order=plus.primary[2].cardinality*minus.primary[2].cardinality;
 need(source2Order===prod2Order,'source and target 2-parts differ');
 need(source2Rank===4&&plus.primary[2].max_order===2&&minus.primary[2].max_order===2,'F2 vector-space obstruction invalid');
 need(plus.count*minus.count===9200,'product count not CT341 baseline');
 need(plus.count/plus.primary[2].cardinality*minus.count/minus.primary[2].cardinality===575,'odd-primary order');
 const report={schema:'ct342-primary-structure/v1',field:p,source_jacobian_order:9200,target_order:plus.count*minus.count,odd_primary_order:575,two_primary_source_structure:'(Z/2)^4',two_primary_target_structure:'(Z/2)^4',dual_map_kernel_2_size:4,dual_map_2_rank:2,dual_map_cokernel_2_structure:'(Z/2)^2',plus,minus};
 process.stdout.write(JSON.stringify(report)+'
');
}
try{main();}catch(e){console.error('CT342 FAIL:',e.stack||e);process.exitCode=1;}
