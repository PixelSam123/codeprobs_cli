use clap::{Parser, Subcommand};
use color_eyre::eyre::{bail, Result};
use comfy_table::{presets, Table};
use reqwest::StatusCode;
use serde::Deserialize;
use std::{collections::HashMap, fs};

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
    Post {
        name: String,
        /// Please do NOT use a real password!
        password: String,
    },
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
        /// Name of the user this answer is posted on the behalf of
        username: String,
        /// Password of the user this answer is posted on the behalf of
        password: String,
    },
    /// Delete answer, based on its ID, for the problem in the current directory
    Delete {
        /// ID of the answer
        id: i32,
        /// Name of the user this answer is deleted on the behalf of
        username: String,
        /// Password of the user this answer is deleted on the behalf of
        password: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    // Make this configurable later
    let server_url = "http://localhost:5200/api/v1/";

    let args = Args::parse();

    exec_args_with_server_url(args, server_url).await
}

async fn exec_args_with_server_url(args: Args, server_url: &str) -> Result<()> {
    match args.action {
        Action::User { action } => match action {
            UserAction::Get => {
                let response = reqwest::get(format!("{}user", server_url))
                    .await?
                    .json::<Vec<User>>()
                    .await?;

                let mut table = Table::new();
                table.load_preset(presets::UTF8_BORDERS_ONLY).set_header(["Username", "Points"]);

                for user in response {
                    table.add_row([user.username.to_string(), user.points.to_string()]);
                }

                println!("{}", table);
            }
            UserAction::Post { name, password } => {
                let post_body = HashMap::from([("username", name), ("password", password)]);

                let client = reqwest::Client::new();
                let response = client
                    .post(format!("{}user", server_url))
                    .json(&post_body)
                    .send()
                    .await?;

                match response.status() {
                    StatusCode::CREATED => println!("User created successfully"),
                    StatusCode::OK => println!("User accepted but not created"),
                    _ => println!("User NOT created! Reason:\n{}", response.text().await?),
                };
            }
        },
        Action::Problem { action } => match action {
            ProblemAction::Instructions => {
                println!("Clone the repository at https://github.com/PixelSam123/codeprobs using your favorite Git client.");
                println!("Copy a folder of the problem you want to do to your desired location.");
                println!("Submission instructions are located in the README.md of a problem.");
            }
        },
        Action::Answer { action } => match action {
            AnswerAction::Get => match get_codeprob_info_id() {
                Ok(codeprob_info_id) => {
                    let response =
                        reqwest::get(format!("{}answer/{}", server_url, codeprob_info_id))
                            .await?
                            .json::<Vec<Answer>>()
                            .await?;

                    for answer in response {
                        println!(
                            "Answer ID: {}, Username: {}, Upvotes: {}, Downvotes: {}",
                            answer.id,
                            answer.user.username,
                            answer.upvote_count,
                            answer.downvote_count
                        );
                        println!("{}", answer.content);
                        println!("------");
                    }
                }
                Err(err) => bail!(err),
            },
            AnswerAction::Post {
                filename,
                username,
                password,
            } => match get_codeprob_info_id() {
                Ok(codeprob_info_id) => match fs::read_to_string(filename) {
                    Ok(file_string) => {
                        let post_body = HashMap::from([("language", "js".to_owned()), ("content", file_string)]);

                        let client = reqwest::Client::new();
                        let response = client
                            .post(format!("{}answer/{}", server_url, codeprob_info_id))
                            .basic_auth(username, Some(password))
                            .json(&post_body)
                            .send()
                            .await?;

                        match response.status() {
                            StatusCode::CREATED => println!("Answer created successfully"),
                            StatusCode::OK => println!("Answer accepted but not created"),
                            StatusCode::UNPROCESSABLE_ENTITY => {
                                let answer_error = response.json::<AnswerError>().await?;

                                println!("Answer NOT created! Reason: {}", answer_error.reason);
                                if let Some(stdout) = answer_error.stdout {
                                    println!("Stdout:\n{}", stdout);
                                };
                                if let Some(stderr) = answer_error.stderr {
                                    println!("Stderr:\n{}", stderr);
                                };
                            }
                            _ => println!("Answer NOT created! Reason:\n{}", response.text().await?),
                        }
                    }
                    Err(err) => bail!("Cannot read the specified file! Reason:\n{}", err),
                },
                Err(err) => bail!(err),
            },
            AnswerAction::Delete {
                id,
                username,
                password,
            } => {
                let client = reqwest::Client::new();
                let response = client
                    .delete(format!("{}answer/{}", server_url, id))
                    .basic_auth(username, Some(password))
                    .send()
                    .await?;

                match response.status() {
                    StatusCode::NO_CONTENT => println!("Answer deleted successfully"),
                    StatusCode::FORBIDDEN => println!("Permission for deleting this answer not met!"),
                    StatusCode::NOT_FOUND => println!("Answer with this ID not found!"),
                    StatusCode::UNAUTHORIZED => println!("Answer NOT deleted! Invalid credentials."),
                    _ => println!("Answer NOT deleted! Unrecognized reason."),
                };
            },
        },
    };

    Ok(())
}

fn get_codeprob_info_id() -> Result<i32, String> {
    match fs::read_to_string("./.codeprob_info.json") {
        Ok(file_string) => match serde_json::from_str::<CodeprobInfo>(&file_string) {
            Ok(codeprob_info) => Ok(codeprob_info.id),
            Err(err) => Err(format!("Invalid structure of the contents of .codeprob_info.json! Reason:\n{}", err)),
        }
        Err(err) => Err(
            format!("Cannot read .codeprob_info.json from this directory! Are you in the right folder? Reason:\n{}", err)
        ),
    }
}

/// User information from the server
#[derive(Deserialize, Debug)]
struct User {
    username: String,
    points: i32,
}

/// Answer information from the server
#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
struct Answer {
    id: i32,
    user: User,
    language: String,
    content: String,
    upvote_count: i32,
    downvote_count: i32,
}

/// Answer creation error information from the server
#[derive(Deserialize, Debug)]
struct AnswerError {
    reason: String,
    stdout: Option<String>,
    stderr: Option<String>,
}

/// Structure of file `.codeprob_info.json`
#[derive(Deserialize, Debug)]
struct CodeprobInfo {
    id: i32,
}
