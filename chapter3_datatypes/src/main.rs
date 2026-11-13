fn main() {
    // Tuple ဥပမာ (type မတူတာတွေ ပေါင်းလို့ရ)
    let person: (&str, i32, bool) = ("Aung Aung", 25, true);
    println!("အမည်: {}, အသက်: {}", person.0, person.1);

    // Array ဥပမာ (type တူရမယ်၊ အရှည်သတ်မှတ်ပြီးသား)
    let scores: [i32; 3] = [95, 88, 100];
    println!("ပထမဆုံး ရမှတ်: {}", scores[0]);
    println!("Array အရှည်: {}", scores.len());
}
