use amethyst::core::transform::Transform;
use amethyst::prelude::*;
use amethyst::assets::{AssetStorage, Loader};
use amethyst::ui::{Anchor, UiImage, UiTransform};
use amethyst::renderer::Camera;
use amethyst::renderer::Texture;
use amethyst::renderer::formats::texture::ImageFormat;
use amethyst::ecs::prelude::{Component, DenseVecStorage};

const GAME_WIDTH: f32 = 100.0;
const GAME_HEIGHT: f32 = 100.0;

pub struct Player;
impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

pub fn init_boshi(world: &mut World) {
    let sprite = {
        let loader = world.read_resource::<Loader>();
        loader.load(
            "img/boshi.jpg",
            ImageFormat::default(),
            (),
            &world.read_resource::<AssetStorage<Texture>>(),
        )
    };

    // let mut transform = Transform::default();
    // transform.set_translation_xyz(GAME_WIDTH / 2.0, GAME_HEIGHT / 2.0, 0.0);
    let transform = UiTransform::new(
        "boshi".to_owned(),
        Anchor::Middle,
        Anchor::Middle,
        GAME_WIDTH / 2.0,
        GAME_HEIGHT / 2.0,
        0.0,
        50.0,
        50.0,
    );

    world
        .create_entity()
        .with(UiImage::Texture(sprite))
        .with(transform)
        .with(Player)
        .build();
}

pub fn init_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(GAME_WIDTH / 2.0, GAME_HEIGHT / 2.0, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(GAME_WIDTH, GAME_HEIGHT))
        .with(transform)
        .build();
}
