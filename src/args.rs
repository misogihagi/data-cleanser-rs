use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long, default_value_t = false)]
    ajima: bool,
    #[arg(long, default_value_t = false)]
    aritayaki: bool,
    #[arg(long, default_value_t = false)]
    athome: bool,
    #[arg(long, default_value_t = false)]
    beer: bool,
    #[arg(long, default_value_t = false)]
    chemicoat: bool,
    #[arg(long, default_value_t = false)]
    cybernet: bool,
    #[arg(long, default_value_t = false)]
    ena: bool,
    #[arg(long, default_value_t = false)]
    esp: bool,
    #[arg(long, default_value_t = false)]
    fastretailing: bool,
    #[arg(long, default_value_t = false)]
    goonet: bool,
    #[arg(long, default_value_t = false)]
    gurubi: bool,
    #[arg(long, default_value_t = false)]
    hiroshima: bool,
    #[arg(long, default_value_t = false)]
    hrpro: bool,
    #[arg(long, default_value_t = false)]
    jmac: bool,
    #[arg(long, default_value_t = false)]
    kenchikuyogo: bool,
    #[arg(long, default_value_t = false)]
    macromill: bool,
    #[arg(long, default_value_t = false)]
    meiwakaiun: bool,
    #[arg(long, default_value_t = false)]
    mintetsu: bool,
    #[arg(long, default_value_t = false)]
    nomura: bool,
    #[arg(long, default_value_t = false)]
    ntt: bool,
    #[arg(long, default_value_t = false)]
    ryugaku: bool,
    #[arg(long, default_value_t = false)]
    smbcnikko: bool,
    #[arg(long, default_value_t = false)]
    soccer: bool,
    #[arg(long, default_value_t = false)]
    sumai1: bool,
    #[arg(long, default_value_t = false)]
    theglenlivet: bool,
    #[arg(long, default_value_t = false)]
    wafermeasurementinspection: bool,
    #[arg(long, default_value_t = false)]
    webtan: bool,

    #[arg(long, default_value_t = false)]
    pub mitsue: bool,
}

impl Args {
    pub fn simples(&self) -> Vec<&str> {
        let mut ret = vec![];
        if self.ajima {
            ret.push("ajima")
        }
        if self.aritayaki {
            ret.push("aritayaki")
        }
        if self.athome {
            ret.push("athome")
        }
        if self.beer {
            ret.push("beer")
        }
        if self.chemicoat {
            ret.push("chemicoat")
        }
        if self.cybernet {
            ret.push("cybernet")
        }
        if self.ena {
            ret.push("ena")
        }
        if self.esp {
            ret.push("esp")
        }
        if self.fastretailing {
            ret.push("fastretailing")
        }
        if self.goonet {
            ret.push("goonet")
        }
        if self.gurubi {
            ret.push("gurubi")
        }
        if self.jmac {
            ret.push("jmac")
        }
        if self.hrpro {
            ret.push("hrpro")
        }
        if self.hiroshima {
            ret.push("hiroshima")
        }
        if self.kenchikuyogo {
            ret.push("kenchikuyogo")
        }
        if self.macromill {
            ret.push("macromill")
        }
        if self.meiwakaiun {
            ret.push("meiwakaiun")
        }
        if self.mintetsu {
            ret.push("mintetsu")
        }
        if self.nomura {
            ret.push("nomura")
        }
        if self.ntt {
            ret.push("ntt")
        }
        if self.ryugaku {
            ret.push("ryugaku")
        }
        if self.smbcnikko {
            ret.push("smbcnikko")
        }
        if self.soccer {
            ret.push("soccer")
        }
        if self.sumai1 {
            ret.push("sumai1")
        }
        if self.theglenlivet {
            ret.push("theglenlivet")
        }
        if self.wafermeasurementinspection {
            ret.push("wafermeasurementinspection")
        }
        if self.webtan {
            ret.push("webtan")
        }
        if self.smbcnikko {
            ret.push("smbcnikko")
        }
        ret
    }
}
