// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_tool_use(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ToolUse,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("toolUseId").string(input.tool_use_id.as_str());
    }
    {
        object.key("name").string(input.name.as_str());
    }
    {
        object.key("input").document(&input.input);
    }
    Ok(())
}
