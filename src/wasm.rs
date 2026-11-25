use wasm_bindgen::prelude::*;

#[derive(Clone)]
struct Point { x:f64, y:f64, a:f64, b:f64 }

#[wasm_bindgen]
pub struct World {
    points: Vec<Point>,
    seed: u64,
    tick: u64,
    coupling: f64,
    noise: f64,
    radius: f64,
    width: f64,
    height: f64,
}

impl World {
    fn random(&mut self) -> f64 {
        self.seed = self.seed.wrapping_add(0x9e3779b97f4a7c15);
        let mut z=self.seed;
        z=(z^(z>>30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z=(z^(z>>27)).wrapping_mul(0x94d049bb133111eb);
        ((z^(z>>31))>>11) as f64 / ((1u64<<53) as f64)
    }
    fn wrap(v:f64, max:f64)->f64 { v.rem_euclid(max) }
}

#[wasm_bindgen]
impl World {
    #[wasm_bindgen(constructor)]
    pub fn new(count:usize, seed:u32) -> World {
        let mut w=World { points:Vec::with_capacity(count.min(2000)), seed:seed as u64, tick:0, coupling:0.08, noise:0.008, radius:0.055, width:1.0, height:1.0 };
        for _ in 0..count.clamp(8,2000) {
            let x=w.random(); let y=w.random(); let a=w.random(); let b=w.random();
            w.points.push(Point{x,y,a,b});
        }
        w
    }
    pub fn step(&mut self, steps:u32) {
        for _ in 0..steps.min(1000) {
            let n=self.points.len();
            let mut da=vec![0.0;n]; let mut db=vec![0.0;n];
            let r2=self.radius*self.radius;
            for i in 0..n { for j in (i+1)..n {
                let mut dx=self.points[j].x-self.points[i].x;
                let mut dy=self.points[j].y-self.points[i].y;
                if dx>0.5 {dx-=1.0} else if dx< -0.5 {dx+=1.0}
                if dy>0.5 {dy-=1.0} else if dy< -0.5 {dy+=1.0}
                let d2=dx*dx+dy*dy;
                if d2<r2 {
                    let weight=(1.0-d2/r2)*self.coupling*0.025;
                    da[i]+=(self.points[j].a-self.points[i].a)*weight;
                    da[j]+=(self.points[i].a-self.points[j].a)*weight;
                    db[i]+=(self.points[j].b-self.points[i].b)*weight;
                    db[j]+=(self.points[i].b-self.points[j].b)*weight;
                }
            }}
            for i in 0..n {
                let na=(self.random()-0.5)*self.noise;
                let nb=(self.random()-0.5)*self.noise;
                let p=&mut self.points[i];
                p.a=(p.a+da[i]+na).clamp(0.0,1.0);
                p.b=(p.b+db[i]+nb).clamp(0.0,1.0);
                p.x=Self::wrap(p.x+(p.a-0.5)*0.00045+(self.random()-0.5)*0.00015,1.0);
                p.y=Self::wrap(p.y+(p.b-0.5)*0.00045+(self.random()-0.5)*0.00015,1.0);
            }
            self.tick+=1;
        }
    }
    pub fn set_coupling(&mut self, value:f64) { self.coupling=value.clamp(0.0,2.0); }
    pub fn set_noise(&mut self, value:f64) { self.noise=value.clamp(0.0,0.1); }
    pub fn set_radius(&mut self, value:f64) { self.radius=value.clamp(0.005,0.25); }
    pub fn count(&self)->usize { self.points.len() }
    pub fn tick(&self)->u64 { self.tick }
    pub fn positions(&self)->Vec<f64> {
        let mut out=Vec::with_capacity(self.points.len()*2);
        for p in &self.points {out.push(p.x);out.push(p.y);}
        out
    }
    pub fn channels(&self)->Vec<f64> {
        let mut out=Vec::with_capacity(self.points.len()*2);
        for p in &self.points {out.push(p.a);out.push(p.b);}
        out
    }
    pub fn metrics(&self)->Vec<f64> {
        let n=self.points.len() as f64;
        let mut ma=0.0;let mut mb=0.0;
        for p in &self.points {ma+=p.a;mb+=p.b;}
        ma/=n;mb/=n;
        let mut va=0.0;let mut vb=0.0;
        for p in &self.points {va+=(p.a-ma).powi(2);vb+=(p.b-mb).powi(2);}
        vec![n,ma,mb,(va/n).sqrt(),(vb/n).sqrt()]
    }
    pub fn state_hash(&self)->String {
        let mut h=0xcbf29ce484222325u64;
        for p in &self.points {
            for v in [p.x,p.y,p.a,p.b] {for b in v.to_bits().to_le_bytes(){h^=b as u64;h=h.wrapping_mul(0x100000001b3);}}
        }
        format!("{:016x}",h)
    }
}
