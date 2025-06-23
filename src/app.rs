pub async fn post_data(url: &str, payload: &serde_json::Value) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client.post(url).json(payload).send().await?;
    response.text().await
}



/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,
    flag: bool,
    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
    post_url: String,
    post_payload: String,
    post_response: Option<String>,
    is_posting: bool,
    
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "very much!".to_owned(),
            value: 1.7,
            flag: true,
            post_url: "https://eofvjpqbx061wr0.m.pipedream.net/post".into(),
            post_payload: r#"{"key123": "value456"}"#.into(),
            post_response: None,
            is_posting: false,
            
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }



    
    
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {



            
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });


        egui::SidePanel::left("bullet points").show(ctx, |ui| {
            if ui.button("about location").clicked() {
                self.flag = true;
            }});


        
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Woodland Wedding 2026 - Kim, Matthias und Yoshi == kimashi9 ");

            
            
            egui_extras::install_image_loaders(ctx);
            
            
            let collage = egui::include_image!("../assets/Collage_Verotterung_Zuschnitt2.jpg");
            let desired_size = egui::vec2(128.0, 128.0); 
            ui.add(egui::Image::new(collage).fit_to_exact_size(desired_size));


            
            
            egui::Window::new("about the location").open(&mut self.flag).show(ctx, |ui| {
                ui.label("tents and bungalows:");
                
            });

            
            
            
            
            ui.horizontal(|ui| {
                ui.label("This is how much I love you: ");
                ui.text_edit_singleline(&mut self.label);
                
            });

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }

            ui.separator();

            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/main/",
                "Source code."
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
