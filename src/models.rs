// Copyright (c) Anthony Wilcox and contributors. All rights reserved.
// Licensed under the GNU GPL v3 license. See LICENSE file in the project
// root for full license information.
use std::io::Write;
use std::fs::OpenOptions;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use chrono::prelude::*;

const AMY_EXT: &str = "amy";
const AMC_EXT: &str = "amc";
const AMR_EXT: &str = "amr";

#[derive(Serialize, Deserialize, Debug)]
pub struct YCH {
    pub id: String,
    /// Uses the local time
    pub date: DateTime<Local>,
    pub customer: String,
    pub reference: String,
    /// The YCH
    pub art: String,
    /// Slot the customer won in the auction
    pub slot: String,
    pub contact: String,
    pub price: String,
    /// Payment information (paypal, crypto, ect)
    pub payment: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Commission {
    pub id: String,
    /// Uses the local time
    pub date: DateTime<Local>,
    pub art: String,
    pub customer: String,
    pub contact: String,
    pub price: String,
    /// Payment information (paypal, crypto, ect)
    pub payment: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub id: String,
    /// Uses the local time
    pub date: DateTime<Local>,
    pub customer: String,
    pub contact: String,
    pub art: String,
    pub description: String,
}


impl YCH {
    pub fn write_ych(ych: YCH) -> Result<()> {
        let new_ych = YCH { id: ych.id, date: ych.date, customer: ych.customer,
            art: ych.art, slot: ych.slot, contact: ych.contact,
            payment: ych.payment, price: ych.price, reference: ych.reference
        };

        let json_string = serde_json::to_string_pretty(&new_ych)?;
        let order_string = new_ych.art.to_owned().to_lowercase();
        let slot_string = new_ych.slot.to_owned().to_lowercase();
        let client_string = new_ych.customer.to_owned().to_lowercase();
        let file_name = format!("{} - {} - {}.{}",
                                order_string, slot_string, client_string, AMY_EXT);
        let mut file = OpenOptions::new().write(true)
            .create_new(true)
            .open(file_name)
            .expect("Could not open file.");

        if let Err(err) = writeln!(file, "{}", format!("{}", json_string)) {
            eprintln!("Couldn't write to file. {}", err);
        }

        Ok(())
    }

    pub fn print_ych(ych: YCH) -> Result<()> {

        let new_ych = YCH { id: ych.id, date: ych.date, customer: ych.customer,
            art: ych.art, slot: ych.slot, contact: ych.contact,
            payment: ych.payment, price: ych.price, reference: ych.reference
        };

        let json_string = serde_json::to_string_pretty(&new_ych)?;

        println!("{}", json_string);

        Ok(())
    }
}

impl Commission {
    pub fn write_comm(comm: Commission) -> Result<()> {

        let new_comm = Commission {
            id: comm.id, date: comm.date, customer: comm.customer,
            contact: comm.contact, payment: comm.payment,
            price: comm.price, description: comm.description,
            art: comm.art,
        };

        let json_string = serde_json::to_string_pretty(&new_comm)?;
        let order_string = new_comm.art.to_string().to_lowercase();
        let client_string = new_comm.customer.to_string().to_lowercase();
        let file_name = format!("{} - {}.{}", order_string, client_string, AMC_EXT);
        let mut file = OpenOptions::new().write(true)
            .create_new(true)
            .open(file_name)
            .expect("Could not open file.");

        if let Err(err) = writeln!(file, "{}", format!("{}", json_string)) {
            eprintln!("Couldn't write to file. {}", err);
        }

        Ok(())
    }


    pub fn print_comm(comm: Commission) -> Result<()> {

        let new_comm = Commission {
            id: comm.id, date: comm.date, customer: comm.customer,
            contact: comm.contact, payment: comm.payment,
            price: comm.price, description: comm.description,
            art: comm.art,
        };

        let json_string = serde_json::to_string_pretty(&new_comm)?;

        println!("{}", json_string);

        Ok(())
    }
}

impl Request {
    pub fn write_req(req: Request) -> Result<()> {

        let new_req = Request {
            id: req.id, date: req.date, contact: req.contact,
            customer: req.customer, description: req.description,
            art: req.art
        };

        let json_string = serde_json::to_string_pretty(&new_req)?;
        let name_string = new_req.art.to_string().to_lowercase();
        let client_string = new_req.customer.to_string().to_lowercase();
        let file_name = format!("{} - {}.{}", name_string, client_string, AMR_EXT);
        let mut file = OpenOptions::new().write(true)
            .create_new(true)
            .open(file_name)
            .expect("Could not open file.");

        if let Err(err) = writeln!(file, "{}", format!("{}", json_string)) {
            eprintln!("Couldn't write to file. {}", err);
        }

        Ok(())
    }

    pub fn print_req(req: Request) -> Result<()> {

        let new_req = Request {
            id: req.id, date: req.date, contact: req.contact,
            customer: req.customer, description: req.description,
            art: req.art
        };

        let json_string = serde_json::to_string_pretty(&new_req)?;

        println!("{}", json_string);

        Ok(())
    }
}