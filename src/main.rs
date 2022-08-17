use clap::{Parser, Subcommand};

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

fn main() {
    let args = Args::parse();

    println!("START debug info\n{:#?}\nEND debug info\n", args);

    match args.action {
        Action::User { action } => match action {
            UserAction::Get => todo!(),
            UserAction::Post { name, password } => todo!(),
        },
        Action::Problem { action } => match action {
            ProblemAction::Instructions => {
                println!("Clone the repository at https://github.com/PixelSam123/codeprobs using your favorite Git client.");
            },
        },
        Action::Answer { action } => match action {
            AnswerAction::Get => todo!(),
            AnswerAction::Post { filename } => todo!(),
        },
    }
}
