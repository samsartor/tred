#[derive(Clone, Debug)]
pub enum Token {
    MapEntry(
        ::std::string::String,
        ::std::option::Option<::std::boxed::Box<Token>>,
    ),
    Map(::std::vec::Vec<::std::boxed::Box<Token>>),
    Array(::std::vec::Vec<::std::boxed::Box<Token>>),
    String(::std::string::String),
    Number(::std::string::String),
    True,
    False,
    Null,
}
pub fn parse(
    input: &str,
) -> Result<::std::vec::Vec<::std::boxed::Box<Token>>, ::tredlib::ParseErr> {
    match _blockfn_0(0usize, input) {
        Result::Ok((_, tree)) => Result::Ok(tree),
        Result::Err(err) => Result::Err(err),
    }
}
lazy_static ! { static ref _REGEX_7 : :: tredlib :: regex :: Regex = :: tredlib :: regex :: Regex :: new ( "^u[\\da-fA-F]{6}" ) . unwrap ( ) ; static ref _REGEX_43 : :: tredlib :: regex :: Regex = :: tredlib :: regex :: Regex :: new ( "^[1-9]" ) . unwrap ( ) ; static ref _REGEX_46 : :: tredlib :: regex :: Regex = :: tredlib :: regex :: Regex :: new ( "^\\d" ) . unwrap ( ) ; static ref _REGEX_4 : :: tredlib :: regex :: Regex = :: tredlib :: regex :: Regex :: new ( "^[^\\\\\"]" ) . unwrap ( ) ; static ref _REGEX_0 : :: tredlib :: regex :: Regex = :: tredlib :: regex :: Regex :: new ( "^[\\s\\n\\r]*" ) . unwrap ( ) ; static ref _REGEX_1 : :: tredlib :: regex :: Regex = :: tredlib :: regex :: Regex :: new ( "^[\\s\\n\\r]+" ) . unwrap ( ) ; }
fn _blockfn_0(
    _start: usize,
    _text: &str,
) -> Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>), ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_append!(
        _at,
        _resvec_61,
        {
            _out.append(_resvec_61);
        },
        _blockfn_8(_at, _text)?
    );
    Ok((_at, _out))
}
fn _blockfn_1(
    _start: usize,
    _text: &str,
) -> Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>), ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_some!(_at, _text, _resvec_2, {}, _blockfn_9(_at, _text));
    Ok((_at, _out))
}
fn _blockfn_2(
    _start: usize,
    _text: &str,
) -> Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>), ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_append!(_at, _resvec_8, {}, _tredgen_match_str!(_at, _text, "\"")?);
    let mut _start_9: usize;
    _start_9 = _at;
    _tredgen_append!(_at, _resvec_10, {}, _blockfn_1(_at, _text)?);
    _out.extend(Some(::std::boxed::Box::new(Token::String(
        _text[_start_9.._at].to_owned(),
    ))));
    _tredgen_append!(_at, _resvec_11, {}, _tredgen_match_str!(_at, _text, "\"")?);
    Ok((_at, _out))
}
fn _blockfn_3(
    _start: usize,
    _text: &str,
) -> Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>), ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_append!(_at, _resvec_12, {}, _tredgen_match_str!(_at, _text, "\"")?);
    let mut _start_13: usize;
    _start_13 = _at;
    _tredgen_append!(_at, _resvec_14, {}, _blockfn_1(_at, _text)?);
    let mut _acc_15 = _text[_start_13.._at].to_owned();
    _tredgen_append!(_at, _resvec_16, {}, _tredgen_match_str!(_at, _text, "\"")?);
    _tredgen_append!(
        _at,
        _resvec_17,
        {},
        _tredgen_match_regex!(_at, _text, _REGEX_0)?
    );
    _tredgen_append!(_at, _resvec_18, {}, _tredgen_match_str!(_at, _text, ":")?);
    _tredgen_append!(
        _at,
        _resvec_19,
        {},
        _tredgen_match_regex!(_at, _text, _REGEX_0)?
    );
    let mut _intoonce_20 = None;
    _tredgen_append!(
        _at,
        _resvec_21,
        {
            _intoonce_20 = _resvec_21.pop().or(_intoonce_20);
        },
        _blockfn_8(_at, _text)?
    );
    _out.extend(Some(::std::boxed::Box::new(Token::MapEntry(
        _acc_15.clone(),
        _intoonce_20.clone(),
    ))));
    Ok((_at, _out))
}
fn _blockfn_4(
    _start: usize,
    _text: &str,
) -> Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>), ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_append!(_at, _resvec_22, {}, _tredgen_match_str!(_at, _text, "{")?);
    _tredgen_append!(
        _at,
        _resvec_23,
        {},
        _tredgen_match_regex!(_at, _text, _REGEX_0)?
    );
    let mut _intolist_24 = ::std::vec::Vec::new();
    _tredgen_some!(
        _at,
        _text,
        _resvec_25,
        {
            _intolist_24.append(_resvec_25);
        },
        _blockfn_3(_at, _text),
        _blockfn_11(_at, _text)
    );
    _out.extend(Some(::std::boxed::Box::new(Token::Map(
        _intolist_24.clone(),
    ))));
    _tredgen_append!(
        _at,
        _resvec_29,
        {
            _intolist_24.append(_resvec_29);
        },
        _tredgen_match_regex!(_at, _text, _REGEX_0)?
    );
    _tredgen_append!(
        _at,
        _resvec_30,
        {
            _intolist_24.append(_resvec_30);
        },
        _tredgen_match_str!(_at, _text, "}")?
    );
    Ok((_at, _out))
}
fn _blockfn_5(
    _start: usize,
    _text: &str,
) -> Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>), ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_append!(_at, _resvec_31, {}, _tredgen_match_str!(_at, _text, "[")?);
    _tredgen_append!(
        _at,
        _resvec_32,
        {},
        _tredgen_match_regex!(_at, _text, _REGEX_0)?
    );
    let mut _intolist_33 = ::std::vec::Vec::new();
    _tredgen_some!(
        _at,
        _text,
        _resvec_34,
        {
            _intolist_33.append(_resvec_34);
        },
        _blockfn_8(_at, _text),
        _blockfn_12(_at, _text)
    );
    _out.extend(Some(::std::boxed::Box::new(Token::Array(
        _intolist_33.clone(),
    ))));
    _tredgen_append!(
        _at,
        _resvec_38,
        {
            _intolist_33.append(_resvec_38);
        },
        _tredgen_match_regex!(_at, _text, _REGEX_0)?
    );
    _tredgen_append!(
        _at,
        _resvec_39,
        {
            _intolist_33.append(_resvec_39);
        },
        _tredgen_match_str!(_at, _text, "]")?
    );
    Ok((_at, _out))
}
fn _blockfn_6(
    _start: usize,
    _text: &str,
) -> Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>), ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    let mut _start_40: usize;
    _start_40 = _at;
    _tredgen_option!(
        _at,
        _text,
        _resvec_41,
        {},
        _tredgen_match_str!(_at, _text, "-")
    );
    _tredgen_or!(
        _at,
        _text,
        _resvec_42,
        {},
        _tredgen_match_str!(_at, _text, "0"),
        _blockfn_13(_at, _text)
    );
    _tredgen_option!(_at, _text, _resvec_47, {}, _blockfn_14(_at, _text));
    _tredgen_option!(_at, _text, _resvec_51, {}, _blockfn_15(_at, _text));
    _out.extend(Some(::std::boxed::Box::new(Token::Number(
        _text[_start_40.._at].to_owned(),
    ))));
    Ok((_at, _out))
}
fn _blockfn_7(
    _start: usize,
    _text: &str,
) -> Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>), ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_or!(
        _at,
        _text,
        _resvec_56,
        {
            _out.append(_resvec_56);
        },
        _blockfn_16(_at, _text),
        _blockfn_17(_at, _text),
        _blockfn_18(_at, _text)
    );
    Ok((_at, _out))
}
fn _blockfn_8(
    _start: usize,
    _text: &str,
) -> Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>), ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_or!(
        _at,
        _text,
        _resvec_60,
        {
            _out.append(_resvec_60);
        },
        _blockfn_2(_at, _text),
        _blockfn_4(_at, _text),
        _blockfn_5(_at, _text),
        _blockfn_7(_at, _text),
        _blockfn_6(_at, _text)
    );
    Ok((_at, _out))
}
fn _blockfn_9(
    _start: usize,
    _text: &str,
) -> Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>), ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_or!(
        _at,
        _text,
        _resvec_3,
        {},
        _tredgen_match_regex!(_at, _text, _REGEX_4),
        _blockfn_10(_at, _text)
    );
    Ok((_at, _out))
}
fn _blockfn_10(
    _start: usize,
    _text: &str,
) -> Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>), ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_append!(_at, _resvec_5, {}, _tredgen_match_str!(_at, _text, "\\")?);
    _tredgen_or!(
        _at,
        _text,
        _resvec_6,
        {},
        _tredgen_match_str!(_at, _text, "\""),
        _tredgen_match_str!(_at, _text, "\\"),
        _tredgen_match_str!(_at, _text, "/"),
        _tredgen_match_str!(_at, _text, "b"),
        _tredgen_match_str!(_at, _text, "n"),
        _tredgen_match_str!(_at, _text, "f"),
        _tredgen_match_str!(_at, _text, "n"),
        _tredgen_match_str!(_at, _text, "r"),
        _tredgen_match_str!(_at, _text, "t"),
        _tredgen_match_regex!(_at, _text, _REGEX_7)
    );
    Ok((_at, _out))
}
fn _blockfn_11(
    _start: usize,
    _text: &str,
) -> Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>), ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_append!(
        _at,
        _resvec_26,
        {},
        _tredgen_match_regex!(_at, _text, _REGEX_0)?
    );
    _tredgen_append!(_at, _resvec_27, {}, _tredgen_match_str!(_at, _text, ",")?);
    _tredgen_append!(
        _at,
        _resvec_28,
        {},
        _tredgen_match_regex!(_at, _text, _REGEX_0)?
    );
    Ok((_at, _out))
}
fn _blockfn_12(
    _start: usize,
    _text: &str,
) -> Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>), ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_append!(
        _at,
        _resvec_35,
        {},
        _tredgen_match_regex!(_at, _text, _REGEX_0)?
    );
    _tredgen_append!(_at, _resvec_36, {}, _tredgen_match_str!(_at, _text, ",")?);
    _tredgen_append!(
        _at,
        _resvec_37,
        {},
        _tredgen_match_regex!(_at, _text, _REGEX_0)?
    );
    Ok((_at, _out))
}
fn _blockfn_13(
    _start: usize,
    _text: &str,
) -> Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>), ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_append!(
        _at,
        _resvec_44,
        {},
        _tredgen_match_regex!(_at, _text, _REGEX_43)?
    );
    _tredgen_some!(
        _at,
        _text,
        _resvec_45,
        {},
        _tredgen_match_regex!(_at, _text, _REGEX_46)
    );
    Ok((_at, _out))
}
fn _blockfn_14(
    _start: usize,
    _text: &str,
) -> Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>), ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_append!(_at, _resvec_48, {}, _tredgen_match_str!(_at, _text, ".")?);
    _tredgen_many!(
        _at,
        _text,
        _resvec_49,
        {},
        _tredgen_match_regex!(_at, _text, _REGEX_46)
    );
    Ok((_at, _out))
}
fn _blockfn_15(
    _start: usize,
    _text: &str,
) -> Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>), ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_or!(
        _at,
        _text,
        _resvec_52,
        {},
        _tredgen_match_str!(_at, _text, "e"),
        _tredgen_match_str!(_at, _text, "E")
    );
    _tredgen_or!(
        _at,
        _text,
        _resvec_53,
        {},
        _tredgen_match_str!(_at, _text, "+"),
        _tredgen_match_str!(_at, _text, "-"),
        _tredgen_match_str!(_at, _text, "")
    );
    _tredgen_many!(
        _at,
        _text,
        _resvec_54,
        {},
        _tredgen_match_regex!(_at, _text, _REGEX_46)
    );
    Ok((_at, _out))
}
fn _blockfn_16(
    _start: usize,
    _text: &str,
) -> Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>), ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_append!(
        _at,
        _resvec_57,
        {},
        _tredgen_match_str!(_at, _text, "true")?
    );
    _out.extend(Some(::std::boxed::Box::new(Token::True)));
    Ok((_at, _out))
}
fn _blockfn_17(
    _start: usize,
    _text: &str,
) -> Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>), ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_append!(
        _at,
        _resvec_58,
        {},
        _tredgen_match_str!(_at, _text, "false")?
    );
    _out.extend(Some(::std::boxed::Box::new(Token::False)));
    Ok((_at, _out))
}
fn _blockfn_18(
    _start: usize,
    _text: &str,
) -> Result<(usize, ::std::vec::Vec<::std::boxed::Box<Token>>), ::tredlib::ParseErr> {
    let mut _at = _start;
    let mut _out = ::std::vec::Vec::new();
    _tredgen_append!(
        _at,
        _resvec_59,
        {},
        _tredgen_match_str!(_at, _text, "null")?
    );
    _out.extend(Some(::std::boxed::Box::new(Token::Null)));
    Ok((_at, _out))
}
