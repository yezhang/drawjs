use glam::{DMat2, DVec2};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    matrix: DMat2,
    translation: DVec2,
}

impl Transform {
    pub fn identity() -> Self {
        Self {
            matrix: DMat2::IDENTITY,
            translation: DVec2::ZERO,
        }
    }

    pub fn from_translation_2d(x: f64, y: f64) -> Self {
        Self {
            matrix: DMat2::IDENTITY,
            translation: DVec2::new(x, y),
        }
    }

    pub fn from_scale_2d(x: f64, y: f64) -> Self {
        Self {
            matrix: DMat2::from_cols(
                DVec2::new(x, 0.0),
                DVec2::new(0.0, y),
            ),
            translation: DVec2::ZERO,
        }
    }

    pub fn from_rotation(rad: f64) -> Self {
        let (s, c) = rad.sin_cos();
        Self {
            matrix: DMat2::from_cols(
                DVec2::new(c, s),
                DVec2::new(-s, c),
            ),
            translation: DVec2::ZERO,
        }
    }

    pub fn multiply(&self, other: &Transform) -> Self {
        Self {
            matrix: self.matrix * other.matrix,
            translation: self.matrix * other.translation + self.translation,
        }
    }

    pub fn compose(&self, other: &Transform) -> Self {
        self.multiply(other)
    }

    pub fn multiply_point_2d(&self, point: DVec2) -> DVec2 {
        self.matrix * point + self.translation
    }

    pub fn multiply_vector_2d(&self, vector: DVec2) -> DVec2 {
        self.matrix * vector
    }

    pub fn inverse(&self) -> Self {
        let inv_matrix = self.matrix.inverse();
        Self {
            matrix: inv_matrix,
            translation: -(inv_matrix * self.translation),
        }
    }

    pub fn translation(&self) -> DVec2 {
        self.translation
    }

    pub fn scale(&self) -> DVec2 {
        DVec2::new(self.matrix.x_axis.x, self.matrix.y_axis.y)
    }
}

impl std::ops::Mul for Transform {
    type Output = Transform;

    fn mul(self, other: Transform) -> Transform {
        self.multiply(&other)
    }
}

impl std::ops::MulAssign for Transform {
    fn mul_assign(&mut self, other: Transform) {
        *self = self.multiply(&other);
    }
}
