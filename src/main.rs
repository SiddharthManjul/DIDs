mod functions;

// use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint16, FheUint32};
// use tfhe::prelude::*;

use functions::converter_functions::{string_to_binary, remove_symbol};

fn main() {

    // let config = ConfigBuilder::default().build();

    // let (client_key, server_key) = generate_keys(config);
    // println!("{:?}", client_key);

    // set_server_key(server_key);

    // let clear_a = 27u16;
    // let clear_b = 34u32;

    // let a = FheUint16::encrypt(clear_a, &client_key);
    // let b = FheUint32::encrypt(clear_b, &client_key);

    // let a = &a;
    
    // let b: FheUint16 = b.cast_into();
    // let b = &b;
    
    // let add =  a + b;

    // let decrypted_add: u16 = add.decrypt(&client_key);

    // let clear_add = clear_a + clear_b as u16;
    // println!("{:?}", clear_add);
    // assert_eq!(decrypted_add, clear_add);



    let msg: &str = "hi, How are you?";
    let converted_msg: Vec<u32> = string_to_binary(&msg);
    println!("Original message is {:?}", msg);
    print!("{:#?}", converted_msg);

    let sum: u32 = converted_msg.iter().sum();
    println!("{:?}", sum);

    let date = "99/99/9999";

    match remove_symbol(date) {
        Ok(integers) => {
            println!("Parsed Integers: {:?}", integers);
            let total: u16 = integers.iter().sum();
            println!("{}", total);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    };

}
