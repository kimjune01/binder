//! Minimal contract kernel used by the typed Binder demo.

#[derive(Debug, PartialEq)]
pub struct Escrow {
    pub held: u64,
    pub recipient: u64,
}

impl Escrow {
    pub fn release(&mut self, amount: u64, buyer_approved: bool, seller_approved: bool) {
        if !(buyer_approved && seller_approved) {
            return;
        }
        self.held -= amount;
        self.recipient += amount;
    }
}
