// Operational byte-preserving preservation of measured executables; no computation.
import fs from 'node:fs';import crypto from 'node:crypto';import {spawn} from 'node:child_process';
const s='research/twist_routes_1_3_20261021',out=s+'/execution-v19/reproduction';
fs.mkdirSync(out+'/executables',{recursive:true});const entries=[
 ['cold-worker','/workspace/shared/twist-cold-native-20261029/4a46570f358cf6c8184e4cae6c4fed5b57058fa2/binary'],
 ['cold-harness','/workspace/shared/twist-cold-harness-native-20261029/1960985246c31436009d9b7ccc53fad57a1afa0e/binary'],
 ['cold-summary-original','/workspace/shared/twist-cold-summary-native-20261029/5e16ddd8a9d3dc51fc059b53589ae0247296cca1/binary'],
 ['cold-summary-corrected','/workspace/shared/twist-cold-validation-target-20261030/release/twist-cold-summary-v2'],
 ['sparse-worker','/workspace/shared/twist-sparse-native-20261030/bfcf5b743775327a0273b2ae4d53f1d982d29c10/worker'],
 ['sparse-harness','/workspace/shared/twist-sparse-native-20261030/bfcf5b743775327a0273b2ae4d53f1d982d29c10/harness'],
 ['sparse-summary','/workspace/shared/twist-sparse-native-20261030/bfcf5b743775327a0273b2ae4d53f1d982d29c10/summary'],
 ['roi.so','/workspace/shared/twist-cold-native-20261029/4a46570f358cf6c8184e4cae6c4fed5b57058fa2/roi.so'],
 ['gp','/workspace/shared/twist-tools-20261023/cas/root/usr/bin/gp'],
 ['valgrind','/workspace/shared/twist-tools-20261025-e26/root/usr/bin/valgrind'],
 ['valgrind.bin','/workspace/shared/twist-tools-20261025-e26/root/usr/bin/valgrind.bin'],
 ['callgrind-amd64-linux','/workspace/shared/twist-tools-20261025-e26/root/usr/libexec/valgrind/callgrind-amd64-linux'],
 ['vgpreload_core-amd64-linux.so','/workspace/shared/twist-tools-20261025-e26/root/usr/libexec/valgrind/vgpreload_core-amd64-linux.so']
];
const sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');const records=[];
for(const [name,p]of entries){const target=out+'/executables/'+name;fs.copyFileSync(p,target,fs.constants.COPYFILE_EXCL);fs.chmodSync(target,fs.statSync(p).mode&0o777);if(sha(target)!==sha(p))throw Error('copy mismatch');records.push({member:'executables/'+name,original_path:p,sha256:sha(p),bytes:fs.statSync(p).size});}
fs.writeFileSync(out+'/EXECUTABLES.json',JSON.stringify({schema:'twist.measured-executables/v1',files:records,scope:'Byte preservation of measured Linux x86-64 executables. Original runtime/library/host/command pins still apply; no new experiment or independent-host replay.'},null,2)+'\n',{flag:'wx'});
const child=spawn('/bin/bash',['-o','pipefail','-c',"tar --sort=name --mtime='2026-10-30 00:00:00Z' --owner=0 --group=0 --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - executables EXECUTABLES.json | xz -6 -T1 > executables.tar.xz"],{cwd:out,stdio:'inherit'});const code=await new Promise((r,j)=>{child.on('error',j);child.on('close',r);});if(code)throw Error('pack failed '+code);fs.writeFileSync(out+'/executables.tar.xz.sha256',sha(out+'/executables.tar.xz')+'  executables.tar.xz\n',{flag:'wx'});console.log({files:records.length,sha256:sha(out+'/executables.tar.xz'),bytes:fs.statSync(out+'/executables.tar.xz').size});
