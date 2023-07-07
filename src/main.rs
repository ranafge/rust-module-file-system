mod house;
mod house2;
use house::{bedroom1, bedroom2};
use house2::{bedroom3, bedroom4};
fn main() {
   println!("{}", bedroom1::is_litht_on());
   println!("{}", bedroom2::is_litht_on());
   println!("{:?}", bedroom2::neighbour_bedroom_ligh_on());
   println!("{}", bedroom1::pasher_basha_ligh_on());
   println!("{}", bedroom3::house2_fan_on());
   println!("{}", bedroom4::house2_fan_on());

}