use crate::common::{ActionParameter, ParameterValue};
use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "serde")]
use serde_with::skip_serializing_none;

/// Node Action Object
#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "serde", skip_serializing_none)]
pub struct Action {
    ///  Name of action as described in the first column of "Actions and Parameters" Identifies the function of the action.
    pub action_type: String,
    ///  ID to distinguish between multiple actions, either instant or with the same type on the same node/edge.
    pub action_id: String,
    ///  Additional information on the action.
    pub action_description: Option<String>,
    ///  Regulates if the action is allowed to be executed during movement and/or parallel to other actions.
    pub blocking_type: BlockingType,
    ///  Array of actionParameter objects for the indicated action e.g. deviceId, loadId, external triggers.
    pub action_parameters: Vec<ActionParameter>,
}

/// Regulates if the action is allowed to be executed during movement and/or parallel to other actions.
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "fmt", derive(Debug))]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "SCREAMING_SNAKE_CASE")
)]
pub enum BlockingType {
    /// Action can happen in parallel with others, including movement.
    None,
    /// Action can happen simultaneously with others, but not while moving.
    Soft,
    /// No other actions can be performed while this action is running.
    Hard,
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use crate::{
        action::Action,
        common::{ActionParameter, ParameterValue},
    };

    use super::BlockingType;
    use rstest::rstest;

    #[rstest]
    fn test_clone_functionality() {
        // Test Copy on enum
        let blocking_type = BlockingType::Hard;
        let copied_blocking_type = blocking_type; // Copy
        let cloned_blocking_type = blocking_type.clone(); // Clone

        // Verify they're all the same
        assert!(matches!(blocking_type, BlockingType::Hard));
        assert!(matches!(copied_blocking_type, BlockingType::Hard));
        assert!(matches!(cloned_blocking_type, BlockingType::Hard));
    }

    #[rstest]
    fn test_partial_eq_functionality() {
        // Test PartialEq for ParameterValue
        let val1 = ParameterValue::Integer(42);
        let val2 = ParameterValue::Integer(42);
        let val3 = ParameterValue::Integer(43);

        assert_eq!(val1, val2);
        assert_ne!(val1, val3);

        // Test PartialEq for ActionParameter
        let param1 = ActionParameter {
            key: String::from("test"),
            value: ParameterValue::Bool(true),
            ..Default::default()
        };
        let param2 = ActionParameter {
            key: String::from("test"),
            value: ParameterValue::Bool(true),
            ..Default::default()
        };
        let param3 = ActionParameter {
            key: String::from("test"),
            value: ParameterValue::Bool(false),
            ..Default::default()
        };

        assert_eq!(param1, param2);
        assert_ne!(param1, param3);

        // Test PartialEq for Action
        let action1 = Action {
            action_type: String::from("move"),
            action_id: String::from("1"),
            action_description: None,
            blocking_type: BlockingType::None,
            action_parameters: vec![param1.clone()],
        };
        let action2 = Action {
            action_type: String::from("move"),
            action_id: String::from("1"),
            action_description: None,
            blocking_type: BlockingType::None,
            action_parameters: vec![param2],
        };

        assert_eq!(action1, action2);

        // Test Eq for BlockingType (enum)
        let blocking1 = BlockingType::Hard;
        let blocking2 = BlockingType::Hard;
        let blocking3 = BlockingType::Soft;

        assert_eq!(blocking1, blocking2);
        assert_ne!(blocking1, blocking3);
    }
}
