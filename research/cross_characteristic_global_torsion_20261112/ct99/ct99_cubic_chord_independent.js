#!/usr/bin/env node
'use strict';
// Independent CT99 source/binary chord controls and full 7-point group-law
// verification. Does NOT rederive CT97's QQ quotient field or number-field root.
// No multiplication-map leading coefficient or scalar label is used.
const ck=(z,msg)=>{if(!z)throw Error(msg)};
const md=z=>(z%37+37)%37;
const inv37=z=>{for(let i=1;i<37;i++)if(md(i*z)===1)return i;throw Error('nonunit')};
const sourceAdd=(P,Q)=>{
 if(!P)return Q;if(!Q)return P;
 const [x,y]=P,[z,w]=Q;
 if(x===z&&md(y+w+1)===0)return null;
 let slope;
 if(x===z){ck(y===w,'not source tangent');slope=md((3*x*x-2*x-10)*inv37(md(2*y+1)))}
 else slope=md((w-y)*inv37(md(z-x)));
 const v=md(slope*slope+1-x-z);
 return [v,md(slope*(x-v)-y-1)]
};
function m12(a,b){let v=0;while(b){if(b&1)v^=a;a<<=1;if(a&4096)a^=0x1225;b>>=1}return v&4095}
function exp12(a,n){let z=1;while(n){if(n&1)z=m12(z,a);a=m12(a,a);n>>=1}return z}
const ax=(a,b)=>[a[0]^b[0],a[1]^b[1]];
function mx(a,b){const t=m12(a[1],b[1]);return [m12(a[0],b[0])^m12(t,12),
  m12(a[0],b[1])^m12(a[1],b[0])^t]}
function fx(a,n){let z=[1,0];while(n){if(n&1)z=mx(z,a);a=mx(a,a);n>>=1}return z}
const eq=(a,b)=>JSON.stringify(a)===JSON.stringify(b);
function binaryAdd(P,Q){
 if(!P)return Q;if(!Q)return P;
 const [x,y]=P,[z,w]=Q;
 let slope;
 if(eq(x,z)){
  if(eq(ax(y,w),[1,0]))return null;
  ck(eq(y,w),'not binary tangent');slope=mx(x,x)
 }else slope=mx(ax(y,w),fx(ax(x,z),2**24-2));
 const u=ax(ax(mx(slope,slope),x),ax(z,[1,0]));
 return [u,ax(ax(mx(slope,ax(x,u)),y),[1,0])]
}
function onBinary(P){let [x,y]=P;
 return eq(ax(mx(y,y),y),ax(mx(mx(x,x),x),mx(x,x)))}
const srcX=[9,27,28],binX=[2,17,1562];
const s37=md(srcX.reduce((a,b)=>a+b,0));
const e237=md(srcX[0]*srcX[1]+srcX[0]*srcX[2]+srcX[1]*srcX[2]);
ck(s37===27&&e237===30,'source symmetric coefficients');
const M37=md(2*9*(s37-1)-e237-10),D37=md((2*4+1)**2);
ck(md(M37*M37)===md((s37-1)*D37),'source chord square');
const slope=md(M37*inv37(2*4+1));
ck(slope===27,'wrong source chord slope');
const s2=1545,e22=1906,a2=2;
ck(m12(e22,e22)===(s2^1),'binary chord square');
const rows=[], tableSource=new Map(), tableBinary=new Map();
for(let i=0;i<3;i++){
 const xx=srcX[i],xb=binX[i];
 const line=md(4+slope*(xx-9));
 const h2=m12(e22,xb^a2);
 // The line and its inverse, sorted by public source ordinate below.
 for(let sign of [1,-1]){
  const yy=sign===1?line:md(-1-line);
  const targetY=[h2^(sign===1?0:1),1];
  const src=[xx,yy], dst=[[xb,0],targetY];
  ck(md(yy*yy+yy)===md(xx*xx*xx-xx*xx-10*xx-20),
    'source point off curve');
  ck(onBinary(dst),'binary chord point off curve');
  rows.push({source:src,target_x:xb,target_y:targetY});
  tableSource.set(JSON.stringify(src),src);
  tableBinary.set(JSON.stringify(src),dst);
 }
}
rows.sort((a,b)=>a.source[0]-b.source[0]||a.source[1]-b.source[1]);
ck(eq(rows.map(z=>z.target_y),
 [[0,1],[1,1],[77,1],[76,1],[2879,1],[2878,1]]),
 'CT91 target orientation mismatch');
// Full operation table, not just six known-answer output rows.
const sources=[null,...rows.map(z=>z.source)];
for(const P of sources)for(const Q of sources){
 const R=sourceAdd(P,Q);
 ck(R===null||tableSource.has(JSON.stringify(R)),'source subgroup not closed');
 const left=binaryAdd(P?tableBinary.get(JSON.stringify(P)):null,
                      Q?tableBinary.get(JSON.stringify(Q)):null);
 const right=R?tableBinary.get(JSON.stringify(R)):null;
 ck(eq(left,right),'nonhomomorphic scalar-blind chord transport');
}
const output={binary_line_certificate:true,experiment:'CT99-cubic-chord-orientation',
 fixed_source_place:true,fixed_target_place:true,global_line_certificate:true,
 online_scalar_observed:false,scalar_independent_online_selection:true,
 source_rows:rows};
function stable(v){if(Array.isArray(v))return '['+v.map(stable).join(',')+']';
 if(v&&typeof v==='object')return '{'+Object.keys(v).sort().map(k=>JSON.stringify(k)+':'+stable(v[k])).join(',')+'}';
 return JSON.stringify(v)}
process.stdout.write(stable(output)+'\n');
