const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 1;

fn main() {
    let (missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);
    println!("{} missiles left", missiles - ready);
    // add another variable but don't use it
    let _ = STARTING_MISSILES;
}
