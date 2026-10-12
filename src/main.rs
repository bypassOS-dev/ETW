use eframe::egui;

mod unused;

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
    eyes: String,
    water: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { 
            show_setings: true,
            work_time: String::new(),
            eyes: String::new(),
            water: String::new(),
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
                    ui.add(egui::TextEdit::singleline(&mut self.work_time).hint_text("Work's time in mins:  "));

                    ui.add_space(20.0);
                    ui.add(egui::TextEdit::singleline(&mut self.eyes).hint_text("How often take rest for your eyes in mins:  "));

                    ui.add_space(20.0);
                    ui.add(egui::TextEdit::singleline(&mut self.water).hint_text("How often you wanna drink in minutes:  "));
                });
            } else {
                
            }
        });
    }
}
