use enigo::Enigo;
use polyfit_rs::polyfit_rs::polyfit;
use std::{thread::sleep, time::Duration};

const RECORDING_TIME: Duration = Duration::from_secs(2);
const DT: Duration = Duration::from_millis(5);
const ITERATIONS: usize = (RECORDING_TIME.as_millis() / DT.as_millis()) as usize;

const MAX_X: f64 = 2560.;
const MAX_Y: f64 = 1440.;

fn main() {
    let mut x_values = [0.; ITERATIONS];
    let mut y_values = [0.; ITERATIONS];
    let mut t_values = [0.; ITERATIONS];
    sleep(Duration::from_secs(2));
    println!("{:?}", Enigo::mouse_location());
    let mut i = 0;
    println!("Started Recording");
    while i < ITERATIONS {
        let (x, y) = Enigo::mouse_location();
        let x: f64 = x.try_into().unwrap();
        let y: f64 = y.try_into().unwrap();
        let x = x / MAX_X;
        let y = y / MAX_Y;
        x_values[i] = x;
        y_values[i] = y;
        t_values[i] = DT.as_secs_f64() * (i as f64);
        sleep(DT);
        i += 1;
    }
    println!("Done, doing calculations...");

    let power = 3; // arbitrary

    let fitx = polyfit(&t_values, &x_values, power);
    let fity = polyfit(&t_values, &y_values, power);

    println!("{:?} {:?}", fitx, fity)
}
