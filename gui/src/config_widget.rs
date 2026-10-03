use gtk4::{self as gtk, glib::{Object, subclass::types::ObjectSubclassIsExt}};
use gtk::glib;

use crate::image_edit::SortingConfig;

mod imp
{
use gtk4::prelude::{BoxExt, ButtonExt, OrientableExt, WidgetExt};
use gtk4::{self as gtk, Align};
use gtk::glib;
use gtk::subclass::prelude::*;
use rips_algorithms::color_utils::SortBy;
use crate::image_edit::{SortingConfig};


    #[derive(Default)]
    pub struct ConfigWidget 
    {
        expander: gtk::Expander,
        expanding_container: gtk::Box,
        head_container: gtk::Box,
        close: gtk::Button,
        dropdown: gtk::DropDown,
        max_container: gtk::Box,
        max_label: gtk::Label,
        max: gtk::SpinButton,
        min_container: gtk::Box,
        min_label: gtk::Label,
        min: gtk::SpinButton,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ConfigWidget
    {
        const NAME: &'static str = "ConfigWidget";
        type Type = super::ConfigWidget;
        type ParentType = gtk::Box;
    }

    impl ConfigWidget
    {
        pub fn get_config(&self) -> SortingConfig
        {
            let sort_by = match self.dropdown.selected()
            {
                0 => SortBy::Red,
                1 => SortBy::Green,
                2 => SortBy::Blue,
                3 => SortBy::Hue,
                4 => SortBy::Chroma,
                5 => SortBy::Saturation,
                6 => SortBy::Lightness,
                7 => SortBy::Luminance,
                8 => SortBy::Value,
                9 => SortBy::Alpha,
                _ => SortBy::Alpha
            };

            SortingConfig { sort_selection: sort_by, upper: self.max.value() as f32, lower: self.min.value() as f32 }
        }
    }


    impl ObjectImpl for ConfigWidget 
    {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.set_orientation(gtk4::Orientation::Vertical);
            obj.set_css_classes(&["config-widget"]);
            obj.set_valign(Align::Start);
            obj.set_halign(Align::Baseline);
            obj.set_vexpand(false);
            obj.set_hexpand(true);
            
            self.expander.set_label(Some("  Red [ 0 - 100 ]"));
            self.expander.set_vexpand(true);
            self.expander.set_hexpand(true);
            self.expander.set_expanded(true);

            self.expanding_container.set_orientation(gtk::Orientation::Vertical);
            self.expanding_container.set_hexpand(true);
            self.expanding_container.set_vexpand(true);

            self.head_container.set_valign(Align::Center);
            self.head_container.set_halign(Align::Baseline);
            self.head_container.set_vexpand(true);
            self.head_container.set_hexpand(true);
            self.head_container.set_orientation(gtk4::Orientation::Horizontal);
            
            self.close.set_halign(Align::Start);
            self.close.set_valign(Align::Start);
            self.close.set_icon_name("window-close-symbolic");
            self.close.set_hexpand(false);
            self.close.set_vexpand(false);
            self.close.set_css_classes(&["widget-close"]);
            self.close.set_size_request(20, 20);
            self.close.set_tooltip_text(Some("Close"));
            self.close.connect_clicked(glib::clone!(#[weak] obj, move |_| obj.unparent()));
            
            self.dropdown.set_model(Some(&gtk::StringList::new(&["Red", "Green", "Blue", "Hue", "Chroma", "Saturation", "Lightness", "Luminance", "Value", "Alpha"])));
            self.dropdown.set_halign(Align::End);
            self.dropdown.set_valign(Align::Center);
            self.dropdown.set_hexpand(true);
            self.dropdown.set_vexpand(true);
            self.dropdown.set_tooltip_text(Some("Sort By..."));

            self.dropdown.connect_selected_notify(glib::clone!(
                #[weak(rename_to=expander)] self.expander,
                #[weak(rename_to=min)] self.min,
                #[weak(rename_to=max)] self.max,
                move |dropdown| 
            {
                
                let dropdown_text = index_to_text(dropdown.selected())
                    + " [ "
                    + &min.value().to_string()
                    + " - "
                    + &max.value().to_string()
                    + " ]";

                expander.set_label(Some(&dropdown_text));
            }));

            self.max_container.set_valign(Align::Center);
            self.max_container.set_halign(Align::Baseline);
            self.max_container.set_vexpand(true);
            self.max_container.set_hexpand(true);
            self.max_container.set_orientation(gtk4::Orientation::Horizontal);

            self.min_container.set_valign(Align::Center);
            self.min_container.set_halign(Align::Baseline);
            self.min_container.set_vexpand(true);
            self.min_container.set_hexpand(true);
            self.min_container.set_orientation(gtk4::Orientation::Horizontal);

            self.max_label.set_text("Max");
            self.max_label.set_halign(Align::Start);
            self.max_label.set_valign(Align::Center);

            self.min_label.set_text("Min");
            self.min_label.set_halign(Align::Start);
            self.min_label.set_valign(Align::Center);

            self.max.set_range(0.0, 100.0);
            self.max.set_value(100.0);
            self.max.set_increments(1.0, 1.0);
            self.max.set_climb_rate(1.0);
            self.max.set_halign(Align::End);
            self.max.set_valign(Align::Center);
            self.max.set_hexpand(true);
            self.max.set_vexpand(true);

            self.max.connect_value_changed(glib::clone!( 
                #[weak(rename_to=expander)] self.expander,
                #[weak(rename_to=min)] self.min,
                #[weak(rename_to=drop)] self.dropdown,
                move |max|
            {
                let dropdown_text = index_to_text(drop.selected())
                    + " [ "
                    + &min.value().to_string()
                    + " - "
                    + &max.value().to_string()
                    + " ]";

                expander.set_label(Some(&dropdown_text));
            }));


            self.min.set_range(0.0, 100.0);
            self.min.set_value(0.0);
            self.min.set_increments(1.0, 1.0);
            self.min.set_climb_rate(1.0);
            self.min.set_halign(Align::End);
            self.min.set_valign(Align::Center);
            self.min.set_hexpand(true);
            self.min.set_vexpand(true);

            self.min.connect_value_changed(glib::clone!( 
                #[weak(rename_to=expander)] self.expander,
                #[weak(rename_to=max)] self.max,
                #[weak(rename_to=drop)] self.dropdown,
                move |min|
            {
                let dropdown_text = index_to_text(drop.selected())
                    + " [ "
                    + &min.value().to_string()
                    + " - "
                    + &max.value().to_string()
                    + " ]";

                expander.set_label(Some(&dropdown_text));
            }));


            self.head_container.append(&self.close);
            self.head_container.append(&self.dropdown);

            self.max_container.append(&self.max_label);
            self.max_container.append(&self.max);

            self.min_container.append(&self.min_label);
            self.min_container.append(&self.min);

            self.expander.set_child(Some(&self.expanding_container));
            
            self.expanding_container.append(&self.head_container);
            self.expanding_container.append(&self.max_container);
            self.expanding_container.append(&self.min_container);

            obj.append(&self.expander);
            
        }
    }
    impl WidgetImpl for ConfigWidget {}
    impl BoxImpl for ConfigWidget {}


    fn index_to_text(i: u32) -> String
    {
        "  ".to_string() + match i 
        {
            0 => "Red",
            1 => "Green",
            2 => "Blue",
            3 => "Hue",
            4 => "Chroma",
            5 => "Saturation",
            6 => "Lightness",
            7 => "Luminance",
            8 => "Value",
            9 => "Alpha",
            _ => "Unknown Selection"
        }
    }


}


glib::wrapper! 
{
    pub struct ConfigWidget(ObjectSubclass<imp::ConfigWidget>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Buildable, gtk::ConstraintTarget, gtk::Actionable, gtk::Orientable; 
}

impl ConfigWidget
{
    pub fn new() -> Self
    {
        Object::builder().build()
    }

    pub fn get_config(&self) -> SortingConfig
    {
        self.imp().get_config()         
    }
}
