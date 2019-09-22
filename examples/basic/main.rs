use process::*;
fn main() {
    let mut canvas = Canvas::createCanvas(800,600);
    canvas.background(Color::from(50,0,0));
    canvas.strokeWeight(5);
    canvas.fill(Color::from(255,0,100));
    canvas.point(50,50);
    canvas.circle(250,250,50);
    canvas.noFill();
    canvas.rect(100,100,150,30);
    canvas.show();
}
