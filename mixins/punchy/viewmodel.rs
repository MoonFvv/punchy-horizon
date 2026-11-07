//! Player viewmodel offsets, independent of animation clips and item tuning.
use glam::{Mat4, Vec3};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Target {
    #[default]
    Arms,
    Items,
}
impl Target {
    pub fn next(self) -> Self {
        match self {
            Self::Arms => Self::Items,
            Self::Items => Self::Arms,
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Self::Arms => "Arms",
            Self::Items => "Items",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Parameter {
    #[default]
    X,
    Y,
    Z,
    Scale,
}
impl Parameter {
    pub const ALL: [Self; 4] = [Self::X, Self::Y, Self::Z, Self::Scale];
    pub fn next(self) -> Self {
        match self {
            Self::X => Self::Y,
            Self::Y => Self::Z,
            Self::Z => Self::Scale,
            Self::Scale => Self::X,
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Self::X => "X (right)",
            Self::Y => "Y (up)",
            Self::Z => "Z (closer)",
            Self::Scale => "Scale",
        }
    }
    pub fn key(self, target: Target) -> &'static str {
        match (target, self) {
            (Target::Arms, Self::X) => "arms_x",
            (Target::Arms, Self::Y) => "arms_y",
            (Target::Arms, Self::Z) => "arms_z",
            (Target::Arms, Self::Scale) => "arms_scale",
            (Target::Items, Self::X) => "items_x",
            (Target::Items, Self::Y) => "items_y",
            (Target::Items, Self::Z) => "items_z",
            (Target::Items, Self::Scale) => "items_scale",
        }
    }
    pub fn default_value(self) -> f32 {
        if self == Self::Scale { 1.0 } else { 0.0 }
    }
    pub fn parse(self, text: Option<&str>) -> f32 {
        let value = text
            .and_then(|s| s.parse::<f32>().ok())
            .filter(|v| v.is_finite())
            .unwrap_or_else(|| self.default_value());
        self.clamp(value)
    }
    fn clamp(self, value: f32) -> f32 {
        if self == Self::Scale {
            value.clamp(0.25, 2.0)
        } else {
            value.clamp(-16.0, 16.0)
        }
    }
    pub fn step(self, value: f32, increase: bool) -> f32 {
        let step = if self == Self::Scale { 0.05 } else { 0.5 };
        // Round to avoid accumulating floating-point drift in persisted values.
        (self.clamp(value + if increase { step } else { -step }) * 100.0).round() / 100.0
    }
    pub fn display(self, value: f32) -> String {
        if self == Self::Scale {
            format!("{:.0}%", value * 100.0)
        } else {
            format!("{value:+.1} px")
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Adjustment {
    pub offset: Vec3,
    pub scale: f32,
}
impl Default for Adjustment {
    fn default() -> Self {
        Self {
            offset: Vec3::ZERO,
            scale: 1.0,
        }
    }
}
impl Adjustment {
    pub fn value(self, parameter: Parameter) -> f32 {
        match parameter {
            Parameter::X => self.offset.x,
            Parameter::Y => self.offset.y,
            Parameter::Z => self.offset.z,
            Parameter::Scale => self.scale,
        }
    }
    /// Offsets use view-space model pixels; scale stays around the object's own
    /// origin so shrinking an item does not pull its position toward the camera.
    pub fn apply(self, pose: Mat4) -> Mat4 {
        Mat4::from_translation(self.offset / 16.0)
            * pose
            * Mat4::from_scale(Vec3::splat(self.scale))
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Viewmodel {
    pub arms: Adjustment,
    pub items: Adjustment,
}
impl Viewmodel {
    pub fn read(mut setting: impl FnMut(&str) -> Option<String>) -> Self {
        let mut read = |target| {
            let values = Parameter::ALL.map(|p| p.parse(setting(p.key(target)).as_deref()));
            Adjustment {
                offset: Vec3::new(values[0], values[1], values[2]),
                scale: values[3],
            }
        };
        Self {
            arms: read(Target::Arms),
            items: read(Target::Items),
        }
    }
    pub fn get(self, target: Target) -> Adjustment {
        match target {
            Target::Arms => self.arms,
            Target::Items => self.items,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn old_settings_keep_existing_poses_and_invalid_values_are_safe() {
        let settings = Viewmodel::read(|_| None);
        let pose = Mat4::from_rotation_y(0.7) * Mat4::from_translation(Vec3::new(0.3, -0.4, -1.0));
        assert_eq!(settings.arms.apply(pose), pose);
        assert_eq!(settings.items.apply(pose), pose);
        for input in ["NaN", "inf", "-inf", "garbage", ""] {
            assert_eq!(Parameter::X.parse(Some(input)), 0.0);
            assert_eq!(Parameter::Scale.parse(Some(input)), 1.0);
        }
        assert_eq!(Parameter::X.parse(Some("100")), 16.0);
        assert_eq!(Parameter::Y.parse(Some("-100")), -16.0);
        assert_eq!(Parameter::Scale.parse(Some("-2")), 0.25);
        assert_eq!(Parameter::Scale.parse(Some("100")), 2.0);
    }
    #[test]
    fn offsets_and_scale_have_independent_observable_effects() {
        let settings = Viewmodel::read(|key| match key {
            "arms_x" => Some("8".into()),
            "arms_y" => Some("-4".into()),
            "arms_z" => Some("2".into()),
            "arms_scale" => Some("0.5".into()),
            "items_scale" => Some("1.5".into()),
            _ => None,
        });
        let pose = Mat4::from_translation(Vec3::new(1.0, 2.0, -3.0))
            * Mat4::from_rotation_z(std::f32::consts::FRAC_PI_2);
        let arm = settings.arms.apply(pose);
        assert!((arm.transform_point3(Vec3::ZERO) - Vec3::new(1.5, 1.75, -2.875)).length() < 1e-6);
        assert!((arm.transform_vector3(Vec3::X) - Vec3::new(0.0, 0.5, 0.0)).length() < 1e-6);
        let item = settings.items.apply(pose);
        assert!((item.transform_point3(Vec3::ZERO) - Vec3::new(1.0, 2.0, -3.0)).length() < 1e-6);
        assert!((item.transform_vector3(Vec3::X) - Vec3::new(0.0, 1.5, 0.0)).length() < 1e-6);
    }
    #[test]
    fn controls_reach_bounds_without_wrapping_or_drift() {
        let mut scale = 1.0;
        for _ in 0..100 {
            scale = Parameter::Scale.step(scale, true);
        }
        assert_eq!(scale, 2.0);
        for _ in 0..100 {
            scale = Parameter::Scale.step(scale, false);
        }
        assert_eq!(scale, 0.25);
        let mut x = 0.0;
        for _ in 0..32 {
            x = Parameter::X.step(x, false);
        }
        assert_eq!(x, -16.0);
        assert_eq!(Parameter::X.step(x, false), -16.0);
    }
}
