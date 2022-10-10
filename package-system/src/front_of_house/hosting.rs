pub fn add_to_waitlist() {}
fn seat_at_table() {
    super::serving::take_order();

    crate::front_of_house::serving::take_order();
}
