#pragma once
// Exhaustive finite-module replay of archived observations, not curve point counting.
#include "json.hpp"
#include <algorithm>
#include <array>
#include <functional>
#include <iostream>
#include <set>
using V=std::array<int,6>;
using M=std::array<V,6>;
using Basis=std::vector<V>;
int m3(int x){return (x%3+3)%3;}
int dot(V a,V b){int n=0;for(int i=0;i<6;++i)n+=a[i]*b[i];return n%3;}
V act(V v,M a){V r{};for(int j=0;j<6;++j)for(int i=0;i<6;++i)r[j]=(r[j]+v[i]*a[i][j])%3;return r;}
M mul(M a,M b){M c{};for(int i=0;i<6;++i)c[i]=act(a[i],b);return c;}
M ident(){M a{};for(int i=0;i<6;++i)a[i][i]=1;return a;}
int rank(Basis a){int r=0;for(int c=0;c<6;++c){int k=r;while(k<int(a.size())&&a[k][c]==0)++k;if(k==int(a.size()))continue;
    std::swap(a[k],a[r]);int z=a[r][c];for(int j=0;j<6;++j)a[r][j]=a[r][j]*z%3;
    for(int i=0;i<int(a.size());++i){if(i!=r){z=a[i][c];for(int j=0;j<6;++j)a[i][j]=m3(a[i][j]-z*a[r][j]);}}
    ++r;}return r;}
bool contains(const Basis& b,V v){for(auto r:b){int c=0;while(c<6&&r[c]==0)++c;require(c<6,"zero basis row");int z=v[c];for(int j=0;j<6;++j)v[j]=m3(v[j]-z*r[j]);}return v==V{};}
bool stable(const Basis& b,M a){for(auto v:b)if(!contains(b,act(v,a)))return false;return true;}
bool isotropic(const Basis& b,M j){for(auto v:b)for(auto w:b)if(dot(act(v,j),w))return false;return true;}
std::vector<Basis> grass(int k){std::vector<Basis> out;std::vector<int> piv;
    std::function<void(int)> choose=[&](int start){if(int(piv.size())<k){for(int c=start;c<6;++c){piv.push_back(c);choose(c+1);piv.pop_back();}return;}
        Basis b(k);std::vector<std::pair<int,int>> slots;for(int i=0;i<k;++i){b[i][piv[i]]=1;for(int c=piv[i]+1;c<6;++c)if(std::find(piv.begin(),piv.end(),c)==piv.end())slots.emplace_back(i,c);}
        std::function<void(size_t)> fill=[&](size_t n){if(n==slots.size()){out.push_back(b);return;}auto [i,c]=slots[n];for(int x=0;x<3;++x){b[i][c]=x;fill(n+1);}};fill(0);
    };choose(0);return out;}
std::vector<V> elements(const Basis& b){std::vector<V> out(1);for(auto v:b){auto old=out;for(int s=1;s<3;++s)for(auto w:old){for(int c=0;c<6;++c)w[c]=(w[c]+s*v[c])%3;out.push_back(w);}}return out;}
void print_basis(const Basis& b){std::cout<<'[';bool first=true;for(auto v:b){if(!first)std::cout<<',';first=false;std::cout<<'[';for(int i=0;i<6;++i)std::cout<<(i?",":"")<<v[i];std::cout<<']';}std::cout<<']';}
void block(M& a,const Json& j,int n,int offset){require(j.arr.size()==size_t(n),"matrix row count");for(int i=0;i<n;++i){require(j.arr[i].arr.size()==size_t(n),"matrix column count");for(int k=0;k<n;++k){int x=j.arr[i].arr[k].integer();require(x>=0&&x<3,"noncanonical F3 entry");a[offset+i][offset+k]=x;}}}
