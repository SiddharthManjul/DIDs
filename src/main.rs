mod functions;

use functions::functions::{
    add_numbers, concatenate_string, remove_space, remove_symbol, string_slice,
    string_to_binary_sum,
};

use bincode;
use tfhe::{prelude::*, CompressedCompactPublicKey};
use tfhe::shortint::parameters::{COMP_PARAM_MESSAGE_2_CARRY_2, PARAM_MESSAGE_2_CARRY_2};
use tfhe::{
    set_server_key, ClientKey, CompressedCiphertextList, CompressedCiphertextListBuilder,
    ConfigBuilder, FheUint64, ServerKey, CompressedServerKey
};

/// Inputs - String, Numbers
/// Conversions - Strings to Numbers -- Require every input to be in numbers, if using ascii values
//// add all the values and use the sum as the input for encryption.
/// Values
/// 1. First_Name and Last_Name as a single entity Full_Name. Convert string to binary (ascii) and store in a vector
/// then add all the stored values and encrypt the resulted sum.
/// 2. Unique Identifier number taken as it is for encryption.
/// 3. Address converted to ascii before encryption.
/// 4. Date of Birth remove symbols / or - before encryption.

fn main() {
    // DATA
    let firstname = "Siddharth";
    let lastname = "Manjul";

    // Mask_ID Name Code
    let reversed_firstname = string_slice(&firstname);
    let reversed_lastname = string_slice(&lastname);
    let _name_code = concatenate_string(&reversed_lastname, &reversed_firstname);
    // println!("Name Code for Mask_ID: {:?}", name_code.to_uppercase());

    // ED1: Name
    let fullname = concatenate_string(&firstname, &lastname);
    let fullname_binary_sum = string_to_binary_sum(&fullname);
    // println!("Name: {:?}", fullname_binary_sum);

    // ED2: Unique Identifier
    let unique_identifier: u64 = 123456789012;
    // println!("Unique Identifier: {}", unique_identifier);

    // ED3: Address
    let address = "123 Street NY";
    let address_no_space = &remove_space(&address);
    let address_binary_sum: u64 = string_to_binary_sum(address_no_space);
    // println!("Address: {:?}", address_binary_sum);

    // ED4: Date of Birth
    let date = "99/99/9999";
    let symbol_removed_date = remove_symbol(date);
    let date_sum = add_numbers(symbol_removed_date);
    // println!("Birth Date: {:?}", date_sum);

    // ENCRYPTION
    let config = ConfigBuilder::with_custom_parameters(PARAM_MESSAGE_2_CARRY_2, None)
        .enable_compression(COMP_PARAM_MESSAGE_2_CARRY_2)
        .build();

    let client_key = ClientKey::generate(config);

    let compressed_server_key = CompressedServerKey::new(&client_key);
    println!("compressed size: {}", bincode::serialize(&compressed_server_key).unwrap().len());
    let server_key = compressed_server_key.decompress();

    set_server_key(server_key);

    let compressed_client_key = CompressedCompactPublicKey::new(&client_key);
    println!("compressed size: {}", bincode::serialize(&compressed_client_key).unwrap().len());

    // Name Encryption
    let ed1 = FheUint64::encrypt(fullname_binary_sum, &client_key);

    // Unique Identifier Encryption
    let ed2 = FheUint64::encrypt(unique_identifier, &client_key);

    // Address Encryption
    let ed3 = FheUint64::encrypt(address_binary_sum, &client_key);

    // Date of Birth Encryption
    let ed4 = FheUint64::encrypt(date_sum, &client_key);

    let compressed_data = CompressedCiphertextListBuilder::new()
        .push(ed1)
        .push(ed2)
        .push(ed3)
        .push(ed4)
        .build()
        .unwrap();

    let serialized_compressed_data = bincode::serialize(&compressed_data).unwrap();
    println!("Serialized size: {} bytes", serialized_compressed_data.len());

    let compressed_list: CompressedCiphertextList = bincode::deserialize(&serialized_compressed_data).unwrap();

    let a: FheUint64 = compressed_list.get(1).unwrap().unwrap();
    let a: u64 = a.decrypt(&client_key);
    println!("{:?}", a);
}
