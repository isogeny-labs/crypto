'use strict';
// CT89 independent BigInt polynomial reduction and upper-triangular basis coordinates.
const G=[-2102915n,3828478n,-6423347n,-1696464n,-39690n,-29862n,-840n,-8n,1n];
const A=[
[4n,0n,0n,0n,0n,0n,0n,0n],
[0n,4n,0n,0n,0n,0n,0n,0n],
[0n,0n,4n,0n,0n,0n,0n,0n],
[0n,0n,0n,4n,0n,0n,0n,0n],
[1n,3n,0n,0n,1n,0n,0n,0n],
[0n,1n,3n,0n,0n,1n,0n,0n],
[0n,0n,1n,3n,0n,0n,1n,0n],
[1n,3n,0n,1n,0n,0n,0n,1n]];
function multiply(a,b){
    let v=Array(15).fill(0n);
    for(let i=0;i<8;i++)for(let j=0;j<8;j++)v[i+j]+=a[i]*b[j];
    for(let k=14;k>=8;k--){let w=v[k];for(let j=0;j<8;j++)v[k-8+j]-=w*G[j];}
    return v.slice(0,8);
}
function toBasis(r){
    // (a_i/4)*(a_j/4) = sum u_k*(a_k/4); hence r=4*A*u.
    if(r.some(x=>x%4n!==0n))throw Error('nonintegral first division');
    let z=r.map(x=>x/4n),u=Array(8).fill(0n);
    for(let i=4;i<8;i++)u[i]=z[i];
    const top=[z[0]-u[4]-u[7],
        z[1]-3n*u[4]-u[5]-3n*u[7],
        z[2]-3n*u[5]-u[6],
        z[3]-3n*u[6]-u[7]];
    for(let i=0;i<4;i++){
        if(top[i]%4n!==0n)throw Error('nonintegral basis coordinate');
        u[i]=top[i]/4n;
    }
    return u;
}
const basis=A;
let count=0;
for(let i=0;i<8;i++)for(let j=0;j<=i;j++){
    toBasis(multiply(basis[i],basis[j]));count++;
}
if(count!==36)throw Error('product count');
const frob=basis.map(a=>toBasis(multiply(a,a)).map(x=>Number((x%2n+2n)%2n)));
function apply(v){
    return Array.from({length:8},(_,i)=>frob.reduce((sum,col,j)=>sum^(col[i]&v[j]),0));
}
function rank(cols){
    let rows=Array.from({length:8},(_,i)=>cols.reduce((a,col,j)=>a|(col[i]<<j),0));
    let k=0;
    for(let j=0;j<8;j++){
        let p=k;while(p<8&&!(rows[p]&(1<<j)))p++;
        if(p>=8)continue;
        [rows[k],rows[p]]=[rows[p],rows[k]];
        for(let i=0;i<8;i++)if(i!==k&&(rows[i]&(1<<j)))rows[i]^=rows[k];
        k++;
    }
    return k;
}
if(rank(frob)!==8)throw Error('Frobenius noninjective');
let q=frob,fix=[];
for(let power=0;power<3;power++){
    let identity=Array.from({length:8},(_,j)=>Array.from({length:8},(_,i)=>Number(i===j)));
    const cols=q.map((col,j)=>col.map((b,i)=>b^identity[j][i]));
    fix.push(8-rank(cols));
    q=q.map(v=>apply(v));
    if(power===1)q=q.map(v=>apply(v));
}
if(JSON.stringify(fix)!=='[2,4,8]')throw Error('Frob fixed dimensions '+fix);
const idempotents=[];
for(let m=0;m<256;m++){
    let a=Array.from({length:8},(_,i)=>(m>>i)&1);
    if(JSON.stringify(apply(a))===JSON.stringify(a))idempotents.push(m);
}
if(JSON.stringify(idempotents)!=='[0,1,240,241]')throw Error('idempotents');
console.log(JSON.stringify({basis_denominator:4,disc_basis_v2:0,
    frob_fixed_dimensions:fix,frob_rank:8,idempotents,
    index_2part:256,products_checked:36,test:'ct89-2-maximal-basis'}));
