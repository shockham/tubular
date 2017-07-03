extern crate caper;

mod shaders;

use caper::types::{ RenderItemBuilder, TransformBuilder, MaterialBuilder };
use caper::game::Game;
use caper::imgui::Ui;
use caper::input::Key;
use caper::mesh::gen_sphere;
use caper::posteffect::PostShaderOptionsBuilder;


fn main() {
    let mut game = Game::new();

    // generate the instance positions
    let transforms = (0..100)
        .map(|i| {
            TransformBuilder::default()
                .pos(((i as f32 % 10f32) * 3f32, 5f32, (i as f32 / 10f32) * 3f32))
                .rot((0f32, 0f32, 0f32, 1f32))
                .scale((0.5f32, 1f32, 0.5f32))
                .cull(false)
                .build()
                .unwrap()
        })
    .collect::<Vec<_>>();

    // create a vector of render items
    game.add_render_item(
        RenderItemBuilder::default()
            .vertices(gen_sphere())
            .material(MaterialBuilder::default()
                .shader_name("tubular".to_string())
                .build()
                .unwrap())
            .instance_transforms(transforms)
            .build()
            .unwrap());

    // initial setup
    {
        shaders::add_custom_shaders(&mut game);
        
        game.renderer.post_effect.post_shader_options = PostShaderOptionsBuilder::default()
            .chrom_amt(1f32)
            .blur_amt(2f32)
            .blur_radius(2f32)
            .bokeh(true)
            .bokeh_focal_depth(0.45f32)
            .bokeh_focal_width(0.4f32)
            .color_offset((1f32, 1f32, 1f32, 1f32))
            .build()
            .unwrap();
    }

    loop {
        // run the engine update
        game.update(|_:&Ui|{ });

        // update the first person inputs
        game.input.handle_fp_inputs(&mut game.cam_state);
 
        // quit
        if game.input.keys_down.contains(&Key::Escape) { break; }
    }
}
