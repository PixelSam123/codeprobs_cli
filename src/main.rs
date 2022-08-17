use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;
use comfy_table::{presets, Table};
use serde::Deserialize;

/// Companion app for coding problems at PixelSam123/codeprobs
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug)]
enum Action {
    User {
        #[clap(subcommand)]
        action: UserAction,
    },
    Problem {
        #[clap(subcommand)]
        action: ProblemAction,
    },
    Answer {
        #[clap(subcommand)]
        action: AnswerAction,
    },
}

/// Fetch or post (sign up) users to the server
#[derive(Subcommand, Debug)]
enum UserAction {
    /// Fetch all users in a leaderboard format
    Get,
    /// Sign up a user
    Post { name: String, password: String },
}

/// Instructions for obtaining the coding problems
#[derive(Subcommand, Debug)]
enum ProblemAction {
    /// Print instructions for obtaining the coding problems
    Instructions,
}

/// Fetch or post answers to the server
#[derive(Subcommand, Debug)]
enum AnswerAction {
    /// Get answers for the problem in the current directory
    Get,
    /// Post answer for the problem in the current directory
    Post {
        /// Point to the file that contains the answer
        filename: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    // Make this configurable later
    let server_url = "http://localhost:5200/api/v1/";

    let args = Args::parse();

    println!("START debug info\n{:#?}\nEND debug info\n", args);

    match args.action {
        Action::User { action } => match action {
            UserAction::Get => {
                let response = reqwest::get(format!("{}user", server_url))
                    .await?
                    .json::<Vec<User>>()
                    .await?;

                let mut table = Table::new();
                table.set_header(vec!["Username", "Points"]);

                for user in response {
                    table
                        .load_preset(presets::UTF8_BORDERS_ONLY)
                        .add_row(vec![user.username.to_string(), user.points.to_string()]);
                }

                println!("{}", table);
            }
            UserAction::Post { name, password } => todo!(),
        },
        Action::Problem { action } => match action {
            ProblemAction::Instructions => {
                println!("Clone the repository at https://github.com/PixelSam123/codeprobs using your favorite Git client.");
            }
        },
        Action::Answer { action } => match action {
            AnswerAction::Get => todo!(),
            AnswerAction::Post { filename } => todo!(),
        },
    };

    Ok(())
}

#[derive(Deserialize, Debug)]
struct User {
    username: String,
    points: i32,
}
