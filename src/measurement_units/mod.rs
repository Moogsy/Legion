/// Structs to make sure that values are scaled according to the proper unit

/// Reresents the speed at which an entity can move in meters / seconds
#[derive(Default)]
pub struct TranslationSpeed(f32);

impl TranslationSpeed {
    pub fn from_coords_per_seconds(cps: f32) -> Self {
        Self(cps)
    }

    pub fn speed(&self) -> f32 {
        self.0
    }
}

/// Represents a rotation speed value in radians per seconds
#[derive(Default)]
pub struct RotationSpeed(f32);

impl RotationSpeed {
    pub fn from_degrees_per_seconds(dps: f32) -> Self {
        Self(dps.to_radians())
    }

    pub fn speed(&self) -> f32 {
        self.0
    }
}