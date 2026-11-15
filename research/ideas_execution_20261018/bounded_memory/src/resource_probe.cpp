// Small Linux wait4 launcher. The child has a hard 512 MiB address-space cap.
#include <chrono>
#include <cstring>
#include <fstream>
#include <iostream>
#include <sys/resource.h>
#include <sys/wait.h>
#include <unistd.h>
int main(int argc,char**argv){
 if(argc<3){std::cerr<<"resource_probe RESULT.json COMMAND...\n";return 2;}
 auto start=std::chrono::steady_clock::now();pid_t child=fork();
 if(child<0){std::cerr<<"fork failed\n";return 2;}
 if(child==0){struct rlimit cap{512ULL*1024*1024,512ULL*1024*1024};if(setrlimit(RLIMIT_AS,&cap)!=0)_exit(126);execvp(argv[2],argv+2);std::cerr<<"exec failed: "<<strerror(errno)<<"\n";_exit(127);}
 struct rusage usage{};int status=0;while(wait4(child,&status,0,&usage)<0){if(errno!=EINTR)return 2;}
 int code=WIFEXITED(status)?WEXITSTATUS(status):128+WTERMSIG(status);double elapsed=std::chrono::duration<double>(std::chrono::steady_clock::now()-start).count();
 std::ofstream out(argv[1]);out<<"{\"exit_status\":"<<code<<",\"signal\":"<<(WIFSIGNALED(status)?WTERMSIG(status):0)<<",\"elapsed_seconds\":"<<elapsed<<",\"user_seconds\":"<<usage.ru_utime.tv_sec+usage.ru_utime.tv_usec/1e6<<",\"system_seconds\":"<<usage.ru_stime.tv_sec+usage.ru_stime.tv_usec/1e6<<",\"max_rss_kib\":"<<usage.ru_maxrss<<",\"address_space_cap_bytes\":536870912}\n";return code;
}
