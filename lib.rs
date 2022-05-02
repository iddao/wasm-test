#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;



#[ink::contract]
mod flipper {
    use hex_literal::hex;
    extern crate hex_slice;
    

   
    #[ink(storage)]
    // pub struct Flipper {
    //     value: bool,
        // AUTH_CERT: struct TheTupleStruct(&[u8]),
    // }
    named_tuple!(
        #[derive(SpreadLayout)]
        pub struct Flipper {
            value: bool,
            AUTH_CERT: &'static [u8],
        }
    );

    impl Flipper {
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self {
                value: init_value,
                AUTH_CERT: &hex!("308"),
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use ink_lang as ink;

        #[ink::test]
        fn default_works() {
            let flipper = Flipper::default();
        }

        #[ink::test]
        fn it_works() {
            let mut flipper = Flipper::new(false);
            flipper.flip();
        }
    }
}
