mod rom;

pub fn init_firmware(headers: rom::Header&) {
    println!("Checking ROM integrity...");
    
    match headers.validate() {
        Ok(()) => println!("Ok!")
        Err(err) => {
            println!("Failed");
            println!("{}", err);
            return (());
        }
    }
}
