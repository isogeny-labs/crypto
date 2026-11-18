// LaTeX layout only: allow line breaks in exact recorded hex digests.
import fs from 'node:fs';
for(const p of process.argv.slice(2)){
 const tex=fs.readFileSync(p,'utf8');
 fs.writeFileSync(p,tex.replace(/\b[a-f0-9]{40,64}\b/g,h=>h.match(/.{1,8}/g).join('\\allowbreak{}')));
}
