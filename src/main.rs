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

#[derive(Debug)]
struct TilePosition {
    x: i32,
    y: i32,
}

struct MyApp {
    // Tileset image
    image: RetainedImage,

    texture: Option<egui::TextureHandle>,

    // Lateral menu
    tileset_info: TilesetInformation,

    // Current selected tile
    selected_tile: TilePosition,
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
        MyApp { image, tileset_info: TilesetInformation::default(), texture: None, selected_tile: TilePosition {x: 0, y: 0}}
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
                let canvas_rect = ui.max_rect();

                let stroke_style = egui::Stroke { width: 2.0, color: egui::Color32::WHITE };

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

                // Add grid
                if self.tileset_info.columns.is_empty() || self.tileset_info.rows.is_empty() || self.tileset_info.width_px.is_empty() || self.tileset_info.height_px.is_empty() {
                    // Skip
                }
                else {
                    for column in 0..(self.tileset_info.columns.parse::<i32>().unwrap() + 1) {
                        let x: f32 = self.tileset_info.width_px.parse::<i32>().unwrap() as f32 * column as f32;
                        let y1: f32 = 0.0;
                        let y2: f32 = self.tileset_info.height_px.parse::<i32>().unwrap() as f32 * self.tileset_info.rows.parse::<i32>().unwrap() as f32;

                        let mut line_1: Vec<egui::Pos2> = Vec::new();
                        line_1.push(egui::pos2(x, y1));
                        line_1.push(egui::pos2(x, y2));
                        painter.add(egui::Shape::line(line_1, stroke_style));
                    }
                    for row in 0..(self.tileset_info.rows.parse::<i32>().unwrap() + 1) {
                        let y: f32 = self.tileset_info.height_px.parse::<i32>().unwrap() as f32 * row as f32;
                        let x1: f32 = 0.0;
                        let x2: f32 = self.tileset_info.width_px.parse::<i32>().unwrap() as f32 * self.tileset_info.columns.parse::<i32>().unwrap() as f32;

                        let mut line_1: Vec<egui::Pos2> = Vec::new();
                        line_1.push(egui::pos2(x1, y));
                        line_1.push(egui::pos2(x2, y));
                        painter.add(egui::Shape::line(line_1, stroke_style));
                    }


                    // Ref: https://github.com/emilk/egui/pull/1396/files
                    let canvas_response = ui.interact(canvas_rect, egui::Id::new("canvas"), egui::Sense::drag());

                    // Ref: https://github.com/emilk/egui/blob/fdd493d48fd23047480ac5a0219fc5f206eb0dd3/crates/egui_demo_lib/src/demo/painting.rs#L48
                    if let Some(pointer_pos) = canvas_response.interact_pointer_pos() {
                        self.selected_tile = TilePosition {
                            x: pointer_pos.x as i32 / self.tileset_info.width_px.parse::<i32>().unwrap(),
                            y: pointer_pos.y as i32 / self.tileset_info.height_px.parse::<i32>().unwrap(),
                        };
                        println!("{:?}", pointer_pos);
                        println!("-> {:?}", self.selected_tile);
                    }

                    // Print rectangle in selected position
                    let rect = egui::Rect {
                        min: egui::Pos2 {x: self.selected_tile.x as f32 * self.tileset_info.width_px.parse::<f32>().unwrap(), y: self.selected_tile.y as f32 * self.tileset_info.height_px.parse::<f32>().unwrap() },
                        max: egui::Pos2 {x: (self.selected_tile.x + 1) as f32 * self.tileset_info.width_px.parse::<f32>().unwrap(), y: (self.selected_tile.y + 1) as f32 * self.tileset_info.height_px.parse::<f32>().unwrap() },
                    };
                    painter.rect_filled(rect, 0.0, egui::Color32::from_rgba_premultiplied(0, 0, 255, 80)); 
                }
            });

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
