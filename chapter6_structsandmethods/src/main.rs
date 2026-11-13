// Struct ကို print ထုတ်ကြည့်နိုင်အောင် Debug trait ထည့်ထားခြင်း
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Rectangle ပေါ်မှာ အလုပ်လုပ်မယ့် methods များ
impl Rectangle {
    // Constructor (JS/PHP က static method သို့မဟုတ် new ကဲ့သို့)
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    // Instance method (&self ဖြင့် rectangle data ကို borrow လုပ်သည်)
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle::new(30, 50);

    println!("စတုဂံ အချက်အလက်: {:?}", rect);
    println!("စတုဂံ၏ ဧရိယာ: {} စတုရန်းယူနစ်", rect.area());
}
