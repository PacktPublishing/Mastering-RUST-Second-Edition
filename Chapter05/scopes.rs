// scopes.rs

fn main() { 
    let level_0_str = String::from("foo"); 
    {  
        let level_1_number = 9; 
        { 
            let mut level_2_vector = vec![1, 2, 3];
            level_2_vector.push(level_1_number);    // can access
        } // level_2_vector goes out of scope here 

        level_2_vector.push(4);    // no longer exists
    } // level_1_number goes out of scope here
} // level_0_str goes out of scope here
