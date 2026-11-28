// Toolchain installation only; this does not execute cryptographic experiments.
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const { spawn } = require('node:child_process');
const root = __dirname;
const plan = JSON.parse(fs.readFileSync(path.join(root, 'rust-toolchain-install-plan.json')));
const receiptPath = path.join(root, 'rust-toolchain-install-receipt.json');
const receiptFd = fs.openSync(receiptPath, 'wx', 0o600);
const logFd = fs.openSync(path.join(root, 'rust-toolchain-install.log'), 'wx', 0o600);
const started = Date.now();
const receipt = { started_utc: new Date(started).toISOString(), plan_sha256: crypto.createHash('sha256').update(fs.readFileSync(path.join(root, 'rust-toolchain-install-plan.json'))).digest('hex'), steps: [], peak_cgroup_bytes: 0, stop_reason: null };
let child;
let stopped = false;
function stop(reason) { if (!stopped) { stopped = true; receipt.stop_reason = reason; if(child) { try { process.kill(-child.pid, 'SIGTERM'); } catch {} } } }
const guard = setInterval(() => {
  const used = Number(fs.readFileSync('/sys/fs/cgroup/memory.current', 'utf8'));
  receipt.peak_cgroup_bytes = Math.max(receipt.peak_cgroup_bytes, used);
  if(used >= plan.limits.cgroup_memory_stop_bytes) stop('physical-memory-limit');
  if(Date.now() - started >= plan.limits.wall_seconds * 1000) stop('setup-deadline');
}, 250);
async function run(cmd, args, cwd) {
  if(stopped) throw new Error(receipt.stop_reason);
  const step = {cmd, args, cwd: cwd || root, started_utc: new Date().toISOString()};
  receipt.steps.push(step);
  fs.writeSync(logFd, JSON.stringify({command:cmd,args})+'\n');
  const code = await new Promise((resolve, reject) => {
    child = spawn(cmd, args, {cwd:cwd || root, stdio:['ignore', logFd, logFd], detached:true});
    child.on('error', reject);
    child.on('exit', (code,signal) => {step.exit_code=code;step.signal=signal;child=null;resolve(code);});
  });
  step.finished_utc = new Date().toISOString();
  if(code !== 0 || stopped) throw new Error(receipt.stop_reason || `${cmd} exited ${code}`);
}
(async () => {
  try {
    const scratch = path.join(root, 'rust-installer-downloads');
    fs.mkdirSync(scratch, {recursive:true});
    fs.mkdirSync(plan.install_prefix, {recursive:true});
    for(const pkg of plan.packages) {
      const archive = path.join(scratch,pkg.filename);
      await run('curl',['--fail','--location','--silent','--show-error','--connect-timeout','20','--max-time','180','--output',archive,pkg.url]);
      const hash = crypto.createHash('sha256').update(fs.readFileSync(archive)).digest('hex');
      if(hash !== pkg.sha256) throw new Error(`digest mismatch for ${pkg.component}`);
      receipt.steps.push({component:pkg.component,sha256:hash,archive_bytes:fs.statSync(archive).size,digest_matches:true});
      await run('tar',['-xJf',archive,'-C',scratch]);
      const extracted = path.join(scratch,pkg.filename.replace(/\.tar\.xz$/,''));
      await run('sh',['install.sh',`--prefix=${plan.install_prefix}`,'--disable-ldconfig'],extracted);
      // Only these newly downloaded, checksum-verified installer copies are removed.
      fs.rmSync(extracted,{recursive:true});
      fs.unlinkSync(archive);
      process.stdout.write(`Installed ${pkg.component}; official archive digest matched.\n`);
    }
    for(const exe of ['rustc','cargo','rustfmt','cargo-clippy']) await run(path.join(plan.install_prefix,'bin',exe),['--version']);
    receipt.status = 'installed';
  } catch(error) { receipt.status='failed';receipt.error=String(error);process.exitCode=1; }
  finally {
    clearInterval(guard);
    receipt.finished_utc=new Date().toISOString();
    fs.writeFileSync(receiptFd,JSON.stringify(receipt,null,2)+'\n');
    fs.closeSync(receiptFd); fs.closeSync(logFd);
    process.stdout.write(JSON.stringify({status:receipt.status,stop_reason:receipt.stop_reason,peak_cgroup_bytes:receipt.peak_cgroup_bytes,receipt:receiptPath})+'\n');
  }
})();
