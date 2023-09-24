use std::{cell::RefCell, rc::Rc};

use gtk::{
    glib::{self, ExitCode},
    prelude::{ApplicationExt, ApplicationExtManual, ButtonExt, GridExt, GtkWindowExt},
    Application, ApplicationWindow, Button, Grid, Label,
};

fn main() -> ExitCode {
    let app = Application::builder()
        .application_id("com.calculadoraGtk")
        .build();

    app.connect_activate(view);

    app.run()
}

pub fn view(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Calculadora")
        .build();

    let layout = Grid::new();

    let commands: Rc<RefCell<Vec<Command>>> = Rc::new(RefCell::new(Vec::new()));
    let number_text = Rc::new(RefCell::new(String::new()));

    let button1 = Button::with_label("1");
    button1.connect_clicked(
        glib::clone!( @strong number_text => move |_| number_text.borrow_mut().push('1'); ),
    );
    let button2 = Button::with_label("2");
    button2.connect_clicked(
        glib::clone!( @strong number_text => move |_| number_text.borrow_mut().push('2'); ),
    );
    let button3 = Button::with_label("3");
    button3.connect_clicked(
        glib::clone!( @strong number_text => move |_| number_text.borrow_mut().push('3'); ),
    );
    let button4 = Button::with_label("4");
    button4.connect_clicked(
        glib::clone!( @strong number_text => move |_| number_text.borrow_mut().push('4'); ),
    );
    let button5 = Button::with_label("5");
    button5.connect_clicked(
        glib::clone!( @strong number_text => move |_| number_text.borrow_mut().push('5'); ),
    );
    let button6 = Button::with_label("6");
    button6.connect_clicked(
        glib::clone!( @strong number_text => move |_| number_text.borrow_mut().push('6'); ),
    );
    let button7 = Button::with_label("7");
    button7.connect_clicked(
        glib::clone!( @strong number_text => move |_| number_text.borrow_mut().push('7'); ),
    );
    let button8 = Button::with_label("8");
    button8.connect_clicked(
        glib::clone!( @strong number_text => move |_| number_text.borrow_mut().push('8'); ),
    );
    let button9 = Button::with_label("9");
    button9.connect_clicked(
        glib::clone!( @strong number_text => move |_| number_text.borrow_mut().push('9'); ),
    );
    let button0 = Button::with_label("0");
    button0.connect_clicked(
        glib::clone!( @strong number_text => move |_| number_text.borrow_mut().push('0'); ),
    );

    let button_some = Button::with_label("+");
    button_some.connect_clicked(
        glib::clone!( @strong number_text, @strong commands => move |_| {
            commands.borrow_mut().push(Command::Number(number_text.borrow().to_string()));
            commands.borrow_mut().push(Command::Soma);
            number_text.borrow_mut().clear();
        }),
    );

    let button_sub = Button::with_label("-");
    button_sub.connect_clicked(
        glib::clone!( @strong number_text, @strong commands => move |_| {
            commands.borrow_mut().push(Command::Number(number_text.borrow().to_string()));
            commands.borrow_mut().push(Command::Sub);
            number_text.borrow_mut().clear();
           
            
        }),
    );
    let button_mult = Button::with_label("*");
    button_mult.connect_clicked(
        glib::clone!( @strong number_text, @strong commands => move |_| {
            commands.borrow_mut().push(Command::Number(number_text.borrow().to_string()));
            commands.borrow_mut().push(Command::Mult);
            number_text.borrow_mut().clear();
        }),
    );
    let button_div = Button::with_label("/");
    button_div.connect_clicked(
        glib::clone!( @strong number_text, @strong commands => move |_| {
            commands.borrow_mut().push(Command::Number(number_text.borrow().to_string()));
            commands.borrow_mut().push(Command::Div);
            number_text.borrow_mut().clear();
        }),
    );

    let result = Button::with_label("=");
    let result_counter = Rc::new(RefCell::new(Label::new(Some("0"))));

    result.connect_clicked(
        glib::clone!(@strong commands, @strong result_counter =>  move |_| {
            commands.borrow_mut().push(Command::Number(number_text.borrow().to_string()));
            number_text.borrow_mut().clear();
            result_counter.borrow_mut().set_label(counter(Rc::clone(&commands)).to_string().as_str());
            commands.borrow_mut().clear();
        }),
    );

    layout.attach(&result_counter.borrow().clone(), 1, 1, 4, 1);
    layout.attach(&button1, 1, 2, 1, 1);
    layout.attach(&button2, 2, 2, 1, 1);
    layout.attach(&button3, 3, 2, 1, 1);
    layout.attach(&button_some, 4, 2, 1, 1);

    layout.attach(&button4, 1, 3, 1, 1);
    layout.attach(&button5, 2, 3, 1, 1);
    layout.attach(&button6, 3, 3, 1, 1);
    layout.attach(&button_sub, 4, 3, 1, 1);

    layout.attach(&button7, 1, 4, 1, 1);
    layout.attach(&button8, 2, 4, 1, 1);
    layout.attach(&button9, 3, 4, 1, 1);
    layout.attach(&button_mult, 4, 4, 1, 1);

    layout.attach(&button0, 2, 5, 1, 1);
    layout.attach(&button_div, 3, 5, 1, 1);
    layout.attach(&result, 4, 5, 1, 1);

    window.set_child(Some(&layout));
    window.present();
}

#[derive(Debug)]
pub enum Command {
    None,
    Soma,
    Sub,
    Div,
    Mult,
    Number(String),
}

fn counter(commands: Rc<RefCell<Vec<Command>>>) -> i32 {
    let mut counter = 0;
    let mut actual_command = &Command::None;

    for command in commands.borrow().iter() {
        match command {
            Command::Number(number) => {
                if let Ok(number_parse) = number.parse::<i32>() {
                    let number_prev = counter;
                    counter = number_parse;
                    
                    match actual_command {
                        Command::Soma => counter += number_prev,
                        Command::Sub => counter = number_prev - counter,
                        Command::Mult => counter *= number_prev,
                        Command::Div => counter = number_prev / counter,
                        _ => {}
                    }
                }
            }
            _ => actual_command = command,
        }
    }

    counter
}
