use std::collections::HashMap;

pub mod cli;
pub mod request;

pub struct HelpText {
    pub tagline: &'static str, // used in <cmd usage>
    pub short_desc: &'static str, // used in DESCRIPTION
    pub synopsis: &'static str, // showcasing the cmd
    /*
     * pub usage:       Option<&'static str>, // overrides USAGE section
     * pub long_desc:   Option<&'static str>, // overrides DESCRIPTION section
     * pub options:     Option<&'static str>, // overrides OPTIONS section
     * pub arguments:   Option<&'static str>, // overrides ARGUMENTS section
     * pub subcommands: Option<&'static str> // overrides SUBCOMMANDS section
     * */
}

pub type RunFn = fn(&request::Request) -> Result<(), String>;

pub type ArgName = &'static str;
pub type OptName = &'static str;
pub type CommandName = &'static str;

pub trait Command {
    fn run(&self, &request::Request) -> Result<(), String>;
    fn get_def(&self) -> &CommandDefinition;
}

// For easily making a command
pub struct CommandDefinition {
    name: CommandName,
    options: HashMap<OptName, Opt>,
    arguments: Vec<Argument>,
    help_text: HelpText,
    subcommands: HashMap<CommandName, Box<Command>>,
}

impl CommandDefinition {
    // TODO: disallow an argument that isnt the last argument from being variadic
    pub fn new(name: CommandName,
               options: Vec<Opt>,
               arguments: Vec<Argument>,
               help_text: HelpText,
               subcommands: Vec<Box<Command>>)
               -> Self {
        CommandDefinition {
            name: name,
            options: options,
            arguments: arguments,
            help_text: help_text,
            subcommands: subcommands.into_iter().map(|cmd| (cmd.name(), cmd)).collect(),
        }
    }

    pub fn name(&self) -> CommandName {
        self.name
    }

    pub fn options(&self) -> Vec<(OptName, &Opt)> {
        let mut v = Vec::new();
        for opt in self.options.iter() {
            for &name in opt.names.iter() {
                v.push((name, opt));
            }
        }
        v
    }

    // TODO: should I remove this now?
    pub fn get_option<'a>(&'a self, name: &str) -> Option<&'a Opt> {
        for opt in self.options.iter() {
            for &opt_name in opt.names.iter() {
                if name == opt_name {
                    return Some(opt);
                }
            }
        }

        None
    }

    pub fn subcommand(&self, subcmd: &str) -> Option<&Command> {
        self.subcommands.get(subcmd)
    }

    pub fn arguments(&self) -> &[Argument] {
        &self.arguments[..]
    }

    pub fn run(&self, req: &request::Request) -> Result<(), String> {
        (self.run)(req)
    }
}

#[derive(Copy, Clone)]
pub enum OptType {
    Bool,
    String,
    Int,
}

// represents an option for a command
pub struct Opt {
    name: OptName, // canonical name for the option
    pub names: Vec<OptName>,
    pub opt_type: OptType,
    description: &'static str,
}

impl Opt {
    // The first name in the `names` vector is used as canonical name
    pub fn new_bool(names: Vec<OptName>, desc: &'static str) -> Self {
        Self::new(names, OptType::Bool, desc)
    }

    fn new(mut names: Vec<OptName>, opt_type: OptType, desc: &'static str) -> Self {
        let canonical = names[0];
        names.sort_by(|a, b| a.len().cmp(&b.len()));
        Opt {
            name: canonical,
            names: names,
            opt_type: opt_type,
            description: desc,
        }
    }

    pub fn name(&self) -> OptName {
        self.name
    }
}

#[derive(Copy, Clone)]
enum ArgumentType {
    String,
    File,
}

pub struct Argument {
    name: ArgName,
    ty: ArgumentType,
    required: bool,
    variadic: bool,
    description: &'static str,
}

impl Argument {
    pub fn new_file(name: ArgName,
                    required: bool,
                    variadic: bool,
                    desc: &'static str)
                    -> Self {
        Self::new(name, ArgumentType::File, required, variadic, desc)
    }

    pub fn new_string(name: ArgName,
                      required: bool,
                      variadic: bool,
                      desc: &'static str)
                      -> Self {
        Self::new(name, ArgumentType::String, required, variadic, desc)
    }

    fn new(name: ArgName,
           ty: ArgumentType,
           required: bool,
           variadic: bool,
           desc: &'static str)
           -> Self {
        Argument {
            name: name,
            ty: ty,
            required: required,
            variadic: variadic,
            description: desc,
        }
    }

    pub fn is_variadic(&self) -> bool {
        self.variadic
    }

    pub fn name(&self) -> ArgName {
        self.name
    }

    pub fn arg_type(&self) -> ArgumentType {
        self.ty
    }
}
