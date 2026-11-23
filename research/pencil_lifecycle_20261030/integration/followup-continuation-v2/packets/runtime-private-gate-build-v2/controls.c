#define _GNU_SOURCE
#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <time.h>
#include <unistd.h>
/* Only uid/gid are mocked; all other fstat data is real filesystem metadata. */
static int mock_owner=1;
static int control_fstat(int fd,struct stat *s){
 int result=fstat(fd,s);
 if(result==0){if(mock_owner==1){s->st_uid=0;s->st_gid=0;}else if(mock_owner==2){s->st_uid=123;s->st_gid=0;}else if(mock_owner==3){s->st_uid=0;s->st_gid=123;}}
 return result;
}
#define fstat control_fstat
#define main never_executed_gate_main
#include "../runtime-private-qualifier/gate.c"
#undef main
#undef fstat
static int checks;
#define CHECK(x) do{if(!(x)){fprintf(stderr,"FAIL line%d %s\n",__LINE__,#x);return 1;}checks++;}while(0)
static int write_token(int dir,const char *bytes,size_t n,mode_t mode){
 int fd=openat(dir,"gate-release",O_WRONLY|O_CREAT|O_TRUNC|O_NOFOLLOW,0600);
 if(fd<0)return 0;
 int ok=write(fd,bytes,n)==(ssize_t)n&&fchmod(fd,mode)==0&&fsync(fd)==0;
 close(fd);return ok;
}
int main(void){
 CHECK(getpid()!=1&&getuid()!=0);
 char directory[]="/tmp/private-gate-controls-XXXXXX";CHECK(mkdtemp(directory)!=NULL);
 int dir=open(directory,O_RDONLY|O_DIRECTORY|O_CLOEXEC);CHECK(dir>=0);
 const char *nonce="01234567-89ab-cdef-0123-456789abcdef";
 char token[39];snprintf(token,sizeof token,"%s\n",nonce);
 CHECK(release_token(dir,nonce)==0);
 for(size_t i=0;i<37;i++){CHECK(write_token(dir,token,i,0444));CHECK(release_token(dir,nonce)==-1);CHECK(unlinkat(dir,"gate-release",0)==0);}
 CHECK(write_token(dir,token,37,0444));CHECK(release_token(dir,nonce)==1);CHECK(await_release(dir,nonce)==1);
 mock_owner=0;CHECK(release_token(dir,nonce)==-1);mock_owner=2;CHECK(release_token(dir,nonce)==-1);mock_owner=3;CHECK(release_token(dir,nonce)==-1);mock_owner=1;
 CHECK(unlinkat(dir,"gate-release",0)==0);
 CHECK(write_token(dir,token,37,0400));CHECK(release_token(dir,nonce)==-1);CHECK(unlinkat(dir,"gate-release",0)==0);
 token[36]='X';CHECK(write_token(dir,token,37,0444));CHECK(release_token(dir,nonce)==-1);CHECK(unlinkat(dir,"gate-release",0)==0);token[36]='\n';token[37]='X';
 CHECK(write_token(dir,token,38,0444));CHECK(release_token(dir,nonce)==-1);CHECK(unlinkat(dir,"gate-release",0)==0);
 CHECK(write_token(dir,token,37,0444));CHECK(linkat(dir,"gate-release",dir,"alias",0)==0);CHECK(release_token(dir,nonce)==-1);CHECK(unlinkat(dir,"alias",0)==0);CHECK(unlinkat(dir,"gate-release",0)==0);
 CHECK(symlinkat("missing",dir,"gate-release")==0);CHECK(release_token(dir,nonce)==-1);CHECK(unlinkat(dir,"gate-release",0)==0);
 CHECK(mkfifoat(dir,"gate-release",0444)==0);CHECK(release_token(dir,nonce)==-1);CHECK(unlinkat(dir,"gate-release",0)==0);
 CHECK(mkdirat(dir,"gate-release",0700)==0);CHECK(release_token(dir,nonce)==-1);CHECK(unlinkat(dir,"gate-release",AT_REMOVEDIR)==0);
 token[0]='f';CHECK(write_token(dir,token,37,0444));CHECK(release_token(dir,nonce)==-1);
 struct timespec before,after;CHECK(clock_gettime(CLOCK_MONOTONIC,&before)==0);CHECK(await_release(dir,nonce)==0);CHECK(clock_gettime(CLOCK_MONOTONIC,&after)==0);
 double elapsed=(after.tv_sec-before.tv_sec)+(after.tv_nsec-before.tv_nsec)/1e9;CHECK(elapsed>=15.0&&elapsed<20.0);
 CHECK(unlinkat(dir,"gate-release",0)==0);CHECK(close(dir)==0);CHECK(rmdir(directory)==0);
 printf("{\"status\":\"helper_controls_passed\",\"checks\":%d,\"partial_lengths\":37,\"real_timeout_seconds\":%.6f,\"uid_gid_mocked\":true,\"whole_gate_executed\":false}\n",checks,elapsed);
 return 0;
}
