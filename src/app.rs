use crate::disk::disk_list::get_disk_list;

use eframe::{
    egui, 
    egui::vec2, 
    egui::pos2, 
    egui::Pos2, 
    egui::RichText, 
    egui::CursorIcon::PointingHand,
    egui::TopBottomPanel,
    egui::SidePanel,
    egui::Margin
};

use std::{
    env,
    fs,
    path::PathBuf,
    process::Command,
    env::consts::OS,
    fs::DirEntry,
    fs::remove_dir_all,
    fs::remove_file
};

#[derive(PartialEq, Debug)]
enum PopupType {
    Rename,
    DiskInfo,
    None
}

#[derive(Debug)]
pub struct FileExplorerApp {
    current_dir: PathBuf,
    program_root: PathBuf,
    selected_item: PathBuf,
    is_disk_selection: bool,
    disk_list: Vec<char>,
    is_main_context_menu_open: bool,
    interact_pointer_pos: Pos2,
    is_open_popup: bool,
    popup_type: PopupType,
    search: String,
    rename: String,
}

impl Default for FileExplorerApp {
    fn default() -> Self {
        let path = env::current_dir().unwrap_or_default();

        Self {
            current_dir: path.clone(),
            program_root: path.clone(),
            selected_item: path,
            is_disk_selection: false,
            disk_list: get_disk_list(),
            is_main_context_menu_open: false,
            interact_pointer_pos: pos2(0.0, 0.0),
            is_open_popup: false,
            popup_type: PopupType::None,
            search: String::new(),
            rename: String::new(),
        }
    }
}

impl FileExplorerApp {
    fn open_file(&self, dir_entry: &DirEntry) {
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

    fn open_folder(&mut self, dir_entry: &DirEntry) {
        self.current_dir = dir_entry.path();
    }

    fn prev_folder(&mut self) {
        let to_prev_folder = self.current_dir.pop();

        if !to_prev_folder {
            self.is_disk_selection = true;

            self.update_disk_list();
        } else {
            if self.is_disk_selection {
                self.is_disk_selection = false;
            }
        }
    }

    fn update_disk_list(&mut self) {
        self.disk_list = get_disk_list();
    }
}

/*struct DiskInfo {
    
}*/

impl eframe::App for FileExplorerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let window_rect = ctx.screen_rect();
        let window_size = window_rect.size();
        
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(5.0);
            
            ui.heading("File Explorer");

            ui.add_space(5.0);
            ui.add(egui::Separator::default());
            ui.add_space(5.0);

            ui.horizontal(|ui| {
                // ui.style_mut().spacing.button_padding = vec2(3.0, 3.0);
                
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

            ui.add_space(10.0);

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

                let mut theme_icon: String = String::new();

                if ctx.style().visuals.dark_mode {
                    theme_icon = "sun.svg".to_string();
                } else {
                    theme_icon = "moon.svg".to_string();
                }

                let color_theme_button = ui.add(egui::Button::image(
                    egui::Image::new(
                        format!("file://{}/{}{}", self.program_root.to_str().unwrap(), "assets/Font_Awesome_Icons/solid/", theme_icon)
                    ).max_size(vec2(20.0, 20.0)),
                ));

                color_theme_button.clone().on_hover_cursor(PointingHand);

                if color_theme_button.clicked() {
                    if ctx.style().visuals.dark_mode {
                        ctx.set_visuals(egui::style::Visuals::light());
                    } else {
                        ctx.set_visuals(egui::style::Visuals::dark());
                    }
                }
            });

            ui.add_space(5.0);
        });

        SidePanel::left("left_panel")
            .min_width(window_size.x / 100.0 * 15.0)
            .max_width(window_size.x / 100.0 * 20.0)
            .frame(egui::Frame {
                inner_margin: Margin {
                    left: 15,
                    right: 15,
                    top: 15,
                    bottom: 15,
                },
                fill: if ctx.style().visuals.dark_mode {
                    egui::Color32::from_rgb(16, 16, 16)
                } else {
                    egui::Color32::from_rgb(248, 248, 248)
                },
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    let button_update = ui.add(egui::Button::image_and_text(
                        egui::Image::new(format!("file://{}/{}", self.program_root.to_str().unwrap(), "assets/Font_Awesome_Icons/solid/arrow-rotate-right.svg")),
                        RichText::new("Update").size(14.0),
                    ));

                    button_update.clone().on_hover_cursor(PointingHand);

                    if button_update.clicked() {
                        self.update_disk_list();
                    }

                    ui.add_space(5.0);
                    ui.add(egui::Separator::default());
                    ui.add_space(5.0);

                    for disk in &self.disk_list {
                        let disk_button = ui.add(egui::Button::image_and_text(
                            egui::Image::new(format!("file://{}/{}", self.program_root.to_str().unwrap(), "assets/Font_Awesome_Icons/solid/hard-drive.svg")),
                            RichText::new(disk.to_string()).size(14.0),
                        ));

                        disk_button.clone().on_hover_cursor(PointingHand);

                        if disk_button.clicked() {
                            let mut path = PathBuf::from(format!("{}:\\", disk));
                            
                            self.current_dir = path;
                            self.is_disk_selection = false;
                        }

                        disk_button.context_menu(|ui| {
                            if ui.button("Інформація про диск").on_hover_cursor(PointingHand).clicked() {
                                println!("Інформація про диск");

                                self.is_open_popup = true;
                                self.popup_type = PopupType::DiskInfo;

                                ui.close_menu();
                            }

                            self.is_main_context_menu_open = false;
                        });
                    }
                });
            });

        egui::CentralPanel::default()
            .frame(egui::Frame {
                inner_margin: Margin {
                    left: 15,
                    right: 15,
                    top: 15,
                    bottom: 15,
                },
                fill: if ctx.style().visuals.dark_mode {
                    egui::Color32::from_rgb(16, 16, 16)
                } else {
                    egui::Color32::from_rgb(248, 248, 248)
                },
                ..Default::default()
            })
            .show(ctx, |ui| {

            egui::ScrollArea::vertical()
            .id_salt("main_scroll_area")
            .auto_shrink(false)
            .max_height(ui.available_height())
            .show(ui, |ui| {
                let response = ui.interact(
                    ui.available_rect_before_wrap(),
                    ui.id().with("main_scroll_area"),
                    egui::Sense::click(),
                );

                if response.clicked_by(egui::PointerButton::Secondary) {
                    if let Some(pointer_pos) = ctx.input(|i| i.pointer.interact_pos()) {
                        self.is_main_context_menu_open = true;
                        self.interact_pointer_pos = pointer_pos;
                    }
                }

                if self.is_main_context_menu_open {
                    egui::Window::new("")
                        .current_pos(self.interact_pointer_pos)
                        .movable(false)
                        .resizable(false)
                        .collapsible(false)
                        .title_bar(false)
                        .show(ctx, |ui| {
                            if ui.button("Оновити").on_hover_cursor(PointingHand).clicked() {
                                println!("Оновити");
                            } else if ui.button("Створити нову папку").on_hover_cursor(PointingHand).clicked() {
                                let mut dir = self.current_dir.clone();
                                dir.push("Нова папка");

                                fs::create_dir(dir).expect("Не вдалося створити нову папку");
                            } else if ui.button("Створити новий файл").on_hover_cursor(PointingHand).clicked() {
                                let mut file = self.current_dir.clone();
                                file.push("Новий файл");

                                fs::File::create(file).expect("Не вдалося створити новий файл");
                            }

                            //ui.close_menu();
                        });
                }

                if self.is_open_popup {
                    egui::Window::new(
                        if self.popup_type == PopupType::DiskInfo {
                            "Інформація про диск"
                        } else if self.popup_type == PopupType::Rename {
                            "Перейменувати"
                        } else {
                            "empty popup"
                        }
                    )
                        .default_size(vec2(
                            window_size.x / 2.0,
                            window_size.y / 2.0,
                        ))
                        .max_size(vec2(
                            window_size.x - 200.0,
                            window_size.y - 200.0,
                        ))
                        .default_pos(pos2(
                            window_size.x / 4.0,
                            window_size.y / 4.0,
                        ))
                        //.movable(false)
                        .resizable(false)
                        .collapsible(false)
                        .show(ctx, |ui| {
                            if self.popup_type == PopupType::DiskInfo {
                                ui.label("Інформація");
                            } else if self.popup_type == PopupType::Rename {
                                ui.horizontal(|ui| {
                                    let new_name_label = ui.label("Перейменувати: ");

                                    ui.text_edit_singleline(&mut self.rename).labelled_by(new_name_label.id);

                                    if ui.button("Перейменувати").on_hover_cursor(PointingHand).clicked() {
                                        let mut new_name = self.selected_item.clone();

                                        new_name.pop();
                                        new_name.push(&self.rename);

                                        fs::rename(&self.selected_item, new_name).expect("Виникла помилка, під час спроби перейменувати файл");
                                    }
                                });
                            } else {}

                            if ui.button("Закрити").on_hover_cursor(PointingHand).clicked() {
                                self.is_open_popup = false;
                            }
                        });
                }

                if ctx.input(|i| i.pointer.primary_clicked()) {
                    if self.is_main_context_menu_open {
                        self.is_main_context_menu_open = false;
                    }
                }

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

                        disk_button.context_menu(|ui| {
                            if ui.button("Інформація про диск").on_hover_cursor(PointingHand).clicked() {
                                println!("Інформація про диск");

                                self.is_open_popup = true;
                                self.popup_type = PopupType::DiskInfo;
                                
                                ui.close_menu();
                            }

                            self.is_main_context_menu_open = false;
                        });
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
                                    self.open_folder(&dir_element);
                                }

                                dir_button.context_menu(|ui| {
                                    if ui.button("Перейменувати").on_hover_cursor(PointingHand).clicked() {
                                        println!("Перейменувати");

                                        self.is_open_popup = true;
                                        self.popup_type = PopupType::Rename;
                                        self.selected_item = dir_element.path();
                                        self.rename = dir_element.file_name().into_string().unwrap();
                                        
                                        ui.close_menu();
                                    }
                                    if ui.button("Копіювати").on_hover_cursor(PointingHand).clicked() {
                                        println!("Копіювати");
                                        ui.close_menu();
                                    }
                                    if ui.button("Вирізати").on_hover_cursor(PointingHand).clicked() {
                                        println!("Вирізати");
                                        ui.close_menu();
                                    }
                                    if ui.button("Видалити").on_hover_cursor(PointingHand).clicked() {
                                        match remove_dir_all(dir_element.path().to_str().unwrap()) {
                                            Ok(()) => {
                                                println!("папка була видалена");
                                            },
                                            Err(err) => {
                                                println!("{:#?}", err);
                                            }
                                        }
                                        ui.close_menu();
                                    }
                                    if ui.button("Копіювати шлях до папки").on_hover_cursor(PointingHand).clicked() {
                                        println!("Копіювати шлях");
                                        
                                        ctx.copy_text(self.current_dir.to_string_lossy().to_string());

                                        ui.close_menu();
                                    }
                                    if ui.button("Інформація про папку").on_hover_cursor(PointingHand).clicked() {
                                        println!("Інформація про папку");
                                        ui.close_menu();
                                    }

                                    self.is_main_context_menu_open = false;
                                });
                            } else if dir_element.metadata().unwrap().is_file() {
                                let file_button = ui.add(egui::Button::image_and_text(
                                    egui::Image::new(format!("file://{}/{}", self.program_root.to_str().unwrap(), "assets/Font_Awesome_Icons/solid/file.svg")),
                                    RichText::new(dir_element.file_name().to_str().unwrap()).size(14.0),
                                ));

                                file_button.clone().on_hover_cursor(PointingHand);

                                if file_button.double_clicked() {
                                    self.open_file(&dir_element);
                                }

                                file_button.context_menu(|ui| {
                                    if ui.button("Перейменувати").on_hover_cursor(PointingHand).clicked() {
                                        println!("Перейменувати");

                                        self.is_open_popup = true;
                                        self.popup_type = PopupType::Rename;
                                        self.selected_item = dir_element.path();
                                        self.rename = dir_element.file_name().into_string().unwrap();
                                        
                                        ui.close_menu();
                                    }
                                    if ui.button("Копіювати").on_hover_cursor(PointingHand).clicked() {
                                        println!("Копіювати");
                                        ui.close_menu();
                                    }
                                    if ui.button("Вирізати").on_hover_cursor(PointingHand).clicked() {
                                        println!("Вирізати");
                                        ui.close_menu();
                                    }
                                    if ui.button("Видалити").on_hover_cursor(PointingHand).clicked() {
                                        match remove_file(dir_element.path().to_str().unwrap()) {
                                            Ok(()) => {
                                                println!("файл був видалений");
                                            },
                                            Err(err) => {
                                                println!("{:#?}", err);
                                            }
                                        }

                                        ui.close_menu();
                                    }
                                    if ui.button("Копіювати шлях до файлу").on_hover_cursor(PointingHand).clicked() {
                                        println!("Копіювати шлях");

                                        ctx.copy_text(dir_element.path().to_string_lossy().to_string());

                                        ui.close_menu();
                                    }
                                    if ui.button("Інформація про файл").on_hover_cursor(PointingHand).clicked() {
                                        println!("Інформація про файл");
                                        ui.close_menu();
                                    }

                                    self.is_main_context_menu_open = false;
                                });
                            }
                        } else {

                        }
                    }
                }
            });
        });
    }
}