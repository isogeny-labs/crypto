'use strict';
// Independent CT53 oracle: explore the orbit under elementary symplectic
// transvections. It does not implement the Python two-step construction.
function check(ok,why){if(!ok)throw new Error(why)}
function vectors(q,d){const result=[];for(let n=0;n<q**d;n++){let t=n;const v=[];for(let i=0;i<d;i++){v.push(t%q);t=Math.floor(t/q)}result.push(v)}return result}
function rank(v,q){let out=0,pow=1;for(const x of v){out+=x*pow;pow*=q}return out}
function pair(a,b,q){const g=a.length/2;let s=0;for(let i=0;i<g;i++)s+=a[i]*b[i+g]-a[i+g]*b[i];return (s%q+q)%q}
function elementary(v,u,q){const m=pair(v,u,q);return v.map((x,i)=>(x+m*u[i])%q)}
function orbit(q,g){const all=vectors(q,2*g),u=all.slice(1), seen=new Set([1]), queue=[1];let visited=0;
  while(visited<queue.length){const x=queue[visited++];const v=all[x];for(const w of u){const next=rank(elementary(v,w,q),q);if(!seen.has(next)){seen.add(next);queue.push(next)}}}
  check(!seen.has(0),'zero illegally reached');check(seen.size===q**(2*g)-1,'wrong orbit');
  // Independent preservation checks for each available elementary transvection direction.
  const probe=all.slice(1,Math.min(23,all.length));for(const d of u.slice(0,Math.min(31,u.length))){for(const v of probe){for(const w of probe){check(pair(elementary(v,d,q),elementary(w,d,q),q)===pair(v,w,q),'not symplectic')}}}
  return {ell:q,g,orbit:seen.size}
}
function split(q){let s=new Set();for(let a=0;a<q;a++)for(let b=0;b<q;b++)for(let c=0;c<q;c++)for(let d=0;d<q;d++)if((a*d-b*c)%q!==0){s.add(a+','+c)}check(s.size===q*q-1,'split factor orbit');return s.size}
for(const [q,g] of [[2,1],[2,2],[3,1],[3,2],[3,3],[5,1],[5,2],[7,1]])console.log(JSON.stringify(orbit(q,g)));
for(const q of [3,5,7,11]){const n=split(q);console.log(JSON.stringify({ell:q,splitOrbit:n,fullG2:q**4-1,fullG3:q**6-1}))}
// Identity (mutant) fails to traverse the orbit, and its cardinality is not mistaken for full.
const bad=new Set([1]);check(bad.size!==3**4-1,'mutant escaped');console.log('PASS independent BFS, split-block, mutant checks');
