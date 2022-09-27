use serenity::{
    builder::{
        CreateApplicationCommand
    },
    model::{
        prelude::{
            channel::{
                Message
            },
        },
        Timestamp
    },
    prelude::{
        *
    }
};

pub fn register(
    command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("bantuan")
        .description("Dapatkan bantuan!")
}