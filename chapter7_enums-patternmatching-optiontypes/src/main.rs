// #[allow(dead_code)] // Compiler ရဲ့ dead code warning ကို ဖျောက်ပစ်ဖို့

enum PaymentStatus {
    Pending,
    Paid(f64),       // Paid ဖြစ်ရင် ပမာဏ (amount) ပါ ထည့်ထားနိုင်တယ်
    Failed(String),  // Failed ဖြစ်ရင် အကြောင်းပြချက်ပါ ပါနိုင်တယ်
}

fn process_payment(status: PaymentStatus) {
    match status {
        PaymentStatus::Pending => println!("ငွေပေးချေမှုကို စောင့်ဆိုင်းနေပါသည်..."),
        PaymentStatus::Paid(amount) => println!("ငွေကျပ် {:.2} ပေးချေမှု အောင်မြင်ပါသည်!", amount),
        PaymentStatus::Failed(reason) => println!("ငွေပေးချေမှု မအောင်မြင်ပါ: {}", reason),
    }
}

// fn main() {
//     let payment1 = PaymentStatus::Paid(150.75);
//     let payment2 = PaymentStatus::Failed(String::from("လက်ကျန်ငွေ မလုံလောက်ပါ"));

//     process_payment(payment1);
//     process_payment(payment2);

//     // Option ဥပမာ
//     let user_name: Option<String> = Some(String::from("Ko Aung"));
//     match user_name {
//         Some(name) => println!("မင်္ဂလာပါ {}", name),
//         None => println!("အသုံးပြုသူ အမည်မသိပါ"),
//     }
// }

fn main() {
    let payment1 = PaymentStatus::Paid(150.75);
    let payment2 = PaymentStatus::Failed(String::from("လက်ကျန်ငွေ မလုံလောက်ပါ"));
    let payment3 = PaymentStatus::Pending; // Pending ကို အသုံးပြုလိုက်ခြင်း

    process_payment(payment1);
    process_payment(payment2);
    process_payment(payment3); // ခေါ်ယူခြင်း

    // ... Option ဥပမာအပိုင်း
}
