use camera::{Camera, Vector, Point};
use eframe::{egui::{self, TextureOptions}, epaint::{ImageData, ColorImage, Color32, ImageDelta, TextureId, Vec2, Pos2}};
use nalgebra::{point, Point3, Unit, Vector3, Perspective3, Isometry3};

mod camera;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(500.0, 500.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Basic renderer",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

struct MyApp {
    init: bool,
    image_id: Option<TextureId>,
    image_delta: ImageDelta,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            init: false,
            image_id: None,
            image_delta: ImageDelta { 
                image: ImageData::Color(
                    ColorImage::new(
                        [500, 500], 
                        Color32::BLACK
                    )
                ), 
                options: TextureOptions { 
                    magnification: egui::TextureFilter::Nearest, 
                    minification: egui::TextureFilter::Nearest 
                }, 
                pos: None
            }
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // On initial, create the 500x500 image texture to render.
        if !self.init {

            // Initiate texture.
            self.image_id = Some(
                ctx.tex_manager().write().alloc(
                    "renderer".to_owned(), 
                    self.image_delta.image.clone(), 
                    self.image_delta.options
                )
            );
        }

        let size = ctx.screen_rect().size();
        let w = size.x as usize;
        let h = size.y as usize;

        // Create pixels.
        let mut pixels = vec![];
        pixels.resize(h*w, Color32::from_rgb(0, 0, 0));



        let camera = Camera::new(
            point![3.,3.,3.],
            Vector::new(-3., -3., -3.), // Look at (0,0,0)
            std::f32::consts::FRAC_PI_4,
            size
        );

        for y in 0..h {
            for x in 0..w {
                let ray = camera.ray(x as f32, y as f32);
                // Check for intersection with world (x,y)-plane.  
                if ray.z >= 0.001 {
                    // Render blue.
                    pixels[w*y+x] = Color32::BLUE;
                } else {
                    let t = camera.pos.z / -ray.z;
                    let p = camera.pos + ray.scale(t);
                    if ((p.x as i32) + (p.y as i32)) % 2 == 0 {
                        pixels[w*y+x] = Color32::BLACK;
                    } else {
                        pixels[w*y+x] = Color32::WHITE;
                    }
                };
            }
        }


        // Render white box at mouse position.
        if let Some(pos) = ctx.input(|i| i.pointer.hover_pos()) {
            let y_min = 0.max((pos.y - 5.) as usize);
            let y_max = h.min((pos.y + 5.) as usize);

            let x_min = 0.max((pos.x - 5.) as usize);
            let x_max = w.min((pos.x + 5.) as usize);

            for y in y_min..y_max {
                for x in x_min..x_max {
                    pixels[w*y+x] = Color32::from_rgb(
                        255,255,255
                    );
                }
            }
        }



        self.image_delta.image = ImageData::Color(ColorImage {
            pixels, 
            size: [w, h]
        });


        // Apply delta to texture.
        let id = self.image_id.unwrap();
        ctx.tex_manager().write().set(id, self.image_delta.clone());


        egui::Area::new("renderer")
            .fixed_pos(egui::pos2(0.0, 0.0))
            .show(ctx, |ui| {
                ui.image(id, Vec2 { x: w as f32, y: h as f32 });
            });

    }
}
