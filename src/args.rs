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
    felissimo: bool,
    #[arg(long, default_value_t = false)]
    goonet: bool,
    #[arg(long, default_value_t = false)]
    gurubi: bool,
    #[arg(long, default_value_t = false)]
    hiroshima: bool,
    #[arg(long, default_value_t = false)]
    hrpro: bool,
    #[arg(long, default_value_t = false)]
    jhs: bool,
    #[arg(long, default_value_t = false)]
    jmac: bool,
    #[arg(long, default_value_t = false)]
    kenchikuyogo: bool,
    #[arg(long, default_value_t = false)]
    livable: bool,
    #[arg(long, default_value_t = false)]
    macromill: bool,
    #[arg(long, default_value_t = false)]
    meiwakaiun: bool,
    #[arg(long, default_value_t = false)]
    mintetsu: bool,
    #[arg(long, default_value_t = false)]
    mizuho: bool,
    #[arg(long, default_value_t = false)]
    moonlight: bool,
    #[arg(long, default_value_t = false)]
    naigai: bool,
    #[arg(long, default_value_t = false)]
    nikken: bool,
    #[arg(long, default_value_t = false)]
    nittsu: bool,
    #[arg(long, default_value_t = false)]
    nomura: bool,
    #[arg(long, default_value_t = false)]
    nrisecure: bool,
    #[arg(long, default_value_t = false)]
    ntt: bool,
    #[arg(long, default_value_t = false)]
    pfa: bool,
    #[arg(long, default_value_t = false)]
    rewords: bool,
    #[arg(long, default_value_t = false)]
    ri: bool,
    #[arg(long, default_value_t = false)]
    ryugaku: bool,
    #[arg(long, default_value_t = false)]
    shimauma: bool,
    #[arg(long, default_value_t = false)]
    smbcnikko: bool,
    #[arg(long, default_value_t = false)]
    smtrc: bool,
    #[arg(long, default_value_t = false)]
    sobien: bool,
    #[arg(long, default_value_t = false)]
    soccer: bool,
    #[arg(long, default_value_t = false)]
    sompocybersecurity: bool,
    #[arg(long, default_value_t = false)]
    sumai1: bool,
    #[arg(long, default_value_t = false)]
    suumo: bool,
    #[arg(long, default_value_t = false)]
    theglenlivet: bool,
    #[arg(long, default_value_t = false)]
    wafermeasurementinspection: bool,
    #[arg(long, default_value_t = false)]
    webtan: bool,
    #[arg(long, default_value_t = false)]
    yodosha: bool,
    #[arg(long, default_value_t = false)]
    zexy: bool,

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
        if self.felissimo {
            ret.push("felissimo")
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
        if self.jhs {
            ret.push("jhs")
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
        if self.livable {
            ret.push("livable")
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
        if self.mizuho {
            ret.push("mizuho")
        }
        if self.moonlight {
            ret.push("moonlight")
        }
        if self.naigai {
            ret.push("naigai")
        }
        if self.nittsu {
            ret.push("nittsu")
        }
        if self.nikken {
            ret.push("nikken")
        }
        if self.nomura {
            ret.push("nomura")
        }
        if self.nrisecure {
            ret.push("nrisecure")
        }
        if self.ntt {
            ret.push("ntt")
        }
        if self.pfa {
            ret.push("pfa")
        }
        if self.rewords {
            ret.push("rewords")
        }
        if self.ri {
            ret.push("ri")
        }
        if self.ryugaku {
            ret.push("ryugaku")
        }
        if self.shimauma {
            ret.push("shimauma")
        }
        if self.smbcnikko {
            ret.push("smbcnikko")
        }
        if self.smtrc {
            ret.push("smtrc")
        }
        if self.sobien {
            ret.push("sobien")
        }
        if self.soccer {
            ret.push("soccer")
        }
        if self.sompocybersecurity {
            ret.push("sompocybersecurity")
        }
        if self.sumai1 {
            ret.push("sumai1")
        }
        if self.suumo {
            ret.push("suumo")
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
        if self.yodosha {
            ret.push("yodosha")
        }
        if self.zexy {
            ret.push("zexy")
        }
        ret
    }
}
