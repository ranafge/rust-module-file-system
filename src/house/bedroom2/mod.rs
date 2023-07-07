use super::bedroom1;

pub fn is_litht_on() -> bool {
    true
}
pub fn neighbour_bedroom_ligh_on() -> bool {
    use super::bedroom1;
    bedroom1::is_litht_on()
}