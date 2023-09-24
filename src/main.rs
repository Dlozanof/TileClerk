use eframe::egui;
use egui::Vec2;
use egui_extras::image::RetainedImage;
use std::{fs::File, io::Read};


#[derive(Default)]
struct TilesetInformation
{
    rows: String,
    columns: String,
    width_px: String,
    height_px: String,
}

struct MyApp {
    // Tileset image
    image: RetainedImage,

    texture: Option<egui::TextureHandle>,


    // Lateral menu
    tileset_info: TilesetInformation,
}


fn load_image_from_path(path: &std::path::Path) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::io::Reader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}

impl MyApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>, image: RetainedImage) -> Self {
        MyApp { image: image, tileset_info: TilesetInformation::default(), texture: None}
    }

    fn update_sidebar(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("side_panel").show(ctx, |ui| {

            let _response = ui.add(egui::Label::new("Rows"));
            let _response = ui.add(egui::TextEdit::singleline(&mut self.tileset_info.rows));
            
            let _response = ui.add(egui::Label::new("Columns"));
            let _response = ui.add(egui::TextEdit::singleline(&mut self.tileset_info.columns));

            let _response = ui.add(egui::Label::new("Tile width"));
            let _response = ui.add(egui::TextEdit::singleline(&mut self.tileset_info.width_px));

            let _response = ui.add(egui::Label::new("Tile height"));
            let _response = ui.add(egui::TextEdit::singleline(&mut self.tileset_info.height_px));
    
            if ui.button("Done!").clicked() {
                println!("{}", "Button clicked")
            }

        });
    }
}



impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                let painter = ui.painter();
                let stroke_style = egui::Stroke { width: 2.0, color: egui::Color32::WHITE };
                let mut lines: Vec<Vec<egui::Pos2>> = Vec::new();
                let mut line_1: Vec<egui::Pos2> = Vec::new();
                line_1.push(egui::pos2(0.0, 0.0));
                line_1.push(egui::pos2(0.0, 64.0));

                let texture: &egui::TextureHandle = &self.texture.get_or_insert_with(|| {
                    // Load the texture only once.
                    ui.ctx().load_texture(
                        "tileset",
                        load_image_from_path(std::path::Path::new("tileset.png")).expect("bad"),
                        Default::default()
                    )
                });

                let mut mesh = egui::Mesh::with_texture(texture.id());
                let rect = egui::Rect::from_min_size(egui::pos2(1.0, 1.0), texture.size_vec2());
                mesh.add_rect_with_uv(rect, egui::Rect::from_two_pos(egui::pos2(0.0,0.0), egui::pos2(1.0, 1.0)), egui::Color32::WHITE);
                painter.add(egui::Shape::mesh(mesh));
                painter.add(egui::Shape::line(line_1, stroke_style));
                //painter.extend(shapes);
            });

            //egui::ScrollArea::new([true, true]).show(ui, |ui| {
            //    self.image.show(ui);
            //    //ui.image(egui::include_image!("tileset.png"));
            //});
        });
        self.update_sidebar(ctx, frame);
    }

}

fn main() -> Result<(), eframe::Error> {
    //env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        //initial_window_size: Some(egui::vec2(600.0, 800.0)),
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
