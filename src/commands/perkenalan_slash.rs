use serenity::{
    builder::{
        CreateApplicationCommand
    },
    model::{
        prelude::{
            command::{
                CommandOptionType
            },
            interaction::{
                application_command::{
                    CommandDataOption,
                    //CommandDataOptionValue
                }
            }
        }
    }
};

pub fn register(
    command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("perkenalan")
        .description("Perkenalkan diri kamu!")
        .create_option(|option| {
            option
                .name("nama")
                .description("Masukkan nama kamu!")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("kelas")
                .description("Kelas berapa kamu?")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("angkatan")
                .description("Angkatan tahun berapa kamu?")
                .kind(CommandOptionType::String)
                .required(true)
        })
}

pub fn run(options: &[CommandDataOption]) -> String {
    let _option = options
        .get(0)
        .expect("Expected string option")
        .resolved
        .as_ref()
        .expect("Expected string object");
    
        todo!()
}