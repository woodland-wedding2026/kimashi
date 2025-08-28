use crate::fractal_clock::FractalClock;




/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,
    flag1: bool,
    flag2: bool,
    flag3: bool,
    flag4: bool,
    value: i32,
    user_input: String,
    password_flag: bool,
    password_string: String,
    language_flag: bool,    
    button1: String,
    button2: String,
    button3: String,
    button4: String,
    #[serde(skip)]
    fractal_clock: FractalClock,
    
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "very much!".to_owned(),
            value: 0,
            flag1: false,
            flag2: false,
            flag3: false,
            flag4: false,
            user_input: "type message..".to_owned(),
            password_flag: true,
            password_string: "type password".to_owned(),
            language_flag: true,  
            button1: "button1".to_owned(),
            button2: "button2".to_owned(),
            button3: "button3".to_owned(),
            button4: "button4".to_owned(),
            fractal_clock: FractalClock::default(),
            
            
            
            
        }
    }
}


impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        
        //Self {
        //    fractal_clock: FractalClock::default(),
        //    ..Default::default()
        //};
        
        
        
        
        
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



        

        if self.language_flag == true {self.button1 = "about location".to_owned()}
        else {self.button1 = "der Ort".to_owned()}
        if self.language_flag == true {self.button2 = "weekend overview".to_owned()}
        else {self.button2 = "der Plan".to_owned()}
        if self.language_flag == true {self.button3 = "help wanted".to_owned()}
        else {self.button3 = "helfende Hände".to_owned()}
        if self.language_flag == true {self.button4 = "contact us".to_owned()}
        else {self.button4 = "Kontakt".to_owned()}
        

        if self.password_flag == true {
            egui::Window::new("password required").show(ctx, |ui| {
                ui.text_edit_singleline(&mut self.password_string); 
                if ui.button("try").clicked() {
                    if self.password_string == "kimashi".to_string() {
                        self.password_flag = false
                }
                
            }});
                       
        
        }
        
        else {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {ctx.send_viewport_cmd(egui::ViewportCommand::Close); }
                    });
                    ui.add_space(16.0);
                }
                egui::widgets::global_theme_preference_buttons(ui);
                if ui.button("de/en").clicked() {
                if self.language_flag == true {self.language_flag = false;}
                else {self.language_flag = true;}
            }
                if ui.button("logout").clicked() {
                self.password_string = "type password".to_string(); 
                self.password_flag = true;
                
            }
                
            });});

        egui::SidePanel::left("bullet points").show(ctx, |ui| {
            //ui.label(format!("You typed: {}", self.user_input));
            if ui.button(self.button1.clone()).clicked() {
                if self.flag1 == true {self.flag1 = false;}
                else {self.flag1 = true;}
            }
            if ui.button(self.button2.clone()).clicked() {
                if self.flag2 == true {self.flag2 = false;}
                else {self.flag2 = true;}
            }
            if ui.button(self.button3.clone()).clicked() {
                if self.flag3 == true {self.flag3 = false;}
                else {self.flag3 = true;}
            }
            if ui.button(self.button4.clone()).clicked() {
                if self.flag4 == true {self.flag4 = false;}
                else {self.flag4 = true;}
            }  
        });
        
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.label("Fractal Clock Example");
            ui.separator();
    
            let available_size = ui.available_size();
            let (rect, _response) = ui.allocate_exact_size(available_size, egui::Sense::hover());
            //let painter = ui.painter_at(rect);
    
            //self.fractal_clock.paint(&painter, rect);






            
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Woodland Wedding 2026 - kimashi == Kim, Matthias and Yoshi19");
            egui_extras::install_image_loaders(ctx);
            let collage = egui::include_image!("../assets/Collage_Verotterung_Zuschnitt2.jpg");
            let desired_size = egui::vec2(340.0, 340.0); 
            ui.add(egui::Image::new(collage).fit_to_exact_size(desired_size));
            egui::Window::new("about the location").open(&mut self.flag1).show(ctx, |ui| {
                ui.label("tents and bungalos, amenities, getting there, ..");                
            });

            egui::Window::new("weekend overview").open(&mut self.flag2).show(ctx, |ui| {
                ui.label("food, children, party and more :)");                
            });

            egui::Window::new("help wanted").open(&mut self.flag3).show(ctx, |ui| {
                ui.label("shifts, decoration team, music, ..");                
            });
            egui::Window::new("contact us").open(&mut self.flag4).show(ctx, |ui| {
                ui.text_edit_singleline(&mut self.user_input); 
                if ui.button("send").clicked() {
                let json1 = format!(r#"{}"#, self.user_input);
                let body1 = json1.as_bytes().to_vec();
                let request1 = ehttp::Request::post("https://ntfy.sh/woodland", body1);
                //use ehttp::Request;
                //let request1 = Request {
                //    headers: ehttp::Headers::new(&[
                //        ("Accept", "*/*"),
                //        ("Content-Type", "text/plain; charset=utf-8"),
                //        ("X-Email", "matthias.hofer@pm.me"),
                //    ]),
                //    ..Request::post("https://ntfy.sh/woodland", body1)
                //};
                ehttp::fetch(request1, move |result: ehttp::Result<ehttp::Response>| {println!("Status code: {:?}", result.unwrap().status);});
                self.value +=1;}
                ui.label(format!("you sent {} messages", self.value));
                ui.hyperlink_to("see messages", "https://ntfy.sh/woodland");
            });
            //ui.horizontal(|ui| {
            //    ui.label("This is how much I love you: ");
            //    ui.text_edit_singleline(&mut self.label);
            //    
            //});

            //ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            //if ui.button("Increment").clicked() {
            //    self.value += 1.0;
            //}

            //ui.separator();

            //ui.add(egui::github_link_file!(
            //    "https://github.com/emilk/eframe_template/blob/main/",
            //    "Source code."
            //));

            //ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            //    powered_by_egui_and_eframe(ui);
            //    egui::warn_if_debug_build(ui);
            //});
        });
        //ctx.request_repaint();
        
        }

        
    }
}

//fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
//    ui.horizontal(|ui| {
//        ui.spacing_mut().item_spacing.x = 0.0;
//        ui.label("Powered by ");
//        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
//        ui.label(" and ");
//        ui.hyperlink_to(
//            "eframe",
//            "https://github.com/emilk/egui/tree/master/crates/eframe",
//        );
//        ui.label(".");
//    });
//}
