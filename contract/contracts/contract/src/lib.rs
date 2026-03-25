#![no_std]
use soroban_sdk::{contract, contractimpl, Env, String, Symbol};

#[contract]
pub struct VeriReviewContract;

#[contractimpl]
impl VeriReviewContract {
    /// Giao dịch ON-CHAIN duy nhất: Lưu mã Hash của bài đánh giá
    pub fn post_review(env: Env, provider_id: String, review_hash: String) {
        // 1. Lưu mã Hash vào bộ nhớ vĩnh viễn của Stellar
        // Chúng ta dùng provider_id (ví dụ: "hotel_01") làm chìa khóa
        env.storage().persistent().set(&provider_id, &review_hash);
        
        // 2. Bắn ra một sự kiện (Event) để xác nhận giao dịch thành công
        // Điều này giúp App ở phía người dùng biết rằng dữ liệu đã lên chuỗi
        env.events().publish(
            (Symbol::new(&env, "review_saved"), provider_id), 
            review_hash
        );
    }

    /// Hàm phụ để kiểm tra dữ liệu đã lưu (Read-only)
    pub fn check_review(env: Env, provider_id: String) -> String {
        env.storage()
            .persistent()
            .get(&provider_id)
            .unwrap_or(String::from_str(&env, "Not Found"))
    }
}