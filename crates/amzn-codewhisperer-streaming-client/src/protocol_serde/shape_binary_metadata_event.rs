// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_binary_metadata_event_payload(
    input: &[u8],
) -> ::std::result::Result<crate::types::BinaryMetadataEvent, ::aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned =
        ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(input)).peekable();
    let tokens = &mut tokens_owned;
    let result =
        crate::protocol_serde::shape_binary_metadata_event::de_binary_metadata_event(tokens)?.ok_or_else(|| {
            ::aws_smithy_json::deserialize::error::DeserializeError::custom("expected payload member value")
        });
    if tokens.next().is_some() {
        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    result
}

pub(crate) fn de_binary_metadata_event<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<
    Option<crate::types::BinaryMetadataEvent>,
    ::aws_smithy_json::deserialize::error::DeserializeError,
>
where
    I: Iterator<
        Item = Result<
            ::aws_smithy_json::deserialize::Token<'a>,
            ::aws_smithy_json::deserialize::error::DeserializeError,
        >,
    >,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::BinaryMetadataEventBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "size" => {
                                builder = builder.set_size(
                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                        .map(i64::try_from)
                                        .transpose()?,
                                );
                            },
                            "mimeType" => {
                                builder = builder.set_mime_type(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                        .transpose()?,
                                );
                            },
                            "contentChecksum" => {
                                builder = builder.set_content_checksum(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                        .transpose()?,
                                );
                            },
                            "contentChecksumType" => {
                                builder = builder.set_content_checksum_type(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                        .map(|s| {
                                            s.to_unescaped()
                                                .map(|u| crate::types::ContentChecksumType::from(u.as_ref()))
                                        })
                                        .transpose()?,
                                );
                            },
                            _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                        }
                    },
                    other => {
                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
                            format!("expected object key or end object, found: {:?}", other),
                        ));
                    },
                }
            }
            Ok(Some(builder.build()))
        },
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
