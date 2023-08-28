use std::io::Write;

use serde::{Deserialize, Serialize};

use crate::{cli, types::*};

static UNMASKED: once_cell::sync::Lazy<regex::Regex> = once_cell::sync::Lazy::new(|| {
    regex::Regex::new(
        r"(?x)
        ^(?P<NNNNNNN>\d{7})
        (?P<DD>\d{2})
        (?P<AAAA>\d{4})
        (?P<J>\d{1})
        (?P<TR>\d{2})
        (?P<OOOO>\d{4})$",
    )
    .unwrap()
});

static ALPHA: once_cell::sync::Lazy<regex::Regex> =
    once_cell::sync::Lazy::new(|| regex::Regex::new(r"[[:alpha:]]").unwrap());

static PUNCT: once_cell::sync::Lazy<regex::Regex> =
    once_cell::sync::Lazy::new(|| regex::Regex::new(r"[[:punct:]]").unwrap());

static BLANK: once_cell::sync::Lazy<regex::Regex> =
    once_cell::sync::Lazy::new(|| regex::Regex::new(r"[[:blank:]]").unwrap());

static SPACE: once_cell::sync::Lazy<regex::Regex> =
    once_cell::sync::Lazy::new(|| regex::Regex::new(r"[[:space:]]").unwrap());

static CNTRL: once_cell::sync::Lazy<regex::Regex> =
    once_cell::sync::Lazy::new(|| regex::Regex::new(r"[[:cntrl:]]").unwrap());

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cnj {
    pub cnj: String,
    pub nnnnnnn: String,
    pub dd: String,
    pub aaaa: String,
    pub j: String,
    pub tr: String,
    pub oooo: String,
    pub is_valid: bool,
    pub v_dd: String,
    pub v_cnj: String,
}

pub fn unmask(cnj: String) -> String {
    let no_alpha = ALPHA.replace_all(&cnj, "");
    let no_punct = PUNCT.replace_all(&no_alpha, "");
    let no_blank = BLANK.replace_all(&no_punct, "");
    let no_space = SPACE.replace_all(&no_blank, "");
    let no_cntrl = CNTRL.replace_all(&no_space, "");
    no_cntrl.to_string()
}

pub fn mask(cnj: String) -> String {
    UNMASKED
        .replace(&cnj, |caps: &regex::Captures| {
            format!(
                "{}-{}.{}.{}.{}.{}",
                &caps[1], &caps[2], &caps[3], &caps[4], &caps[5], &caps[6],
            )
        })
        .into()
}

pub fn has_20_len(cnj: &String) -> bool {
    let cnj_len = cnj.len();
    let has_20_len = cnj_len == 20;
    let is_empty = cnj.is_empty();

    if !is_empty && !has_20_len {
        if cnj_len < 20 {
            writeln!(std::io::stderr(), "Error: length < than 20 [{cnj}]").ok();
        } else {
            writeln!(std::io::stderr(), "Error: length > than 20 [{cnj}]").ok();
        }
    }

    has_20_len
}

pub fn new(cnj: String) -> Cnj {
    let captures = &UNMASKED.captures(&cnj).unwrap();

    let nnnnnnn = captures.get(1).unwrap().as_str().to_owned();
    let dd = captures.get(2).unwrap().as_str().to_owned();
    let aaaa = captures.get(3).unwrap().as_str().to_owned();
    let j = captures.get(4).unwrap().as_str().to_owned();
    let tr = captures.get(5).unwrap().as_str().to_owned();
    let oooo = captures.get(6).unwrap().as_str().to_owned();

    let cnj = mask(cnj);

    Cnj {
        cnj: cnj.clone(),
        nnnnnnn,
        dd: dd.clone(),
        aaaa,
        j,
        tr,
        oooo,
        is_valid: false,
        v_dd: dd,
        v_cnj: cnj,
    }
}

pub fn check_dd(cnj: Cnj) -> Cnj {
    let nnnnnnn = &cnj.nnnnnnn;
    let dd = &cnj.dd;
    let aaaa = &cnj.aaaa;
    let j = &cnj.j;
    let tr = &cnj.tr;
    let oooo = &cnj.oooo;

    // Check out implementation details in:
    // https://atos.cnj.jus.br/files/resolucao_65_16122008_04032013165912.pdf
    let remainder = nnnnnnn.parse::<i32>().unwrap() % 97;
    let remainder2 = format!("{remainder}{aaaa}{j}{tr}").parse::<i32>().unwrap() % 97;
    let remainder3 = format!("{remainder2}{oooo}00").parse::<i32>().unwrap() % 97;
    let calc_dd = 98 - remainder3;

    let mut r = cnj.clone();
    r.is_valid = calc_dd == dd.parse::<i32>().unwrap();
    let mut v_dd = calc_dd.to_string();
    if v_dd.len() == 1usize {
        v_dd = format!("0{v_dd}");
    }
    r.v_dd = v_dd.clone();
    r.v_cnj = format!("{nnnnnnn}-{v_dd}.{aaaa}.{j}.{tr}.{oooo}");
    r
}

pub fn print(cnjs: Vec<Cnj>, output: cli::Output) -> Result<()> {
    use crate::cli::Output::*;

    if cnjs.is_empty() {
        writeln!(
            std::io::stderr(),
            "Error: there aren't valid CNJ numbers to print."
        )?;
        std::process::exit(1);
    }

    match output {
        Csv => {
            let mut wtr = csv::WriterBuilder::new().from_writer(std::io::stdout());
            for c in cnjs {
                wtr.serialize(c)?;
            }
            wtr.flush()?;

            Ok(())
        }
        Json => {
            writeln!(
                std::io::stdout(),
                "{}",
                serde_json::to_string(&cnjs).unwrap()
            )?;

            Ok(())
        }
        Table => {
            let s = format!("{space:width$}", space = " ", width = 4);
            let mut stdout = std::io::stdout();

            writeln!(
                stdout,
                "{cnj:<25}{s}nnnnnnn{s}dd{s}aaaa{s}j{s}tr{s}oooo{s}is_valid{s}v_dd{s}v_cnj",
                cnj = "cnj"
            )?;

            for c in cnjs {
                writeln!(
                    stdout,
                    "{cnj}{s}{nnnnnnn}{s}{dd}{s}{aaaa}{s}{j}{s}{tr}{s}{oooo}{s}{is_valid:<8}{s}{v_dd}{s}  {v_cnj}",
                    cnj = c.cnj,
                    nnnnnnn = c.nnnnnnn,
                    dd = c.dd,
                    aaaa = c.aaaa,
                    j = c.j,
                    tr = c.tr,
                    oooo = c.oooo,
                    is_valid = c.is_valid,
                    v_dd = c.v_dd,
                    v_cnj = c.v_cnj,
                )?;
            }

            Ok(())
        }
        Vertical => {
            let s = format!("{space:width$}", space = " ", width = 1);
            let mut stdout = std::io::stdout();

            writeln!(
                stdout,
                "------------------------------------------------------"
            )?;

            for c in cnjs {
                writeln!(stdout, "     cnj:{s}{}", c.cnj)?;
                writeln!(stdout, " nnnnnnn:{s}{}", c.nnnnnnn)?;
                writeln!(stdout, "      dd:{s}{}", c.dd)?;
                writeln!(stdout, "    aaaa:{s}{}", c.aaaa)?;
                writeln!(stdout, "       j:{s}{}", c.j)?;
                writeln!(stdout, "      tr:{s}{}", c.tr)?;
                writeln!(stdout, "    oooo:{s}{}", c.oooo)?;
                writeln!(stdout, "is_valid:{s}{}", c.is_valid)?;
                writeln!(stdout, "    v_dd:{s}{}", c.v_dd)?;
                writeln!(stdout, "   v_cnj:{s}{}", c.v_cnj)?;

                writeln!(
                    stdout,
                    "------------------------------------------------------"
                )?;
            }

            Ok(())
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::cnj;

    #[test]
    fn test_valid_cnj() {
        let valid: cnj::Cnj = vec!["1234567-38.1011.1.21.3141".to_string()]
            .into_iter()
            .map(cnj::unmask)
            .filter(cnj::has_20_len)
            .map(cnj::new)
            .map(cnj::check_dd)
            .collect::<Vec<cnj::Cnj>>()
            .first()
            .unwrap()
            .clone();

        assert!(valid.is_valid);
        assert_eq!(valid.dd, valid.v_dd);
        assert_eq!(valid.v_dd, "38");

        let nnnnnnn = &valid.nnnnnnn;
        let v_dd = &valid.v_dd;
        let aaaa = &valid.aaaa;
        let j = &valid.j;
        let tr = &valid.tr;
        let oooo = &valid.oooo;

        assert_eq!(
            format!("{nnnnnnn}{aaaa}{j}{tr}{oooo}{v_dd}")
                .parse::<i128>()
                .unwrap()
                % 97,
            1
        );
    }

    #[test]
    fn test_invalid_cnj() {
        let invalid: cnj::Cnj = vec!["1234567-89.1011.1.21.3141".to_string()]
            .into_iter()
            .map(cnj::unmask)
            .filter(cnj::has_20_len)
            .map(cnj::new)
            .map(cnj::check_dd)
            .collect::<Vec<cnj::Cnj>>()
            .first()
            .unwrap()
            .clone();

        assert!(!invalid.is_valid);
        assert_ne!(invalid.dd, invalid.v_dd);
        assert_eq!(invalid.v_dd, "38");

        let nnnnnnn = &invalid.nnnnnnn;
        let dd = &invalid.dd;
        let v_dd = &invalid.v_dd;
        let aaaa = &invalid.aaaa;
        let j = &invalid.j;
        let tr = &invalid.tr;
        let oooo = &invalid.oooo;

        assert_ne!(
            format!("{nnnnnnn}{aaaa}{j}{tr}{oooo}{dd}")
                .parse::<i128>()
                .unwrap()
                % 97,
            1
        );

        assert_eq!(
            format!("{nnnnnnn}{aaaa}{j}{tr}{oooo}{v_dd}")
                .parse::<i128>()
                .unwrap()
                % 97,
            1
        );
    }
}
