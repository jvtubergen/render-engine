use std::{time::Duration, thread};

use camera::{Camera, Vector, Point};
use eframe::{egui::{self, TextureOptions, Modifiers, Key}, epaint::{ImageData, ColorImage, Color32, ImageDelta, TextureId, Vec2, Pos2}};
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
    changed: bool,
    image_id: Option<TextureId>,
    image_delta: ImageDelta,
    camera: Camera
}


impl Default for MyApp {
    fn default() -> Self {
        Self {
            init: false,
            changed: true,
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
            },
            camera: Camera::new(
                point![3.,3.,3.],
                Vector::new(-3., -3., -3.), // Look at (0,0,0)
                std::f32::consts::FRAC_PI_4,
                Vec2 { x: 500.0, y: 500.0 }
            )
        }
    }
}

struct Ray {
    pos: Point,
    dir: Vector
}

fn trace_ray(ray: &Ray) -> Color32 {
    if ray.dir.z >= 0.001 {
        // Render blue.
        Color32::BLUE
    } else {
        let t = ray.pos.z / -ray.dir.z;
        let p = ray.pos + ray.dir.scale(t);
        if ((p.x as i32) + (p.y as i32)) % 2 == 0 {
            Color32::BLACK
        } else {
            Color32::WHITE
        }
    }
}

impl eframe::App for MyApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        ctx.request_repaint_after(Duration::from_secs(100));


        // On initial, create the 500x500 image texture to render.
        if !self.init {

            // Disable initial render flow.
            self.init  = true;

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
        let id = self.image_id.unwrap();

        self.changed = self.changed || size != self.camera.screen;

        if self.changed {

            self.changed = false;

            self.camera.screen = size;

            // Create 16 batches for multithreaded computing rays.
            let mut batch_ranges = vec![];
            
            let step_y = h / 16;
            for j in 0..15 {
                batch_ranges.push((j * step_y, (j+1) * step_y));
            }
            batch_ranges.push((15 * step_y, h));

            // Generate rays.
            let fov = self.camera.fov;
            let pos = self.camera.pos;
            let w_c = self.camera.w_c;


            // Create thread per batch.
            let mut threads = vec![];
            for range in batch_ranges {
                threads.push(thread::spawn(move || {
                    let mut result = vec![];
                    for y in range.0 .. range.1 {
                        for x in 0..w {
                            let ray = Ray {
                                pos,
                                dir: w_c * Vector::new(
                                    2.0 * ((x as f32 / w as f32) - 0.5) *  f32::tan(0.5 * fov),
                                    2.0 * ((y as f32 / h as f32) - 0.5) * -f32::tan(0.5 * fov), 
                                    1.
                                )
                            };
                            result.push(trace_ray(&ray));
                        }
                    }
                    result
                }))
            }
            

            // Combine results.
            let result : Vec<Vec<Color32>> = threads.into_iter().map(|h| h.join().unwrap()).collect();
            let result = result.concat();

            self.image_delta.image = ImageData::Color(ColorImage {
                pixels: result, 
                size: [w, h]
            });


            // Apply delta to texture.
            ctx.tex_manager().write().set(id, self.image_delta.clone());

        }

        egui::Area::new("renderer")
            .fixed_pos(egui::pos2(0.0, 0.0))
            .show(ctx, |ui| {
                ui.image(id, Vec2 { x: w as f32, y: h as f32 });
            });

    }
}
