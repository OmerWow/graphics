extern crate cairo;
extern crate gio;
extern crate gtk;
#[macro_use]
extern crate lazy_static;
extern crate global;
use std::env::args;
use std::f64::consts::PI;
use global::Global;
use gio::prelude::*;
use gtk::prelude::*;
use gtk::DrawingArea;
use gtk::{Window,WindowType, Label};
use gtk::{ApplicationWindow,Application};

use cairo::{Context, FontSlant, FontWeight};

pub struct Canvas{
    height:u64,
    width:u64,
    fill:Color,
    //x:u64,
    //y:u64,
    window:ApplicationWindow,
    app:Application,
}
pub struct Color{
    r:u8,
    g:u8,
    b:u8,
}
impl Color{
    pub fn new(r:u8,g:u8,b:u8)->Color{
        Color{r,g,b}
    }
    pub fn from(r:u8,g:u8,b:u8)->Color{
        Color{r,g,b}
    }
}
impl Canvas{
    pub fn createCanvas(width:u64,height:u64)->Canvas{
        let app = Application::new(
            Some("com.github.guyl"),
            Default::default(),
        )
        .expect("Initialization failed...");
        let can1:ApplicationWindow = ApplicationWindow::new(&app);
        let can2 = Canvas{height,width,fill:Color::from(220,220,220),window:can1,app};
        can2
    }
    pub fn show(&mut self){
        (*self).app.connect_activate( |app|{
            self.window.set_title("myCanvas");
            self.window.set_border_width(10);
            self.window.set_position(gtk::WindowPosition::Center);
            self.window.set_default_size(self.width as i32,self.height as i32);
            self.window.show_all();
        });
        self.app.run(&args().collect::<Vec<_>>());
    }
}
/*
pub fn background<T>(backcolor:T)
where 
    T: color,
    T:u8
{
    match backcolor{
        (_,_,_)=>set_source_rgb(backcolor[0]/255.0,backcolor[1]/255.0,backcolor[2]/255.0) ,
        _=>set_source_rgb(backcolor/255.0,backcolor/255.0,backcolor/255.0),
    } 
}*/
