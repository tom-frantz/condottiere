use crate::resources::camera::CameraEntity;
use crate::resources::{PIXEL_OFFSET, PIXEL_SIZE};
use amethyst::core::ecs::*;
use amethyst::core::Transform;
use amethyst::renderer::Camera;
use amethyst::ui::{Anchor, UiTransform};
use amethyst::window::ScreenDimensions;

#[derive(Default, Debug)]
pub struct TransformUiMatcher;

impl<'s> System<'s> for TransformUiMatcher {
    type SystemData = (
        ReadExpect<'s, CameraEntity>,
        ReadExpect<'s, ScreenDimensions>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, UiTransform>,
    );

    fn run(
        &mut self,
        (camera_entity, screen_dimensions, transforms, mut ui_transforms): Self::SystemData,
    ) {
        let camera = transforms.get(camera_entity.entity).unwrap().translation();

        let game_to_real_height = |game_y: f32| {
            game_y * screen_dimensions.height() * screen_dimensions.hidpi_factor() as f32
                / camera_entity.height
        };

        let game_to_real_width = |game_x: f32| {
            game_x * screen_dimensions.width() * screen_dimensions.hidpi_factor() as f32
                / camera_entity.width
        };

        let start_x = camera.x - camera_entity.width / 2.0 + PIXEL_OFFSET;
        let start_y = camera.y - camera_entity.height / 2.0 + PIXEL_OFFSET;

        for (transform, ui_transform) in (&transforms, &mut ui_transforms).join() {
            let translation = transform.translation();

            ui_transform.anchor = Anchor::BottomLeft;
            ui_transform.local_x = game_to_real_width(translation.x / PIXEL_SIZE - start_x);
            ui_transform.local_y = game_to_real_height(translation.y / PIXEL_SIZE - start_y);
            // println!("{} {}", ui_transform.local_x, ui_transform.local_y);
        }
    }
}
