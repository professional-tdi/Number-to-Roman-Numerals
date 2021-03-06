fn main() {
    let x: u32 = 4;
    println!("{}", convert_num_to_roman(x))
}

fn convert_num_to_roman(mut num: u32) -> String {
    let values: [u32; 13] = [1000,900,500,400,100,90,50,40,10,9,5,4,1];
    let roman_letters:[&str; 13] = ["M","CM","D","CD","C","XC","L","XL","X","IX","V","IV","I"];

    let mut ending: String = String::new();

    for i in 0..values.len() {
        while num >= values[i] {
            num -= values[i];
            ending.push_str(roman_letters[i]);
        }
    }
    ending
}
//fn convert_num_to_roman2(&mut num:u32)->String{let values:[u32;13]=[1000,900,500,400,100,90,50,40,10,9,5,4,1];let roman_letters:[&str;13]=["M","CM","D","CD","C","XC","L","XL","X","IX","V","IV","I"];let mut ending:String=String::new();for i in 0..values.len(){while num>=values[i]{num=num-values[i];ending.push_str(roman_letters[i]);}}ending}