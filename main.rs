mod validate;
mod summarize;
mod merge;

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(name = "MedIntegrator")]
#[command(about = "A CLI tool for health data integration and analysis", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Load a CSV or JSON file
    LoadFile {
        #[arg(short, long)]
        path: String,
    },

    /// Validate health data file
    Validate {
        #[arg(short, long)]
        path: String,
    },

    /// Summarize health data
    Summarize {
        #[arg(short, long)]
        path: String,
    },

    /// Merge multiple data files
    MergeFiles {},

    /// Predict risk (AI placeholder)
    PredictRisk {},
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PatientRecord {
    pub patient_id: u32,
    pub date: String,
    pub heart_rate: u32,
    pub bp_systolic: u32,
    pub bp_diastolic: u32,
    pub temperature: f32,
    pub blood_sugar: f32,
    pub steps: u32,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::LoadFile { path } => {
            let file = std::fs::File::open(path).expect("Failed to open file");
            let mut rdr = csv::Reader::from_reader(file);

            for result in rdr.deserialize() {
                let record: PatientRecord = result.expect("Failed to parse record");
                println!("{:?}", record);
            }
        }

        Commands::Validate { path } => {
            println!("Validating data...");
            validate::run_validation(path);
        }

        Commands::Summarize { path } => {
            println!("Summarizing data...");
            summarize::summarize_data(path);
        }

        Commands::MergeFiles {} => {
            println!("Merging mock_data/patients_sample.csv with a second file...");
            merge::merge_files(
                vec![
                    "mock_data/patients_sample.csv",
                    "mock_data/another_sample.csv"
                ],
                "mock_data/merged_output.csv",
            );
        }

        Commands::PredictRisk {} => {
            println!("Predicting risk (placeholder)...");
        }
    }
}
