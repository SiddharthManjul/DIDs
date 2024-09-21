use bincode;
use std::io::Cursor;
use tfhe::{ConfigBuilder, ServerKey, generate_keys, set_server_key, FheUint32};
use tfhe::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ConfigBuilder::default().build();

    // Generate client and server keys
    let (client_key, server_key) = generate_keys(config);

    // Encrypt u32 values instead of u8
    let msg1: u32 = 1;
    let msg2: u32 = 0;
    let msg3: u32 = 2;

    let value_1 = FheUint32::encrypt(msg1, &client_key);
    let value_2 = FheUint32::encrypt(msg2, &client_key);
    let value_3 = FheUint32::encrypt(msg3, &client_key);

    // Prepare to send data to the server
    let mut serialized_data = Vec::new();
    bincode::serialize_into(&mut serialized_data, &server_key)?;
    bincode::serialize_into(&mut serialized_data, &value_1)?;
    bincode::serialize_into(&mut serialized_data, &value_2)?;
    bincode::serialize_into(&mut serialized_data, &value_3)?;

    // Simulate sending serialized data to a server and getting back the result
    let serialized_result = server_function(&serialized_data)?;
    let result: FheUint32 = bincode::deserialize(&serialized_result)?;

    // println!("Serialized result: {:?}", serialized_result);

    // Decrypt the result on the client side
    let output: u32 = result.decrypt(&client_key);
    assert_eq!(output, msg1 + msg2 + msg3);
    println!("Decrypted output: {}", output);

    println!("Size of serialized_data: {} bytes", serialized_data.len());
    Ok(())
}

fn server_function(serialized_data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut serialized_data = Cursor::new(serialized_data);

    // Deserialize the server key and encrypted values
    let server_key: ServerKey = bincode::deserialize_from(&mut serialized_data)?;
    let ct_1: FheUint32 = bincode::deserialize_from(&mut serialized_data)?;
    let ct_2: FheUint32 = bincode::deserialize_from(&mut serialized_data)?;
    let ct_3: FheUint32 = bincode::deserialize_from(&mut serialized_data)?;

    // Set the server key in the current thread
    set_server_key(server_key);

    // Perform the addition on the server side
    let result = ct_1 + ct_2 + ct_3;

    // Serialize the result back to be sent to the client
    let serialized_result = bincode::serialize(&result)?;

    Ok(serialized_result)
}
