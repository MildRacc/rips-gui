use gtk4::{self as gtk, ConstraintTarget, glib::{Object, property::PropertyGet, subclass::types::ObjectSubclassIsExt}, prelude::WidgetExt};
use gtk::glib;

use crate::image_edit::SortingConfig;

mod imp
{
    use gtk4::AccessibleProperty::Sort;
use gtk4::prelude::{BoxExt, ButtonExt, OrientableExt, WidgetExt};
use gtk4::{self as gtk, Align, Label};
    use gtk::glib;
    use gtk::subclass::prelude::*;
    use crate::image_edit::{SortBy, SortingConfig};


    #[derive(Default)]
    pub struct ConfigWidget 
    {
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
            
            self.min.set_range(0.0, 100.0);
            self.min.set_value(0.0);
            self.min.set_increments(1.0, 1.0);
            self.min.set_climb_rate(1.0);
            self.min.set_halign(Align::End);
            self.min.set_valign(Align::Center);
            self.min.set_hexpand(true);
            self.min.set_vexpand(true);

            self.head_container.append(&self.close);
            self.head_container.append(&self.dropdown);

            self.max_container.append(&self.max_label);
            self.max_container.append(&self.max);

            self.min_container.append(&self.min_label);
            self.min_container.append(&self.min);


            obj.append(&self.head_container);
            obj.append(&self.max_container);
            obj.append(&self.min_container);
            
        }
    }
    impl WidgetImpl for ConfigWidget {}
    impl BoxImpl for ConfigWidget {}



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
