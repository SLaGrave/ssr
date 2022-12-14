use std::ops::{Index, IndexMut, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
use std::fmt;
use std::fmt::Display;

//////////////////////////////////////////////////////////////////////////////
// Custom Vec3 Implementation
//////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3]
}

// Aliases
pub type Point3 = Vec3;
pub type Color = Vec3;

// Constructor
impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 {
            e: [e0, e1, e2]
        }  // No semi-colon, so this is returned
    }
}

//////////////////////////////////////////////////////////////////////////////
// Impl other traits
//////////////////////////////////////////////////////////////////////////////
impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        &mut self.e[index]
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self[0] + other[0], self[1] + other[1], self[2] + other[2]]
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) -> () {
        *self = Vec3 {e: [self[0] + other[0], self[1] + other[1], self[2] + other[2]]};
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self[0] - other[0], self[1] - other[1], self[2] - other[2]]
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) -> () {
        *self = Vec3 {e: [self[0] - other[0], self[1] - other[1], self[2] - other[2]]};
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3 {
            e: [self[0]*other, self[1]*other, self[2]*other]
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) -> () {
        *self = Vec3{e: [self[0]*other, self[1]*other, self[2]*other]};
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        Vec3 {
            e: [self[0]/other, self[1]/other, self[2]/other]
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) -> () {
        *self = Vec3{e: [self[0]/other, self[1]/other, self[2]/other]};
    }
}

//////////////////////////////////////////////////////////////////////////////
// Utility Functions
//////////////////////////////////////////////////////////////////////////////
impl Vec3{
    pub fn x(self) -> f64 {
        self[0]
    }
    
    pub fn y(self) -> f64 {
        self[1]
    }
    
    pub fn z(self) -> f64 {
        self[2]
    }
    
    pub fn dot(self, other: Vec3) -> f64 {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
    }
    
    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
    }
    
    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3{
            e: [
                self[1] * other[2] - self[2] * other[1],
                self[2] * other[0] - self[0] * other[2],
                self[0] * other[1] - self[1] * other[0]
            ]
        }
    }
    
    pub fn normalized(self) -> Vec3 {
        self / self.length()
    }
}

//////////////////////////////////////////////////////////////////////////////
// Display Trait
//////////////////////////////////////////////////////////////////////////////
// Look man, I'm gonna be realy with you... I don't
// really know how this function works. I can explain
// the other ones, but this one is magic to me - I
// basically just copied the book and trusted it.
impl Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self[0], self[1], self[2])
    }
}

//////////////////////////////////////////////////////////////////////////////
// Color Utility
//////////////////////////////////////////////////////////////////////////////
impl Color {
    pub fn format_color(self) -> String {
        format!("{} {} {}", (255.999 * self[0]) as u64,
                            (255.999 * self[1]) as u64,
                            (255.999 * self[2]) as u64)
    }
}