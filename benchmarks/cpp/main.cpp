#include <algorithm>
#include <chrono>
#include <cstdint>
#include <iomanip>
#include <iostream>
#include <tuple>
#include <vector>
struct S { double a,b,c; };
struct P { std::uint32_t target,key; double value; };
int main() {
 constexpr std::size_t N=200000, D=6, STEPS=8;
 auto start=std::chrono::steady_clock::now();
 std::vector<std::size_t> off; std::vector<std::uint32_t> edges;
 off.reserve(N+1); edges.reserve(N*D); off.push_back(0);
 for(std::size_t i=0;i<N;++i){for(std::size_t k=0;k<D;++k) edges.push_back((i*37+k*7919+1)%N); off.push_back(edges.size());}
 std::vector<S> a(N),b(N);
 for(std::size_t i=0;i<N;++i) a[i]={double(i%101)*.01,double((i*17)%97)*.001,double((i*31)%89)*.0001};
 std::vector<P> ps; ps.reserve(N/5+1);
 for(std::size_t step=0;step<STEPS;++step){
  for(std::size_t i=0;i<N;++i){double sum=0;for(std::size_t e=off[i];e<off[i+1];++e) sum+=a[edges[e]].b*double((e*17)%13+1)*1e-6;b[i]={a[i].a+sum,a[i].b+a[i].a*1e-7,a[i].c+(a[i].a+sum)*1e-9};}
  ps.clear();for(std::size_t i=0;i<N;i+=5) ps.push_back({std::uint32_t((i*7919+17)%N),std::uint32_t(i),a[i].c+double(step)*1e-8});
  std::sort(ps.begin(),ps.end(),[](const P&x,const P&y){return std::tie(x.target,x.key)<std::tie(y.target,y.key);});
  std::uint32_t prev=UINT32_MAX;for(const auto&p:ps)if(p.target!=prev){b[p.target].a=p.value;prev=p.target;}
  a.swap(b);
 }
 double sum=0;for(std::size_t i=0;i<N;++i)sum+=a[i].a*double(1+i%7)+a[i].b*double(1+i%11)+a[i].c;
 double ms=std::chrono::duration<double,std::milli>(std::chrono::steady_clock::now()-start).count();
 std::cout<<std::setprecision(17)<<"{\"language\":\"cpp\",\"entities\":"<<N<<",\"relations\":"<<N*D<<",\"steps\":"<<STEPS<<",\"elapsed_ms\":"<<ms<<",\"checksum\":"<<sum<<"}\n";
}