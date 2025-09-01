use crate::fractal_clock::FractalClock;
use crate::snake::SnakeGame;
use crate::snake::Direction;
use crate::painting::PaintingApp;


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
    flag5: bool,
    flag6: bool,
    flag7: bool,
    value: i32,
    user_input: String,
    password_flag: bool,
    password_string: String,
    language_flag: bool,    
    button1: String,
    button2: String,
    button3: String,
    button4: String,
    button5: String,
    button6: String,
    button7: String,
    #[serde(skip)]
    fractal_clock: FractalClock,
    snake: SnakeGame,
    last_time: f64,
    #[serde(skip)]
    painting_app: PaintingApp,
    saved_image_data: Option<String>,
    
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
            flag5: false,
            flag6: false,
            flag7: false,
            user_input: "type message..".to_owned(),
            password_flag: true,
            password_string: "type password".to_owned(),
            language_flag: true,  
            button1: "button1".to_owned(),
            button2: "button2".to_owned(),
            button3: "button3".to_owned(),
            button4: "button4".to_owned(),
            button5: "button2".to_owned(),
            button6: "button3".to_owned(),
            button7: "button4".to_owned(),
            fractal_clock: FractalClock::default(),
            snake: SnakeGame::default(),
            last_time: 0.0,

            painting_app: PaintingApp::default(),
            saved_image_data: None,
            
            
            
        }
    }
}


impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Try to restore previous state first
        if let Some(storage) = cc.storage {
            if let Some(loaded) = eframe::get_value::<Self>(storage, eframe::APP_KEY) {
                return loaded;
            }
        }
    
        // If no previous state, return a new default instance
        Self {
            fractal_clock: FractalClock::default(),
            ..Default::default()
        }










        
        //Self {
        //    fractal_clock: FractalClock::default(),
        //    ..Default::default()
        //};
        
        
        
        
        
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        
        
        //if let Some(storage) = cc.storage {
        //    return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        //}

        //Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    



    
   
    
    
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {


        let time = ctx.input(|i| i.time);
        let dt = (time - self.last_time) as f32;
        self.last_time = time;
        

        if self.language_flag == true {self.button1 = "about location".to_owned()}
        else {self.button1 = "der Ort".to_owned()}
        if self.language_flag == true {self.button2 = "weekend overview".to_owned()}
        else {self.button2 = "der Plan".to_owned()}
        if self.language_flag == true {self.button3 = "help wanted".to_owned()}
        else {self.button3 = "helfende Hände".to_owned()}
        if self.language_flag == true {self.button4 = "contact us".to_owned()}
        else {self.button4 = "Kontakt".to_owned()}

        if self.language_flag == true {self.button5 = "painting".to_owned()}
        else {self.button5 = "malen".to_owned()}
        if self.language_flag == true {self.button6 = "play snake".to_owned()}
        else {self.button6 = "spiel Snake".to_owned()}
        if self.language_flag == true {self.button7 = "fractal clock".to_owned()}
        else {self.button7 = "Fraktaluhr".to_owned()}
        

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
            if ui.button(self.button5.clone()).clicked() {
                if self.flag5 == true {self.flag5 = false;}
                else {self.flag5 = true;}
            }  
            if ui.button(self.button6.clone()).clicked() {
                if self.flag6 == true {self.flag6 = false;}
                else {self.flag6 = true;}
            }  
            if ui.button(self.button7.clone()).clicked() {
                if self.flag7 == true {self.flag7 = false;}
                else {self.flag7 = true;}
            }  






            
        });


        





            
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {

            
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Woodland Wedding 2026 - kimashi == Kim, Matthias and Yoshi");
            egui_extras::install_image_loaders(ctx);
            let collage = egui::include_image!("../assets/Collage_Verotterung_Zuschnitt2.jpg");
            let desired_size = egui::vec2(340.0, 340.0); 
            let collage2 = egui::include_image!("../assets/Collage_Verotterung_Zuschnitt2.jpg");
            let desired_size2 = egui::vec2(340.0, 340.0); 
            let collage3 = egui::include_image!("../assets/Collage_Verotterung_Zuschnitt2.jpg");
            let desired_size3 = egui::vec2(340.0, 340.0); 
            let collage4 = egui::include_image!("../assets/Collage_Verotterung_Zuschnitt2.jpg");
            let desired_size4 = egui::vec2(340.0, 340.0); 
            ui.add(egui::Image::new(collage).fit_to_exact_size(desired_size));
            ui.add(egui::Image::new(collage2).fit_to_exact_size(desired_size2));
            ui.add(egui::Image::new(collage3).fit_to_exact_size(desired_size3));
            ui.add(egui::Image::new(collage4).fit_to_exact_size(desired_size4));
            
            egui::Window::new("about the location").open(&mut self.flag1).show(ctx, |ui| {
                ui.label("tents and bungalos, amenities, getting there, ..");                
            });


            egui::Window::new("Painting")
                .open(&mut self.flag5)
                .show(ctx, |ui| {

                if ui.button("Send Painting").clicked() {
                    self.saved_image_data = self.painting_app.export_json(ctx).clone();

                    if let Some(image_data) = &self.saved_image_data {
                            let request1 = ehttp::Request::post("https://ntfy.sh/woodland", format!(r#"{}"#, "new image received").as_bytes().to_vec());                        
                            ehttp::fetch(request1, move |result: ehttp::Result<ehttp::Response>| {println!("Status code: {:?}", result.unwrap().status);});

                            use ehttp::Request;
                            let request = Request {
                                headers: ehttp::Headers::new(&[
                                    ("Content-Type", "application/json"),
                                    ("X-Access-Key", "$2a$10$fgUGfRK3yfJUxFr4TJXSIOJYfpsU2zRnWs6jxmq4wE20oqYU5xHIW"),
                                    ("X-Collection-Id", "68b546c943b1c97be932c82e"),
                                    ("X-Bin-Private", "true"),
                                ]),
                                ..Request::post("https://api.jsonbin.io/v3/b", image_data.as_bytes().to_vec())
                            };
                            ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {println!("Status code: {:?}", result.unwrap().status);});




                        
                        }

                    
                }







                    
                    self.painting_app.ui(ui);
                });





            
            egui::Window::new("snake game").open(&mut self.flag6).show(ctx, |ui| {
                if ui.input(|i| i.key_pressed(egui::Key::R)) {
                self.snake.reset();
            }

            self.snake.ui(ui, dt);

            ui.horizontal(|ui| {
            //if ui.button("⬅️").clicked() {
            //    self.snake.next_dir = Direction::Left;
            //}
            if ui.add(egui::Button::new("⬅️").min_size(egui::vec2(40.0, 40.0))).clicked() {
                self.snake.next_dir = Direction::Left;
            }
            if ui.button("⬆️").clicked() {
                self.snake.next_dir = Direction::Up;
            }
            if ui.button("⬇️").clicked() {
                self.snake.next_dir = Direction::Down;
            }
            if ui.button("➡️").clicked() {
                self.snake.next_dir = Direction::Right;
            }
            if ui.button("🔁 Restart").clicked() {
                self.snake.reset(); // your existing restart logic
            }
        });
            });

            egui::Window::new("fractal clock").open(&mut self.flag7).show(ctx, |ui| {
                ui.label("Fractal Clock Example");
            ui.separator();

            let desired_size = egui::vec2(400.0, 400.0);
            self.fractal_clock.ui(ui);
            
            let available_size = ui.available_size();
            //let (rect, _response) = ui.allocate_exact_size(available_size, egui::Sense::hover());
            let (rect, _response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());
            let painter = ui.painter_at(rect);
    
            let time = ctx.input(|i| i.time); // egui's built-in time
            self.fractal_clock.paint(&painter, rect, time);

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
           });
        });
        ctx.request_repaint();
        
        }

        
    }
}

