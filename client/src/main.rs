use std::fmt::format;
use md5;

mod Input;
mod Output;
struct StructInput {
    complexity: u32,
    message: String,
}
fn main() {
    let input = StructInput {
        complexity: 9,
        message: String::from("hello")
    };
    let mut name_in_binary = "".to_string();
    for seed in 0..1 {
        let result = format!("{:016X}", seed);
        let res = format!("{}{}\n",result.to_string(),input.message.to_string());
        print!("{}\n",res);
        let digest = md5::compute(res);

        let hashcode = format!("{:032X}", digest);
        print!("{}\n",hashcode);


        /*for character in hashcode.clone().into_bytes() {
            print!("{}\n",character);
            name_in_binary += &format!("0{:b} ", character);
        }*/
        print!("{}\n",name_in_binary);



    }



    /*while{
        stringconcaténer
        hashcode = md5(stringFFFffgv)
 erateSeed(seed)
    }*/
    //let result = generateSeed(seed);
//return server


}
/*fn generateSeed(mut seed:u64) ->String{
    seed+=1;
    return format!("{:016X}", seed);
}*/

