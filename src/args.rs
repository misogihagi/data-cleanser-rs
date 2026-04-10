use clap::Parser;

macro_rules! define_args {
    ($($site:ident),* $(,)?) => {
        #[derive(Parser, Debug)]
        #[command(author, version, about, long_about = None)]
        pub struct Args {
            $(
                #[arg(long, default_value_t = false)]
                pub $site: bool,
            )*

            #[arg(long, default_value_t = false)]
            pub elitenetwork: bool,
            #[arg(long, default_value_t = false)]
            pub mitsue: bool,
            #[arg(long, default_value_t = false)]
            pub token: bool,
        }

        impl Args {
            pub fn common(&self) -> Vec<&'static str> {
                let mut ret = vec![];
                $(
                    if self.$site {
                        ret.push(stringify!($site));
                    }
                )*
                ret
            }
        }
    };
}

define_args!(
    ajima,
    aritayaki,
    athome,
    beer,
    chemicoat,
    chintai,
    civileng,
    cybernet,
    ena,
    esp,
    fastretailing,
    felissimo,
    fukuwatanabe,
    goonet,
    gurubi,
    hiroshima,
    hrpro,
    jhs,
    jmac,
    kabuwatanabe,
    kenchikuyogo,
    konest,
    kuraemon,
    kyokutok,
    livable,
    macromill,
    meiwakaiun,
    mintetsu,
    mizuho,
    moonlight,
    naigai,
    nichiren,
    nikken,
    nisso,
    nittsu,
    nomura,
    nrisecure,
    ntt,
    pfa,
    rewords,
    ri,
    ryugaku,
    shimauma,
    smbcnikko,
    smtrc,
    sobien,
    soccer,
    sompocybersecurity,
    sumai1,
    suumo,
    theglenlivet,
    universalooh,
    wafermeasurementinspection,
    webtan,
    yodosha,
    zexy,
);