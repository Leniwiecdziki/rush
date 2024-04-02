#![allow(dead_code)]

use std::collections::HashMap;
use std::env;

#[derive(Debug)]
 pub struct CommandStatus {
    pub code: Option<i32>,
    pub success: bool,
    pub signal: Option<i32>,
    pub core_dumped: bool 
}

pub const SPLIT_COMMANDS:[&str;4] = ["then", "next", "end", "else"];
pub const NESTABLE_OPERATORS:[&str;1] = ["test"];
pub const CMP_OPERATORS:[&str;2] = ["test", "else"];
pub const END_LOGIC:[&str;2] = ["end", "else"];

// These functions will be used to report success or failure when built-in or super commands are running
// This is usefull because typically we don't want the shell to abnormally quit when syntax of "test" statement is incorrect
// Instead, we just want to say "Hey! There is a bug!"
// BUT when rush would work as a subshell just to execute a script, we won't even need it anymore
pub fn report_success(index:usize, returns:&mut HashMap<usize, CommandStatus>) {
    let command_status = CommandStatus {code: Some(0),success: true,signal: None,core_dumped: false};
    returns.insert(index, command_status);
}
pub fn report_failure(index:usize, returns:&mut HashMap<usize, CommandStatus>) {
    let command_status = CommandStatus {code: Some(1),success: false,signal: None,core_dumped: false};
    returns.insert(index, command_status);
}


pub fn split_commands(mut words:Vec<String>, spliting_keywords:Vec<&str>) -> Vec<Vec<String>> {
    // This list contains all commands passed by the user 
    let mut commands: Vec<Vec<String>> = Vec::new();
    /*
    This will be used to separate SUPER COMMANDS from anything else
    Expected output: ('af' 'file'), ('then'), ('ad' 'dir')
    */ 

    // First of all, when there's a word that starts with "$"
    // replace it with variable contents
    let mut i = 0;
    // todo - add some comments here
    while i < words.len() {
        let w = words[i].clone();
        if let Some(w) = w.strip_prefix('$') {
            let variable_contents=env::var_os(w).unwrap_or_default();

            words.remove(i);
            words.insert(i, variable_contents.into_string().unwrap_or_default());
            i += 1;
        }
    };
        
    let mut command = Vec::new();
    let mut index = 0;
    while index < words.len() {
        // If built-in keyword appears
        if spliting_keywords.contains(&words[index].as_str()) {
            // Separate keyword from PREVIOUSLY collected words
            // Expected output: ('af' 'file'), ('then' 'ad' 'dir')
            let (before_keyword, right) = words.split_at(index);
            // Convert everything to a vector
            let (before_keyword, right) = (before_keyword.to_vec(), right.to_vec());

            // Separate keyword from NEXT words, that are not collected yet
            // Expected output: ('af' 'file'), ('then'), ('ad' 'dir')
            let (keyword, after_keyword) = {
                let (keyword, after_keyword) = right.split_at(1);
                (keyword.to_vec(), after_keyword.to_vec())
            };

            // Send previous words to "commands"
            // Example: ('af' 'file')
            if !before_keyword.is_empty() {
                // Do not append anything if there is nothing before keyword!
                commands.push(before_keyword.to_vec());
            }
            // Send keyword to "commands" exclusively
            // Example: ('then')
            commands.push(keyword.to_vec());
            // We no longer need to deal with ('af' 'file') and ('then') so remove them from words
            words = after_keyword.to_vec();
            // Start over with new words
            // Example: ('ad' 'dir')
            index = 0;
        }
        // If there is not built-in command 
        else {
            command.push(words[index].clone());
            index += 1;
            if index == words.len() {
                commands.push(words.clone());
            };
        };
    };
    commands
}