mod functions;

use functions::functions::{
    add_numbers, concatenate_string, remove_space, remove_symbol, string_slice,
    string_to_binary_sum,
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
    let name_code = concatenate_string(&reversed_lastname, &reversed_firstname);
    println!("Name Code for Mask_ID: {:?}", name_code.to_uppercase());
    
    // ED1: Name Encryption
    let fullname = concatenate_string(&firstname, &lastname);
    let fullname_binary_sum = string_to_binary_sum(&fullname);
    println!("Name: {:?}", fullname_binary_sum);

    // ED2: Unique Identifier Encryption
    let unique_identifier: u64 = 123456789012;
    println!("Unique Identifier: {}", unique_identifier);

    // ED3: Address Encryption
    let address = "123 Street NY";
    let address_no_space = &remove_space(&address);
    let address_binary_sum: u64 = string_to_binary_sum(address_no_space);
    println!("Address: {:?}", address_binary_sum);

    // ED4: Date of Birth Encryption
    let date = "99/99/9999";
    let symbol_removed_date = remove_symbol(date);
    let date_sum = add_numbers(symbol_removed_date);
    println!("Birth Date: {:?}", date_sum);

    // ENCRYPTION
    
}
