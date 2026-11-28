#include <algorithm>
#include <array>
#include <cassert>
#include <cmath>
#include <cstdint>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <map>
#include <numeric>
#include <set>
#include <string>
#include <vector>
#include "rng-local.hpp"

constexpr int prime_count=8, curves_per_prime=6, bootstrap_count=10'000;
using Key=std::array<int,7>; // field, curve, B offset, base, seed, law, v
using Row=std::array<long long,20>;

Row parse(std::string line) {
    Row result{};
    std::size_t start=0;
    for (unsigned i=0;i<result.size();++i) {
        std::size_t end=line.find(',',start);
        assert((end==std::string::npos)==(i==result.size()-1));
        result[i]=std::stoll(line.substr(start,end==std::string::npos?end:end-start));
        start=end+1;
    }
    return result;
}

struct Summary {
    double mean=0, low=0, high=0;
    int wins=0,ties=0,losses=0;
};

Summary summarize(const std::vector<double>& delta) {
    assert(delta.size()==prime_count*curves_per_prime);
    Summary out;
    for (auto x:delta) {
        out.mean+=x/delta.size();
        if (x>1e-12) ++out.wins;
        else if (x<-1e-12) ++out.losses;
        else ++out.ties;
    }
    Random rng{0x202611178001ULL};
    std::vector<double> resampled;
    for (int i=0;i<bootstrap_count;++i) {
        double sum=0;
        for (int f=0;f<prime_count;++f) for (int j=0;j<curves_per_prime;++j)
            sum+=delta[f*curves_per_prime+rng.bounded(curves_per_prime)];
        resampled.push_back(sum/delta.size());
    }
    std::sort(resampled.begin(),resampled.end());
    out.low=resampled[249];
    out.high=resampled[9749];
    return out;
}

double avg(const std::vector<double>& v) {
    return std::accumulate(v.begin(),v.end(),0.0)/v.size();
}

int main(int argc,char** argv) {
    assert(argc==3);
    std::ifstream raw(argv[1]);
    std::ofstream paired(argv[2]);
    assert(raw && paired);
    std::string line;
    std::getline(raw,line);
    assert(line.starts_with("field_id,"));
    std::map<Key,Row> rows;
    std::set<std::array<int,3>> curve_size;
    long long checks=0,recoveries=0,enumerated=0;
    while (std::getline(raw,line)) {
        auto row=parse(line);
        Key key{int(row[0]),int(row[1]),int(row[6]),int(row[8]),int(row[9]),int(row[10]),int(row[11])};
        assert(rows.emplace(key,row).second);
        assert(row[15]*row[15]<=row[16]*row[17]);
        recoveries+=row[19];
        if (key[5]==0 && key[6]==0) { checks+=row[18]; enumerated+=row[12]; }
        curve_size.insert({key[0],key[1],key[2]});
    }
    assert(rows.size()==25920 && curve_size.size()==144);
    paired<<"field_id,curve_id,p,a,b,r,relative_B,B,v,law_set,interval,dickson,random_mean,random_sd,random_min,random_max,dickson_interval_pp,dickson_random_pp\n";
    std::cout<<std::fixed<<std::setprecision(4)
             <<"rows "<<rows.size()<<" independent signed group checks "<<checks
             <<" labelled recoveries "<<recoveries<<" signed tuples "<<enumerated<<'\n';
    paired<<std::setprecision(10);
    auto count=[&](int f,int c,int b,int arm,int seed,int law_set,int v,bool raw_coverage) {
        const auto& first=rows.at({f,c,b,arm,seed,0,v});
        double hits=0;
        if (raw_coverage) return double(first[14])/first[5];
        if (!law_set) return double(first[17])/first[5];
        for (int law=1;law<5;++law)
            hits+=double(rows.at({f,c,b,arm,seed,law,v})[17])/(4*first[5]);
        return hits;
    };
    auto report=[&](std::string name,const std::vector<double>& values) {
        auto s=summarize(values);
        std::cout<<name<<" "<<s.mean<<" ["<<s.low<<", "<<s.high<<"] "
                 <<"wins/ties/losses "<<s.wins<<'/'<<s.ties<<'/'<<s.losses<<'\n';
    };
    for (int b=-1;b<=1;++b) for (int v=0;v<=1;++v) for (int law_set=0;law_set<=1;++law_set) {
        std::vector<double> vs_interval, vs_random, interval, dickson, random_per_curve, random_sd_per_curve;
        for (int f=0;f<prime_count;++f) for (int c=0;c<curves_per_prime;++c) {
            double i=count(f,c,b,0,0,law_set,v,false), d=count(f,c,b,1,0,law_set,v,false);
            std::vector<double> samples;
            for (int seed=0;seed<16;++seed) samples.push_back(count(f,c,b,2,seed,law_set,v,false));
            double r=avg(samples), var=0;
            for (auto x:samples) var+=(x-r)*(x-r)/15;
            const double sd=std::sqrt(var);
            const auto& cell=rows.at({f,c,b,0,0,0,v});
            auto [lo,hi]=std::minmax_element(samples.begin(),samples.end());
            paired<<f<<','<<c<<','<<cell[2]<<','<<cell[3]<<','<<cell[4]<<','<<cell[5]<<','
                  <<b<<','<<cell[7]<<','<<v<<','<<law_set<<','<<i<<','<<d<<','<<r<<','
                  <<sd<<','<<*lo<<','<<*hi<<','<<(d-i)*100<<','<<(d-r)*100<<'\n';
            vs_interval.push_back((d-i)*100);
            vs_random.push_back((d-r)*100);
            interval.push_back(i*100);
            dickson.push_back(d*100);
            random_per_curve.push_back(r*100);
            random_sd_per_curve.push_back(sd*100);
        }
        std::cout<<"B offset "<<b<<" v "<<v<<" seeded "<<law_set<<", coverage percent: interval "
                 <<avg(interval)<<" Dickson "<<avg(dickson)<<" random "<<avg(random_per_curve)
                 <<", random seed within-curve SD mean "<<avg(random_sd_per_curve)<<'\n';
        report("  Dickson minus interval pp",vs_interval);
        report("  Dickson minus random pp",vs_random);
    }
    for (int v=0;v<=1;++v) for (int arm=0;arm<3;++arm) {
        std::vector<double> delta;
        for (int f=0;f<prime_count;++f) for (int c=0;c<curves_per_prime;++c) {
            double sum=0;
            int replicates=arm==2?16:1;
            for (int i=0;i<replicates;++i) {
                sum+=count(f,c,0,arm,i,1,v,false)-count(f,c,0,arm,i,0,v,false);
            }
            delta.push_back(100*sum/replicates);
        }
        report("B0 seeded minus equal pp v="+std::to_string(v)+" arm="+std::to_string(arm),delta);
    }
    for (int arm=0;arm<3;++arm) {
        std::vector<double> raw_hits, ratios, gaps;
        for (int f=0;f<prime_count;++f) for (int c=0;c<curves_per_prime;++c) {
            int replicates=arm==2?16:1;
            for (int i=0;i<replicates;++i) {
                raw_hits.push_back(count(f,c,0,arm,i,0,0,true));
                for (int j=1;j<5;++j) {
                    const auto& row=rows.at({f,c,0,arm,i,j,0});
                    double n=row[15], r=row[5], e=row[16];
                    const double analytical_null=n+n*(n-1)/r;
                    ratios.push_back(e/analytical_null);
                    gaps.push_back(row[17]/r-n*n/r/e);
                }
            }
        }
        std::cout<<"B0 arm="<<arm<<" raw support "<<100*avg(raw_hits)<<"% "
                 <<"informative energy / iid-group null "<<avg(ratios)
                 <<" lower-bound mean gap "<<100*avg(gaps)<<" pp\n";
    }
}
