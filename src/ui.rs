use glib::clone;
use gtk;
use gtk::prelude::*;
use gio::prelude::*;

use std::env::args;

use crate::mouse;

fn click_button(brightness: u8) {
    println!("{:?}", brightness);

    let (mut handle, _) = mouse::init();
    mouse::write_brightness(&mut handle, brightness).expect("Write Bright is Fail");
}

fn update_brightness(label: &gtk::Label, slider: &gtk::Scale) {
    let (mut handle, _) = mouse::init();
    let level = mouse::read_brightness(&mut handle).unwrap();

    label.set_text(&level.to_string());
    let res = slider.get_value();
    println!("{}", res);
    slider.set_value(level.into())
}

fn update_model(label: &gtk::Label) {
    let (_, device) = mouse::init();
    let model = mouse::get_name(device.product_id());

    label.set_text(&model.to_string());
}

pub fn build_ui(_application: &gtk::Application) {
    println!("buildui");


    gtk::init().unwrap_or_else(|_| panic!("panic!"));
    let glade_src = include_str!("glade/main.glade");
    let builder = gtk::Builder::from_string(glade_src);

    let window: gtk::Window = builder.get_object("window").unwrap();
    let lvl0: gtk::Button = builder.get_object("lvl0").unwrap();
    let lvl1: gtk::Button = builder.get_object("lvl1").unwrap();
    let lvl2: gtk::Button = builder.get_object("lvl2").unwrap();
    let lvl3: gtk::Button = builder.get_object("lvl3").unwrap();
    let model: gtk::Label = builder.get_object("model").unwrap();
    let brightness: gtk::Label = builder.get_object("brightness").unwrap();
    let slider: gtk::Scale = builder.get_object("scale").unwrap();

    update_model(&model);
    update_brightness(&brightness, &slider);

    lvl0.connect_clicked(clone!(@weak window => move |_| {
        println!("lvl0 clicked");
        click_button(0);
    }));

    lvl1.connect_clicked(clone!(@weak window => move |_| {
        println!("lvl1 clicked");
        click_button(1);
    }));

    lvl2.connect_clicked(clone!(@weak window => move |_| {
        println!("lvl2 clicked");
        click_button(2);
    }));

    lvl3.connect_clicked(clone!(@weak window => move |_| {
        println!("lvl3 clicked");
        click_button(3);
    }));

    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}

pub fn launch() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.gtktest"),
        Default::default(),
    )
    .expect("Initialization failed...");

    println!("launch");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
