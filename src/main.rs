
pub mod mouse;
mod ui;

fn main() {
    ui::launch();
    println!("A4Tech Mouse Control");
    mouse::test();
}
