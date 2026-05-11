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
    user_input_en: String,
    user_input_de: String,
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
    rsvp4: String,

    rsvp5: String,
    rsvp6: String,

    submitflag: bool,
    schickflag: bool,

    pic_name_en: String,
    pic_name_de: String,

    submit_names: String,
    submit_emails: String,
    submit_songs: String,
    submit_comments: String,

    rsvp_flag1: bool,
    rsvp_flag2: bool,

    decline_names: String,
    decline_comments: String,

    pic_value: i32,

	new_formflag: bool,
	
	rsvp01: String,
    rsvp02: String,
    rsvp03: String,
    rsvp04: String,
    rsvp05: String,
    rsvp06: String,

	helper_names: String,
    helper_task: String,
    helper_with: String,
    helper_no: String,
	helper_fri: String,
    helper_sun: String,


    

    

    
    
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
            user_input_en: "type message..".to_owned(),
            user_input_de: "Nachricht schreiben..".to_owned(),
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
            rsvp4: "".to_owned(),

            rsvp5: "".to_owned(),
            rsvp6: "".to_owned(),

            submitflag: false,
            schickflag: false,
            
            pic_name_en: "who is painting? type your name ..".to_owned(),
            pic_name_de: "wer malt? lass uns deinen Namen wissen ..".to_owned(),

            submit_names: "".to_owned(),
            submit_emails: "".to_owned(),
            submit_songs: "".to_owned(),
            submit_comments: "".to_owned(),
            
            rsvp_flag1: false,
            rsvp_flag2: false,

            decline_names: "".to_owned(),
            decline_comments: "".to_owned(),

            pic_value: 0,

			new_formflag: false,

			rsvp01: "".to_owned(),
            rsvp02: "".to_owned(),
            rsvp03: "".to_owned(),
            rsvp04: "".to_owned(),

            rsvp05: "".to_owned(),
            rsvp06: "".to_owned(),
			helper_names: "".to_owned(),
		    helper_task: "".to_owned(),
		    helper_with: "".to_owned(),
		    helper_no: "".to_owned(),
			helper_fri: "".to_owned(),
		    helper_sun: "".to_owned(),



            
            
            
        }
    }
}


impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        
        
        
        cc.egui_ctx.set_visuals(egui::Visuals::dark());    // force dark visuals
        cc.egui_ctx.request_repaint();  
        
        
        // Try to restore previous state first
        if let Some(storage) = cc.storage {
            if let Some(loaded) = eframe::get_value::<Self>(storage, eframe::APP_KEY) {
                //cc.egui_ctx.set_visuals(egui::Visuals::dark());
                //cc.egui_ctx.request_repaint();
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

        if self.language_flag == true {self.button7 = "fractal clock".to_owned()}
        else {self.button7 = "Fraktaluhr".to_owned()}
        
        if self.language_flag == true {self.button2 = "schedule".to_owned()}
        else {self.button2 = "Ablauf".to_owned()}

        if self.language_flag == true {self.button5 = "painting".to_owned()}
        else {self.button5 = "Bild malen".to_owned()}

        if self.language_flag == true {self.button4 = "help wanted".to_owned()}
        else {self.button4 = "helfende Hände".to_owned()}
        
        if self.language_flag == true {self.button3 = "presents".to_owned()}
        else {self.button3 = "Geschenke".to_owned()}
        
        if self.language_flag == true {self.button6 = "play snake".to_owned()}
        else {self.button6 = "Snake spielen".to_owned()}
        
        if self.language_flag == true {self.button8 = "what to bring".to_owned()}
        else {self.button8 = "bitte mitbringen".to_owned()}
        
        if self.language_flag == true {self.button9 = "contact".to_owned()}
        else {self.button9 = "Kontakt".to_owned()}

        if self.language_flag == true {self.formbutton = "RSVP".to_owned()}
        else {self.formbutton = "Rückmelden".to_owned()}
        

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
                //egui::widgets::global_theme_preference_buttons(ui);

                //ui.horizontal(|ui| {
                //    if ui.button("☀ Light").clicked() {
                //        ui.ctx().set_visuals(egui::Visuals::light());
                //    }
                //    if ui.button("🌙 Dark").clicked() {
                //        ui.ctx().set_visuals(egui::Visuals::dark());
                //    }
                //});

                
                if self.language_flag == true {
                    
                    ui.horizontal(|ui| {
                // Access the current visuals
                let visuals = ui.ctx().style().visuals.clone();
            
                // Pick label and next theme based on current mode
                let (label, new_visuals) = if visuals.dark_mode {
                    ("☀ switch to light theme ☀", egui::Visuals::light())
                } else {
                    ("🌙 switch to dark theme 🌙", egui::Visuals::dark())
                };
            
                // Draw one toggle button
                if ui.button(label).clicked() {
                    ui.ctx().set_visuals(new_visuals);
                }
            }); }
                else {
                ui.horizontal(|ui| {
                    // Access the current visuals
                let visuals = ui.ctx().style().visuals.clone();
            
                // Pick label and next theme based on current mode
                let (label, new_visuals) = if visuals.dark_mode {
                    ("☀ zum Hellmodus wechseln ☀", egui::Visuals::light())
                } else {
                    ("🌙 zum Dunkelmodus wechseln 🌙", egui::Visuals::dark())
                };
            
                // Draw one toggle button
                if ui.button(label).clicked() {
                    ui.ctx().set_visuals(new_visuals);
                }
            });
                }
                
                
                

                
                if ui.button("Deutsch <--> English").clicked() {
                if self.language_flag == true {self.language_flag = false;}
                else {self.language_flag = true;}
            }
                
                
            });});

        
        egui::SidePanel::left("bullet points").show(ctx, |ui| {
            //ui.label(format!("You typed: {}", self.user_input));
            if ui.button(egui::RichText::new(self.button1.clone()).size(17.0).color(egui::Color32::from_rgb(0, 183, 255)).strong()).clicked() {
                if self.flag1 == true {self.flag1 = false;}
                else {self.flag1 = true;}
            }
            if ui.button(egui::RichText::new(self.button7.clone()).size(17.0).color(egui::Color32::from_rgb(251, 0, 255)).strong()).clicked() {
                if self.flag7 == true {self.flag7 = false;}
                else {self.flag7 = true;}
            }
            if ui.button(egui::RichText::new(self.button2.clone()).size(17.0).color(egui::Color32::from_rgb(0, 183, 255)).strong()).clicked() {
                if self.flag2 == true {self.flag2 = false;}
                else {self.flag2 = true;}
            }
            if ui.button(egui::RichText::new(self.button5.clone()).size(17.0).color(egui::Color32::from_rgb(251, 0, 255)).strong()).clicked() {
                if self.flag5 == true {self.flag5 = false;}
                else {self.flag5 = true;}
            }  
            
			//if ui.button(egui::RichText::new(self.button4.clone()).size(17.0).color(egui::Color32::from_rgb(0, 183, 255)).strong()).clicked() {
            //   if self.flag4 == true {self.flag4 = false;}
            //   else {self.flag4 = true;}
            //}  

			if cyber_button(ui, &self.button4, 17.0).clicked() {
                if self.flag4 == true {self.flag4 = false;}
                else {self.flag4 = true;}
            }  


			
            if ui.button(egui::RichText::new(self.button3.clone()).size(17.0).color(egui::Color32::from_rgb(0, 183, 255)).strong()).clicked() {
                if self.flag3 == true {self.flag3 = false;}
                else {self.flag3 = true;}
            }  
            if ui.button(egui::RichText::new(self.button6.clone()).size(17.0).color(egui::Color32::from_rgb(251, 0, 255)).strong()).clicked() {
                if self.flag6 == true {self.flag6 = false;}
                else {self.flag6 = true;}
            } 
            
			
			//if ui.button(egui::RichText::new(self.button8.clone()).size(17.0).color(egui::Color32::from_rgb(0, 183, 255)).strong()).clicked() {
            //    if self.flag8 == true {self.flag8 = false;}
            //    else {self.flag8 = true;}
            //}  

			if cyber_button(ui, &self.button8, 17.0).clicked() {
                if self.flag8 == true {self.flag8 = false;}
                else {self.flag8 = true;}
            } 

			
            if ui.button(egui::RichText::new(self.button9.clone()).size(17.0).color(egui::Color32::from_rgb(0, 183, 255)).strong()).clicked() {
                if self.flag9 == true {self.flag9 = false;}
                else {self.flag9 = true;}
            }  
            //ui.label("");
            //if ui.button(egui::RichText::new(self.formbutton.clone()).size(17.0).color(egui::Color32::from_rgb(206,101,43)).strong()).clicked() {
            //    if self.formflag == true {self.formflag = false;}
            //    else {self.formflag = true;}
            //}
            







            
        });


        





            
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                ui.vertical_centered_justified(|ui| {


             

            
            // The central panel the region left after adding TopPanel's and SidePanel's
            //ui.heading("Woodland Wedding 2026 - kimashi == Kim, Matthias and Yoshi");
            
            //ui.heading(egui::RichText::new("Woodland Wedding 2026 \nKIMASHI = Kim, Matthias and Yoshi\n").size(34.0).color(egui::Color32::DARK_GREEN).strong());
            cyber_rainbow_text(ui, "Woodland Wedding 2026 \nKIMASHI = Kim, Matthias and Yoshi\n", 34.0);
			
			ui.label("");
                
            egui_extras::install_image_loaders(ctx);
            let collage = egui::include_image!("../assets/Collage_Verotterung_Zuschnitt2.jpg");
            let desired_size = egui::vec2(340.0, 340.0); 
            
            ui.add(egui::Image::new(collage).fit_to_exact_size(desired_size));

            if self.language_flag == true {
                
                ui.label(egui::RichText::new("\nDear almost-party guests,\n").size(26.0).color(egui::Color32::DARK_GREEN).strong()); 

				ui.label(egui::RichText::new("we can hardly believe it — the big weekend is almost here! Here’s the final round of really important information:").size(17.0)); 
				
				ui.label(egui::RichText::new("\nHelping Hands Schedule\n").size(26.0).color(egui::Color32::DARK_GREEN).strong()); 
				
				ui.label(egui::RichText::new("please get back to us by May 24th!").size(26.0).color(egui::Color32::RED).strong()); 
				
				ui.label(egui::RichText::new("To help us coordinate all our lovely helpers, we already need your support now. Please get back to us by May 24th and let us know which tasks you’d enjoy helping with and when you’ll be available. We’ll organize everything and do our best to create a plan that works well for everyone.").size(17.0)); 
				
				ui.label(egui::RichText::new("If we don’t hear from you, we’ll assume you’re happy to help wherever needed and that you’ll be around until late Sunday afternoon.").size(17.0)); 
				
				if ui.button(egui::RichText::new("click me to go to the reply form").size(24.0).color(egui::Color32::RED).strong()).clicked() {
						if self.flag4 == true {self.flag4 = false;}
						else {self.flag4 = true;}
					} 
				
				ui.label(egui::RichText::new("\nGoogle Doc/Sheet for Buffet Contributions and Carpooling:\n").size(26.0).color(egui::Color32::DARK_GREEN).strong()); 
				
				ui.label(egui::RichText::new("Please add what you’d like to bring for the Friday buffet: ideally finger food that can easily be eaten by hand — sweet or savory are both very welcome.").size(17.0));
				
				ui.label(egui::RichText::new("(no stress at all if you can’t bring anything because of travel or time constraints)").size(17.0));
				
				ui.add(
				    egui::Hyperlink::from_label_and_url(
				        egui::RichText::new("click me - Potluck List - click me").size(24.0).color(egui::Color32::from_rgb(251, 0, 255)),
				        "https://docs.google.com/document/d/1oSsfuwkMAKTApy1AX0LG_zBfuD6Ym0_B82CBWN-xvKE/edit?usp=sharing",
				    )
				);
				
				ui.label(egui::RichText::new("Please add your name if you can offer seats in your car for the trip — and make sure to include your phone number!").size(17.0)); 
				
				ui.add(
				    egui::Hyperlink::from_label_and_url(
				        egui::RichText::new("click me - Carpool Table - click me").size(24.0).color(egui::Color32::from_rgb(251, 0, 255)),
				        "https://docs.google.com/spreadsheets/d/1vfl5wpG0HaQNEFuzkOHtKUigDhtELqi9hJA-zkjgttQ/edit?usp=sharing",
				    )
				);
				
				ui.label(egui::RichText::new("\nPacking List:\n").size(26.0).color(egui::Color32::DARK_GREEN).strong()); 
				
				ui.label(egui::RichText::new("Here are a few things we’d recommend bringing along to make the weekend extra cozy and comfortable:\n(Bed linen and towels are provided in the bungalows.)\n\n- Camping chair\n- Picnic blanket\n- Seat cushion for the beer benches\n- A cozy blanket for sitting outside in the evening\n- Swimwear\n- Flip-flops or sandals\n- Sauna towel\n- Sun protection\n- Mosquito repellent\n- Rain gear — just in case\n- Layers for cooler evenings (\"onion style\" clothing works best!)").size(17.0)); 
				
				ui.label(egui::RichText::new("\nAlso let us know if you’d still like to sign up for the shuttle from Beeskow to the venue.").size(17.0)); 
				
				ui.label(egui::RichText::new("\nWe truly can’t wait to celebrate with you all!").size(17.0)); 
				
				ui.label(egui::RichText::new("See you very soon\nKIMASHI\n\n").size(26.0).color(egui::Color32::DARK_GREEN).strong());

            
            }
            else {

				ui.label(egui::RichText::new("\nLiebe fast-schon-Partygäste,\n").size(26.0).color(egui::Color32::DARK_GREEN).strong()); 

				ui.label(egui::RichText::new("wir können es kaum glauben, es ist schon fast so weit! Hier noch die letzten wirklich wichtigen Infos:").size(17.0)); 
				
				ui.label(egui::RichText::new("\nMithelf-Plan\n").size(26.0).color(egui::Color32::DARK_GREEN).strong()); 
				
				ui.label(egui::RichText::new("bitte bis zum 24.05. zurückmelden!").size(26.0).color(egui::Color32::RED).strong()); 
				
				ui.label(egui::RichText::new("Damit wir alle Helferlis gut einteilen können, brauchen wir schon jetzt eure Hilfe. Bitte meldet euch bis zum 24.05. zurück und gebt an, welche Aufgaben ihr gern übernehmen möchtet und wann ihr dafür Zeit habt. Wir teilen euch dann ein und versuchen, den Plan für alle passend zu gestalten.").size(17.0)); 
				
				ui.label(egui::RichText::new("Sollten wir von euch nichts hören, gehen wir davon aus, dass ihr auf alle Aufgabenbereiche Lust habt und am Sonntag bis zum späten Nachmittag am Start seid.").size(17.0)); 
				
				if ui.button(egui::RichText::new("klick mich um zum Antwortformular zu kommen").size(24.0).color(egui::Color32::RED).strong()).clicked() {
               		if self.flag4 == true {self.flag4 = false;}
               		else {self.flag4 = true;}
        			} 
				
				ui.label(egui::RichText::new("\nGoogle Doc/Tabelle für Mitbring-Buffet und Mitfahrzentrale:\n").size(26.0).color(egui::Color32::DARK_GREEN).strong()); 
				
				ui.label(egui::RichText::new("Bitte tragt ein, was ihr für das Buffet am Freitag mitbringt: bitte Fingerfood, das entspannt aus der Hand gegessen werden kann - gerne süß oder salzig.").size(17.0));
				
				ui.label(egui::RichText::new("(kein Stress falls ihr wegen Zeitmangel oder langer Fahrt nichts beitragen könnt)").size(17.0));
				
				ui.add(
				    egui::Hyperlink::from_label_and_url(
				        egui::RichText::new("klick mich - Buffetliste - klick mich").size(24.0).color(egui::Color32::from_rgb(251, 0, 255)),
				        "https://docs.google.com/document/d/1oSsfuwkMAKTApy1AX0LG_zBfuD6Ym0_B82CBWN-xvKE/edit?usp=sharing",
				    )
				);
				
				ui.label(egui::RichText::new("Bitte tragt ein, falls ihr für die Anfahrt noch Plätze im Auto anbieten könnt und gebt unbedingt eure Telefonnummer mit an!").size(17.0)); 
				
				ui.add(
				    egui::Hyperlink::from_label_and_url(
				        egui::RichText::new("klick mich - Mitfahrtabelle - klick mich").size(24.0).color(egui::Color32::from_rgb(251, 0, 255)),
				        "https://docs.google.com/spreadsheets/d/1vfl5wpG0HaQNEFuzkOHtKUigDhtELqi9hJA-zkjgttQ/edit?usp=sharing",
				    )
				);
				
				ui.label(egui::RichText::new("\nMitbringliste:\n").size(26.0).color(egui::Color32::DARK_GREEN).strong()); 
				
				ui.label(egui::RichText::new("Hier unsere Empfehlung, was ihr mitbringen könnt, um das Wochenende gemütlich zu verbringen:\n(In den Bungalows sind Handtücher und Bettwäsche vorhanden)\n\n- Campingstuhl\n- Picknickdecke\n- Sitzkissen für die Bierbank\n- Kuscheldecke zum abends draußen sitzen\n- Badesachen\n- Badelatschen\n- Sauna-Handtuch\n- Sonnenschutz\n- Mückenzeug\n- Regenkleidung: man weiß ja nie\n- Zwiebelschalen: falls es abends kalt wird").size(17.0)); 
				
				ui.label(egui::RichText::new("\nMeldet euch außerdem bei uns, falls ihr euch noch für den Shuttle von Beeskow zum Gelände anmelden wollt.").size(17.0)); 
				
				ui.label(egui::RichText::new("\nWir freuen uns schon unbändig auf euch!").size(17.0)); 
				
				ui.label(egui::RichText::new("Bis sehr bald\nKIMASHI\n\n").size(26.0).color(egui::Color32::DARK_GREEN).strong()); 
                
                
                            
            }
            //if ui.button(egui::RichText::new(self.formbutton.clone()).size(17.0).color(egui::Color32::from_rgb(206,101,43)).strong()).clicked() {
            //        if self.formflag == true {self.formflag = false;}
            //        else {self.formflag = true;}
            //    }
            //ui.label("");
            
            egui::Window::new(self.button1.clone()).open(&mut self.flag1).show(ctx, |ui| {
                egui::ScrollArea::both().show(ui, |ui| {

                if self.language_flag == true {

ui.label(egui::RichText::new("You can reach the venue in a number of different ways: using public transport, your own car, a rental car or by taxi/Uber. Here's an overview:\n").size(17.0)); 

ui.label(egui::RichText::new("possibly CARPOOLING:").size(23.0).color(egui::Color32::DARK_GREEN).strong().underline()); 
ui.label("");
ui.label(egui::RichText::new("Please add your name if you can offer seats in your car for the trip — and make sure to include your phone number!").size(17.0)); 
ui.add(
    egui::Hyperlink::from_label_and_url(
        egui::RichText::new("click me - Carpool Table - click me").size(24.0).color(egui::Color32::from_rgb(251, 0, 255)),
        "https://docs.google.com/spreadsheets/d/1vfl5wpG0HaQNEFuzkOHtKUigDhtELqi9hJA-zkjgttQ/edit?usp=sharing",
    )
);
ui.label("");
					
ui.label(egui::RichText::new("PUBLIC TRANSPORT might be a bit complicated:").size(23.0).color(egui::Color32::DARK_GREEN).strong().underline()); 
ui.label("");
ui.label(egui::RichText::new("Around 2.5 hours from Berlin Ostkreuz via regional train and two buses to Chossewitz, then a 20-minute walk (1.5 km).\nCost: approx. 15 euros per person.\n").size(17.0)); 

ui.label(egui::RichText::new("TAXI or UBER:").size(23.0).color(egui::Color32::DARK_GREEN).strong().underline()); 
ui.label("");
ui.label(egui::RichText::new("If you book a large taxi or Uber that accomodates 6–8 people, the cost is about  40 euros per person one way.Duration: approx. 1.5 hours. If you’re interested in this option and need help organizing it, please contact us before April 1st.\n").size(17.0)); 

ui.label(egui::RichText::new("CAR:").size(23.0).color(egui::Color32::DARK_GREEN).strong().underline()); 
ui.label("");
ui.label(egui::RichText::new("Duration: approx. 1.5 hours, with parking available directly at the venue.\nRenting a car for 3 days for 4–5 people costs about 50 euros per person for the round trip, including gas.").size(17.0));                     
ui.horizontal_wrapped(|ui| {
ui.label(egui::RichText::new("Important: mind the exact ").size(17.0)); 
ui.add(
    egui::Hyperlink::from_label_and_url(
        egui::RichText::new("ROUTE").size(17.0).color(egui::Color32::from_rgb(251, 0, 255)),
        "https://maps.app.goo.gl/WtrCENyrHQ23ziUL9",
    )
);
});


ui.label(egui::RichText::new("\nHELICOPTER:").size(23.0).color(egui::Color32::DARK_GREEN).strong().underline());
ui.label("");
ui.horizontal_wrapped(|ui| {
ui.label(egui::RichText::new("Sadly unaffordable. However, here is the link to the classic").size(17.0)); 
ui.add(
    egui::Hyperlink::from_label_and_url(
        egui::RichText::new("helicopter game").size(17.0).color(egui::Color32::from_rgb(251, 0, 255)),
        "https://www.addictinggames.com/clicker/helicopter-game",
    )
);
});


}
                




else {

ui.label(egui::RichText::new("Zum Gelände kommt man auf allen erdenklichen Wegen, mit Bus&Bahn, dem eigenen oder gemieteten Auto oder mit Taxi bzw. Uber. Hier ein Überblick:\n").size(17.0)); 

ui.label(egui::RichText::new("mögliche Mitfahrgelegenheiten:").size(23.0).color(egui::Color32::DARK_GREEN).strong().underline()); 
ui.label("");
ui.label(egui::RichText::new("Bitte tragt ein, falls ihr für die Anfahrt noch Plätze im Auto anbieten könnt und gebt unbedingt eure Telefonnummer mit an!").size(17.0)); 
ui.add(
    egui::Hyperlink::from_label_and_url(
        egui::RichText::new("klick mich - Mitfahrtabelle - klick mich").size(24.0).color(egui::Color32::from_rgb(251, 0, 255)),
        "https://docs.google.com/spreadsheets/d/1vfl5wpG0HaQNEFuzkOHtKUigDhtELqi9hJA-zkjgttQ/edit?usp=sharing",
    )
);
ui.label("");
	
ui.label(egui::RichText::new("etwas umständlich mit ÖFFIS:").size(23.0).color(egui::Color32::DARK_GREEN).strong().underline()); 
ui.label("");
ui.label(egui::RichText::new("Ungefähr 2.5 Stunden von Berlin Ostkreuz mit Regio und zwei Bussen bis Chossewitz, von dort 20 Minuten (1.5km) laufen. Kosten circa 15 Euro pro Person.\n").size(17.0)); 

ui.label(egui::RichText::new("TAXI oder UBER:").size(23.0).color(egui::Color32::DARK_GREEN).strong().underline()); 
ui.label("");
ui.label(egui::RichText::new("Ein Großraumtaxi/ -Uber für circa 6-8 Leute kostet circa  40 Euro pro Person und Fahrt. Dauer circa 1.5 Stunden. Falls ihr Interesse an dieser Option habt und Hilfe beim Organisieren braucht, wendet euch bitte bis zum 01.04.26 an uns.\n").size(17.0)); 

ui.label(egui::RichText::new("AUTO:").size(23.0).color(egui::Color32::DARK_GREEN).strong().underline()); 
ui.label("");
ui.label(egui::RichText::new("Parkplätze direkt vor dem Gelände. Ein Auto mieten für 3 Tage und 4-5 Leute kostet circa 50 Euro pro Person für hin&zurück, inklusive Benzin. Dauer circa 1.5 Stunden.").size(17.0)); 
ui.horizontal_wrapped(|ui| {
ui.label(egui::RichText::new("WICHTIG: exakte ").size(17.0)); 
ui.add(
    egui::Hyperlink::from_label_and_url(
        egui::RichText::new("ROUTE").size(17.0).color(egui::Color32::from_rgb(251, 0, 255)),
        "https://maps.app.goo.gl/WtrCENyrHQ23ziUL9",
    )
);
});


ui.label(egui::RichText::new("\nHELIKOPTER:").size(23.0).color(egui::Color32::DARK_GREEN).strong().underline());
ui.label("");
ui.horizontal_wrapped(|ui| {
ui.label(egui::RichText::new("fast unbezahlbar teuer, dafür hier der Link zum").size(17.0));
ui.add(
    egui::Hyperlink::from_label_and_url(
        egui::RichText::new("Helikopterspiel").size(17.0).color(egui::Color32::from_rgb(251, 0, 255)),
        "https://www.addictinggames.com/clicker/helicopter-game",
    )
);
            });

}
            });
                });



            egui::Window::new(self.button7.clone())
                .open(&mut self.flag7)
                .resizable(true)
                .default_width(400.0)
                .default_height(450.0)
                .show(ctx, |ui| {
                    egui::ScrollArea::both().show(ui, |ui| {
                    
                    
                    //ui.separator();

                    
            
                    self.fractal_clock.ui(ui, self.language_flag);
            
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
                    });





                

            egui::Window::new(self.button2.clone()).open(&mut self.flag2).show(ctx, |ui| {
                egui::ScrollArea::both().show(ui, |ui| {
                

                //if self.language_flag == true {ui.label("More details about the schedule will be shared here soon. For now, just a rough outline:\n\nThursday: Helping with setup (optional, please coordinate with us)\n\nFriday: Arrive and be welcomed from 3 PM onward\n\nSaturday: lots of fun and boundless joy!!\n\nSunday: Check out of the bungalows by 12 PM. Help with cleanup if possible until early evening\n\nAccommodation:\n\nThere are small bungalows directly on the event grounds for about half of the guests. These are reserved for the older generations and families with small children. We’ll let you know in the coming months if we’ve planned a spot in a bungalow for you. If you need a bungalow for reasons unknown to us, feel free to reach out directly.\n\nFor everyone else, there’s plenty of space for your own tents – complete with nature vibes and a festival feeling. Clean indoor toilets and showers are accessible to all.\n\nIf you don’t want to stay on the event grounds, there are guesthouses and hotels in the surrounding villages. If you need help with arrangements, feel free to contact us – you're welcome to use the contact button on the website.");}
                //else {ui.label("Genaueres zum Ablauf findet ihr in der nächsten Zeit hier. Vorab nur ganz grob:\n\nDonnerstag: Mithelfen beim Aufbauen (optional, in Absprache mit uns)\n\nFreitag: ab 15 Uhr ankommen und empfangen werden\n\nSamstag: großer Spaß und unbändige Freude -\n\nSonntag: Auschecken aus den Bungalows bis 12 Uhr; Mithelfen beim Aufräumen wenn möglich bis zum frühen Abend\n\nÜbernachtung:\n\nEs gibt direkt auf dem Veranstaltungsgelände kleine Bungalows für etwa die Hälfte der Gäste. Diese sind für die früheren Jahrgänge und Familien mit kleinen Kindern vorgesehen. Wir sagen euch in den nächsten Monaten Bescheid, wenn wir für euch einen Platz im Bungalow geplant haben. Solltet ihr aus uns unbekannten Gründen einen Bungalow brauchen, meldet euch gerne direkt bei uns.\n\nFür alle anderen gibt es reichlich Platz für eigene Zelte inkl. Naturromantik und Festival-Feeling. Feste Toiletten und Duschen sind für alle zugänglich.\n\nWenn ihr nicht auf dem Gelände übernachten möchtet, gibt es in den umliegenden Orten Pensionen und Hotels. Wenn ihr Hilfe beim Organisieren braucht, wendet euch an uns, gerne auch über den Kontakt-Button der Website.");}

                if self.language_flag == true {


                    ui.label(egui::RichText::new("More details about the schedule will be shared here soon. For now, just a rough outline:\n").size(17.0));
                    
                    ui.horizontal_wrapped(|ui| {
                    ui.label(egui::RichText::new("Thursday: ").size(17.0).color(egui::Color32::from_rgb(0, 183, 255)).strong()); ui.label(egui::RichText::new("helping with setup (optional, please coordinate with us)\n").size(17.0));
                    });
                    
                    ui.horizontal_wrapped(|ui| {
                    ui.label(egui::RichText::new("Friday: ").size(17.0).color(egui::Color32::from_rgb(0, 183, 255)).strong()); ui.label(egui::RichText::new("arrive and be welcomed from 3 PM onward\n").size(17.0));
                    });
                    
                    ui.horizontal_wrapped(|ui| {
                    ui.label(egui::RichText::new("Saturday: ").size(17.0).color(egui::Color32::from_rgb(0, 183, 255)).strong()); ui.label(egui::RichText::new("lots of fun and boundless joy!!\n").size(17.0));
                    });
                    
                    ui.horizontal_wrapped(|ui| {
                    ui.label(egui::RichText::new("Sunday: ").size(17.0).color(egui::Color32::from_rgb(0, 183, 255)).strong()); ui.label(egui::RichText::new("check out of the bungalows by 12 PM. Help with cleanup if possible until early evening\n").size(17.0));
                    });
                    
                    
                    ui.label(egui::RichText::new("Accommodation:\n").size(23.0).color(egui::Color32::DARK_GREEN).strong().underline());
                    
                    ui.label(egui::RichText::new("There are small bungalows directly on the event grounds for about half of the guests. These are reserved for the older generations and families with small children. We’ll let you know in the coming months if we’ve planned a spot in a bungalow for you. If you need a bungalow for reasons unknown to us, feel free to reach out directly.\n\nFor everyone else, there’s plenty of space for your own tents – complete with nature vibes and a festival feeling. Clean indoor toilets and showers are accessible to all.\n\nIf you don’t want to stay on the event grounds, there are guesthouses and hotels in the surrounding villages. If you need help with arrangements, feel free to contact us – you're welcome to use the contact button on the website.").size(17.0));
                    
                    }
                                    
                    else {
                    
                    ui.label(egui::RichText::new("Genaueres zum Ablauf findet ihr in der nächsten Zeit hier. Vorab nur ganz grob:\n").size(17.0));
                    
                    ui.horizontal_wrapped(|ui| {
                    ui.label(egui::RichText::new("Donnerstag: ").size(17.0).color(egui::Color32::from_rgb(0, 183, 255)).strong()); ui.label(egui::RichText::new("Mithelfen beim Aufbauen (optional, in Absprache mit uns)\n").size(17.0));
                    });
                    
                    ui.horizontal_wrapped(|ui| {
                    ui.label(egui::RichText::new("Freitag: ").size(17.0).color(egui::Color32::from_rgb(0, 183, 255)).strong()); ui.label(egui::RichText::new("ab 15 Uhr ankommen und empfangen werden\n").size(17.0));
                    });
                    
                    ui.horizontal_wrapped(|ui| {
                    ui.label(egui::RichText::new("Samstag: ").size(17.0).color(egui::Color32::from_rgb(0, 183, 255)).strong()); ui.label(egui::RichText::new("großer Spaß und unbändige Freude!!\n").size(17.0));
                    });
                    
                    ui.horizontal_wrapped(|ui| {
                    ui.label(egui::RichText::new("Sonntag: ").size(17.0).color(egui::Color32::from_rgb(0, 183, 255)).strong()); ui.label(egui::RichText::new("Auschecken aus den Bungalows bis 12 Uhr; Mithelfen beim Aufräumen wenn möglich bis zum frühen Abend\n").size(17.0));
                    });
                    
                    ui.label(egui::RichText::new("Übernachtung:\n").size(23.0).color(egui::Color32::DARK_GREEN).strong().underline());
                    
                    ui.label(egui::RichText::new("Es gibt direkt auf dem Veranstaltungsgelände kleine Bungalows für etwa die Hälfte der Gäste. Diese sind für die früheren Jahrgänge und Familien mit kleinen Kindern vorgesehen. Wir sagen euch in den nächsten Monaten Bescheid, wenn wir für euch einen Platz im Bungalow geplant haben. Solltet ihr aus uns unbekannten Gründen einen Bungalow brauchen, meldet euch gerne direkt bei uns.\n\nFür alle anderen gibt es reichlich Platz für eigene Zelte inkl. Naturromantik und Festival-Feeling. Feste Toiletten und Duschen sind für alle zugänglich.\n\nWenn ihr nicht auf dem Gelände übernachten möchtet, gibt es in den umliegenden Orten Pensionen und Hotels. Wenn ihr Hilfe beim Organisieren braucht, wendet euch an uns, gerne auch über den Kontakt-Button der Website.").size(17.0));

                }


                
            });
            });


            egui::Window::new(self.button5.clone())
                .open(&mut self.flag5)
                .show(ctx, |ui| {
                    egui::ScrollArea::both().show(ui, |ui| {

                if self.language_flag == true {
                    ui.label(egui::RichText::new("paint a picture and send it to us :)").size(17.0).color(egui::Color32::from_rgb(0, 183, 255)));  
                    ui.horizontal(|ui| {
                    //ui.text_edit_singleline(&mut self.pic_name_en); 
                    ui.add(egui::TextEdit::singleline(&mut self.pic_name_en).font(egui::FontId::proportional(17.0)));
                                               
                                               if ui.button(egui::RichText::new("Send Painting").size(17.0)).clicked() {
                    self.saved_image_data = self.painting_app.export_json(ctx).clone();

                    if let Some(image_data) = &self.saved_image_data {
                            let request1 = ehttp::Request::post("https://ntfy.sh/woodland", format!(r#"{} from {}"#, "new image received", self.pic_name_en).as_bytes().to_vec());                        
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
                        self.pic_value +=1;
                }
                    });

                        if self.pic_value == 0 {ui.label(egui::RichText::new("you have sent no paintings yet").size(17.0));}

                        else if self.pic_value == 1 {ui.label(egui::RichText::new("you have sent one painting").size(17.0));}
                        
                        else {let myentext = format!("you have sent {} paintings", self.value); ui.label(egui::RichText::new(myentext).size(17.0));}
                    
                                              }
                else {
                    
                    
                     
                    ui.label(egui::RichText::new("mal ein Bild uns schick es uns :)").size(17.0).color(egui::Color32::from_rgb(0, 183, 255)));  
                    ui.horizontal(|ui| {
                        //ui.text_edit_singleline(&mut self.pic_name_de);
                        ui.add(egui::TextEdit::singleline(&mut self.pic_name_de).font(egui::FontId::proportional(17.0)));
                      if ui.button(egui::RichText::new("Bild abschicken").size(17.0)).clicked() {
                    self.saved_image_data = self.painting_app.export_json(ctx).clone();

                    if let Some(image_data) = &self.saved_image_data {
                            let request1 = ehttp::Request::post("https://ntfy.sh/woodland", format!(r#"{} from {}"#, "new image received", self.pic_name_de).as_bytes().to_vec());                        
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
                          self.pic_value +=1;
                }
                    });

                        if self.pic_value == 0 {ui.label(egui::RichText::new("du hast bisher keine Bilder geschickt").size(17.0));}

                        else if self.pic_value == 1 {ui.label(egui::RichText::new("du hast ein Bild geschickt").size(17.0));}
                        
                        else {let mydetext = format!("du hast {} Bilder geschickt", self.value); ui.label(egui::RichText::new(mydetext).size(17.0));}


                    
                     }

                self.painting_app.ui(ui, self.language_flag);
                

               
                    
                






                    
                    
                });
                });


            egui::Window::new(self.button4.clone()).open(&mut self.flag4).show(ctx, |ui| {

                egui::ScrollArea::both().show(ui, |ui| {

                    if self.language_flag == true {

                        if self.new_formflag == false {

    ui.label(egui::RichText::new("Please use the form below to respond by no later than May 24th").size(24.0).color(egui::Color32::RED));
	ui.label("");						
	ui.label(egui::RichText::new("To help us organize everything smoothly, we’d really appreciate your support already now. Please tell us which tasks you’d be happy to help with and when you’re available. We’ll coordinate everything and do our best to create a plan that works well for everyone.\n\nMost tasks are not continuous — it’s more about keeping an eye on a certain area and helping out when needed. There will always be clear instructions on what to do.\n\nThe following tasks are available:\n\nFriday / Saturday:\n- Help set up breakfast\n- Help prepare dinner\n- Refill and keep the buffet tidy\n- Clear dishes and help with washing up\n- Keep an eye on the midnight snack\n\n- Prepare and maintain the campfire\n- Refill wood for the sauna and pool area\n- Help with technical setup/support\n- Prepare and keep outdoor areas tidy\n- Flexible helper: refill drinks, tidy up, wash dishes as needed\n\n\nSunday:\n- Clean up the kitchen\n- Clear and tidy the buffet\n- Sort leftover food\n- Organize drink crates and bottle returns\n\n- Carefully take down decorations\n- Tidy up the chill-out area\n- Clean up the party storage area\n- Clean up outdoor areas\n").size(23.0));

	ui.label(egui::RichText::new("The following tasks are available:\n\n").size(23.0).color(egui::Color32::DARK_GREEN).strong().underline()); 
	ui.label(egui::RichText::new("Friday / Saturday:\n").size(23.0).color(egui::Color32::from_rgb(0, 183, 255)));
	ui.label(egui::RichText::new("- Help set up breakfast\n- Help prepare dinner\n- Refill and keep the buffet tidy\n- Clear dishes and help with washing up\n- Keep an eye on the midnight snack\n\n- Prepare and maintain the campfire\n- Refill wood for the sauna and pool area\n- Help with technical setup/support\n- Prepare and keep outdoor areas tidy\n- Flexible helper: refill drinks, tidy up, wash dishes as needed\n\n\n").size(23.0));
	ui.label(egui::RichText::new("Sunday:\n").size(23.0).color(egui::Color32::from_rgb(0, 183, 255)));
	ui.label(egui::RichText::new("- Clean up the kitchen\n- Clear and tidy the buffet\n- Sort leftover food\n- Organize drink crates and bottle returns\n\n- Carefully take down decorations\n- Tidy up the chill-out area\n- Clean up the party storage area\n- Clean up outdoor areas\n").size(23.0));




    ui.label(egui::RichText::new("please enter the names of all guests you want to reply for:").size(17.0));
    ui.horizontal_wrapped(|ui| {
    ui.label(egui::RichText::new("name(s): ").size(17.0).color(egui::Color32::DARK_GREEN));
    ui.add(egui::TextEdit::singleline(&mut self.rsvp01).font(egui::FontId::proportional(17.0)));
    });

    ui.label(egui::RichText::new("\nplease tell us what your favorite helper task(s) would be:").size(17.0));
    ui.horizontal_wrapped(|ui| {
    ui.label(egui::RichText::new("prefered helper task(s): ").size(17.0).color(egui::Color32::DARK_GREEN));
    ui.add(egui::TextEdit::singleline(&mut self.rsvp02).font(egui::FontId::proportional(17.0)));
    });

    ui.label(egui::RichText::new("\nis there someone you would like to partner up with for your task?").size(17.0));
    ui.horizontal_wrapped(|ui| {
    ui.label(egui::RichText::new("gladly with: ").size(17.0).color(egui::Color32::DARK_GREEN));
    ui.add(egui::TextEdit::singleline(&mut self.rsvp03).font(egui::FontId::proportional(17.0)));
    });

    ui.label(egui::RichText::new("\nis there a task and/or time you would like to avoid?").size(17.0));
    ui.horizontal_wrapped(|ui| {
    ui.label(egui::RichText::new("please not: ").size(17.0).color(egui::Color32::DARK_GREEN));
    ui.add(egui::TextEdit::singleline(&mut self.rsvp04).font(egui::FontId::proportional(17.0)));
    });

    ui.label(egui::RichText::new("\nwhen will you be arriving on Friday?").size(17.0));
    ui.horizontal_wrapped(|ui| {
    ui.label(egui::RichText::new("arrival Friday: ").size(17.0).color(egui::Color32::DARK_GREEN));
    ui.add(egui::TextEdit::singleline(&mut self.rsvp05).font(egui::FontId::proportional(17.0)));
    });

    ui.label(egui::RichText::new("\nwhen will you be leaving on Sunday?").size(17.0));
    ui.horizontal_wrapped(|ui| {
    ui.label(egui::RichText::new("departure Sunday: ").size(17.0).color(egui::Color32::DARK_GREEN));
    ui.add(egui::TextEdit::singleline(&mut self.rsvp06).font(egui::FontId::proportional(17.0)));
    });

		ui.label("");					

	if ui.button(egui::RichText::new("submit").size(17.0).color(egui::Color32::from_rgb(0, 183, 255))).clicked() {
self.helper_names = self.rsvp01.clone();
self.helper_task = self.rsvp02.clone();
self.helper_with = self.rsvp03.clone();
self.helper_no = self.rsvp04.clone();
self.helper_fri = self.rsvp05.clone();
self.helper_sun = self.rsvp06.clone();
let message = format!(r#"CONFIRMED::: names: {} ; prefs: {} ; with: {}; please no: {} ; arrival: {} ; departure: {}"#, self.helper_names, self.helper_task, self.helper_with, self.helper_no, self.helper_fri, self.helper_sun);
let body1 = message.as_bytes().to_vec();
let request1 = ehttp::Request::post("https://ntfy.sh/woodland", body1);
ehttp::fetch(request1, move |result: ehttp::Result<ehttp::Response>| {println!("Status code: {:?}", result.unwrap().status);});
self.new_formflag = true;
}

}
else {
    ui.label(egui::RichText::new("Thank you!! you have sent a reply:").size(23.0));
    ui.label("");

    ui.horizontal_wrapped(|ui| {
    ui.label(egui::RichText::new("name(s): ").size(17.0).color(egui::Color32::DARK_GREEN));
    ui.label(egui::RichText::new(self.helper_names.clone()).size(17.0));
    });

    ui.horizontal_wrapped(|ui| {
    ui.label(egui::RichText::new("prefered helper task(s): ").size(17.0).color(egui::Color32::DARK_GREEN));
    ui.label(egui::RichText::new(self.helper_task.clone()).size(17.0));
    });

    ui.horizontal_wrapped(|ui| {
    ui.label(egui::RichText::new("gladly with: ").size(17.0).color(egui::Color32::DARK_GREEN));
    ui.label(egui::RichText::new(self.helper_with.clone()).size(17.0));
    });

    ui.horizontal_wrapped(|ui| {
    ui.label(egui::RichText::new("please not: ").size(17.0).color(egui::Color32::DARK_GREEN));
    ui.label(egui::RichText::new(self.helper_no.clone()).size(17.0));
    });

    ui.horizontal_wrapped(|ui| {
    ui.label(egui::RichText::new("arrival Friday: ").size(17.0).color(egui::Color32::DARK_GREEN));
    ui.label(egui::RichText::new(self.helper_fri.clone()).size(17.0));
    });

    ui.horizontal_wrapped(|ui| {
    ui.label(egui::RichText::new("departure Sunday: ").size(17.0).color(egui::Color32::DARK_GREEN));
    ui.label(egui::RichText::new(self.helper_sun.clone()).size(17.0));
    });

    ui.label("");

    if ui.button(egui::RichText::new("go back and send more replies").size(17.0).color(egui::Color32::from_rgb(0, 183, 255))).clicked(){
		self.rsvp01.clear();
		self.rsvp02.clear();
		self.rsvp03.clear();
		self.rsvp04.clear();
		self.rsvp05.clear();
		self.rsvp06.clear();
		self.new_formflag = false; }


}


                        
                   }
                else {

if self.new_formflag == false {


	ui.label(egui::RichText::new("Bitte benutze bis spätestens 24ter Mai das Formular weiter unten um uns zu antworten.").size(24.0).color(egui::Color32::RED));
	ui.label("");
	
	ui.label(egui::RichText::new("Damit wir alle Helferlis gut einteilen können, brauchen wir schon jetzt eure Hilfe. Bitte gebt an, welche Aufgaben ihr gern übernehmen möchtet und wann ihr dafür Zeit habt. Wir teilen euch ein und versuchen, den Plan für alle passend zu gestalten.\n\nViele Aufgaben sind nicht durchgehend - es geht eher darum, im Blick zu haben, ob im jeweiligen Bereich etwas gebraucht wird. Es wird immer genaue Anweisungen geben, was zu tun ist.\n\n").size(23.0));

	ui.label(egui::RichText::new("Folgende Aufgaben stehen zur Auswahl:\n\n").size(23.0).color(egui::Color32::DARK_GREEN).strong().underline()); 
	
	ui.label(egui::RichText::new("Freitag / Samstag:\n").size(23.0).color(egui::Color32::from_rgb(0, 183, 255)));
	ui.label(egui::RichText::new("- Frühstück mit aufbauen\n- Abendessen vorbereiten helfen\n- Buffet nachfüllen und sauber halten\n- Essen abräumen und spülen\n- Mitternachtssnack im Blick haben\n\n- Lagerfeuer machen\n- Holz nachlegen bei Sauna und Pool\n- bei Technik mithelfen\n- Außenbereiche vorbereiten/ sauber halten\n- Springer: Getränke nachfüllen, aufräumen, spülen nach Bedarf\n\n\n").size(23.0));
	ui.label(egui::RichText::new("Sonntag:\n").size(23.0).color(egui::Color32::from_rgb(0, 183, 255)));
	ui.label(egui::RichText::new("- Küche aufräumen\n- Buffet aufräumen\n- Lebensmittel sortieren\n- Getränkekisten/ Pfand sortieren\n\n- Deko vorsichtig abbauen\n- Chill-Area aufräumen\n- Party-Kabuff aufräumen\n- Außenbereiche aufräumen\n").size(23.0));





	
	ui.label(egui::RichText::new("bitte gib die Namen aller Gäste ein für die du antworten willst:").size(17.0));
	ui.horizontal_wrapped(|ui| {
	ui.label(egui::RichText::new("Name(n): ").size(17.0).color(egui::Color32::DARK_GREEN));
	ui.add(egui::TextEdit::singleline(&mut self.rsvp01).font(egui::FontId::proportional(17.0)));
	});

	ui.label(egui::RichText::new("\nsag uns bitte deine Lieblingsaufgabe(n):").size(17.0));
	ui.horizontal_wrapped(|ui| {
	ui.label(egui::RichText::new("Lieblingsaufgabe(n): ").size(17.0).color(egui::Color32::DARK_GREEN));
	ui.add(egui::TextEdit::singleline(&mut self.rsvp02).font(egui::FontId::proportional(17.0)));
	});

	ui.label(egui::RichText::new("\nwürdest du deine Aufgabe gerne mit jemandem zusammen machen?").size(17.0));
	ui.horizontal_wrapped(|ui| {
	ui.label(egui::RichText::new("Wunschpartner*in: ").size(17.0).color(egui::Color32::DARK_GREEN));
	ui.add(egui::TextEdit::singleline(&mut self.rsvp03).font(egui::FontId::proportional(17.0)));
	});

	ui.label(egui::RichText::new("\ngibt es eine Aufgabe oder Zeit die überhaupt nicht passt?").size(17.0));
	ui.horizontal_wrapped(|ui| {
	ui.label(egui::RichText::new("bitte nicht: ").size(17.0).color(egui::Color32::DARK_GREEN));
	ui.add(egui::TextEdit::singleline(&mut self.rsvp04).font(egui::FontId::proportional(17.0)));
	});

	ui.label(egui::RichText::new("\nwann kommst du am Freitag an?").size(17.0));
	ui.horizontal_wrapped(|ui| {
	ui.label(egui::RichText::new("Ankunft Freitag: ").size(17.0).color(egui::Color32::DARK_GREEN));
	ui.add(egui::TextEdit::singleline(&mut self.rsvp05).font(egui::FontId::proportional(17.0)));
	});

	ui.label(egui::RichText::new("\nwann fährst du am Sonntag ab?").size(17.0));
	ui.horizontal_wrapped(|ui| {
	ui.label(egui::RichText::new("Abfahrt Sonntag: ").size(17.0).color(egui::Color32::DARK_GREEN));
	ui.add(egui::TextEdit::singleline(&mut self.rsvp06).font(egui::FontId::proportional(17.0)));
	});

	ui.label("");

	if ui.button(egui::RichText::new("abschicken").size(17.0).color(egui::Color32::from_rgb(0, 183, 255))).clicked() {
self.helper_names = self.rsvp01.clone();
self.helper_task = self.rsvp02.clone();
self.helper_with = self.rsvp03.clone();
self.helper_no = self.rsvp04.clone();
self.helper_fri = self.rsvp05.clone();
self.helper_sun = self.rsvp06.clone();
let message = format!(r#"CONFIRMED::: names: {} ; prefs: {} ; with: {}; please no: {} ; arrival: {} ; departure: {}"#, self.helper_names, self.helper_task, self.helper_with, self.helper_no, self.helper_fri, self.helper_sun);
let body1 = message.as_bytes().to_vec();
let request1 = ehttp::Request::post("https://ntfy.sh/woodland", body1);
ehttp::fetch(request1, move |result: ehttp::Result<ehttp::Response>| {println!("Status code: {:?}", result.unwrap().status);});
self.new_formflag = true;
}

}
else {
	ui.label(egui::RichText::new("Danke!! du hast eine Antwort gesendet:").size(23.0));
	ui.label("");

	ui.horizontal_wrapped(|ui| {
	ui.label(egui::RichText::new("Name(n): ").size(17.0).color(egui::Color32::DARK_GREEN));
	ui.label(egui::RichText::new(self.helper_names.clone()).size(17.0));
	});

	ui.horizontal_wrapped(|ui| {
	ui.label(egui::RichText::new("Lieblingsaufgabe(n): ").size(17.0).color(egui::Color32::DARK_GREEN));
	ui.label(egui::RichText::new(self.helper_task.clone()).size(17.0));
	});

	ui.horizontal_wrapped(|ui| {
	ui.label(egui::RichText::new("Wunschpartner*in: ").size(17.0).color(egui::Color32::DARK_GREEN));
	ui.label(egui::RichText::new(self.helper_with.clone()).size(17.0));
	});

	ui.horizontal_wrapped(|ui| {
	ui.label(egui::RichText::new("bitte nicht: ").size(17.0).color(egui::Color32::DARK_GREEN));
	ui.label(egui::RichText::new(self.helper_no.clone()).size(17.0));
	});

	ui.horizontal_wrapped(|ui| {
	ui.label(egui::RichText::new("Ankunft Freitag: ").size(17.0).color(egui::Color32::DARK_GREEN));
	ui.label(egui::RichText::new(self.helper_fri.clone()).size(17.0));
	});

	ui.horizontal_wrapped(|ui| {
	ui.label(egui::RichText::new("Abfahrt Sonntag: ").size(17.0).color(egui::Color32::DARK_GREEN));
	ui.label(egui::RichText::new(self.helper_sun.clone()).size(17.0));
	});

	ui.label("");

	if ui.button(egui::RichText::new("zurück und mehr Antworten senden").size(17.0).color(egui::Color32::from_rgb(0, 183, 255))).clicked(){
		self.rsvp01.clear();
		self.rsvp02.clear();
		self.rsvp03.clear();
		self.rsvp04.clear();
		self.rsvp05.clear();
		self.rsvp06.clear();
		self.new_formflag = false; }


}


                    
                    
                     }
                
            });
                
                
                
            });

                

            egui::Window::new(self.button3.clone()).open(&mut self.flag3).show(ctx, |ui| {
                egui::ScrollArea::both().show(ui, |ui| {

                //if self.language_flag == true {ui.label("You’re invited to enjoy food and drinks all weekend, and there will be no cost for staying overnight in a tent or bungalow. What matters most is that everyone can be there and be comfortable.\n\nOf course our pockets will feel a bit empty after the event. If it’s possible for you to help fill them up again, we would be very grateful for any financial contribution — no matter how great or small the amount.\n\nIf you would like to make a contribution, please send it anytime to:\n\nPaypal: woodland.wedding@pm.me   Subject: Woodland Wedding\n\nBank Account: Kimberley Hofer & Matthias Hofer, DE...  Subject: Woodland Wedding");}
                //else {ui.label("Ihr seid das ganze Wochenende auf Essen und Trinken eingeladen, auch für die Übernachtung im Zelt oder Bungalow fallen prinzipiell keine Kosten für euch an. Das Wichtigste ist, dass alle dabei sein können und sich wohl fühlen.\n\nNatürlich wird unser Geldbeutel nach diesem Event ganz schön Magenknurren haben. Wenn es für euch möglich ist, sind wir über finanzielles Futter in jeder Höhe sehr dankbar.\n\nGerne jederzeit an:\n\nPaypal: woodland.wedding@pm.me   Betreff: Woodland Wedding\n\nKonto: Kimberley Hofer & Matthias Hofer, DE...  Betreff: Woodland Wedding");}

                if self.language_flag == true {

                ui.label(egui::RichText::new("You’re invited to enjoy food and drinks all weekend, and there will be no cost for staying overnight in a tent or bungalow. What matters most is that everyone can be there and be comfortable.\n\nOf course our pockets will feel a bit empty after the event. If it’s possible for you to help fill them up again, we would be very grateful for any financial contribution — no matter how great or small the amount.\n").size(17.0));

				ui.label(egui::RichText::new("As a few people have already asked, we’re happy to share a suggested contribution (for those who are comfortably able to give):\n\nPer adult staying in a tent: €85.17\nPer adult staying in a bungalow: €142.41\n\nIf you would like to make a contribution, please send it anytime to:").size(17.0));

					
                ui.horizontal_wrapped(|ui| {
                ui.label(egui::RichText::new("Paypal: ").size(17.0).color(egui::Color32::from_rgb(251, 0, 255)).strong()); ui.label(egui::RichText::new("woodland.wedding@pm.me; Subject: Woodland Wedding\n").size(17.0));
                });
                
                ui.horizontal_wrapped(|ui| {
                ui.label(egui::RichText::new("Bank Account: ").size(17.0).color(egui::Color32::from_rgb(251, 0, 255)).strong()); ui.label(egui::RichText::new("Kimberley Hofer & Matthias Hofer, DE  8743  0609  6712  8485  0000; Subject: Woodland Wedding").size(17.0));
                });
                
                }
                                
                
                else {
                
                ui.label(egui::RichText::new("Ihr seid das ganze Wochenende auf Essen und Trinken eingeladen, auch für die Übernachtung im Zelt oder Bungalow fallen prinzipiell keine Kosten für euch an. Das Wichtigste ist, dass alle dabei sein können und sich wohl fühlen.\n\nNatürlich wird unser Geldbeutel nach diesem Event ganz schön Magenknurren haben.\n").size(17.0));

				ui.label(egui::RichText::new("Da wir schon ein paar Mal gefragt wurden, geben wir euch eine \"Spenden-Empfehlung\" (für Menschen mit zufriedenem Geldbeutel):\n\nPro Erwachsene*r im Zelt: 85,17€\nPro Erwachsene*r im Bungalow: 142,41€\n\nWenn es für euch möglich ist, sind wir über finanzielles Futter in jeder Höhe sehr dankbar.\n\nGerne jederzeit an:\n").size(17.0));
                

					
                ui.horizontal_wrapped(|ui| {
                ui.label(egui::RichText::new("Paypal: ").size(17.0).color(egui::Color32::from_rgb(251, 0, 255)).strong()); ui.label(egui::RichText::new("woodland.wedding@pm.me; Betreff: Woodland Wedding\n").size(17.0));
                });
                
                ui.horizontal_wrapped(|ui| {
                ui.label(egui::RichText::new("Konto: ").size(17.0).color(egui::Color32::from_rgb(251, 0, 255)).strong()); ui.label(egui::RichText::new("Kimberley Hofer & Matthias Hofer, DE  8743  0609  6712  8485  0000; Betreff: Woodland Wedding").size(17.0));
                });
                
                }
                
            });
            });


            egui::Window::new(self.button6.clone()).open(&mut self.flag6).show(ctx, |ui| {
                egui::ScrollArea::both().show(ui, |ui| {
                if ui.input(|i| i.key_pressed(egui::Key::R)) {
                self.snake.reset();
            }

            if self.language_flag == true {ui.collapsing(egui::RichText::new("how to play").size(17.0), |ui| {
            ui.label(egui::RichText::new("press R for restart and use arrow keys for playing:\n").size(17.0));
            ui.label(egui::RichText::new("on desktop, use your keyboard").size(17.0));
            ui.label(egui::RichText::new("on mobile, use touch-keys below").size(17.0));
        });}
            else {ui.collapsing(egui::RichText::new("Spielanleitung").size(17.0), |ui| {
                
            ui.label(egui::RichText::new("R-Taste für Neustarten und Pfeiltasten für Steuerung:\n").size(17.0));       
            ui.label(egui::RichText::new("am PC, Tastatur benutzen").size(17.0));
            ui.label(egui::RichText::new("am Handy, Touch-Tasten unten benutzen").size(17.0));
        });}


                
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
            });

                
            

            egui::Window::new(self.button8.clone()).open(&mut self.flag8).show(ctx, |ui| {
                egui::ScrollArea::both().show(ui, |ui| {

                if self.language_flag == true 

                {
                
                ui.label(egui::RichText::new("Please add what you’d like to bring for the Friday buffet: ideally finger food that can easily be eaten by hand — sweet or savory are both very welcome.\n").size(23.0).color(egui::Color32::DARK_GREEN).strong().underline());
                
                ui.label(egui::RichText::new("(no stress at all if you can’t bring anything because of travel or time constraints)").size(17.0));
                
                ui.add(
                    egui::Hyperlink::from_label_and_url(
                        egui::RichText::new("click me - Potluck List - click me").size(24.0).color(egui::Color32::from_rgb(251, 0, 255)),
                        "https://docs.google.com/document/d/1oSsfuwkMAKTApy1AX0LG_zBfuD6Ym0_B82CBWN-xvKE/edit?usp=sharing",
                    )
                );

				
                

				ui.label("");
					
				ui.label(egui::RichText::new("Things to bring:\n").size(23.0).color(egui::Color32::DARK_GREEN).strong().underline());
					
                ui.label(egui::RichText::new("Here are a few things we’d recommend bringing along to make the weekend extra cozy and comfortable:\n(Bed linen and towels are provided in the bungalows.)\n\n- Camping chair\n- Picnic blanket\n- Seat cushion for the beer benches\n- A cozy blanket for sitting outside in the evening\n- Swimwear\n- Flip-flops or sandals\n- Sauna towel\n- Sun protection\n- Mosquito repellent\n- Rain gear — just in case\n- Layers for cooler evenings (\"onion style\" clothing works best!)").size(17.0));

				ui.label("");
				
                ui.label(egui::RichText::new("Dress code:\n").size(23.0).color(egui::Color32::DARK_GREEN).strong().underline());
                
                ui.label(egui::RichText::new("no need for formal evening wear unless it makes you happy. We love colorful, glittery, shiny and extravagant outfits. But pajamas, pirate costumes and potato sacks are also favorites. So – feel free to express yourself!").size(17.0));

                }
                                
                
                else {

				ui.label(egui::RichText::new("Bitte tragt ein, was ihr für das Buffet am Freitag mitbringt: bitte Fingerfood, das entspannt aus der Hand gegessen werden kann - gerne süß oder salzig.\n").size(23.0).color(egui::Color32::DARK_GREEN).strong().underline());
              
                ui.label(egui::RichText::new("(kein Stress falls ihr wegen Zeitmangel oder langer Fahrt nichts beitragen könnt)").size(17.0));
                
                ui.add(
                    egui::Hyperlink::from_label_and_url(
                        egui::RichText::new("klick mich - Mitbring-Buffet-Liste - klick mich").size(24.0).color(egui::Color32::from_rgb(251, 0, 255)),
                        "https://docs.google.com/document/d/1oSsfuwkMAKTApy1AX0LG_zBfuD6Ym0_B82CBWN-xvKE/edit?usp=sharing",
                    )
                );
                ui.label("");
				ui.label(egui::RichText::new("Mitbringliste:\n").size(23.0).color(egui::Color32::DARK_GREEN).strong().underline());
					
                ui.label(egui::RichText::new("Hier unsere Empfehlung, was ihr mitbringen könnt, um das Wochenende gemütlich zu verbringen:\n(In den Bungalows sind Handtücher und Bettwäsche vorhanden)\n\n- Campingstuhl\n- Picknickdecke\n- Sitzkissen für die Bierbank\n- Kuscheldecke zum abends draußen sitzen\n- Badesachen\n- Badelatschen\n- Sauna-Handtuch\n- Sonnenschutz\n- Mückenzeug\n- Regenkleidung: man weiß ja nie\n- Zwiebelschalen: falls es abends kalt wird").size(17.0));
				ui.label("");
				ui.label(egui::RichText::new("Dress code:\n").size(23.0).color(egui::Color32::DARK_GREEN).strong().underline());
                
                ui.label(egui::RichText::new("Förmliche Abendgarderobe ist nicht nötig, es sei denn sie macht euch glücklich. Wir freuen uns immer über farbenfrohe, glitzernde, schillernde und extravagante Outfits. Aber auch Pyjamas, Piratenkostüme und Kartoffelsäcke gehören zu unseren Favoriten. Ihr habt also die freie Wahl!\n\n").size(17.0));

                }
                
            });
            });

            //egui::Window::new(self.button9.clone()).open(&mut self.flag9).show(ctx, |ui| {
             //   ui.text_edit_singleline(&mut self.user_input); 
             //   if ui.button("send").clicked() {
             //   let json1 = format!(r#"{}"#, self.user_input);
             //   let body1 = json1.as_bytes().to_vec();
             //   let request1 = ehttp::Request::post("https://ntfy.sh/woodland", body1);
                //use ehttp::Request;
                //let request1 = Request {
                //    headers: ehttp::Headers::new(&[
                //        ("Accept", "*/*"),
                //        ("Content-Type", "text/plain; charset=utf-8"),
                //        ("X-Email", "matthias.hofer@pm.me"),
                //    ]),
                //    ..Request::post("https://ntfy.sh/woodland", body1)
                //};
           //     ehttp::fetch(request1, move |result: ehttp::Result<ehttp::Response>| {println!("Status code: {:?}", result.unwrap().status);});
           //     self.value +=1;}
           //     ui.label(format!("you sent {} messages", self.value));
           //     ui.hyperlink_to("see messages", "https://ntfy.sh/woodland");
            //});

            egui::Window::new(self.button9.clone()).open(&mut self.flag9).show(ctx, |ui| {
                egui::ScrollArea::both().show(ui, |ui| {

                if self.language_flag == true {

ui.label(egui::RichText::new("Here you have the opportunity to contact us directly. Questions, suggestions and messages of any kind are very welcome. Is there anything else that you think we might have forgotten or that should be taken into account?").size(17.0));

ui.label("");

ui.add(egui::TextEdit::singleline(&mut self.user_input_en).font(egui::FontId::proportional(17.0)));

if ui.button(egui::RichText::new("send").size(17.0)).clicked() {
      let json1 = format!(r#"{}"#, self.user_input_en);
      let body1 = json1.as_bytes().to_vec();
      let request1 = ehttp::Request::post("https://ntfy.sh/woodland", body1);
      ehttp::fetch(request1, move |result: ehttp::Result<ehttp::Response>| {println!("Status code: {:?}", result.unwrap().status);});
      self.value +=1;}



if self.value == 0 {ui.label(egui::RichText::new("you have sent no messages yet").size(17.0));}

else if self.value == 1 {ui.label(egui::RichText::new("you have sent one message").size(17.0));}

else {let myentext = format!("you have sent {} messages", self.value); ui.label(egui::RichText::new(myentext).size(17.0));}
                                                                                                 
                                              }


else {

ui.label(egui::RichText::new("Hier gibt es die Möglichkeit uns direkt zu kontaktieren. Fragen, Vorschläge und Mitteilungen aller Art sind höchst willkommen. Fällt dir noch irgendwas ein was wir vergessen haben oder noch berücksichtigt werden sollte?").size(17.0));

ui.label("");

ui.add(egui::TextEdit::singleline(&mut self.user_input_de).font(egui::FontId::proportional(17.0)));

if ui.button(egui::RichText::new("abschicken").size(17.0)).clicked() {
                          let json1 = format!(r#"{}"#, self.user_input_de);
                          let body1 = json1.as_bytes().to_vec();
                          let request1 = ehttp::Request::post("https://ntfy.sh/woodland", body1);
                          ehttp::fetch(request1, move |result: ehttp::Result<ehttp::Response>| {println!("Status code: {:?}", result.unwrap().status);});
                          self.value +=1;}



if self.value == 0 {ui.label(egui::RichText::new("du hast bisher keine Nachrichten geschickt").size(17.0));}

else if self.value == 1 {ui.label(egui::RichText::new("du hast eine Nachricht geschickt").size(17.0));}

else {let mydetext = format!("du hast {} Nachrichten geschickt", self.value); ui.label(egui::RichText::new(mydetext).size(17.0));}

}
                
            });
            });







                

            egui::Window::new(self.formbutton.clone()).open(&mut self.formflag).show(ctx, |ui| {
                egui::ScrollArea::both().show(ui, |ui| {

               

                 if self.language_flag == true {

                        if self.rsvp_flag1 == false {
                            if ui.button(egui::RichText::new("go to the confirm form").size(17.0).color(egui::Color32::GREEN)).clicked() {self.rsvp_flag2 = true; self.rsvp_flag1 = true;}
                            if ui.button(egui::RichText::new("go to the decline form").size(17.0).color(egui::Color32::RED)).clicked() {self.rsvp_flag2 = false; self.rsvp_flag1 = true;}
                        }
                        else {

                            if self.rsvp_flag2 == true {

                                ui.label(egui::RichText::new("please enter your data below:\n").size(23.0));
                                ui.label(egui::RichText::new("please enter the names of all people you want to confirm:").size(17.0));
                                ui.horizontal_wrapped(|ui| {
                                ui.label(egui::RichText::new("name(s): ").size(17.0).color(egui::Color32::DARK_GREEN));
                                ui.add(egui::TextEdit::singleline(&mut self.rsvp1).font(egui::FontId::proportional(17.0)));
                                });
                                ui.label(egui::RichText::new("\nplease tell us all email-addresses we can send you updates to:").size(17.0));
                                ui.horizontal_wrapped(|ui| {
                                ui.label(egui::RichText::new("email-address(es): ").size(17.0).color(egui::Color32::DARK_GREEN));
                                ui.add(egui::TextEdit::singleline(&mut self.rsvp2).font(egui::FontId::proportional(17.0)));
                                });
                                ui.label(egui::RichText::new("\ntell us some songs you would appreciate to hear during the event:").size(17.0));
                                ui.horizontal_wrapped(|ui| {
                                ui.label(egui::RichText::new("like-to-hear song(s): ").size(17.0).color(egui::Color32::DARK_GREEN));
                                ui.add(egui::TextEdit::singleline(&mut self.rsvp3).font(egui::FontId::proportional(17.0)));
                                });
                                ui.label(egui::RichText::new("\ngive us any comments you might have here:").size(17.0));
                                ui.horizontal_wrapped(|ui| {
                                ui.label(egui::RichText::new("comment(s): ").size(17.0).color(egui::Color32::DARK_GREEN));
                                ui.add(egui::TextEdit::singleline(&mut self.rsvp4).font(egui::FontId::proportional(17.0)));
                                });
                                ui.label("");
                                if self.submitflag == false {ui.label("");}
                                else {
                                ui.label(egui::RichText::new("your SUBMITTED data:").size(23.0));
                                ui.label("");
                                ui.horizontal_wrapped(|ui| {
                                ui.label(egui::RichText::new("name(s): ").size(17.0).color(egui::Color32::DARK_GREEN));
                                ui.label(egui::RichText::new(self.submit_names.clone()).size(17.0));
                                });
                                ui.horizontal_wrapped(|ui| {
                                ui.label(egui::RichText::new("email-address(es): ").size(17.0).color(egui::Color32::DARK_GREEN));
                                ui.label(egui::RichText::new(self.submit_emails.clone()).size(17.0));
                                });
                                ui.horizontal_wrapped(|ui| {
                                ui.label(egui::RichText::new("song(s): ").size(17.0).color(egui::Color32::DARK_GREEN));
                                ui.label(egui::RichText::new(self.submit_songs.clone()).size(17.0));
                                });
                                ui.horizontal_wrapped(|ui| {
                                ui.label(egui::RichText::new("comments(s): ").size(17.0).color(egui::Color32::DARK_GREEN));
                                ui.label(egui::RichText::new(self.submit_comments.clone()).size(17.0));
                                });
                                ui.label("");
                                }
                                if ui.button(egui::RichText::new("submit").size(17.0).color(egui::Color32::from_rgb(0, 183, 255))).clicked() {
                                self.submit_names = self.rsvp1.clone();
                                self.submit_emails = self.rsvp2.clone();
                                self.submit_songs = self.rsvp3.clone();
                                self.submit_comments = self.rsvp4.clone();
                                let message = format!(r#"CONFIRMED::: names: {} ; emails: {} ; songs: {}; comments: {}"#, self.submit_names, self.submit_emails, self.submit_songs, self.submit_comments);
                                let body1 = message.as_bytes().to_vec();
                                let request1 = ehttp::Request::post("https://ntfy.sh/woodland", body1);
                                ehttp::fetch(request1, move |result: ehttp::Result<ehttp::Response>| {println!("Status code: {:?}", result.unwrap().status);});
                                self.submitflag = true;
                                }
                                ui.label("");
                                ui.label("");
                                ui.horizontal_wrapped(|ui| {
                                    ui.label(egui::RichText::new("go to the ").size(17.0));
                                    if ui.button(egui::RichText::new("decline form").size(17.0).color(egui::Color32::RED)).clicked() {self.rsvp_flag2 = false;};
                                    
                                });
                                
                                
                            }
                            else {
                                ui.label(egui::RichText::new("please enter your data below:\n").size(23.0));
                                ui.label(egui::RichText::new("please enter the names of all people who cannot come:").size(17.0));
                                ui.horizontal_wrapped(|ui| {
                                ui.label(egui::RichText::new("name(s): ").size(17.0).color(egui::Color32::RED));
                                ui.add(egui::TextEdit::singleline(&mut self.rsvp5).font(egui::FontId::proportional(17.0)));
                                });
                                
                                
                                ui.label(egui::RichText::new("\ngive us any comments you might have here:").size(17.0));
                                ui.horizontal_wrapped(|ui| {
                                ui.label(egui::RichText::new("comment(s): ").size(17.0).color(egui::Color32::RED));
                                ui.add(egui::TextEdit::singleline(&mut self.rsvp6).font(egui::FontId::proportional(17.0)));
                                });
                                ui.label("");
                                if self.submitflag == false {ui.label("");}
                                else {
                                ui.label(egui::RichText::new("your SUBMITTED data:").size(23.0));
                                ui.label("");
                                ui.horizontal_wrapped(|ui| {
                                ui.label(egui::RichText::new("name(s): ").size(17.0).color(egui::Color32::RED));
                                ui.label(egui::RichText::new(self.decline_names.clone()).size(17.0));
                                });
                                
                                
                                ui.horizontal_wrapped(|ui| {
                                ui.label(egui::RichText::new("comments(s): ").size(17.0).color(egui::Color32::RED));
                                ui.label(egui::RichText::new(self.decline_comments.clone()).size(17.0));
                                });
                                ui.label("");
                                }
                                if ui.button(egui::RichText::new("submit").size(17.0).color(egui::Color32::from_rgb(0, 183, 255))).clicked() {
                                self.decline_names = self.rsvp5.clone();
                                //self.submit_emails = self.rsvp2.clone();
                                //self.submit_songs = self.rsvp3.clone();
                                self.decline_comments = self.rsvp6.clone();
                                let message = format!(r#"DECLINED::: names: {} ; comments: {}"#, self.decline_names, self.decline_comments);
                                let body1 = message.as_bytes().to_vec();
                                let request1 = ehttp::Request::post("https://ntfy.sh/woodland", body1);
                                ehttp::fetch(request1, move |result: ehttp::Result<ehttp::Response>| {println!("Status code: {:?}", result.unwrap().status);});
                                self.submitflag = true;
                                }
                                ui.label("");
                                ui.label("");
                                ui.horizontal_wrapped(|ui| {
                                    ui.label(egui::RichText::new("go to the ").size(17.0));
                                    if ui.button(egui::RichText::new("confirm form").size(17.0).color(egui::Color32::GREEN)).clicked() {self.rsvp_flag2 = true;};
                                    
                                });
                                
                            }

                        }
                        
                        
                     
                        }
                else {
                        if self.rsvp_flag1 == false {
                            if ui.button(egui::RichText::new("zum Zusageformular").size(17.0).color(egui::Color32::GREEN)).clicked() {self.rsvp_flag2 = true; self.rsvp_flag1 = true;}
                            if ui.button(egui::RichText::new("zum Absageformular").size(17.0).color(egui::Color32::RED)).clicked() {self.rsvp_flag2 = false; self.rsvp_flag1 = true;}
                        }

else {
    
    if self.rsvp_flag2 == true {

            ui.label(egui::RichText::new("gib hier bitte deine Daten ein:\n").size(23.0));
            ui.label(egui::RichText::new("bitte gib die Namen aller Gäste ein die du bestätigen willst:").size(17.0));
            ui.horizontal_wrapped(|ui| {
            ui.label(egui::RichText::new("Name(n): ").size(17.0).color(egui::Color32::DARK_GREEN));
            ui.add(egui::TextEdit::singleline(&mut self.rsvp1).font(egui::FontId::proportional(17.0)));
            });
            ui.label(egui::RichText::new("\nbitte gib uns alle eMail-Adressen an die wir Updates schicken können:").size(17.0));
            ui.horizontal_wrapped(|ui| {
            ui.label(egui::RichText::new("email-Adresse(n): ").size(17.0).color(egui::Color32::DARK_GREEN));
            ui.add(egui::TextEdit::singleline(&mut self.rsvp2).font(egui::FontId::proportional(17.0)));
            });
            ui.label(egui::RichText::new("\nwenn du dich freuen würdest spezielle Lieder auf der Party zu hören, gib sie bitte hier ein:").size(17.0));
            ui.horizontal_wrapped(|ui| {
            ui.label(egui::RichText::new("Wunschlied(er): ").size(17.0).color(egui::Color32::DARK_GREEN));
            ui.add(egui::TextEdit::singleline(&mut self.rsvp3).font(egui::FontId::proportional(17.0)));
            });
            ui.label(egui::RichText::new("\nhier kannst du allgemeine Kommentare eingeben:").size(17.0));
            ui.horizontal_wrapped(|ui| {
            ui.label(egui::RichText::new("Kommentar(e): ").size(17.0).color(egui::Color32::DARK_GREEN));
            ui.add(egui::TextEdit::singleline(&mut self.rsvp4).font(egui::FontId::proportional(17.0)));
            });
            ui.label("");
            if self.submitflag == false {ui.label("");}
            else {
            ui.label(egui::RichText::new("deine ABGESCHICKTEN Daten:").size(23.0));
            ui.label("");
            ui.horizontal_wrapped(|ui| {
            ui.label(egui::RichText::new("Name(n): ").size(17.0).color(egui::Color32::DARK_GREEN));
            ui.label(egui::RichText::new(self.submit_names.clone()).size(17.0));
            });
            ui.horizontal_wrapped(|ui| {
            ui.label(egui::RichText::new("email-Adresse(n): ").size(17.0).color(egui::Color32::DARK_GREEN));
            ui.label(egui::RichText::new(self.submit_emails.clone()).size(17.0));
            });
            ui.horizontal_wrapped(|ui| {
            ui.label(egui::RichText::new("Wunschlied(er): ").size(17.0).color(egui::Color32::DARK_GREEN));
            ui.label(egui::RichText::new(self.submit_songs.clone()).size(17.0));
            });
            ui.horizontal_wrapped(|ui| {
            ui.label(egui::RichText::new("Kommentar(e): ").size(17.0).color(egui::Color32::DARK_GREEN));
            ui.label(egui::RichText::new(self.submit_comments.clone()).size(17.0));
            });
            ui.label("");
            }
            if ui.button(egui::RichText::new("abschicken").size(17.0).color(egui::Color32::from_rgb(0, 183, 255))).clicked() {
            self.submit_names = self.rsvp1.clone();
            self.submit_emails = self.rsvp2.clone();
            self.submit_songs = self.rsvp3.clone();
            self.submit_comments = self.rsvp4.clone();
            let message = format!(r#"ZUGESAGT::: names: {} ; emails: {} ; songs: {}; comments: {}"#, self.submit_names, self.submit_emails, self.submit_songs, self.submit_comments);
            let body1 = message.as_bytes().to_vec();
            let request1 = ehttp::Request::post("https://ntfy.sh/woodland", body1);
            ehttp::fetch(request1, move |result: ehttp::Result<ehttp::Response>| {println!("Status code: {:?}", result.unwrap().status);});
            self.submitflag = true;
            }
            ui.label("");
            ui.label("");
            ui.horizontal_wrapped(|ui| {
                ui.label(egui::RichText::new("gehe zum ").size(17.0));
                if ui.button(egui::RichText::new("Absageformular").size(17.0).color(egui::Color32::RED)).clicked() {self.rsvp_flag2 = false;};
                
            });


    }
    else {

    ui.label(egui::RichText::new("gib hier bitte deine Daten ein:\n").size(23.0));
    ui.label(egui::RichText::new("bitte gib die Namen aller Gäste ein die nicht kommen können:").size(17.0));
    ui.horizontal_wrapped(|ui| {
    ui.label(egui::RichText::new("Name(n): ").size(17.0).color(egui::Color32::RED));
    ui.add(egui::TextEdit::singleline(&mut self.rsvp5).font(egui::FontId::proportional(17.0)));
    });
    
    
    ui.label(egui::RichText::new("\nhier kannst du allgemeine Kommentare eingeben:").size(17.0));
    ui.horizontal_wrapped(|ui| {
    ui.label(egui::RichText::new("Kommentar(e): ").size(17.0).color(egui::Color32::RED));
    ui.add(egui::TextEdit::singleline(&mut self.rsvp6).font(egui::FontId::proportional(17.0)));
    });
    ui.label("");
    if self.submitflag == false {ui.label("");}
    else {
    ui.label(egui::RichText::new("deine ABGESCHICKTEN Daten:").size(23.0));
    ui.label("");
    ui.horizontal_wrapped(|ui| {
    ui.label(egui::RichText::new("Name(n): ").size(17.0).color(egui::Color32::RED));
    ui.label(egui::RichText::new(self.decline_names.clone()).size(17.0));
    });
    
    
    ui.horizontal_wrapped(|ui| {
    ui.label(egui::RichText::new("Kommentar(e): ").size(17.0).color(egui::Color32::RED));
    ui.label(egui::RichText::new(self.decline_comments.clone()).size(17.0));
    });
    ui.label("");
    }
    if ui.button(egui::RichText::new("abschicken").size(17.0).color(egui::Color32::from_rgb(0, 183, 255))).clicked() {
    self.decline_names = self.rsvp5.clone();
    //self.submit_emails = self.rsvp2.clone();
    //self.submit_songs = self.rsvp3.clone();
    self.decline_comments = self.rsvp6.clone();
    let message = format!(r#"ABGESAGT::: names: {} ; comments: {}"#, self.decline_names, self.decline_comments);
    let body1 = message.as_bytes().to_vec();
    let request1 = ehttp::Request::post("https://ntfy.sh/woodland", body1);
    ehttp::fetch(request1, move |result: ehttp::Result<ehttp::Response>| {println!("Status code: {:?}", result.unwrap().status);});
    self.submitflag = true;
    }
    ui.label("");
    ui.label("");
    ui.horizontal_wrapped(|ui| {
        ui.label(egui::RichText::new("gehe zum ").size(17.0));
        if ui.button(egui::RichText::new("Zusageformular").size(17.0).color(egui::Color32::GREEN)).clicked() {self.rsvp_flag2 = true;};
        
    });



    }




}

                        }



                
                
                
                
            });
            });



                


            





            
            

            


            

            
           });
        });
        });
        ctx.request_repaint();
        
        }

        
    }
}


use eframe::egui::{self, *};

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> Color32 {
    let i = (h * 6.0).floor();
    let f = h * 6.0 - i;
    let p = v * (1.0 - s);
    let q = v * (1.0 - f * s);
    let t = v * (1.0 - (1.0 - f) * s);

    let (r, g, b) = match i as i32 % 6 {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        _ => (v, p, q),
    };

    Color32::from_rgb(
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8,
    )
}

pub fn cyber_button(
    ui: &mut Ui,
    text: &str,
    size: f32,
) -> Response {
    let desired_size = vec2(text.len() as f32 * size * 0.62, size + 10.0);

    let (rect, response) =
        ui.allocate_exact_size(desired_size, Sense::click());

    let painter = ui.painter();

    let time = ui.ctx().input(|i| i.time) as f32;

    let mut x = rect.min.x;

    for (i, ch) in text.chars().enumerate() {
        let hue = (time * 0.18 + i as f32 * 0.07) % 1.0;
        let color = hsv_to_rgb(hue, 1.0, 1.0);

        let pos = Pos2::new(x, rect.min.y);

        // glow
        for offset in [
            Vec2::new(-2.0, 0.0),
            Vec2::new(2.0, 0.0),
            Vec2::new(0.0, -2.0),
            Vec2::new(0.0, 2.0),
        ] {
            painter.text(
                pos + offset,
                Align2::LEFT_TOP,
                ch,
                FontId::proportional(size + 4.0),
                Color32::from_rgba_premultiplied(
                    color.r(),
                    color.g(),
                    color.b(),
                    40,
                ),
            );
        }

        // body
        painter.text(
            pos,
            Align2::LEFT_TOP,
            ch,
            FontId::proportional(size),
            color,
        );

        // white core
        painter.text(
            pos + Vec2::new(0.5, 0.5),
            Align2::LEFT_TOP,
            ch,
            FontId::proportional(size - 6.0),
            Color32::BLACK,
        );

        x += size * 0.62;
    }

    // optional hover outline
    if response.hovered() {
        painter.rect_stroke(
            rect.expand(4.0),
            4.0,
            Stroke::new(1.0, Color32::WHITE),
            StrokeKind::Outside,
        );
    }

    ui.ctx().request_repaint();

    response
}

use eframe::egui::{Color32, FontId, Pos2, Vec2};

pub fn cyber_rainbow_text(
    ui: &mut egui::Ui,
    text: &str,
    size: f32,
) {
    let time = ui.ctx().input(|i| i.time) as f32;
    let painter = ui.painter();

    let start = ui.cursor().min;

    let lines: Vec<&str> = text.lines().collect();

    // -------------------------------------------------
    // AUTO SCALE TO FIT AVAILABLE WIDTH
    // -------------------------------------------------

    let available_width = ui.available_width();

    let test_font = FontId::proportional(size);

    let widest_line = lines
        .iter()
        .map(|line| {
            painter
                .layout_no_wrap(
                    line.to_string(),
                    test_font.clone(),
                    Color32::WHITE,
                )
                .size()
                .x
        })
        .fold(0.0, f32::max);

    let scale = if widest_line > available_width {
        available_width / widest_line
    } else {
        1.0
    };

    let scaled_size = size * scale;

    let font_id = FontId::proportional(scaled_size);

    // -------------------------------------------------
    // RENDER
    // -------------------------------------------------

    for (line_idx, line) in lines.iter().enumerate() {

        let line_galley = painter.layout_no_wrap(
            line.to_string(),
            font_id.clone(),
            Color32::WHITE,
        );

        let line_width = line_galley.size().x;

        let mut x =
            start.x
            + (available_width - line_width) * 0.5;

        let y =
            start.y
            + line_idx as f32 * (scaled_size + 8.0);

        for (i, ch) in line.chars().enumerate() {

            let hue =
                (time * 0.18 + i as f32 * 0.07) % 1.0;

            let color = hsv_to_rgb(hue, 1.0, 1.0);

            let pos = Pos2::new(x, y);

            // glow
            for offset in [
                Vec2::new(-2.0, 0.0),
                Vec2::new(2.0, 0.0),
                Vec2::new(0.0, -2.0),
                Vec2::new(0.0, 2.0),
            ] {
                painter.text(
                    pos + offset,
                    egui::Align2::LEFT_TOP,
                    ch,
                    FontId::proportional(scaled_size + 4.0),
                    Color32::from_rgba_premultiplied(
                        color.r(),
                        color.g(),
                        color.b(),
                        40,
                    ),
                );
            }

            // body
            painter.text(
                pos,
                egui::Align2::LEFT_TOP,
                ch,
                font_id.clone(),
                color,
            );

            // white core
            painter.text(
                pos + Vec2::new(0.5, 0.5),
                egui::Align2::LEFT_TOP,
                ch,
                FontId::proportional(
                    (scaled_size - 6.0).max(1.0)
                ),
                Color32::BLACK,
            );

            // real glyph advance
            let glyph_galley = painter.layout_no_wrap(
                ch.to_string(),
                font_id.clone(),
                color,
            );

            x += glyph_galley.size().x;
        }
    }

    ui.add_space(
        lines.len() as f32 * (scaled_size + 8.0)
    );

    ui.ctx().request_repaint();
}
