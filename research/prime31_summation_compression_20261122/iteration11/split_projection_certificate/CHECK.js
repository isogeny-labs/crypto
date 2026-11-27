// Deterministic integer model only; node CHECK.js.
let cells=0,rootChecks=0,exactChecks=0,singletonChecks=0,falseAccept=0;
const vals=[0,1,2,3];
for(let s=0;s<16;s++){
 const roots=vals.filter(i=>(s>>i)&1);
 const q=z=>roots.reduce((a,b)=>a*(z-b),1);
 for(const z of vals){if((q(z)===0)!==Boolean((s>>z)&1))throw Error("roots");rootChecks++;}
 for(let t=0;t<16;t++){
  cells++;
  const contained=vals.every(i=>!((t>>i)&1)||q(i)===0);
  if(contained!==((t&~s)===0))throw Error("subset");
  if((contained&&(s&~t)===0)!==(s===t))throw Error("exact");exactChecks++;
  if(contained&&s!==t)falseAccept++;
  if(roots.length===1){if((contained&&t!==0)!==(s===t))throw Error("singleton");singletonChecks++;}
 }
}
if(cells!==256||rootChecks!==64||exactChecks!==256||singletonChecks!==64||falseAccept!==65)throw Error("counts");
console.log({cells,rootChecks,exactChecks,singletonChecks,falseAccept});
