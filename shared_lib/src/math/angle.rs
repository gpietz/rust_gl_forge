use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

#[derive(Default, Debug, Clone, Copy)]
pub struct Angle {
    radians: f32,
}

impl Angle {
    pub const ZERO: Self = Self {
        radians: 0.0,
    };

    /// Constructor for an angle in radians.
    pub const fn from_radians(radians: f32) -> Self {
        Self {
            radians,
        }
    }

    /// Constructor for an angle in degrees.
    pub fn from_degrees(degrees: f32) -> Self {
        Self {
            radians: degrees.to_radians(),
        }
    }

    /// Return the angle in degrees.
    pub fn as_degrees(&self) -> f32 {
        self.radians.to_degrees()
    }

    /// Return the angle in radians.
    pub const fn as_radians(&self) -> f32 {
        self.radians
    }

    /// Wrap to range [-180°, 180°)
    pub fn wrap_signed(self) -> Self {
        let pi = std::f32::consts::PI;
        let two_pi = 2.0 * pi;

        // Add pi to the angle to shift the range from [-pi, pi) to [0, 2pi)
        let shifted = self.radians + pi;

        // Perform Euclidean remainder operation to wrap the angle within [0, 2pi)
        let wrapped = shifted.rem_euclid(two_pi);

        // Subtract pi to shift the range back to [-pi, pi)
        let result = wrapped - pi;

        Self::from_radians(result)
    }

    /// Wrap to range [0°, 360°)
    pub fn wrap_unsigned(self) -> Self {
        let two_pi = 2.0 * std::f32::consts::PI;

        // Perform Euclidean remainder operation to wrap the angle within [0, 2pi)
        let wrapped = self.radians.rem_euclid(two_pi);

        Self::from_radians(wrapped)
    }
}

/// Constructor for an angle in degrees
pub fn degrees(angle: f32) -> Angle {
    Angle::from_degrees(angle)
}

/// Constructor for an angle in radians
pub const fn radians(angle: f32) -> Angle {
    Angle::from_radians(angle)
}

impl Add for Angle {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::from_radians(self.radians + other.radians)
    }
}

impl AddAssign for Angle {
    fn add_assign(&mut self, other: Self) {
        self.radians += other.radians;
    }
}

impl Sub for Angle {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::from_radians(self.radians - other.radians)
    }
}

impl SubAssign for Angle {
    fn sub_assign(&mut self, other: Self) {
        self.radians -= other.radians;
    }
}

impl Mul<f32> for Angle {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::from_radians(self.radians * rhs)
    }
}

impl MulAssign<f32> for Angle {
    fn mul_assign(&mut self, rhs: f32) {
        self.radians *= rhs;
    }
}

impl Div<f32> for Angle {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::from_radians(self.radians / rhs)
    }
}

impl DivAssign<f32> for Angle {
    fn div_assign(&mut self, rhs: f32) {
        self.radians /= rhs;
    }
}

impl Rem for Angle {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self::from_radians(self.radians % rhs.radians)
    }
}

impl RemAssign for Angle {
    fn rem_assign(&mut self, rhs: Self) {
        self.radians %= rhs.radians;
    }
}

impl Neg for Angle {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::from_radians(-self.radians)
    }
}

// Custom literals for angles
pub mod literals {
    use super::Angle;

    pub fn deg(angle: f32) -> Angle {
        Angle::from_degrees(angle)
    }

    pub const fn rad(angle: f32) -> Angle {
        Angle::from_radians(angle)
    }
}
