use serenity::{
    builder::{
        CreateApplicationCommand
    },
    model::{
        prelude::{
            command::{
                CommandOptionType
            },
            application::{
                interaction::{
                    application_command::{
                        ApplicationCommandInteraction
                    }
                }
            },
            channel::{
                Message
            },
        }
    },
    prelude::{
        *
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

pub fn run(options: &ApplicationCommandInteraction) -> String {
    let nama = options
    .data
    .options
    .iter()
    .find(|option| option.name == "nama")
    .cloned();

    let kelas = options
    .data
    .options
    .iter()
    .find(|option| option.name == "kelas")
    .cloned();

    let angkatan = options
    .data
    .options
    .iter()
    .find(|option| option.name == "angkatan")
    .cloned();

    format!("{}, {}, {}", nama.unwrap().value.unwrap().as_str().unwrap(), kelas.unwrap().value.unwrap().as_str().unwrap(), angkatan.unwrap().value.unwrap().as_str().unwrap())
}