#[allow(non_camel_case_types)]
pub enum ed {
    e, 
    d,
}

#[allow(non_snake_case)]
pub struct EncryptOrDecrypt {
    pub Ans: String,
}

impl EncryptOrDecrypt {
    #[allow(unreachable_code)]
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


#[allow(non_camel_case_types)]
pub enum rw {
    r,
    w
}

#[allow(non_snake_case)]
pub struct RandomOrType {
    pub Ans: String,
}

#[allow(unreachable_code)]
impl RandomOrType {
    pub fn new(answer: String) -> rw {
        if answer.trim() != "r" && answer.trim() != "w" {
            panic!("please type an r or a w.");
        } else if &answer[..] == "r" {
            return rw::r;
        } else if &answer[..] == "w" {
            return rw::w;
        } else {
            panic!("please type an r or a w.");
            return rw::r;
        }
    }
}


pub struct Key {
    pub ans: String,
}

impl Key {
    pub fn new(answer: String) -> String {
        if answer.trim().len() != 32 {
            panic!("please type a 32 character key.");
        } else {
            return answer;
        }
    }
}

pub struct Nonce {
    pub ans: String,
}

impl Nonce {
    pub fn new(answer: String) -> String {
        if answer.trim().len() != 12 {
            panic!("please type a 12 character nonce");
        } else {
            return answer;
        }
    }
}