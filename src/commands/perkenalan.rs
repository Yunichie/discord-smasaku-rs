use crate::capitalizer::capitalize;

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
                    Interaction,
                    InteractionResponseType
                },
                component::{
                    InputTextStyle
                }
            },
        },
        Timestamp
    },
    prelude::{
        *
    }
};
use regex::Regex;
use std::time::Duration;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("perkenalan")
        .description("[Sedang dalam pengerjaan; gunakan perkenalan-slash]")
}

pub async fn run(interaction: Interaction, ctx: &Context) {
    let modal = interaction.application_command().unwrap()
    .create_interaction_response(&ctx.http, |resp| {
        resp.kind(InteractionResponseType::Modal)
            .interaction_response_data(|response| {
                response.custom_id("perkenalan");
                response.title("Perkenalan");
                response.components(|act_row| {
                    act_row.create_action_row(|modal| {
                        modal.create_input_text(|input| {
                            input.custom_id("perkenalan_nama");
                            input.style(InputTextStyle::Short);
                            input.label("Nama");
                            input.placeholder("Masukkan nama kamu!");
                            input.required(true)
                        })
                    });
                    act_row.create_action_row(|modal| {
                        modal.create_input_text(|input| {
                            input.custom_id("perkenalan_kelas");
                            input.style(InputTextStyle::Short);
                            input.label("Kelas");
                            input.placeholder("Masukkan kelas kamu!");
                            input.required(true)
                        })
                    });
                    act_row.create_action_row(|modal| {
                        modal.create_input_text(|input| {
                            input.custom_id("perkenalan_angkatan");
                            input.style(InputTextStyle::Short);
                            input.label("Angkatan");
                            input.placeholder("Angkatan tahun berapa kamu?");
                            input.required(true)
                        })
                    });
                    act_row.create_action_row(|modal| {
                        modal.create_input_text(|input| {
                            input.custom_id("perkenalan_medsos");
                            input.style(InputTextStyle::Short);
                            input.label("Media Sosial");
                            input.placeholder("Media sosial kamu! (opsional)");
                            input.required(false)
                        })
                    })
                })
            })  
    }).await;

    // TODO: Buat collector untuk mendapatkan user modal input
    // if let Some(value) = &interaction
    //                      .modal_submit().unwrap()
    //                      .message.unwrap()
    //                      .author.await_modal_interaction(ctx).timeout(Duration::from_secs) ...?

    println!("{:?}", modal);

    if let Err(why) = modal {
        println!("Error sending message: {:?}", why);
    }
}