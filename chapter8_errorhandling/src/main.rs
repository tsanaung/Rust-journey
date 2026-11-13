// 0 ဖြင့် စားမိရင် Error String ပြန်ပေးပြီး၊ အောင်မြင်ရင် f64 အဖြေ ပြန်ပေးမည့် function
fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err(String::from("သုည (0) ဖြင့် စား၍ မရပါ!"))
    } else {
        Ok(numerator / denominator)
    }
}

fn main() {
    let test_cases = [(10.0, 2.0), (10.0, 0.0)];

    for (a, b) in test_cases {
        let result = divide(a, b);

        // match ဖြင့် Ok ရော Err ပါ စနစ်တကျ ခွဲခြမ်းကိုင်တွယ်ခြင်း
        match result {
            Ok(value) => println!("{} / {} = {}", a, b, value),
            Err(error_msg) => println!("အမှားတွေ့ရှိ: {}", error_msg),
        }
    }
}
