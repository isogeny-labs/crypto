#define _GNU_SOURCE
#include <errno.h>
#include <fcntl.h>
#include <stdint.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/statfs.h>
#include <sys/statvfs.h>
#include <time.h>
#include <unistd.h>

#define CGROUP2_MAGIC 0x63677270L
static const char *good_request="8a8cdbda79fd92a2fc0d62d3add10e6f8a79ce7222c4dba491ae703645e3ac92";
static const char *wrong_request="0a8cdbda79fd92a2fc0d62d3add10e6f8a79ce7222c4dba491ae703645e3ac92";
static const char *wrong_owner="04136fa355b3678a1146ad16f7e8649e94fb4fc21fe77e8310c060f61caaff8a";
extern char **environ;
static int fail(const char *s){fprintf(stderr,"qualification gate: %s\n",s);return 126;}
static int bounded(int fd,char *buf,size_t cap){
 size_t used=0;
 while(used<cap){ssize_t n=read(fd,buf+used,cap-used);if(n<0){if(errno==EINTR)continue;return -1;}if(n==0){buf[used]=0;return (int)used;}used+=(size_t)n;}
 return -1; /* cap is payload capacity; callers allocate cap+1 */
}
static int exact_at(int dir,const char *name,const char *wanted){
 int f=openat(dir,name,O_RDONLY|O_NOFOLLOW|O_CLOEXEC|O_NONBLOCK);if(f<0)return 0;
 struct stat st;char b[65];int n=-1;
 if(fstat(f,&st)==0&&S_ISREG(st.st_mode))n=bounded(f,b,64);
 int saved=errno;close(f);errno=saved;
 return n>=0&&(size_t)n==strlen(wanted)&&memcmp(b,wanted,(size_t)n)==0;
}
static int exact_path(const char *name,const char *wanted){
 int f=open(name,O_RDONLY|O_NOFOLLOW|O_CLOEXEC|O_NONBLOCK);if(f<0)return 0;
 char b[257];int n=bounded(f,b,256);close(f);
 return n>=0&&(size_t)n==strlen(wanted)&&memcmp(b,wanted,(size_t)n)==0;
}
static int nonce_ok(const char *s){
 if(strlen(s)!=36)return 0;
 for(int i=0;i<36;i++){if(i==8||i==13||i==18||i==23){if(s[i]!='-')return 0;}else if(!((s[i]>='0'&&s[i]<='9')||(s[i]>='a'&&s[i]<='f')))return 0;}
 return 1;
}
static int mount_ok(void){
 int fd=open("/proc/self/mountinfo",O_RDONLY|O_NOFOLLOW|O_CLOEXEC|O_NONBLOCK);if(fd<0)return 0;
 char b[16385];int n=bounded(fd,b,16384);close(fd);if(n<0)return 0;
 int found=0;char *line_save=NULL;
 for(char *line=strtok_r(b,"\n",&line_save);line;line=strtok_r(NULL,"\n",&line_save)){
  char *sep=strstr(line," - ");if(!sep)continue;
  if(strncmp(sep+3,"cgroup2 ",8)!=0)continue;
  *sep=0;char *save=NULL,*part=strtok_r(line," ",&save);int col=1;char *root=NULL,*point=NULL,*opts=NULL;
  while(part){if(col==4)root=part;if(col==5)point=part;if(col==6)opts=part;col++;part=strtok_r(NULL," ",&save);}
  if(point&&strcmp(point,"/sys/fs/cgroup")==0){
   if(++found!=1||!root||strcmp(root,"/")!=0||!opts)return 0;
   int ro=0;char *osave=NULL;for(char *o=strtok_r(opts,",",&osave);o;o=strtok_r(NULL,",",&osave)){if(strcmp(o,"rw")==0)return 0;if(strcmp(o,"ro")==0)ro=1;}
   if(!ro)return 0;
  }
 }
 return found==1;
}
/* Private-runtime amendment: no tested program runs before an exact nonce file.
 * Source only: new binary/control/review evidence is required before admission. */
static int release_token(int work,const char *nonce){
 int f=openat(work,"gate-release",O_RDONLY|O_NOFOLLOW|O_CLOEXEC|O_NONBLOCK);
 if(f<0)return errno==ENOENT?0:-1;
 struct stat st;char b[39];int n=-1;
 if(fstat(f,&st)==0&&S_ISREG(st.st_mode)&&st.st_uid==0&&st.st_gid==0&&
    (st.st_mode&07777)==0444&&st.st_nlink==1&&st.st_size==37)n=bounded(f,b,38);
 close(f);
 return n==37&&memcmp(b,nonce,36)==0&&b[36]=='\n'?1:-1;
}
static void stopped(int signum){(void)signum;_exit(126);}
static int await_release(int work,const char *nonce){
 struct timespec start,now,pause={0,10000000};
 if(clock_gettime(CLOCK_MONOTONIC,&start)!=0)return 0;
 for(;;){
  if(clock_gettime(CLOCK_MONOTONIC,&now)!=0||now.tv_sec<start.tv_sec||
    now.tv_sec-start.tv_sec>15||(now.tv_sec-start.tv_sec==15&&now.tv_nsec>=start.tv_nsec))return 0;
  /* Docker archive extraction may expose incomplete bytes or metadata.
   * Only the exact completed token releases; every other observation waits
   * within the same fixed deadline and can never authorize exec. */
  int value=release_token(work,nonce);if(value==1)return 1;
  if(nanosleep(&pause,NULL)!=0)return 0; /* signal: fail closed, no exec */
 }
}
int main(int argc,char **argv){
 struct sigaction sa;memset(&sa,0,sizeof sa);sa.sa_handler=stopped;sigemptyset(&sa.sa_mask);
 if(sigaction(SIGTERM,&sa,NULL)!=0||sigaction(SIGINT,&sa,NULL)!=0)return fail("signal handler");
 if(argc!=3||!nonce_ok(argv[2])||(strcmp(argv[1],"Q1")&&strcmp(argv[1],"Q2")&&strcmp(argv[1],"Q3")))return fail("fixed case and nonce required");
 if(getpid()!=1||getuid()!=1000||geteuid()!=1000||getgid()!=1000||getegid()!=1000)return fail("PID/UID/GID boundary");
 if(!exact_path("/proc/self/cgroup","0::/\n")||!mount_ok())return fail("private cgroup v2 mount mapping");
 int d=open("/",O_RDONLY|O_DIRECTORY|O_CLOEXEC);if(d<0)return fail("root descriptor");
 const char *parts[]={"sys","fs","cgroup"};
 for(size_t i=0;i<3;i++){int next=openat(d,parts[i],O_RDONLY|O_DIRECTORY|O_NOFOLLOW|O_CLOEXEC);close(d);d=next;if(d<0)return fail("cgroup directory descriptor");}
 struct statfs fs;struct statvfs vfs;struct stat st;
 if(fstatfs(d,&fs)!=0||fs.f_type!=CGROUP2_MAGIC||fstatvfs(d,&vfs)!=0||!(vfs.f_flag&ST_RDONLY)||fstat(d,&st)!=0){close(d);return fail("kernel cgroup2 read-only filesystem");}
 if(!exact_at(d,"pids.max","1\n")||!exact_at(d,"pids.current","1\n")||!exact_at(d,"cgroup.procs","1\n")){close(d);return fail("effective PID controller observation");}
 struct timespec ts;if(clock_gettime(CLOCK_MONOTONIC,&ts)!=0||ts.tv_sec<0||ts.tv_nsec<0||ts.tv_nsec>=1000000000){close(d);return fail("observation clock");}
 char receipt[1024];int n=snprintf(receipt,sizeof receipt,"{\"schema\":\"software.preexec-pids/v1\",\"case\":\"%s\",\"nonce\":\"%s\",\"pid\":1,\"uid\":1000,\"gid\":1000,\"cgroup_mapping\":\"0::/\",\"cgroup_device\":%llu,\"cgroup_inode\":%llu,\"pids_max\":1,\"pids_current\":1,\"monotonic_seconds\":%llu,\"monotonic_nanoseconds\":%lu,\"boundary\":\"before_tested_program_exec\"}\n",argv[1],argv[2],(unsigned long long)st.st_dev,(unsigned long long)st.st_ino,(unsigned long long)ts.tv_sec,(unsigned long)ts.tv_nsec);
 if(n<0||(size_t)n>=sizeof receipt){close(d);return fail("receipt length");}
 /* Recheck effective limit immediately before publishing the pre-exec observation. */
 if(!exact_at(d,"pids.max","1\n")||!exact_at(d,"pids.current","1\n")){close(d);return fail("PID limit changed before exec");}
 int work=open("/work",O_RDONLY|O_DIRECTORY|O_NOFOLLOW|O_CLOEXEC);
 struct statfs workfs;struct stat tokenst;
 if(work<0||fstatfs(work,&workfs)!=0||workfs.f_type!=0x01021994L){close(d);if(work>=0)close(work);return fail("release tmpfs");}
 errno=0;
 if(fstatat(work,"gate-release",&tokenst,AT_SYMLINK_NOFOLLOW)==0||errno!=ENOENT){close(d);close(work);return fail("preexisting release token");}
 size_t sent=0;while(sent<(size_t)n){ssize_t w=write(STDOUT_FILENO,receipt+sent,(size_t)n-sent);if(w<0&&errno==EINTR)continue;if(w<=0)return fail("receipt write");sent+=(size_t)w;}
 if(!await_release(work,argv[2])){close(work);close(d);return fail("release token timeout or mismatch");}
 close(work);
 if(!exact_at(d,"pids.max","1\n")||!exact_at(d,"pids.current","1\n")||!exact_at(d,"cgroup.procs","1\n")){close(d);return fail("PID limit changed at release");}
 close(d);
 if(strcmp(argv[1],"Q3")==0){char *v[]={"/usr/local/bin/isolab-fresh-owner","--check","/fixtures/owner-invalid.json",(char*)wrong_owner,NULL};execve(v[0],v,environ);}
 else{char *v[]={"/usr/local/bin/ecbench","diagnostic-preflight","--request","/fixtures/request.json","--request-sha256",(char*)(strcmp(argv[1],"Q1")==0?good_request:wrong_request),NULL};execve(v[0],v,environ);}
 fprintf(stderr,"qualification gate: exec failed\n");return 127;
}
