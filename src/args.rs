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
    amazonpay,
    aritayaki,
    athome,
    beer,
    chemicoat,
    chintai,
    civileng,
    coocan,
    cybernet,
    daiichilife,
    efjapan,
    ebcc,
    ena,
    esp,
    fastretailing,
    felissimo,
    fooddies,
    footballbox,
    footballcottage,
    footballzone,
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
    jiki,
    jmac,
    kabuwatanabe,
    kenchikuyogo,
    kddi,
    konest,
    kuraemon,
    kyokutok,
    livable,
    macromill,
    meiwakaiun,
    mintetsu,
    mizuho,
    moonlight,
    naganofc,
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
    sakaiku,
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
    toraiz,
    unew,
    universalooh,
    wafermeasurementinspection,
    webtan,
    yodosha,
    yatsuhashi,
    zexy,
);
