use eframe::egui;
use egui::{Color32, Pos2, Rect, Vec2};
use rand::Rng; 
use std::collections::VecDeque;

struct FeedWaterApp {
    true_level: f32,         
    pv_history: VecDeque<f32>, 
    filtered_pv: f32,         
    rpm_pompa: f32,
    sdv_inlet: bool,
    sdv_outlet: bool,
    background: Option<egui::TextureHandle>,
}

impl Default for FeedWaterApp {
    fn default() -> Self {
        let initial_level = 50.0;
        let mut history = VecDeque::with_capacity(100);
        for _ in 0..100 { history.push_back(initial_level); }

        Self {
            true_level: initial_level,
            pv_history: history,
            filtered_pv: initial_level,
            rpm_pompa: 750.0,
            sdv_inlet: true,
            sdv_outlet: true,
            background: None,
        }
    }
}

impl FeedWaterApp {
    fn calculate_logic(&mut self) {
        let mut rng = rand::thread_rng();
        let noise = rng.gen_range(-1.5..1.5); 
        let noisy_sample = self.true_level + noise;

        self.pv_history.push_back(noisy_sample);
        if self.pv_history.len() > 100 {
            self.pv_history.pop_front();
        }
        
        let sum: f32 = self.pv_history.iter().sum();
        self.filtered_pv = sum / self.pv_history.len() as f32;

        let sp = 50.0;
        let kp = 18.75; 
        let error = sp - self.filtered_pv;
        self.rpm_pompa = (750.0 + (kp * error)).clamp(0.0, 1500.0);

        self.sdv_inlet = self.filtered_pv <= 90.0;  
        self.sdv_outlet = self.filtered_pv >= 10.0; 
    }
}

impl eframe::App for FeedWaterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let texture = {
            let handle = self.background.get_or_insert_with(|| {
                let img_data = std::fs::read("background.png").expect("background.png missing");
                let image = image::load_from_memory(&img_data).expect("gagal load gambar");
                let size = [image.width() as _, image.height() as _];
                ctx.load_texture("bg", egui::ColorImage::from_rgba_unmultiplied(size, &image.to_rgba8()), Default::default())
            });
            handle.clone()
        };

        self.calculate_logic();

        egui::CentralPanel::default().frame(egui::Frame::none()).show(ctx, |ui| {
            ui.painter().image(texture.id(), ui.max_rect(), Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)), Color32::WHITE);

            let slider_rect = Rect::from_min_size(Pos2::new(70.0, 180.0), Vec2::new(40.0, 245.0));
            ui.allocate_ui_at_rect(slider_rect, |ui| {
                ui.style_mut().spacing.slider_width = 245.0; 
                ui.style_mut().spacing.interact_size.y = 20.0; 
                ui.add_sized(ui.available_size(), egui::Slider::new(&mut self.true_level, 0.0..=100.0)
                    .vertical()
                    .show_value(false));
            });

            ui.painter().text(
                Pos2::new(325.0, 125.0), 
                egui::Align2::CENTER_CENTER,
                format!("{:.0} RPM", self.rpm_pompa),
                egui::FontId::proportional(14.0),
                Color32::BLACK,
            );

            ui.painter().text(
                Pos2::new(275.0, 295.0), 
                egui::Align2::CENTER_CENTER,
                format!("{:.1}%", self.filtered_pv),
                egui::FontId::proportional(14.0),
                Color32::BLACK,
            );

            let inlet_color = if self.sdv_inlet { Color32::GREEN } else { Color32::RED };
            ui.painter().circle_filled(Pos2::new(225.0, 215.0), 15.0, inlet_color);

            let outlet_color = if self.sdv_outlet { Color32::GREEN } else { Color32::RED };
            ui.painter().circle_filled(Pos2::new(666.0, 420.0), 15.0, outlet_color);
        });

        ctx.request_repaint();
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 480.0]) 
            .with_resizable(false)
            .with_maximize_button(false),       
        ..Default::default()
    };
    eframe::run_native("Simulation Feed Water Tank", options, Box::new(|cc| {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        Ok(Box::new(FeedWaterApp::default()))
    }))
}