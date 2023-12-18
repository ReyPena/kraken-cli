use crate::modules::header_module::generate_header;

pub fn init_interactive_menu((width, _height): (usize, usize)) {
    print!("{}", generate_header(width));

    println!("Welcome to interactive mode!");

    println!()
}

fn generate_item_menu() {

}


{
    "version": #
    "defaults": {
        "[commandName]": {
            "args" : {
                key: value
            },
            "envVars": {
                key: value
            }
        }
    }
    "commands": [{
        "commandName": "", //command name
        "binaryPath": "", //executable or script file
        "script": "", //binaries
    }]
}