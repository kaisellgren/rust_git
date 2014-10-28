use meta::Meta;

pub trait HasMeta {
    fn get_meta(&self) -> &Meta;
}