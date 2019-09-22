extern crate cairo;
extern crate gio;
extern crate gtk;
#[allow(unused_imports)]
use std::env::args;
#[allow(unused_imports)]
use std::f64::consts::PI;
#[allow(unused_imports)]
use gio::prelude::*;
#[allow(unused_imports)]
use gtk::prelude::*;
#[allow(unused_imports)]
use gtk::DrawingArea;
#[allow(unused_imports)]
use gtk::{Window,WindowType, Label};
#[allow(unused_imports)]
use gtk::{ApplicationWindow,Application};
#[allow(unused_imports)]
use cairo::{Context, FontSlant, FontWeight};
#[allow(unused_imports)]
use gtk::main_quit;

#[derive(Clone,Copy)]
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
pub struct Canvas{
    window:Window,
    draw_area:DrawingArea,
    width:i32,
    height:i32,
    weight:u8,
    background:Color,
    fill:bool,
    fill_color:Color,
}
impl Canvas{
    ///createCanvas as in the js lib p5 and java's processig takes 2 arguments width and height and
    ///creates a canvas based upon them
    pub fn createCanvas(width:i32,height:i32)->Canvas{
        if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        }
        let window:Window = Window::new(WindowType::Toplevel);
        window.set_title("myCanvas");
        window.set_border_width(0);
        window.set_position(gtk::WindowPosition::Center);
        window.set_default_size(width,height);
        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });
        let draw_area = Box::new(DrawingArea::new)();
        Canvas{window,draw_area,width,height,weight:1u8,background:Color::from(255,255,255),fill:false,fill_color:Color::from(255,255,255)}
    }
    ///sets the background color via the color struct
    pub fn background(&mut self,color:Color){
        self.fill_color=color.clone();
        self.background = color.clone();
        self.draw_area.connect_draw(move|_, cr| {
            cr.set_source_rgb(color.r as f64/255.0,color.g as f64/255.0 ,color.b as f64/255.0);
            cr.paint();
            Inhibit(false)
        });
    }
    ///sets the width of the lines ad borders that the shapes creates next have
    pub fn strokeWeight(&mut self,weight:u8){
        self.weight=weight;
    }
    ///recieves x,y coordinates and a second pair of coordinates and streches a line between them
    pub fn line(&self,x1:u16,y1:u16,x2:u16,y2:u16){
        let weight1 = self.weight.clone(); 
        self.draw_area.connect_draw(move|_, cr| {
            cr.move_to(y1 as f64,x1 as f64);
            cr.set_line_width(weight1 as f64);
            cr.rel_line_to(y2 as f64/*+0.001*weight1 as f64*/,x2 as f64 /*+ 0.001*weight1 as f64*/);
            cr.stroke();
            Inhibit(false)
        });
    }
    ///recieves x,y coordinates and width and height and creates a rectangle
    pub fn rect(&self,x1:u16,y1:u16,width:u16,height:u16){
        let weight1 = self.weight.clone(); 
        let fill1 = self.fill.clone();
        let fill_col = self.fill_color.clone();
        let col = self.background.clone();
        self.draw_area.connect_draw(move|_, cr| {
            cr.move_to(y1 as f64,x1 as f64);
            cr.set_line_width(weight1 as f64);
            cr.line_to((x1+width) as f64,y1 as f64);
            cr.line_to((x1+width) as f64,(y1+height) as f64);
            cr.line_to(x1 as f64,(y1+height) as f64);
            cr.line_to(x1 as f64,(y1-weight1 as u16/2) as f64);
            if fill1{
                cr.set_source_rgb(fill_col.r as f64/255.0,fill_col.g as f64/255.0 ,fill_col.b as f64/255.0);
                cr.fill();
                cr.set_source_rgb(col.r as f64/255.0,col.g as f64/255.0 ,col.b as f64/255.0);
            }
            cr.stroke();
            Inhibit(false)
        });
    }
    ///recieves x and y coordinates and puts a circle in the location
    pub fn point(&self,x1:u16,y1:u16){
        let height1= self.height.clone();
        let width1= self.width.clone();
        let weight1 = self.weight.clone(); 
        let fill1 = self.fill.clone();
        let fill_col = self.fill_color.clone();
        let col = self.background.clone();
        self.draw_area.connect_draw(move|_, cr| {
            if fill1{
                cr.set_source_rgb(fill_col.r as f64/255.0,fill_col.g as f64/255.0 ,fill_col.b as f64/255.0);
                cr.arc(x1 as f64,y1 as f64 , weight1 as f64, 0., 2f64 * PI);
                cr.fill();
                cr.set_source_rgb(col.r as f64/255.0,col.g as f64/255.0 ,col.b as f64/255.0);
            }else{
                cr.set_source_rgb(0f64,0f64,0f64);
                cr.arc(x1 as f64,y1 as f64 , weight1 as f64, 0., 2f64 * PI);
                cr.fill();
                cr.set_source_rgb(col.r as f64/255.0,col.g as f64/255.0 ,col.b as f64/255.0);
            }
            Inhibit(false)
        });
    }
    ///still not working
    pub fn ellipse(&self,x1:u16,y1:u16,foc1:u16,foc2:u16){
        let weight1 = self.weight.clone(); 
        let fill1 = self.fill.clone();
        let fill_col = self.fill_color.clone();
        let col = self.background.clone();
        self.draw_area.connect_draw(move|_, cr| {
            if fill1{
                cr.set_source_rgb(fill_col.r as f64/255.0,fill_col.g as f64/255.0 ,fill_col.b as f64/255.0);
                cr.arc(x1 as f64,y1 as f64 , (foc1 as f32/foc2 as f32) as f64, 0., 2f64 * PI);
                cr.fill();
                cr.set_source_rgb(col.r as f64/255.0,col.g as f64/255.0 ,col.b as f64/255.0);
            }else{
                cr.set_source_rgb(0f64,0f64,0f64);
                cr.arc(x1 as f64,y1 as f64 , foc1 as f64, 0., (foc2 as f32/foc1 as f32) as f64 * PI);
                cr.stroke();
                cr.set_source_rgb(col.r as f64/255.0,col.g as f64/255.0 ,col.b as f64/255.0);
            }
            Inhibit(false)
        });
    }
    ///function that takes location(x,y coordinates) and a radius and creates a circle in the
    ///requested spot
    pub fn circle(&self,x1:u16,y1:u16,rad:u16){
        let weight1 = self.weight.clone(); 
        let fill1 = self.fill.clone();
        let fill_col = self.fill_color.clone();
        let col = self.background.clone();
        self.draw_area.connect_draw(move|_, cr| {
            if fill1{
                cr.set_source_rgb(fill_col.r as f64/255.0,fill_col.g as f64/255.0 ,fill_col.b as f64/255.0);
                cr.arc(x1 as f64,y1 as f64 , rad as f64, 0., 2f64 * PI);
                cr.fill();
                cr.set_source_rgb(col.r as f64/255.0,col.g as f64/255.0 ,col.b as f64/255.0);
            }else{
                cr.set_source_rgb(0f64,0f64,0f64);
                cr.arc(x1 as f64,y1 as f64 , rad as f64, 0., 2f64 * PI);
                cr.stroke();
                cr.set_source_rgb(col.r as f64/255.0,col.g as f64/255.0 ,col.b as f64/255.0);
            }
            Inhibit(false)
        });
    }
    ///a second option for creating a circle
    pub fn circ(&self,x1:u16,y1:u16,rad:u16){
        self.circle(x1,y1,rad);
    }
    ///a second option to creating a rectangle
    pub fn rectangle(&self,x1:u16,y1:u16,width:u16,height:u16){
        self.rect(x1,y1,width,height);
    }
    ///receives a color and enables filling of shapes
    pub fn fill(&mut self,color:Color){
        self.fill=true;
        self.fill_color=color;
    }
    ///shuts of fill()
    pub fn noFill(&mut self){
        self.fill=false;
    }
    ///function to show to canvas in the window
    pub fn show(self){
        self.window.add(&self.draw_area);
        self.window.show_all();
        gtk::main();
    }
}

