#include "identity_arithmetic.hpp"
int main(int argc,char** argv) {
    try {
        require(argc==2,"usage: identity OUTPUT.json");
        const auto t=var("t"),r=var("r"),s=var("s");
        issue(argv[1],"Generic residual quadratic root-product expansion",{"t","r","s"},
              mul({sub(t,r),sub(t,s)}),
              add({::power(t,2),neg(mul({add({r,s}),t})),mul({r,s})}),2);
    } catch(const std::exception& error) { std::cerr << "REJECT " << error.what() << '\n'; return 1; }
}
