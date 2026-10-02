use std::{cell::RefCell, process::exit, rc::Rc};

use gtk4::{self as gtk, Application, ApplicationWindow, Button, CssProvider, Label, Orientation, Picture, gdk::Display, gio::prelude::{ActionMapExtManual, ApplicationExt, ApplicationExtManual}, glib::{self}, prelude::{BoxExt, ButtonExt, GtkWindowExt, WidgetExt}};

mod config_widget;
mod image_edit;

use crate::{config_widget::ConfigWidget, image_edit::{SortingConfig, picture_from_working_image}};
use rips_algorithms as algorithms;

struct App
{
    pub app: Application,
    configurators: Rc<RefCell<Vec<ConfigWidget>>>,
    main_box: gtk::Box,
    view: Picture,
    sorting_configs: Rc<RefCell<Vec<SortingConfig>>>,
}

impl App
{
    pub fn new() -> Self
    {
        if gtk::init().is_err() { exit(1) }

        let configurators: Rc<RefCell<Vec<ConfigWidget>>> = Rc::new(RefCell::new(Vec::new()));

        let provider = CssProvider::new();
        provider.load_from_path("styles/styles.css");
        gtk::style_context_add_provider_for_display(&Display::default().unwrap(), &provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

        let app = Application::builder().application_id("org.pixel.racc").build();
        
        let main_box = gtk::Box::new(Orientation::Vertical, 0);
        let view = Picture::new();
        let sorting_configs = Rc::new(RefCell::new(Vec::<SortingConfig>::new()));
        Self {app, configurators, view, main_box, sorting_configs} 
    }

    pub fn init(&mut self)
    {
        self.main_box = self.build_primary_view();

        self.app.connect_activate(glib::clone!(#[weak(rename_to=main_box)] self.main_box, move |app|{
            let window = ApplicationWindow::builder()
                .application(app)
                .default_width(1920)
                .default_height(1080)
                .title("Sick Pixel Fucker Applicatition")
                .build();


            window.set_child(Some(&main_box));

            window.present();
        }));
    }

    fn build_primary_view(&mut self) -> gtk::Box
    {
        let main_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();



        let menu_bar = gtk::Box::builder()
            .name("menu-bar")
            .orientation(gtk::Orientation::Horizontal)
            .halign(gtk::Align::Baseline)
            .valign(gtk::Align::Center)
            .hexpand(true)
            .vexpand(true)
            .build();

        let popover = self.build_popover_bar();

        menu_bar.append(&popover);
            

        let work_area = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .halign(gtk::Align::Baseline)
            .valign(gtk::Align::Center)
            .hexpand(true)
            .vexpand(true)
            .build();

        let conf_window = gtk::Box::builder()
            .name("config")
            .hexpand(false)
            .vexpand(true)
            .overflow(gtk::Overflow::Visible)
            .orientation(gtk::Orientation::Vertical)
            .build(); 

        let configuration_scrolling_area = gtk::ScrolledWindow::builder()
            .name("config-scroll")
            .hexpand(true)
            .vexpand(true)
            .overflow(gtk::Overflow::Visible)
            .build();

        let configuration_box = gtk::Box::builder()
            .name("config-box")
            .hexpand(true)
            .vexpand(true)
            .valign(gtk4::Align::Baseline)
            .halign(gtk4::Align::Baseline)
            .orientation(Orientation::Vertical)
            .overflow(gtk::Overflow::Visible)
            .build();

        let control_panel_area = gtk::Box::builder()
            .name("runner_area")
            .hexpand(true)
            .vexpand(false)
            .halign(gtk::Align::Baseline)
            .valign(gtk::Align::End)
            .orientation(gtk::Orientation::Vertical)
            .build();

        let sort_btn = Button::builder()
            .name("sort-button")
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .label("Sort")
            .build();


        let image_view = picture_from_working_image();
        image_view.set_widget_name("working-image");
        image_view.set_halign(gtk::Align::Center);
        image_view.set_valign(gtk::Align::Center);
        image_view.set_hexpand(true);
        image_view.set_vexpand(true);
        

        self.view = image_view.clone();
       
        sort_btn.connect_clicked(glib::clone!(
        #[strong(rename_to=configurators)] self.configurators,
        #[strong(rename_to=sorting_configs)] self.sorting_configs,
        #[weak(rename_to=view)] self.view,
        move |_| {
            println!("start sort"); 
            let mut config_widgets = configurators.borrow_mut();
            let mut sort_confs = sorting_configs.borrow_mut();

            config_widgets.retain(|c| {c.parent().is_some()});


            sort_confs.clear();
            for widget in config_widgets.iter_mut()
            {
                sort_confs.push(widget.get_config());
            }

            for conf in sort_confs.iter()
            {
                let upper = conf.upper;
                let lower = conf.lower;

                match conf.sort_selection
                {
                    image_edit::SortBy::Red => { image_edit::sort(|(r, _, _)| upper / 100.0 >= r && r >= lower / 100.0); },
                    image_edit::SortBy::Green => { image_edit::sort(|(_, g, _)| upper / 100.0 >= g && g >= lower / 100.0); },
                    image_edit::SortBy::Blue => { image_edit::sort(|(_, _, b)| upper / 100.0 >= b && b >= lower / 100.0); },
                    image_edit::SortBy::Hue => { image_edit::sort(|(r, g, b)| {
                        let (h, _, _) = algorithms::color_utils::rgb2hsl(r, g, b);
                        upper / 100.0 > h / 360.0 && h / 360.0 > lower / 100.0
                    }) },
                    image_edit::SortBy::Saturation => { image_edit::sort(|(r, g, b)| {
                        let (_, s, _) = algorithms::color_utils::rgb2hsl(r, g, b);
                        upper / 100.0 > s / 100.0 && s / 100.0 > lower / 100.0
                    }) },
                    image_edit::SortBy::Lightness => { image_edit::sort(|(r, g, b)| {
                        let (_, _, l) = algorithms::color_utils::rgb2hsl(r, g, b);
                        upper / 100.0 > l / 100.0 && l / 100.0 > lower / 100.0
                    }) }
                    _ => {}
                } 
            }

            view.set_paintable(Some(&image_edit::texture_from_working_image()));
            println!("sorted");
        }));

            

        control_panel_area.append(&sort_btn);


        conf_window.append(&configuration_scrolling_area);
        conf_window.append(&control_panel_area);

        configuration_scrolling_area.set_child(Some(&configuration_box));

        work_area.append(&conf_window);
        work_area.append(&image_view);

        main_box.append(&menu_bar);
        main_box.append(&work_area);

        let add_conf_button = Button::builder()
            .name("new-conf-btn")
            .hexpand(true)
            .vexpand(true)
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Start)
            .build();


        add_conf_button.connect_clicked(gtk::glib::clone!(
        #[strong(rename_to=configurators)] self.configurators,
        #[weak] configuration_box, 
        move |b| {
            
            let new_conf = ConfigWidget::new();
            new_conf.insert_before(&configuration_box, Some(b));
            configurators.borrow_mut().push(new_conf);
        }));

        configuration_box.insert_child_after(&add_conf_button, None::<&Label>);
    
        let add_label = Label::builder()
            .label("Add Configuration")
            .name("plus-label")
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .build();

        add_conf_button.set_child(Some(&add_label));
    
        main_box
    }

    fn build_popover_bar(&mut self) -> gtk::PopoverMenuBar
    {
        
        let file_menu = gtk::gio::Menu::new();
        file_menu.append(Some("_Open"), Some("app.file_open"));
        file_menu.append(Some("_Export"), Some("app.export"));
        file_menu.append(Some("_Quit"), Some("app.quit"));
       
        let edit_menu = gtk::gio::Menu::new();
        edit_menu.append(Some("_Nothing"), None);

        let window_menu = gtk::gio::Menu::new();
        window_menu.append(Some("_Help"), Some("app.help"));

        let top_level_menu = gtk::gio::Menu::new();
        top_level_menu.append_submenu(Some("_File"), &file_menu);
        top_level_menu.append_submenu(Some("_Edit"), &edit_menu);
        top_level_menu.append_submenu(Some("_Window"), &window_menu);

        

        let popover = gtk::PopoverMenuBar::builder()
            .name("popover-menu")
            .menu_model(&top_level_menu)
            .halign(gtk::Align::Start)
            .valign(gtk::Align::Start)
            .hexpand(true)
            .vexpand(true)
            .build();
    
        
        let file_open = gtk::gio::ActionEntry::builder("file_open")
            .activate(|_app: &gtk::Application, _, _| 
            {
                println!("open");
                let dialog = rfd::FileDialog::new()
                    .add_filter("image", &["png", "jpg", "jpeg"])
                    .set_title("Select an image");
                    
                if let Some(path) = dialog.pick_file()
                {
                    std::thread::spawn(move || 
                    {
                        let _ = image_edit::change_working_image(path);
                        println!("Changed image");
                    });
                }

            })
            .build();

        let export = gtk::gio::ActionEntry::builder("export")
            .activate(|_app: &gtk::Application, _, _| 
            {
                println!("export");
                let dialog = rfd::FileDialog::new()
                    .add_filter("image", &["png", "jpg", "jpeg"])
                    .set_title("Export location");

                std::thread::spawn(move || {
                    if let Some(path) = dialog.save_file()
                    {
                        let _ = image_edit::export(path);
                    }
                });


            })
            .build();

        let quit = gtk::gio::ActionEntry::builder("quit")
            .activate(|_app: &gtk::Application, _, _| exit(0))
            .build();

        
        let help = gtk::gio::ActionEntry::builder("help")
            .activate(|_app: &gtk::Application, _, _| { let _ = open::that("https://github.com/MildRacc/rips-gui"); })
            .build();

        self.app.add_action_entries([file_open, export, quit, help]);


        popover
    }

}





fn main()
{
    let mut app = App::new();
    app.init();
    app.app.run();
}
