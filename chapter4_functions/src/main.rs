fn add(a: i32, b: i32) -> i32 {
    a + b // ; မပါရင် return ဖြစ်သွားတယ်
}

fn main() {
    let sum = add(10, 20);
    println!("ပေါင်းလဒ်: {}", sum);

    // if ကို expression အဖြစ် အသုံးပြုခြင်း
    let score = 75;
    let result = if score >= 50 { "အောင်မြင်" } else { "ကျရှုံး" };
    println!("ရလဒ်: {}", result);

    // for loop ဖြင့် array ပတ်ခြင်း
    let numbers = [10, 20, 30];
    for num in numbers {
        println!("နံပါတ်: {}", num);
    }
}

