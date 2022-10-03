use chrono::prelude::*;
use chrono::{Date, Datelike, Utc};
use num_traits::FromPrimitive;
use std::cmp::Reverse;
use std::collections::HashMap;

use crate::config::Config;
use crate::consts::{MIN_AMOUNT_EARTHQUAKE, MIN_MAGNITUDE_EARTHQUAKE};
use crate::models::Earthquake;
use crate::schemas::input::RawEarthquake;

pub struct Processor {
    config: Config,
}

impl Processor {
    pub fn new(config: Config) -> Self {
        Processor { config }
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let response = self.get_raw_response()?;
        let earthquakes = self.get_earthquakes(response.json::<Vec<RawEarthquake>>()?)?;

        let mut earthquakes = earthquakes
            .into_iter()
            .filter(|earthquake| earthquake.magnitude > MIN_MAGNITUDE_EARTHQUAKE)
            .collect::<Vec<Earthquake>>();

        earthquakes.sort_by_key(|earthquake| earthquake.datetimeutc);

        let grouped_by_birthday = self.group_earthquakes_by_birthday(&earthquakes);

        self.show_info(&earthquakes, &grouped_by_birthday);

        Ok(())
    }

    fn get_raw_response(&self) -> Result<reqwest::blocking::Response, Box<dyn std::error::Error>> {
        let response = reqwest::blocking::get(&self.config.url_data)?;

        Ok(response)
    }

    fn get_earthquakes(
        &self,
        raw_data: Vec<RawEarthquake>,
    ) -> Result<Vec<Earthquake>, Box<dyn std::error::Error>> {
        let earthquakes: Vec<Earthquake> = raw_data.into_iter().map(Earthquake::from).collect();

        Ok(earthquakes)
    }

    fn group_earthquakes_by_birthday(
        &self,
        earthquakes: &Vec<Earthquake>,
    ) -> HashMap<String, Vec<Earthquake>> {
        let mut grouped = HashMap::<String, Vec<Earthquake>>::new();
        let mut date: String;
        let mut current_date: Date<Utc>;

        for earthquake in earthquakes {
            current_date = earthquake.datetimeutc.date();
            date = format!(
                "{day}-{month:#?}",
                day = current_date.day(),
                month = Month::from_u32(current_date.month()).unwrap()
            );
            grouped
                .entry(date)
                .or_insert(vec![])
                .push(earthquake.clone());
        }

        grouped
    }

    fn show_info(&self, earthquakes: &Vec<Earthquake>, grouped: &HashMap<String, Vec<Earthquake>>) {
        let amount_earthquakes = earthquakes.len();

        let (start_date, end_date) = (
            earthquakes.first().unwrap().datetimeutc,
            earthquakes.last().unwrap().datetimeutc,
        );

        println!(
            "There were {amount} greater or equal than > 5.0 (magnitude) from {start_date} to {end_date} in Mexico",
            amount = amount_earthquakes,
            start_date = start_date,
            end_date = end_date
        );

        let amount_birthday = grouped.len();
        println!(
            "{amount_birthday} were at the same date",
            amount_birthday = amount_birthday
        );

        self.print_more_frequent_ones(grouped);
    }

    fn print_more_frequent_ones(&self, grouped: &HashMap<String, Vec<Earthquake>>) {
        let mut more_frequent_ones: Vec<(usize, String)> = grouped
            .iter()
            .filter(|(_, v)| v.len() >= MIN_AMOUNT_EARTHQUAKE)
            .map(|(date, v)| (v.len(), format!("{date}: {group_len}", group_len = v.len())))
            .collect::<Vec<(usize, String)>>();

        more_frequent_ones.sort_by_key(|data| Reverse(data.0));
        let more_frequent_ones: Vec<String> = more_frequent_ones
            .iter()
            .map(|(_, message)| message.to_string())
            .collect();

        for frequent in more_frequent_ones {
            println!("{frequent}");
        }
    }
}
