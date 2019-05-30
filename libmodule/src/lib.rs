#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod libfoo {
    pub fn gimme_a_song() -> String {
        String::from("Ace of Spades")
    }
}
