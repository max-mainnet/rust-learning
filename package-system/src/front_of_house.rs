// private
pub mod hosting;

fn some_functon() {}

mod serving {
    pub fn take_order() {
        super::eat_at_bed();
    }
    fn serve_order() {}
    fn take_payment() {}
}

fn eat_at_table() {
    hosting::add_to_waitlist();
}

fn eat_at_bed() {
    eat_at_table();
}
