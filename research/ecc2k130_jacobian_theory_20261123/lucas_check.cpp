// Exact arithmetic proof-check, not a curve search or a benchmark.
#include <algorithm>
#include <iostream>
#include <string>
using I = __int128_t;
std::string dec(I x) { if (!x) return "0"; bool neg=x<0; if(neg)x=-x; std::string s; while(x){s+=char('0'+x%10);x/=10;} if(neg)s+='-'; std::reverse(s.begin(),s.end());return s; }
int main(){ I a=0,b=1; for(int n=2;n<=131;n++){I c=-b-2*a;a=b;b=c; if(n==3||n==5||n==131)std::cout<<"U"<<n<<"="<<dec(b)<<"\n";} }
