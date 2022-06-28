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

    for seed in 0..100 {
        let result = format!("{:016X}", seed);
        let res = format!("{}{}\n",result.to_string(),input.message.to_string());

        let digest = md5::compute(res);
        print!("{}\n",format!("{:X}", digest));
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

