use std::{ops, f32};
use serde::{Serializer,Serialize, ser::SerializeStruct};

#[derive(Default, Clone,PartialEq,Copy)]
pub struct Stats {
    pub hp: f32,
    pub sp: f32,
    pub status:[i32;1]
    //FIXME add speed/enum
}
impl Serialize for Stats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Stats", 2)?;
        state.serialize_field("hp",&self.hp.to_string())?;
        state.serialize_field("sp",&self.sp.to_string())?;
        state.end()
    }
}
impl Stats {
    // fn regulate(&mut self, nrm:Stats) {
    //     //this function approches the values 
    //     //adding to stamina, decrementing status
    //     //hp is ignored (allows for overflow of hp)
    //     //i.e. hp is unregulated
    //     todo!()
    // }
}

impl ops::Add<Stats> for Stats {
    type Output = Stats;
    fn add(self, rhs: Stats) -> Self::Output {
        Stats {
            hp:self.hp + rhs.hp,
            sp:self.sp + rhs.sp,
            status: {
                        let mut result = [0; 1]; 
                        for (result_status, (&self_status, &rhs_status)) in result
                                 .iter_mut().zip(self.status.iter().zip(rhs.status.iter())) {
                             *result_status = self_status + rhs_status;
                        }
                        result
                }
        }
    }
}
impl ops::Sub<Stats> for Stats {
    type Output = Stats;
    fn sub(self, rhs: Stats) -> Self::Output {
        Stats {
            hp:self.hp - rhs.hp,
            sp:self.sp - rhs.sp,
            status: {
                //TODO move to a function
                        let mut result = [0; 1]; 
                        for (result_status, (&self_status, &rhs_status)) in result
                                 .iter_mut().zip(self.status.iter().zip(rhs.status.iter())) {
                             *result_status = self_status + rhs_status;
                        }
                        result
                }
        }
    }
}

impl ops::AddAssign for Stats {
    fn add_assign(&mut self, rhs: Self) {
        self.hp += rhs.hp;
        self.sp += rhs.sp;
        for i in 0..self.status.len() {
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

