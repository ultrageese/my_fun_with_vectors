use std::{fmt::Display, ops::{Add, Mul, Sub}};
pub struct Vector{
    pub x:f64,
    pub y:f64
}

impl Add for Vector{
    type Output = Self;
    fn add(self, other: Self) -> Self{
        Self{
            x: self.x+ other.x,
             y: self.y + other.y
        }
    }
}
impl Sub for Vector{
    type Output = Self;
    fn sub(self, other: Self) -> Self{
        Self { 
            x: self.x-other.x, 
            y: self.y-other.y 
        }
    }
}
impl Mul<i32> for Vector{
    type Output = Self;
    fn mul(self, rhs: i32) -> Self {
        Self { x: self.x*(rhs as f64), y: self.y*(rhs as f64) }
    }
}


impl Display for Vector{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}