//Constant in rust

fn main() {
    // const APVALUE = 12; //error
    // must define datatype with any constant
    //must be Capitalize every character of its name during declaration

    const FATHERCNIC: i128 = 1234567891011;
    println!("The CNIC of his Father will be: {}", FATHERCNIC);

    const COMPANYNAME: &str = "MT Products";
    println!("My comapony name will be: {}", COMPANYNAME);
}
