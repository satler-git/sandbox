fn main() {
    let flags = xflags::parse_or_exit! {
        /// Decrease the input number
        optional -d, --decrease
        /// num to succ
        required num: u32
    };

    if flags.decrease {
        println!("{}", flags.num.checked_sub(1).unwrap_or_default());
    } else {
        println!("{}", flags.num + 1);
    }
}
