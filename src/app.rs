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

    submitflag: bool,
    schickflag: bool,

    pic_name_en: String,
    pic_name_de: String,

    submit_names: String,
    submit_emails: String,
    submit_songs: String,
    


    

    

    
    
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

            submitflag: false,
            schickflag: false,
            
            pic_name_en: "who is painting? type your name ..".to_owned(),
            pic_name_de: "wer malt? lass uns deinen Namen wissen ..".to_owned(),

            submit_names: "".to_owned(),
            submit_emails: "".to_owned(),
            submit_songs: "".to_owned(),
            
            


            
            
            
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
            if ui.button(egui::RichText::new(self.button1.clone()).size(17.0).color(egui::Color32::from_rgb(218, 196, 55)).strong()).clicked() {
                if self.flag1 == true {self.flag1 = false;}
                else {self.flag1 = true;}
            }
            if ui.button(egui::RichText::new(self.button7.clone()).size(17.0).color(egui::Color32::from_rgb(0, 183, 255)).strong()).clicked() {
                if self.flag7 == true {self.flag7 = false;}
                else {self.flag7 = true;}
            }
            if ui.button(egui::RichText::new(self.button2.clone()).size(17.0).color(egui::Color32::from_rgb(218, 196, 55)).strong()).clicked() {
                if self.flag2 == true {self.flag2 = false;}
                else {self.flag2 = true;}
            }
            if ui.button(egui::RichText::new(self.button5.clone()).size(17.0).color(egui::Color32::from_rgb(0, 183, 255)).strong()).clicked() {
                if self.flag5 == true {self.flag5 = false;}
                else {self.flag5 = true;}
            }  
            if ui.button(egui::RichText::new(self.button4.clone()).size(17.0).color(egui::Color32::from_rgb(218, 196, 55)).strong()).clicked() {
                if self.flag4 == true {self.flag4 = false;}
                else {self.flag4 = true;}
            }  
            if ui.button(egui::RichText::new(self.button3.clone()).size(17.0).color(egui::Color32::from_rgb(218, 196, 55)).strong()).clicked() {
                if self.flag3 == true {self.flag3 = false;}
                else {self.flag3 = true;}
            }  
            if ui.button(egui::RichText::new(self.button6.clone()).size(17.0).color(egui::Color32::from_rgb(0, 183, 255)).strong()).clicked() {
                if self.flag6 == true {self.flag6 = false;}
                else {self.flag6 = true;}
            } 
            if ui.button(egui::RichText::new(self.button8.clone()).size(17.0).color(egui::Color32::from_rgb(218, 196, 55)).strong()).clicked() {
                if self.flag8 == true {self.flag8 = false;}
                else {self.flag8 = true;}
            }  
            if ui.button(egui::RichText::new(self.button9.clone()).size(17.0).color(egui::Color32::from_rgb(218, 196, 55)).strong()).clicked() {
                if self.flag9 == true {self.flag9 = false;}
                else {self.flag9 = true;}
            }  
            ui.label("");
            if ui.button(egui::RichText::new(self.formbutton.clone()).size(17.0).color(egui::Color32::from_rgb(222, 82, 196)).strong()).clicked() {
                if self.formflag == true {self.formflag = false;}
                else {self.formflag = true;}
            }  







            
        });


        





            
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {


             

            
            // The central panel the region left after adding TopPanel's and SidePanel's
            //ui.heading("Woodland Wedding 2026 - kimashi == Kim, Matthias and Yoshi");
            
            ui.heading(egui::RichText::new("Woodland Wedding 2026 \nKIMASHI = Kim, Matthias and Yoshi\n").size(34.0).color(egui::Color32::DARK_GREEN).strong());
            
                
            egui_extras::install_image_loaders(ctx);
            let collage = egui::include_image!("../assets/Collage_Verotterung_Zuschnitt2.jpg");
            let desired_size = egui::vec2(340.0, 340.0); 
            
            ui.add(egui::Image::new(collage).fit_to_exact_size(desired_size));

            if self.language_flag == true {
                
                ui.label(egui::RichText::new("\n... download completed! :)\n\nTwo years have passed since we, Kim and Matthi, got married. Together with our newly minted family member Yoshi, we have become a whole: KIMASHI :)\n\nTherefore, the time has come to have a big celebration with all of you, our friends and family from near and far. We have secured a marvelous venue amidst a forest situated directly by a lake. From Friday till Sunday, June 12th - 14th in 2026, we are looking forward to hosting you. – There will be great food, a whirlpool, a little sauna and fantastic music. A key moment of the weekend will be a ceremony which is meant to formally commemorate the start of our family.\n\nWe're organizing almost everything ourselves — from food and decorations to music. And for that, we need your support! Let us know if there's something specific you'd like to contribute. Tasks like cleaning up, cooking, dishwashing, etc., will be planned in advance so that all the work is divided up between all of us fairly  — nobody will be forced to do anything that they don't want to do. But remember, in every job that must be done there is an element of fun! 😉\n\nPlease also take a look at the rest of the website. It contains important information and a few fun surprises to be discovered. Also, we will be posting updates in the respective sections as we get closer to the event. So, please check back.\n\nWe sincerely hope that all of you will be able to make it. Please RSVP by January 7th, 2026 as this is vital for our planning.").size(17.0));    
            }
            else {
                
                ui.label(egui::RichText::new("\n... Download abgeschlossen! :)\n\nVor inzwischen 2 Jahren haben wir, Kim und Matthi, geheiratet. Zusammen mit unserem frisch gebackenen Familienmitglied Yoshi sind wir jetzt komplett: KIMASHI :)\n\nDarum ist jetzt die Zeit gekommen, mit euch allen, klein und groß, von nah und fern, Freunde und Familie, ein großes Fest zu feiern. Dafür haben wir einen zauberhaften Ort mitten im Wald direkt an einem Badesee gefunden. Von Freitag bis Sonnatg, den 12. bis 14. Juni 2026, gibt es lekkres Essen und Getränke, einen Whirlpool, eine kleine Sauna und ganz viel Musik. Ein zentraler Moment wird am Samstag eine Zeremonie sein, die unsere Familiengründung feierlich besiegelt.\n\nVom Essen über Deko bis zur Musik organisieren wir fast alles selbst. Und dafür brauchen wir eure Unterstützung! Sagt uns gerne Bescheid, wenn ihr etwas Bestimmtes beitragen möchtet. Aufgaben wie Aufräumen, Kochen, Abspülen u.ä. werden wir vorab so organisieren, dass sie fair verteilt werden und niemand etwas machen muss, was nicht wenigstens ein bisschen Spaß macht. In every job that must be done there is an element of fun! 😉\n\nBitte guckt euch auch den Rest der Website an. Ihr findet dort weitere wichtige Infos und ein paar lustige Überraschungen. Wenn das Wochenende näher rückt, schaut für aktuelle Infos gern wieder vorbei!\n\nSchickt uns bitte bis zum 07.01.2026 eure feste Zu- oder Absage. Wir hoffen sehr, dass alle dabei sein können und freuen uns auf euch.").size(17.0));                
                }
            if ui.button(egui::RichText::new(self.formbutton.clone()).size(17.0).color(egui::Color32::from_rgb(222, 82, 196)).strong()).clicked() {
                    if self.formflag == true {self.formflag = false;}
                    else {self.formflag = true;}
                }
            
            
            egui::Window::new(self.button1.clone()).open(&mut self.flag1).show(ctx, |ui| {
                egui::ScrollArea::both().show(ui, |ui| {

                if self.language_flag == true {

ui.label(egui::RichText::new("You can reach the venue in a number of different ways: using public transport, your own car, a rental car or by taxi/Uber. Here's an overview:\n").size(17.0)); 

ui.label(egui::RichText::new("PUBLIC TRANSPORT might be a bit complicated:").size(23.0).color(egui::Color32::RED).strong().underline()); 
ui.label("");
ui.label(egui::RichText::new("Around 2.5 hours from Berlin Ostkreuz via regional train and two buses to Chossewitz, then a 20-minute walk (1.5 km).\nCost: approx. 15 euros per person.\n").size(17.0)); 

ui.label(egui::RichText::new("TAXI or UBER:").size(23.0).color(egui::Color32::RED).strong().underline()); 
ui.label("");
ui.label(egui::RichText::new("If you book a large taxi or Uber that accomodates 6–8 people, the cost is about  40 euros per person one way.Duration: approx. 1.5 hours. If you’re interested in this option and need help organizing it, please contact us before April 1st.\n").size(17.0)); 

ui.label(egui::RichText::new("CAR:").size(23.0).color(egui::Color32::RED).strong().underline()); 
ui.label("");
ui.label(egui::RichText::new("Duration: approx. 1.5 hours, with parking available directly at the venue.\nRenting a car for 3 days for 4–5 people costs about 50 euros per person for the round trip, including gas.").size(17.0));                     
ui.horizontal(|ui| {
ui.label(egui::RichText::new("Important: mind the exact ").size(17.0)); 
ui.add(
    egui::Hyperlink::from_label_and_url(
        egui::RichText::new("ROUTE").size(17.0).color(egui::Color32::RED),
        "https://maps.app.goo.gl/WtrCENyrHQ23ziUL9",
    )
);
});


ui.label(egui::RichText::new("\nHELICOPTER:").size(23.0).color(egui::Color32::RED).strong().underline());
ui.label("");
ui.horizontal(|ui| {
ui.label(egui::RichText::new("Sadly unaffordable. However, here is the link to the classic").size(17.0)); 
ui.add(
    egui::Hyperlink::from_label_and_url(
        egui::RichText::new("helicopter game").size(17.0).color(egui::Color32::RED),
        "https://maps.app.goo.gl/WtrCENyrHQ23ziUL9",
    )
);
});


}
                




else {

ui.label(egui::RichText::new("Zum Gelände kommt man auf allen erdenklichen Wegen, mit Bus&Bahn, dem eigenen oder gemieteten Auto oder mit Taxi bzw. Uber. Hier ein Überblick:\n").size(17.0)); 

ui.label(egui::RichText::new("etwas umständlich mit ÖFFIS:").size(23.0).color(egui::Color32::RED).strong().underline()); 
ui.label("");
ui.label(egui::RichText::new("Ungefähr 2.5 Stunden von Berlin Ostkreuz mit Regio und zwei Bussen bis Chossewitz, von dort 20 Minuten (1.5km) laufen. Kosten circa 15 Euro pro Person.\n").size(17.0)); 

ui.label(egui::RichText::new("TAXI oder UBER:").size(23.0).color(egui::Color32::RED).strong().underline()); 
ui.label("");
ui.label(egui::RichText::new("Ein Großraumtaxi/ -Uber für circa 6-8 Leute kostet circa  40 Euro pro Person und Fahrt. Dauer circa 1.5 Stunden. Falls ihr Interesse an dieser Option habt und Hilfe beim Organisieren braucht, wendet euch bitte bis zum 01.04.26 an uns.\n").size(17.0)); 

ui.label(egui::RichText::new("AUTO:").size(23.0).color(egui::Color32::RED).strong().underline()); 
ui.label("");
ui.label(egui::RichText::new("Parkplätze direkt vor dem Gelände. Ein Auto mieten für 3 Tage und 4-5 Leute kostet circa 50 Euro pro Person für hin&zurück, inklusive Benzin. Dauer circa 1.5 Stunden.").size(17.0)); 
ui.horizontal(|ui| {
ui.label(egui::RichText::new("WICHTIG: exakte ").size(17.0)); 
ui.add(
    egui::Hyperlink::from_label_and_url(
        egui::RichText::new("ROUTE").size(17.0).color(egui::Color32::RED),
        "https://maps.app.goo.gl/WtrCENyrHQ23ziUL9",
    )
);
});


ui.label(egui::RichText::new("\nHELIKOPTER:").size(23.0).color(egui::Color32::RED).strong().underline());
ui.label("");
ui.horizontal(|ui| {
ui.label(egui::RichText::new("fast unbezahlbar teuer, dafür hier der Link zum").size(17.0));
ui.add(
    egui::Hyperlink::from_label_and_url(
        egui::RichText::new("Helikopterspiel").size(17.0).color(egui::Color32::RED),
        "https://maps.app.goo.gl/WtrCENyrHQ23ziUL9",
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
                    
                    ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Thursday: ").size(17.0).color(egui::Color32::RED).strong()); ui.label(egui::RichText::new("helping with setup (optional, please coordinate with us)\n").size(17.0));
                    });
                    
                    ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Friday: ").size(17.0).color(egui::Color32::RED).strong()); ui.label(egui::RichText::new("arrive and be welcomed from 3 PM onward\n").size(17.0));
                    });
                    
                    ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Saturday: ").size(17.0).color(egui::Color32::RED).strong()); ui.label(egui::RichText::new("lots of fun and boundless joy!!\n").size(17.0));
                    });
                    
                    ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Sunday: ").size(17.0).color(egui::Color32::RED).strong()); ui.label(egui::RichText::new("check out of the bungalows by 12 PM. Help with cleanup if possible until early evening\n").size(17.0));
                    });
                    
                    
                    ui.label(egui::RichText::new("Accommodation:\n").size(23.0).color(egui::Color32::RED).strong().underline());
                    
                    ui.label(egui::RichText::new("There are small bungalows directly on the event grounds for about half of the guests. These are reserved for the older generations and families with small children. We’ll let you know in the coming months if we’ve planned a spot in a bungalow for you. If you need a bungalow for reasons unknown to us, feel free to reach out directly.\n\nFor everyone else, there’s plenty of space for your own tents – complete with nature vibes and a festival feeling. Clean indoor toilets and showers are accessible to all.\n\nIf you don’t want to stay on the event grounds, there are guesthouses and hotels in the surrounding villages. If you need help with arrangements, feel free to contact us – you're welcome to use the contact button on the website.").size(17.0));
                    
                    }
                                    
                    else {
                    
                    ui.label(egui::RichText::new("Genaueres zum Ablauf findet ihr in der nächsten Zeit hier. Vorab nur ganz grob:\n").size(17.0));
                    
                    ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Donnerstag: ").size(17.0).color(egui::Color32::RED).strong()); ui.label(egui::RichText::new("Mithelfen beim Aufbauen (optional, in Absprache mit uns)\n").size(17.0));
                    });
                    
                    ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Freitag: ").size(17.0).color(egui::Color32::RED).strong()); ui.label(egui::RichText::new("ab 15 Uhr ankommen und empfangen werden\n").size(17.0));
                    });
                    
                    ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Samstag: ").size(17.0).color(egui::Color32::RED).strong()); ui.label(egui::RichText::new("großer Spaß und unbändige Freude -\n").size(17.0));
                    });
                    
                    ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Sonntag: ").size(17.0).color(egui::Color32::RED).strong()); ui.label(egui::RichText::new("Auschecken aus den Bungalows bis 12 Uhr; Mithelfen beim Aufräumen wenn möglich bis zum frühen Abend\n").size(17.0));
                    });
                    
                    ui.label(egui::RichText::new("Übernachtung:\n").size(23.0).color(egui::Color32::RED).strong().underline());
                    
                    ui.label(egui::RichText::new("Es gibt direkt auf dem Veranstaltungsgelände kleine Bungalows für etwa die Hälfte der Gäste. Diese sind für die früheren Jahrgänge und Familien mit kleinen Kindern vorgesehen. Wir sagen euch in den nächsten Monaten Bescheid, wenn wir für euch einen Platz im Bungalow geplant haben. Solltet ihr aus uns unbekannten Gründen einen Bungalow brauchen, meldet euch gerne direkt bei uns.\n\nFür alle anderen gibt es reichlich Platz für eigene Zelte inkl. Naturromantik und Festival-Feeling. Feste Toiletten und Duschen sind für alle zugänglich.\n\nWenn ihr nicht auf dem Gelände übernachten möchtet, gibt es in den umliegenden Orten Pensionen und Hotels. Wenn ihr Hilfe beim Organisieren braucht, wendet euch an uns, gerne auch über den Kontakt-Button der Website.").size(17.0));

                }


                
            });
            });


            egui::Window::new(self.button5.clone())
                .open(&mut self.flag5)
                .show(ctx, |ui| {
                    egui::ScrollArea::both().show(ui, |ui| {

                if self.language_flag == true {
                    ui.label(egui::RichText::new("paint a picture and send it to us :)").size(17.0));  
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
                }
                    });


                    
                                              }
                else {
                    
                    
                     
                    ui.label(egui::RichText::new("mal ein Bild uns schick es uns :)").size(17.0));  
                    ui.horizontal(|ui| {
                        //ui.text_edit_singleline(&mut self.pic_name_de);
                        ui.add(egui::TextEdit::singleline(&mut self.pic_name_en).font(egui::FontId::proportional(17.0)));
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
                }
                    });
                     }

                self.painting_app.ui(ui, self.language_flag);
                

               
                    
                






                    
                    
                });
                });


            egui::Window::new(self.button4.clone()).open(&mut self.flag4).show(ctx, |ui| {

                egui::ScrollArea::both().show(ui, |ui| {

                    if self.language_flag == true {
                    ui.label(egui::RichText::new("More detailed info about how the event will be organized will be shared here — so feel free to check back in again. We would love to hear your creative ideas for the following areas:\n\n").size(17.0));

                    ui.label(egui::RichText::new("Decorations:\n").size(23.0).color(egui::Color32::RED).strong().underline());
                    
                    ui.label(egui::RichText::new("We’d love to craft decorations together with you over the coming months s. We already have a few ideas and are excited for more inspiration from you. Get in touch with us if you’d like to join in!\n\n").size(17.0));
                    
                    ui.label(egui::RichText::new("Music:\n").size(23.0).color(egui::Color32::RED).strong().underline());
                    
                    ui.label(egui::RichText::new("We want to fill the weekend with lots of different kinds of music— a cozy playlist for morning coffee, live music at sunset, and DJ sets with techno and goa taking us through the night in the Party Cabin. If you’d like to contribute in any way, please reach out as soon as possible!\n\n").size(17.0));
                    
                    ui.label(egui::RichText::new("Kids Area:\n").size(23.0).color(egui::Color32::RED).strong().underline());
                    
                    ui.label(egui::RichText::new("We want your kids to feel welcome and be able to join in the fun. We will be organizing a special area for them where they can play and frolic about. We would love for the parents to participate in creating a fun space. Maybe you know some fun games or have some cool toys you can bring?\n\n").size(17.0));
                    
                    ui.label(egui::RichText::new("Kitchen & Co.:\n").size(23.0).color(egui::Color32::RED).strong().underline());
                    
                    ui.label(egui::RichText::new("Cooking for around 100 guests will be a beautiful team effort! We’ll have everything well prepared and will plan each meal together with a few lead coordinators. Of course, this is where we’ll need the most support from all of you. A few weeks before the event, you’ll find a schedule here where you can sign up for tasks like chopping, serving, dishwashing, etc.").size(17.0));
                                        }
                else {
                    ui.label(egui::RichText::new("Im Laufe der Zeit wird es hier noch mehr Infos zur konkreten Orga geben. Schaut also gerne nochmal rein. Folgende Bereiche sind schon klar und freuen sich über eure kreativen Ideen:\n\n").size(17.0));

                    ui.label(egui::RichText::new("Deko:\n").size(23.0).color(egui::Color32::RED).strong().underline());
                    
                    ui.label(egui::RichText::new("Wir wollen zusammen mit euch über die nächste Zeit Deko basteln. Wir haben schon ein paar Ideen und freuen uns über noch mehr Inspiration von euch. Meldet euch bei uns, wenn ihr mitmachen wollt!\n\n").size(17.0));
                    
                    ui.label(egui::RichText::new("Musik:\n").size(23.0).color(egui::Color32::RED).strong().underline());
                    
                    ui.label(egui::RichText::new("Wir wollen das Wochenende mit richtig viel verschiedener Musik füllen. Eine gemütliche Playlist zum Frühstückskaffee, Live Musik zum Sonnenuntergang, mit Techno und Goa DJ-Sets durch die Nacht im Party-Kabuff. Wenn ihr hier etwas beitragen könnt, meldet euch gerne so bald wie möglich!\n\n").size(17.0));
                    
                    ui.label(egui::RichText::new("Kids Area:\n").size(23.0).color(egui::Color32::RED).strong().underline());
                    
                    ui.label(egui::RichText::new("Eure Kinder sind natürlich herzlich willkommen und sollen sich bei uns wohl fühlen. Dafür wollen wir einen Kinderbereich einrichten, wo die Kleinen spielen, toben und Spaß haben können. Es wäre großartig, wenn die Eltern sich hier mit kreativen Ideen einbringen.\n\n").size(17.0));
                    
                    ui.label(egui::RichText::new("Küche & Co:\n").size(23.0).color(egui::Color32::RED).strong().underline());
                    
                    ui.label(egui::RichText::new("Für ca. 100 Gäste Essen zuzubereiten, wird ein wunderschöner Kraftakt! Wir werden alles gut vorbereiten und jede Mahlzeit zusammen mit ein paar Hauptverantwortlichen planen. Natürlich brauchen wir hier am meisten Unterstützung von euch allen. Ein paar Wochen vor der Feier findet ihr hier einen Schichtplan, wo ihr euch zum Schnippeln, Servieren, Spülen etc. eintragen könnt.").size(17.0));

                     }
                
            });
                
                
                
            });

                

            egui::Window::new(self.button3.clone()).open(&mut self.flag3).show(ctx, |ui| {
                egui::ScrollArea::both().show(ui, |ui| {

                //if self.language_flag == true {ui.label("You’re invited to enjoy food and drinks all weekend, and there will be no cost for staying overnight in a tent or bungalow. What matters most is that everyone can be there and be comfortable.\n\nOf course our pockets will feel a bit empty after the event. If it’s possible for you to help fill them up again, we would be very grateful for any financial contribution — no matter how great or small the amount.\n\nIf you would like to make a contribution, please send it anytime to:\n\nPaypal: woodland.wedding@pm.me   Subject: Woodland Wedding\n\nBank Account: Kimberley Hofer & Matthias Hofer, DE...  Subject: Woodland Wedding");}
                //else {ui.label("Ihr seid das ganze Wochenende auf Essen und Trinken eingeladen, auch für die Übernachtung im Zelt oder Bungalow fallen prinzipiell keine Kosten für euch an. Das Wichtigste ist, dass alle dabei sein können und sich wohl fühlen.\n\nNatürlich wird unser Geldbeutel nach diesem Event ganz schön Magenknurren haben. Wenn es für euch möglich ist, sind wir über finanzielles Futter in jeder Höhe sehr dankbar.\n\nGerne jederzeit an:\n\nPaypal: woodland.wedding@pm.me   Betreff: Woodland Wedding\n\nKonto: Kimberley Hofer & Matthias Hofer, DE...  Betreff: Woodland Wedding");}

                if self.language_flag == true {

                ui.label(egui::RichText::new("You’re invited to enjoy food and drinks all weekend, and there will be no cost for staying overnight in a tent or bungalow. What matters most is that everyone can be there and be comfortable.\n\nOf course our pockets will feel a bit empty after the event. If it’s possible for you to help fill them up again, we would be very grateful for any financial contribution — no matter how great or small the amount.\n\nIf you would like to make a contribution, please send it anytime to:\n").size(17.0));
                
                ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Paypal: ").size(17.0).color(egui::Color32::RED).strong()); ui.label(egui::RichText::new("woodland.wedding@pm.me   Subject: Woodland Wedding\n").size(17.0));
                });
                
                ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Bank Account: ").size(17.0).color(egui::Color32::RED).strong()); ui.label(egui::RichText::new("Kimberley Hofer & Matthias Hofer, DE...  Subject: Woodland Wedding").size(17.0));
                });
                
                }
                                
                
                else {
                
                ui.label(egui::RichText::new("Ihr seid das ganze Wochenende auf Essen und Trinken eingeladen, auch für die Übernachtung im Zelt oder Bungalow fallen prinzipiell keine Kosten für euch an. Das Wichtigste ist, dass alle dabei sein können und sich wohl fühlen.\n\nNatürlich wird unser Geldbeutel nach diesem Event ganz schön Magenknurren haben. Wenn es für euch möglich ist, sind wir über finanzielles Futter in jeder Höhe sehr dankbar.\n\nGerne jederzeit an:\n").size(17.0));
                
                ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Paypal: ").size(17.0).color(egui::Color32::RED).strong()); ui.label(egui::RichText::new("woodland.wedding@pm.me   Betreff: Woodland Wedding\n").size(17.0));
                });
                
                ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Konto: ").size(17.0).color(egui::Color32::RED).strong()); ui.label(egui::RichText::new("Kimberley Hofer & Matthias Hofer, DE...  Betreff: Woodland Wedding").size(17.0));
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
                
                ui.label(egui::RichText::new("Dress code:\n").size(23.0).color(egui::Color32::RED).strong().underline());
                
                ui.label(egui::RichText::new("no need for formal evening wear unless it makes you happy. We love colorful, glittery, shiny and extravagant outfits. But pajamas, pirate costumes and potato sacks are also favorites. So – feel free to express yourself!\n\n").size(17.0));
                
                ui.label(egui::RichText::new("More of what to bring will be posted here in our next update, so please check back in with us.").size(17.0));
                
                
                }
                                
                
                else {
                
                ui.label(egui::RichText::new("Dress code:\n").size(23.0).color(egui::Color32::RED).strong().underline());
                
                ui.label(egui::RichText::new("Förmliche Abendgarderobe ist nicht nötig, es sei denn sie macht euch glücklich. Wir freuen uns immer über farbenfrohe, glitzernde, schillernde und extravagante Outfits. Aber auch Pyjamas, Piratenkostüme und Kartoffelsäcke gehören zu unseren Favoriten. Ihr habt also die freie Wahl!\n\n").size(17.0));
                
                ui.label(egui::RichText::new("Alles Weitere findet ihr nach unserem nächsten Update hier. Bitte schaut nochmal rein!").size(17.0));
                
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

ui.label(egui::RichText::new("please enter your data below:\n").size(23.0));

ui.label(egui::RichText::new("please enter the names of all people you want to confirm:").size(17.0));
ui.horizontal(|ui| {
ui.label(egui::RichText::new("name(s): ").size(17.0).color(egui::Color32::RED));
ui.add(egui::TextEdit::singleline(&mut self.rsvp1).font(egui::FontId::proportional(17.0)));

});
ui.label(egui::RichText::new("\nplease tell us all email-addresses we can send you updates to:").size(17.0));
ui.horizontal(|ui| {
ui.label(egui::RichText::new("email-address(es): ").size(17.0).color(egui::Color32::RED));
ui.add(egui::TextEdit::singleline(&mut self.rsvp2).font(egui::FontId::proportional(17.0)));

});
ui.label(egui::RichText::new("\ntell us some songs you would appreciate to hear during the event:").size(17.0));
ui.horizontal(|ui| {
ui.label(egui::RichText::new("like-to-hear song(s): ").size(17.0).color(egui::Color32::RED));
ui.add(egui::TextEdit::singleline(&mut self.rsvp3).font(egui::FontId::proportional(17.0)));

});
ui.label("");

if self.submitflag == false {ui.label("");}
else {

ui.label(egui::RichText::new("your SUBMITTED data:").size(23.0));
ui.label("");
ui.horizontal(|ui| {
ui.label(egui::RichText::new("name(s): ").size(17.0).color(egui::Color32::RED));
ui.label(egui::RichText::new(self.submit_names.clone()).size(17.0));
});
ui.horizontal(|ui| {
ui.label(egui::RichText::new("email-address(es): ").size(17.0).color(egui::Color32::RED));
ui.label(egui::RichText::new(self.submit_emails.clone()).size(17.0));
});
ui.horizontal(|ui| {
ui.label(egui::RichText::new("song(s): ").size(17.0).color(egui::Color32::RED));
ui.label(egui::RichText::new(self.submit_songs.clone()).size(17.0));
});

ui.label("");
}


if ui.button(egui::RichText::new("submit").size(17.0)).clicked() {
self.submit_names = self.rsvp1.clone();
self.submit_emails = self.rsvp2.clone();
self.submit_songs = self.rsvp3.clone();
let message = format!(r#"names: {} ; emails: {} ; songs: {}"#, self.submit_names, self.submit_emails, self.submit_songs);
let body1 = message.as_bytes().to_vec();
let request1 = ehttp::Request::post("https://ntfy.sh/woodland", body1);
ehttp::fetch(request1, move |result: ehttp::Result<ehttp::Response>| {println!("Status code: {:?}", result.unwrap().status);});
self.submitflag = true;
}
                    
                }



else {
ui.label(egui::RichText::new("gib hier bitte deine Daten ein:\n").size(23.0));
ui.label(egui::RichText::new("bitte gib die Namen aller Gäste ein die du bestätigen willst:").size(17.0));
ui.horizontal(|ui| {
ui.label(egui::RichText::new("Name(n): ").size(17.0).color(egui::Color32::RED));
ui.add(egui::TextEdit::singleline(&mut self.rsvp1).font(egui::FontId::proportional(17.0)));

});
ui.label(egui::RichText::new("\nbitte gib uns alle eMail-Adressen an die wir Updates schicken können:").size(17.0));
ui.horizontal(|ui| {
ui.label(egui::RichText::new("email-Adresse(n): ").size(17.0).color(egui::Color32::RED));
ui.add(egui::TextEdit::singleline(&mut self.rsvp2).font(egui::FontId::proportional(17.0)));

});
ui.label(egui::RichText::new("\nwenn du dich freuen würdest spezielle Lieder auf der Party zu hören, gib sie bitte hier ein:").size(17.0));
ui.horizontal(|ui| {
ui.label(egui::RichText::new("Wunschlied(er): ").size(17.0).color(egui::Color32::RED));
ui.add(egui::TextEdit::singleline(&mut self.rsvp3).font(egui::FontId::proportional(17.0)));

});
ui.label("");


if self.submitflag == false {ui.label("");}
else {

ui.label(egui::RichText::new("deine ABGESCHICKTEN Daten:").size(23.0));
ui.label("");
ui.horizontal(|ui| {
ui.label(egui::RichText::new("Name(n): ").size(17.0).color(egui::Color32::RED));
ui.label(egui::RichText::new(self.submit_names.clone()).size(17.0));
});
ui.horizontal(|ui| {
ui.label(egui::RichText::new("email-Adresse(n): ").size(17.0).color(egui::Color32::RED));
ui.label(egui::RichText::new(self.submit_emails.clone()).size(17.0));
});
ui.horizontal(|ui| {
ui.label(egui::RichText::new("Wunschlied(er): ").size(17.0).color(egui::Color32::RED));
ui.label(egui::RichText::new(self.submit_songs.clone()).size(17.0));
});

ui.label("");
}


if ui.button(egui::RichText::new("abschicken").size(17.0)).clicked() {
self.submit_names = self.rsvp1.clone();
self.submit_emails = self.rsvp2.clone();
self.submit_songs = self.rsvp3.clone();
let message = format!(r#"names: {} ; emails: {} ; songs: {}"#, self.submit_names, self.submit_emails, self.submit_songs);
let body1 = message.as_bytes().to_vec();
let request1 = ehttp::Request::post("https://ntfy.sh/woodland", body1);
ehttp::fetch(request1, move |result: ehttp::Result<ehttp::Response>| {println!("Status code: {:?}", result.unwrap().status);});
self.submitflag = true;
}
}



                
                
                
                
            });
            });



                


            





            
            

            


            

            
           });
        });
        ctx.request_repaint();
        
        }

        
    }
}

