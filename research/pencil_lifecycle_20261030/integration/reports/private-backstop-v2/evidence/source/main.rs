//! Native qualification fixture. NEVER execute on the host. No unit-test entry.
#[allow(dead_code)]
mod deadline;
use std::{os::unix::{process::CommandExt,process::ExitStatusExt},process::{Command,Stdio},time::{Duration,Instant}};
fn main() {
    if std::env::args().nth(1).as_deref()==Some("--detached-sleeper") {
        // This arm never invokes the namespace watchdog.
        if unsafe {libc::getpid()}==1 {std::process::exit(73);}
        loop {std::thread::sleep(Duration::from_secs(60));}
    }
    if unsafe {libc::getpid()}!=1 || unsafe {libc::geteuid()}!=0
        || std::env::var("ISOLAB_PRIVATE_QUALIFICATION").as_deref()!=Ok("reviewed-container-only/v1") {
        eprintln!("fixture guard rejected before watchdog installation");
        std::process::exit(72);
    }
    let begin=Instant::now();
    let cutoff=deadline::Deadline::capture(deadline::now().expect("UTC")+3).expect("admission anchor");
    let mut command=Command::new("/fixture");
    command.arg("--detached-sleeper").stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null());
    unsafe {command.pre_exec(|| {
        if libc::setsid()<0 {return Err(std::io::Error::last_os_error());}
        Ok(())
    });}
    let mut child=command.spawn().expect("detached child spawn");
    let pid=child.id();
    eprintln!("fixture_admitted pid=1 euid=0 child_pid={pid} cutoff_utc={} monotonic_anchor=captured",cutoff.utc_seconds);
    std::thread::Builder::new().name("reap-observer".into()).spawn(move || {
        loop {
            match child.try_wait() {
                Ok(Some(status))=>{
                    // Best-effort observation; absence remains incomplete, never inferred.
                    eprintln!("detached_child_reaped signal={:?} code={:?} elapsed_ns={}",status.signal(),status.code(),begin.elapsed().as_nanos());
                    return;
                }
                Ok(None)=>std::thread::sleep(Duration::from_millis(1)),
                Err(error)=>{eprintln!("detached_child_wait_error={error}");return;}
            }
        }
    }).expect("observer thread");
    deadline::install_namespace_backstop(cutoff,"controller-owned-private-pid-namespace/v1").expect("backstop install");
    eprintln!("watchdog_installed main_thread_blocking");
    // No polling of Deadline on this main thread; the watchdog owns termination.
    loop {unsafe {libc::pause();}}
}
