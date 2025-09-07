//Use Getters and Setters

pub struct Command {
    label: &'static str,
    description: &'static str,
    options: Vec<CommandOption>,
    permissions: Vec<&'static str>, // e.g., ["bank_owner", "cfo", "money_loader", "customer"]
}

impl Command {
    pub fn new(
        label: &'static str,
        description: &'static str,
        options: Vec<CommandOption>,
        permissions: Vec<&'static str>,
    ) -> Self {
        Command {
            label,
            description,
            options,
            permissions,
        }
    }

    //Getter
    pub fn label(&self) -> &str {
        self.label
    }
    pub fn description(&self) -> &str {
        self.description
    }
    pub fn options(&self) -> &Vec<CommandOption> {
        &self.options
    }
    pub fn permissions(&self) -> &Vec<&str> {
        &self.permissions
    }

    //Setter
    pub fn set_label(&mut self, label: &'static str) {
        self.label = label;
    }
    pub fn set_description(&mut self, description: &'static str) {
        self.description = description;
    }
    pub fn set_options(&mut self, options: Vec<CommandOption>) {
        self.options = options;
    }

    //push new option
    pub fn add_option(&mut self, option: CommandOption) {
        self.options.push(option);
    }

    pub fn set_permissions(&mut self, permissions: Vec<&'static str>) {
        self.permissions = permissions;
    }

    //push new permission
    pub fn add_permission(&mut self, permission: &'static str) {
        self.permissions.push(permission);
    }
}

pub struct CommandOption {
    pub label: &'static str,
    pub description: &'static str,
}

impl CommandOption {
    pub fn new(label: &'static str, description: &'static str) -> Self {
        CommandOption { label, description }
    }
}
