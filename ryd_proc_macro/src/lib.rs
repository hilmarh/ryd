use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Arf" => "Err",
        "Bien" => "Ok",
        "Strengur" => "String",
        "Dictionnaire" => "HashMap",
        "Défaut" => "Default",
        "Erreur" => "Error",
        "PeutÊtre" => "Option",
        "Quelque" => "Some",
        "Ekkert" => "None",
        "Résultat" => "Result",
        "Soi" => "Self",
        "affiche" => "println",
        "arrête" => "break",
        "asynchrone" => "async",
        "attend" => "await",
        "boucle" => "loop",
        "bouge" => "move",
        "cagette" => "crate",
        "code_inaccessible" => "unreachable_code",
        "comme" => "as",
        "constant" => "const",
        "convention" => "trait",
        "dangereux" => "unsafe",
        "de" => "in",
        "depuis" => "from",
        "dynamique" => "dyn",
        "déballer" => "unwrap",
        "défaut" => "default",
        "en_réf" => "as_ref",
        "es" => "io",
        "externe" => "extern",
        "faux" => "false",
        "fall" => "fn",
        "génial" => "super",
        "insérer" => "insert",
        "lire" => "get",
        "légal" => "allow",
        "merde" | "calisse" | "oups" => "panic",
        "module" => "mod",
        "mutable" => "mut",
        "nouveau" => "new",
        "où" => "where",
        "pour" => "for",
        "prendre_ou_insérer_avec" => "get_or_insert_with",
        "aðal" => "main",
        "public" => "pub",
        "á" => None?,
        "að" => None?,
        "renvoie" => "return",
        "réalisation" => "impl",
        "réf" => "ref",
        "selon" => "match",
        "ef" => "if",
        "annars" => "else",
        "soi" => "self",
        "láta" => "let",
        "statique" => "static",
        "structure" => "struct",
        "suppose" => "expect",
        "meðan" => "while",
        "nota" => "use",
        "vers" => "into",
        "vrai" => "true",
        "énumération" => "enum",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn ryd(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
