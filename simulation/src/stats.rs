use std::{ops, f32};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone, Copy)]
pub struct Stats {
    pub hp: f32,
    pub sp: f32,
    pub status:[i32;1]
    //FIXME add speed/enum
}

impl Stats {
    fn regulate(&mut self, nrm:Stats) {
        //this function approches the values 
        //adding to stamina, decrementing status
        //hp is ignored (allows for overflow of hp)
        //i.e. hp is unregulated
        todo!()
    }
}

impl ops::AddAssign for Stats {
    fn add_assign(&mut self, rhs: Self) {
        self.hp += rhs.hp;
        self.sp += rhs.sp;
        for i in 0..=self.status.len() {
            self.status[i] += rhs.status[i]
        }
    }
}

impl ops::MulAssign for Stats {
    fn mul_assign(&mut self, rhs: Self) {
        self.hp *= rhs.hp;
        self.sp *= rhs.sp;
    }
}

impl ops::SubAssign for Stats {
    fn sub_assign(&mut self, rhs: Self) {
        //auto gaurd
        self.hp -= rhs.hp;
        if self.sp - rhs.sp < 0.0 {
            self.hp += self.sp - rhs.sp;
            self.sp = 0.0;
        } 
        else {
            self.sp -= rhs.sp;
        }
        for i in 0..=self.status.len() {
            self.status[i] -= rhs.status[i]
        }
    }
}

