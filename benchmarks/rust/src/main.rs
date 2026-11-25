use std::time::Instant;
const N:usize=200_000; const D:usize=6; const STEPS:usize=8;
#[derive(Clone,Copy,Default)] struct S{a:f64,b:f64,c:f64}
#[derive(Clone,Copy)] struct P{target:u32,key:u32,value:f64}
fn main(){
 let start=Instant::now();let mut off=Vec::with_capacity(N+1);let mut edges=Vec::with_capacity(N*D);off.push(0);
 for i in 0..N{for k in 0..D{edges.push(((i*37+k*7919+1)%N) as u32);}off.push(edges.len());}
 let mut a=Vec::with_capacity(N);let mut b=vec![S::default();N];
 for i in 0..N{a.push(S{a:(i%101) as f64*0.01,b:((i*17)%97) as f64*0.001,c:((i*31)%89) as f64*0.0001});}
 let mut ps=Vec::with_capacity(N/5+1);
 for step in 0..STEPS{
  for i in 0..N{let mut sum=0.0_f64;for e in off[i]..off[i+1]{sum+=a[edges[e] as usize].b*((e*17%13+1) as f64)*1e-6;}b[i]=S{a:a[i].a+sum,b:a[i].b+a[i].a*1e-7,c:a[i].c+(a[i].a+sum)*1e-9};}
  ps.clear();for i in (0..N).step_by(5){ps.push(P{target:((i*7919+17)%N) as u32,key:i as u32,value:a[i].c+step as f64*1e-8});}
  ps.sort_by_key(|p|(p.target,p.key));let mut prev=None;for p in &ps{if prev!=Some(p.target){b[p.target as usize].a=p.value;prev=Some(p.target);}}
  std::mem::swap(&mut a,&mut b);
 }
 let mut sum=0.0_f64;for(i,s)in a.iter().enumerate(){sum+=s.a*(1+i%7) as f64+s.b*(1+i%11) as f64+s.c;}
 let ms=start.elapsed().as_secs_f64()*1000.0;
 println!("{{\"language\":\"rust\",\"entities\":{},\"relations\":{},\"steps\":{},\"elapsed_ms\":{:.9},\"checksum\":{:.17}}}",N,N*D,STEPS,ms,sum);
}