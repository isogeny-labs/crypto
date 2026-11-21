// Source/executable bookkeeping, with no mathematical result computation.
const fs=require('fs'),crypto=require('crypto'),path=require('path');
const [version,binary,...amendments]=process.argv.slice(2);
if(!/^\d+$/.test(version)||!binary)throw Error('usage: freeze VERSION BINARY [AMENDMENT...]');
const base='research/prime_field_transfer_goal_20261116/';
const source=base+'round-04/',receipts=base+'receipts/round-04/';
const qualifier='/tmp/prime-transfer-round04-qualifier-'+version.padStart(2,'0');
const hash=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
if(fs.existsSync(qualifier))throw Error('refusing qualifier replacement');
for(const name of fs.readdirSync(receipts).filter(n=>/^build-manifest-\d+\.json$/.test(n))){
  const previous=JSON.parse(fs.readFileSync(receipts+name));
  for(const pin of previous.pins)if(hash(pin.path)!==pin.sha256)throw Error('changed prior input '+pin.path);
}
const files=fs.readdirSync(source+'v'+version).filter(n=>/\.(hpp|cpp)$/.test(n))
  .map(n=>source+'v'+version+'/'+n).concat([
  source+'native_base.hpp',source+'PROTOCOL.md',source+'provenance.cjs',__filename,
  base+'round-02/local_torsion.cpp',...amendments]);
fs.mkdirSync(qualifier);
const pins=[];
for(const input of files){
  const relative=path.relative(base,input),output=path.join(qualifier,relative);
  if(relative.startsWith('..'))throw Error('input outside study');
  fs.mkdirSync(path.dirname(output),{recursive:true});fs.copyFileSync(input,output,fs.constants.COPYFILE_EXCL);
  fs.chmodSync(output,0o444);pins.push({path:input,sha256:hash(input),qualifier_path:output,qualifier_sha256:hash(output)});
}
const executable=qualifier+'/recognizer';fs.copyFileSync(binary,executable,fs.constants.COPYFILE_EXCL);fs.chmodSync(executable,0o555);
fs.writeFileSync(receipts+'build-manifest-'+version.padStart(2,'0')+'.json',JSON.stringify({
  schema:'development.provenance/v1',phase:'build',timestamp:new Date().toISOString(),qualifier,pins,
  executable:{path:executable,sha256:hash(executable)}},null,2)+'\n',{flag:'wx'});
console.log('frozen qualifier '+version+'; '+pins.length+' source pins');
