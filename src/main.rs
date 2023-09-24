use eframe::egui;
use egui_extras::image::RetainedImage;
use std::{fs::File, io::Read};



pub struct MyApp {

    // Tileset image
    pub image: RetainedImage,
}

impl MyApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>, image: RetainedImage) -> Self {
        MyApp { image: image }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::new([true, true]).show(ui, |ui| {
                self.image.show(ui);
                //ui.image(egui::include_image!("tileset.png"));
            });
        });
    }

}

fn main() -> Result<(), eframe::Error> {
    //env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(600.0, 800.0)),
        ..Default::default()
    };

    let mut buffer = vec![];    
    File::open ("tileset.png").expect("bad").read_to_end(&mut buffer).expect("Bad");
    let img = RetainedImage::from_image_bytes ("tileset.png", &buffer[..]).expect("bad");


    eframe::run_native(
        "Image Viewer",
        options,
        Box::new(|cc| {
            Box::new(MyApp::new(cc, img))
        }),
    )
}



//impl eframe::App for MyApp {
//    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//        egui::CentralPanel::default().show(ctx, |ui| {
//            egui::ScrollArea::new([true, true]).show(ui, |ui| {
//                ui.image(egui::include_image!("tileset.png"));
//
//                ui.add(
//                    egui::Image::new("https://picsum.photos/seed/1.759706314/1024")
//                        .rounding(egui::Rounding::same(10.0)),
//                );
//            });
//        });
//    }
//}
