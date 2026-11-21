'use strict';
// CT122 independent JavaScript: GF(32) enumeration and matrix-power torsion orbits.
const mod=(a,p)=>((a%p)+p)%p;
function gfTimes(a,b){ let z=0; for(let i=0;i<5;i++){if(b&1)z^=a; b>>>=1; a<<=1;if(a&32)a^=0x25;}return z; }
function f32Count(coef){let [a1,a2,a3,a4,a6]=coef.map(v=>mod(v,2)); let cnt=1; for(let x=0;x<32;x++)for(let y=0;y<32;y++){
 const lhs=gfTimes(y,y) ^ (a1?gfTimes(x,y):0) ^ (a3?y:0);
 const rhs=gfTimes(gfTimes(x,x),x)^(a2?gfTimes(x,x):0)^(a4?x:0)^(a6?1:0);
 if(lhs===rhs)cnt++;
}return cnt;}
function discr(a){let [u,v,w,x,z]=a.map(BigInt);
 let b2=u*u+4n*v,b4=2n*x+u*w,b6=w*w+4n*z;
 let b8=u*u*z+4n*v*z-u*w*x+v*w*w-x*x;
 return -b2*b2*b8-8n*b4*b4*b4-27n*b6*b6+9n*b2*b4*b6;
}
function f2Count(a){let n=1;for(let x=0;x<2;x++)for(let y=0;y<2;y++){
 let [a1,a2,a3,a4,a6]=a.map(v=>mod(v,2));
 if(mod(y*y+a1*x*y+a3*y-x*x*x-a2*x*x-a4*x-a6,2)===0)n++;
}return n;}
function mult(A,B){let C=[];for(let i=0;i<2;i++){C.push([]);for(let j=0;j<2;j++)C[i][j]=mod(A[i][0]*B[0][j]+A[i][1]*B[1][j],11);}return C;}
function eq(v,w){return v[0]===w[0]&&v[1]===w[1];}
function action(M,v){return [mod(M[0][0]*v[0]+M[0][1]*v[1],11),mod(M[1][0]*v[0]+M[1][1]*v[1],11)];}
function counts(t){let base=[[0,9],[1,mod(t,11)]],powers=[],A=[[1,0],[0,1]];
 for(let n=1;n<=120;n++){A=mult(base,A);powers.push(A);}
 let stats=new Map();for(let x=0;x<11;x++)for(let y=0;y<11;y++){if(x===0&&y===0)continue;
 const v=[x,y];const neg=[mod(-x,11),mod(-y,11)];
 let d=0,k=0;for(let n=1;n<=120;n++){const w=action(powers[n-1],v);if(k===0&&(eq(w,v)||eq(w,neg)))k=n;if(d===0&&eq(w,v))d=n;if(d&&k)break;}
 if(!d||!k)throw new Error('missing orbit');let key=d+','+k;stats.set(key,(stats.get(key)||0)+1);
 }return [...stats.entries()].map(([key,count])=>{let [point,x]=key.split(',').map(Number);return {count,point,x};}).sort((a,b)=>a.point-b.point||a.x-b.x);
}
function order(a){let k=1,r=mod(a,11);while(r!==1){r=mod(r*a,11);k++;if(k>10)throw new Error('bad eigenvalue');}return k;}
const orig=[0,-1,1,-10,-20], cases={ordinary_trace_minus_one:[59,58,60,-10,39],ordinary_trace_plus_one:[59,-1,60,-10,39],supersingular_existing:orig};
let result={};for(let name of Object.keys(cases)){
 let a=cases[name];if(!a.every((v,i)=>mod(v-orig[i],59)===0))throw new Error('source mismatch');
 const d2=Number((discr(a)%2n+2n)%2n), d59=Number((discr(a)%59n+59n)%59n);
 if(d2!==1||d59!==19)throw new Error('bad reduction');
 let t=3-f2Count(a), eigen=[];for(let i=1;i<11;i++)if(mod(i*i-t*i+2,11)===0)eigen.push(i);
 let dist=counts(t),n32=f32Count(a);
 if(n32!==(({1:22,'-1':44,'-2':25})[t]))throw new Error('GF32 independent group count');
 if(dist.reduce((z,c)=>z+c.count,0)!==120)throw new Error('all torsion check');
 result[name]={curve_coeffs:a,discriminant_mod2:d2,discriminant_mod59:d59,f2_points:f2Count(a),trace:t,frobenius_eigenvalues_mod11:eigen,eigenvalue_orders:eigen.map(order).sort((a,b)=>a-b),f32_group_order:n32,torsion_degree_distribution:dist};
}
function sorted(o){if(Array.isArray(o))return o.map(sorted);if(o!==null&&typeof o==='object'){let out={};for(let k of Object.keys(o).sort())out[k]=sorted(o[k]);return out;}return o;}
console.log(JSON.stringify(sorted(result)));
