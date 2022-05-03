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
    // struct ThisIsATupleStrucThatHoldsU8SliceAndMakeThemUsable<'a>(&'a [u8]);
    struct User {
        id: i32,
    }

    #[ink(storage)]
    
    pub struct Flipper {
        value: i32,
        cerf: User,
    }


    impl Flipper {
        #[ink(constructor)]
        pub fn new(init_value: i32, init_user: User) -> Self {
            Self {
                value: init_value,
                cerf: init_user,
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

        #[ink(message)]
        pub fn get(&self) -> i32 {
            self.value
        }
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
            let mut flipper = Flipper::new(1, User{id: 1});
            flipper.flip();
        }
    }
}
