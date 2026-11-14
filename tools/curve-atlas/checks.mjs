import assert from 'node:assert/strict';
import {build,mod,inv,points} from './dist/math.mjs';
let count=0;
for(const p of [5,7,11,13,17,101])for(let a=0;a<Math.min(p,5);a++)for(let b=0;b<Math.min(p,5);b++){
 if(mod(4*a*a*a+27*b*b,p)===0)continue;
 const g=build({p,a,b,c:0,maxN:6,depth:2,quartic:true,jacobians:true});
 for(const e of g.edges.filter(e=>e.kind==='isogeny')){
 const A=g.nodes.find(n=>n.id===e.source),B=g.nodes.find(n=>n.id===e.target),r=e.kernel,t=mod(3*r*r+A.a,p);
 assert.equal(points(A.a,A.b,p),points(B.a,B.b,p));
 for(let x=0;x<p;x++)for(let y=0;y<p;y++)if(x!==r&&mod(y*y-x*x*x-A.a*x-A.b,p)===0){let z=inv(mod(x-r,p),p),X=mod(x+t*z,p),Y=mod(y*(1-t*z*z),p);assert.equal(mod(Y*Y-X*X*X-B.a*X-B.b,p),0);count++}
 }
 for(const C of g.nodes.filter(n=>n.n)){
 assert(Number.isInteger(C.genus));assert(C.genus>1);
 for(let u=0;u<p;u++)for(let v=0;v<p;v++){let X=mod(u**C.n,p),R=mod(X**3+a*X+b,p);if(mod(v**C.m,p)===R){let Y=mod(v**(C.m/2),p);assert.equal(mod(Y*Y-X**3-a*X-b,p),0);count++}}
 }
}
assert.throws(()=>build({p:15,a:2,b:3,c:0,maxN:3,depth:1}));
assert.throws(()=>build({p:101,a:0,b:0,c:0,maxN:3,depth:1}));
const baseline=build({p:101,a:2,b:3,c:0,maxN:3,depth:1});
assert.deepEqual(baseline.nodes.filter(n=>n.n).map(n=>n.genus),[2,4,3,7,12]);
assert(build({p:101,a:2,b:3,c:-1,maxN:3,depth:1}).skipped.length);
console.log(`Passed ${count} finite-field map identity checks, rational isogeny point-count checks, genus checks, and validation checks.`);
