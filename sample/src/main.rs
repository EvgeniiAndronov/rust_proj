mod core;
mod lagrange;

fn main() {
    let x_c = vec![1.0, 5.2, 16.2, 18.3, 19.0];
    let y_c = vec![2.1, 3.1, 14.7, 19.5, 22.4];

    let res: f64 = lagrange::lagrange_interpolation(17.0,&x_c, &y_c);
    
    println!("x:{}; y:{}",17.0,res);
}

// fn note() {
//     let mut notes: Vec<String> = Vec::new();
//     loop {
//         core::print_menue();
//         let command: String = core::read_input();
//         match command.trim() {
//             "show" => core::show_notes(&notes),
//             "add" => core::add_notes(&mut notes),
//             _ => break,
//         }
//     }
// }