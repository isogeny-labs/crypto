#!/usr/bin/env node
// CT105 independent JS: exhaustive vector / projective Frobenius cycles.
'use strict';
function check(b,m){if(!b)throw Error(m);}
const n=11;
function apply(a,v){return [(a[0]*v[0]+a[1]*v[1])%n,(a[2]*v[0]+a[3]*v[1])%n];}
const src=[0,7,1,5],dst=[0,9,1,9];
function norm(v){if(v[0]===0)return [0,1];let t=0;
 for(let i=1;i<n;i++)if(v[0]*i%n===1){t=i;break;}
 check(t!==0,'noninvertible vector');return [1,v[1]*t%n];}
function cycle(a,projective){let vectors=[];
 if(projective){vectors=[[0,1]];for(let k=0;k<n;k++)vectors.push([1,k]);}
 else{for(let x=0;x<n;x++)for(let y=0;y<n;y++)if(x||y)vectors.push([x,y]);}
 const remaining=new Set(vectors.map(v=>v.join(','))),counts=new Map();
 for(const v of vectors){const key=v.join(',');if(!remaining.has(key))continue;
  let w=v.slice(),len=0;
  do{check(remaining.delete(w.join(',')),'orbit repeated before closure');len++;
    w=apply(a,w);if(projective)w=norm(w);
  }while(w.join(',')!==key);
  counts.set(len,(counts.get(len)||0)+1);
 }
 check(remaining.size===0,'leftover vectors');
 return [...counts.entries()].sort((a,b)=>a[0]-b[0]);
}
let v=[1,0];for(let i=0;i<4;i++)v=apply(dst,v);check(v.join(',')==='7,0','Frob4 e1');
v=[0,1];for(let i=0;i<4;i++)v=apply(dst,v);check(v.join(',')==='0,7','Frob4 e2');
const scalars=new Set();let s=1;for(let i=0;i<10;i++){scalars.add(s);s=s*7%n;}
check(s===1&&scalars.size===10,'unit generator');
const so=cycle(src,false),bo=cycle(dst,false),sp=cycle(src,true),bp=cycle(dst,true);
check(JSON.stringify(so)==='[[1,10],[5,22]]','source oriented');
check(JSON.stringify(bo)==='[[40,3]]','binary oriented');
check(JSON.stringify(sp)==='[[1,2],[5,2]]','source projective');
check(JSON.stringify(bp)==='[[4,3]]','binary projective');
const result={binary_frobenius_four_scalar:7,binary_oriented_cycles:bo,
 binary_projective_cycles:bp,binary_residue_degree:40,
 cyclic_automorphism_order:10,fixed_field_degree:12,
 relative_binary_residue_degree:10,relative_source_split_primes:10,
 source_oriented_cycles:so,source_projective_cycles:sp};
console.log(JSON.stringify(result));
