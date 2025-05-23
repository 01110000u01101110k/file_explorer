#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::{egui, egui::vec2, egui::RichText, egui::CursorIcon::PointingHand};
use display_info::DisplayInfo;
use windows::{
    Win32::Storage::FileSystem::GetLogicalDrives,
};

use std::{
    env,
    path::PathBuf,
    sync::Arc,
    process::Command,
    env::consts::OS,
    fs::DirEntry
};

fn main() -> eframe::Result {
    //env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let display_info = DisplayInfo::all().unwrap(); // getting information about the display

    let icon = include_bytes!("../assets/logo.png");
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([
                (display_info[0].width / 2) as f32,
                (display_info[0].height / 2) as f32,
            ])
            .with_icon(
                // display our icon
                Arc::new(eframe::icon_data::from_png_bytes(&icon[..]).unwrap()),
            ),
        ..Default::default()
    };

    eframe::run_native(
        "File Explorer",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<FileExplorerApp>::default())
        }),
    )
}

 #[derive(Debug)]
struct FileExplorerApp {
    current_dir: PathBuf,
    program_root: PathBuf,
    is_disk_selection: bool,
    disk_list: Vec<char>,
    search: String,
}

impl Default for FileExplorerApp {
    fn default() -> Self {
        let path = env::current_dir().unwrap_or_default();

        Self {
            current_dir: path.clone(),
            program_root: path,
            is_disk_selection: false,
            disk_list: Vec::new(),
            search: String::new(),
        }
    }
}

impl FileExplorerApp {
    fn open_file(&self, dir_entry: DirEntry) {
        match OS {
            "windows" => {
                Command::new("cmd")
                    .args(&[
                        "/C", 
                        "explorer", 
                        dir_entry.path().to_str().unwrap()
                    ])
                    .status()
                    .expect(&format!("could not be opened {}", dir_entry.path().to_str().unwrap()));
            },
            "linux" => {
                Command::new("xdg-open")
                    .arg(dir_entry.path().to_str().unwrap())
                    .status()
                    .expect(&format!("could not be opened {}", dir_entry.path().to_str().unwrap()));
            },
            _ => {
                println!("операційна система не підтримується");
            }
        }
    }

    fn open_folder(&mut self, dir_entry: DirEntry) {
        self.current_dir = dir_entry.path();
    }

    fn prev_folder(&mut self) {
        let to_prev_folder = self.current_dir.pop();

        println!("{:#?}", self.current_dir);

        if !to_prev_folder {
            self.is_disk_selection = true;

            self.save_disk_list();
        } else {
            if self.is_disk_selection {
                self.is_disk_selection = false;
            }
        }
    }

    fn save_disk_list(&mut self) {
        match OS {
            "windows" => {
                let drives_mask = unsafe { GetLogicalDrives() };

                self.disk_list = Vec::new();

                for i in 0..26 {
                    if drives_mask & (1 << i) != 0 {
                        let letter = (b'A' + i as u8) as char;

                        self.disk_list.push(letter);
                    }
                }
            },
            "linux" => {
                // todo - реалізувати версію для лінукс систем
                /*let file = File::open("/proc/mounts")?;
                let reader = BufReader::new(file);
                let mut mount_points = Vec::new();

                for line in reader.lines() {
                    if let Ok(line) = line {
                        let fields: Vec<&str> = line.split_whitespace().collect();
                        
                        if fields.len() >= 2 {
                            mount_points.push(fields[1].to_string());
                        }
                    }
                }*/
            },
            _ => {
                println!("операційна система не підтримується");
            }
        }
    }
}

impl eframe::App for FileExplorerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("File Explorer");

            ui.add_space(5.0);
            ui.add(egui::Separator::default());
            ui.add_space(5.0);

            ui.horizontal(|ui| {
                let name_label = ui.label("Search: ");

                ui.text_edit_singleline(&mut self.search).labelled_by(name_label.id);

                let search_button = ui.add(egui::Button::image(
                    egui::Image::new(
                        format!("file://{}/{}", self.program_root.to_str().unwrap(), "assets/Font_Awesome_Icons/solid/magnifying-glass.svg")
                    ).max_size(vec2(20.0, 20.0)),
                ));

                search_button.clone().on_hover_cursor(PointingHand);

                if search_button.clicked() {
                    
                }
            });

            ui.horizontal(|ui| {
                let prev_folder_button = ui.add(egui::Button::image(
                    egui::Image::new(
                        format!("file://{}/{}", self.program_root.to_str().unwrap(), "assets/Font_Awesome_Icons/solid/arrow-left.svg")
                    ).max_size(vec2(20.0, 20.0)),
                ));

                prev_folder_button.clone().on_hover_cursor(PointingHand);

                if prev_folder_button.clicked() {
                    self.prev_folder();
                }
            });

            ui.add_space(5.0);
            ui.add(egui::Separator::default());
            ui.add_space(5.0);

            if self.is_disk_selection {
                for disk in &self.disk_list {
                    let disk_button = ui.add(egui::Button::image_and_text(
                        egui::Image::new(format!("file://{}/{}", self.program_root.to_str().unwrap(), "assets/Font_Awesome_Icons/solid/hard-drive.svg")),
                        RichText::new(disk.to_string()).size(14.0),
                    ));

                    disk_button.clone().on_hover_cursor(PointingHand);

                    if disk_button.double_clicked() {
                        let mut path = PathBuf::from(format!("{}:\\", disk));
                    
                        self.current_dir = path;
                        self.is_disk_selection = false;
                    }
                }
            } else {
                for dir_element in self.current_dir.read_dir().unwrap() {
                    if let Ok(dir_element) = dir_element {
                        if dir_element.metadata().unwrap().is_dir() {
                            let dir_button = ui.add(egui::Button::image_and_text(
                                egui::Image::new(format!("file://{}/{}", self.program_root.to_str().unwrap(), "assets/Font_Awesome_Icons/solid/folder.svg")),
                                RichText::new(dir_element.file_name().to_str().unwrap()).size(14.0),
                            ));

                            dir_button.clone().on_hover_cursor(PointingHand);

                            if dir_button.double_clicked() {
                                self.open_folder(dir_element);
                            }
                        } else if dir_element.metadata().unwrap().is_file() {
                            let file_button = ui.add(egui::Button::image_and_text(
                                egui::Image::new(format!("file://{}/{}", self.program_root.to_str().unwrap(), "assets/Font_Awesome_Icons/solid/file.svg")),
                                RichText::new(dir_element.file_name().to_str().unwrap()).size(14.0),
                            ));

                            file_button.clone().on_hover_cursor(PointingHand);

                            if file_button.double_clicked() {
                                self.open_file(dir_element);
                            }
                        }
                    } else {

                    }
                }
            }
        });
    }
}