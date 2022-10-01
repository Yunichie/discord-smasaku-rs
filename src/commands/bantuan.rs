use serenity::{
    builder::{
        CreateApplicationCommand
    },
    model::{
        application::{
            interaction::{
                application_command::{
                    ApplicationCommandInteraction
                },
                //Interaction,
                InteractionResponseType
            }
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

/*pub async fn run(ctx: &Context, c_id: ChannelId) {
    let msg = c_id
    .send_message(&ctx.http, |msg| {
        msg
        .embed(|e| {
            e
            .title("Bantuan")
            .fields(vec![
                ("Nama", "`/perkenalan`", false),
                ("Penggunaan", "/perkenalan `nama:` `kelas:` `angkatan:`", false),
                ("Format", r#"
`nama`: `nama kamu`
**Karakter Aa-Zz**
      
`kelas`: `kelas kamu`
**[XI/XII] [MIPA/IPS] [1-12]**,
**[X] [1-12]**
 `angkatan`: `tahun masuk`
**4 digit angka**;
**4 digit angka/4 digit angka**
                "#, false),
                ("Contoh", r#"
1. /perkenalan `nama: Nama Saya` `kelas: X 3` `angkatan: 2021`
2. /perkenalan `nama: Nama Saya` `kelas: XI MIPA 3` `angkatan: 2021/2022` `medsos: @medsos_saya`
                "#, false),
                ("Catatan", r#"
1. Tidak perlu memasukkan tanda [] untuk `kelas`.
2. Gunakan hanya salah satu format dari dua untuk `angkatan`.
3. Tidak perlu memasukkan **spasi** setelah menekan/memilih opsi yang hendak diisi.
4. Tidak perlu memasukkan **spasi** setelah selesai memasukkan `nama`, `kelas`, dan `angkatan`; langsung tekan opsi yang diperlukan.
5. Opsi `medsos` adalah opsional; tidak wajib diisi.
                "#, false)
            ])
            .timestamp(Timestamp::now())
        })
    }).await;

    if let Err(why) = msg {
        println!("Error sending message: {:?}", why);
    }
}*/

pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) {
    let user = &interaction.user;
    //let thumbnail = &user.face();

    let bantuan = interaction
    .create_interaction_response(&ctx.http, |resp| {
        resp
        .kind(InteractionResponseType::ChannelMessageWithSource)
        .interaction_response_data(|msg| {
            msg.embed(|e| {
                e
                .color((247, 10, 10))
                .title("Bantuan")
                .fields(vec![
                    ("Nama", "`/perkenalan`", false),
                    ("Penggunaan", "/perkenalan `nama:` `kelas:` `angkatan:`", false),
                    ("Format", r#"
    `nama`: `nama kamu`
    **Karakter Aa-Zz**
          
    `kelas`: `kelas kamu`
    **[XI/XII] [MIPA/IPS] [1-12]**,
    **[X] [1-12]**
     `angkatan`: `tahun masuk`
    **4 digit angka**;
    **4 digit angka/4 digit angka**
                    "#, false),
                    ("Contoh", r#"
    1. /perkenalan `nama: Nama Saya` `kelas: X 3` `angkatan: 2021`
    2. /perkenalan `nama: Nama Saya` `kelas: XI MIPA 3` `angkatan: 2021/2022` `medsos: @medsos_saya`
                    "#, false),
                    ("Catatan", r#"
    1. Tidak perlu memasukkan tanda [] untuk `kelas`.
    2. Gunakan hanya salah satu format dari dua untuk `angkatan`.
    3. Tidak perlu memasukkan **spasi** setelah menekan/memilih opsi yang hendak diisi.
    4. Tidak perlu memasukkan **spasi** setelah selesai memasukkan `nama`, `kelas`, dan `angkatan`; langsung tekan opsi yang diperlukan.
    5. Opsi `medsos` adalah opsional; tidak wajib diisi.
                    "#, false)
                ])
                .footer(|f| f.icon_url(&user.avatar_url().unwrap()).text(&user.tag()))
                .timestamp(Timestamp::now())
            })
        })
    }).await;
    
    if let Err(why) = bantuan {
        println!("Error sending message: {:?}", why);
    }
}