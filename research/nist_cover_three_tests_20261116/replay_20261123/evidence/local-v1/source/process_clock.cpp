#include <sys/resource.h>
#include <sys/wait.h>
#include <fcntl.h>
#include <unistd.h>
#include <cerrno>
#include <chrono>
#include <cstdio>
#include <iostream>
#include <stdexcept>

int main(int argc,char** argv){try{
    if(argc<3)throw std::runtime_error("usage: process_clock NEW_REPORT COMMAND [ARGS...]");
    int report=open(argv[1],O_WRONLY|O_CREAT|O_EXCL|O_CLOEXEC,0600);
    if(report<0)throw std::runtime_error("clock report must be a new writable file");
    auto start=std::chrono::steady_clock::now();pid_t child=fork();
    if(child<0){close(report);throw std::runtime_error("clock fork failed");}
    if(child==0){execvp(argv[2],argv+2);std::perror("clock exec");_exit(127);}
    int status=0;struct rusage usage{};pid_t result;
    do{result=wait4(child,&status,0,&usage);}while(result<0&&errno==EINTR);
    if(result!=child){close(report);throw std::runtime_error("clock wait failed");}
    auto wall=std::chrono::duration_cast<std::chrono::microseconds>(std::chrono::steady_clock::now()-start).count();
    int code=WIFEXITED(status)?WEXITSTATUS(status):128+(WIFSIGNALED(status)?WTERMSIG(status):0);
    long long user=1000000LL*usage.ru_utime.tv_sec+usage.ru_utime.tv_usec;
    long long system=1000000LL*usage.ru_stime.tv_sec+usage.ru_stime.tv_usec;
    int written=dprintf(report,"{\"schema\":\"p192.process-clock/v1\",\"wall_us\":%lld,\"user_us\":%lld,\"system_us\":%lld,\"max_child_rss_kib\":%ld,\"exit_code\":%d,\"cpu_scope\":\"wait4 child and waited descendants; Linux\"}\n",static_cast<long long>(wall),user,system,usage.ru_maxrss,code);
    close(report);if(written<0)throw std::runtime_error("clock report write failed");return code;
}catch(const std::exception& e){std::cerr<<"REJECT "<<e.what()<<'\n';return 1;}}
