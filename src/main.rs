use std::{cell::RefCell, rc::Rc};

use gtk4::{self as gtk, Application, ApplicationWindow, Button, CssProvider, Label, Orientation, Picture, Window, builders::WindowBuilder, gdk::Display, gio::prelude::{ApplicationExt, ApplicationExtManual}, glib::{self, object::ObjectExt}, prelude::{BoxExt, ButtonExt, GtkWindowExt, WidgetExt}};
use image::{self, GenericImage, GenericImageView, Pixel, Rgb, Rgba};

mod color_utils;
mod config_widget;
mod image_edit;
use color_utils::rgb2hsl;

use crate::{config_widget::ConfigWidget, image_edit::{SortingConfig, picture_from_working_image}};


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
        gtk::init();

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
        let main_box = gtk::Box::builder().build();
        

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
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .label("Sort")
            .build();


        let image_view = picture_from_working_image();
        image_view.set_halign(gtk::Align::Center);
        image_view.set_valign(gtk::Align::Center);
        

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
                    image_edit::SortBy::Red => { image_edit::sort(|(r, _, _)| upper / 100.0 > r && r > lower / 100.0); },
                    image_edit::SortBy::Green => { image_edit::sort(|(_, g, _)| upper / 100.0 > g && g > lower / 100.0); },
                    image_edit::SortBy::Blue => { image_edit::sort(|(_, _, b)| upper / 100.0 > b && b > lower / 100.0); },
                    image_edit::SortBy::Hue => { image_edit::sort(|(r, g, b)| {
                        let (h, _, _) = pixel_sorting::color_utils::rgb2hsl(r, g, b);
                        upper / 100.0 > h / 360.0 && h / 360.0 > lower / 100.0
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
        main_box.append(&conf_window);

        main_box.append(&image_view);

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

}





fn main() {
    println!("Hello, world!");

    let mut app = App::new();
    app.init();
    app.app.run();

    return;

    let img = image::open("/home/sashad/Pictures/art/StoryTellingCollage.png").unwrap();
    println!("image imported");
        
    let mut image_buf = img.to_rgba8();
    let width = image_buf.width();
    let height = image_buf.height();

    let condition = |r, g, b| {
        
        let (h,s,l) = rgb2hsl(r as f32, g as f32, b as f32);

        return ((100.0 > h) && (h > 25.0)) || ((s > 60.0) && (l > 60.0))
    };


    for y in 0..height
    {
    
        let mut is_grouping = false;
        let mut group_start = 0u32;
        let mut sort_group: Vec<Rgba<u8>> = Vec::new();
    

        for x in 0..width
        {
            
            let r = image_buf.get_pixel(x, y).0[0];
            let g = image_buf.get_pixel(x, y).0[1];
            let b = image_buf.get_pixel(x, y).0[2];
           
            if condition(r, g, b) && !is_grouping
            {
                is_grouping = true;
                group_start = x;
                sort_group.clear();
            }

            if is_grouping
            {
                sort_group.push(*image_buf.get_pixel(x, y));
            }

            let at_row_end = x == width-1;
            let should_flush = (!condition(r, g, b) && is_grouping) || (at_row_end && is_grouping);

            if should_flush
            {
                is_grouping = false;
                sort_group.sort_by(|a, b| a.0[0].cmp(&b.0[0]));

                for (sorted, pixel) in sort_group.iter().enumerate()
                {
                    let target_x = group_start + sorted as u32;
                    if target_x < width
                    {
                        image_buf.put_pixel(target_x, y, *pixel);
                    }
                }

            }

        }

    }

    println!("exporting");
    image_buf.save("/home/sashad/Pictures/sorted_fucker2.png").unwrap();
}




