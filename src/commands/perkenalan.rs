use crate::capitalizer::capitalize;

use serenity::{
    builder::{
        CreateApplicationCommand
    },
    collector::{
        modal_interaction_collector::{
            CollectModalInteraction
        }
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
                    InputTextStyle,
                    ActionRowComponent
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
    let modal = interaction.to_owned().application_command().unwrap()
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

    if let Err(why) = modal {
        println!("Error sending message: {:?}", why);
    }

    let response = CollectModalInteraction::new(&ctx.shard)
                    .author_id(interaction.application_command().unwrap().user.id)
                    .timeout(Duration::from_secs(3600))
                    .await
                    .unwrap();

    let nama = match response.data.components.get(0).unwrap().components.get(0).unwrap() {
        ActionRowComponent::InputText(inp) => inp,
        _ => return
    };

    let kelas = match response.data.components.get(1).unwrap().components.get(0).unwrap() {
        ActionRowComponent::InputText(inp) => inp,
        _ => return
    };

    let angkatan = match response.data.components.get(2).unwrap().components.get(0).unwrap() {
        ActionRowComponent::InputText(inp) => inp,
        _ => return
    };

    let medsos = match response.data.components.get(3).unwrap().components.get(0).unwrap() {
        ActionRowComponent::InputText(inp) => inp,
        _ => return
    };

    //println!("{} {} {} {}", nama.value, kelas.value, angkatan.value, medsos.value);
    //println!("{:?}", medsos);

    // TODO: Implementasi logic

}