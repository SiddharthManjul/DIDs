mod functions;

use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint16, FheUint32};
use tfhe::prelude::*;

use functions::covert_binary;

fn main() {

    let config = ConfigBuilder::default().build();

    let (client_key, server_key) = generate_keys(config);
    println!("{:?}", client_key);

    set_server_key(server_key);

    let clear_a = 27u16;
    let clear_b = 34u32;

    let a = FheUint16::encrypt(clear_a, &client_key);
    let b = FheUint32::encrypt(clear_b, &client_key);

    let a = &a;
    
    let b: FheUint16 = b.cast_into();
    let b = &b;
    
    let add =  a + b;

    let decrypted_add: u16 = add.decrypt(&client_key);

    let clear_add = clear_a + clear_b as u16;
    println!("{:?}", clear_add);
    assert_eq!(decrypted_add, clear_add);

    println!("Hello, world!");
}
