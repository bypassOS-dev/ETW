use eframe::egui;

mod check;
use check::make_check;

#[tokio::main]
async fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "ETW", 
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    ).unwrap();
    Ok(())
}

struct MyApp {
    show_setings: bool,

    work_time: String,
    work_time_bool: bool,

    eyes: String,
    eyes_bool: bool,

    water: String,
    water_bool: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { 
            show_setings: true,

            work_time: String::new(),
            work_time_bool: true,

            eyes: String::new(),
            eyes_bool: true,

            water: String::new(),
            water_bool: true,
        }
    }
}


impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.show_setings {
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    ui.heading(egui::RichText::new("Hi! Input this:  "));


                    ui.add_space(20.0);
                    if self.work_time_bool == false {
                        ui.label(egui::RichText::new("INPUT ONLY NUMBERS MORE THEN 0!").color(egui::Color32::from_rgb(180, 50, 50)));
                    }
                    ui.add(egui::TextEdit::singleline(&mut self.work_time).hint_text("Work's time in mins:  "));


                    ui.add_space(20.0);
                    if self.eyes_bool == false {
                        ui.label(egui::RichText::new("INPUT ONLY NUMBERS MORE THEN 0!").color(egui::Color32::from_rgb(180, 50, 50)));
                    }
                    ui.add(egui::TextEdit::singleline(&mut self.eyes).hint_text("How often take rest for your eyes in mins:  "));


                    ui.add_space(20.0);
                    if self.water_bool == false {
                        ui.label(egui::RichText::new("INPUT ONLY NUMBERS MORE THEN 0!").color(egui::Color32::from_rgb(180, 50, 50)));
                    }
                    ui.add(egui::TextEdit::singleline(&mut self.water).hint_text("How often you wanna drink in minutes:  "));

                    
                    ui.add_space(20.0);
                    if ui.button(egui::RichText::new("Save").size(20.0)).clicked() {
                        (self.eyes_bool, self.water_bool, self.work_time_bool) = make_check(&self.eyes, &self.water, &self.work_time);
                    }
                });
            } else {
                
            }
        });
    }
}