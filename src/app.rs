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
    flag8: bool,
    flag9: bool,
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
    button8: String,
    button9: String,
    #[serde(skip)]
    fractal_clock: FractalClock,
    snake: SnakeGame,
    last_time: f64,
    #[serde(skip)]
    painting_app: PaintingApp,
    saved_image_data: Option<String>,

    formflag: bool,
    formbutton: String,

    rsvp1: String,
    rsvp2: String,
    rsvp3: String,

    submitflag: bool,
    schickflag: bool,
    

    

    
    
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
            flag8: false,
            flag9: false,
            user_input: "type message..".to_owned(),
            password_flag: true,
            password_string: "type password".to_owned(),
            language_flag: true,  
            button1: "button1".to_owned(),
            button2: "button2".to_owned(),
            button3: "button3".to_owned(),
            button4: "button4".to_owned(),
            button5: "button5".to_owned(),
            button6: "button6".to_owned(),
            button7: "button7".to_owned(),
            button8: "button8".to_owned(),
            button9: "button9".to_owned(),
            fractal_clock: FractalClock::default(),
            snake: SnakeGame::default(),
            last_time: 0.0,

            painting_app: PaintingApp::default(),
            saved_image_data: None,


            formflag: false,
            formbutton: "form".to_owned(),

            rsvp1: "".to_owned(),
            rsvp2: "".to_owned(),
            rsvp3: "".to_owned(),

            submitflag: false,
            schickflag: false,
            

            


            
            
            
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
        

        if self.language_flag == true {self.button1 = "getting there".to_owned()}
        else {self.button1 = "Anfahrt".to_owned()}
        if self.language_flag == true {self.button2 = "schedule".to_owned()}
        else {self.button2 = "Ablauf".to_owned()}
        if self.language_flag == true {self.button3 = "presents".to_owned()}
        else {self.button3 = "Geschenke".to_owned()}
        if self.language_flag == true {self.button4 = "help wanted".to_owned()}
        else {self.button4 = "helfende Hände".to_owned()}

        if self.language_flag == true {self.button5 = "painting".to_owned()}
        else {self.button5 = "malen".to_owned()}
        if self.language_flag == true {self.button6 = "play snake".to_owned()}
        else {self.button6 = "Snake spielen".to_owned()}
        if self.language_flag == true {self.button7 = "fractal clock".to_owned()}
        else {self.button7 = "Fraktaluhr".to_owned()}

        if self.language_flag == true {self.button8 = "dress code".to_owned()}
        else {self.button8 = "Dresscode".to_owned()}
        if self.language_flag == true {self.button9 = "contact".to_owned()}
        else {self.button9 = "Kontakt".to_owned()}

        if self.language_flag == true {self.formbutton = "RSVP".to_owned()}
        else {self.formbutton = "Rückmeldung".to_owned()}
        

        if self.password_flag == true {
            egui::Window::new("choose language / Sprache wählen").show(ctx, |ui| {
                //ui.text_edit_singleline(&mut self.password_string); 
                if ui.button("English").clicked() {
                        self.language_flag = true;
                        self.password_flag = false}
                if ui.button("Deutsch").clicked() {
                        self.language_flag = false;
                        self.password_flag = false}
            });
                       
        
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
                if ui.button("Deutsch <--> English").clicked() {
                if self.language_flag == true {self.language_flag = false;}
                else {self.language_flag = true;}
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
            if ui.button(self.button8.clone()).clicked() {
                if self.flag8 == true {self.flag8 = false;}
                else {self.flag8 = true;}
            }  
            if ui.button(self.button9.clone()).clicked() {
                if self.flag9 == true {self.flag9 = false;}
                else {self.flag9 = true;}
            }  







            
        });


        





            
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {


             

            
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Woodland Wedding 2026 - kimashi == Kim, Matthias and Yoshi");

            if ui.button(self.formbutton.clone()).clicked() {
                if self.formflag == true {self.formflag = false;}
                else {self.formflag = true;}
            }
                
            egui_extras::install_image_loaders(ctx);
            let collage = egui::include_image!("../assets/Collage_Verotterung_Zuschnitt2.jpg");
            let desired_size = egui::vec2(340.0, 340.0); 
            
            ui.add(egui::Image::new(collage).fit_to_exact_size(desired_size));
            
            
            egui::Window::new(self.button1.clone()).open(&mut self.flag1).show(ctx, |ui| {

                if self.language_flag == true {ui.label("You can reach the venue in all kinds of ways: by public transport, your own or a rented car, or by taxi/Uber. In terms of both cost and travel time, the options are closer than you might think. Here's an overview: \n A bit complicated with public transport: \n Around 2.5 hours from Berlin Ostkreuz via regional train and two buses to Chossewitz, then a 20-minute walk (1.5 km). \n Cost: approx. 15 euros per person. \n\n TAXI or UBER: \n If you book a large taxi or Uber that fits 6–8 people, the cost is about 25–30 euros per person. \n Duration: approx. 1.5 hours. \n\n CAR: \n Duration: approx. 1.5 hours, with parking available directly at the venue. Renting a car for 3 days for 4–5 people leads to a cost of about 50 euros per person for the round trip, including fuel. \n Important: mind the exact route:"); ui.hyperlink_to("ROUTE","https://maps.app.goo.gl/WtrCENyrHQ23ziUL9"); ui.label(" \n\n HELICOPTER: \n kind of unaffordable. However, here the link to the classic helicopter game:"); 
                                               ui.hyperlink_to("helicopter game", "https://www.addictinggames.com/clicker/helicopter-game");}
                else {ui.label("Zum Gelände kommt man auf allen erdenklichen Wegen, mit Bus&Bahn, dem eigenen oder gemieteten Auto oder mit Taxi bzw. Uber. Preislich und zeitlich liegen die Möglichkeiten näher zusammen als man denken würde. Hier ein Überblick: \n\netwas umständlich mit Öffis:\nUngefähr 2.5 Stunden von Berlin Ostkreuz mit Regio und zwei Bussen bis Chossewitz, von dort 20 Minuten (1.5km) laufen. Kosten circa 15 Euro pro Person.\n\nTAXI oder UBER:\nWenn man ein Großraumtaxi bzw GroßraumUber bestellt was 6-8 Leute transportiert kommt man auf Kosten von circa 25-30 Euro pro Person. Dauer circa 1.5 Stunden.\n\nAUTO:\nDauer circa 1.5 Stunden, Parkplätze direkt vor dem Gelände. Ein Auto mieten für 3 Tage und 4-5 Leute führt zu Kosten von circa 50 Euro für hin&zurück, inklusive Benzin.\n--> exakte Route, wichtig!!"); ui.hyperlink_to("ROUTE",  "https://maps.app.goo.gl/WtrCENyrHQ23ziUL9"); ui.label(" <--\n\nHELIKOPTER: fast unbezahlbar teuer, dafür hier der Link zum Game:"); 
                      ui.hyperlink_to("Helikopterspiel", "https://www.addictinggames.com/clicker/helicopter-game");}
                //ui.label("tents and bungalos, amenities, getting there, ..");



                
            });

            egui::Window::new(self.formbutton.clone()).open(&mut self.formflag).show(ctx, |ui| {

                if self.language_flag == true {
                    ui.horizontal(|ui| {
                        ui.label("names: ");
                        ui.text_edit_singleline(&mut self.rsvp1);
                    });
                    ui.horizontal(|ui| {
                        ui.label("email-addresses: ");
                        ui.text_edit_singleline(&mut self.rsvp2);
                    });
                    ui.horizontal(|ui| {
                        ui.label("like-to-hear song(s): ");
                        ui.text_edit_singleline(&mut self.rsvp3);
                    });
                    
                    if self.submitflag == false {ui.label("your entered data:")}
                    else {ui.label("your submitted date")}
                    
                    ui.horizontal(|ui| {
                        ui.label("names: ");
                        ui.label(self.rsvp1.clone());
                    });
                    ui.horizontal(|ui| {
                        ui.label("email-addresses: ");
                        ui.label(self.rsvp2.clone());
                    });
                    ui.horizontal(|ui| {
                        ui.label("songs: ");
                        ui.label(self.rsvp3.clone());
                    });  

                    if ui.button("submit").clicked() {
                        let message = format!(r#"names: {} ; emails: {} ; songs: {}"#, self.rsvp1.clone(), self.rsvp2, self.rsvp3);
                        let body1 = message.as_bytes().to_vec();
                        let request1 = ehttp::Request::post("https://ntfy.sh/woodland", body1);
                        ehttp::fetch(request1, move |result: ehttp::Result<ehttp::Response>| {println!("Status code: {:?}", result.unwrap().status);});
                        self.submitflag = true;
                    }
                    
                }
                else {
                    ui.horizontal(|ui| {
                        ui.label("Namen: ");
                        ui.text_edit_singleline(&mut self.rsvp1);
                    });
                    ui.horizontal(|ui| {
                        ui.label("email-Adressen: ");
                        ui.text_edit_singleline(&mut self.rsvp2);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Wunschlied(er): ");
                        ui.text_edit_singleline(&mut self.rsvp3);
                    });
                    
                    if self.schickflag == false {ui.label("deine eingegebenen Daten:")}
                    else {ui.label("deine abgeschickten Daten")}
                    
                    ui.horizontal(|ui| {
                        ui.label("Namen: ");
                        ui.label(self.rsvp1.clone());
                    });
                    ui.horizontal(|ui| {
                        ui.label("email-Adresses: ");
                        ui.label(self.rsvp2.clone());
                    });
                    ui.horizontal(|ui| {
                        ui.label("Wunschlied(er): ");
                        ui.label(self.rsvp3.clone());
                    }); 
                    if ui.button("abschicken").clicked() {
                        let message = format!(r#"names: {} ; emails: {} ; songs: {}"#, self.rsvp1.clone(), self.rsvp2, self.rsvp3);
                        let body1 = message.as_bytes().to_vec();
                        let request1 = ehttp::Request::post("https://ntfy.sh/woodland", body1);
                        ehttp::fetch(request1, move |result: ehttp::Result<ehttp::Response>| {println!("Status code: {:?}", result.unwrap().status);});
                        self.schickflag = true;
                    }
                }

                



                
                
                
                
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
            ui.add_space(175.0);
            if ui.add(egui::Button::new("⬆️").min_size(egui::vec2(50.0, 50.0))).clicked() {
                self.snake.try_change_dir(Direction::Up);
            }
            });
            ui.add_space(8.0);
            ui.horizontal(|ui| {
            ui.add_space(115.0);
            if ui.add(egui::Button::new("⬅️").min_size(egui::vec2(50.0, 50.0))).clicked() {
                self.snake.try_change_dir(Direction::Left);
            }
            ui.add_space(10.0);
            if ui.button("🔁 R").clicked() {
                self.snake.reset(); // your existing restart logic
            }
            ui.add_space(10.0);
            if ui.add(egui::Button::new("➡️").min_size(egui::vec2(50.0, 50.0))).clicked() {
                self.snake.try_change_dir(Direction::Right);
            }    
            });
            ui.add_space(8.0);
            ui.horizontal(|ui| {
            ui.add_space(175.0);
            if ui.add(egui::Button::new("⬇️").min_size(egui::vec2(50.0, 50.0))).clicked() {
                self.snake.try_change_dir(Direction::Down);
            }
            });





                
            

                
            

                
           

                
            
           

                
            
            });

            egui::Window::new("Fractal Clock")
                .open(&mut self.flag7)
                .resizable(true)
                .default_width(400.0)
                .default_height(450.0)
                .show(ctx, |ui| {
                    ui.label("Fractal Clock Example");
                    ui.separator();
            
                    self.fractal_clock.ui(ui);
            
                    ui.add_space(10.0);
            
                    // Use available space BEFORE wrapping to avoid height issues
                    let avail = ui.available_size_before_wrap();
                    let side = avail.x.min(avail.y); // keep it square
            
                    // Horizontally center the canvas
                    let horiz_margin = (avail.x - side).max(0.0) / 2.0;
                    ui.add_space(horiz_margin);
            
                    let square_size = egui::vec2(side, side);
                    let (rect, _response) = ui.allocate_exact_size(square_size, egui::Sense::hover());
                    let painter = ui.painter_at(rect);
            
                    let time = ctx.input(|i| i.time);
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

