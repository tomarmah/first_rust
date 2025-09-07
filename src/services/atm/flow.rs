use super::structs::{Command, CommandOption};
/**
 * This module handles the flow of the ATM service.
 * It allows:
 *
 * 1. Bank owner to:
 *  - create a bank
 *  - add/remove CFO
 *  - see bank balance
 *  - see total bank deposits
 *  - see total bank withdrawals
 *
 * 2. Banks CFO to:
 *   - Setup ATMs
 *   - See balances of all ATMs
 *   - See balance of a particular ATM
 *   - See notifications of low balance ATMs
 *   - Add/Remove Money loader
 *   - Send ATM load requests to money loaders
 *
 * 3. Money Loaders to:
 *   - See all load requests
 *   - Accept/Reject load requests
 *   - Load money to ATMs
 *   - Request money to load ATM
 *   - See their balance
 *
 * 4. Customers to:
 *   - Withdraw money from ATM
 *   - Deposit money into account
 *   - See their account balance
 *   - See their transaction history
 *   - Transfer money to another account
 *   - Change their PIN
 */
use std::io;

/**
 * A command consists of a label, description and options.
 * The options of a command are the various kinds of actions it can perform.
 * For example, the "role" command can have options like "switch", "list", "select"
 *  - switch: switch to a different role
 *  - list: list all available roles
 *  - select: select a role to perform actions
 *
 * -- bank commands -- {create, delete, add/remove cfo, view bank balance, view total deposits, view total withdrawals}
 *  -- to perform these commands user must be a bank owner
 * -- atm commands -- {setup atm, view all atm balances, view atm balance, view low balance notifications, add/remove money loader, send load request}
 *  -- to perform these commands user must be a cfo
 * -- load commands -- {view load requests, accept/reject load request, load money to atm, request money to load atm, view loader balance}
 *  -- to perform these commands user must be a money loader
 * -- customer commands -- {withdraw money, deposit money, view account balance, view transaction history, transfer money, change pin}
 *  -- to perform these commands user must be a customer
 * -- exit command -- {exit}
 */

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
    ]

    // commands;
}

pub fn start() {
    let commands = load_commands();
    println!("Welcome to the ATM service! To see all commands type 'help or h'");
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
            "exit" => {
                println!("Exiting ATM service. Goodbye!\n");
                break;
            }
            _ => {
                println!("Invalid command. Type 'help' or 'h' to see all commands.\n");
            }
        }

        // break;
    }
}
