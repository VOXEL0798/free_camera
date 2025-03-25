use bevy_butler::*;

use bevy::{
    app::Update,
    input::mouse::MouseMotion,
    math::Vec3,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
#[butler_plugin]
pub struct FreeCameraPlugin;

#[derive(Resource, Default)]
#[resource(plugin = FreeCameraPlugin, init = CameraData::new(Vec3::new(-2.5, 4.5, 9.0)))]
pub struct CameraData {
    pub position: Vec3,
    pub direction: Vec3,
    pub pitch: f32,
    pub yaw: f32,
    pub speed: f32,
}

impl CameraData {
    fn update(&mut self) {
        let tar_x = self.pitch.sin() * self.yaw.cos();
        let tar_z = self.pitch.cos() * self.yaw.cos();
        let tar_y = self.yaw.sin();

        self.direction = Vec3::new(tar_x, tar_y, tar_z) * 1.0;
    }

    fn new(position: Vec3) -> Self {
        Self {
            position,
            speed: 0.1f32,
            ..default()
        }
    }
}

#[system(schedule = Startup, plugin = FreeCameraPlugin)]
fn setup(mut commands: Commands) {
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        CameraMark,
    ));
}

#[derive(Component)]
struct CameraMark;

#[system(schedule = Update, plugin = FreeCameraPlugin)]
fn update(
    mut t_cam: Query<&mut Transform, With<CameraMark>>,
    mut camera: ResMut<CameraData>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    mut mouse_motion: EventReader<MouseMotion>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let mut primary_window = q_windows.single_mut();
    let mut transform = t_cam.single_mut();

    if keys.just_pressed(KeyCode::Escape) {
        if let CursorGrabMode::None = primary_window.cursor_options.grab_mode {
            primary_window.cursor_options.grab_mode = CursorGrabMode::Locked;
            primary_window.cursor_options.visible = false;
        } else if let CursorGrabMode::Locked = primary_window.cursor_options.grab_mode {
            primary_window.cursor_options.grab_mode = CursorGrabMode::None;
            primary_window.cursor_options.visible = true;
        }
    }

    let direction = camera.direction;
    let speed = camera.speed;

    if let CursorGrabMode::Locked = primary_window.cursor_options.grab_mode {
        if keys.pressed(KeyCode::KeyW) {
            camera.position += direction * speed;
            *transform = transform.with_translation(camera.position);
        }

        if keys.pressed(KeyCode::KeyA) {
            camera.position -= direction.cross(Vec3::Y).normalize() * speed;
            *transform = transform.with_translation(camera.position);
        }

        if keys.pressed(KeyCode::KeyD) {
            camera.position += direction.cross(Vec3::Y).normalize() * speed;
            *transform = transform.with_translation(camera.position);
        }

        if keys.pressed(KeyCode::KeyS) {
            camera.position -= direction * speed;
            *transform = transform.with_translation(camera.position);
        }

        if keys.pressed(KeyCode::Space) {
            camera.position += Vec3::Y * speed;
            *transform = transform.with_translation(camera.position);
        }

        if keys.pressed(KeyCode::ShiftLeft) {
            camera.position -= Vec3::Y * speed;
            *transform = transform.with_translation(camera.position);
        }

        for motion in mouse_motion.read() {
            camera.pitch -= motion.delta.x * 0.0009;
            camera.yaw -= motion.delta.y * 0.0006;
            camera.update();
        }

        camera.update();
        *transform = transform
            .looking_at(camera.position + camera.direction, Vec3::Y)
            .with_translation(camera.position);
    }
}
