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
}

impl Default for MyApp {
    fn default() -> Self {
        Self { 
            show_setings: true 
        }
    }
}


impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {

    }
}