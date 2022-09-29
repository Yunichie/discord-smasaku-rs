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
    framework::{
        standard::{
            CommandResult
        }
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

pub async fn run(ctx: Context, msg: Message) -> CommandResult {
    let msg = msg
    .channel_id
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

    Ok(())
}