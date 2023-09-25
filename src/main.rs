use eframe::{egui::{self, TextureOptions}, epaint::{ImageData, ColorImage, Color32, ImageDelta, TextureId, Vec2}};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(600.0, 600.0)),
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


        // Show image in egui.
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("This is an image:");
            ui.image(id, Vec2 { x: 500., y: 500. });
        });
    }
}


struct MyImage {
    image_data: [u8;500*500]
}

impl Into<ImageData> for MyImage {
    fn into(self) -> ImageData {

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

        let color_image = ColorImage {
            pixels, 
            size: [500, 500]
        };

        ImageData::Color(color_image)
        
    }
} //
