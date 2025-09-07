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
        Command {
            label: "role",
            description: "This commnand handles role management",
            options: vec![
                CommandOption {
                    label: "list",
                    description: "see all available command options",
                },
                CommandOption {
                    label: "all",
                    description: "List all available roles",
                },
                CommandOption {
                    label: "select",
                    description: "Select a role to perform actions",
                },
                CommandOption {
                    label: "switch",
                    description: "Switch to a different role",
                },
            ],
            permissions: vec!["all"],
        },
        Command {
            label: "bank",
            description: "Manage a bank (Bank Owner only)",
            permissions: vec!["bank_owner"],
            options: vec![
                CommandOption {
                    label: "list",
                    description: "see all available options",
                },
                CommandOption {
                    label: "create",
                    description: "Create a new bank (Bank Owner only)",
                },
                CommandOption {
                    label: "delete",
                    description: "Delete an existing bank (Bank Owner only)",
                },
                CommandOption {
                    label: "add_cfo",
                    description: "Add a CFO to the bank (Bank Owner only)",
                },
                CommandOption {
                    label: "remove_cfo",
                    description: "Remove a CFO from the bank (Bank Owner only)",
                },
                CommandOption {
                    label: "balance",
                    description: "View the bank's balance (Bank Owner only)",
                },
                CommandOption {
                    label: "deposits",
                    description: "View total deposits in the bank (Bank Owner only)",
                },
                CommandOption {
                    label: "withdrawals",
                    description: "View total withdrawals from the bank (Bank Owner only)",
                },
            ],
        },
        Command {
            label: "atm",
            description: "Manage ATMs (CFO only)",
            permissions: vec!["cfo"],
            options: vec![
                CommandOption {
                    label: "list",
                    description: "see all available options",
                },
                CommandOption {
                    label: "setup",
                    description: "Setup a new ATM (CFO only)",
                },
                CommandOption {
                    label: "all_balances",
                    description: "View balances of all ATMs (CFO only)",
                },
                CommandOption {
                    label: "balance",
                    description: "View balance of a specific ATM (CFO only)",
                },
                CommandOption {
                    label: "low_balance_notifications",
                    description: "View low balance ATM notifications (CFO only)",
                },
                CommandOption {
                    label: "add_money_loader",
                    description: "Add a money loader (CFO only)",
                },
                CommandOption {
                    label: "remove_money_loader",
                    description: "Remove a money loader (CFO only)",
                },
                CommandOption {
                    label: "send_load_request",
                    description: "Send an ATM load request to a money loader (CFO only)",
                },
            ],
        },
        Command {
            label: "load",
            description: "Manage all loading activities (Money Loader only)",
            permissions: vec!["money_loader"],
            options: vec![
                CommandOption {
                    label: "list",
                    description: "see all available options",
                },
                CommandOption {
                    label: "view_requests",
                    description: "View all load requests (Money Loader only)",
                },
                CommandOption {
                    label: "accept_request",
                    description: "Accept a load request (Money Loader only)",
                },
                CommandOption {
                    label: "reject_request",
                    description: "Reject a load request (Money Loader only)",
                },
                CommandOption {
                    label: "load_money",
                    description: "Load money to an ATM (Money Loader only)",
                },
                CommandOption {
                    label: "request_money",
                    description: "Request money to load an ATM (Money Loader only)",
                },
                CommandOption {
                    label: "view_balance",
                    description: "View your balance (Money Loader only)",
                },
            ],
        },
        Command {
            label: "customer",
            description: "Perform banking acitvities (Customer only)",
            permissions: vec!["customer"],
            options: vec![
                CommandOption {
                    label: "list",
                    description: "see all available options",
                },
                CommandOption {
                    label: "withdraw",
                    description: "Withdraw money from an ATM (Customer only)",
                },
                CommandOption {
                    label: "deposit",
                    description: "Deposit money into your account (Customer only)",
                },
                CommandOption {
                    label: "balance",
                    description: "View your account balance (Customer only)",
                },
                CommandOption {
                    label: "transactions",
                    description: "View your transaction history (Customer only)",
                },
                CommandOption {
                    label: "transfer",
                    description: "Transfer money to another account (Customer only)",
                },
                CommandOption {
                    label: "change_pin",
                    description: "Change your PIN (Customer only)",
                },
            ],
        },
        Command {
            label: "exit",
            description: "Exit the ATM service",
            options: vec![],
            permissions: vec!["all"],
        },
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
                    println!("{} - {}", command.label, command.description);
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
