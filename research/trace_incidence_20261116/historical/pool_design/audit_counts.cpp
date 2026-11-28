#include <cstdint>
#include <iostream>
#include <vector>

// Independent point-count path: evaluate the cubic directly, rather than
// the generator's finite-difference recurrence. Small fields additionally
// use Euler's criterion rather than a residue lookup.
static uint64_t power(uint64_t a,uint64_t e,uint64_t p) {
    uint64_t r=1;
    while(e){if(e&1)r=r*a%p;a=a*a%p;e>>=1;}
    return r;
}
int main(){
    uint64_t p,a,b,claimed,previous=0;
    std::vector<bool> squares;
    bool ok=true;
    while(std::cin>>p>>a>>b>>claimed){
        if(p!=previous){
            squares.assign(p,false);
            for(uint64_t y=0;y<=(p-1)/2;++y)squares[y*y%p]=true;
            previous=p;
        }
        uint64_t count=1;
        for(uint64_t x=0;x<p;++x){
            uint64_t value=((x*x%p)*x+a*x+b)%p;
            if(!value){++count;continue;}
            bool is_square = p<100000 ? power(value,(p-1)/2,p)==1 : squares[value];
            if(is_square)count+=2;
        }
        std::cout<<"{\"p\":"<<p<<",\"a\":"<<a<<",\"b\":"<<b
                 <<",\"claimed\":"<<claimed<<",\"independent_count\":"<<count
                 <<",\"matches\":"<<(count==claimed?"true":"false")<<"}\n";
        ok=ok && count==claimed;
    }
    return ok?0:1;
}
