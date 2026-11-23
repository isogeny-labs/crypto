use std::fs::OpenOptions;
use std::os::unix::io::AsRawFd;
use std::path::Path;
// The extracted primitive's signal path is not exercised: all acquisitions use wait=false.
mod cryptanalysis { pub mod ecbench { pub mod signals { pub fn interrupted() -> Option<i32> { None } } } }
include!("benchlock.rs");

struct Child { pid: libc::pid_t, ready: i32, release: i32 }
impl Child {
    fn finish(mut self) {
        send(self.release);
        let mut status=0;
        assert_eq!(unsafe{libc::waitpid(self.pid,&mut status,0)},self.pid);
        self.pid=0;
        assert!(libc::WIFEXITED(status)); assert_eq!(libc::WEXITSTATUS(status),0);
        println!("child_reaped status={status}");
    }
}
impl Drop for Child {
    fn drop(&mut self) { unsafe {
        libc::close(self.ready); libc::close(self.release);
        if self.pid>0 { libc::kill(self.pid,libc::SIGKILL); let mut status=0; libc::waitpid(self.pid,&mut status,0); }
    } }
}
// These functions are used only on the parent side.
fn send(fd:i32) { let b=1u8; assert_eq!(unsafe{libc::write(fd,(&b as *const u8).cast(),1)},1); }
fn recv(fd:i32) { let mut b=0u8; assert_eq!(unsafe{libc::read(fd,(&mut b as *mut u8).cast(),1)},1); assert_eq!(b,1); }
fn child_holding(fd:i32, guard:Option<&mut BenchLock>, destroy_body:bool)->Child {
    let mut ready=[0;2]; let mut release=[0;2];
    assert_eq!(unsafe{libc::pipe(ready.as_mut_ptr())},0);
    assert_eq!(unsafe{libc::pipe(release.as_mut_ptr())},0);
    let pid=unsafe{libc::fork()}; assert!(pid>=0);
    if pid==0 { unsafe {
        // No heap allocation, formatting, panic or Rust unwinding on this path.
        libc::alarm(15); libc::close(ready[0]); libc::close(release[1]);
        if destroy_body {
            if let Some(g)=guard { g.release_owner_lock(); }
            else { libc::_exit(90); }
            // Execute descriptor close without dropping String/allocation state.
            if libc::close(fd)!=0 { libc::_exit(91); }
        }
        let b=1u8;
        if libc::write(ready[1],(&b as *const u8).cast(),1)!=1 { libc::_exit(92); }
        let mut c=0u8;
        if libc::read(release[0],(&mut c as *mut u8).cast(),1)!=1 || c!=1 { libc::_exit(93); }
        if !destroy_body && libc::close(fd)!=0 { libc::_exit(94); }
        libc::close(ready[1]); libc::close(release[0]); libc::_exit(0);
    } }
    unsafe{libc::close(ready[1]);libc::close(release[0]);}
    let child=Child{pid,ready:ready[0],release:release[1]}; recv(child.ready);
    println!("child_ready pid={pid} destructor_body={destroy_body}"); child
}
fn probe(path:&Path, expected:bool) {
    let f=OpenOptions::new().write(true).open(path).unwrap();
    let rc=unsafe{libc::flock(f.as_raw_fd(),libc::LOCK_EX|libc::LOCK_NB)};
    let errno=if rc==0 {0} else {std::io::Error::last_os_error().raw_os_error().unwrap()};
    println!("probe acquired={} errno={errno}",rc==0);
    assert_eq!(rc==0,expected);
    if !expected {assert!(errno==libc::EAGAIN||errno==libc::EWOULDBLOCK);}
}
fn acquire(path:&Path)->BenchLock {BenchLock::acquire(path.to_str().unwrap(),false).unwrap()}
fn early_return(path:&Path, child:&mut Option<Child>)->Result<(),&'static str> {
    let g=acquire(path);
    *child=Some(child_holding(g._file.as_raw_fd(),None,false));
    Err("intentional setup failure")
}
fn main() {
    unsafe{libc::alarm(15);}
    let root=std::env::args().nth(1).expect("unique output directory argument");
    let root=Path::new(&root); std::fs::create_dir(root).unwrap();
    println!("parent_pid={}",std::process::id());
    let path=root.join("c1"); let g=acquire(&path); probe(&path,false); drop(g);probe(&path,true);println!("C1 PASS");
    let path=root.join("c2");let f=OpenOptions::new().create(true).truncate(false).write(true).open(&path).unwrap();
    assert_eq!(unsafe{libc::flock(f.as_raw_fd(),libc::LOCK_EX|libc::LOCK_NB)},0);
    let flags=unsafe{libc::fcntl(f.as_raw_fd(),libc::F_GETFD)}; assert!(flags>=0 && flags&libc::FD_CLOEXEC!=0);println!("C2 fd_cloexec=true");
    let child=child_holding(f.as_raw_fd(),None,false);drop(f);probe(&path,false);child.finish();probe(&path,true);println!("C2 PASS");
    let path=root.join("c3");let g=acquire(&path);let child=child_holding(g._file.as_raw_fd(),None,false);drop(g);probe(&path,true);child.finish();println!("C3 PASS");
    let path=root.join("c4");let mut g=acquire(&path);let fd=g._file.as_raw_fd();let child=child_holding(fd,Some(&mut g),true);probe(&path,false);drop(g);probe(&path,true);child.finish();println!("C4 PASS");
    let path=root.join("c6");let mut child=None;
    assert_eq!(early_return(&path,&mut child),Err("intentional setup failure"));
    probe(&path,true);child.unwrap().finish();println!("C6 PASS");
    // All forks completed before the first Rust-created thread.
    let path=root.join("c5");let g=acquire(&path);std::thread::spawn(move||drop(g)).join().unwrap();probe(&path,true);println!("C5 PASS");
    println!("ALL_SIX_CONTROLS_PASS");
}
