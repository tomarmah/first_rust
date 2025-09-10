use super::structs::{Command, CommandOption};
use std::io;

fn load_commands() -> Vec<Command> {
    vec![
        Command::new(
            "role",
            "This command handles role management",
            vec![
                CommandOption::new("list", "see all available command options"),
                CommandOption::new("all", "List all available roles"),
                CommandOption::new("select", "Select a role to perform actions"),
                CommandOption::new("switch", "Switch to a different role"),
            ],
            vec!["all"],
        ),
        Command::new(
            "bank",
            "Manage a bank (Bank Owner only)",
            vec![
                CommandOption::new("list", "see all available options"),
                CommandOption::new("create", "Create a new bank (Bank Owner only)"),
                CommandOption::new("delete", "Delete an existing bank (Bank Owner only)"),
                CommandOption::new("add_cfo", "Add a CFO to the bank (Bank Owner only)"),
                CommandOption::new("remove_cfo", "Remove a CFO from the bank (Bank Owner only)"),
                CommandOption::new("balance", "View the bank's balance (Bank Owner only)"),
                CommandOption::new(
                    "deposits",
                    "View total deposits in the bank (Bank Owner only)",
                ),
                CommandOption::new(
                    "withdrawals",
                    "View total withdrawals from the bank (Bank Owner only)",
                ),
            ],
            vec!["bank_owner"],
        ),
        Command::new(
            "atm",
            "Manage ATMs (CFO only)",
            vec![
                CommandOption::new("list", "see all available options"),
                CommandOption::new("setup", "Setup a new ATM (CFO only)"),
                CommandOption::new("all_balances", "View balances of all ATMs (CFO only)"),
                CommandOption::new("balance", "View balance of a specific ATM (CFO only)"),
                CommandOption::new(
                    "low_balance_notifications",
                    "View low balance ATM notifications (CFO only)",
                ),
                CommandOption::new("add_money_loader", "Add a money loader (CFO only)"),
                CommandOption::new("remove_money_loader", "Remove a money loader (CFO only)"),
                CommandOption::new(
                    "send_load_request",
                    "Send an ATM load request to a money loader (CFO only)",
                ),
            ],
            vec!["cfo"],
        ),
        Command::new(
            "load",
            "Manage all loading activities (Money Loader only)",
            vec![
                CommandOption::new("list", "see all available options"),
                CommandOption::new(
                    "view_requests",
                    "View all load requests (Money Loader only)",
                ),
                CommandOption::new(
                    "accept_request",
                    "Accept a load request (Money Loader only)",
                ),
                CommandOption::new(
                    "reject_request",
                    "Reject a load request (Money Loader only)",
                ),
                CommandOption::new("load_money", "Load money to an ATM (Money Loader only)"),
                CommandOption::new(
                    "request_money",
                    "Request money to load an ATM (Money Loader only)",
                ),
                CommandOption::new("view_balance", "View your balance (Money Loader only)"),
            ],
            vec!["money_loader"],
        ),
        Command::new(
            "customer",
            "Perform banking acitvities (Customer only)",
            vec![
                CommandOption::new("list", "see all available options"),
                CommandOption::new("withdraw", "Withdraw money from an ATM (Customer only)"),
                CommandOption::new("deposit", "Deposit money into your account (Customer only)"),
                CommandOption::new("balance", "View your account balance (Customer only)"),
                CommandOption::new(
                    "transactions",
                    "View your transaction history (Customer only)",
                ),
                CommandOption::new(
                    "transfer",
                    "Transfer money to another account (Customer only)",
                ),
                CommandOption::new("change_pin", "Change your PIN (Customer only)"),
            ],
            vec!["customer"],
        ),
        Command::new("exit", "Exit the ATM service", vec![], vec!["all"]),
        // Command::new("back", "Go back to previous menu", vec![], vec!["all"]),
        // Command::new("main", "Go back to main menu", vec![], vec!["all"]),
    ]

    // commands;
}

fn load_roles() -> Vec<&'static str> {
    vec!["bank_owner", "cfo", "money_loader", "customer"]
}

fn load_general_command_options() -> Vec<CommandOption> {
    vec![
        CommandOption::new("help", "Display help information"),
        CommandOption::new("exit", "Exit the ATM service"),
        CommandOption::new("back", "Go back to Previous menu"),
        CommandOption::new("main", "Go back to Main menu"),
    ]
}

fn find_command<'a>(commands: &'a [Command], label: &str) -> Result<&'a Command, String> {
    commands
        .iter()
        .find(|&cmd| cmd.label().to_lowercase() == label.to_lowercase())
        .ok_or_else(|| format!("Command '{}' not found", label))
}

fn find_command_opt<'a>(command: &'a Command, label: &str) -> Result<&'a CommandOption, String> {
    command
        .options()
        .iter()
        .find(|&opt| opt.label().to_lowercase() == label.to_lowercase())
        .ok_or_else(|| format!("Command option '{}' not found", label))
}

fn print_general_options(gen_options: &[CommandOption]) {
    for option in gen_options {
        println!("{} - {}", option.label(), option.description());
    }
    println!();
}

pub fn start() {
    let commands = load_commands();
    let gen_command_opts = load_general_command_options();
    let roles = load_roles();
    println!("Welcome to the ATM service! To see all commands type 'help or h'");
    let mut current_command: &Command = &Command::new("", "", Vec::new(), Vec::new());
    let mut current_command_option: &CommandOption = &CommandOption::new("", "");
    // Default to first command
    loop {
        // Here we would handle user input and direct to the appropriate functionality
        // For now, we just break the loop to avoid an infinite loop in this example
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input.");

        match input.trim() {
            "help" | "h" => {
                println!("\nAvailable Commands:\n");
                for command in &commands {
                    println!("{} - {}", command.label(), command.description());
                }
                println!("\n");
            }
            "role" | "bank" | "atm" | "load" | "customer" => {
                println!("Available options for this command are:\n");
                match find_command(&commands, input.trim()) {
                    Ok(cmd) => {
                        current_command = cmd;
                        for option in cmd.options() {
                            println!("{} - {}", option.label(), option.description());
                        }
                        println!();
                        print_general_options(&gen_command_opts);
                    }
                    Err(err) => {
                        println!("{}", err);
                    }
                }
            }
            "list" => {
                for option in current_command.options() {
                    println!("{} - {}", option.label(), option.description());
                }
                println!();
                print_general_options(&gen_command_opts);
            }
            "all" => {
                println!("Available roles are:\n");
                for role in &roles {
                    println!("- {}", role);
                }
                println!();
                print_general_options(&gen_command_opts);
            }
            "switch" => match current_command.label().to_lowercase().as_str() {
                "role" => {
                    match find_command_opt(&current_command, input.trim()) {
                        Ok(cmd) => {
                            current_command_option = cmd;
                        }
                        Err(err) => {
                            println!("{}", err);
                        }
                    }
                    println!("Please select a role to switch to:");
                    for role in &roles {
                        println!("- {}", role);
                    }
                }
                "" | _ => println!("No command selected. Please select a command first.\n"),
            },
            "bank_owner" | "cfo" | "money_loader" | "customer" => {
                //Add checks to prevent invalid role switches
                println!(
                    "Switched to role: {}\n\nPlease select an option:\n",
                    input.trim()
                );
                let command_label = match input.trim() {
                    "bank_owner" => "bank",
                    "cfo" => "atm",
                    "money_loader" => "load",
                    "customer" => "customer",
                    _ => "",
                };
                match find_command(&commands, command_label) {
                    Ok(cmd) => {
                        for option in cmd.options() {
                            println!("{} - {}", option.label(), option.description());
                        }
                        println!();
                        print_general_options(&gen_command_opts);
                    }
                    Err(err) => {
                        println!("{}", err);
                    }
                }
                // print_command_options(input.trim(), &commands, &gen_command_opts);
                // print_general_options(&gen_command_opts);
            }
            "exit" => {
                println!("Exiting ATM service. Goodbye!\n");
                break;
            }
            _ => {
                println!("Invalid command. Type 'help' or 'h' to see all commands.\n");
            }
        }
    }
}
