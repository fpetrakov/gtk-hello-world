use gtk::{prelude::*, Button};
use gtk::{Application, ApplicationWindow};

fn main() {
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let button = Button::builder()
        .label("Press me!")
        .margin_bottom(12)
        .margin_top(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    button.connect_clicked(move |button| button.set_label("Hello World!"));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&button)
        .build();

    window.present();
}
