#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

use hex_literal::hex;
use streamsha::hash_state::{HashState, Sha256HashState};
use streamsha::traits::{Resumable, StreamHasher};
use streamsha::{Sha1, Sha256};
extern crate hex_slice;
#[macro_use]
extern crate hex_literal;
// extern crate lazy_static;


#[ink::contract]
mod flipper {
    use ink_storage::traits::{
        PackedLayout,
        SpreadLayout,
    };


    #[derive(
        Debug,
        // Copy,
        Clone,
        PartialEq,
        Eq,
        scale::Encode,
        scale::Decode,
        SpreadLayout,
        PackedLayout,
    )]
    #[cfg_attr(
        feature = "std",
        derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
    )]
    #[ink(storage)]
    
    pub struct Flipper {
        value: i32,
        cerf: streamsha::Sha256,
    }


    impl Flipper {
        #[ink(constructor)]
        
        pub fn new(init_value: i32) -> Self {
            Self {
                value: init_value,
                cerf: streamsha::Sha256::new(),
            }
        }

        // #[ink(constructor)]
        // pub fn default() -> Self {
        //     Self::new(Default::default())
        // }

        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = self.value + 1;
        }

        // #[ink(message)]
        // pub fn get(&self) -> i32 {
        //     self.value
        // }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use ink_lang as ink;

        // #[ink::test]
        // fn default_works() {
        //     let flipper = Flipper::default();
        // }

        #[ink::test]
        fn it_works() {
            let mut flipper = Flipper::new(1);
            flipper.flip();
        }
    }
}
