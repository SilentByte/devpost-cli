/*
 * DEVPOST CLI IDEA
 * Copyright (c) 2023 SilentByte <https://silentbyte.com/>
 */

use std::time::Duration;

use clap::{
    ArgMatches,
    Command,
};
use colored::Colorize;

use crate::cli;

pub fn command() -> Command {
    Command::new("list").about("Lists all your projects")
}

pub fn handle(_matches: &ArgMatches) -> anyhow::Result<()> {
    let projects = vec![
        ("Pothole AI", "What if we used Edge Analytics on a dashcam to map out where potholes were, so that local governments and communities knew where to focus their efforts?", "Facebook Hackathon: AI"),
        ("Traffic Cop", "WiFi Beacon & Client sniffing pipeline for analysing the movements of Human Traffickers.", "Serverless Apps for Social Good"),
        ("HealthCam", "Camera-enabled IoT device powered by AWS SageMaker to detect if people are wearing masks when accessing restricted areas in hospitals in an effort to flatten the curve of COVID-19.", "AWS Marketplace Developer Challenge: ML Powered Solutions"),
        ("Lyngua","A novel, AI-driven language learning experience powered by Azure Cognitive Models.", "Azure AI Hackathon"),
        ("Robin Accountant","A new AI experiment assisting you with budgeting and record-keeping for your personal or business needs.", "Facebook Hackathon: AI"),
        ("Art愛 / ArtAI","ArtAI is an interactive art installation that collects people's ideas in real-time from social media and uses deep learning and AI art generation to curate these ideas into a dynamic display.", "SAAI Factory - Hackathon on Art and AI"),
        ("CySec Intern","This is a casual game where you explore the world as a newly hired intern and learn more about cybersecurity concepts on your journey.", "Secureworks Cybersecurity Literacy Challenge"),
        ("Eye5G","An experimental aid for people with visual impairments featuring low-latency, real-time object detection using 5G Edge Computing.", "5G Edge Computing Challenge with AWS and Verizon"),
        ("Aviator 5G","Aviator 5G is a real-time mission control software package for unmanned air, ground, and water-based vehicles powered by AWS Wavelength & 5G Edge Computing.", "5G Edge Computing Challenge with AWS"),
        ("DisaVu","DisaVu is a disaster response solution that helps direct relief resources to where they are needed most using a sophisticated machine learning approach.", "AWS Disaster Response Hackathon"),
        ("Savee","Savee is the new hub for shopping, selling, payments, and money transfers. Conveniently packaged in one Super App and powered by Rapyd.", "Build the Best Super App"),
    ];

    cli::progress_bar(Duration::from_secs(1));

    println!("Here's a list of all your Devpost projects!");
    println!();

    for (i, p) in projects.iter().enumerate() {
        let index = format!("{})", i + 1).bold();
        println!(
            "{} {} | {}\n{}\n",
            index,
            p.0.bright_red().bold(),
            p.2.bold(),
            p.1.white()
        )
    }

    Ok(())
}
