use std::ops;


pub type Color = Vec3<f64>;
pub type Point = Vec3<f64>;

#[derive(Debug, Clone, PartialEq)]
pub struct Ray {
    pub origin: Point,
    pub direction: Point
}

impl Ray {
    pub fn new(origin: Point, direction: Point) -> Self {
        Ray{origin, direction}
    }
    pub fn at(&self, t:f64) -> Point {
        self.origin + self.direction * t
    }
    pub fn normalize(&mut self) {
        self.direction /= self.direction.length();
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl Vec3<f64> {
    pub fn new() -> Self {
        Vec3{x:0.0,y:0.0,z:0.0}
    }
    pub fn length_squared(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn dot(v1: &Self, v2: &Self) -> f64 {
        v1.x*v2.x + v1.y*v2.y + v1.z*v2.z
    }
    pub fn project(v1: &Self, v2: &Self) -> Self {
        v2.clone() * (Vec3::dot(v1, v2) / v2.length_squared()) 
    }
}

impl<T: ops::MulAssign + Copy> ops::MulAssign<T> for Vec3<T> {
    fn mul_assign(&mut self, t:T) {
        self.x *= t;
        self.y *= t;
        self.z *= t;
    }
}

impl<T: ops::Mul<Output = T> + Copy> ops::Mul<T> for Vec3<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Vec3{
            x:self.x * rhs,
            y:self.y * rhs,
            z:self.z * rhs
        }
    }
}


impl<T: ops::DivAssign + Copy> ops::DivAssign<T> for Vec3<T>  {
    fn div_assign(&mut self, rhs:T) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl<T: ops::Div<Output = T> + Copy> ops::Div<T> for Vec3<T>  {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Vec3{
            x:self.x / rhs,
            y:self.y / rhs,
            z:self.z / rhs
        }
    }
}


impl<T: ops::AddAssign> ops::AddAssign for Vec3<T>  {
    fn add_assign(&mut self, rhs: Vec3<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: ops::Add<Output = T>> ops::Add for Vec3<T> {
    type Output = Self;
    fn add(self, rhs: Vec3<T>) -> Self::Output {
        Vec3{
            x:self.x + rhs.x, 
            y:self.y + rhs.y,
            z:self.z + rhs.z
        }
    }
}

impl<T: ops::SubAssign> ops::SubAssign for Vec3<T> {
    fn sub_assign(&mut self, rhs: Vec3<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T: ops::Sub<Output = T>> ops::Sub for Vec3<T> {
    type Output = Self;
    fn sub(self, rhs: Vec3<T>) -> Self::Output {
        Vec3{
            x:self.x - rhs.x, 
            y:self.y - rhs.y,
            z:self.z - rhs.z
        }
    }
}