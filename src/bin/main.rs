extern crate osc_midi_bridge;

fn main() {
    //Go to library ASAP.
    match osc_midi_bridge::run() {
        Ok(_) => (),
        Err(err) => println!("Error!\n{}", err),
    }
}
