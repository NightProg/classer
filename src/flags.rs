#[derive(Debug, Clone, PartialEq, Default)]
pub struct Flags<T>
where
    T: std::ops::BitAnd<Output = T> + std::ops::BitOr<Output = T> + PartialEq + Clone + Default,
{
    pub flags: T,
}

impl<T> Flags<T>
where
    T: std::ops::BitAnd<Output = T> + std::ops::BitOr<Output = T> + PartialEq + Clone + Default,
{
    pub fn new(flags: T) -> Flags<T> {
        Flags { flags }
    }
    pub fn is_set(&self, flag: T) -> bool {
        self.flags.clone() & flag.clone() == flag.clone()
    }

    pub fn set(&mut self, flag: T) {
        self.flags = self.flags.clone() | flag;
    }

    pub fn get_all_flags(&self, possible_flags: Vec<T>) -> Vec<T> {
        possible_flags
            .into_iter()
            .filter(|flag| self.is_set(flag.clone()))
            .collect()
    }
}
