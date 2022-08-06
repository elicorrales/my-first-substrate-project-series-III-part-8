#![cfg_attr(not(feature = "std"), no_std)]
use ink_lang as ink;
#[ink::contract]
mod hello_world {
    #[ink(storage)]
    pub struct HelloWorld {}
    impl HelloWorld {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        // notice that both types of logging calls work
        #[ink(message)]
        pub fn sayhello(&mut self) {
            ink_env::debug_message("\n\nHello from sayhello()\n\n");
            ink_env::debug_println!("\n\nHello from sayhello()\n\n");
        }

        #[ink(message)]
        pub fn saybye(&mut self) {
            ink_env::debug_message("\n\nBye from saybye() before panic\n\n");
            ink_env::debug_println!("\n\nBye from saybye() before panic\n\n");
            panic!("Panicking at saybye");
            ink_env::debug_message("\n\nBye from saybye() after panic\n\n");
            ink_env::debug_println!("\n\nBye from saybye() after panic\n\n");
        }
    }
}
