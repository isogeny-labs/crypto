#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <cstdint>
#include <ctime>
#include <unistd.h>
#include <sys/resource.h>
int main(int argc,char**argv){if(argc!=5)return 3;int delay=atoi(argv[1]),rc=atoi(argv[2]);const char*mode=argv[3];FILE*f=fopen(argv[4],"w");if(!f)return 4;fputs("post-exec main reached\n",f);fclose(f);volatile char*mem=nullptr;if(!strcmp(mode,"alloc")){mem=(volatile char*)malloc(8*1024*1024);if(!mem)return 5;for(int i=0;i<8*1024*1024;i+=4096)mem[i]=1;}
if(!strcmp(mode,"partial")){fputs("{partial",stdout);fflush(stdout);}timespec wait{delay/1000,(delay%1000)*1000000L};nanosleep(&wait,nullptr);if(!strcmp(mode,"bad")){fputs("not-json",stdout);fflush(stdout);_exit(rc);}timespec t{};clock_gettime(CLOCK_MONOTONIC,&t);rusage u{};getrusage(RUSAGE_SELF,&u);unsigned long long ns=(unsigned long long)t.tv_sec*1000000000ULL+t.tv_nsec;printf("{\"exitstamp_ns\":%llu,\"child_peak_rss_kib\":%ld}\n",ns,u.ru_maxrss);fflush(stdout);_exit(rc);}
