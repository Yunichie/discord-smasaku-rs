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

    println!("{:?}", medsos);
    /*match medsos {
        None => {
            medsos = medsos.unwrap().value.unwrap().as_str().unwrap() = "-";
            medsos = "-";
        }
    }*/

    //let medsos = medsos.unwrap().value.unwrap();
    //let medsos = medsos.as_str().unwrap();

    let user = &options.user;
    //let cache = &ctx.cache;
    let roles = &options.member.as_ref().unwrap().roles;

    // Ganti role id!
    // User sudah mempunyai role smasaku
    if roles.iter().any(|&i| i.as_u64() == &1025826518259224608) {
        println!("contains 1025826518259224608");
    }

    // User belum mempunyai role smasaku
    if roles.iter().any(|&i| i.as_u64() == &1025826518259224608) == false {
        println!("contains 1025826518259224608");
    }

    // RegEx


    let perkenalan_slash = options
    .create_interaction_response(&ctx.http, |resp| {
        resp
        .kind(InteractionResponseType::ChannelMessageWithSource)
        .interaction_response_data(|msg| {
            msg
            //.add_file("./image/welcome.jpg")
            .embed(|e| {
                e
                .color((247, 10, 10))
                .title("Perkenalan")
                .fields(vec![
                    ("Nama", nama.unwrap().value.unwrap().as_str().unwrap(), false),
                    ("Kelas", kelas.unwrap().value.unwrap().as_str().unwrap(), false),
                    ("Angkatan", angkatan.unwrap().value.unwrap().as_str().unwrap(), false),
                    //("Media Sosial", medsos, false)
                ])
                .thumbnail(&user.avatar_url().unwrap())
                .footer(|f| f.text(&user.tag()))
                .timestamp(Timestamp::now())
            })
            //.add_file("./image/welcome.jpg")
        })
    }).await;

    if let Err(why) = perkenalan_slash {
        println!("Error sending message: {:?}", why);
    }
}