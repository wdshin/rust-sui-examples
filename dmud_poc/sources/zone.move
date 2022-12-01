module dmud_poc::zone {
    // The first part of the module is a list of imports. 
    // The second part is a list of types. 
    // The third part is a list of functions. 
    // The fourth part is a list of tests.

    // Part 1: imports
    //use sui::url::{Self, Url};
    //use std::string;
    //use sui::vec_map::{Self,VecMap};
    //use std::vector;

    use sui::object::{UID};
    //use sui::event;
    //use sui::transfer;
    //use sui::tx_context::{Self, TxContext};

    struct Zone has key, store  {
        id: UID,
    }

    // Part3 : functions

    // ===== Events =====

    // ===== Private Functions =====

    // --- Initialization
    

    // ===== Public Functions =====


    // ===== Public View Functions =====
    
    // Part 4: tests
}

