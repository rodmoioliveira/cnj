pub enum Help {
    Long,
    Short,
}

use Help::*;

macro_rules! help {
    ($func_name:ident,$short:expr,$long:expr) => {
        pub fn $func_name(h: Help) -> &'static str {
            match h {
                Short => $short,
                Long => $long,
            }
        }
    };
}

help!(
    validate_about,
    "Check if a CNJ number has the correct validation digits",
    "\
Check if a CNJ number has the correct validation digits according to the CNJ specification
available at https://github.com/rodmoioliveira/cnj/blob/main/spec/cnj_spec.pdf"
);

help!(
    completion_about,
    "Generate auto-completion for shells",
    "\
Generate auto-completion for several shells:

    cnj completion bash > cnj.bash
    cnj completion fish > cnj.fish
    cnj completion zsh > _cnj
    cnj completion powershell > _cnj.ps1

For bash, move cnj.bash to $XDG_CONFIG_HOME/bash_completion or /etc/bash_completion.d/.

For fish, move cnj.fish to $HOME/.config/fish/completions/.

For zsh, move _cnj to one of your $fpath directories.

For PowerShell, add . _cnj.ps1 to your PowerShell profile (note the leading period).
If the _cnj.ps1 file is not on your PATH, do . /path/to/_cnj.ps1 instead."
);

help!(
    cnjs_help,
    "A list of space-separated CNJs",
    "\
A list of space-separated CNJs. The provided CNJ can be in any of the following
formats:

    with mask:       1234567-38.1011.1.21.3141
    without mask:    12345678910111213141

Examples:

    cnj validate 1234567-38.1011.1.21.3141 12345678910111213141
    echo 1234567-38.1011.1.21.3141 12345678910111213141 | xargs cnj validate
    cat list_of_cnjs.csv | cnj validate"
);

help!(
    format_help,
    "Change the output format",
    "Change the output format"
);
