pub fn ternary_fn(condition: bool) -> &'static str {
    let result = if condition { "Kgeui" } else { "Fodase" }; // return não é obrigatorio. A ultima declaracao de um ecopo sera retornado para o escopo pai;

    return result;
}
