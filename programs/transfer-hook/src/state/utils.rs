use anchor_lang::prelude::*;

pub fn is_past(end_date: i64) -> bool {
    end_date < Clock::get().unwrap().unix_timestamp
}