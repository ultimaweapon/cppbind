use super::{Segment, Symbol, SymbolError, TemplateArg};
use std::cmp::min;
use std::iter::Peekable;
use std::slice::Iter;

pub fn parse(mangled: &[u8]) -> Result<Symbol<'static>, SymbolError> {
    let mut name = Vec::new();
    let mut iter = mangled.iter().peekable();

    match *iter.next().ok_or(SymbolError::UnknownSymbol)? {
        b'N' => parse_nested_name(&mut name, &mut iter)?,
        _ => return Err(SymbolError::UnknownSymbol),
    }

    Ok(Symbol { name, sig: None })
}

fn parse_nested_name(
    segments: &mut Vec<Segment>,
    iter: &mut Peekable<Iter<u8>>,
) -> Result<(), SymbolError> {
    loop {
        let b = *iter.next().ok_or(SymbolError::UnknownSymbol)?;

        match b {
            b'0' => return Err(SymbolError::UnknownSymbol), // Identifier with zero length?
            b'1'..=b'9' => segments.push(Segment::Ident(parse_source_name(iter, b)?.into())),
            b'I' => parse_template_args(segments, iter)?,
            b'E' => break,
            _ => return Err(SymbolError::UnknownSymbol),
        }
    }

    Ok(())
}

fn parse_source_name(iter: &mut Peekable<Iter<u8>>, first: u8) -> Result<String, SymbolError> {
    // Get length.
    let mut len = Into::<usize>::into(first - b'0');

    while let Some(&b) = iter.next_if(|b| b.is_ascii_digit()) {
        len = len
            .checked_mul(10)
            .and_then(move |v| v.checked_add((b - b'0').into()))
            .ok_or(SymbolError::UnknownSymbol)?;
    }

    // This ABI does not yet specify a mangling for identifiers containing characters outside of
    // _A-Za-z0-9.
    let mut name = String::with_capacity(min(len, 128));

    for _ in 0..len {
        // We don't need to handle unicode here due to the above rule.
        let b = *iter.next().ok_or(SymbolError::UnknownSymbol)?;

        name.push(b.into());
    }

    Ok(name)
}

fn parse_template_args(
    segments: &mut Vec<Segment>,
    iter: &mut Peekable<Iter<u8>>,
) -> Result<(), SymbolError> {
    loop {
        let b = *iter.next().ok_or(SymbolError::UnknownSymbol)?;
        let a = match b {
            b'E' => break,
            b'0' => return Err(SymbolError::UnknownSymbol), // Identifier with zero length?
            b'1'..=b'9' => TemplateArg::Ident(parse_source_name(iter, b)?.into()),
            b'X' => todo!("expression"),
            b'L' => todo!("simple expressions"),
            b'J' => todo!("argument pack"),
            _ => todo!(),
        };

        segments.push(Segment::TemplateArg(a));
    }

    Ok(())
}
