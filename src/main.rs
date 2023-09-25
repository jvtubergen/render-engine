use eframe::{egui::{self, TextureOptions, Area}, epaint::{ImageData, ColorImage, Color32, ImageDelta, TextureId, Vec2}};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(500.0, 500.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Show an image with eframe/egui",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

struct MyApp {
    init: bool,
    image_id: Option<TextureId>
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            init: false,
            image_id: None
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {


        let options : TextureOptions = TextureOptions { 
            magnification: egui::TextureFilter::Nearest, 
            minification: egui::TextureFilter::Nearest 
        }; 

        // On initial, create the 500x500 image texture to render.
        if !self.init {

            let image : ImageData = ImageData::Color(ColorImage::new([500, 500], Color32::BLACK));

            // Initiate texture.
            self.image_id = Some(ctx.tex_manager().write().alloc("renderer".to_owned(), image, options));
        }


        // Create pixels.
        let mut pixels = vec![];
        pixels.resize(500*500, Color32::from_rgb(0, 0, 0));


        // Fill in the coloring.
        for y in 0..500 {
            for x in 0..500 {
                pixels[500*y+x] = Color32::from_rgb(
                    rand::random(), 
                    rand::random(), 
                    rand::random()
                );
            }
        }

        if let Some(pos) = ctx.input(|i| i.pointer.hover_pos()) {
            let y_min = 0.max((pos.y - 5.) as usize);
            let y_max = 500.min((pos.y + 5.) as usize);

            let x_min = 0.max((pos.x - 5.) as usize);
            let x_max = 500.min((pos.x + 5.) as usize);

            for y in y_min..y_max {
                for x in x_min..x_max {
                    pixels[500*y+x] = Color32::from_rgb(
                        255, 
                        255, 
                        255
                    );
                }
            }
        }


        let image = ColorImage {
            pixels, 
            size: [500, 500]
        };


        let data = ImageData::Color(image);


        let delta = ImageDelta {
            image: data,
            pos: None,
            options
        };


        // Apply delta to texture.
        let id = self.image_id.unwrap();
        ctx.tex_manager().write().set(id, delta);


        egui::Area::new("renderer")
            .fixed_pos(egui::pos2(0.0, 0.0))
            .show(ctx, |ui| {
                ui.image(id, Vec2 { x: 500., y: 500. });
            });

    }
}
