extern crate piston_window;
extern crate itertools;
extern crate simple_ecs;

mod initializer;
mod typedefs;

use std::cmp::Ord;
use itertools::*;
use piston_window::*;
use typedefs::*;
use initializer::Initializer;
use simple_ecs::either::*;
use simple_ecs::entity::EntityId;
use simple_ecs::components::SetComponent;
use simple_ecs::system::{SimpleSystem, SystemStore};

static DRAW_STATE: DrawState = DrawState {
    scissor: None,
    stencil: None,
    blend: None,
};

struct SettingsComponent {
    up: Key,
    down: Key,
    left: Key,
    right: Key,
}

impl Default for SettingsComponent {
    fn default() -> Self {
        SettingsComponent {
            up: Key::Up,
            down: Key::Down,
            left: Key::Left,
            right: Key::Right,
        }
    }
}

struct CameraComponent;
impl SetComponent for CameraComponent {}

struct RenderComponent {
    texture: Texture2d,
    depth: i8,
}
impl SetComponent for RenderComponent {}

struct VelocityComponent(f64, f64);
impl SetComponent for VelocityComponent {}

#[derive(Default)]
struct AccelerationComponent(f64, f64);
impl SetComponent for AccelerationComponent {}

struct PositionComponent(f64, f64);
impl SetComponent for PositionComponent {}

struct ClipComponent(u32, u32, u32, u32);
impl SetComponent for ClipComponent {}

struct ScaleComponent(f64);
impl SetComponent for ScaleComponent {}

struct MovementSystem;

impl SimpleSystem<(), UpdateData> for MovementSystem {
    type Input = (
        &'static PositionComponent,
        &'static VelocityComponent,
        Option<&'static AccelerationComponent>,
    );
    type Output = (PositionComponent, VelocityComponent);

    fn update(
        &mut self,
        entities: &[
            (
                EntityId,
                (
                    &PositionComponent,
                    &VelocityComponent,
                    Option<&AccelerationComponent>,
                )
            )
        ],
        ud: &UpdateData
    ) -> Vec<(EntityId, (PositionComponent, VelocityComponent), ())> {
        if let Some(Event::Update(UpdateArgs { dt })) = ud.event {
            entities.into_iter()
                .map(|
                    &(
                        e,
                        (
                            &PositionComponent(x, y),
                            &VelocityComponent(vx, vy),
                            o_ac
                        )
                    )
                | {
                    let &AccelerationComponent(ax, ay) = o_ac
                        .unwrap_or(&Default::default());
                    (
                        e,
                        (
                            PositionComponent(x + vx * dt, y + vy * dt),
                            VelocityComponent(
                                vx + ax * dt,
                                vy + ay * dt,
                            ),
                        ),
                        ()
                    )
                })
                .collect()
        } else {
            vec![]
        }
    }
}

struct InputSystem;

impl SimpleSystem<(), UpdateData> for InputSystem {
    type Input = (
        &'static AccelerationComponent,
        Option<&'static SettingsComponent>,
    );
    type Output = AccelerationComponent;

    fn update(
        &mut self,
        entities: &[
            (
                EntityId,
                (
                    &AccelerationComponent,
                    Option<&SettingsComponent>,
                )
            )
        ],
        ud: &UpdateData
    ) -> Vec<(EntityId, AccelerationComponent, ())> {
        unimplemented!();
    }
}

struct RenderSystem;

impl SimpleSystem<(), UpdateData> for RenderSystem {
    type Input = Either<(
        &'static RenderComponent,
        &'static PositionComponent,
        Option<&'static ClipComponent>,
        Option<&'static ScaleComponent>,
    ), (
        &'static PositionComponent,
        &'static CameraComponent,
    )>;
    type Output = ();

    fn update(
        &mut self,
        entities: &[
            (
                EntityId,
                Either<(
                    &RenderComponent,
                    &PositionComponent,
                    Option<&ClipComponent>,
                    Option<&ScaleComponent>,
                ), (
                    &PositionComponent,
                    &CameraComponent,
                )>
            )
        ],
        ud: &UpdateData
    ) -> Vec<(EntityId, (), ())> {
        ud.draw_2d(|_, g| {
            clear([1.0, 1.0, 1.0, 1.0], g);
        });

        let Size { width, height } = ud.window.borrow().draw_size();

        let &PositionComponent(camera_x, camera_y) = entities.iter()
            .filter_map(|&(_, ref either)| either.right().map(|&(p, _)| p))
            .next()
            .unwrap_or(&PositionComponent(0.0, 0.0));

        let (offset_x, offset_y) = (
            camera_x - width as f64 / 2.0,
            camera_y - height as f64 / 2.0,
        );

        for &(rc, pc, o_cc, o_sc) in entities
            .iter()
            .filter_map(|&(_, ref either)| either.left())
            .sorted_by(|a, b|
                a.0.depth.cmp(&b.0.depth)
            )
        {
            ud.draw_2d(|c, g| {
                let &ClipComponent(sx, sy, w, h) = o_cc
                    .unwrap_or(
                        &ClipComponent(
                            0,
                            0,
                            rc.texture.get_width(),
                            rc.texture.get_height()
                        )
                    );

                let &ScaleComponent(scale) = o_sc
                    .unwrap_or(&ScaleComponent(1.0));

                let &PositionComponent(x, y) = pc;

                Image::new()
                    .rect([
                        x - offset_x,
                        y - offset_y,
                        w as f64 * scale,
                        h as f64 * scale
                    ])
                    .src_rect([sx as i32, sy as i32, w as i32, h as i32])
                    .draw(&rc.texture, &DRAW_STATE, c.transform, g);
            });
        }

        vec![]
    }
}

struct CameraSystem {
    follow: EntityId,
}

impl SimpleSystem<(), UpdateData> for CameraSystem {
    type Input = (
        &'static PositionComponent,
        Option<(&'static VelocityComponent, &'static CameraComponent)>,
    );
    type Output = (PositionComponent, VelocityComponent);

    fn update(
        &mut self,
        entities: &[
            (
                EntityId,
                (
                    &PositionComponent,
                    Option<(&VelocityComponent, &CameraComponent)>,
                )
            )
        ],
        ud: &UpdateData
    ) -> Vec<(EntityId, (PositionComponent, VelocityComponent), ())> {
        if let Some(camera_id) = entities.iter()
            .filter_map(|&(e, (_, o))| o.map(|_| e))
            .next()
        {
        }

        unimplemented!();
    }
}

fn main() {
    let window: PistonWindow = WindowSettings::new("Roit", [512; 2])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let tc0 = RenderComponent {
        texture: Texture::from_path(
            &mut *window.factory.borrow_mut(),
            "face.png",
            Flip::None,
            &TextureSettings::new()
        ).unwrap(),
        depth: 1,
    };

    let tc1 = RenderComponent {
        texture: Texture::from_path(
            &mut *window.factory.borrow_mut(),
            "face.png",
            Flip::None,
            &TextureSettings::new()
        ).unwrap(),
        depth: 0,
    };

    let init_sys = Initializer::new(move |es| {
        let e = es.create_entity();
        (
            tc0,
            PositionComponent(-40.0, -40.0),
            VelocityComponent(5.0, 0.0),
            ClipComponent(100, 160, 120, 120),
            ScaleComponent(2.0),
            AccelerationComponent(-5.0, 0.0),
        ).set_component(es, e);

        let e = es.create_entity();
        (
            tc1,
            PositionComponent(-40.0, -40.0),
            VelocityComponent(10.0, 0.0),
            ScaleComponent(0.5),
        ).set_component(es, e);

        let e = es.create_entity();
        (
            PositionComponent(0.0, 0.0),
            VelocityComponent(0.0, -10.0),
            CameraComponent,
        ).set_component(es, e);

        let e = es.create_entity();
        es.set_component(
            e,
            SettingsComponent::default()
        );
    });

    let mut sys_store = SystemStore::with_systems(
        vec![
            Box::new(init_sys),
            Box::new(MovementSystem),
            Box::new(RenderSystem),
        ]
    );

    for e in window.max_fps(u64::max_value()) {
        sys_store.update(e);
    }
}
