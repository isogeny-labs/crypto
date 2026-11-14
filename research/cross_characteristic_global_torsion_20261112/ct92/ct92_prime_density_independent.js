'use strict';
// CT92 independent modular matrix rank and fixed-vector enumeration.
const mod = (x, n) => (x % n + n) % n;
const gcd=(a,b)=>b===0?a:gcd(b,a%b);
function count(ell) {
 let group=0, eligible=0, total=0;
 const histogram=new Map([[0,0],[ell-1,0],[ell*ell-1,0]]);
 for(let a=0;a<ell;a++) for(let b=0;b<ell;b++)
  for(let c=0;c<ell;c++) for(let d=0;d<ell;d++) {
   if(mod(a*d-b*c,ell)===0)continue;
   group++;
   // Independent count via rank of A-I rather than vector enumeration.
   let fixed=0;
   if(a===1&&b===0&&c===0&&d===1)fixed=ell*ell-1;
   else if(mod((a-1)*(d-1)-b*c,ell)===0)fixed=ell-1;
   if(fixed)eligible++;
   total+=fixed;
   if(!histogram.has(fixed))throw Error('unexpected rank');
   histogram.set(fixed,histogram.get(fixed)+1);
  }
 if(group!==ell*(ell-1)*(ell-1)*(ell+1)||eligible!==ell*(ell*ell-2)||total!==group)throw Error('Burnside');
 if(histogram.get(ell*ell-1)!==1||histogram.get(ell-1)!==eligible-1)throw Error('fixed histogram');
 const num=eligible,den=group,g=gcd(num,den);
 return {ell,gl2_order:group,matrices_with_eigenvalue_one:eligible,
  degree_one_prime_counts:[...histogram].sort((x,y)=>x[0]-y[0]),
  chebotarev_density:`${num/g}/${den/g}`,average_degree_one_places:1,
  oriented_field_degree_if_transitive:ell*ell-1,
  x_field_degree_if_transitive:(ell*ell-1)/2,
  dense_monic_x_polynomial_entries:(ell*ell+1)/2,
  source_subgroup_full_lookup_entries:ell};
}
function curveOrder(p) {
 let n=1;
 for(let x=0;x<p;x++)for(let y=0;y<p;y++)
  if(mod(y*y+y-x*x*x+x*x+10*x+20,p)===0)n++;
 return n;
}
const source=curveOrder(37),target=curveOrder(2);
if(source!==35||target!==5||(38-source)!==3)throw Error('curve count');
let roots=[];for(let a=0;a<7;a++)if(mod(a*a-3*a+2,7)===0)roots.push(a);
if(JSON.stringify(roots)!=='[1,2]')throw Error('Frobenius roots');
let result={experiment:'ct92-degree-one-place-density',
  galois_image_hypothesis:'full_GL2_mod_ell',
  source_prime:37,source_curve_order:source,source_trace:3,
  source_eigenvalues_mod_7:roots,source_degree_one_primes:6,
  binary_curve_order:target,ell_cases:[3,5,7,11].map(count)};
function canonical(x) {
 if(Array.isArray(x))return x.map(canonical);
 if(x!==null&&typeof x==='object')return Object.fromEntries(Object.keys(x).sort().map(k=>[k,canonical(x[k])]));
 return x;
}
console.log(JSON.stringify(canonical(result)));
