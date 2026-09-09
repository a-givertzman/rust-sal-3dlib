//! Расчет [осадки в произвольной точке](https://github.com/a-givertzman/sss/blob/master/design/algorithm/part03_draft/chapter01_floatingPosition/chapter02_draftPoint.md)
use crate::math::Position;
//
pub struct Draught {
    midel_x: f64,
    draught_mid: f64,
    tg_t: f64,
    tg_h: f64,
    cos_h: f64,
}
//
impl Draught {
    //
    pub fn new(heel_deg: f64, trim_deg: f64, midel_x: f64, draught_mid: f64) -> Self {
        let heel_r = heel_deg.to_radians();
        let trim_r = trim_deg.to_radians();        
        let tg_t = trim_r.tan();
        let tg_h = heel_r.tan();
        let cos_h = heel_r.cos();
        //println!("midel_x:{midel_x} heel:{heel} trim:{trim} heel_r:{heel_r} trim_r:{trim_r} draught_mid:{draught_mid} tg_h:{tg_h} tg_t:{tg_t} cos_h:{cos_h}");
        Self {
            midel_x,
            draught_mid,
            tg_t,
            tg_h,
            cos_h,
        }
    }
    //
    pub fn value(&self, p: &Position) -> f64 {
        let d_zi = p.y() * self.tg_h + (p.x() - self.midel_x) * self.tg_t / self.cos_h;        
      //  println!("p:{} d_zi:{d_zi} z_fix:{z_fix}", p.print());
        self.draught_mid + d_zi
    }
    // Осадна на носовом перпендикуляре
    pub fn bow(&self, length_lbp: f64) -> f64 {
        let p = Position::new(length_lbp, 0.0, -self.draught_mid);
        self.value(&p)
    } 
    // Осадна на кормовом перпендикуляре
    pub fn stern(&self) -> f64 {
        let p = Position::new(0.0, 0.0, -self.draught_mid);
        self.value(&p)
    }
    // Средняя осадка
    pub fn mean(&self, waterline_x: f64, waterline_y: f64) -> f64 {
        let p = Position::new(waterline_x, waterline_y, -self.draught_mid);
        self.value(&p)
    }  
}
