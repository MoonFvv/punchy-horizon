//! Punchy! Moves — third-person dynamic shoulder camera, orbit/free camera,
//! stationary turn easing, and third-person locomotion/pose animation.
use glam::{Mat4, Quat, Vec3};

/// Configurable camera parameters for the shoulder camera.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CameraParameter {
    #[default]
    Horizontal,
    Vertical,
    Depth,
    Fov,
}

impl CameraParameter {
    pub const ALL: [Self; 4] = [Self::Horizontal, Self::Vertical, Self::Depth, Self::Fov];

    pub fn next(self) -> Self {
        match self {
            Self::Horizontal => Self::Vertical,
            Self::Vertical => Self::Depth,
            Self::Depth => Self::Fov,
            Self::Fov => Self::Horizontal,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Horizontal => "Horizontal (shoulder X)",
            Self::Vertical => "Vertical (height Y)",
            Self::Depth => "Distance (depth Z)",
            Self::Fov => "Field of view (FOV)",
        }
    }

    pub fn key(self) -> &'static str {
        match self {
            Self::Horizontal => "moves_camera_horizontal",
            Self::Vertical => "moves_camera_vertical",
            Self::Depth => "moves_camera_depth",
            Self::Fov => "moves_camera_fov",
        }
    }

    pub fn default_value(self) -> f32 {
        match self {
            Self::Horizontal => 1.17,
            Self::Vertical => -0.11,
            Self::Depth => 2.5,
            Self::Fov => 70.0,
        }
    }

    pub fn clamp(self, val: f32) -> f32 {
        match self {
            Self::Horizontal => val.clamp(-2.0, 2.0),
            Self::Vertical => val.clamp(-1.5, 2.0),
            Self::Depth => val.clamp(1.0, 8.0),
            Self::Fov => val.clamp(30.0, 110.0),
        }
    }

    pub fn step(self, value: f32, increase: bool) -> f32 {
        let step = match self {
            Self::Horizontal | Self::Vertical => 0.1,
            Self::Depth => 0.25,
            Self::Fov => 5.0,
        };
        let delta = if increase { step } else { -step };
        (self.clamp(value + delta) * 100.0).round() / 100.0
    }

    pub fn display(self, value: f32) -> String {
        match self {
            Self::Horizontal | Self::Vertical => format!("{value:+.2} blk"),
            Self::Depth => format!("{value:.2} blk"),
            Self::Fov => format!("{value:.0}°"),
        }
    }

    pub fn parse(self, text: Option<&str>) -> f32 {
        let value = text
            .and_then(|s| s.parse::<f32>().ok())
            .filter(|v| v.is_finite())
            .unwrap_or_else(|| self.default_value());
        self.clamp(value)
    }
}

/// Shoulder camera settings and runtime state.
#[derive(Clone, Copy, Debug)]
pub struct ShoulderCamera {
    pub enabled: bool,
    pub horizontal: f32,
    pub vertical: f32,
    pub depth: f32,
    pub fov: f32,
    pub rotation_yaw: f32,
    pub rotation_pitch: f32,
    pub free_camera: bool,
    pub free_camera_yaw: f32,
    pub free_camera_pitch: f32,
    pub smoothed_horizontal: f32,
    pub smoothed_vertical: f32,
    pub smoothed_depth: f32,
    pub initialized: bool,
}

impl Default for ShoulderCamera {
    fn default() -> Self {
        Self {
            enabled: true,
            horizontal: CameraParameter::Horizontal.default_value(),
            vertical: CameraParameter::Vertical.default_value(),
            depth: CameraParameter::Depth.default_value(),
            fov: CameraParameter::Fov.default_value(),
            rotation_yaw: 0.0,
            rotation_pitch: 0.0,
            free_camera: false,
            free_camera_yaw: 0.0,
            free_camera_pitch: 0.0,
            smoothed_horizontal: CameraParameter::Horizontal.default_value(),
            smoothed_vertical: CameraParameter::Vertical.default_value(),
            smoothed_depth: CameraParameter::Depth.default_value(),
            initialized: false,
        }
    }
}

impl ShoulderCamera {
    /// Read settings from the game's persistent mod settings.
    pub fn read<F: Fn(&str) -> Option<String>>(get_setting: F) -> Self {
        let mut cam = Self::default();
        if let Some(s) = get_setting("moves_camera_enabled") {
            cam.enabled = s != "false";
        }
        cam.horizontal = CameraParameter::Horizontal
            .parse(get_setting(CameraParameter::Horizontal.key()).as_deref());
        cam.vertical = CameraParameter::Vertical
            .parse(get_setting(CameraParameter::Vertical.key()).as_deref());
        cam.depth =
            CameraParameter::Depth.parse(get_setting(CameraParameter::Depth.key()).as_deref());
        cam.fov = CameraParameter::Fov.parse(get_setting(CameraParameter::Fov.key()).as_deref());
        if let Some(v) = get_setting("moves_camera_rot_yaw")
            .and_then(|s| s.parse::<f32>().ok())
            .filter(|v| v.is_finite())
        {
            cam.rotation_yaw = v.clamp(-30.0, 30.0);
        }
        if let Some(v) = get_setting("moves_camera_rot_pitch")
            .and_then(|s| s.parse::<f32>().ok())
            .filter(|v| v.is_finite())
        {
            cam.rotation_pitch = v.clamp(-30.0, 30.0);
        }
        cam.smoothed_horizontal = cam.horizontal;
        cam.smoothed_vertical = cam.vertical;
        cam.smoothed_depth = cam.depth;
        cam
    }

    /// Whether the shoulder camera is currently enabled.
    pub fn is_active(&self) -> bool {
        self.enabled
    }

    /// Reset all camera settings to factory defaults.
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Update smoothing transitions.
    pub fn update(&mut self, elapsed_seconds: f32) {
        self.update_smoothing(elapsed_seconds);
    }

    /// Update smoothing transitions.
    pub fn update_smoothing(&mut self, elapsed_seconds: f32) {
        if !self.initialized {
            self.smoothed_horizontal = self.horizontal;
            self.smoothed_vertical = self.vertical;
            self.smoothed_depth = self.depth;
            self.initialized = true;
            return;
        }
        let alpha = 1.0 - (-12.0 * elapsed_seconds.clamp(0.0, 0.1)).exp();
        self.smoothed_horizontal += (self.horizontal - self.smoothed_horizontal) * alpha;
        self.smoothed_vertical += (self.vertical - self.smoothed_vertical) * alpha;
        self.smoothed_depth += (self.depth - self.smoothed_depth) * alpha;
    }

    /// Toggle free/orbit camera mode.
    pub fn toggle_free_camera(&mut self, current_yaw: f32, current_pitch: f32) -> bool {
        self.free_camera = !self.free_camera;
        if self.free_camera {
            self.free_camera_yaw = current_yaw;
            self.free_camera_pitch = current_pitch;
        }
        self.free_camera
    }

    /// Turn free camera when orbit mode is active.
    pub fn turn_free_camera(&mut self, yaw_delta: f32, pitch_delta: f32) -> bool {
        if !self.free_camera {
            return false;
        }
        self.free_camera_yaw += yaw_delta * 0.15;
        self.free_camera_pitch = (self.free_camera_pitch + pitch_delta * 0.15).clamp(-90.0, 90.0);
        true
    }

    /// Compute view yaw and pitch.
    pub fn solve_rotation(&self, player_yaw: f32, player_pitch: f32) -> (f32, f32) {
        let base_yaw = if self.free_camera {
            self.free_camera_yaw
        } else {
            player_yaw
        };
        let base_pitch = if self.free_camera {
            self.free_camera_pitch
        } else {
            player_pitch
        };
        let final_yaw = base_yaw + self.rotation_yaw;
        let final_pitch = (base_pitch + self.rotation_pitch).clamp(-90.0, 90.0);
        (final_yaw, final_pitch)
    }

    /// Compute forward, right, and up basis vectors from yaw and pitch (in degrees).
    /// Follows Minecraft conventions: Yaw 0 is +Z (South), Yaw 90 is -X (West).
    pub fn basis_vectors(yaw: f32, pitch: f32) -> (Vec3, Vec3, Vec3) {
        let yaw_rad = yaw.to_radians();
        let pitch_rad = pitch.to_radians();
        let (sin_y, cos_y) = yaw_rad.sin_cos();
        let (sin_p, cos_p) = pitch_rad.sin_cos();

        // Forward direction:
        let forward = Vec3::new(-sin_y * cos_p, -sin_p, cos_y * cos_p).normalize();
        // Right direction (perpendicular in horizontal plane):
        let right = Vec3::new(cos_y, 0.0, sin_y).normalize();
        // Up direction (forward x right points up in Minecraft coordinates):
        let up = forward.cross(right).normalize();
        (forward, right, up)
    }

    /// Calculate the ideal desired position without clipping.
    pub fn desired_position(&self, pivot: Vec3, forward: Vec3, right: Vec3, up: Vec3) -> Vec3 {
        pivot - forward * self.smoothed_depth
            + right * self.smoothed_horizontal
            + up * self.smoothed_vertical
    }

    /// Solve camera position, incorporating obstacle clipping.
    /// `clip_test` takes `(from, to)` and returns `Some(hit_pos)` if an obstacle blocks the ray.
    pub fn solve_camera_position<F: FnMut(Vec3, Vec3) -> Option<Vec3>>(
        &self,
        pivot: Vec3,
        forward: Vec3,
        right: Vec3,
        up: Vec3,
        mut clip_test: F,
    ) -> Vec3 {
        let desired = self.desired_position(pivot, forward, right, up);
        if let Some(hit) = clip_test(pivot, desired) {
            // Pull in from obstacle with small safety buffer
            let dir = (desired - pivot).normalize();
            let hit_dist = hit.distance(pivot);
            let safe_dist = (hit_dist - 0.1).max(0.2);
            pivot + dir * safe_dist
        } else {
            desired
        }
    }
}

/// Unproject center-of-screen ray for crosshair targeting in third person.
pub fn unproject_center_ray(view: Mat4, projection: Mat4) -> Option<(Vec3, Vec3)> {
    let inv = (projection * view).inverse();
    let p_near = inv.project_point3(Vec3::new(0.0, 0.0, -1.0));
    let p_far = inv.project_point3(Vec3::new(0.0, 0.0, 1.0));
    if p_near.is_finite() && p_far.is_finite() {
        let dir = (p_far - p_near).normalize();
        if dir.is_finite() {
            return Some((p_near, dir));
        }
    }
    None
}

/// Locomotion action state for third-person player animations.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LocomotionState {
    #[default]
    Idle,
    Walking,
    Sprinting,
    Crouching,
    Jumping,
    Falling,
    Swimming,
    Crawling,
    Climbing,
    ElytraFlying,
    Riding,
}

impl LocomotionState {
    pub fn clip_name(self) -> &'static str {
        match self {
            Self::Idle => "disable_punchy_moves_idle",
            Self::Walking => "disable_punchy_moves_walk",
            Self::Sprinting => "punchy_moves_sprint",
            Self::Crouching => "disable_punchy_moves_walk",
            Self::Jumping => "punchy_moves_jump",
            Self::Falling => "disable_punchy_moves_fall",
            Self::Swimming => "swiming",
            Self::Crawling => "crawl",
            Self::Climbing => "disable_punchy_moves_walk",
            Self::ElytraFlying => "disable_punchy_moves_fall",
            Self::Riding => "newhorse_riding_idle",
        }
    }
}

/// Humanoid bone transform in standard model coordinates:
/// rotations in degrees [pitch, yaw, roll], translation in model blocks.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BoneTransform {
    pub rotation: Vec3,
    pub translation: Vec3,
    pub scale: Vec3,
}

impl BoneTransform {
    pub fn new(rotation: Vec3, translation: Vec3) -> Self {
        Self {
            rotation,
            translation,
            scale: Vec3::ONE,
        }
    }

    pub fn to_matrix(self) -> Mat4 {
        Mat4::from_translation(self.translation)
            * Mat4::from_quat(Quat::from_euler(
                glam::EulerRot::ZYX,
                self.rotation.z.to_radians(),
                self.rotation.y.to_radians(),
                self.rotation.x.to_radians(),
            ))
            * Mat4::from_scale(self.scale)
    }

    pub fn lerp(self, other: Self, alpha: f32) -> Self {
        Self {
            rotation: self.rotation.lerp(other.rotation, alpha),
            translation: self.translation.lerp(other.translation, alpha),
            scale: self.scale.lerp(other.scale, alpha),
        }
    }
}

/// Full humanoid pose covering the standard player parts.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HumanoidPose {
    pub head: BoneTransform,
    pub body: BoneTransform,
    pub right_arm: BoneTransform,
    pub left_arm: BoneTransform,
    pub right_leg: BoneTransform,
    pub left_leg: BoneTransform,
}

impl HumanoidPose {
    pub fn lerp(&self, other: &Self, alpha: f32) -> Self {
        Self {
            head: self.head.lerp(other.head, alpha),
            body: self.body.lerp(other.body, alpha),
            right_arm: self.right_arm.lerp(other.right_arm, alpha),
            left_arm: self.left_arm.lerp(other.left_arm, alpha),
            right_leg: self.right_leg.lerp(other.right_leg, alpha),
            left_leg: self.left_leg.lerp(other.left_leg, alpha),
        }
    }
}

/// Stationary turning controller for third-person player models.
/// Keeps the body turning smoothly toward the head/camera angle without snapping.
#[derive(Clone, Copy, Debug, Default)]
pub struct TurnInPlace {
    pub body_yaw: f32,
    pub previous_body_yaw: f32,
    pub max_head_diff: f32,
    pub ease_rate: f32,
}

impl TurnInPlace {
    pub fn new() -> Self {
        Self {
            body_yaw: 0.0,
            previous_body_yaw: 0.0,
            max_head_diff: 50.0,
            ease_rate: 0.3,
        }
    }

    /// Wrap angle difference into [-180, 180] degrees.
    pub fn wrap_degrees(mut angle: f32) -> f32 {
        angle %= 360.0;
        if angle >= 180.0 {
            angle -= 360.0;
        }
        if angle < -180.0 {
            angle += 360.0;
        }
        angle
    }

    /// Update stationary turn in place each tick.
    pub fn update(&mut self, target_yaw: f32, moving: bool, swinging: bool) {
        self.previous_body_yaw = self.body_yaw;
        if moving || swinging {
            // When moving or swinging, body aligns quickly with direction/target
            self.body_yaw += Self::wrap_degrees(target_yaw - self.body_yaw) * 0.6;
        } else {
            // Stationary: ease toward target within max difference
            self.body_yaw += Self::wrap_degrees(target_yaw - self.body_yaw) * self.ease_rate;
            let diff = Self::wrap_degrees(target_yaw - self.body_yaw);
            if diff.abs() > self.max_head_diff {
                self.body_yaw += diff - diff.signum() * self.max_head_diff;
            }
        }
    }

    /// Sample interpolated body yaw for rendering.
    pub fn sample_yaw(&self, alpha: f32) -> f32 {
        self.previous_body_yaw + Self::wrap_degrees(self.body_yaw - self.previous_body_yaw) * alpha
    }
}

/// High-level third-person locomotion controller.
#[derive(Clone, Debug, Default)]
pub struct MovesLocomotion {
    pub enabled: bool,
    pub turn: TurnInPlace,
    pub state: LocomotionState,
    pub anim_time: f32,
    pub previous_pose: HumanoidPose,
    pub current_pose: HumanoidPose,
    pub blend_weight: f32,
}

impl MovesLocomotion {
    pub fn new() -> Self {
        Self {
            enabled: true,
            turn: TurnInPlace::new(),
            state: LocomotionState::Idle,
            anim_time: 0.0,
            previous_pose: HumanoidPose::default(),
            current_pose: HumanoidPose::default(),
            blend_weight: 1.0,
        }
    }

    pub fn tick(
        &mut self,
        target_yaw: f32,
        state: LocomotionState,
        delta_time: f32,
        moving: bool,
        swinging: bool,
    ) {
        if state != self.state {
            self.previous_pose = self.current_pose.clone();
            self.state = state;
            self.anim_time = 0.0;
            self.blend_weight = 0.0;
        }
        self.turn.update(target_yaw, moving, swinging);
        self.anim_time += delta_time;
        self.blend_weight = (self.blend_weight + delta_time * 5.0).min(1.0);
    }

    /// Calculate procedural arm and leg swings based on limb walk distance/speed.
    pub fn procedural_walk_pose(limb_pos: f32, limb_speed: f32) -> HumanoidPose {
        let leg_swing = (limb_pos * 0.6662).cos() * 1.4 * limb_speed;
        let arm_swing = (limb_pos * 0.6662).sin() * 1.0 * limb_speed;

        let mut pose = HumanoidPose::default();
        pose.right_leg.rotation.x = leg_swing.to_degrees();
        pose.left_leg.rotation.x = -leg_swing.to_degrees();
        pose.right_arm.rotation.x = -arm_swing.to_degrees();
        pose.left_arm.rotation.x = arm_swing.to_degrees();
        pose
    }

    /// Sample current blended pose.
    pub fn sample_pose(&self, limb_pos: f32, limb_speed: f32) -> HumanoidPose {
        let base = Self::procedural_walk_pose(limb_pos, limb_speed);
        if self.blend_weight >= 1.0 {
            base
        } else {
            self.previous_pose.lerp(&base, self.blend_weight)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camera_parameter_defaults_and_keys() {
        for p in CameraParameter::ALL {
            assert!(p.default_value().is_finite());
            assert!(!p.label().is_empty());
            assert!(!p.key().is_empty());
            assert_eq!(p.parse(None), p.default_value());
            assert_eq!(p.parse(Some("nan")), p.default_value());
            assert_eq!(p.parse(Some("invalid")), p.default_value());
        }
    }

    #[test]
    fn camera_parameter_stepping_and_clamping() {
        let h = CameraParameter::Horizontal;
        let v0 = h.default_value();
        let v1 = h.step(v0, true);
        assert!(v1 > v0);
        let v2 = h.step(v1, false);
        assert_eq!(v2, v0);

        // Clamping bounds
        assert_eq!(h.parse(Some("100.0")), 2.0);
        assert_eq!(h.parse(Some("-100.0")), -2.0);

        let depth = CameraParameter::Depth;
        assert_eq!(depth.parse(Some("50.0")), 8.0);
        assert_eq!(depth.parse(Some("0.1")), 1.0);

        let fov = CameraParameter::Fov;
        assert_eq!(fov.parse(Some("200.0")), 110.0);
        assert_eq!(fov.parse(Some("10.0")), 30.0);
    }

    #[test]
    fn shoulder_camera_positioning() {
        let cam = ShoulderCamera::default();
        let pivot = Vec3::new(10.0, 64.0, 20.0);
        let (fwd, right, up) = ShoulderCamera::basis_vectors(0.0, 0.0);

        // Facing south (+Z):
        assert!((fwd.z - 1.0).abs() < 1e-5);
        assert!((fwd.x).abs() < 1e-5);
        assert!((right.x - 1.0).abs() < 1e-5);
        assert!((up.y - 1.0).abs() < 1e-5);

        let desired = cam.desired_position(pivot, fwd, right, up);
        // Back (-depth on Z), right (+horizontal on X), up (+vertical on Y)
        assert_eq!(desired.x, pivot.x + cam.horizontal);
        assert_eq!(desired.y, pivot.y + cam.vertical);
        assert_eq!(desired.z, pivot.z - cam.depth);
    }

    #[test]
    fn shoulder_camera_obstacle_clipping() {
        let cam = ShoulderCamera::default();
        let pivot = Vec3::new(0.0, 64.0, 0.0);
        let (fwd, right, up) = ShoulderCamera::basis_vectors(0.0, 0.0);

        // Without obstacle, reaches desired position:
        let pos1 = cam.solve_camera_position(pivot, fwd, right, up, |_, _| None);
        assert_eq!(pos1, cam.desired_position(pivot, fwd, right, up));

        // With obstacle halfway:
        let hit_point = pivot + (pos1 - pivot) * 0.5;
        let pos2 = cam.solve_camera_position(pivot, fwd, right, up, |_, _| Some(hit_point));
        assert!(pos2.distance(pivot) < pos1.distance(pivot));
    }

    #[test]
    fn orbit_free_camera_controls() {
        let mut cam = ShoulderCamera::default();
        assert!(!cam.free_camera);

        // Toggle on: captures player orientation
        cam.toggle_free_camera(180.0, 15.0);
        assert!(cam.free_camera);
        assert_eq!(cam.free_camera_yaw, 180.0);
        assert_eq!(cam.free_camera_pitch, 15.0);

        // Turning in orbit mode:
        assert!(cam.turn_free_camera(10.0, 5.0));
        assert_eq!(cam.free_camera_yaw, 181.5);
        assert_eq!(cam.free_camera_pitch, 15.75);

        // Pitch is clamped to [-90, 90]
        cam.turn_free_camera(0.0, 1000.0);
        assert_eq!(cam.free_camera_pitch, 90.0);
        cam.turn_free_camera(0.0, -2000.0);
        assert_eq!(cam.free_camera_pitch, -90.0);

        // Toggle off:
        cam.toggle_free_camera(0.0, 0.0);
        assert!(!cam.free_camera);
        assert!(!cam.turn_free_camera(10.0, 5.0));
    }

    #[test]
    fn stationary_turn_in_place_easing() {
        let mut turn = TurnInPlace::new();
        turn.body_yaw = 0.0;

        // Small head turn eases smoothly:
        turn.update(30.0, false, false);
        let stationary_yaw = turn.body_yaw;
        assert!(stationary_yaw > 0.0 && stationary_yaw < 30.0);

        // Large head turn (e.g. 90 deg) is constrained by max_head_diff (50 deg):
        turn.body_yaw = 0.0;
        turn.update(90.0, false, false);
        assert!(turn.body_yaw >= 40.0);

        // While moving or swinging, turns faster to align with target:
        let mut turn_moving = TurnInPlace::new();
        turn_moving.body_yaw = 0.0;
        turn_moving.update(30.0, true, false);
        assert!(turn_moving.body_yaw > stationary_yaw);
    }

    #[test]
    fn humanoid_pose_interpolation() {
        let mut p1 = HumanoidPose::default();
        let mut p2 = HumanoidPose::default();
        p1.right_arm.rotation.x = -20.0;
        p2.right_arm.rotation.x = 40.0;

        let mid = p1.lerp(&p2, 0.5);
        assert_eq!(mid.right_arm.rotation.x, 10.0);

        let mat = mid.right_arm.to_matrix();
        assert!(mat.is_finite());
    }

    #[test]
    fn settings_read_and_persistence() {
        use std::collections::BTreeMap;
        let mut map = BTreeMap::new();
        map.insert("moves_camera_enabled".to_string(), "true".to_string());
        map.insert("moves_camera_horizontal".to_string(), "1.50".to_string());
        map.insert("moves_camera_vertical".to_string(), "0.20".to_string());
        map.insert("moves_camera_depth".to_string(), "3.00".to_string());
        map.insert("moves_camera_fov".to_string(), "85.0".to_string());

        let cam = ShoulderCamera::read(|k| map.get(k).cloned());
        assert!(cam.enabled);
        assert_eq!(cam.horizontal, 1.50);
        assert_eq!(cam.vertical, 0.20);
        assert_eq!(cam.depth, 3.00);
        assert_eq!(cam.fov, 85.0);
    }
}
