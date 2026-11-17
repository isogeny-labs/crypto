#define _GNU_SOURCE
#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
static const char *mount_fixture;
static int fixture_open(const char *path,int flags){
 if(strcmp(path,"/proc/self/mountinfo")==0){if(!mount_fixture){errno=ENOENT;return -1;}return open(mount_fixture,flags);}
 return open(path,flags);
}
#define open fixture_open
#define main original_gate_main
#include "gate.c"
#undef main
#undef open
#ifndef FIXTURES
#error FIXTURES must name the bound immutable fixture directory
#endif
static int checks;
#define CHECK(x) do {if(!(x)){fprintf(stderr,"CONTROL_FAILURE line%d: %s\n",__LINE__,#x);return 1;}checks++;} while(0)
static char path[1024];
static const char *fixture(const char *name){int n=snprintf(path,sizeof path,"%s/%s",FIXTURES,name);if(n<0||(size_t)n>=sizeof path)abort();return path;}
int main(void){
 if(getpid()==1){fprintf(stderr,"Refusing controls at PID1\n");return 2;}
 const char *nonce="01234567-89ab-cdef-0123-456789abcdef";
 CHECK(nonce_ok(nonce));
 const char *bad[]={"","01234567-89ab-cdef-0123-456789abcde","01234567-89ab-cdef-0123-456789abcdef0","01234567-89Ab-cdef-0123-456789abcdef","01234567_89ab-cdef-0123-456789abcdef","01234567-89ab-cdeg-0123-456789abcdef"};
 for(size_t i=0;i<sizeof bad/sizeof bad[0];i++)CHECK(!nonce_ok(bad[i]));
 puts("PASS nonce");
 const char *mounts[]={"mount-good","mount-rw","mount-fstype","mount-root","mount-duplicate","mount-missing","mount-overlong","mount-truncated"};
 for(size_t i=0;i<sizeof mounts/sizeof mounts[0];i++){mount_fixture=fixture(mounts[i]);CHECK(mount_ok()==(i==0));}
 mount_fixture=NULL;CHECK(!mount_ok());puts("PASS mountinfo");
 int dir=open(FIXTURES,O_RDONLY|O_DIRECTORY|O_CLOEXEC);CHECK(dir>=0);
 const char *vals[]={"value-good","value-empty","value-no-newline","value-two","value-max","value-64","value-65","value-nul"};
 for(size_t i=0;i<sizeof vals/sizeof vals[0];i++)CHECK(exact_at(dir,vals[i],"1\n")==(i==0));
 CHECK(!exact_at(dir,"absent-fixed-control","1\n"));CHECK(close(dir)==0);puts("PASS exact-values");
 const char *readnames[]={"value-empty","value-good","value-64","value-65"};const int expected[]={0,2,-1,-1};
 for(size_t i=0;i<4;i++){int fd=open(fixture(readnames[i]),O_RDONLY|O_CLOEXEC);CHECK(fd>=0);char buf[65];CHECK(bounded(fd,buf,64)==expected[i]);CHECK(close(fd)==0);}
 puts("PASS bounded-reader");
 char *v[]={"gate","Q1",(char*)nonce,NULL};
 v[1]="Q1";CHECK(original_gate_main(3,v)==126);
 v[1]="Q2";CHECK(original_gate_main(3,v)==126);
 v[1]="Q3";CHECK(original_gate_main(3,v)==126);
 v[1]="Q4";CHECK(original_gate_main(3,v)==126);
 v[1]="Q1";v[2]="invalid";CHECK(original_gate_main(3,v)==126);
 CHECK(original_gate_main(2,v)==126);puts("PASS host-negative-and-fixed-cases");
 printf("ALL_CONTROLS_PASS checks=%d\n",checks);return 0;
}
