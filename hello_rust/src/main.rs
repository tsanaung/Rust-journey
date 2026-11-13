//fn main() {
//    println!("Hello, world!");
//}

// Chapter 2: Variables & Mutability (မပြောင်းလဲနိုင်ခြင်းနှင့် ပြောင်းလဲနိုင်ခြင်း)
//fn main() {
//    let x = 5;
//    println!("x ရဲ့ တန်ဖိုးက: {}", x);

//     x = 10; // ဒီနေရာမှာ တန်ဖိုး အသစ်ပြောင်းကြည့်မယ်
//     println!("x ရဲ့ တန်ဖိုးအသစ်က: {}", x);
// }

fn main() {
    let y = 5;
    let y = y + 1; // y အသစ်က y အဟောင်းကို shadow (ဖုံးအုပ်) လိုက်တယ်
    {
        let y = y * 2; // block scope ထဲမှာပဲ အကျုံးဝင်တယ်
        println!("Inner scope ထဲက y: {}", y); // ၁၂
    }
    println!("Outer scope ထဲက y: {}", y); // ၆
}


