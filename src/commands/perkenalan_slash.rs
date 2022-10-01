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
    
    let user = &options.user;
    let thumbnail = &user.avatar_url().unwrap();

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
                    ("Media Sosial", "Belum diimplementasi!", false)
                ])
                .thumbnail(thumbnail)
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