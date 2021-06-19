use macroquad::prelude::*;
use macroquad::rand::gen_range;

type ImplIteratorMut<'a, Item> =
    ::std::iter::Chain<
        ::std::slice::IterMut<'a, Item>,
        ::std::slice::IterMut<'a, Item>,
    >
;
trait SplitOneMut {
    type Item;

    fn split_one_mut (
        self: &'_ mut Self,
        i: usize,
    ) -> (&'_ mut Self::Item, ImplIteratorMut<'_, Self::Item>);
}

impl<T> SplitOneMut for [T] {
    type Item = T;
    
    fn split_one_mut (
        self: &'_ mut Self,
        i: usize,
    ) -> (&'_ mut Self::Item, ImplIteratorMut<'_, Self::Item>)
    {
        let (prev, current_and_end) = self.split_at_mut(i);
        let (current, end) = current_and_end.split_at_mut(1);
        (
            &mut current[0],
            prev.iter_mut().chain(end),
        )
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Window Conf".to_owned(),
        window_width: 1024,
        window_height: 720,
        ..Default::default()
    }
}

#[derive(Copy, Clone)]
struct Boid {
    p1: Vec2,
    p2: Vec2,
    p3: Vec2,
    angle: f32,
    centroid: Vec2,
    velocity: Vec2,
}


impl Boid {
    fn new(p1: Vec2, p2: Vec2, p3: Vec2) -> Boid {
        let centroid = vec2((p1.x + p2.x + p3.x) / 3., (p1.y + p2.y + p3.y) / 3.);
        Boid {
            p1,
            p2,
            p3,
            centroid,
            angle: (90.0_f32).to_radians(),
            velocity: vec2(gen_range(-5.0, 5.0), gen_range(-5.0, 5.0)),
        }
    }
    fn rotate(&mut self, r_angle: f32) {
        let angle = r_angle - self.angle;
        self.angle = r_angle;

        let p1_x = self.p1.x - self.centroid.x;
        self.p1.x =
            p1_x * angle.cos() + (self.p1.y - self.centroid.y) * angle.sin() + self.centroid.x;
        self.p1.y =
            -p1_x * angle.sin() + (self.p1.y - self.centroid.y) * angle.cos() + self.centroid.y;

        let p2_x = self.p2.x - self.centroid.x;
        self.p2.x =
            p2_x * angle.cos() + (self.p2.y - self.centroid.y) * angle.sin() + self.centroid.x;
        self.p2.y =
            -p2_x * angle.sin() + (self.p2.y - self.centroid.y) * angle.cos() + self.centroid.y;

        let p3_x = self.p3.x - self.centroid.x;
        self.p3.x =
            p3_x * angle.cos() + (self.p3.y - self.centroid.y) * angle.sin() + self.centroid.x;
        self.p3.y =
            -p3_x * angle.sin() + (self.p3.y - self.centroid.y) * angle.cos() + self.centroid.y;
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut boids: Vec<Boid> = Vec::new();
    let delta: f64 = 1. / 60.;
    let mut time_left: f64 = 0.;
    for _ in 0..100 {
        let pos = vec2(gen_range(0.0, screen_width()), gen_range(0.0, screen_height()));
        boids.push(Boid::new(
            vec2(pos.x, pos.y),
            vec2(pos.x + 5., pos.y - 15.),
            vec2(pos.x + 10., pos.y),
        ));
    }
    let boid_count: f32 = boids.len() as f32;

    loop {
        time_left += get_frame_time() as f64;
        while time_left >= delta {
            time_left -= delta;
            for i in 0..boid_count as usize {
                let (boid, others) = boids.split_one_mut(i);
                let mut nbrs = 0;
                let mut center = Vec2::ZERO;
                let mut avg_vel = Vec2::ZERO;
                let mut move_ = Vec2::ZERO;

                for boid_nbr in others {
                    if (boid.centroid - boid_nbr.centroid).length() < 75. {
                        center += boid_nbr.centroid;
                        avg_vel += boid_nbr.velocity;
                        nbrs += 1;
                    }
                    if (boid.centroid - boid_nbr.centroid).length() < 25. {
                        move_ += boid.centroid - boid_nbr.centroid;
                    }
                }
                boid.velocity += move_ * 0.05;
                if nbrs > 0 {
                    center /= nbrs as f32;
                    avg_vel /= nbrs as f32;
                    boid.velocity += (center - boid.centroid) * 0.005;
                    boid.velocity += (avg_vel - boid.velocity) * 0.05;
                }
                let margin = 80.;
                if boid.centroid.x < margin {
                    boid.velocity.x += 1.;
                }
                if boid.centroid.x > screen_width() - margin {
                    boid.velocity.x -= 1.;
                }
                if boid.centroid.y < margin {
                    boid.velocity.y += 1.;
                }
                if boid.centroid.y > screen_height() - margin {
                    boid.velocity.y -= 1.;
                }
                boid.velocity = boid.velocity.clamp_length_max(5.5);
                boid.centroid += boid.velocity;
                boid.p1 += boid.velocity;
                boid.p2 += boid.velocity;
                boid.p3 += boid.velocity;
                let m_angle = -(boid.velocity.y).atan2(boid.velocity.x);
                boid.rotate(m_angle);
            }
        }
        clear_background(Color::from_rgba(36, 42, 54, 255));
        for boid in boids.iter() {
            draw_triangle(
                boid.p1,
                boid.p2,
                boid.p3,
                Color::from_rgba(129, 161, 193, 255),
            );
        }
        next_frame().await
    }
}
