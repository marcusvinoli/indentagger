pub struct OptionCode {
    input_string : Option<str>,
    option_char : Option<str>,
    option_alias : Option<str>, 
    value : Option<str>
    present : Option<bool>
}

impl OptionCode{
    pub fn new(option_char : char){
        input_string : Option::None;
        option_alias : Option::None;
        value : Option::None;
        present : Option::None;
    }

    pub fn set_alias(alias_str : &str){
        &self.option_alias = Some(alias_str)
    }

    pub fn extract_value(input_str : &str) -> Option<String> {
        set_input_string(input_str);
        search_on_input_option_char();
        search_on_input_option_alias();
        if is_option_code_found() {
            println!("Okay!");
        } else {
            println!("Not okay...");
        }     
    }

    fn set_input_string(input_str : &str) {
        &self input_string = Some(input_str);
    }

    fn search_on_input_option_char() {
        let start_index : match search_on_input(&self.option_char) {
            Option::None => {
                Option::None
            }
            Option::Some(x) => {
                x
            }
        }
    }

    fn search_on_input(&str_to_search : Option<str>) -> Option<usize> {
        let search_element : str = &str_to_search.unwrap();
        let search_string : str = &self.input_string.unwrap();
        search_string.find(search_element)
    }

    fn is_option_code_found() -> bool {
        match &self.present {
            Option::None => {
                false
            }
            _ => {
                true
            }
        }
    }
}