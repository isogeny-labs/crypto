// Offline transport/receipt integrity only; no AWS calls or scientific calculations.
import fs from 'node:fs';
import crypto from 'node:crypto';
const study='research/twist_routes_1_3_20261021',execution=study+'/execution-v2';
const report=study+'/reports/publication-20261025-setec',output=process.argv[2];
if(!output||fs.existsSync(output))throw Error('fresh check receipt required');
const read=p=>JSON.parse(fs.readFileSync(p,'utf8'));
const sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const meta=read(execution+'/RAW_ARTIFACTS.json'),assessment=read(report+'/ASSESSMENT.json');
const dir=execution+'/publication-20261025-setec-v1',receipt=read(dir+'/RECEIPT.json');
const downloadedArchive=process.argv[3]||receipt.downloaded_path;
const expectedRemote='s3://setec-astronomy-artifacts/store';
if(assessment.remote!==expectedRemote||receipt.remote!==expectedRemote||read(study+'/store/config.json').remote!==expectedRemote||meta.remote!==expectedRemote)throw Error('wrong artifact destination');
if(receipt.status!=='remote-object-and-fresh-download-verified'||!receipt.conditional_write_performed||!receipt.conditional_write_checksum_verified||!receipt.fresh_download_verified||!receipt.remote_verified)throw Error('publication incomplete');
if(receipt.sha256!==meta.archive.sha256||receipt.bytes!==meta.archive.bytes||sha(downloadedArchive)!==receipt.sha256||fs.statSync(downloadedArchive).size!==receipt.bytes)throw Error('download identity mismatch');
if(receipt.credentials_persisted||receipt.credential_values_in_receipts)throw Error('credential persistence');
if(sha(dir+'/PUBLISHER.mjs')!==receipt.script_sha256)throw Error('publisher snapshot mismatch');
for(const step of receipt.steps){
  if(step.exit_code!==0)throw Error('failed publication step');
  for(const suffix of ['stdout','stderr']){
    const p=dir+'/'+step.label+'.'+suffix;
    if(sha(p)!==step[suffix+'_sha256'])throw Error('output hash mismatch');
    if(/(?:AKIA|ASIA)[A-Z0-9]{16}/.test(fs.readFileSync(p,'utf8')))throw Error('key-shaped output');
  }
}
const indexPath=study+'/store/index.json',index=read(indexPath).entries[receipt.name];
if(!index||index.sha256!==receipt.sha256||index.bytes!==receipt.bytes||sha(indexPath)!==receipt.index_after_sha256)throw Error('native index mismatch');
const members=read(dir+'/ARCHIVE_CHECK.json');
if(members.status!=='all-original-member-bytes-verified'||members.complete_tree_members_verified!==36083||members.archived_members_verified!==35979||members.loose_metadata_restored_and_verified!==104||members.unexpected_members!==0)throw Error('member check incomplete');
if(members.manifest_sha256!==sha(execution+'/RAW_MEMBERS_FINAL.json')||members.verifier_sha256!==sha(study+'/verify-published-archive.mjs'))throw Error('member-check provenance mismatch');
const replayDir=execution+'/publication-20261025-setec-replay',replay=read(replayDir+'/gp.json'),gate=read(replayDir+'/adjudication/ADJUDICATION.json');
if(!gate.accepted||!gate.raw_hashes_match||!gate.script_hash_matches||!gate.completion_marker_present||gate.unaccepted_error_lines.length!==0)throw Error('native CAS gate rejected');
if(!replay.historical_input_byte_identity||!replay.historical_tool_byte_identity||replay.exit_code!==0||sha(replay.argv.at(-1))!==replay.script_sha256||sha(replayDir+'/REPLAYER.sh')!==replay.replayer_sha256)throw Error('frozen CAS replay mismatch');
for(const suffix of ['stdout','stderr'])if(sha(replayDir+'/gp.'+suffix)!==replay[suffix+'_sha256'])throw Error('CAS output changed');
for(const name of ['publication-20261025-destination-discovery','publication-20261025-destination-discovery-v2']){
  const p=execution+'/'+name,r=read(p+'/RECEIPT.json');
  if(sha(p+'/INSPECTOR.mjs')!==r.script_sha256||!r.read_only||r.remote_write_performed)throw Error('discovery receipt mismatch');
}
if(assessment.policy_applied||assessment.full_scientific_rerun||assessment.independent_host_replay||assessment.scientific_results_changed||assessment.performance_claim)throw Error('unsupported promotion');
const files=fs.readdirSync(report).filter(n=>!/^REPORT\.(aux|log|out|tex)$/.test(n)&&fs.statSync(report+'/'+n).isFile()).map(n=>({path:report+'/'+n,sha256:sha(report+'/'+n),bytes:fs.statSync(report+'/'+n).size}));
fs.writeFileSync(output,JSON.stringify({schema:'twist.corrected-publication-integrity/v1',status:'passed',remote:expectedRemote,archive_sha256:receipt.sha256,archive_bytes:receipt.bytes,publication_receipt_sha256:sha(dir+'/RECEIPT.json'),member_check_sha256:sha(dir+'/ARCHIVE_CHECK.json'),native_adjudication_sha256:sha(replayDir+'/adjudication/ADJUDICATION.json'),offline_only:true,scientific_calculations_performed:false,files},null,2)+'\n');
console.log('Corrected destination, native index, download, member-check and replay receipt integrity: passed');
