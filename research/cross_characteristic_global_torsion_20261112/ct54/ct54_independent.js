#!/usr/bin/env node
'use strict';
// Independent characteristic-polynomial census and brute-force point counts.
function require(c, m) { if (!c) throw new Error(m); }
function census(l) {
 let total=0, positive=0, identity=0, oneLine=0, oriented=0;
 for(let a=0;a<l;a++) for(let b=0;b<l;b++)
 for(let c=0;c<l;c++) for(let d=0;d<l;d++) {
   const det=((a*d-b*c)%l+l)%l;
   if(det===0)continue;
   total++;
   if(a===1 && c===0)oriented++;
   if((1-(a+d)+det)%l===0) {
     positive++;
     if(a===1 && b===0 && c===0 && d===1)identity++;
     else oneLine++;
   }
 }
 const size=(l*l-1)*(l*l-l);
 require(total===size,'order mismatch '+l);
 require(positive===l*(l*l-2),'fixed eigenvalue count '+l);
 require(oneLine===l*l*l-2*l-1 && identity===1,'rank decomposition '+l);
 require(oriented===l*(l-1), 'fixed oriented point stabilizer '+l);
 require(oriented!==positive, 'mutant conflates oriented and existential events');
 console.log('GL2 '+l+' total '+total+' positive '+positive+' any_density '+(positive/total).toPrecision(10)+' oriented_density '+(oriented/total).toPrecision(10));
}
function primes(limit){let v=[]; for(let n=2;n<=limit;n++){let prime=true;for(let f=2;f*f<=n;f++)if(n%f===0){prime=false;break;}if(prime)v.push(n)} return v;}
function points(p){let ct=1;for(let x=0;x<p;x++) for(let y=0;y<p;y++){
 const rhs=x*x*x-x*x-10*x-20;
 if(((y*y+y-rhs)%p+p)%p===0)ct++;
}return ct;}
function run() {
 for(let l of [2,3,5,7,11])census(l);
 let ps=primes(257).filter(p=>![2,3,5,11].includes(p));
 let hit3=0,hit5=0;
 for(let p of ps){let order=points(p);hit3+=(order%3===0);hit5+=(order%5===0);}
 require(hit5===ps.length,'rational torsion special control');
 console.log('independent curve brute-force p<=257 count '+ps.length+' l=3 '+hit3+' l=5 '+hit5);
 console.log('PASS independent characteristic-polynomial and brute-force checks');
}
run();
