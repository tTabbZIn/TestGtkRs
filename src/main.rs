use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button, Box as GtkBox, Orientation, CssProvider};
use gtk::gdk::Display;
use std::env;



const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() -> glib::ExitCode {
    // Cria uma nova aplicação
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);
    
    // Executa a aplicação
    app.run()
}

fn build_ui(app: &Application) {


    let os = format!("     {} \n", std::env::consts::OS);
    let cpu = "    Amd Ryzen5 \n";
    let gpu = "󰮄    Nvidea 2070 super\n";
    let terminal = "   Kitty\n";
    let wm = "     i3\n\n\n\n";
    let cat1 = "──────▄▀▄─────▄▀▄\n";
    let cat2 = "─────▄█░░▀▀▀▀▀░░█▄\n";
    let cat3 = "─▄▄──█░░░░░░░░░░░█──▄▄\n";
    let cat4= "█▄▄█─█░░▀░░┬░░▀░░█─█▄▄█";
    let info = os + cpu + gpu + terminal + wm + cat1 + cat2 + cat3 + cat4;
    let bt_status = Button::builder()
        .label(info)
        .build();

    bt_status.set_size_request(400, 350);
    bt_status.set_widget_name("status");

    let bt_clock = Button::builder()
        .label("   00:00:00")
        .build();

    bt_clock.set_size_request(400, 50);
    bt_clock.set_widget_name("clock");


    let button_image = Button::builder()
        .build();
    button_image.set_size_request(200, 200);
    button_image.set_widget_name("btimage");

    let button_off = Button::builder()
        .label("  ")
        .build();
    button_off.set_size_request(100, 100);
    button_off.set_widget_name("bt1");

    let button = Button::builder()
        .label("  ")
        .build();
    button.set_size_request(100, 100);
    button.set_widget_name("bt2"); 

    let button_exit = Button::builder()
        .label(" 󰿅 ")
        .build();
    button_exit.set_size_request(100, 100); // Define largura e altura
    button_exit.set_widget_name("bt3"); 

    let button_logo = Button::builder()
        .label("  ")
        .build();
    button_logo.set_size_request(100, 100); // Define largura e altura
    button_logo.set_widget_name("bt4"); 

    // Cria uma caixa para manter os botões
    let boximage = GtkBox::new(Orientation::Horizontal, 0);
    boximage.append(&button_image);

    let box1 = GtkBox::new(Orientation::Horizontal, 0);
    box1.append(&button);
    box1.append(&button_off);

    let box2 = GtkBox::new(Orientation::Horizontal, 0);
    box2.append(&button_exit);
    box2.append(&button_logo);


    let container2 = GtkBox::new(Orientation::Vertical, 0);
    container2.append(&bt_status);
    container2.append(&bt_clock);

    let container1 = GtkBox::new(Orientation::Vertical, 0);
    container1.append(&boximage);
    container1.append(&box1);
    container1.append(&box2);

    let container_main = GtkBox::new(Orientation::Horizontal,0);
    container_main.append(&container2);
    container_main.append(&container1);
    container_main.set_widget_name("container");

    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&container_main)
        .build();


    window.present();
}

fn load_css() {
    // Carrega o arquivo CSS e adiciona ao provider
    let provider = CssProvider::new();

    provider.load_from_string(include_str!("styles/style.css"));

    // Adiciona o provider à tela padrão
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Não foi possível conectar a um display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

