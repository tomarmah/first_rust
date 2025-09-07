//Use Getters and Setters

pub struct Command {
    pub label: &'static str,
    pub description: &'static str,
    pub options: Vec<CommandOption>,
    pub permissions: Vec<&'static str>, // e.g., ["bank_owner", "cfo", "money_loader", "customer"]
}

pub struct CommandOption {
    pub label: &'static str,
    pub description: &'static str,
}
