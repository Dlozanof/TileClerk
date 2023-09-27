use eframe::egui;
use egui_extras::image::RetainedImage;
use std::{fs::File, io::Read};
use std::io::prelude::*;
use tile_clerk_lib::*;

struct CentralPanel {
    // Loaded image
    texture: Option<egui::TextureHandle>,

    // Current selected tile
    selected_tile: TileMetadata,

    // Selected path for PNG image
    image_path: Option<String>,
}

struct MyApp {
    // Tileset image
    image: RetainedImage,

    // Lateral menu
    tileset_info: TilesetInformation,

    // Central panel
    central_panel: CentralPanel,
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

// This macro creates a `label [field   ]` structure by providing the label contents
// and where to store them in the app state. Macro benefit is that it allows to use
// a reference in the second parameter, while a function does not because is already
// taken by `self`.
macro_rules! flabel {
    ($ui: expr, $label_content: expr, $data_storage: expr) => {
        {
            $ui.horizontal(|ui| {
                let _response = ui.add(egui::Label::new($label_content));
                let _response = ui.add(egui::TextEdit::singleline($data_storage));
            });
        }
    };
}

impl MyApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>, image: RetainedImage) -> Self {
        MyApp {
            image,
            tileset_info: TilesetInformation::default(),
            central_panel: CentralPanel {
                texture: None,
                selected_tile: TileMetadata {name: String::new(), x: 0, y: 0},
                image_path: None, 
            }
        }
    }

    fn update_bottom_bar(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {

            // Load PNG
            ui.horizontal(|ui| {
                if ui.button("Load image").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.central_panel.image_path = Some(path.display().to_string());
                    }
                }

                if let Some(picked_path) = &self.central_panel.image_path {
                        ui.label("Picked file:");
                        ui.monospace(picked_path);
                }
            });
        });
    }


    fn update_sidebar(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("side_panel").show(ctx, |ui| {

            // Show tileset information
            flabel!(ui, "Rows", &mut self.tileset_info.rows);
            flabel!(ui, "Columns", &mut self.tileset_info.columns);
            flabel!(ui, "Tile width", &mut self.tileset_info.width_px);
            flabel!(ui, "Tile height", &mut self.tileset_info.height_px);

            // Show selected tile information
            ui.add(egui::Separator::default());
            let _response = ui.add(egui::Label::new(format!("Columnn: {}", self.central_panel.selected_tile.x)));
            let _response = ui.add(egui::Label::new(format!("Row: {}", self.central_panel.selected_tile.y)));
            flabel!(ui, "Tile name", &mut self.central_panel.selected_tile.name);
    
            // Button to save tile information
            if ui.button("Save tile information").clicked() {
                println!("{}", "Button clicked");
                self.tileset_info.tiles.push(self.central_panel.selected_tile.clone());
            }

            // List of tile data
            ui.add(egui::Separator::default());
            for tile in &self.tileset_info.tiles {
                let _response = ui.add(egui::Label::new(format!("{:?}", tile)));
            }

            // Button to save tileset to a file
            if ui.button("Export .pkg file").clicked() {
                let export_string = serde_json::to_string(&self.tileset_info).expect("bad");
                let mut file = File::create("outout.json").expect("bad");
                file.write_all(format!("{}", export_string).as_bytes()).expect("bad");
            }

        });
    }

    fn update_central_panel(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {


            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                let painter = ui.painter();
                let canvas_rect = ui.max_rect();

                // Draw tileset
                if let Some(path) = &self.central_panel.image_path {
                    let texture: &egui::TextureHandle = &self.central_panel.texture.get_or_insert_with(|| {
                        // Load the texture only once.
                        ui.ctx().load_texture(
                            "tileset",
                            load_image_from_path(std::path::Path::new(&path)).expect("bad"),
                            Default::default()
                        )
                    });

                    let mut mesh = egui::Mesh::with_texture(texture.id());
                    let rect = egui::Rect::from_min_size(egui::pos2(1.0, 1.0), texture.size_vec2());
                    mesh.add_rect_with_uv(rect, egui::Rect::from_two_pos(egui::pos2(0.0,0.0), egui::pos2(1.0, 1.0)), egui::Color32::WHITE);
                    painter.add(egui::Shape::mesh(mesh));
                }

                // Add grid
                if !(self.tileset_info.columns.is_empty() || self.tileset_info.rows.is_empty() || self.tileset_info.width_px.is_empty() || self.tileset_info.height_px.is_empty()) {
                    // Line information
                    let stroke_style = egui::Stroke { width: 2.0, color: egui::Color32::WHITE };

                    // Draw vertical lines
                    for column in 0..(self.tileset_info.columns.parse::<i32>().unwrap() + 1) {
                        let x: f32 = self.tileset_info.width_px.parse::<i32>().unwrap() as f32 * column as f32;
                        let y1: f32 = 0.0;
                        let y2: f32 = self.tileset_info.height_px.parse::<i32>().unwrap() as f32 * self.tileset_info.rows.parse::<i32>().unwrap() as f32;

                        let mut line_1: Vec<egui::Pos2> = Vec::new();
                        line_1.push(egui::pos2(x, y1));
                        line_1.push(egui::pos2(x, y2));
                        painter.add(egui::Shape::line(line_1, stroke_style));
                    }

                    // Draw horizontal lines
                    for row in 0..(self.tileset_info.rows.parse::<i32>().unwrap() + 1) {
                        let y: f32 = self.tileset_info.height_px.parse::<i32>().unwrap() as f32 * row as f32;
                        let x1: f32 = 0.0;
                        let x2: f32 = self.tileset_info.width_px.parse::<i32>().unwrap() as f32 * self.tileset_info.columns.parse::<i32>().unwrap() as f32;

                        let mut line_1: Vec<egui::Pos2> = Vec::new();
                        line_1.push(egui::pos2(x1, y));
                        line_1.push(egui::pos2(x2, y));
                        painter.add(egui::Shape::line(line_1, stroke_style));
                    }

                    // Draw selected tile blue square
                    // Ref: https://github.com/emilk/egui/pull/1396/files
                    let canvas_response = ui.interact(canvas_rect, egui::Id::new("canvas"), egui::Sense::drag());

                    // Ref: https://github.com/emilk/egui/blob/fdd493d48fd23047480ac5a0219fc5f206eb0dd3/crates/egui_demo_lib/src/demo/painting.rs#L48
                    if let Some(pointer_pos) = canvas_response.interact_pointer_pos() {
                        let clicked_position = TileMetadata {
                            name: String::new(),
                            x: pointer_pos.x as i32 / self.tileset_info.width_px.parse::<i32>().unwrap(),
                            y: pointer_pos.y as i32 / self.tileset_info.height_px.parse::<i32>().unwrap(),
                        };
                        if (clicked_position.x < self.tileset_info.columns.parse::<i32>().unwrap()) && 
                            (clicked_position.y < self.tileset_info.rows.parse::<i32>().unwrap()) {
                            self.central_panel.selected_tile = clicked_position;
                            println!("{:?}", self.central_panel.selected_tile);
                        }
                    }

                    let rect = egui::Rect {
                        min: egui::Pos2 {x: self.central_panel.selected_tile.x as f32 * self.tileset_info.width_px.parse::<f32>().unwrap(), y: self.central_panel.selected_tile.y as f32 * self.tileset_info.height_px.parse::<f32>().unwrap() },
                        max: egui::Pos2 {x: (self.central_panel.selected_tile.x + 1) as f32 * self.tileset_info.width_px.parse::<f32>().unwrap(), y: (self.central_panel.selected_tile.y + 1) as f32 * self.tileset_info.height_px.parse::<f32>().unwrap() },
                    };
                    painter.rect_filled(rect, 0.0, egui::Color32::from_rgba_premultiplied(0, 0, 255, 80)); 
                }
            });
        });
    }
}


impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.update_central_panel(ctx, frame);
        self.update_sidebar(ctx, frame);
        self.update_bottom_bar(ctx, frame);
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
