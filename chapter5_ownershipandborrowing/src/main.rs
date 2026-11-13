// fn main() {
//     let s1 = String::from("hello");

//     // s1 ကို ပိုင်ဆိုင်မှု မပေးဘဲ ခဏငှားသုံးမယ် (&s1 ဟာ Reference ဖြစ်ပါတယ်)
//     let len = calculate_length(&s1);

//     // s1 ကို ပိုင်ဆိုင်မှု မဆုံးရှုံးသေးလို့ ဒီနေရာမှာ ဆက်သုံးနိုင်ပါသေးတယ်
//     println!("စာသား: '{}' ရဲ့ အရှည်က {} ဖြစ်ပါတယ်", s1, len);

//     // အကယ်၍ & မပါဘဲ 'let s2 = s1;' လို့ ရေးလိုက်ရင် s1 ဟာ s2 ဆီ Move ဖြစ်သွားပြီး s1 သေသွားပါမယ်
// }

// // &String လို့ ရေးထားတာကြောင့် s1 ရဲ့ ပိုင်ဆိုင်မှုကို မယူဘဲ borrow (ငှား) ပဲ လုပ်တာပါ
// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world!"); // စာသား အနောက်မှာ ထပ်ပေါင်းထည့်ခြင်း
}

fn main() {
    let mut message = String::from("hello");

    // &mut ဖြင့် ပြင်ဆင်ခွင့် ပေးပြီး ငှားလိုက်တယ်
    change(&mut message);

    println!("ပြင်ဆင်ပြီး စာသား: {}", message);

    // စမ်းသပ်ချက်- အောက်က နှစ်ကြောင်းကို un-comment လုပ်ပြီး run ကြည့်ပါ
    // let r1 = &message; // ဖတ်ခွင့်
    // let r2 = &mut message; // ပြင်ခွင့် (error တက်ပါလိမ့်မယ်)
    // println!("{}, {}", r1, r2);
}
