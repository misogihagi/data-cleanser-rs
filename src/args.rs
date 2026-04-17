use clap::Parser;

macro_rules! define_args {
    ($($site:ident),* $(,)?) => {
        #[derive(Parser, Debug)]
        #[command(author, version, about, long_about = None)]
        pub struct Args {
            /// Run all sites
            #[arg(long, default_value_t = false)]
            pub all: bool,

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
                    if self.all || self.$site {
                        ret.push(stringify!($site));
                    }
                )*
                ret
            }

            pub fn is_elitenetwork(&self) -> bool {
                self.all || self.elitenetwork
            }

            pub fn is_mitsue(&self) -> bool {
                self.all || self.mitsue
            }

            pub fn is_token(&self) -> bool {
                self.all || self.token
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
    efjapan,
    ena,
    esp,
    fastretailing,
    felissimo,
    footballbox,
    fukuwatanabe,
    globis,
    goonet,
    gurubi,
    hiroshima,
    homemateresearchsoccer,
    hrpro,
    jfa,
    jfadocuments,
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
    sufu,
    suumo,
    theglenlivet,
    universalooh,
    wafermeasurementinspection,
    webtan,
    yodosha,
    zexy,
);
