pub struct Paths{
    input_file : String, 
    output_file: String, 
}

pub enum Command{
    Invalid(String),
    Indent(Paths),
    Help,
}

pub fn parse_command(input_command : &str) -> Command {
    
}
