#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod helloworld2 {

    #[ink(storage)]
    pub struct HelloWorld2 {}

    impl HelloWorld2 {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn sayhello2(&mut self) {
            ink_env::debug_message("\n\nHello from sayhello2()\n\n");
        }

        #[ink(message)]
        pub fn saybye2(&mut self) {
            ink_env::debug_message("\n\nHello from sayhello2()\n\n");
            panic!("\n\nPanicking at saybye2()\n\n");
        }
    }
}
