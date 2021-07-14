use super::{CdcChange, Thing};
use crate::as_variant;

use sproot::models::Alerts;
use std::{
    io::{Error, ErrorKind},
    ptr::addr_of_mut,
};

impl From<&CdcChange> for Result<Alerts, Error> {
    fn from(data: &CdcChange) -> Result<Alerts, Error> {
        // Create a non initialized variable
        let mut alert = std::mem::MaybeUninit::<Alerts>::uninit();
        // Get the mutable ptr of the previous unitialized variable
        let alert_ptr = alert.as_mut_ptr();

        // Safety
        // - alert_ptr should never be ptr::null_mut()
        //   doing so is Undefined Behavior because it dereferences a NULL pointer.
        assert_ne!(alert_ptr, std::ptr::null_mut());
        // Counter to be sure we got all our fields
        let mut matched: i8 = 0;
        // Iterate over all columns name and get their position
        // -> because their position correspond to their value in columnvalues.
        for (pos, val) in data.columnnames.iter().enumerate() {
            // Convert to str to match against static str
            match val.as_str() {
                "id" => unsafe {
                    // addr_of_mut get the address of the dereferenced struct's field
                    // .write set the value for that memory address
                    addr_of_mut!((*alert_ptr).id).write(
                        // as_variant will get the innver value from the enum Thing
                        // in this case it's a i32 so we need to dereference it
                        // If it's a String or a OptionString we don't need to dereference it,
                        // just need to "clone" it (own it).
                        *as_variant!(&data.columnvalues[pos], Thing::Number)
                            .expect("ID is not an i32"),
                    );
                    // Increment 1 for the matched count
                    matched += 1;
                },
                "name" => unsafe {
                    addr_of_mut!((*alert_ptr).name).write(
                        as_variant!(&data.columnvalues[pos], Thing::String)
                            .expect("name is not a String")
                            .to_owned(),
                    );
                    matched += 1;
                },
                "table" => unsafe {
                    addr_of_mut!((*alert_ptr).table).write(
                        as_variant!(&data.columnvalues[pos], Thing::String)
                            .expect("table is not a String")
                            .to_owned(),
                    );
                    matched += 1;
                },
                "lookup" => unsafe {
                    addr_of_mut!((*alert_ptr).lookup).write(
                        as_variant!(&data.columnvalues[pos], Thing::String)
                            .expect("lookup is not a String")
                            .to_owned(),
                    );
                    matched += 1;
                },
                "timing" => unsafe {
                    addr_of_mut!((*alert_ptr).timing).write(
                        *as_variant!(&data.columnvalues[pos], Thing::Number)
                            .expect("timing is not an i32"),
                    );
                    matched += 1;
                },
                "warn" => unsafe {
                    addr_of_mut!((*alert_ptr).warn).write(
                        as_variant!(&data.columnvalues[pos], Thing::String)
                            .expect("warn is not a String")
                            .to_owned(),
                    );
                    matched += 1;
                },
                "crit" => unsafe {
                    addr_of_mut!((*alert_ptr).crit).write(
                        as_variant!(&data.columnvalues[pos], Thing::String)
                            .expect("crit is not a String")
                            .to_owned(),
                    );
                    matched += 1;
                },
                "info" => unsafe {
                    addr_of_mut!((*alert_ptr).info).write(
                        as_variant!(&data.columnvalues[pos], Thing::OptionString)
                            .expect("info is not a Option<String>")
                            .to_owned(),
                    );
                    matched += 1;
                },
                "host_uuid" => unsafe {
                    addr_of_mut!((*alert_ptr).host_uuid).write(
                        as_variant!(&data.columnvalues[pos], Thing::String)
                            .expect("host_uuid is not a String")
                            .to_owned(),
                    );
                    matched += 1;
                },
                "where_clause" => unsafe {
                    addr_of_mut!((*alert_ptr).where_clause).write(
                        as_variant!(&data.columnvalues[pos], Thing::OptionString)
                            .expect("where_clause is not an Option<String>")
                            .to_owned(),
                    );
                    matched += 1;
                },
                // In case we don't have a known field
                _ => {
                    error!(
                        "Unknown field {} present with value {:?}",
                        val, &data.columnvalues[pos]
                    )
                }
            }
        }
        // Sanitizer to assure we got all our fields
        if matched != 10 {
            error!("Not all fields were found. Count : {}", matched);
            return Err(Error::new(ErrorKind::Other, "Not all fields were found."));
        }
        // Assume init is safe as we made sure it was correctly initialized for all fields
        Ok(unsafe { alert.assume_init() })
    }
}
