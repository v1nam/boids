use macroquad::prelude::*;
use macroquad::rand::{gen_range, ChooseRandom};

type ImplIteratorMut<'a, Item> =
    ::std::iter::Chain<::std::slice::IterMut<'a, Item>, ::std::slice::IterMut<'a, Item>>;
trait SplitOneMut {
    type Item;

    fn split_one_mut(
        self: &'_ mut Self,
        i: usize,
    ) -> (&'_ mut Self::Item, ImplIteratorMut<'_, Self::Item>);
}

impl<T> SplitOneMut for [T] {
    type Item = T;

    fn split_one_mut(
        self: &'_ mut Self,
        i: usize,
    ) -> (&'_ mut Self::Item, ImplIteratorMut<'_, Self::Item>) {
        let (prev, current_and_end) = self.split_at_mut(i);
        let (current, end) = current_and_end.split_at_mut(1);
        (&mut current[0], prev.iter_mut().chain(end))
    }
}

const MOVE_SPEED: f32 = 0.3;
const LOOK_SPEED: f32 = 0.14;

struct Boid {
    position: Vec3,
    velocity: Vec3,
    p1: Vec3,
    p2: Vec3,
    color: Color,
}

pub async fn run() {
    let mut boids: Vec<Boid> = Vec::new();
    let colors = vec![
        Color::from_rgba(129, 161, 193, 255),
        Color::from_rgba(191, 97, 106, 255),
        Color::from_rgba(208, 135, 112, 255),
        Color::from_rgba(163, 190, 140, 255),
        Color::from_rgba(235, 203, 139, 255),
        Color::from_rgba(143, 188, 187, 255),
        Color::from_rgba(136, 192, 208, 255),
    ];
    for _ in 0..100 {
        let pos = vec3(
            gen_range(-12.0, 12.0),
            gen_range(-12.0, 12.0),
            gen_range(-12.0, 12.0),
        );
        let velocity = vec3(
            gen_range(-0.2, 0.2),
            gen_range(-0.2, 0.2),
            gen_range(-0.2, 0.2),
        );
        boids.push(Boid {
            position: pos,
            velocity,
            p1: pos + vec3(0.2, 0.2, 0.2),
            p2: pos - vec3(0.2, 0.2, 0.2),
            color: *colors.choose().unwrap(),
        });
    }
    let boid_count: usize = boids.len();

    let mut x: f32 = 0.0;
    let mut switch: bool = false;
    let bounds: f32 = 8.0;

    let world_up: Vec3 = vec3(0.0, 1.0, 0.0);
    let mut yaw: f32 = 1.18;
    let mut pitch: f32 = 0.0;

    let mut front = vec3(
        yaw.cos() * pitch.cos(),
        pitch.sin(),
        yaw.sin() * pitch.cos(),
    )
    .normalize();
    let mut right = front.cross(world_up).normalize();
    let mut up;

    let mut position = vec3(0.0, 1.0, 0.0);
    let mut last_mouse_position: Vec2 = mouse_position().into();

    set_cursor_grab(true);
    show_mouse(false);

    loop {
        let delta = get_frame_time();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            position += front * MOVE_SPEED;
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            position -= front * MOVE_SPEED;
        }

        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            position -= right * MOVE_SPEED;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            position += right * MOVE_SPEED;
        }

        let mouse_position: Vec2 = mouse_position().into();
        let mouse_delta = mouse_position - last_mouse_position;
        last_mouse_position = mouse_position;

        yaw += mouse_delta.x * delta * LOOK_SPEED;
        pitch += mouse_delta.y * delta * -LOOK_SPEED;

        pitch = if pitch > 1.5 { 1.5 } else { pitch };
        pitch = if pitch < -1.5 { -1.5 } else { pitch };

        front = vec3(
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            yaw.sin() * pitch.cos(),
        )
        .normalize();

        right = front.cross(world_up).normalize();
        up = right.cross(front).normalize();

        x += if switch { 0.04 } else { -0.04 };
        if x >= bounds || x <= -bounds {
            switch = !switch;
        }

        clear_background(Color::from_rgba(36, 42, 54, 255));

        set_camera(&Camera3D {
            position,
            up,
            target: front + position,
            ..Default::default()
        });

        for bi in 0..boid_count {
            let (boid, others) = boids.split_one_mut(bi);
            let mut nbrs = 0;
            let mut center = Vec3::ZERO;
            let mut avg_vel = Vec3::ZERO;
            let mut move_ = Vec3::ZERO;

            for boid_nbr in others {
                if (boid.position - boid_nbr.position).length() < 2.7 {
                    center += boid_nbr.position;
                    avg_vel += boid_nbr.velocity;
                    nbrs += 1;
                }
                if (boid.position - boid_nbr.position).length() < 0.5 {
                    move_ += boid.position - boid_nbr.position;
                }
            }

            if (boid.position - position).length() < 0.5 {
                move_ += (boid.position - position) * 1.3;
            }
            boid.velocity += move_ * 0.05;
            if nbrs > 0 {
                center /= nbrs as f32;
                avg_vel /= nbrs as f32;
                boid.velocity += (center - boid.position) * 0.001;
                boid.velocity += (avg_vel - boid.velocity) * 0.05;
            }
            let margin = 3.5;
            for axi in 0..3 {
                if boid.position[axi] >= 12. - margin {
                    boid.velocity[axi] -= 0.005;
                }
                if boid.position[axi] <= -12. + margin {
                    boid.velocity[axi] += 0.005;
                }
            }
            boid.velocity = boid.velocity.clamp_length_max(0.2);
            boid.position += boid.velocity;
            boid.p1 = boid.position + boid.velocity.normalize() * 0.2;
            boid.p2 = boid.position - boid.velocity.normalize() * 0.2;
            draw_line_3d(boid.p1, boid.p2, boid.color);
        }
        draw_cube_wires(
            vec3(0., 0., 0.),
            vec3(24., 24., 24.),
            Color::from_rgba(216, 222, 233, 255),
        );

        set_default_camera();
        next_frame().await
    }
}
