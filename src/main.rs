#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use eframe::egui;

enum ItemPriority {
    Low,
    Medium,
    High,
    Critical,
}

enum ItemStatus {
    New,
    Open,
    Closed,
}

enum UserType {
    Client,
    Admin,
}

#[allow(clippy::upper_case_acronyms)]
type DateTime = String;

struct ItemComment {
    commenter: String,
    message: String,
    time_created: DateTime,
}

struct Issue {
    name: String,
    description: String,
    reporter: String,
    comment_thread: Vec<ItemComment>,
    priority: ItemPriority,
    status: ItemStatus,
    time_created: DateTime,
}

struct User {
    email: String,
    password: String,
    username: String,
    user_type: UserType,
}

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Rust Issue Tracker",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<ClientApp>::default())
        }),
    )
}

// Defines the variables stored within the app
struct ClientApp {
    users: Vec<User>,
    issues: Vec<Issue>,
    loggedInAs: usize,
    age: u32,
}

// Instantiates the default variables.
impl Default for ClientApp {
    fn default() -> Self {
        Self {
            users: Vec::from([
                User {
                    email: String::from("admin@test.com"),
                    password: String::from("encrypted-password"),
                    username: String::from("TheBeesKnees"),
                    user_type: UserType::Admin,
                },
                User {
                    email: String::from("client@test.com"),
                    password: String::from("encrypted-password"),
                    username: String::from("BiteAtTheHand"),
                    user_type: UserType::Client,
                },
            ]),
            loggedInAs: 0,
            issues: Vec::new(),
            age: 21,
        }
    }
}

// Defines the interface and interactions
impl eframe::App for ClientApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rust Issue Tracker");
            ui.horizontal(|ui| {
                let name_label = ui.label("Currently logged in as: ");
                ui.text_edit_singleline(&mut self.users[self.loggedInAs].username)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                self.age += 1;
            }
            ui.label(format!("Age {}",self.age));

            // ui.image(egui::include_image!(
            // ));
        });
    }
}

// USE IMPL
// USE A COMPOSITE
