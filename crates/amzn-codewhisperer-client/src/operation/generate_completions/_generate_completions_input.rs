// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct GenerateCompletionsInput {
    #[allow(missing_docs)] // documentation missing in model
    pub file_context: ::std::option::Option<crate::types::FileContext>,
    /// Represents the state of an Editor
    pub editor_state: ::std::option::Option<crate::types::EditorState>,
    #[allow(missing_docs)] // documentation missing in model
    pub max_results: ::std::option::Option<i32>,
    #[allow(missing_docs)] // documentation missing in model
    pub prediction_types: ::std::option::Option<::std::vec::Vec<crate::types::PredictionType>>,
    #[allow(missing_docs)] // documentation missing in model
    pub next_token: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub reference_tracker_configuration: ::std::option::Option<crate::types::ReferenceTrackerConfiguration>,
    #[allow(missing_docs)] // documentation missing in model
    pub supplemental_contexts: ::std::option::Option<::std::vec::Vec<crate::types::SupplementalContext>>,
    #[allow(missing_docs)] // documentation missing in model
    pub customization_arn: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub opt_out_preference: ::std::option::Option<crate::types::OptOutPreference>,
    #[allow(missing_docs)] // documentation missing in model
    pub user_context: ::std::option::Option<crate::types::UserContext>,
    #[allow(missing_docs)] // documentation missing in model
    pub profile_arn: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub workspace_id: ::std::option::Option<::std::string::String>,
    /// Unique identifier for the model
    pub model_id: ::std::option::Option<::std::string::String>,
}
impl GenerateCompletionsInput {
    #[allow(missing_docs)] // documentation missing in model
    pub fn file_context(&self) -> ::std::option::Option<&crate::types::FileContext> {
        self.file_context.as_ref()
    }

    /// Represents the state of an Editor
    pub fn editor_state(&self) -> ::std::option::Option<&crate::types::EditorState> {
        self.editor_state.as_ref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }

    #[allow(missing_docs)] // documentation missing in model
    /// If no value was sent for this field, a default will be set. If you want to determine if no
    /// value was sent, use `.prediction_types.is_none()`.
    pub fn prediction_types(&self) -> &[crate::types::PredictionType] {
        self.prediction_types.as_deref().unwrap_or_default()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn reference_tracker_configuration(
        &self,
    ) -> ::std::option::Option<&crate::types::ReferenceTrackerConfiguration> {
        self.reference_tracker_configuration.as_ref()
    }

    #[allow(missing_docs)] // documentation missing in model
    /// If no value was sent for this field, a default will be set. If you want to determine if no
    /// value was sent, use `.supplemental_contexts.is_none()`.
    pub fn supplemental_contexts(&self) -> &[crate::types::SupplementalContext] {
        self.supplemental_contexts.as_deref().unwrap_or_default()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn customization_arn(&self) -> ::std::option::Option<&str> {
        self.customization_arn.as_deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn opt_out_preference(&self) -> ::std::option::Option<&crate::types::OptOutPreference> {
        self.opt_out_preference.as_ref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn user_context(&self) -> ::std::option::Option<&crate::types::UserContext> {
        self.user_context.as_ref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn profile_arn(&self) -> ::std::option::Option<&str> {
        self.profile_arn.as_deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn workspace_id(&self) -> ::std::option::Option<&str> {
        self.workspace_id.as_deref()
    }

    /// Unique identifier for the model
    pub fn model_id(&self) -> ::std::option::Option<&str> {
        self.model_id.as_deref()
    }
}
impl ::std::fmt::Debug for GenerateCompletionsInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("GenerateCompletionsInput");
        formatter.field("file_context", &self.file_context);
        formatter.field("editor_state", &self.editor_state);
        formatter.field("max_results", &self.max_results);
        formatter.field("prediction_types", &self.prediction_types);
        formatter.field("next_token", &"*** Sensitive Data Redacted ***");
        formatter.field("reference_tracker_configuration", &self.reference_tracker_configuration);
        formatter.field("supplemental_contexts", &self.supplemental_contexts);
        formatter.field("customization_arn", &self.customization_arn);
        formatter.field("opt_out_preference", &self.opt_out_preference);
        formatter.field("user_context", &self.user_context);
        formatter.field("profile_arn", &self.profile_arn);
        formatter.field("workspace_id", &self.workspace_id);
        formatter.field("model_id", &self.model_id);
        formatter.finish()
    }
}
impl GenerateCompletionsInput {
    /// Creates a new builder-style object to manufacture
    /// [`GenerateCompletionsInput`](crate::operation::generate_completions::GenerateCompletionsInput).
    pub fn builder() -> crate::operation::generate_completions::builders::GenerateCompletionsInputBuilder {
        crate::operation::generate_completions::builders::GenerateCompletionsInputBuilder::default()
    }
}

/// A builder for
/// [`GenerateCompletionsInput`](crate::operation::generate_completions::GenerateCompletionsInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct GenerateCompletionsInputBuilder {
    pub(crate) file_context: ::std::option::Option<crate::types::FileContext>,
    pub(crate) editor_state: ::std::option::Option<crate::types::EditorState>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) prediction_types: ::std::option::Option<::std::vec::Vec<crate::types::PredictionType>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) reference_tracker_configuration: ::std::option::Option<crate::types::ReferenceTrackerConfiguration>,
    pub(crate) supplemental_contexts: ::std::option::Option<::std::vec::Vec<crate::types::SupplementalContext>>,
    pub(crate) customization_arn: ::std::option::Option<::std::string::String>,
    pub(crate) opt_out_preference: ::std::option::Option<crate::types::OptOutPreference>,
    pub(crate) user_context: ::std::option::Option<crate::types::UserContext>,
    pub(crate) profile_arn: ::std::option::Option<::std::string::String>,
    pub(crate) workspace_id: ::std::option::Option<::std::string::String>,
    pub(crate) model_id: ::std::option::Option<::std::string::String>,
}
impl GenerateCompletionsInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn file_context(mut self, input: crate::types::FileContext) -> Self {
        self.file_context = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_file_context(mut self, input: ::std::option::Option<crate::types::FileContext>) -> Self {
        self.file_context = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_file_context(&self) -> &::std::option::Option<crate::types::FileContext> {
        &self.file_context
    }

    /// Represents the state of an Editor
    pub fn editor_state(mut self, input: crate::types::EditorState) -> Self {
        self.editor_state = ::std::option::Option::Some(input);
        self
    }

    /// Represents the state of an Editor
    pub fn set_editor_state(mut self, input: ::std::option::Option<crate::types::EditorState>) -> Self {
        self.editor_state = input;
        self
    }

    /// Represents the state of an Editor
    pub fn get_editor_state(&self) -> &::std::option::Option<crate::types::EditorState> {
        &self.editor_state
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        &self.max_results
    }

    /// Appends an item to `prediction_types`.
    ///
    /// To override the contents of this collection use
    /// [`set_prediction_types`](Self::set_prediction_types).
    pub fn prediction_types(mut self, input: crate::types::PredictionType) -> Self {
        let mut v = self.prediction_types.unwrap_or_default();
        v.push(input);
        self.prediction_types = ::std::option::Option::Some(v);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_prediction_types(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::PredictionType>>,
    ) -> Self {
        self.prediction_types = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_prediction_types(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::PredictionType>> {
        &self.prediction_types
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn reference_tracker_configuration(mut self, input: crate::types::ReferenceTrackerConfiguration) -> Self {
        self.reference_tracker_configuration = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_reference_tracker_configuration(
        mut self,
        input: ::std::option::Option<crate::types::ReferenceTrackerConfiguration>,
    ) -> Self {
        self.reference_tracker_configuration = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_reference_tracker_configuration(
        &self,
    ) -> &::std::option::Option<crate::types::ReferenceTrackerConfiguration> {
        &self.reference_tracker_configuration
    }

    /// Appends an item to `supplemental_contexts`.
    ///
    /// To override the contents of this collection use
    /// [`set_supplemental_contexts`](Self::set_supplemental_contexts).
    pub fn supplemental_contexts(mut self, input: crate::types::SupplementalContext) -> Self {
        let mut v = self.supplemental_contexts.unwrap_or_default();
        v.push(input);
        self.supplemental_contexts = ::std::option::Option::Some(v);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_supplemental_contexts(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::SupplementalContext>>,
    ) -> Self {
        self.supplemental_contexts = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_supplemental_contexts(
        &self,
    ) -> &::std::option::Option<::std::vec::Vec<crate::types::SupplementalContext>> {
        &self.supplemental_contexts
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn customization_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.customization_arn = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_customization_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.customization_arn = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_customization_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.customization_arn
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn opt_out_preference(mut self, input: crate::types::OptOutPreference) -> Self {
        self.opt_out_preference = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_opt_out_preference(mut self, input: ::std::option::Option<crate::types::OptOutPreference>) -> Self {
        self.opt_out_preference = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_opt_out_preference(&self) -> &::std::option::Option<crate::types::OptOutPreference> {
        &self.opt_out_preference
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn user_context(mut self, input: crate::types::UserContext) -> Self {
        self.user_context = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_user_context(mut self, input: ::std::option::Option<crate::types::UserContext>) -> Self {
        self.user_context = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_user_context(&self) -> &::std::option::Option<crate::types::UserContext> {
        &self.user_context
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn profile_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.profile_arn = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_profile_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.profile_arn = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_profile_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.profile_arn
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn workspace_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.workspace_id = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_workspace_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.workspace_id = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_workspace_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.workspace_id
    }

    /// Unique identifier for the model
    pub fn model_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.model_id = ::std::option::Option::Some(input.into());
        self
    }

    /// Unique identifier for the model
    pub fn set_model_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.model_id = input;
        self
    }

    /// Unique identifier for the model
    pub fn get_model_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.model_id
    }

    /// Consumes the builder and constructs a
    /// [`GenerateCompletionsInput`](crate::operation::generate_completions::GenerateCompletionsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::generate_completions::GenerateCompletionsInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::generate_completions::GenerateCompletionsInput {
            file_context: self.file_context,
            editor_state: self.editor_state,
            max_results: self.max_results,
            prediction_types: self.prediction_types,
            next_token: self.next_token,
            reference_tracker_configuration: self.reference_tracker_configuration,
            supplemental_contexts: self.supplemental_contexts,
            customization_arn: self.customization_arn,
            opt_out_preference: self.opt_out_preference,
            user_context: self.user_context,
            profile_arn: self.profile_arn,
            workspace_id: self.workspace_id,
            model_id: self.model_id,
        })
    }
}
impl ::std::fmt::Debug for GenerateCompletionsInputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("GenerateCompletionsInputBuilder");
        formatter.field("file_context", &self.file_context);
        formatter.field("editor_state", &self.editor_state);
        formatter.field("max_results", &self.max_results);
        formatter.field("prediction_types", &self.prediction_types);
        formatter.field("next_token", &"*** Sensitive Data Redacted ***");
        formatter.field("reference_tracker_configuration", &self.reference_tracker_configuration);
        formatter.field("supplemental_contexts", &self.supplemental_contexts);
        formatter.field("customization_arn", &self.customization_arn);
        formatter.field("opt_out_preference", &self.opt_out_preference);
        formatter.field("user_context", &self.user_context);
        formatter.field("profile_arn", &self.profile_arn);
        formatter.field("workspace_id", &self.workspace_id);
        formatter.field("model_id", &self.model_id);
        formatter.finish()
    }
}
