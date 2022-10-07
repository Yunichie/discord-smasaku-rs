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
                    application_command::{
                        ApplicationCommandInteraction
                    },
                    InteractionResponseType
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
        .create_option(|option| {
            option
                .name("medsos")
                .description("Media sosial kamu! (opsional)")
                .kind(CommandOptionType::String)
                .required(false)
        })
}

pub async fn run(options: &ApplicationCommandInteraction, ctx: &Context) {
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

    let mut medsos = options
    .data
    .options
    .iter()
    .find(|option| option.name == "medsos")
    .cloned();

    let nama = nama.unwrap().value.unwrap().as_str().unwrap().to_owned();
    let kelas = kelas.unwrap().value.unwrap().as_str().unwrap().to_owned();
    let angkatan = angkatan.unwrap().value.unwrap().as_str().unwrap().to_owned();

    println!("{:?}", medsos);
    
    // TODO: medsos
    /*match medsos {
        None => println!("none"),
        Some(s) => {
            let medsos = &medsos.unwrap().value.unwrap().as_str().unwrap();
        }
    }*/

    /*if medsos.is_none() {
        // change medsos
    }*/

    //let medsos = medsos.unwrap().value.unwrap();
    //let medsos = medsos.as_str().unwrap();

    let user = &options.user;
    //let cache = &ctx.cache;
    let roles = &options.member.as_ref().unwrap().roles;
    let role_id = &1025826518259224608;
    let ch_id = &1024284784077320255;

    // TODO: role id & channel id
    // User sudah mempunyai role smasaku
    if roles.iter().any(|&i| i.as_u64() == role_id) {
        let sudah_punya_role = options.channel_id.say(&ctx.http, "Kamu sudah memperkenalkan diri!").await;

        if let Err(why) = sudah_punya_role {
            println!("Error sending message: {:?}", why);
        }
    }

    // User belum mempunyai role smasaku
    if !roles.iter().any(|&i| i.as_u64() == role_id) && options.channel_id.as_u64() != ch_id {
        let ch_err = options.channel_id.say(&ctx.http, "Kamu hanya bisa memperkenalkan diri di #introduction").await;

        if let Err(why) = ch_err {
            println!("Error sending message: {:?}", why);
        }
    }

    // TODO: RegEx; embed error
    // RegEx
    let xi_xii = Regex::new(r"^(XI|XII)\s(MIPA|IPS)\s([0-9]|1[0-2])$").unwrap();
    let x = Regex::new(r"^(X)\s([0-9]|1[0-2])$").unwrap();
    let regex_angkatan = Regex::new(r#"^(([0-9]){4})|(^([0-9]){4}/([0-9]){4})$"#).unwrap();

    if !(x.is_match(&kelas)) && !(xi_xii.is_match(&kelas)) {
        // Kirim embed error!
        let kls_err = options
        .create_interaction_response(&ctx.http, |resp| {
            resp
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|msg| {
                msg
                .embed(|e| {
                    e
                    .color((247, 10, 10))
                    .title("Format perkenalan tidak sesuai!")
                    .description(r#"
Format Kelas yang benar:
**[XI/XII] [MIPA/IPS] [1-12]**;
**X [1-12]**.

Contoh: X 3, XI MIPA 3
                    "#)
                    .footer(|f| f.text("[] tidak perlu dimasukkan."))
                })
            })
        }).await;

        if let Err(why) = kls_err {
            println!("Error sending message: {:?}", why);
        }
    }

    if !(regex_angkatan.is_match(&angkatan)) {
        // Kirim embed error!
        let angkt_err = options
        .create_interaction_response(&ctx.http, |resp| {
            resp
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|msg| {
                msg
                .embed(|e| {
                    e
                    .color((247, 10, 10))
                    .title("Format perkenalan tidak sesuai!")
                    .description(r#"
Format Angkatan yang benar:
1. **[tahun]/[tahun]**;
2. **[tahun masuk]**.

Contoh: 2021/2022 atau cukup 2021.
                    "#)
                    .footer(|f| f.text("[] tidak perlu dimasukkan."))
                })
            })
        }).await;

        if let Err(why) = angkt_err {
            println!("Error sending message: {:?}", why);
        }
    }

    if !roles.iter().any(|&i| i.as_u64() == role_id) && options.channel_id.as_u64() == ch_id {
        let perkenalan_slash = options
    .create_interaction_response(&ctx.http, |resp| {
        resp
        .kind(InteractionResponseType::ChannelMessageWithSource)
        .interaction_response_data(|msg| {
            msg
            .embed(|e| {
                e
                //.attachment("./image/welcome.jpg")
                .color((247, 10, 10))
                .title("Perkenalan")
                .fields(vec![
                    ("Nama", capitalize::capitalize(&nama), false),
                    ("Kelas", kelas.to_uppercase(), false),
                    ("Angkatan", angkatan, false),
                    //("Media Sosial", medsos, false)
                ])
                .thumbnail(&user.avatar_url().unwrap())
                .footer(|f| f.text(&user.tag()))
                .timestamp(Timestamp::now())
            })
            // TODO: .add_file()
            //.add_file("./image/welcome.jpg")
        })//.edit_original_interaction_response()
    })
    .await;

        let _followup = options
        .create_followup_message(&ctx.http, |resp| {
            resp.content("@user Jangan lupa untuk mengambil _roles_ di #roles setelah memperkenalkan diri.")
        }).await;
        
        if let Err(why) = perkenalan_slash {
            println!("Error sending message: {:?}", why);
        }
    }
}