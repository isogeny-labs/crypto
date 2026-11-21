#pragma once
// Included inside namespace weber. Classical division polynomials on y²=x³+1.
// Keeping coordinate polynomials integral avoids rational-function reduction
// at every group operation. Every division below is checked exact.
struct Coordinate {
    P real,imag;
};
inline P coordinate_equation(){return {E(1),E(0),E(0),E(1)};}
inline Coordinate coordinate_mul(const Coordinate& a,const Coordinate& b){
    return {a.real*b.real+coordinate_equation()*(a.imag*b.imag),
            a.real*b.imag+a.imag*b.real};
}
inline Coordinate coordinate_sub(const Coordinate& a,const Coordinate& b){
    return {a.real-b.real,a.imag-b.imag};
}
inline Coordinate coordinate_power(Coordinate a,unsigned n){
    Coordinate out{{E(1)},{}};
    while(n){if(n&1)out=coordinate_mul(out,a);n>>=1;if(n)a=coordinate_mul(a,a);}
    return out;
}
struct DivisionPolynomials {
    std::vector<Coordinate> memo;
    std::vector<bool> ready;
    explicit DivisionPolynomials(unsigned n):memo(n+1),ready(n+1,false){
        require(n>=4,"division cache too small");
        memo[0]={};memo[1]={{E(1)},{}};memo[2]={{},{E(2)}};
        memo[3]={{E(0),E(12),E(0),E(0),E(3)},{}};
        memo[4]={{},{E(-32),E(0),E(0),E(80),E(0),E(0),E(4)}};
        for(unsigned j=0;j<=4;++j)ready[j]=true;
    }
    const Coordinate& psi(unsigned n){
        require(n<memo.size(),"division cache index");if(ready[n])return memo[n];
        unsigned m=n/2;Coordinate value;
        if(n&1)value=coordinate_sub(
            coordinate_mul(psi(m+2),coordinate_power(psi(m),3)),
            coordinate_mul(psi(m-1),coordinate_power(psi(m+1),3)));
        else{
            Coordinate bracket=coordinate_sub(
                coordinate_mul(psi(m+2),coordinate_power(psi(m-1),2)),
                coordinate_mul(psi(m-2),coordinate_power(psi(m+1),2)));
            Coordinate numerator=coordinate_mul(psi(m),bracket);
            require(numerator.imag.empty(),"even division parity");
            auto qr=divide(numerator.real,scale(coordinate_equation(),E(2)));
            require(qr.second.empty(),"nonexact division by2y");value.imag=std::move(qr.first);
        }
        require((n&1)?value.imag.empty():value.real.empty(),"division polynomial parity");
        memo[n]=std::move(value);ready[n]=true;return memo[n];
    }
};
inline F weber_function(P numerator,P denominator){
    require(!denominator.empty(),"zero coordinate denominator");
    size_t first=0;while(denominator[first].zero())++first;unsigned residue=first%3;
    auto compress=[residue](const P& input){
        P out(input.size()/3+1);
        for(size_t j=0;j<input.size();++j)if(!input[j].zero()){
            require(j%3==residue,"coordinate expression is not Weber invariant");out[j/3]=input[j];
        }
        trim(out);return out;
    };
    return {compress(numerator),compress(denominator)};
}
inline SymbolicPoint times(unsigned n){
    require(n>0,"symbolic identity has no affine functions");if(n==1)return {};
    DivisionPolynomials cache(n+2);
    Coordinate psi=cache.psi(n),square=coordinate_power(psi,2);
    require(square.imag.empty(),"division square parity");
    Coordinate adjacent=coordinate_mul(cache.psi(n-1),cache.psi(n+1));
    require(adjacent.imag.empty(),"division adjacent parity");
    P x{E(0),E(1)},denominator=x*square.real;
    F f=weber_function(denominator-adjacent.real,denominator);
    Coordinate numerator=coordinate_sub(
        coordinate_mul(cache.psi(n+2),coordinate_power(cache.psi(n-1),2)),
        coordinate_mul(cache.psi(n-2),coordinate_power(cache.psi(n+1),2)));
    P gdenominator,gnumerator;
    if(n&1){
        require(numerator.imag.empty(),"odd ordinate parity");
        gnumerator=numerator.real;
        gdenominator=scale(coordinate_equation()*coordinate_power(psi,3).real,E(4));
    }else{
        require(numerator.real.empty(),"even ordinate parity");
        gnumerator=numerator.imag;
        P cube=psi.imag*psi.imag*psi.imag,g=coordinate_equation();
        gdenominator=scale(g*g*cube,E(4));
    }
    return {f,weber_function(gnumerator,gdenominator)};
}
