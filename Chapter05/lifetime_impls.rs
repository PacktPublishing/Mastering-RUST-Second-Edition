// lifetime_impls.rs

#[derive(Debug)]
struct Number<'a> { 
    num: &'a u8 
}

impl<'a> Number<'a> { 
    fn get_num(&self) -> &'a u8 { 
        self.num 
    }  
    fn set_num(&mut self, new_number: &'a u8) { 
        self.num = new_number 
    }
}

fn main() {
    let a = 10;
    let mut num = Number { num: &a };
    num.set_num(&23);
    println!("{:?}", num.get_num());
}
