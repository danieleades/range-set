use range_set::RangeSet;

fn main() {
    let mut set = RangeSet::default();

    set.insert(1_u32);
    set.insert(3_u32);
    set.insert(4_u32);
    set.insert(5_u32);
    set.insert(6_u32);
    set.insert(8_u32);

    println!("{:#?}", set);
}
