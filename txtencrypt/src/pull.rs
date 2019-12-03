#[allow(non_camel_case_types)]
pub enum ed {
    e, 
    d,
}

#[allow(non_snake_case)]
pub struct EncryptOrDecrypt {
    pub Ans: String,
}

#[allow(unreachable_code)]
impl EncryptOrDecrypt {
    pub fn new(answer: String) -> ed {
        if answer != "e".to_string() && answer != "d".to_string() {
            panic!("please type an e or a d!");
        } else if &answer[..] == "e" {
            return ed::e;
        } else if &answer[..] == "d" {
            return ed::d;
        } else {
            panic!("answer was not e or d!!!");
            return ed::d;
        }
    }
}