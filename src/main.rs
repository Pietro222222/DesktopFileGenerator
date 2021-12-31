use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use gtk::{prelude::*, Application, ApplicationWindow, Image};
use gtk::{Box, Button, CheckButton, Entry, FileChooserAction, FileChooserButton, Label};

struct DesktopFile {
    pub path: String,
    pub logo_path: Option<PathBuf>,
    pub bin: String,
    pub terminal: bool,
}

impl DesktopFile {
    pub fn new(path: String, logo: Option<PathBuf>, bin: String, terminal: bool) -> Self {
        Self {
            bin: bin,
            logo_path: logo,
            path: path,
            terminal: terminal,
        }
    }
    pub fn save(&self, fast: bool) {
        let path = if fast {
            format!(
                "{}/.local/share/applications/{}.desktop",
                std::env::var("HOME").unwrap(),
                self.path
            )
        } else {
            format!(
                "{}/Downloads/{}.desktop",
                std::env::var("HOME").unwrap(),
                self.path
            )
        };
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)
            .unwrap();
        let mut content = format!("[Desktop Entry]\nEncoding=UTF-8\nVersion=1.0\nType=Application\nTerminal={}\nExec={}\nName={}", self.terminal, self.bin, self.path);
        if let Some(imagepath) = self.logo_path.clone() {
            let icon = format!(
                "\nIcon={}",
                imagepath.into_os_string().into_string().unwrap()
            );
            content.push_str(&icon);
        }
        file.write(content.as_bytes()).unwrap();
    }
}

fn main() {
    let application = Application::builder()
        .application_id("pietro.gtk.desktopfilegenerator")
        .build();
    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Desktop File Generator")
            .default_height(500)
            .default_width(350)
            .build();

        let gtk_box = Box::new(gtk::Orientation::Vertical, 6);
        let desktop_filename = Entry::new();
        let gtk_box_bool_options = Box::new(gtk::Orientation::Horizontal, 5);
        let fast_install = CheckButton::with_label("Fast Install");
        let terminal = CheckButton::with_label("Terminal Application");
        let logo = FileChooserButton::new("Logo", FileChooserAction::Open);
        let logo_desc = Label::new(Some("Image File"));
        let bin = FileChooserButton::new("Binary", FileChooserAction::Open);
        let bin_desc = Label::new(Some("Executable File"));
        let generate = Button::with_label("Generate Desktop File");
        let gtk_box_logo = Box::new(gtk::Orientation::Horizontal, 3);
        let gtk_box_bin = Box::new(gtk::Orientation::Horizontal, 3);
        let warning = Label::new(None);
        let image_preview = Image::new();
        gtk_box.add(&desktop_filename);
        gtk_box_bool_options.add(&terminal);
        gtk_box_bool_options.add(&fast_install);
        gtk_box.add(&gtk_box_bool_options);
        gtk_box_logo.add(&logo_desc);
        gtk_box_logo.add(&logo);
        gtk_box_bin.add(&bin_desc);
        gtk_box_bin.add(&bin);
        gtk_box.add(&gtk_box_logo);
        gtk_box.add(&gtk_box_bin);
        gtk_box.add(&warning);
        gtk_box.add(&image_preview);
        gtk_box.add(&generate);

        window.add(&gtk_box);
        generate.connect_clicked(move |_| {
            warning.set_text("");
            let filename = desktop_filename.text().to_string();
            if filename.is_empty() {
                warning.set_text("First Field Cannot Be Empty!");
                return;
            }
            let bin_file = match bin.filename() {
                Some(d) => d,
                None => {
                    warning.set_text("Please select an executable file");
                    return;
                }
            };
            let image_path = logo.filename();
            if let Some(path) = &image_path {
                image_preview.set_from_file(path);
            } else {
                warning.set_text("No Image File Was provided");
            }
            let desktopfile = DesktopFile::new(
                filename,
                image_path,
                bin_file.into_os_string().into_string().unwrap(),
                terminal.is_active(),
            );
            desktopfile.save(fast_install.activate());
        });

        window.show_all();
    });
    application.run();
}
