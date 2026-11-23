'use strict';
// CT91 independent target torsion verification; polynomial division + F2 linear solver.
const F12POLY=0x1225;
function mul12(a,b) {
 let raw=0;
 for(let i=0;i<12;i++) if ((b>>i)&1) raw^=a<<i;
 for(let j=22;j>=12;j--)if((raw>>j)&1)raw^=F12POLY<<(j-12);
 return raw;
}
function pow12(a,n) {
 let c=1;
 while(n>0){if(n%2)c=mul12(c,a);a=mul12(a,a);n=Math.floor(n/2);}
 return c;
}
function trace12(a){let o=0;for(let i=0;i<12;i++){o^=a;a=mul12(a,a);}return o;}
const g=mul12(2,mul12(2,2))^mul12(2,2);
if(trace12(g)!==1)throw Error('quadratic tower');
const one=[1,0];
const fa=(a,b)=>[a[0]^b[0],a[1]^b[1]];
function fm(a,b){const t=mul12(a[1],b[1]);return [mul12(a[0],b[0])^mul12(t,g),mul12(a[0],b[1])^mul12(a[1],b[0])^t];}
function fe(a,n){let c=one;while(n>0){if(n%2)c=fm(c,a);a=fm(a,a);n=Math.floor(n/2);}return c;}
function eq(a,b){return a[0]===b[0]&&a[1]===b[1];}
const ff=x=>[x,0];
function check(P){if(P===null)return true;const[x,y]=P;return eq(fa(fm(y,y),y),fa(fm(fm(x,x),x),fm(x,x)));}
function group(P,Q) {
 if(P===null)return Q;
 if(Q===null)return P;
 const[x,y]=P,[u,v]=Q;
 if(eq(x,u)&&eq(fa(y,v),one))return null;
 let lam,nu;
 if(eq(x,u)&&eq(y,v)){lam=fm(x,x);nu=fa(y,fm(lam,x));}
 else{lam=fm(fa(y,v),fe(fa(x,u),16777214));nu=fa(y,fm(lam,x));}
 const xx=fa(fa(fa(fm(lam,lam),one),x),u),yy=fa(fa(fm(lam,xx),nu),one);
 if(!check([xx,yy]))throw Error('group closure');
 return [xx,yy];
}
function neg(P){return P===null?null:[P[0],fa(P[1],one)];}
function frob(P,n){if(P===null)return null;let[x,y]=P;for(let i=0;i<n;i++){x=fm(x,x);y=fm(y,y);}return[x,y];}
const key=P=>JSON.stringify(P);
function line(P){if(!check(P))throw Error('off curve');const pts=[];let cur=null;
 for(let i=1;i<=7;i++){cur=group(cur,P);if(i<7)pts.push(cur);}
 if(cur!==null||new Set(pts.map(key)).size!==6)throw Error('not order7');
 return new Set(pts.map(key));
}
// Solve (a^2+a=z) using an independent 12x12 Gaussian elimination.
function asRoot(z){
 let rows=[];
 for(let i=0;i<12;i++){
  let mask=0;
  for(let j=0;j<12;j++) if(((mul12(1<<j,1<<j)^(1<<j))>>i)&1)mask|=1<<j;
  rows.push(mask|(((z>>i)&1)<<12));
 }
 let pivots=[],rank=0;
 for(let c=0;c<12;c++){
  let k=rank;while(k<12&&((rows[k]>>c)&1)===0)k++;
  if(k===12)continue;
  [rows[k],rows[rank]]=[rows[rank],rows[k]];
  for(let i=0;i<12;i++)if(i!==rank&&((rows[i]>>c)&1))rows[i]^=rows[rank];
  pivots.push(c);rank++;
 }
 for(let i=rank;i<12;i++)if(rows[i]&(1<<12))throw Error('AS no solution');
 let ans=0;
 for(let i=0;i<rank;i++) if((rows[i]>>12)&1)ans|=1<<pivots[i];
 if((mul12(ans,ans)^ans)!==z)throw Error('AS inconsistent');
 return ans;
}
function rootsFor(c){
 const cubic=x=>mul12(x,mul12(x,x))^mul12(1545,mul12(x,x))^mul12(1906,x)^c;
 let first=-1;
 for(let x=0;x<4096;x++)if(cubic(x)===0){first=x;break;}
 if(first<0)throw Error('cubic no root');
 const v=1545^first,w=1906^mul12(v,first);
 if((c^mul12(w,first))!==0)throw Error('division remainder');
 let quad=[];
 for(let x=0;x<4096;x++)if((mul12(x,x)^mul12(v,x)^w)===0)quad.push(x);
 const roots=[first,...quad].sort((a,b)=>a-b);
 if(roots.length!==3||new Set(roots).size!==3)throw Error('nonseparable cubic');
 if((roots[0]^roots[1]^roots[2])!==1545)throw Error('Vieta 1');
 if((mul12(roots[0],roots[1])^mul12(roots[0],roots[2])^mul12(roots[1],roots[2]))!==1906)throw Error('Vieta 2');
 if(mul12(mul12(roots[0],roots[1]),roots[2])!==c)throw Error('Vieta 3');
 return roots;
}
let base=[];let outcomes=[];
for(const [id,c] of [[240,1261],[241,1260]]){
 const roots=rootsFor(c),points=[];
 for(const x of roots){const rhs=mul12(x,mul12(x,x))^mul12(x,x);
  if(trace12(rhs)!==1)throw Error('y in base field');
  const a=asRoot(rhs^g);
  for(const t of [a,a^1]){
   const P=[ff(x),[t,1]];
   if(!check(P))throw Error('bad ordinate');
   points.push(P);
  }
 }
 points.sort((a,b)=>a[0][0]-b[0][0]||a[1][0]-b[1][0]);
 if(line(points[0]).size!==6||points.some(P=>!line(points[0]).has(key(P))))throw Error('cubic not subgroup');
 base.push(points[0]);
 outcomes.push({cubic_constant:c,idempotent:id,line_order:7,ordinate_trace:1,x_roots:roots});
}
if(JSON.stringify(outcomes.map(o=>o.x_roots))!==JSON.stringify([[2,17,1562],[122,383,1804]]))throw Error('roots');
const allLines=[];
for(const P of base){let orbit=[];for(let i=0;i<4;i++)orbit.push(line(frob(P,i)));if(new Set(orbit.map(s=>[...s].sort().join(';'))).size!==4)throw Error('short line orbit');allLines.push(...orbit);}
let frobMult=[];
for(const P of base){
 const target=key(frob(P,4));let Q=null,kFound=-1;
 for(let k=0;k<7;k++){if(key(Q)===target)kFound=k;Q=group(Q,P);}
 if(kFound!==3)throw Error('Frob4 scalar');frobMult.push(kFound);
}
for(const i of [1,2,3,4,6,8,12]) if(key(frob(base[0],i))===key(base[0]))throw Error('proper subfield');
let universe=new Set();for(const L of allLines){if(L.size!==6)throw Error('size');for(const a of L){if(universe.has(a))throw Error('line collision');universe.add(a);}}
if(allLines.length!==8||universe.size!==48)throw Error('not full torsion');
for(const s of universe){const P=JSON.parse(s);if(key(frob(P,24))!==s||key(frob(P,12))!==key(neg(P)))throw Error('Frobenius 12/24');}
// Independent prime-field group law and literal public known-answer fixture.
function srcAdd(P,Q){
 if(P===null)return Q;if(Q===null)return P;
 const[x,y]=P,[u,v]=Q,mod=n=>((n%37)+37)%37;
 if(x===u&&mod(y+v+1)===0)return null;
 const inv37=n=>{for(let t=1;t<37;t++)if(mod(n*t)===1)return t;throw Error('F37 inverse');};
 const m=(x===u&&y===v)?mod((3*x*x-2*x-10)*inv37(mod(2*y+1))):mod((v-y)*inv37(mod(u-x)));
 const n=mod(y-m*x),X=mod(m*m+1-x-u),Y=mod(-m*X-n-1);
 if(mod(Y*Y+Y-X*X*X+X*X+10*X+20)!==0)throw Error('source curve');
 return [X,Y];
}
let src=[null],dst=[null];for(let j=1;j<7;j++){src.push(srcAdd(src[j-1],[9,4]));dst.push(group(dst[j-1],base[0]));}
const expectSrc=[null,[9,4],[27,9],[28,0],[28,36],[27,27],[9,32]];
const expectDst=[null,[[2,0],[0,1]],[[17,0],[77,1]],[[1562,0],[2879,1]],[[1562,0],[2878,1]],[[17,0],[76,1]],[[2,0],[1,1]]];
if(JSON.stringify(src)!==JSON.stringify(expectSrc)||JSON.stringify(dst)!==JSON.stringify(expectDst))throw Error('public DLP controls');
if(srcAdd(src[6],[9,4])!==null||group(dst[6],base[0])!==null)throw Error('wrong order');
for(let i=0;i<7;i++)if(dst.findIndex(P=>key(P)===key(dst[i]))!==i)throw Error('toy DLP');
const result={disjoint_lines:8,experiment:'ct91-binary-7-torsion-partition',field_degree:24,frob4_scalar_per_seed:frobMult,frobenius_line_orbit_sizes:[4,4],nonzero_torsion_points:48,toy_public_log_cases:7,target_scan_used_for_verification:true,pairs:outcomes,scalar_free_source_recognition:false,selected:240,selected_target_x:2,unselected_prime_compatible_with_curve:true};
console.log(JSON.stringify(Object.fromEntries(Object.keys(result).sort().map(k=>[k,result[k]]))));
