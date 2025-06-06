// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_code_fix_job_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::start_code_fix_job::StartCodeFixJobInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.snippet_range {
        #[allow(unused_mut)]
        let mut object_2 = object.key("snippetRange").start_object();
        crate::protocol_serde::shape_range::ser_range(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.upload_id {
        object.key("uploadId").string(var_3.as_str());
    }
    if let Some(var_4) = &input.description {
        object.key("description").string(var_4.as_str());
    }
    if let Some(var_5) = &input.rule_id {
        object.key("ruleId").string(var_5.as_str());
    }
    if let Some(var_6) = &input.code_fix_name {
        object.key("codeFixName").string(var_6.as_str());
    }
    if let Some(var_7) = &input.reference_tracker_configuration {
        #[allow(unused_mut)]
        let mut object_8 = object.key("referenceTrackerConfiguration").start_object();
        crate::protocol_serde::shape_reference_tracker_configuration::ser_reference_tracker_configuration(
            &mut object_8,
            var_7,
        )?;
        object_8.finish();
    }
    if let Some(var_9) = &input.profile_arn {
        object.key("profileArn").string(var_9.as_str());
    }
    Ok(())
}
