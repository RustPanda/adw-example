use adw::{
    gtk::{Box, Label, ScrolledWindow},
    prelude::*,
    Application, ApplicationWindow, Clamp, HeaderBar,
};

fn main() {
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(640)
            .default_height(400)
            .title("Hello, Panda!")
            .build();

        let v_box = Box::new(adw::gtk::Orientation::Vertical, 1);

        {
            let header_bar = HeaderBar::builder()
                .css_classes(vec!["flat".to_string()])
                .build();

            v_box.append(&header_bar);
        }

        {
            let v_box_h1_h6 = Box::new(adw::gtk::Orientation::Vertical, 8);

            // h1
            {
                let label = Label::builder()
                    .use_markup(true)
                    .label("<span size='30pt'>h1. Bootstrap heading</span>")
                    .build();

                label.set_halign(adw::gtk::Align::Start);
                label.set_wrap(true);

                v_box_h1_h6.append(&label);
            }

            // h2
            {
                let label = Label::builder()
                    .use_markup(true)
                    .label("<span size='24pt'>h2. Bootstrap heading</span>")
                    .build();

                label.set_halign(adw::gtk::Align::Start);
                label.set_wrap(true);

                v_box_h1_h6.append(&label);
            }

            // h3
            {
                let label = Label::builder()
                    .use_markup(true)
                    .label("<span size='21pt'>h3. Bootstrap heading</span>")
                    .build();

                label.set_halign(adw::gtk::Align::Start);
                label.set_wrap(true);

                v_box_h1_h6.append(&label);
            }

            // h4
            {
                let label = Label::builder()
                    .use_markup(true)
                    .label("<span size='18pt'>h4. Bootstrap heading</span>")
                    .build();

                label.set_halign(adw::gtk::Align::Start);
                label.set_wrap(true);

                v_box_h1_h6.append(&label);
            }

            // h5
            {
                let label = Label::builder()
                    .use_markup(true)
                    .label("<span size='15pt'>h5. Bootstrap heading</span>")
                    .build();

                label.set_halign(adw::gtk::Align::Start);
                label.set_wrap(true);

                v_box_h1_h6.append(&label);
            }

            // h6
            {
                let label = Label::builder()
                    .use_markup(true)
                    .label("<span size='12pt'>h6. Bootstrap heading</span>")
                    .build();

                label.set_halign(adw::gtk::Align::Start);
                label.set_wrap(true);

                v_box_h1_h6.append(&label);
            }

            let clamp = Clamp::builder()
                .margin_start(12)
                .margin_end(12)
                .margin_bottom(12)
                .maximum_size(1140)
                .child(&v_box_h1_h6)
                .build();

            let scrolled_window = ScrolledWindow::builder()
                .vexpand(true)
                .kinetic_scrolling(true)
                .hscrollbar_policy(adw::gtk::PolicyType::Never) // Disable horizontal scrolling
                .vscrollbar_policy(adw::gtk::PolicyType::Automatic)
                .min_content_width(360)
                .child(&clamp)
                .build();

            v_box.append(&scrolled_window);
        }

        window.set_content(Some(&v_box));
        window.show();
    });

    app.run();
}
