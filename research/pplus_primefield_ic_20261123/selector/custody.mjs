// Source and file-custody I/O only. All field and research calculations run in Rust.
import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import assert from 'node:assert/strict';
import { fileURLToPath } from 'node:url';

const root=path.dirname(fileURLToPath(import.meta.url));
const evidence=path.join(root,'evidence','attempt-02');
const mode=process.argv[2];
const compiler=process.argv[3];
const files=['PROTOCOL.md','production.rs','qualifier.rs','source-manifest.json','custody.mjs','run.sh'];
const hash=b=>crypto.createHash('sha256').update(b).digest('hex');
const read=f=>fs.readFileSync(f);
const write=(name,value)=>fs.writeFileSync(path.join(evidence,name),JSON.stringify(value,null,2)+'\n',{flag:'wx'});
const snapshot=()=>Object.fromEntries(files.map(name=>{
    const bytes=read(path.join(root,name));
    return [name,{sha256:hash(bytes),bytes:bytes.length}];
}));

if(mode==='source') {
    const manifest=JSON.parse(read(path.join(root,'source-manifest.json')));
    const selected=read(path.join(root,'production.rs'));
    assert.equal(hash(selected),'c6dff02843fceab1cf221465056598e6cd94d3d29a350b72eed85bee4c02c56c');
    assert.equal(hash(selected),manifest.production_module_sha256);
    const locations={
        'src/cryptanalysis/ic_framework/pkm_semaev.rs':'pkm_semaev.rs.txt',
        'src/cryptanalysis/ic_boundary.rs':'ic_boundary.rs.txt',
    };
    let offset=0;
    const sourceRecords={};
    for(const part of manifest.selected_ranges_and_wrappers) {
        const actual=selected.subarray(offset,offset+part.bytes);
        assert.equal(actual.length,part.bytes);
        assert.equal(hash(actual),part.sha256);
        if(part.kind==='verbatim_production_range') {
            const full=read(path.join(root,'..','upstream',locations[part.source_path]));
            assert.equal(hash(full),part.source_sha256);
            const blob=crypto.createHash('sha1').update(Buffer.from(`blob ${full.length}\0`)).update(full).digest('hex');
            assert.equal(blob,part.source_git_blob);
            assert.deepEqual(actual,full.subarray(part.byte_start_inclusive,part.byte_end_exclusive));
            sourceRecords[part.source_path]={git_blob:blob,sha256:hash(full),bytes:full.length};
        } else assert.equal(part.kind,'wrapper_only');
        offset+=part.bytes;
    }
    assert.equal(offset,selected.length);
    write('source-byte-check.json',{
        schema:'pplus-source-custody/v1',revision:manifest.revision,
        selected_module_sha256:hash(selected),selected_module_bytes:selected.length,
        all_declared_byte_ranges_match:true,source_records:sourceRecords,
        scope:'source custody only, no field arithmetic or result calculation',
    });
} else if(mode==='freeze') {
    assert.ok(compiler);
    const suspicious=['RUSTFLAGS','RUSTC_WRAPPER','RUSTC_WORKSPACE_WRAPPER','RUSTC_BOOTSTRAP','LD_PRELOAD','LD_LIBRARY_PATH'];
    const checkedEnvironment=Object.fromEntries(suspicious.map(name=>[name,process.env[name]??null]));
    assert.ok(!Object.values(checkedEnvironment).some(Boolean));
    write('pre-build.json',{
        schema:'pplus-custody/v2',files:snapshot(),
        compiler:{path:compiler,sha256:hash(read(compiler)),version_file_sha256:hash(read(path.join(evidence,'compiler-version.txt')))},
        checked_environment:checkedEnvironment,
        source_byte_check_sha256:hash(read(path.join(evidence,'source-byte-check.json'))),
        scope:'source, compiler and wrapper custody only',
    });
} else if(mode==='binary') {
    const pre=JSON.parse(read(path.join(evidence,'pre-build.json')));
    assert.deepEqual(snapshot(),pre.files);
    assert.equal(hash(read(pre.compiler.path)),pre.compiler.sha256);
    write('pre-execution.json',{
        files:snapshot(),compiler_sha256:pre.compiler.sha256,
        executable_sha256:hash(read(path.join(evidence,'pplus-selector-qualifier'))),
    });
} else if(mode==='finish') {
    const pre=JSON.parse(read(path.join(evidence,'pre-build.json')));
    const binaryPre=JSON.parse(read(path.join(evidence,'pre-execution.json')));
    assert.deepEqual(snapshot(),pre.files);
    assert.equal(hash(read(pre.compiler.path)),pre.compiler.sha256);
    const executableSha=hash(read(path.join(evidence,'pplus-selector-qualifier')));
    assert.equal(executableSha,binaryPre.executable_sha256);
    const outputs={};
    for(const file of fs.readdirSync(evidence).filter(x=>/\.(stdout|stderr|exit)$/.test(x))) {
        const bytes=read(path.join(evidence,file));
        assert.ok(bytes.length<=65536,`log exceeds 64 KiB; retain it and stop: ${file}`);
        outputs[file]={bytes:bytes.length,sha256:hash(bytes)};
    }
    write('post-execution.json',{
        schema:'pplus-custody/v2',files:snapshot(),executable_sha256:executableSha,
        source_hashes_unchanged:true,executable_hash_unchanged:true,compiler_hash_unchanged:true,
        raw_output_custody:outputs,all_captured_logs_within_64kib:true,
        scope:'custody only; research counts are emitted directly by the native binary',
    });
} else throw Error(`unknown custody mode: ${mode}`);
